#![cfg(feature = "dassl-native-tests")]

use std::fs;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use slatec::dassl::{
    DaeError, DaeInputError, DaeJacobianMode, DaeOptions, DaeSession, DaeStatus, DaeTolerance,
    ResidualAction,
};

fn decay_residual_f64()
-> impl FnMut(f64, &[f64], &[f64], &mut [f64]) -> Result<ResidualAction, &'static str> {
    |_: f64, y, y_prime, output| {
        output[0] = y_prime[0] + y[0];
        if output.len() > 1 {
            output[1] = y[1];
        }
        Ok(ResidualAction::Continue)
    }
}

fn scalar_f64() -> DaeTolerance<f64> {
    DaeTolerance::Scalar {
        relative: 1.0e-7,
        absolute: 1.0e-9,
    }
}

#[test]
fn f64_index_one_decay_and_continuation_match_analytic_solution() {
    let mut session = DaeSession::new(
        0.0,
        vec![1.0, 0.0],
        vec![-1.0, 0.0],
        decay_residual_f64(),
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        session.advance_to(0.4).unwrap().status,
        DaeStatus::ReachedTarget
    );
    let result = session.advance_to(1.0).unwrap();
    assert_eq!(result.status, DaeStatus::ReachedTarget);
    assert!((session.state()[0] - (-1.0_f64).exp()).abs() < 3.0e-5);
    assert!(session.state()[1].abs() < 3.0e-7);
    assert!(result.diagnostics.maximum_absolute_residual.unwrap_or(1.0) < 2.0e-5);
}

#[test]
fn finite_difference_banded_iteration_matches_dense_dassl() {
    let residual = |_: f64, y: &[f64], y_prime: &[f64], output: &mut [f64]| {
        output[0] = y_prime[0] + 40.0 * y[0] - 20.0 * y[1];
        output[1] = y_prime[1] - 20.0 * y[0] + 40.0 * y[1] - 20.0 * y[2];
        output[2] = y_prime[2] - 20.0 * y[1] + 40.0 * y[2];
        Ok::<_, &'static str>(ResidualAction::Continue)
    };
    let mut banded = DaeSession::new(
        0.0,
        vec![1.0, 0.0, 0.0],
        vec![-40.0, 20.0, 0.0],
        residual,
        scalar_f64(),
        DaeOptions {
            jacobian_mode: DaeJacobianMode::FiniteDifferenceBanded {
                lower_bandwidth: 1,
                upper_bandwidth: 1,
            },
            ..DaeOptions::default()
        },
    )
    .unwrap();
    banded.advance_to(0.2).unwrap();

    let mut dense = DaeSession::new(
        0.0,
        vec![1.0, 0.0, 0.0],
        vec![-40.0, 20.0, 0.0],
        |_: f64, y: &[f64], y_prime: &[f64], output: &mut [f64]| {
            output[0] = y_prime[0] + 40.0 * y[0] - 20.0 * y[1];
            output[1] = y_prime[1] - 20.0 * y[0] + 40.0 * y[1] - 20.0 * y[2];
            output[2] = y_prime[2] - 20.0 * y[1] + 40.0 * y[2];
            Ok::<_, &'static str>(ResidualAction::Continue)
        },
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    dense.advance_to(0.2).unwrap();

    for (banded, dense) in banded.state().iter().zip(dense.state()) {
        assert!((banded - dense).abs() < 4.0e-5);
    }
}

#[test]
fn f32_vector_tolerances_preserve_algebraic_component() {
    let residual = |_: f32, y: &[f32], y_prime: &[f32], output: &mut [f32]| {
        output[0] = y_prime[0] + y[0];
        output[1] = y[1];
        Ok::<_, &'static str>(ResidualAction::Continue)
    };
    let mut session = DaeSession::new(
        0.0,
        vec![1.0, 0.0],
        vec![-1.0, 0.0],
        residual,
        DaeTolerance::Vector {
            relative: vec![2.0e-4, 2.0e-4],
            absolute: vec![2.0e-6, 2.0e-6],
        },
        DaeOptions::default(),
    )
    .unwrap();
    session.advance_to(1.0).unwrap();
    assert!((session.state()[0] - (-1.0_f32).exp()).abs() < 3.0e-3);
    assert!(session.state()[1].abs() < 3.0e-4);
}

#[test]
fn backwards_direction_is_preserved() {
    let mut backwards = DaeSession::new(
        1.0,
        vec![(-1.0_f64).exp()],
        vec![-(-1.0_f64).exp()],
        decay_residual_f64(),
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        backwards.advance_to(0.0).unwrap().status,
        DaeStatus::ReachedTarget
    );
    assert!((backwards.state()[0] - 1.0).abs() < 4.0e-5);
}

#[test]
fn callback_error_panic_and_nonfinite_are_contained_and_fresh_session_recovers() {
    let mut callback_error = DaeSession::new(
        0.0,
        vec![1.0],
        vec![-1.0],
        |_: f64, _: &[f64], _: &[f64], _: &mut [f64]| {
            Err::<ResidualAction, _>("expected callback error")
        },
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        callback_error.advance_to(0.1),
        Err(DaeError::Callback(_))
    ));

    let mut panicking = DaeSession::new(
        0.0,
        vec![1.0],
        vec![-1.0],
        |_: f64, _: &[f64], _: &[f64], _: &mut [f64]| -> Result<ResidualAction, &'static str> {
            panic!("expected panic")
        },
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        panicking.advance_to(0.1),
        Err(DaeError::CallbackPanicked)
    ));

    let mut nonfinite = DaeSession::new(
        0.0,
        vec![1.0],
        vec![-1.0],
        |_: f64, _: &[f64], _: &[f64], output: &mut [f64]| {
            output[0] = f64::NAN;
            Ok::<_, &'static str>(ResidualAction::Continue)
        },
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        nonfinite.advance_to(0.1),
        Err(DaeError::NonFiniteResidual { .. })
    ));

    let mut fresh = DaeSession::new(
        0.0,
        vec![1.0],
        vec![-1.0],
        decay_residual_f64(),
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        fresh.advance_to(0.1).unwrap().status,
        DaeStatus::ReachedTarget
    );
}

#[test]
fn recoverable_residual_action_can_retry_and_invalid_input_never_enters_native_code() {
    let first = Arc::new(AtomicBool::new(true));
    let attempt = Arc::clone(&first);
    let mut session = DaeSession::new(
        0.0,
        vec![1.0],
        vec![-1.0],
        move |_, y: &[f64], y_prime: &[f64], output: &mut [f64]| {
            if attempt.swap(false, Ordering::SeqCst) {
                return Ok::<_, &'static str>(ResidualAction::RecoverableFailure);
            }
            output[0] = y_prime[0] + y[0];
            Ok(ResidualAction::Continue)
        },
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        session.advance_to(0.1).unwrap().status,
        DaeStatus::ReachedTarget
    );

    let invalid = DaeSession::new(
        0.0,
        Vec::<f64>::new(),
        Vec::new(),
        decay_residual_f64(),
        scalar_f64(),
        DaeOptions::default(),
    );
    assert!(matches!(
        invalid,
        Err(DaeError::InvalidInput(DaeInputError::EmptyState))
    ));
}

#[test]
fn inconsistent_algebraic_initial_pair_fails_without_contaminating_a_fresh_session() {
    let mut inconsistent = DaeSession::new(
        0.0,
        vec![0.0_f64],
        vec![0.0],
        |_: f64, y: &[f64], _: &[f64], output: &mut [f64]| {
            // This algebraic equation cannot hold for the supplied y(0).
            output[0] = y[0] - 1.0;
            Ok::<_, &'static str>(ResidualAction::Continue)
        },
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        inconsistent.advance_to(0.1),
        Err(DaeError::NativeFailure(_)) | Err(DaeError::NativeContractViolation { .. })
    ));

    let mut fresh = DaeSession::new(
        0.0,
        vec![1.0],
        vec![-1.0],
        decay_residual_f64(),
        scalar_f64(),
        DaeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        fresh.advance_to(0.1).unwrap().status,
        DaeStatus::ReachedTarget
    );
}

#[test]
fn advance_creates_no_current_or_temporary_directory_artifact() {
    const CHILD: &str = "SLATEC_DASSL_NO_FILE_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let current = std::env::current_dir().unwrap();
        let configured_temp = std::env::temp_dir();
        assert_eq!(fs::read_dir(&current).unwrap().count(), 0);
        assert_eq!(fs::read_dir(&configured_temp).unwrap().count(), 0);

        let mut session = DaeSession::new(
            0.0,
            vec![1.0],
            vec![-1.0],
            decay_residual_f64(),
            scalar_f64(),
            DaeOptions::default(),
        )
        .unwrap();
        assert_eq!(
            session.advance_to(0.1).unwrap().status,
            DaeStatus::ReachedTarget
        );
        assert_eq!(fs::read_dir(&current).unwrap().count(), 0);
        assert_eq!(fs::read_dir(&configured_temp).unwrap().count(), 0);
        return;
    }

    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "slatec-dassl-no-files-{}-{unique}",
        std::process::id()
    ));
    let current = root.join("current");
    let configured_temp = root.join("temp");
    fs::create_dir_all(&current).unwrap();
    fs::create_dir(&configured_temp).unwrap();
    let status = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("advance_creates_no_current_or_temporary_directory_artifact")
        .arg("--nocapture")
        .current_dir(&current)
        .env(CHILD, "1")
        .env("TMP", &configured_temp)
        .env("TEMP", &configured_temp)
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(fs::read_dir(&current).unwrap().count(), 0);
    assert_eq!(fs::read_dir(&configured_temp).unwrap().count(), 0);
    fs::remove_dir_all(&root).unwrap();
}
