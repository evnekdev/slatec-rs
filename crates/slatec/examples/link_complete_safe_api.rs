fn main() {
    let gamma = slatec::special::gamma::gamma(1.0).expect("validated gamma input");
    let dot = slatec::linear_algebra::blas::level1::ddot(&[2.0], &[3.0]).expect("matching vectors");
    assert_eq!(gamma, 1.0);
    assert_eq!(dot, 6.0);
}
