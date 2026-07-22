// slatec-safe-example
//! DASSL residual session with an internally finite-differenced banded matrix.
//! Requires `std,source-build,dassl`.

use slatec::dassl::{DaeJacobianMode, DaeOptions, DaeSession, DaeTolerance, ResidualAction};

fn main() {
    let residual = |_: f64, y: &[f64], yp: &[f64], out: &mut [f64]| {
        out[0] = yp[0] + 40.0 * y[0] - 40.0 * y[1];
        out[1] = yp[1] - 40.0 * y[0] + 80.0 * y[1] - 40.0 * y[2];
        out[2] = yp[2] - 40.0 * y[1] + 40.0 * y[2];
        Ok::<_, core::convert::Infallible>(ResidualAction::Continue)
    };
    let options = DaeOptions {
        jacobian_mode: DaeJacobianMode::FiniteDifferenceBanded {
            lower_bandwidth: 1,
            upper_bandwidth: 1,
        },
        ..DaeOptions::default()
    };
    let mut session = DaeSession::new(
        0.0,
        vec![1.0, 0.0, 0.0],
        vec![-40.0, 40.0, 0.0],
        residual,
        DaeTolerance::Scalar {
            relative: 1.0e-7,
            absolute: 1.0e-9,
        },
        options,
    )
    .expect("valid index-1 initial pair");

    session.advance_to(0.1).expect("DASSL output");
    assert!((session.state().iter().sum::<f64>() - 1.0).abs() < 2.0e-5);
}
