// slatec-safe-example
// Minimum features: std, external-backend (or another provider), nonlinear-expert.
// Original SLATEC routines: DNSQ and SNSQ.

use slatec::nonlinear::{
    ExpertNonlinearOptions, solve_system_expert, solve_system_expert_f32,
};

fn main() -> Result<(), slatec::nonlinear::NonlinearError> {
    let result = solve_system_expert(
        &[1.0],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        ExpertNonlinearOptions::default(),
    )?;
    assert!((result.solution[0] - 2.0_f64.sqrt()).abs() < 1.0e-8);
    assert!(result.residual_norm < 1.0e-10);

    let result_f32 = solve_system_expert_f32(
        &[1.0],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        ExpertNonlinearOptions::single_precision(),
    )?;
    assert!((result_f32.solution[0] - 2.0_f32.sqrt()).abs() < 2.0e-4);
    Ok(())
}
