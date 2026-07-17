#![cfg(all(
    feature = "least-squares-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native validation for reviewed GNU MinGW `SNLS1E`/`DNLS1E` and
//! `SNLS1`/`DNLS1` profiles.
//!
//! This is intentionally separate from source-only CI: it executes the
//! original Fortran easy drivers, their finite-difference subsidiaries, and
//! the contained callback bridge against the explicit native provider.

use std::sync::{Arc, Barrier};
use std::thread;

use slatec::least_squares::{
    CovarianceEligibility, CovarianceError, CovarianceOptions, CovarianceScaling,
    ExpertLeastSquaresOptions, LeastSquaresError, LeastSquaresOptions, LeastSquaresScaling,
    LeastSquaresStatus, covariance_from_expert_fit, covariance_from_expert_fit_f32,
    estimate_covariance, estimate_covariance_f32, estimate_covariance_finite_difference,
    estimate_covariance_finite_difference_f32, least_squares, least_squares_expert,
    least_squares_expert_f32, least_squares_f32, least_squares_with_jacobian,
    least_squares_with_jacobian_f32,
};
use slatec::nonlinear::{
    ExpertNonlinearOptions, NonlinearOptions, solve_system, solve_system_expert,
};
use slatec::quadrature::{IntegrationOptions, integrate};
use slatec::roots::{RootBracket, RootOptions, find_root};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn linear_fit() -> Result<slatec::least_squares::LeastSquaresResult<f64>, LeastSquaresError> {
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys = [1.0, 3.0, 5.0, 7.0, 9.0];
    least_squares(
        &[0.2, 0.3],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        LeastSquaresOptions::default(),
    )
}

#[test]
fn easy_drivers_fit_linear_exponential_and_overdetermined_models() {
    let line = linear_fit().unwrap();
    close(line.parameters[0], 1.0, 2.0e-9);
    close(line.parameters[1], 2.0, 2.0e-9);
    assert!(line.residual_norm < 2.0e-9);
    assert!(line.cost < 2.0e-18);
    assert!(line.function_evaluations > 0);
    assert!(matches!(
        line.status,
        LeastSquaresStatus::ConvergedResidual
            | LeastSquaresStatus::ConvergedParameters
            | LeastSquaresStatus::ConvergedResidualAndParameters
            | LeastSquaresStatus::ConvergedOrthogonality
    ));

    let xs = [0.0, 1.0, 2.0, 3.0];
    let ys = [1.5, 3.0, 6.0, 12.0];
    let exponential = least_squares(
        &[1.0, 0.5],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] * (parameters[1] * x).exp() - y;
            }
        },
        LeastSquaresOptions::default(),
    )
    .unwrap();
    close(exponential.parameters[0], 1.5, 3.0e-8);
    close(exponential.parameters[1], core::f64::consts::LN_2, 3.0e-8);
    assert!(exponential.residual_norm < 3.0e-8);

    let xs = [-2.0, -1.0, 0.0, 1.0, 2.0, 3.0];
    let ys = [9.0, 2.0, 1.0, 6.0, 17.0, 34.0];
    let quadratic = least_squares(
        &[0.0, 0.0, 0.0],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x + parameters[2] * x * x - y;
            }
        },
        LeastSquaresOptions::default(),
    )
    .unwrap();
    close(quadratic.parameters[0], 1.0, 2.0e-8);
    close(quadratic.parameters[1], 2.0, 2.0e-8);
    close(quadratic.parameters[2], 3.0, 2.0e-8);
    assert!(quadratic.residual_norm < 2.0e-8);

    let single = least_squares_f32(
        &[0.0_f32, 0.0],
        4,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
                parameters[0] + 3.0 * parameters[1] - 7.0,
            ]);
        },
        LeastSquaresOptions::single_precision(),
    )
    .unwrap();
    close(f64::from(single.parameters[0]), 1.0, 2.0e-3);
    close(f64::from(single.parameters[1]), 2.0, 2.0e-3);
    assert!(single.residual_norm < 3.0e-3);

    let single_noisy = least_squares_f32(
        &[0.0_f32, 0.0],
        4,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.1,
                parameters[0] + 2.0 * parameters[1] - 4.9,
                parameters[0] + 3.0 * parameters[1] - 7.2,
            ]);
        },
        LeastSquaresOptions::single_precision(),
    )
    .unwrap();
    assert!(single_noisy.cost > 0.0);

    let scaled_x = [0.0, 1_000.0, 2_000.0, 3_000.0];
    let scaled_y = [1_000.0, 1_001.0, 1_002.0, 1_003.0];
    let scaled = least_squares(
        &[900.0, 0.01],
        scaled_x.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in scaled_x.iter().zip(scaled_y.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        LeastSquaresOptions::default(),
    )
    .unwrap();
    close(scaled.parameters[0], 1_000.0, 2.0e-7);
    close(scaled.parameters[1], 0.001, 2.0e-10);
}

#[test]
fn a_noisy_overdetermined_fit_has_nonzero_cost() {
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys = [1.0, 3.1, 4.8, 7.2, 8.9];
    let fit = least_squares(
        &[0.0, 0.0],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        LeastSquaresOptions::default(),
    )
    .unwrap();
    assert!(fit.residual_norm > 0.01);
    assert!(fit.cost > 0.0);
    close(
        fit.cost,
        0.5 * fit.residual_norm * fit.residual_norm,
        1.0e-14,
    );
}

#[test]
fn expert_drivers_fit_rectangular_models_with_and_without_jacobians() {
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys = [1.0, 3.1, 4.8, 7.2, 8.9];
    let options = ExpertLeastSquaresOptions::default();
    let finite_difference = least_squares_expert(
        &[0.0, 0.0],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        options,
    )
    .unwrap();
    let analytic = least_squares_with_jacobian(
        &[0.0, 0.0],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        |_, _, mut jacobian| {
            for (row, &x) in xs.iter().enumerate() {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, x).unwrap();
            }
        },
        options,
    )
    .unwrap();
    close(
        finite_difference.parameters[0],
        analytic.parameters[0],
        2.0e-8,
    );
    close(
        finite_difference.parameters[1],
        analytic.parameters[1],
        2.0e-8,
    );
    close(finite_difference.cost, analytic.cost, 2.0e-12);
    assert!(analytic.cost > 0.0);
    assert_eq!(finite_difference.jacobian_evaluations, 0);
    assert!(analytic.jacobian_evaluations > 0);
    assert!(analytic.function_evaluations > 0);

    let exponential_x = [0.0, 0.5, 1.0, 1.5, 2.0];
    let exponential_y = [1.5, 2.121_320_343_6, 3.0, 4.242_640_687_1, 6.0];
    let exponential = least_squares_with_jacobian(
        &[1.0, 0.5],
        exponential_x.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in exponential_x
                .iter()
                .zip(exponential_y.iter())
                .zip(residuals)
            {
                *residual = parameters[0] * (parameters[1] * x).exp() - y;
            }
        },
        |parameters, _, mut jacobian| {
            for (row, &x) in exponential_x.iter().enumerate() {
                let exponential = (parameters[1] * x).exp();
                jacobian.set(row, 0, exponential).unwrap();
                jacobian
                    .set(row, 1, parameters[0] * x * exponential)
                    .unwrap();
            }
        },
        ExpertLeastSquaresOptions::default(),
    )
    .unwrap();
    close(exponential.parameters[0], 1.5, 3.0e-8);
    close(exponential.parameters[1], core::f64::consts::LN_2, 3.0e-8);

    let scale = [1_000.0, 0.001];
    let scaled = least_squares_expert(
        &[900.0, 0.01],
        4,
        |parameters, residuals| {
            for (index, residual) in residuals.iter_mut().enumerate() {
                let x = index as f64 * 1_000.0;
                *residual = parameters[0] + parameters[1] * x - (1_000.0 + index as f64);
            }
        },
        ExpertLeastSquaresOptions {
            scaling: LeastSquaresScaling::User(&scale),
            ..ExpertLeastSquaresOptions::default()
        },
    )
    .unwrap();
    close(scaled.parameters[0], 1_000.0, 3.0e-7);
    close(scaled.parameters[1], 0.001, 3.0e-10);

    let single = least_squares_with_jacobian_f32(
        &[0.0_f32, 0.0],
        4,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
                parameters[0] + 3.0 * parameters[1] - 7.0,
            ])
        },
        |_, _, mut jacobian| {
            for row in 0..4 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, row as f32).unwrap();
            }
        },
        ExpertLeastSquaresOptions::single_precision(),
    )
    .unwrap();
    close(f64::from(single.parameters[0]), 1.0, 2.0e-3);
    close(f64::from(single.parameters[1]), 2.0, 2.0e-3);
    assert!(single.jacobian_evaluations > 0);

    let single_fd = least_squares_expert_f32(
        &[0.0_f32],
        1,
        |parameters, residuals| residuals[0] = parameters[0] - 1.0,
        ExpertLeastSquaresOptions::single_precision(),
    )
    .unwrap();
    close(f64::from(single_fd.parameters[0]), 1.0, 2.0e-3);
    assert_eq!(single_fd.jacobian_evaluations, 0);
}

#[test]
fn expert_validation_callback_failures_and_level_one_statuses_recover() {
    let default_options = ExpertLeastSquaresOptions::default();
    assert!(matches!(
        least_squares_expert(&[1.0, 2.0], 1, |_, _| {}, default_options),
        Err(LeastSquaresError::Underdetermined { .. })
    ));
    assert_eq!(
        least_squares_expert(
            &[1.0],
            1,
            |_, _| {},
            ExpertLeastSquaresOptions {
                maximum_function_evaluations: Some(0),
                ..default_options
            }
        ),
        Err(LeastSquaresError::InvalidMaximumFunctionEvaluations)
    );
    assert_eq!(
        least_squares_expert(
            &[1.0],
            1,
            |_, _| {},
            ExpertLeastSquaresOptions {
                finite_difference_step: Some(-1.0),
                ..default_options
            }
        ),
        Err(LeastSquaresError::InvalidFiniteDifferenceStep)
    );
    assert_eq!(
        least_squares_expert(
            &[1.0],
            1,
            |_, _| {},
            ExpertLeastSquaresOptions {
                step_bound_factor: 0.0,
                ..default_options
            }
        ),
        Err(LeastSquaresError::InvalidStepBoundFactor)
    );
    assert!(matches!(
        least_squares_expert(
            &[1.0],
            1,
            |_, _| panic!("contained expert residual panic"),
            default_options
        ),
        Err(LeastSquaresError::CallbackPanicked)
    ));
    assert!(matches!(
        least_squares_with_jacobian(
            &[1.0],
            1,
            |x, r| r[0] = x[0] - 1.0,
            |_, _, _| panic!("contained expert Jacobian panic"),
            default_options
        ),
        Err(LeastSquaresError::JacobianPanicked)
    ));
    assert!(matches!(
        least_squares_with_jacobian(
            &[1.0],
            1,
            |x, r| r[0] = x[0] - 1.0,
            |_, _, mut j| j.set(0, 0, f64::NAN).unwrap(),
            default_options
        ),
        Err(LeastSquaresError::JacobianReturnedNonFinite { row: 0, column: 0 })
    ));
    let limited = least_squares_expert(
        &[0.0],
        1,
        |x, r| r[0] = x[0] * x[0] - 2.0,
        ExpertLeastSquaresOptions {
            maximum_function_evaluations: Some(1),
            ..default_options
        },
    )
    .unwrap();
    assert_eq!(limited.status, LeastSquaresStatus::MaximumEvaluations);
    assert!(linear_fit().is_ok());
}

#[test]
fn expert_calls_restore_the_legacy_level_one_control() {
    let mut before = 0;
    // SAFETY: the reviewed XGETF ABI writes one valid INTEGER while this
    // serialized native test owns the process-global Fortran runtime.
    unsafe { slatec_sys::legacy_error::xgetf(&mut before) };

    let limited = least_squares_expert(
        &[0.0],
        1,
        |x, residuals| residuals[0] = x[0] * x[0] - 2.0,
        ExpertLeastSquaresOptions {
            maximum_function_evaluations: Some(1),
            ..ExpertLeastSquaresOptions::default()
        },
    );
    assert!(matches!(
        limited,
        Ok(result) if result.status == LeastSquaresStatus::MaximumEvaluations
    ));

    let mut after_completion = 0;
    // SAFETY: see the pre-call XGETF safety rationale above.
    unsafe { slatec_sys::legacy_error::xgetf(&mut after_completion) };
    assert_eq!(after_completion, before);

    assert!(matches!(
        least_squares_with_jacobian(
            &[1.0],
            1,
            |x, residuals| residuals[0] = x[0] - 1.0,
            |_, _, _| panic!("contained Jacobian panic must restore XERROR"),
            ExpertLeastSquaresOptions::default(),
        ),
        Err(LeastSquaresError::JacobianPanicked)
    ));
    let mut after_failure = 0;
    // SAFETY: see the pre-call XGETF safety rationale above.
    unsafe { slatec_sys::legacy_error::xgetf(&mut after_failure) };
    assert_eq!(after_failure, before);
}

#[test]
fn validation_and_callback_failures_are_contained_and_recover() {
    assert_eq!(
        least_squares(&[], 1, |_, _| {}, LeastSquaresOptions::default()),
        Err(LeastSquaresError::EmptyParameters)
    );
    assert_eq!(
        least_squares(&[1.0], 0, |_, _| {}, LeastSquaresOptions::default()),
        Err(LeastSquaresError::EmptyResiduals)
    );
    assert!(matches!(
        least_squares(&[1.0, 2.0], 1, |_, _| {}, LeastSquaresOptions::default()),
        Err(LeastSquaresError::Underdetermined { .. })
    ));
    assert_eq!(
        least_squares(&[f64::NAN], 1, |_, _| {}, LeastSquaresOptions::default()),
        Err(LeastSquaresError::NonFiniteInitialValue { index: 0 })
    );
    assert_eq!(
        least_squares(
            &[1.0],
            1,
            |_, _| {},
            LeastSquaresOptions { tolerance: -1.0 },
        ),
        Err(LeastSquaresError::InvalidTolerance)
    );
    assert!(matches!(
        least_squares(
            &[1.0],
            1,
            |_, _| panic!("contained least-squares callback panic"),
            LeastSquaresOptions::default(),
        ),
        Err(LeastSquaresError::CallbackPanicked)
    ));
    for non_finite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(matches!(
            least_squares(
                &[1.0],
                1,
                move |_, residuals| residuals[0] = non_finite,
                LeastSquaresOptions::default(),
            ),
            Err(LeastSquaresError::CallbackReturnedNonFinite { index: 0 })
        ));
    }
    assert!(linear_fit().is_ok());
}

#[test]
fn nested_callback_families_are_rejected_without_deadlock() {
    let nested_least_squares = least_squares(
        &[0.0],
        1,
        |_, residuals| {
            assert!(matches!(
                least_squares(
                    &[0.0],
                    1,
                    |_, inner_residuals| inner_residuals[0] = 0.0,
                    LeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            residuals[0] = 0.0;
        },
        LeastSquaresOptions::default(),
    );
    assert!(nested_least_squares.is_ok());

    let nested_expert_least_squares = least_squares_expert(
        &[0.0],
        1,
        |_, residuals| {
            assert!(matches!(
                least_squares_expert(
                    &[0.0],
                    1,
                    |_, inner_residuals| inner_residuals[0] = 0.0,
                    ExpertLeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            residuals[0] = 0.0;
        },
        ExpertLeastSquaresOptions::default(),
    );
    assert!(nested_expert_least_squares.is_ok());

    let nested_covariance = estimate_covariance(
        &[0.0],
        2,
        |_, residuals| {
            assert!(matches!(
                least_squares(
                    &[0.0],
                    1,
                    |_, inner_residuals| inner_residuals[0] = 0.0,
                    LeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            residuals.fill(0.0);
        },
        |_, _, mut jacobian| {
            jacobian.set(0, 0, 1.0).unwrap();
            jacobian.set(1, 0, 1.0).unwrap();
        },
        CovarianceOptions::default(),
    );
    assert!(nested_covariance.is_ok());

    let nested_covariance_from_easy = least_squares(
        &[0.0],
        1,
        |_, residuals| {
            assert!(matches!(
                estimate_covariance(
                    &[0.0],
                    2,
                    |_, inner_residuals| inner_residuals.fill(0.0),
                    |_, _, mut jacobian| {
                        jacobian.set(0, 0, 1.0).unwrap();
                        jacobian.set(1, 0, 1.0).unwrap();
                    },
                    CovarianceOptions::default(),
                ),
                Err(CovarianceError::NestedNativeCallback)
            ));
            residuals[0] = 0.0;
        },
        LeastSquaresOptions::default(),
    );
    assert!(nested_covariance_from_easy.is_ok());

    let nested_root = least_squares(
        &[0.0],
        1,
        |_, residuals| {
            assert!(matches!(
                find_root(
                    |x| x * x - 2.0,
                    RootBracket {
                        lower: 1.0,
                        upper: 2.0
                    },
                    RootOptions::default(),
                ),
                Err(slatec::roots::RootError::NestedNativeCallback)
            ));
            residuals[0] = 0.0;
        },
        LeastSquaresOptions::default(),
    );
    assert!(nested_root.is_ok());

    let nested_quadrature = least_squares(
        &[0.0],
        1,
        |_, residuals| {
            assert!(matches!(
                integrate(|x| x, 0.0, 1.0, IntegrationOptions::default()),
                Err(slatec::quadrature::IntegrationError::NestedIntegration)
            ));
            residuals[0] = 0.0;
        },
        LeastSquaresOptions::default(),
    );
    assert!(nested_quadrature.is_ok());

    let nested_easy = least_squares(
        &[0.0],
        1,
        |_, residuals| {
            assert!(matches!(
                solve_system(&[1.0], |_, f| f[0] = 0.0, NonlinearOptions::default()),
                Err(slatec::nonlinear::NonlinearError::NestedNativeCallback)
            ));
            assert!(matches!(
                solve_system_expert(&[1.0], |_, f| f[0] = 0.0, ExpertNonlinearOptions::default(),),
                Err(slatec::nonlinear::NonlinearError::NestedNativeCallback)
            ));
            residuals[0] = 0.0;
        },
        LeastSquaresOptions::default(),
    );
    assert!(nested_easy.is_ok());

    let root = find_root(
        |x| {
            assert!(matches!(
                least_squares(&[0.0], 1, |_, r| r[0] = 0.0, LeastSquaresOptions::default()),
                Err(LeastSquaresError::NestedNativeCallback)
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

    let expert_root = find_root(
        |x| {
            assert!(matches!(
                least_squares_expert(
                    &[0.0],
                    1,
                    |_, residuals| residuals[0] = 0.0,
                    ExpertLeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            x - 0.5
        },
        RootBracket {
            lower: 0.0,
            upper: 1.0,
        },
        RootOptions::default(),
    );
    assert!(expert_root.is_ok());

    let quadrature = integrate(
        |x| {
            assert!(matches!(
                least_squares(
                    &[0.0],
                    1,
                    |_, residuals| residuals[0] = 0.0,
                    LeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            x
        },
        0.0,
        1.0,
        IntegrationOptions::default(),
    );
    assert!(quadrature.is_ok());

    let expert_quadrature = integrate(
        |x| {
            assert!(matches!(
                least_squares_expert(
                    &[0.0],
                    1,
                    |_, residuals| residuals[0] = 0.0,
                    ExpertLeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            x
        },
        0.0,
        1.0,
        IntegrationOptions::default(),
    );
    assert!(expert_quadrature.is_ok());

    let easy = solve_system(
        &[1.0],
        |x, residuals| {
            assert!(matches!(
                least_squares(
                    &[0.0],
                    1,
                    |_, inner_residuals| inner_residuals[0] = 0.0,
                    LeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            residuals[0] = x[0] - 1.0;
        },
        NonlinearOptions::default(),
    );
    assert!(easy.is_ok());

    let expert = solve_system_expert(
        &[1.0],
        |x, residuals| {
            assert!(matches!(
                least_squares(
                    &[0.0],
                    1,
                    |_, inner_residuals| inner_residuals[0] = 0.0,
                    LeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            residuals[0] = x[0] - 1.0;
        },
        ExpertNonlinearOptions::default(),
    );
    assert!(expert.is_ok());

    let expert_system = solve_system_expert(
        &[1.0],
        |x, residuals| {
            assert!(matches!(
                least_squares_expert(
                    &[0.0],
                    1,
                    |_, inner_residuals| inner_residuals[0] = 0.0,
                    ExpertLeastSquaresOptions::default(),
                ),
                Err(LeastSquaresError::NestedNativeCallback)
            ));
            residuals[0] = x[0] - 1.0;
        },
        ExpertNonlinearOptions::default(),
    );
    assert!(expert_system.is_ok());
}

#[test]
fn parallel_least_squares_calls_share_native_serialization() {
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || {
            barrier.wait();
            least_squares_expert(
                &[0.0, 0.0],
                3,
                |parameters, residuals| {
                    residuals.copy_from_slice(&[
                        parameters[0] - 1.0,
                        parameters[0] + parameters[1] - 3.0,
                        parameters[0] + 2.0 * parameters[1] - 5.0,
                    ]);
                },
                ExpertLeastSquaresOptions::default(),
            )
            .unwrap()
            .parameters[1]
        }));
    }
    barrier.wait();
    for worker in workers {
        close(worker.join().unwrap(), 2.0, 2.0e-9);
    }
}

#[test]
fn parallel_covariance_calls_share_native_serialization() {
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || {
            barrier.wait();
            estimate_covariance_finite_difference(
                &[1.1, 1.96],
                5,
                |parameters, residuals| {
                    for (row, residual) in residuals.iter_mut().enumerate() {
                        *residual = parameters[0] + parameters[1] * row as f64
                            - [1.1, 2.9, 5.2, 7.1, 8.8][row];
                    }
                },
                CovarianceOptions::default(),
            )
            .unwrap()
            .variance_scale
        }));
    }
    barrier.wait();
    for worker in workers {
        close(worker.join().unwrap(), 0.092 / 3.0, 2.0e-12);
    }
}

#[test]
fn covariance_matches_the_linear_jtj_inverse_in_both_jacobian_modes_and_precisions() {
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let ys = [1.1, 2.9, 5.2, 7.1, 8.8];
    let expected = [
        0.0184,
        -0.006_133_333_333_333_333,
        -0.006_133_333_333_333_333,
        0.003_066_666_666_666_667,
    ];
    let analytic = estimate_covariance(
        &[1.1, 1.96],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        |_, _, mut jacobian| {
            for (row, &x) in xs.iter().enumerate() {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, x).unwrap();
            }
        },
        CovarianceOptions::default(),
    )
    .unwrap();
    assert_eq!(analytic.rank, 2);
    assert_eq!(analytic.permutation, vec![0, 1]);
    assert_eq!(analytic.get(0, 1), analytic.get(1, 0));
    close(analytic.residual_sum_of_squares, 0.092, 2.0e-14);
    close(analytic.variance_scale, 0.092 / 3.0, 2.0e-14);
    for (&actual, &reference) in analytic.covariance.iter().zip(expected.iter()) {
        close(actual, reference, 3.0e-11);
    }
    let standard_errors = analytic.standard_errors().unwrap();
    close(standard_errors[0], expected[0].sqrt(), 3.0e-12);
    let correlation = analytic.correlation_matrix().unwrap();
    close(correlation[0], 1.0, 2.0e-12);
    close(correlation[3], 1.0, 2.0e-12);

    let finite_difference = estimate_covariance_finite_difference(
        &[1.1, 1.96],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x - y;
            }
        },
        CovarianceOptions::default(),
    )
    .unwrap();
    for (&actual, &reference) in finite_difference.covariance.iter().zip(expected.iter()) {
        close(actual, reference, 2.0e-8);
    }

    let single = estimate_covariance_f32(
        &[1.1_f32, 1.96],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x as f32 - y as f32;
            }
        },
        |_, _, mut jacobian| {
            for (row, &x) in xs.iter().enumerate() {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, x as f32).unwrap();
            }
        },
        CovarianceOptions::default(),
    )
    .unwrap();
    for (&actual, &reference) in single.covariance.iter().zip(expected.iter()) {
        close(f64::from(actual), reference, 2.0e-5);
    }
    let single_fd = estimate_covariance_finite_difference_f32(
        &[1.1_f32, 1.96],
        xs.len(),
        |parameters, residuals| {
            for ((&x, &y), residual) in xs.iter().zip(ys.iter()).zip(residuals) {
                *residual = parameters[0] + parameters[1] * x as f32 - y as f32;
            }
        },
        CovarianceOptions::default(),
    )
    .unwrap();
    assert_eq!(single_fd.rank, 2);
}

#[test]
fn covariance_handles_zero_residuals_square_scaling_and_rank_deficiency() {
    let exact = estimate_covariance(
        &[1.0, 2.0],
        3,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
            ]);
        },
        |_, _, mut jacobian| {
            for row in 0..3 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, row as f64).unwrap();
            }
        },
        CovarianceOptions::default(),
    )
    .unwrap();
    assert_eq!(exact.residual_sum_of_squares, 0.0);
    assert!(exact.covariance.iter().all(|value| *value == 0.0));
    assert!(matches!(
        exact.correlation_matrix(),
        Err(CovarianceError::ZeroVarianceDiagonal { .. })
    ));

    assert!(matches!(
        estimate_covariance(
            &[1.0, 2.0],
            2,
            |parameters, residuals| residuals
                .copy_from_slice(&[parameters[0] - 1.0, parameters[0] + parameters[1] - 3.0]),
            |_, _, mut jacobian| {
                jacobian.set(0, 0, 1.0).unwrap();
                jacobian.set(0, 1, 0.0).unwrap();
                jacobian.set(1, 0, 1.0).unwrap();
                jacobian.set(1, 1, 1.0).unwrap();
            },
            CovarianceOptions::default(),
        ),
        Err(CovarianceError::NonPositiveDegreesOfFreedom { .. })
    ));
    let square_native = estimate_covariance(
        &[1.0, 2.0],
        2,
        |parameters, residuals| {
            residuals.copy_from_slice(&[parameters[0] - 1.0, parameters[0] + parameters[1] - 3.0])
        },
        |_, _, mut jacobian| {
            jacobian.set(0, 0, 1.0).unwrap();
            jacobian.set(0, 1, 0.0).unwrap();
            jacobian.set(1, 0, 1.0).unwrap();
            jacobian.set(1, 1, 1.0).unwrap();
        },
        CovarianceOptions {
            scaling: CovarianceScaling::Native,
        },
    )
    .unwrap();
    assert_eq!(square_native.variance_scale, 1.0);
    close(square_native.covariance[0], 1.0, 2.0e-12);
    close(square_native.covariance[1], -1.0, 2.0e-12);
    close(square_native.covariance[3], 2.0, 2.0e-12);

    let mut before = 0;
    // SAFETY: this serialized native test reads the reviewed process-global
    // XERROR control before a singular covariance call.
    unsafe { slatec_sys::legacy_error::xgetf(&mut before) };
    let singular = estimate_covariance(
        &[1.0, 1.0],
        3,
        |parameters, residuals| {
            for (row, residual) in residuals.iter_mut().enumerate() {
                *residual = parameters[0] + parameters[1] - (row as f64 + 1.0);
            }
        },
        |_, _, mut jacobian| {
            for row in 0..3 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, 1.0).unwrap();
            }
        },
        CovarianceOptions::default(),
    );
    assert!(matches!(
        singular,
        Err(CovarianceError::RankDeficient { parameter_count: 2 })
    ));
    let mut after = 0;
    // SAFETY: DCOV's scoped XERROR policy must restore this value on INFO=2.
    unsafe { slatec_sys::legacy_error::xgetf(&mut after) };
    assert_eq!(after, before);
}

#[test]
fn covariance_callbacks_and_fit_adapter_are_contained_and_recover() {
    assert!(matches!(
        estimate_covariance(
            &[1.0],
            2,
            |_, _| panic!("contained covariance residual panic"),
            |_, _, mut jacobian| {
                jacobian.set(0, 0, 1.0).unwrap();
                jacobian.set(1, 0, 1.0).unwrap();
            },
            CovarianceOptions::default(),
        ),
        Err(CovarianceError::CallbackPanicked)
    ));
    assert!(matches!(
        estimate_covariance_finite_difference(
            &[1.0],
            2,
            |_, residuals| {
                residuals[0] = f64::NAN;
                residuals[1] = 0.0;
            },
            CovarianceOptions::default(),
        ),
        Err(CovarianceError::CallbackReturnedNonFinite { index: 0 })
    ));
    assert!(matches!(
        estimate_covariance(
            &[1.0],
            2,
            |parameters, residuals| {
                residuals[0] = parameters[0];
                residuals[1] = parameters[0];
            },
            |_, _, mut jacobian| {
                jacobian.set(0, 0, f64::NAN).unwrap();
                jacobian.set(1, 0, 1.0).unwrap();
            },
            CovarianceOptions::default(),
        ),
        Err(CovarianceError::JacobianReturnedNonFinite { row: 0, column: 0 })
    ));
    let fit = least_squares_with_jacobian(
        &[0.0, 0.0],
        3,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
            ])
        },
        |_, _, mut jacobian| {
            for row in 0..3 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, row as f64).unwrap();
            }
        },
        ExpertLeastSquaresOptions::default(),
    )
    .unwrap();
    let from_fit = covariance_from_expert_fit(
        &fit,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
            ])
        },
        |_, _, mut jacobian| {
            for row in 0..3 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, row as f64).unwrap();
            }
        },
        CovarianceOptions::default(),
        CovarianceEligibility::ConvergedOnly,
    )
    .unwrap();
    assert_eq!(from_fit.rank, 2);
    let fit_f32 = least_squares_with_jacobian_f32(
        &[0.0, 0.0],
        3,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
            ])
        },
        |_, _, mut jacobian| {
            for row in 0..3 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, row as f32).unwrap();
            }
        },
        ExpertLeastSquaresOptions::single_precision(),
    )
    .unwrap();
    let from_fit_f32 = covariance_from_expert_fit_f32(
        &fit_f32,
        |parameters, residuals| {
            residuals.copy_from_slice(&[
                parameters[0] - 1.0,
                parameters[0] + parameters[1] - 3.0,
                parameters[0] + 2.0 * parameters[1] - 5.0,
            ])
        },
        |_, _, mut jacobian| {
            for row in 0..3 {
                jacobian.set(row, 0, 1.0).unwrap();
                jacobian.set(row, 1, row as f32).unwrap();
            }
        },
        CovarianceOptions::default(),
        CovarianceEligibility::ConvergedOnly,
    )
    .unwrap();
    assert_eq!(from_fit_f32.rank, 2);
    assert!(
        estimate_covariance_finite_difference(
            &[1.1, 1.96],
            5,
            |parameters, residuals| {
                for (row, residual) in residuals.iter_mut().enumerate() {
                    *residual =
                        parameters[0] + parameters[1] * row as f64 - [1.1, 2.9, 5.2, 7.1, 8.8][row];
                }
            },
            CovarianceOptions::default(),
        )
        .is_ok()
    );
}
