// slatec-safe-example
// Minimum features: std, external-backend (or another provider), nonlinear-expert.
// Original SLATEC routines: DNSQ and SNSQ with IOPT=1.

use slatec::nonlinear::{
    ExpertNonlinearOptions, solve_system_with_jacobian, solve_system_with_jacobian_f32,
};

fn main() -> Result<(), slatec::nonlinear::NonlinearError> {
    let result = solve_system_with_jacobian(
        &[1.0],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        |x, _, mut jacobian| jacobian.set(0, 0, 2.0 * x[0]).unwrap(),
        ExpertNonlinearOptions::default(),
    )?;
    assert!(result.residual_norm < 1.0e-10);

    let result_f32 = solve_system_with_jacobian_f32(
        &[1.0],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        |x, _, mut jacobian| jacobian.set(0, 0, 2.0 * x[0]).unwrap(),
        ExpertNonlinearOptions::single_precision(),
    )?;
    assert!(result_f32.residual_norm < 1.0e-4);
    Ok(())
}
