// slatec-safe-example
//! BDF integration with a checked analytic banded SDRIV3 Jacobian.
//! Requires `std,source-build,ode-sdrive-expert`.

use slatec::ode::{OdeIteration, OdeMethod, OdeOptions, OdeSession, OdeTolerance, OdeTolerances};

fn main() {
    let options = OdeOptions {
        method: OdeMethod::Bdf,
        maximum_order: None,
        maximum_steps: None,
        maximum_step: None,
        iteration: OdeIteration::AnalyticBanded {
            lower_bandwidth: 1,
            upper_bandwidth: 1,
        },
    };
    let mut session = OdeSession::new_with_jacobian(
        0.0_f64,
        vec![1.0, 0.0, 0.0],
        |_, state, derivative| {
            derivative[0] = -40.0 * state[0] + 40.0 * state[1];
            derivative[1] = 40.0 * state[0] - 80.0 * state[1] + 40.0 * state[2];
            derivative[2] = 40.0 * state[1] - 40.0 * state[2];
            Ok::<_, ()>(())
        },
        |_, _, mut jacobian| {
            jacobian.set(0, 0, -40.0).expect("diagonal is in band");
            jacobian.set(0, 1, 40.0).expect("upper diagonal is in band");
            jacobian.set(1, 0, 40.0).expect("lower diagonal is in band");
            jacobian.set(1, 1, -80.0).expect("diagonal is in band");
            jacobian.set(1, 2, 40.0).expect("upper diagonal is in band");
            jacobian.set(2, 1, 40.0).expect("lower diagonal is in band");
            jacobian.set(2, 2, -40.0).expect("diagonal is in band");
            Ok::<_, ()>(())
        },
        OdeTolerances {
            relative: 1.0e-8,
            absolute: OdeTolerance::Scalar(1.0e-10),
        },
        options,
    )
    .expect("valid checked SDRIV3 session");

    session.integrate_to(0.1).expect("BDF output");
    assert!((session.state().iter().sum::<f64>() - 1.0).abs() < 1.0e-7);
}
