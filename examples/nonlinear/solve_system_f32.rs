//! Solve a two-variable nonlinear system with the finite-difference SNSQE driver.
//!
//! Required features: `std`, one native backend, and `nonlinear-easy`.

// slatec-safe-example
use slatec::nonlinear::{NonlinearOptions, solve_system_f32};

fn main() -> Result<(), slatec::nonlinear::NonlinearError> {
    let result = solve_system_f32(
        &[0.8_f32, 2.2_f32],
        |x, residual| {
            residual[0] = x[0] + x[1] - 3.0;
            residual[1] = x[0] * x[0] + x[1] * x[1] - 5.0;
        },
        NonlinearOptions::single_precision(),
    )?;
    assert!((result.solution[0] - 1.0).abs() < 2.0e-4);
    assert!((result.solution[1] - 2.0).abs() < 2.0e-4);
    assert!(result.residual_norm < 4.0e-4);
    Ok(())
}
