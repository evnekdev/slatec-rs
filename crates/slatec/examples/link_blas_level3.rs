fn main() {
    use slatec::blas::{Transpose, level3::dgemm_contiguous};
    let mut output = [0.0];
    dgemm_contiguous(
        Transpose::None,
        Transpose::None,
        1,
        1,
        1,
        1.0,
        &[2.0],
        &[3.0],
        0.0,
        &mut output,
    )
    .expect("one-element matrix product");
    assert_eq!(output, [6.0]);
}
