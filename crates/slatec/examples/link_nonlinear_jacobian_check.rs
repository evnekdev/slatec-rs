use slatec::nonlinear::check_jacobian;

fn main() {
    let result = check_jacobian(
        &[1.5],
        |x, residual| residual[0] = x[0] * x[0] - 2.0,
        |x, _, mut jacobian| jacobian.set(0, 0, 2.0 * x[0]).unwrap(),
    )
    .expect("DCKDER link probe");
    assert!(result.scores[0] > 0.5);
}
