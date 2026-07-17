// slatec-safe-example
//! Safe error termination from an `SDRIV3` RHS callback.
//!
//! Required Cargo features: `std,external-backend,ode-sdrive-expert`.

use slatec::ode::{OdeError, OdeOptions, OdeSession, OdeTolerance, OdeTolerances};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut calls = 0_usize;
    let mut session = OdeSession::new(
        0.0_f32,
        vec![1.0],
        move |_time, _state, derivative| -> Result<(), &'static str> {
            calls += 1;
            if calls == 3 {
                return Err("deliberate callback stop");
            }
            derivative[0] = -1.0;
            Ok(())
        },
        OdeTolerances {
            relative: 1.0e-5,
            absolute: OdeTolerance::Scalar(1.0e-7),
        },
        OdeOptions::default(),
    )?;
    assert!(matches!(session.integrate_to(1.0), Err(OdeError::Callback(_))));
    Ok(())
}
