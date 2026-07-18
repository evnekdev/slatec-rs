// slatec-safe-example
//! Continuation with session-owned `DDASSL` workspace.
//! Requires `std,external-backend,dassl`.

use slatec::dassl::{DaeOptions, DaeSession, DaeTolerance, ResidualAction};

fn main() {
    let residual = |_: f64, y: &[f64], yp: &[f64], out: &mut [f64]| {
        out[0] = yp[0] + y[0];
        Ok::<_, core::convert::Infallible>(ResidualAction::Continue)
    };
    let mut session = DaeSession::new(
        0.0,
        vec![1.0],
        vec![-1.0],
        residual,
        DaeTolerance::Scalar {
            relative: 1.0e-7,
            absolute: 1.0e-9,
        },
        DaeOptions::default(),
    )
    .unwrap();
    session.advance_to(0.5).unwrap();
    session.advance_to(1.0).unwrap();
    assert!((session.state()[0] - (-1.0_f64).exp()).abs() < 2.0e-5);
}
