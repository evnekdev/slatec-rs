// slatec-safe-example
//! Exponential decay with safe `DDRIV3` session ownership.
//!
//! Run with `--features std,external-backend,ode-sdrive-expert` and a linked
//! reviewed SLATEC backend. This solves a linear ODE, not a least-squares fit.

use slatec::ode::{OdeOptions, OdeSession, OdeStatus, OdeTolerance, OdeTolerances};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tolerances = OdeTolerances {
        relative: 1.0e-8,
        absolute: OdeTolerance::Scalar(1.0e-10),
    };
    let mut session = OdeSession::new(
        0.0_f64,
        vec![1.0],
        |_time, state, derivative| -> Result<(), &'static str> {
            derivative[0] = -2.0 * state[0];
            Ok(())
        },
        tolerances,
        OdeOptions::default(),
    )?;
    let result = session.integrate_to(1.0)?;
    assert_eq!(result.status, OdeStatus::ReachedTarget);
    assert!((session.state()[0] - (-2.0_f64).exp()).abs() < 2.0e-6);
    Ok(())
}
