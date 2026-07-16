#![cfg(all(
    feature = "blas-level23-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Requires the explicit GNU MinGW native-link environment used by the raw FFI
//! validation. This target is intentionally not part of ordinary source-only
//! CI.

use slatec::blas::level2::*;
use slatec::blas::level3::*;
use slatec::blas::{Diagonal, Side, Transpose, Triangle};

fn close_f64(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "expected {expected}, got {actual}"
    );
}

fn close_f32(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() <= 1.0e-5,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn double_precision_level2_and_level3_match_reference_cases() {
    let a = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]; // 2 by 3, column-major
    let mut y = [0.0; 2];
    dgemv_contiguous(
        Transpose::None,
        2,
        3,
        1.0,
        &a,
        &[1.0, 2.0, 3.0],
        0.0,
        &mut y,
    )
    .unwrap();
    assert_eq!(y, [22.0, 28.0]);
    let mut yt = [0.0; 3];
    dgemv(
        Transpose::Transpose,
        2,
        3,
        1.0,
        &a,
        2,
        &[1.0, 2.0],
        1,
        0.0,
        &mut yt,
        1,
    )
    .unwrap();
    assert_eq!(yt, [5.0, 11.0, 17.0]);

    let mut rank_one = [0.0; 4];
    dger(2, 2, 1.0, &[1.0, 2.0], 1, &[3.0, 4.0], 1, &mut rank_one, 2).unwrap();
    assert_eq!(rank_one, [3.0, 6.0, 4.0, 8.0]);

    let symmetric = [1.0, 0.0, 2.0, 3.0];
    let mut sy = [0.0; 2];
    dsymv(
        Triangle::Upper,
        2,
        1.0,
        &symmetric,
        2,
        &[1.0, 2.0],
        1,
        0.0,
        &mut sy,
        1,
    )
    .unwrap();
    assert_eq!(sy, [5.0, 8.0]);
    let mut tx = [1.0, 2.0];
    dtrmv(
        Triangle::Upper,
        Transpose::None,
        Diagonal::NonUnit,
        2,
        &symmetric,
        2,
        &mut tx,
        1,
    )
    .unwrap();
    assert_eq!(tx, [5.0, 6.0]);
    dtrsv(
        Triangle::Upper,
        Transpose::None,
        Diagonal::NonUnit,
        2,
        &symmetric,
        2,
        &mut tx,
        1,
    )
    .unwrap();
    assert_eq!(tx, [1.0, 2.0]);

    let square = [1.0, 2.0, 3.0, 4.0];
    let mut c = [0.0; 4];
    dgemm_contiguous(
        Transpose::None,
        Transpose::None,
        2,
        2,
        2,
        1.0,
        &square,
        &[1.0, 0.0, 0.0, 1.0],
        0.0,
        &mut c,
    )
    .unwrap();
    assert_eq!(c, square);
    dgemm(
        Transpose::None,
        Transpose::None,
        2,
        2,
        2,
        1.0,
        &square,
        2,
        &[1.0, 0.0, 0.0, 1.0],
        2,
        0.0,
        &mut c,
        2,
    )
    .unwrap();
    dtrmm(
        Side::Left,
        Triangle::Upper,
        Transpose::None,
        Diagonal::NonUnit,
        2,
        2,
        1.0,
        &symmetric,
        2,
        &mut c,
        2,
    )
    .unwrap();
    dtrsm(
        Side::Left,
        Triangle::Upper,
        Transpose::None,
        Diagonal::NonUnit,
        2,
        2,
        1.0,
        &symmetric,
        2,
        &mut c,
        2,
    )
    .unwrap();
    assert_eq!(c, square);

    let mut syrk = [0.0; 4];
    dsyrk(
        Triangle::Upper,
        Transpose::None,
        2,
        2,
        1.0,
        &square,
        2,
        0.0,
        &mut syrk,
        2,
    )
    .unwrap();
    close_f64(syrk[0], 10.0);
    close_f64(syrk[2], 14.0);
    close_f64(syrk[3], 20.0);
}

#[test]
fn single_precision_wrappers_and_zero_size_paths_are_callable() {
    let a = [1.0_f32, 2.0, 3.0, 4.0];
    let mut y = [0.0_f32; 2];
    sgemv_contiguous(Transpose::None, 2, 2, 1.0, &a, &[1.0, 1.0], 0.0, &mut y).unwrap();
    assert_eq!(y, [4.0, 6.0]);
    sgemv(
        Transpose::None,
        2,
        2,
        1.0,
        &a,
        2,
        &[1.0, 1.0],
        1,
        0.0,
        &mut y,
        1,
    )
    .unwrap();

    let mut update = [0.0_f32; 4];
    sger(2, 2, 1.0, &[1.0, 2.0], 1, &[3.0, 4.0], 1, &mut update, 2).unwrap();
    let mut sy = [0.0_f32; 2];
    ssymv(
        Triangle::Lower,
        2,
        1.0,
        &a,
        2,
        &[1.0, 1.0],
        1,
        0.0,
        &mut sy,
        1,
    )
    .unwrap();
    let mut tx = [1.0_f32, 1.0];
    strmv(
        Triangle::Lower,
        Transpose::None,
        Diagonal::Unit,
        2,
        &a,
        2,
        &mut tx,
        1,
    )
    .unwrap();
    strsv(
        Triangle::Lower,
        Transpose::None,
        Diagonal::Unit,
        2,
        &a,
        2,
        &mut tx,
        1,
    )
    .unwrap();

    let mut c = [0.0_f32; 4];
    sgemm_contiguous(
        Transpose::None,
        Transpose::None,
        2,
        2,
        2,
        1.0,
        &a,
        &[1.0, 0.0, 0.0, 1.0],
        0.0,
        &mut c,
    )
    .unwrap();
    sgemm(
        Transpose::None,
        Transpose::None,
        2,
        2,
        2,
        1.0,
        &a,
        2,
        &[1.0, 0.0, 0.0, 1.0],
        2,
        0.0,
        &mut c,
        2,
    )
    .unwrap();
    strmm(
        Side::Right,
        Triangle::Lower,
        Transpose::None,
        Diagonal::Unit,
        2,
        2,
        1.0,
        &a,
        2,
        &mut c,
        2,
    )
    .unwrap();
    strsm(
        Side::Right,
        Triangle::Lower,
        Transpose::None,
        Diagonal::Unit,
        2,
        2,
        1.0,
        &a,
        2,
        &mut c,
        2,
    )
    .unwrap();
    ssyrk(
        Triangle::Lower,
        Transpose::None,
        2,
        2,
        1.0,
        &a,
        2,
        0.0,
        &mut c,
        2,
    )
    .unwrap();
    close_f32(c[0], 10.0);

    sgemv_contiguous(Transpose::None, 0, 0, 1.0, &[], &[], 0.0, &mut []).unwrap();
    sgemm_contiguous(
        Transpose::None,
        Transpose::None,
        0,
        0,
        0,
        1.0,
        &[],
        &[],
        0.0,
        &mut [],
    )
    .unwrap();
}
