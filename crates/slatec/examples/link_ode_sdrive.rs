//! Narrow link probe for the safe `DDRIV3` session family.

use slatec::ode::{OdeOptions, OdeSession, OdeTolerance, OdeTolerances};

fn main() {
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
    // Force the reviewed `DDRIV3` foreign-call path into this narrow link
    // probe; construction alone would not monomorphize the native dispatch.
    let _step = session.integrate_to(1.0).unwrap();
}
