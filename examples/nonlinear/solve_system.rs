//! Solve a two-variable nonlinear system with the finite-difference DNSQE driver.
//!
//! Required features: `std`, one native backend, and `nonlinear-easy`.

// slatec-safe-example
use slatec::nonlinear::{NonlinearOptions, solve_system};

fn main() -> Result<(), slatec::nonlinear::NonlinearError> {
    let result = solve_system(
        &[0.8, 2.2],
        |x, residual| {
            residual[0] = x[0] + x[1] - 3.0;
            residual[1] = x[0] * x[0] + x[1] * x[1] - 5.0;
        },
        NonlinearOptions::default(),
    )?;
    assert!((result.solution[0] - 1.0).abs() < 1.0e-8);
    assert!((result.solution[1] - 2.0).abs() < 1.0e-8);
    assert!(result.residual_norm < 1.0e-8);
    Ok(())
}
