#![cfg(all(
    feature = "native-serialization-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

use std::sync::{Arc, Barrier};

use slatec::bounded_least_squares::{
    BoundedLeastSquaresOptions, BoundedLeastSquaresProblem, VariableBounds,
    solve_bounded_least_squares,
};
use slatec::dassl::{DaeOptions, DaeSession, DaeTolerance, ResidualAction};
use slatec::fftpack::RealFftPlan;
use slatec::interpolation::bspline::BSpline;
use slatec::linear_least_squares::{
    MatrixRef, NonnegativeLeastSquaresProblem, VariableConstraint, solve_nonnegative_least_squares,
};
use slatec::linear_programming::{LinearProgram, LpBound, SparseColumns};
use slatec::native_serialization_test_support::{reset, snapshot};
use slatec::ode::{OdeOptions, OdeSession, OdeTolerance, OdeTolerances};
use slatec::pchip::PiecewiseCubicHermite;
use slatec::quadrature::{IntegrationOptions, integrate};
use slatec::roots::{RootBracket, RootOptions, find_root};

fn run_ode() {
    let mut session = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), ()> {
            derivative[0] = -state[0];
            Ok(())
        },
        OdeTolerances {
            relative: 1.0e-8,
            absolute: OdeTolerance::Scalar(1.0e-10),
        },
        OdeOptions::default(),
    )
    .unwrap();
    session.integrate_to(0.2).unwrap();
}

fn run_dassl() {
    let residual = |_: f64, y: &[f64], y_prime: &[f64], output: &mut [f64]| {
        output[0] = y_prime[0] + y[0];
        Ok::<_, core::convert::Infallible>(ResidualAction::Continue)
    };
    let mut session = DaeSession::new(
        0.0,
        vec![1.0],
        vec![-1.0],
        residual,
        DaeTolerance::Scalar {
            relative: 1.0e-6,
            absolute: 1.0e-8,
        },
        DaeOptions::default(),
    )
    .unwrap();
    session.advance_to(0.2).unwrap();
}

fn run_bounded() {
    let values = [1.0_f64, 0.0, 0.0, 1.0];
    let rhs = [-1.0, 2.0];
    let bounds = [VariableBounds::Lower(0.0), VariableBounds::Unbounded];
    let matrix = MatrixRef::column_major(&values, 2, 2, 2).unwrap();
    solve_bounded_least_squares(
        BoundedLeastSquaresProblem {
            matrix,
            rhs: &rhs,
            bounds: &bounds,
        },
        BoundedLeastSquaresOptions::default(),
    )
    .unwrap();
}

fn run_nonnegative() {
    let values = [1.0_f64, 0.0, 0.0, 1.0];
    let rhs = [-1.0, 2.0];
    let constraints = [
        VariableConstraint::Nonnegative,
        VariableConstraint::Nonnegative,
    ];
    let matrix = MatrixRef::column_major(&values, 2, 2, 2).unwrap();
    solve_nonnegative_least_squares(NonnegativeLeastSquaresProblem {
        least_squares_matrix: matrix,
        least_squares_rhs: &rhs,
        equality_matrix: None,
        equality_rhs: None,
        variable_constraints: &constraints,
    })
    .unwrap();
}

fn run_root() {
    find_root(
        |x| x * x - 2.0,
        RootBracket {
            lower: 1.0,
            upper: 2.0,
        },
        RootOptions::default(),
    )
    .unwrap();
}

fn run_quadrature() {
    integrate(|x| x * x, 0.0, 1.0, IntegrationOptions::default()).unwrap();
}

fn run_linear_programming() {
    let matrix =
        SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0]).unwrap();
    let result = LinearProgram::<f64>::new(
        vec![1.0, 2.0],
        matrix,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
    )
    .unwrap()
    .solve()
    .unwrap();
    assert!(result.solution.is_some());
}

fn run_real_fftpack() {
    let mut values = [1.0_f32, 0.0, -1.0, 0.0, 0.5, -0.5, 0.25];
    let mut plan = RealFftPlan::new(values.len()).unwrap();
    plan.forward(&mut values).unwrap();
    plan.backward(&mut values).unwrap();
}

fn run_pchip() {
    let curve = PiecewiseCubicHermite::<f64>::monotone(&[0.0, 0.5, 1.0], &[0.0, 0.3, 1.0]).unwrap();
    assert!(curve.evaluate(0.25).unwrap().is_finite());
}

fn run_bspline() {
    let curve = BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2).unwrap();
    assert!((curve.integrate(0.0, 1.0).unwrap() - 0.5).abs() < 1.0e-12);
}

fn concurrent_pair(left: fn(), right: fn()) {
    let barrier = Arc::new(Barrier::new(3));
    let spawn = |work: fn()| {
        let barrier = Arc::clone(&barrier);
        std::thread::spawn(move || {
            barrier.wait();
            for _ in 0..12 {
                work();
            }
        })
    };
    let left = spawn(left);
    let right = spawn(right);
    barrier.wait();
    left.join().unwrap();
    right.join().unwrap();
}

#[test]
fn different_hosted_families_never_overlap_native_lock_scopes() {
    reset();
    concurrent_pair(run_ode, run_bounded);
    concurrent_pair(run_dassl, run_ode);
    concurrent_pair(run_dassl, run_linear_programming);
    concurrent_pair(run_dassl, run_pchip);
    concurrent_pair(run_dassl, run_bspline);
    concurrent_pair(run_dassl, run_real_fftpack);
    concurrent_pair(run_bounded, run_nonnegative);
    concurrent_pair(run_root, run_quadrature);
    concurrent_pair(run_linear_programming, run_ode);
    concurrent_pair(run_linear_programming, run_bounded);
    concurrent_pair(run_real_fftpack, run_ode);
    concurrent_pair(run_real_fftpack, run_linear_programming);
    concurrent_pair(run_pchip, run_ode);
    concurrent_pair(run_pchip, run_real_fftpack);
    concurrent_pair(run_bspline, run_ode);
    concurrent_pair(run_bspline, run_pchip);
    concurrent_pair(run_bspline, run_real_fftpack);
    let observed = snapshot();
    assert_eq!(observed.active, 0);
    assert_eq!(observed.maximum_active, 1);
    assert!(!observed.owner_present);
}

#[test]
fn nested_non_callback_entry_and_panic_restore_the_lock() {
    reset();
    let result = integrate(
        |x| slatec::special::gamma::gamma(x + 1.0).unwrap(),
        0.0,
        1.0,
        IntegrationOptions::default(),
    )
    .unwrap();
    assert!(result.value.is_finite());
    let nested = snapshot();
    assert!(nested.nested_same_thread > 0);
    assert_eq!(nested.maximum_active, 1);

    let _ = integrate(
        |_x| -> f64 { panic!("contained callback panic") },
        0.0,
        1.0,
        IntegrationOptions::default(),
    );
    let restored = snapshot();
    assert_eq!(restored.active, 0);
    assert_eq!(restored.maximum_active, 1);
    assert!(!restored.owner_present);
    run_root();
}
