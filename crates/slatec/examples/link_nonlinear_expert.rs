use slatec::nonlinear::{ExpertNonlinearOptions, solve_system_expert};

fn main() {
    let result = solve_system_expert(
        &[1.0],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        ExpertNonlinearOptions::default(),
    )
    .expect("DNSQ expert link probe");
    assert!(result.residual_norm < 1.0e-8);
}
