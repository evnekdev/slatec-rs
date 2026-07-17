// slatec-safe-example
//! Harmonic oscillator using an `SDRIV3` safe session.
//!
//! Required Cargo features: `std,external-backend,ode-sdrive-expert`.

use slatec::ode::{OdeOptions, OdeSession, OdeStatus, OdeTolerance, OdeTolerances};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = OdeSession::new(
        0.0_f32,
        vec![1.0, 0.0],
        |_time, state, derivative| -> Result<(), &'static str> {
            derivative[0] = state[1];
            derivative[1] = -state[0];
            Ok(())
        },
        OdeTolerances {
            relative: 1.0e-5,
            absolute: OdeTolerance::Vector(vec![1.0e-7, 1.0e-7]),
        },
        OdeOptions::default(),
    )?;
    let result = session.integrate_to(core::f32::consts::FRAC_PI_2)?;
    assert_eq!(result.status, OdeStatus::ReachedTarget);
    assert!(session.state()[0].abs() < 2.0e-3);
    assert!((session.state()[1] + 1.0).abs() < 2.0e-3);
    Ok(())
}
