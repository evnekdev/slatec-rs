//! Operation-granularity probe for the safe single-precision AXPY wrapper.

fn main() {
    let mut y = [3.0_f32, 4.0];
    slatec::blas::level1::saxpy(2.0, &[1.0, 2.0], &mut y).expect("valid AXPY input");
    assert_eq!(y, [5.0, 8.0]);
}
