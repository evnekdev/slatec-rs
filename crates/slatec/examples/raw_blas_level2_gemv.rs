//! Run with `cargo run -p slatec --example raw_blas_level2_gemv --features source-build,blas` on the supported GNU MinGW target.
//!
//! Storage is column-major and the trailing `1` is the GNU Fortran hidden
//! length for the one-byte `TRANS` buffer.

use core::ffi::c_char;
use slatec_sys::FortranInteger;

fn main() {
    slatec_src::ensure_linked();
    let mut trans = b'N' as c_char;
    let mut m: FortranInteger = 2;
    let mut n: FortranInteger = 3;
    let mut alpha = 1.0_f64;
    let mut beta = 0.0_f64;
    let mut lda: FortranInteger = 2;
    let mut inc: FortranInteger = 1;
    let mut a = [1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0]; // 2 by 3, column-major
    let mut x = [1.0_f64, 2.0, 3.0];
    let mut y = [0.0_f64, 0.0];
    unsafe {
        slatec_sys::blas::level2::dgemv(
            &mut trans,
            &mut m,
            &mut n,
            &mut alpha,
            a.as_mut_ptr(),
            &mut lda,
            x.as_mut_ptr(),
            &mut inc,
            &mut beta,
            y.as_mut_ptr(),
            &mut inc,
            1,
        );
    }
    assert_eq!(y, [22.0, 28.0]);
}
