//! Native-profile example covering the safe BLAS Level 1 operation families.
// slatec-safe-example: every indexed Level 1 function maps to its family case below.

use slatec::blas::level1::*;

fn main() -> Result<(), slatec::blas::BlasError> {
    let mut copied = [0.0; 3];
    dcopy(&[1.0, 2.0, 3.0], &mut copied)?;
    assert_eq!(copied, [1.0, 2.0, 3.0]);
    dcopy_strided(2, &[1.0, 9.0, 2.0], 2, &mut copied, 1)?;

    let (mut x, mut y) = ([1.0, 2.0], [3.0, 4.0]);
    dswap(&mut x, &mut y)?;
    dswap_strided(1, &mut x, -1, &mut y, 1)?;
    dscal(2.0, &mut x)?;
    dscal_strided(1, 0.5, &mut x, 1)?;
    daxpy(2.0, &x, &mut y)?;
    daxpy_strided(1, 1.0, &x, 1, &mut y, 1)?;
    let dot = ddot(&x, &y)?;
    assert!(dot.is_finite());
    assert!(ddot_strided(1, &x, 1, &y, 1)?.is_finite());
    assert!(dnrm2(&x)?.is_finite());
    assert!(dnrm2_strided(1, &x, 1)?.is_finite());
    assert!(dasum(&x)?.is_finite());
    assert!(dasum_strided(1, &x, 1)?.is_finite());
    assert!(idamax(&x)?.is_some());
    assert!(idamax_strided(1, &x, 1)?.is_some());
    drot(1.0, 0.0, &mut x, &mut y)?;
    drot_strided(1, 1.0, 0.0, &mut x, 1, &mut y, 1)?;

    let (mut sx, mut sy) = ([1.0_f32, 2.0], [3.0_f32, 4.0]);
    let mut scopy_out = [0.0_f32; 2];
    scopy(&sx, &mut scopy_out)?;
    scopy_strided(1, &sx, 1, &mut scopy_out, 1)?;
    sswap(&mut sx, &mut sy)?;
    sswap_strided(1, &mut sx, 1, &mut sy, 1)?;
    sscal(2.0, &mut sx)?;
    sscal_strided(1, 0.5, &mut sx, 1)?;
    saxpy(1.0, &sx, &mut sy)?;
    saxpy_strided(1, 1.0, &sx, 1, &mut sy, 1)?;
    assert!(sdot(&sx, &sy)?.is_finite());
    assert!(sdot_strided(1, &sx, 1, &sy, 1)?.is_finite());
    assert!(snrm2(&sx)?.is_finite());
    assert!(snrm2_strided(1, &sx, 1)?.is_finite());
    assert!(sasum(&sx)?.is_finite());
    assert!(sasum_strided(1, &sx, 1)?.is_finite());
    assert!(isamax(&sx)?.is_some());
    assert!(isamax_strided(1, &sx, 1)?.is_some());
    srot(1.0, 0.0, &mut sx, &mut sy)?;
    srot_strided(1, 1.0, 0.0, &mut sx, 1, &mut sy, 1)?;
    Ok(())
}
