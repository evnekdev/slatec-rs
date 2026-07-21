#![cfg(all(
    feature = "blas-level1-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native tests are deliberately profile- and environment-gated. Run them
//! only after `build-native-ffi --offline` has produced verified local output:
//!
//! ```text
//! $env:SLATEC_NATIVE_LIB_DIR = "..."
//! $env:SLATEC_GFORTRAN_RUNTIME_DIR = "..."
//! cargo test -p slatec --features blas-level1-native-tests --target x86_64-pc-windows-gnu
//! ```

use slatec::linear_algebra::blas::level1::*;

fn assert_close_f64(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "expected {expected}, got {actual}"
    );
}

fn assert_close_f32(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 1.0e-5,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn double_precision_wrappers_match_small_reference_cases() {
    let x = [1.0, 2.0, 3.0];
    let mut y = [0.0; 3];
    dcopy(&x, &mut y).unwrap();
    assert_eq!(y, x);

    let mut a = [1.0, 2.0];
    let mut b = [3.0, 4.0];
    dswap(&mut a, &mut b).unwrap();
    assert_eq!(a, [3.0, 4.0]);
    assert_eq!(b, [1.0, 2.0]);

    dscal(2.0, &mut y).unwrap();
    assert_eq!(y, [2.0, 4.0, 6.0]);
    daxpy(-0.5, &x, &mut y).unwrap();
    assert_eq!(y, [1.5, 3.0, 4.5]);
    assert_close_f64(ddot(&x, &y).unwrap(), 21.0);
    assert_close_f64(dnrm2(&[3.0, 4.0]).unwrap(), 5.0);
    assert_close_f64(dasum(&[-1.0, 2.0, -3.0]).unwrap(), 6.0);
    assert_eq!(idamax(&[1.0, -5.0, 2.0]).unwrap(), Some(1));

    let mut rx = [1.0, 0.0];
    let mut ry = [0.0, 1.0];
    drot(0.0, 1.0, &mut rx, &mut ry).unwrap();
    assert_eq!(rx, [0.0, 1.0]);
    assert_eq!(ry, [-1.0, 0.0]);
}

#[test]
fn single_precision_wrappers_match_small_reference_cases() {
    let x = [1.0_f32, 2.0, 3.0];
    let mut y = [0.0_f32; 3];
    scopy(&x, &mut y).unwrap();
    assert_eq!(y, x);

    let mut a = [1.0_f32, 2.0];
    let mut b = [3.0_f32, 4.0];
    sswap(&mut a, &mut b).unwrap();
    assert_eq!(a, [3.0, 4.0]);
    assert_eq!(b, [1.0, 2.0]);

    sscal(2.0, &mut y).unwrap();
    assert_eq!(y, [2.0, 4.0, 6.0]);
    saxpy(-0.5, &x, &mut y).unwrap();
    assert_eq!(y, [1.5, 3.0, 4.5]);
    assert_close_f32(sdot(&x, &y).unwrap(), 21.0);
    assert_close_f32(snrm2(&[3.0, 4.0]).unwrap(), 5.0);
    assert_close_f32(sasum(&[-1.0, 2.0, -3.0]).unwrap(), 6.0);
    assert_eq!(isamax(&[1.0, -5.0, 2.0]).unwrap(), Some(1));

    let mut rx = [1.0_f32, 0.0];
    let mut ry = [0.0_f32, 1.0];
    srot(0.0, 1.0, &mut rx, &mut ry).unwrap();
    assert_eq!(rx, [0.0, 1.0]);
    assert_eq!(ry, [-1.0, 0.0]);

    let px = [1.0_f32, 0.0, 2.0, 0.0, 3.0];
    let mut py = [0.0_f32; 5];
    scopy_strided(3, &px, 2, &mut py, 2).unwrap();
    assert_eq!(py, px);
    let mut q = [4.0_f32, 0.0, 5.0, 0.0, 6.0];
    sswap_strided(3, &mut py, 2, &mut q, 2).unwrap();
    assert_eq!(py, [4.0, 0.0, 5.0, 0.0, 6.0]);
    sscal_strided(3, 0.5, &mut py, 2).unwrap();
    saxpy_strided(3, 1.0, &px, 2, &mut py, 2).unwrap();
    assert_close_f32(sdot_strided(3, &px, 2, &py, 2).unwrap(), 30.0);
    assert_close_f32(snrm2_strided(3, &px, 2).unwrap(), 14.0_f32.sqrt());
    assert_close_f32(sasum_strided(3, &px, 2).unwrap(), 6.0);
    assert_eq!(isamax_strided(3, &px, 2).unwrap(), Some(2));
    srot_strided(3, 1.0, 0.0, &mut py, 2, &mut q, 2).unwrap();
}

#[test]
fn strided_and_empty_cases_use_blas_backing_store_conventions() {
    let x = [1.0, 0.0, 2.0, 0.0, 3.0];
    let mut y = [4.0, 0.0, 5.0, 0.0, 6.0];
    daxpy_strided(3, 2.0, &x, 2, &mut y, 2).unwrap();
    assert_eq!(y, [6.0, 0.0, 9.0, 0.0, 12.0]);

    let mut reverse = [0.0; 5];
    dcopy_strided(3, &x, -2, &mut reverse, -2).unwrap();
    assert_eq!(reverse, x);
    assert_close_f64(ddot_strided(3, &x, -2, &x, -2).unwrap(), 14.0);
    assert_close_f64(dnrm2_strided(3, &x, -2).unwrap(), 14.0_f64.sqrt());
    assert_close_f64(
        dasum_strided(3, &[-1.0, 0.0, 2.0, 0.0, -3.0], -2).unwrap(),
        6.0,
    );
    assert_eq!(
        idamax_strided(3, &[1.0, 0.0, -5.0, 0.0, 2.0], -2).unwrap(),
        Some(1)
    );

    let mut sx = [1.0, 0.0, 2.0, 0.0, 3.0];
    let mut sy = [4.0, 0.0, 5.0, 0.0, 6.0];
    dswap_strided(3, &mut sx, 2, &mut sy, 2).unwrap();
    assert_eq!(sx, [4.0, 0.0, 5.0, 0.0, 6.0]);
    assert_eq!(sy, [1.0, 0.0, 2.0, 0.0, 3.0]);
    dscal_strided(3, 2.0, &mut sx, 2).unwrap();
    assert_eq!(sx, [8.0, 0.0, 10.0, 0.0, 12.0]);
    drot_strided(3, 1.0, 0.0, &mut sx, 2, &mut sy, 2).unwrap();

    let mut empty = [];
    daxpy(1.0, &[], &mut empty).unwrap();
    assert_eq!(ddot(&[], &[]).unwrap(), 0.0);
    assert_eq!(idamax(&[]).unwrap(), None);
}
