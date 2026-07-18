// slatec-safe-example
//! Contained residual error with `DDASSL`; no panic crosses Fortran.
//! Requires `std,external-backend,dassl`.

use slatec::dassl::{DaeOptions, DaeSession, DaeTolerance, ResidualAction};

fn main() {
    let residual = |_: f64, _: &[f64], _: &[f64], _: &mut [f64]| {
        Err::<ResidualAction, _>("application residual failure")
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
    assert!(session.advance_to(1.0).is_err());
}
