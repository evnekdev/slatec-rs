//! Run with `cargo run -p slatec --example raw_blas_complex --features source-build,blas` on the supported GNU MinGW target.
//!
//! `Complex32` is the reviewed GNU Fortran `COMPLEX` storage record.  This
//! direct raw call still needs the two hidden character lengths.

use core::ffi::c_char;
use slatec_sys::{Complex32, FortranInteger};

fn main() {
    slatec_src::ensure_linked();
    let mut trans = b'N' as c_char;
    let mut n: FortranInteger = 1;
    let mut alpha = Complex32 { re: 1.0, im: 0.0 };
    let mut beta = Complex32 { re: 0.0, im: 0.0 };
    let mut a = [Complex32 { re: 2.0, im: 3.0 }];
    let mut b = [Complex32 { re: 1.0, im: 0.0 }];
    let mut c = [Complex32::default()];
    unsafe {
        slatec_sys::blas::level3::cgemm(
            &mut trans,
            &mut trans,
            &mut n,
            &mut n,
            &mut n,
            &mut alpha,
            a.as_mut_ptr(),
            &mut n,
            b.as_mut_ptr(),
            &mut n,
            &mut beta,
            c.as_mut_ptr(),
            &mut n,
            1,
            1,
        );
    }
    assert_eq!(c, a);
}
