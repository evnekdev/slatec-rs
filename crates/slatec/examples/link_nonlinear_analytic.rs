use slatec::nonlinear::{ExpertNonlinearOptions, solve_system_with_jacobian};

fn main() {
    let result = solve_system_with_jacobian(
        &[1.0],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        |x, _, mut jacobian| jacobian.set(0, 0, 2.0 * x[0]).unwrap(),
        ExpertNonlinearOptions::default(),
    )
    .expect("DNSQ analytic-Jacobian link probe");
    assert!(result.residual_norm < 1.0e-8);
}
