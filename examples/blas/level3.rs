//! Native-profile examples for every exposed real BLAS Level 3 family.
// slatec-safe-example: GEMM, TRMM, TRSM, and SYRK in both precisions.

use slatec::blas::level3::*;
use slatec::blas::{Diagonal, Side, Transpose, Triangle};

fn main() -> Result<(), slatec::blas::BlasError> {
    let a = [1.0, 3.0, 2.0, 4.0];
    let b = [5.0, 7.0, 6.0, 8.0];
    let mut c = [0.0; 4];
    dgemm_contiguous(Transpose::None, Transpose::None, 2, 2, 2, 1.0, &a, &b, 0.0, &mut c)?;
    assert_eq!(c, [19.0, 43.0, 22.0, 50.0]);
    dgemm(Transpose::None, Transpose::None, 2, 2, 2, 1.0, &a, 2, &b, 2, 0.0, &mut c, 2)?;
    dtrmm(Side::Left, Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, 2, 1.0, &a, 2, &mut c, 2)?;
    dtrsm(Side::Left, Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, 2, 1.0, &a, 2, &mut c, 2)?;
    dsyrk(Triangle::Upper, Transpose::None, 2, 2, 1.0, &a, 2, 0.0, &mut c, 2)?;

    let af = [1.0_f32, 3.0, 2.0, 4.0];
    let bf = [5.0_f32, 7.0, 6.0, 8.0];
    let mut cf = [0.0_f32; 4];
    sgemm_contiguous(Transpose::None, Transpose::None, 2, 2, 2, 1.0, &af, &bf, 0.0, &mut cf)?;
    sgemm(Transpose::None, Transpose::None, 2, 2, 2, 1.0, &af, 2, &bf, 2, 0.0, &mut cf, 2)?;
    strmm(Side::Left, Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, 2, 1.0, &af, 2, &mut cf, 2)?;
    strsm(Side::Left, Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, 2, 1.0, &af, 2, &mut cf, 2)?;
    ssyrk(Triangle::Upper, Transpose::None, 2, 2, 1.0, &af, 2, 0.0, &mut cf, 2)?;
    Ok(())
}
