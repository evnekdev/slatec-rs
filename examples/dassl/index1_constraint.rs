// slatec-safe-example
//! Index-1 DAE session over SLATEC `DDASSL`.
//! Requires `std,external-backend,dassl`.

use slatec::dassl::{DaeOptions, DaeSession, DaeTolerance, ResidualAction};

fn main() {
    let residual = |_: f64, y: &[f64], yp: &[f64], out: &mut [f64]| {
        out[0] = yp[0] + y[0];
        out[1] = y[1];
        Ok::<_, core::convert::Infallible>(ResidualAction::Continue)
    };
    let mut session = DaeSession::new(
        0.0,
        vec![1.0, 0.0],
        vec![-1.0, 0.0],
        residual,
        DaeTolerance::Scalar {
            relative: 1.0e-6,
            absolute: 1.0e-8,
        },
        DaeOptions::default(),
    )
    .expect("valid initial index-1 pair");
    session.advance_to(1.0).expect("DASSL output");
    assert!((session.state()[0] - (-1.0_f64).exp()).abs() < 2.0e-4);
    assert!(session.state()[1].abs() < 2.0e-6);
}
