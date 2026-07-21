//! Operation-granularity probe for the safe double-precision GEMV wrapper.

fn main() {
    use slatec::linear_algebra::blas::{Transpose, level2::dgemv_contiguous};

    let mut y = [0.0_f64, 0.0];
    dgemv_contiguous(
        Transpose::None,
        2,
        3,
        1.0,
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        &[1.0, 2.0, 3.0],
        0.0,
        &mut y,
    )
    .expect("valid GEMV input");
    assert_eq!(y, [22.0, 28.0]);
}
