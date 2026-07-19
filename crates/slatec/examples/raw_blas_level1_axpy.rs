//! Run with `cargo run -p slatec --example raw_blas_level1_axpy --features source-build,blas` on the supported GNU MinGW target.
//!
//! `slatec` has safe BLAS wrappers; this is deliberately the raw ABI.

use slatec_sys::FortranInteger;

fn main() {
    slatec_src::ensure_linked();
    let mut n: FortranInteger = 3;
    let mut alpha = 2.0_f64;
    let mut inc: FortranInteger = 1;
    let mut x = [1.0_f64, 2.0, 3.0];
    let mut y = [4.0_f64, 5.0, 6.0];
    unsafe {
        slatec_sys::blas::level1::daxpy(
            &mut n,
            &mut alpha,
            x.as_mut_ptr(),
            &mut inc,
            y.as_mut_ptr(),
            &mut inc,
        );
    }
    assert_eq!(y, [6.0, 9.0, 12.0]);
}
