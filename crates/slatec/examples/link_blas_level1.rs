fn main() {
    let value = slatec::linear_algebra::blas::level1::ddot(&[1.0, 2.0], &[3.0, 4.0])
        .expect("matching vector lengths");
    assert_eq!(value, 11.0);
}
