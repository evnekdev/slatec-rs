// slatec-safe-example
// Minimum features: alloc, external-backend (or another provider), nonlinear-jacobian-check.
// Original SLATEC routines: DCKDER and CHKDER.

use slatec::nonlinear::{check_jacobian, check_jacobian_f32};

fn main() -> Result<(), slatec::nonlinear::JacobianCheckError> {
    let checked = check_jacobian(
        &[1.5],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        |x, _, mut jacobian| jacobian.set(0, 0, 2.0 * x[0]).unwrap(),
    )?;
    assert!(checked.scores[0] > 0.5);

    let checked_f32 = check_jacobian_f32(
        &[1.5],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        |x, _, mut jacobian| jacobian.set(0, 0, 2.0 * x[0]).unwrap(),
    )?;
    assert!(checked_f32.scores[0] > 0.5);
    Ok(())
}
