//! Run with `cargo run -p slatec --example raw_blas_level2_triangular_solve --features source-build,blas` on the supported GNU MinGW target.
//!
//! The three trailing `1` values are the hidden GNU Fortran lengths for
//! `UPLO`, `TRANS`, and `DIAG`.

use core::ffi::c_char;
use slatec_sys::FortranInteger;

fn main() {
    slatec_src::ensure_linked();
    let mut uplo = b'U' as c_char;
    let mut trans = b'N' as c_char;
    let mut diag = b'N' as c_char;
    let mut n: FortranInteger = 2;
    let mut lda: FortranInteger = 2;
    let mut inc: FortranInteger = 1;
    let mut a = [2.0_f64, 0.0, 1.0, 3.0]; // upper triangle, column-major
    let mut x = [4.0_f64, 6.0];
    unsafe {
        slatec_sys::blas::level2::dtrsv(
            &mut uplo,
            &mut trans,
            &mut diag,
            &mut n,
            a.as_mut_ptr(),
            &mut lda,
            x.as_mut_ptr(),
            &mut inc,
            1,
            1,
            1,
        );
    }
    assert_eq!(x, [1.0, 2.0]);
}
