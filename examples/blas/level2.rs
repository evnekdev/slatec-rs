//! Native-profile examples for every exposed real BLAS Level 2 family.
// slatec-safe-example: GEMV, GER, SYMV, TRMV, and TRSV in both precisions.

use slatec::blas::level2::*;
use slatec::blas::{Diagonal, Transpose, Triangle};

fn main() -> Result<(), slatec::blas::BlasError> {
    let a = [1.0, 3.0, 2.0, 4.0];
    let mut y = [0.0, 0.0];
    dgemv_contiguous(Transpose::None, 2, 2, 1.0, &a, &[1.0, 1.0], 0.0, &mut y)?;
    assert_eq!(y, [3.0, 7.0]);
    dgemv(Transpose::None, 2, 2, 1.0, &a, 2, &[1.0, 1.0], 1, 0.0, &mut y, 1)?;
    let mut update = [0.0; 4];
    dger(2, 2, 1.0, &[1.0, 2.0], 1, &[3.0, 4.0], 1, &mut update, 2)?;
    dsymv(Triangle::Upper, 2, 1.0, &a, 2, &[1.0, 1.0], 1, 0.0, &mut y, 1)?;
    dtrmv(Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, &a, 2, &mut y, 1)?;
    dtrsv(Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, &a, 2, &mut y, 1)?;

    let af = [1.0_f32, 3.0, 2.0, 4.0];
    let mut yf = [0.0_f32, 0.0];
    sgemv_contiguous(Transpose::None, 2, 2, 1.0, &af, &[1.0, 1.0], 0.0, &mut yf)?;
    sgemv(Transpose::None, 2, 2, 1.0, &af, 2, &[1.0, 1.0], 1, 0.0, &mut yf, 1)?;
    let mut updatef = [0.0_f32; 4];
    sger(2, 2, 1.0, &[1.0, 2.0], 1, &[3.0, 4.0], 1, &mut updatef, 2)?;
    ssymv(Triangle::Upper, 2, 1.0, &af, 2, &[1.0, 1.0], 1, 0.0, &mut yf, 1)?;
    strmv(Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, &af, 2, &mut yf, 1)?;
    strsv(Triangle::Upper, Transpose::None, Diagonal::NonUnit, 2, &af, 2, &mut yf, 1)?;
    Ok(())
}
