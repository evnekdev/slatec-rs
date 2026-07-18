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
use slatec::linear_least_squares::{
    MatrixRef, NonnegativeLeastSquaresProblem, VariableConstraint, solve_nonnegative_least_squares,
};
use slatec::native_serialization_test_support::{reset, snapshot};
use slatec::ode::{OdeOptions, OdeSession, OdeTolerance, OdeTolerances};
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
    concurrent_pair(run_bounded, run_nonnegative);
    concurrent_pair(run_root, run_quadrature);
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
