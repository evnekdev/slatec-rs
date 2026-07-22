#![cfg(feature = "ode-sdrive-expert-native-tests")]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use slatec::ode::{
    OdeError, OdeInputError, OdeIteration, OdeJacobianWriter, OdeMethod, OdeOptions, OdeSession,
    OdeStatus, OdeTolerance, OdeTolerances, max_native_call_concurrency_for_test,
    reset_native_call_concurrency_for_test, with_native_runtime_lock_for_test,
};

fn scalar_tolerances<T>(relative: T, absolute: T) -> OdeTolerances<T> {
    OdeTolerances {
        relative,
        absolute: OdeTolerance::Scalar(absolute),
    }
}

#[test]
fn double_exponential_decay_continues_to_multiple_targets() {
    let mut session = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -2.0 * state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        session.integrate_to(0.25).unwrap().status,
        OdeStatus::ReachedTarget
    );
    assert_eq!(
        session.integrate_to(1.0).unwrap().status,
        OdeStatus::ReachedTarget
    );
    assert!((session.state()[0] - (-2.0_f64).exp()).abs() < 3.0e-6);
}

#[test]
fn direct_and_segmented_continuation_agree() {
    let build_session = || {
        OdeSession::new(
            0.0_f64,
            vec![1.0],
            |_time, state, derivative| -> Result<(), ()> {
                derivative[0] = -state[0];
                Ok(())
            },
            scalar_tolerances(1.0e-9, 1.0e-11),
            OdeOptions::default(),
        )
        .unwrap()
    };
    let mut direct = build_session();
    let mut segmented = build_session();
    direct.integrate_to(1.0).unwrap();
    segmented.integrate_to(0.2).unwrap();
    segmented.integrate_to(0.6).unwrap();
    segmented.integrate_to(1.0).unwrap();
    assert!((direct.state()[0] - segmented.state()[0]).abs() < 2.0e-7);
    assert!((direct.state()[0] - (-1.0_f64).exp()).abs() < 2.0e-6);
}

#[test]
fn single_harmonic_oscillator_accepts_component_tolerances() {
    let mut session = OdeSession::new(
        0.0_f32,
        vec![1.0, 0.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = state[1];
            derivative[1] = -state[0];
            Ok(())
        },
        OdeTolerances {
            relative: 1.0e-5,
            absolute: OdeTolerance::Vector(vec![1.0e-7, 1.0e-7]),
        },
        OdeOptions::default(),
    )
    .unwrap();
    session.integrate_to(core::f32::consts::FRAC_PI_2).unwrap();
    assert!(session.state()[0].abs() < 3.0e-3);
    assert!((session.state()[1] + 1.0).abs() < 3.0e-3);
}

#[test]
fn backward_integration_is_supported_from_a_fresh_session() {
    let mut session = OdeSession::new(
        1.0_f64,
        vec![1.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = 1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-9, 1.0e-11),
        OdeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        session.integrate_to(0.0).unwrap().status,
        OdeStatus::ReachedTarget
    );
    assert!(session.state()[0].abs() < 1.0e-7);
}

#[test]
fn bdf_functional_iteration_remains_rhs_only() {
    let mut session = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -10.0 * state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-7, 1.0e-9),
        OdeOptions {
            method: OdeMethod::Bdf,
            maximum_order: Some(5),
            maximum_steps: None,
            maximum_step: Some(0.01),
            iteration: OdeIteration::Functional,
        },
    )
    .unwrap();
    session.integrate_to(0.2).unwrap();
    assert!((session.state()[0] - (-2.0_f64).exp()).abs() < 1.0e-4);
}

#[test]
fn stiff_dense_analytic_and_internal_jacobians_agree() {
    let options = OdeOptions {
        method: OdeMethod::Bdf,
        maximum_order: Some(5),
        maximum_steps: None,
        maximum_step: Some(0.02),
        iteration: OdeIteration::AnalyticDense,
    };
    let mut analytic = OdeSession::new_with_jacobian(
        0.0_f64,
        vec![1.0, 0.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -40.0 * state[0] + 20.0 * state[1];
            derivative[1] = 20.0 * state[0] - 40.0 * state[1];
            Ok(())
        },
        |_time, _state, mut jacobian: OdeJacobianWriter<'_, f64>| -> Result<(), ()> {
            jacobian.set(0, 0, -40.0).unwrap();
            jacobian.set(0, 1, 20.0).unwrap();
            jacobian.set(1, 0, 20.0).unwrap();
            jacobian.set(1, 1, -40.0).unwrap();
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        options,
    )
    .unwrap();
    analytic.integrate_to(0.2).unwrap();

    let mut finite_difference = OdeSession::new(
        0.0_f64,
        vec![1.0, 0.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -40.0 * state[0] + 20.0 * state[1];
            derivative[1] = 20.0 * state[0] - 40.0 * state[1];
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions {
            iteration: OdeIteration::FiniteDifferenceDense,
            ..options
        },
    )
    .unwrap();
    finite_difference.integrate_to(0.2).unwrap();

    for (analytic, finite_difference) in analytic.state().iter().zip(finite_difference.state()) {
        assert!((analytic - finite_difference).abs() < 2.0e-6);
    }
}

#[test]
fn stiff_banded_analytic_and_internal_jacobians_agree() {
    let options = OdeOptions {
        method: OdeMethod::Bdf,
        maximum_order: Some(5),
        maximum_steps: None,
        maximum_step: Some(0.01),
        iteration: OdeIteration::AnalyticBanded {
            lower_bandwidth: 1,
            upper_bandwidth: 1,
        },
    };
    let rhs = |_time: f64, state: &[f64], derivative: &mut [f64]| -> Result<(), ()> {
        derivative[0] = -40.0 * state[0] + 20.0 * state[1];
        derivative[1] = 20.0 * state[0] - 40.0 * state[1] + 20.0 * state[2];
        derivative[2] = 20.0 * state[1] - 40.0 * state[2];
        Ok(())
    };
    let mut analytic = OdeSession::new_with_jacobian(
        0.0_f64,
        vec![1.0, 0.0, 0.0],
        rhs,
        |_time, _state, mut jacobian: OdeJacobianWriter<'_, f64>| -> Result<(), ()> {
            jacobian.set(0, 0, -40.0).unwrap();
            jacobian.set(0, 1, 20.0).unwrap();
            jacobian.set(1, 0, 20.0).unwrap();
            jacobian.set(1, 1, -40.0).unwrap();
            jacobian.set(1, 2, 20.0).unwrap();
            jacobian.set(2, 1, 20.0).unwrap();
            jacobian.set(2, 2, -40.0).unwrap();
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        options,
    )
    .unwrap();
    analytic.integrate_to(0.2).unwrap();

    let mut finite_difference = OdeSession::new(
        0.0_f64,
        vec![1.0, 0.0, 0.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -40.0 * state[0] + 20.0 * state[1];
            derivative[1] = 20.0 * state[0] - 40.0 * state[1] + 20.0 * state[2];
            derivative[2] = 20.0 * state[1] - 40.0 * state[2];
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions {
            iteration: OdeIteration::FiniteDifferenceBanded {
                lower_bandwidth: 1,
                upper_bandwidth: 1,
            },
            ..options
        },
    )
    .unwrap();
    finite_difference.integrate_to(0.2).unwrap();

    for (analytic, finite_difference) in analytic.state().iter().zip(finite_difference.state()) {
        assert!((analytic - finite_difference).abs() < 2.0e-6);
    }
}

#[test]
fn analytic_jacobian_panic_and_nonfinite_values_are_contained() {
    let options = OdeOptions {
        method: OdeMethod::Bdf,
        iteration: OdeIteration::AnalyticDense,
        ..OdeOptions::default()
    };
    let mut panicking = OdeSession::new_with_jacobian(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -state[0];
            Ok(())
        },
        |_time, _state, _jacobian: OdeJacobianWriter<'_, f64>| -> Result<(), ()> {
            panic!("contained Jacobian panic")
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        options,
    )
    .unwrap();
    assert!(matches!(
        panicking.integrate_to(0.1),
        Err(OdeError::JacobianCallbackPanicked)
    ));

    let mut nonfinite = OdeSession::new_with_jacobian(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -state[0];
            Ok(())
        },
        |_time, _state, mut jacobian: OdeJacobianWriter<'_, f64>| -> Result<(), ()> {
            jacobian.set(0, 0, f64::NAN).unwrap();
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        options,
    )
    .unwrap();
    assert!(matches!(
        nonfinite.integrate_to(0.1),
        Err(OdeError::NonFiniteJacobian { row: 0, column: 0 })
    ));
}

#[test]
fn callback_error_terminates_without_leaking_context() {
    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    let mut failing = OdeSession::new(
        0.0_f64,
        vec![1.0],
        move |_time, _state, derivative| -> Result<(), &'static str> {
            if callback_calls.fetch_add(1, Ordering::SeqCst) > 1 {
                return Err("stop");
            }
            derivative[0] = -1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        failing.integrate_to(1.0),
        Err(OdeError::Callback("stop"))
    ));
    assert!(matches!(
        failing.integrate_to(1.0),
        Err(OdeError::SessionFailed)
    ));

    let mut recovered = OdeSession::new(
        0.0_f64,
        vec![0.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = 1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    recovered.integrate_to(1.0).unwrap();
    assert!((recovered.state()[0] - 1.0).abs() < 1.0e-6);
}

#[test]
fn callback_panic_and_nonfinite_derivative_are_contained() {
    let mut panicking = OdeSession::new(
        0.0_f32,
        vec![1.0],
        |_time, _state, _derivative| -> Result<(), ()> { panic!("contained") },
        scalar_tolerances(1.0e-5, 1.0e-7),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        panicking.integrate_to(1.0),
        Err(OdeError::CallbackPanicked)
    ));

    let mut nonfinite = OdeSession::new(
        0.0_f32,
        vec![1.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = f32::NAN;
            Ok(())
        },
        scalar_tolerances(1.0e-5, 1.0e-7),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        nonfinite.integrate_to(1.0),
        Err(OdeError::NonFiniteDerivative { index: 0 })
    ));
}

#[test]
fn callback_failure_restores_xerror_and_allows_fresh_session() {
    with_native_runtime_lock_for_test(|| {
        let mut before = 0;
        // SAFETY: the reviewed XGETF ABI writes one valid native INTEGER.
        unsafe { slatec_sys::legacy_error::xgetf(&mut before) };
        let mut failing = OdeSession::new(
            0.0_f64,
            vec![1.0],
            |_time, _state, _derivative| -> Result<(), ()> { Err(()) },
            scalar_tolerances(1.0e-8, 1.0e-10),
            OdeOptions::default(),
        )
        .unwrap();
        assert!(matches!(
            failing.integrate_to(1.0),
            Err(OdeError::Callback(()))
        ));
        let mut after = 0;
        // SAFETY: the reviewed XGETF ABI writes one valid native INTEGER.
        unsafe { slatec_sys::legacy_error::xgetf(&mut after) };
        assert_eq!(
            after, before,
            "SDRIV3 callback termination must restore XERROR"
        );
    });

    let mut fresh = OdeSession::new(
        0.0_f64,
        vec![0.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = 1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    fresh.integrate_to(1.0).unwrap();
    assert!((fresh.state()[0] - 1.0).abs() < 1.0e-6);
}

#[test]
fn rejects_invalid_input_before_native_entry() {
    let result = OdeSession::new(
        0.0_f64,
        Vec::new(),
        |_time, _state, _derivative| -> Result<(), ()> { Ok(()) },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    );
    assert!(matches!(
        result,
        Err(OdeError::InvalidInput(OdeInputError::EmptyState))
    ));

    let result = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, _state, _derivative| -> Result<(), ()> { Ok(()) },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions {
            maximum_order: Some(13),
            ..OdeOptions::default()
        },
    );
    assert!(matches!(
        result,
        Err(OdeError::InvalidInput(OdeInputError::InvalidMaximumOrder))
    ));
}

#[test]
fn independent_sessions_are_serialized_without_context_cross_talk() {
    reset_native_call_concurrency_for_test();
    let handles = (0..4)
        .map(|_| {
            std::thread::spawn(|| {
                let mut session = OdeSession::new(
                    0.0_f64,
                    vec![0.0],
                    |_time, _state, derivative| -> Result<(), ()> {
                        derivative[0] = 1.0;
                        Ok(())
                    },
                    scalar_tolerances(1.0e-8, 1.0e-10),
                    OdeOptions::default(),
                )
                .unwrap();
                session.integrate_to(1.0).unwrap();
                session.state()[0]
            })
        })
        .collect::<Vec<_>>();
    for handle in handles {
        assert!((handle.join().unwrap() - 1.0).abs() < 1.0e-6);
    }
    assert_eq!(max_native_call_concurrency_for_test(), 1);
}

#[test]
fn nested_session_use_from_rhs_is_rejected_before_native_reentry() {
    let rejected = Arc::new(AtomicUsize::new(0));
    let rejected_in_rhs = Arc::clone(&rejected);
    let mut outer = OdeSession::new(
        0.0_f64,
        vec![1.0],
        move |_time, state, derivative| -> Result<(), ()> {
            let mut nested = OdeSession::new(
                0.0_f64,
                vec![0.0],
                |_time, _state, output| -> Result<(), ()> {
                    output[0] = 1.0;
                    Ok(())
                },
                scalar_tolerances(1.0e-8, 1.0e-10),
                OdeOptions::default(),
            )
            .unwrap();
            if matches!(nested.integrate_to(1.0), Err(OdeError::ReentrantCall)) {
                rejected_in_rhs.fetch_add(1, Ordering::SeqCst);
            }
            derivative[0] = -state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    outer.integrate_to(0.25).unwrap();
    assert!(rejected.load(Ordering::SeqCst) > 0);
}

#[test]
fn excess_work_status_preserves_a_session_for_continuation() {
    let mut session = OdeSession::new(
        0.0_f64,
        vec![0.0],
        |_time, _state, derivative| -> Result<(), ()> {
            derivative[0] = 1.0;
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions {
            maximum_steps: Some(1),
            maximum_step: Some(0.05),
            ..OdeOptions::default()
        },
    )
    .unwrap();
    assert_eq!(
        session.integrate_to(0.5).unwrap().status,
        OdeStatus::ExcessWork
    );
    for _ in 0..32 {
        if session.integrate_to(0.5).unwrap().status == OdeStatus::ReachedTarget {
            break;
        }
    }
    assert!((session.time() - 0.5).abs() < 1.0e-12);
    assert!((session.state()[0] - 0.5).abs() < 1.0e-6);
}

#[test]
fn tolerance_adjustment_is_recoverable() {
    let mut session = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-30, 1.0e-30),
        OdeOptions::default(),
    )
    .unwrap();
    assert_eq!(
        session.integrate_to(0.1).unwrap().status,
        OdeStatus::ToleranceAdjusted
    );
    assert_eq!(
        session.integrate_to(0.1).unwrap().status,
        OdeStatus::ReachedTarget
    );
    assert!((session.state()[0] - (-0.1_f64).exp()).abs() < 1.0e-10);
}

fn failed_then_clean_f64() -> f64 {
    let mut failed = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, _state, _derivative| -> Result<(), ()> { Err(()) },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        failed.integrate_to(0.25),
        Err(OdeError::Callback(()))
    ));
    let mut clean = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    clean.integrate_to(0.25).unwrap();
    clean.state()[0]
}

fn failed_then_clean_f32() -> f32 {
    let mut failed = OdeSession::new(
        0.0_f32,
        vec![1.0],
        |_time, _state, _derivative| -> Result<(), ()> { Err(()) },
        scalar_tolerances(1.0e-5, 1.0e-7),
        OdeOptions::default(),
    )
    .unwrap();
    assert!(matches!(
        failed.integrate_to(0.25),
        Err(OdeError::Callback(()))
    ));
    let mut clean = OdeSession::new(
        0.0_f32,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-5, 1.0e-7),
        OdeOptions::default(),
    )
    .unwrap();
    clean.integrate_to(0.25).unwrap();
    clean.state()[0]
}

#[test]
fn saved_ier_does_not_contaminate_new_sdrive_sessions_under_the_lock() {
    let failed_then_clean = failed_then_clean_f64();
    let mut clean_then_failed = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -state[0];
            Ok(())
        },
        scalar_tolerances(1.0e-8, 1.0e-10),
        OdeOptions::default(),
    )
    .unwrap();
    clean_then_failed.integrate_to(0.25).unwrap();
    let clean_then_failed = clean_then_failed.state()[0];
    let failed_then_clean_thread = std::thread::spawn(failed_then_clean_f64).join().unwrap();
    let expected = (-0.25_f64).exp();
    assert!((failed_then_clean - expected).abs() < 2.0e-6);
    assert!((clean_then_failed - expected).abs() < 2.0e-6);
    assert!((failed_then_clean_thread - expected).abs() < 2.0e-6);

    let single_after_failure = failed_then_clean_f32();
    let single_after_failure_thread = std::thread::spawn(failed_then_clean_f32).join().unwrap();
    let single_expected = (-0.25_f32).exp();
    assert!((single_after_failure - single_expected).abs() < 3.0e-3);
    assert!((single_after_failure_thread - single_expected).abs() < 3.0e-3);
}
