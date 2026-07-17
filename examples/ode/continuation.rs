// slatec-safe-example
//! Same-direction continuation over `DDRIV3`'s owned native workspace.
//!
//! Required Cargo features: `std,external-backend,ode-sdrive-expert`.

use slatec::ode::{OdeOptions, OdeSession, OdeStatus, OdeTolerance, OdeTolerances};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut session = OdeSession::new(
        0.0_f64,
        vec![0.0],
        |_time, _state, derivative| -> Result<(), &'static str> {
            derivative[0] = 1.0;
            Ok(())
        },
        OdeTolerances {
            relative: 1.0e-9,
            absolute: OdeTolerance::Scalar(1.0e-11),
        },
        OdeOptions::default(),
    )?;
    assert_eq!(session.integrate_to(0.25)?.status, OdeStatus::ReachedTarget);
    assert_eq!(session.integrate_to(1.0)?.status, OdeStatus::ReachedTarget);
    assert!((session.time() - 1.0).abs() < 1.0e-12);
    assert!((session.state()[0] - 1.0).abs() < 1.0e-7);
    Ok(())
}
