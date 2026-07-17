#![cfg(all(
    feature = "least-squares-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native validation for the reviewed GNU MinGW SNLS1E/DNLS1E profile.
//!
//! This is intentionally separate from source-only CI: it executes the
//! original Fortran easy drivers, their finite-difference subsidiaries, and
//! the contained callback bridge against the explicit native provider.

use std::sync::{Arc, Barrier};
use std::thread;

use slatec::least_squares::{
    LeastSquaresError, LeastSquaresOptions, LeastSquaresStatus, least_squares, least_squares_f32,
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
}

#[test]
fn parallel_least_squares_calls_share_native_serialization() {
    let barrier = Arc::new(Barrier::new(3));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let barrier = Arc::clone(&barrier);
        workers.push(thread::spawn(move || {
            barrier.wait();
            linear_fit().unwrap().parameters[1]
        }));
    }
    barrier.wait();
    for worker in workers {
        close(worker.join().unwrap(), 2.0, 2.0e-9);
    }
}
