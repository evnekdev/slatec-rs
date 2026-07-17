#![cfg(all(
    feature = "nonlinear-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native GNU MinGW validation for expert nonlinear and Jacobian-check APIs.

use std::sync::{Arc, Barrier};
use std::thread;

use slatec::nonlinear::{
    ExpertNonlinearOptions, JacobianCheckError, JacobianStructure, NonlinearError,
    NonlinearOptions, NonlinearStatus, VariableScaling, check_jacobian, check_jacobian_f32,
    solve_system, solve_system_expert, solve_system_expert_f32, solve_system_with_jacobian,
    solve_system_with_jacobian_f32,
};
use slatec::quadrature::{IntegrationOptions, integrate};
use slatec::roots::{RootBracket, RootOptions, find_root};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn scalar_expert() -> Result<slatec::nonlinear::ExpertNonlinearResult<f64>, NonlinearError> {
    solve_system_expert(
        &[1.0],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        ExpertNonlinearOptions::default(),
    )
}

#[test]
fn finite_difference_expert_solvers_cover_both_precisions_and_bands() {
    let scalar = scalar_expert().unwrap();
    close(scalar.solution[0], 2.0_f64.sqrt(), 2.0e-9);
    assert!(scalar.residual_norm < 2.0e-9);
    assert_eq!(scalar.jacobian_evaluations, 0);
    assert_eq!(scalar.status, NonlinearStatus::Converged);

    let pair = solve_system_expert(
        &[0.8, 2.2],
        |x, residual| {
            residual[0] = x[0] + x[1] - 3.0;
            residual[1] = x[0] * x[0] + x[1] * x[1] - 5.0;
        },
        ExpertNonlinearOptions::default(),
    )
    .unwrap();
    close(pair.solution[0], 1.0, 2.0e-8);
    close(pair.solution[1], 2.0, 2.0e-8);

    let coupled = solve_system_expert(
        &[1.2, 1.9, 2.8],
        |x, residual| {
            residual[0] = x[0] + x[1] + x[2] - 6.0;
            residual[1] = x[0] * x[0] + x[1] * x[1] + x[2] * x[2] - 14.0;
            residual[2] = x[0] * x[1] * x[2] - 6.0;
        },
        ExpertNonlinearOptions::default(),
    )
    .unwrap();
    for (actual, expected) in coupled.solution.iter().zip([1.0, 2.0, 3.0]) {
        close(*actual, expected, 1.0e-7);
    }

    let banded = solve_system_expert(
        &[1.0, 1.0, 1.0],
        |x, residual| {
            residual[0] = x[0] * x[0] - 1.0;
            residual[1] = x[1] * x[1] - 4.0;
            residual[2] = x[2] * x[2] - 9.0;
        },
        ExpertNonlinearOptions {
            jacobian_structure: JacobianStructure::Banded {
                lower_bandwidth: 0,
                upper_bandwidth: 0,
            },
            ..ExpertNonlinearOptions::default()
        },
    )
    .unwrap();
    for (actual, expected) in banded.solution.iter().zip([1.0, 2.0, 3.0]) {
        close(*actual, expected, 2.0e-7);
    }

    let single = solve_system_expert_f32(
        &[1.0_f32],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        ExpertNonlinearOptions::single_precision(),
    )
    .unwrap();
    close(f64::from(single.solution[0]), 2.0_f64.sqrt(), 3.0e-4);
}

#[test]
fn analytic_jacobians_solve_reference_systems_and_report_counts() {
    let analytic = solve_system_with_jacobian(
        &[0.8, 2.2],
        |x, residual| {
            residual[0] = x[0] + x[1] - 3.0;
            residual[1] = x[0] * x[0] + x[1] * x[1] - 5.0;
        },
        |x, residual, mut jacobian| {
            assert_eq!(residual.len(), 2);
            jacobian.set(0, 0, 1.0).unwrap();
            jacobian.set(0, 1, 1.0).unwrap();
            jacobian.set(1, 0, 2.0 * x[0]).unwrap();
            jacobian.set(1, 1, 2.0 * x[1]).unwrap();
        },
        ExpertNonlinearOptions::default(),
    )
    .unwrap();
    close(analytic.solution[0], 1.0, 2.0e-9);
    close(analytic.solution[1], 2.0, 2.0e-9);
    assert!(analytic.function_evaluations > 0);
    assert!(analytic.jacobian_evaluations > 0);

    let analytic_f32 = solve_system_with_jacobian_f32(
        &[1.0_f32],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        |x, _, mut jacobian| jacobian.set(0, 0, 2.0 * x[0]).unwrap(),
        ExpertNonlinearOptions::single_precision(),
    )
    .unwrap();
    close(f64::from(analytic_f32.solution[0]), 2.0_f64.sqrt(), 3.0e-4);
    assert!(analytic_f32.jacobian_evaluations > 0);
}

#[test]
fn jacobian_checkers_distinguish_correct_and_incorrect_derivatives() {
    let correct = check_jacobian(
        &[1.5, 0.75],
        |x, f| {
            f[0] = x[0] * x[0] + x[1];
            f[1] = x[0] - x[1] * x[1];
        },
        |x, _, mut j| {
            j.set(0, 0, 2.0 * x[0]).unwrap();
            j.set(0, 1, 1.0).unwrap();
            j.set(1, 0, 1.0).unwrap();
            j.set(1, 1, -2.0 * x[1]).unwrap();
        },
    )
    .unwrap();
    assert!(correct.scores.iter().all(|score| *score > 0.5));

    let incorrect = check_jacobian(
        &[1.5, 0.75],
        |x, f| {
            f[0] = x[0] * x[0] + x[1];
            f[1] = x[0] - x[1] * x[1];
        },
        |x, _, mut j| {
            j.set(0, 0, -2.0 * x[0]).unwrap();
            j.set(0, 1, -1.0).unwrap();
            j.set(1, 0, 1.0).unwrap();
            j.set(1, 1, -2.0 * x[1]).unwrap();
        },
    )
    .unwrap();
    assert!(incorrect.scores[0] < correct.scores[0]);
    assert!(incorrect.suspicious_rows.contains(&0));

    let single = check_jacobian_f32(
        &[1.5_f32],
        |x, f| f[0] = x[0] * x[0],
        |x, _, mut j| j.set(0, 0, 2.0 * x[0]).unwrap(),
    )
    .unwrap();
    assert!(single.scores[0] > 0.5);

    let constant = check_jacobian(
        &[1.0],
        |_, f| f[0] = 3.0,
        |_, _, mut j| j.set(0, 0, 0.0).unwrap(),
    )
    .unwrap();
    assert!((0.0..=1.0).contains(&constant.scores[0]));
}

#[test]
fn expert_validation_rejects_bad_scaling_bands_and_controls() {
    let scales = [1.0, 0.0];
    assert!(matches!(
        solve_system_expert(
            &[1.0, 1.0],
            |_, f| f.fill(0.0),
            ExpertNonlinearOptions {
                scaling: VariableScaling::User(&scales),
                ..ExpertNonlinearOptions::default()
            },
        ),
        Err(NonlinearError::InvalidScalingValue { index: 1 })
    ));
    assert!(matches!(
        solve_system_expert(
            &[1.0],
            |_, f| f.fill(0.0),
            ExpertNonlinearOptions {
                jacobian_structure: JacobianStructure::Banded {
                    lower_bandwidth: 1,
                    upper_bandwidth: 0,
                },
                ..ExpertNonlinearOptions::default()
            },
        ),
        Err(NonlinearError::InvalidBandwidth { .. })
    ));
    assert_eq!(
        check_jacobian(&[], |_, _| {}, |_, _, _| {}),
        Err(JacobianCheckError::EmptySystem)
    );
}

#[test]
fn residual_and_jacobian_failures_are_contained_and_recover() {
    assert!(matches!(
        solve_system_expert(
            &[1.0],
            |_, _| panic!("contained expert residual panic"),
            ExpertNonlinearOptions::default(),
        ),
        Err(NonlinearError::CallbackPanicked)
    ));
    assert!(matches!(
        solve_system_expert(
            &[1.0],
            |_, f| f[0] = f64::NAN,
            ExpertNonlinearOptions::default(),
        ),
        Err(NonlinearError::CallbackReturnedNonFinite { index: 0 })
    ));
    assert!(matches!(
        solve_system_with_jacobian(
            &[1.0],
            |x, f| f[0] = x[0] * x[0] - 2.0,
            |_, _, _| panic!("contained expert Jacobian panic"),
            ExpertNonlinearOptions::default(),
        ),
        Err(NonlinearError::JacobianPanicked)
    ));
    assert!(matches!(
        solve_system_with_jacobian(
            &[1.0],
            |x, f| f[0] = x[0] * x[0] - 2.0,
            |_, _, _| {},
            ExpertNonlinearOptions::default(),
        ),
        Err(NonlinearError::JacobianReturnedNonFinite { row: 0, column: 0 })
    ));
    assert!(scalar_expert().is_ok());
}

#[test]
fn every_cross_family_nested_direction_is_rejected_without_deadlock() {
    let expert = solve_system_expert(
        &[1.0],
        |_, residual| {
            assert!(matches!(
                solve_system_expert(&[1.0], |_, f| f[0] = 0.0, ExpertNonlinearOptions::default()),
                Err(NonlinearError::NestedNativeCallback)
            ));
            assert!(matches!(
                solve_system(&[1.0], |_, f| f[0] = 0.0, NonlinearOptions::default()),
                Err(NonlinearError::NestedNativeCallback)
            ));
            assert!(
                find_root(
                    |x| x - 0.5,
                    RootBracket {
                        lower: 0.0,
                        upper: 1.0
                    },
                    RootOptions::default(),
                )
                .is_err()
            );
            assert!(integrate(|x| x, 0.0, 1.0, IntegrationOptions::default()).is_err());
            residual[0] = 0.0;
        },
        ExpertNonlinearOptions::default(),
    );
    assert!(expert.is_ok());

    let easy = solve_system(
        &[1.0],
        |_, residual| {
            assert!(matches!(
                scalar_expert(),
                Err(NonlinearError::NestedNativeCallback)
            ));
            residual[0] = 0.0;
        },
        NonlinearOptions::default(),
    );
    assert!(easy.is_ok());

    let root = find_root(
        |x| {
            assert!(matches!(
                scalar_expert(),
                Err(NonlinearError::NestedNativeCallback)
            ));
            x - 0.5
        },
        RootBracket {
            lower: 0.0,
            upper: 1.0,
        },
        RootOptions::default(),
    );
    assert!(root.is_ok());

    let quadrature = integrate(
        |x| {
            assert!(matches!(
                scalar_expert(),
                Err(NonlinearError::NestedNativeCallback)
            ));
            x
        },
        0.0,
        1.0,
        IntegrationOptions::default(),
    );
    assert!(quadrature.is_ok());
}

#[test]
fn expert_easy_root_and_quadrature_parallel_calls_serialize() {
    let barrier = Arc::new(Barrier::new(5));
    let expert_barrier = Arc::clone(&barrier);
    let expert = thread::spawn(move || {
        expert_barrier.wait();
        scalar_expert().unwrap().solution[0]
    });
    let easy_barrier = Arc::clone(&barrier);
    let easy = thread::spawn(move || {
        easy_barrier.wait();
        solve_system(
            &[1.0],
            |x, f| f[0] = x[0] * x[0] - 2.0,
            NonlinearOptions::default(),
        )
        .unwrap()
        .solution[0]
    });
    let root_barrier = Arc::clone(&barrier);
    let root = thread::spawn(move || {
        root_barrier.wait();
        find_root(
            |x| x * x - 2.0,
            RootBracket {
                lower: 1.0,
                upper: 2.0,
            },
            RootOptions::default(),
        )
        .unwrap()
        .estimate
    });
    let quadrature_barrier = Arc::clone(&barrier);
    let quadrature = thread::spawn(move || {
        quadrature_barrier.wait();
        integrate(|x| x, 0.0, 1.0, IntegrationOptions::default())
            .unwrap()
            .value
    });
    barrier.wait();
    close(expert.join().unwrap(), 2.0_f64.sqrt(), 2.0e-9);
    close(easy.join().unwrap(), 2.0_f64.sqrt(), 2.0e-9);
    close(root.join().unwrap(), 2.0_f64.sqrt(), 2.0e-9);
    close(quadrature.join().unwrap(), 0.5, 2.0e-12);
}
