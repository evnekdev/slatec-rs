//! Run with `cargo run -p slatec --example raw_blas_level3_gemm --features source-build,blas` on the supported GNU MinGW target.
//!
//! `A`, `B`, and `C` are column-major; each visible character flag has a
//! trailing GNU Fortran hidden length of one.

use core::ffi::c_char;
use slatec_sys::FortranInteger;

fn main() {
    slatec_src::ensure_linked();
    let mut trans = b'N' as c_char;
    let mut n: FortranInteger = 2;
    let mut alpha = 1.0_f64;
    let mut beta = 0.0_f64;
    let mut ld: FortranInteger = 2;
    let mut a = [1.0_f64, 2.0, 3.0, 4.0];
    let mut identity = [1.0_f64, 0.0, 0.0, 1.0];
    let mut c = [0.0_f64; 4];
    unsafe {
        slatec_sys::blas::level3::dgemm(
            &mut trans,
            &mut trans,
            &mut n,
            &mut n,
            &mut n,
            &mut alpha,
            a.as_mut_ptr(),
            &mut ld,
            identity.as_mut_ptr(),
            &mut ld,
            &mut beta,
            c.as_mut_ptr(),
            &mut ld,
            1,
            1,
        );
    }
    assert_eq!(c, a);
}
