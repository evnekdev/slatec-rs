//! Narrow link probe for the safe `DDRIV3` session family.

use slatec::ode::{OdeOptions, OdeSession, OdeTolerance, OdeTolerances};

fn main() {
    let _session = OdeSession::new(
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
}
