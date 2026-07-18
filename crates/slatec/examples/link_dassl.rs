//! Narrow native-link probe for the reviewed `DDASSL` closure.

use slatec::dassl::{DaeOptions, DaeSession, DaeTolerance, ResidualAction};

fn main() {
    let residual = |_: f64, y: &[f64], yp: &[f64], output: &mut [f64]| {
        output[0] = yp[0] + y[0];
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
    .expect("valid DASSL initial state");
    let _ = session.advance_to(0.1).expect("DDASSL");
}
