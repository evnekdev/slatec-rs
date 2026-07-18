// slatec-safe-example
//! Scalar-tolerance `SDASSL` DAE session with a purely algebraic component.
//! Requires `std,external-backend,dassl`.

use slatec::dassl::{DaeOptions, DaeSession, DaeTolerance, ResidualAction};

fn main() {
    let residual = |_: f32, y: &[f32], yp: &[f32], out: &mut [f32]| {
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
            relative: 2.0e-4,
            absolute: 2.0e-6,
        },
        DaeOptions::default(),
    )
    .expect("valid DAE");
    session.advance_to(0.5).expect("DASSL output");
    assert!(session.state()[1].abs() < 2.0e-3);
}
