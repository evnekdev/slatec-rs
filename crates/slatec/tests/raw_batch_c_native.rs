//! Representative native smoke coverage for Batch C raw ABI classes.
//!
//! These tests exercise compiler-probed complex returns, complex arrays,
//! mutation through complex outputs, and status-bearing complex drivers. The
//! controlled raw-FFI validation suite separately covers CHARACTER hidden
//! lengths and LOGICAL scalar, array, and return conventions.

#![cfg(all(
    feature = "raw-batch-c-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

use slatec_sys::{Complex32, FortranInteger};

fn assert_complex_close(actual: Complex32, expected: Complex32, tolerance: f32) {
    assert!((actual.re - expected.re).abs() <= tolerance);
    assert!((actual.im - expected.im).abs() <= tolerance);
}

#[test]
fn cdotu_returns_a_complex_dot_product() {
    slatec_src::ensure_linked();
    let mut n: FortranInteger = 2;
    let mut increment: FortranInteger = 1;
    let mut x = [
        Complex32 { re: 1.0, im: 2.0 },
        Complex32 { re: 3.0, im: -1.0 },
    ];
    let mut y = [
        Complex32 { re: 2.0, im: -1.0 },
        Complex32 { re: -1.0, im: 4.0 },
    ];
    let result = unsafe {
        slatec_sys::blas::level1::cdotu(
            &mut n,
            x.as_mut_ptr(),
            &mut increment,
            y.as_mut_ptr(),
            &mut increment,
        )
    };
    assert_complex_close(result, Complex32 { re: 5.0, im: 16.0 }, 1.0e-5);
}

#[test]
fn cdcdot_adds_the_complex_bias() {
    slatec_src::ensure_linked();
    let mut n: FortranInteger = 1;
    let mut increment: FortranInteger = 1;
    let mut bias = Complex32 { re: 0.5, im: -0.5 };
    let mut x = [Complex32 { re: 1.0, im: 2.0 }];
    let mut y = [Complex32 { re: 2.0, im: -1.0 }];
    let result = unsafe {
        slatec_sys::blas::level1::cdcdot(
            &mut n,
            &mut bias,
            x.as_mut_ptr(),
            &mut increment,
            y.as_mut_ptr(),
            &mut increment,
        )
    };
    assert_complex_close(result, Complex32 { re: 4.5, im: 2.5 }, 1.0e-5);
}

#[test]
fn cgefa_mutates_a_complex_matrix_and_writes_pivots() {
    slatec_src::ensure_linked();
    let mut matrix = [
        Complex32 { re: 2.0, im: 0.0 },
        Complex32 { re: 1.0, im: 1.0 },
        Complex32 { re: 1.0, im: -1.0 },
        Complex32 { re: 3.0, im: 0.0 },
    ];
    let original = matrix;
    let mut leading_dimension: FortranInteger = 2;
    let mut order: FortranInteger = 2;
    let mut pivots = [0_i32; 2];
    let mut status: FortranInteger = -1;
    unsafe {
        slatec_sys::linear_algebra::dense::complex::cgefa(
            matrix.as_mut_ptr(),
            &mut leading_dimension,
            &mut order,
            pivots.as_mut_ptr(),
            &mut status,
        )
    };
    assert_eq!(status, 0);
    assert!(pivots.iter().all(|pivot| *pivot >= 1));
    assert_ne!(matrix, original);
}

#[test]
fn cairy_writes_a_complex_output_and_status() {
    slatec_src::ensure_linked();
    let mut argument = Complex32 { re: 0.0, im: 0.0 };
    let mut derivative: FortranInteger = 0;
    let mut scaling: FortranInteger = 1;
    let mut answer = Complex32::default();
    let mut underflows: FortranInteger = -1;
    let mut status: FortranInteger = -1;
    unsafe {
        slatec_sys::special::complex::cairy(
            &mut argument,
            &mut derivative,
            &mut scaling,
            &mut answer,
            &mut underflows,
            &mut status,
        )
    };
    assert_eq!(status, 0);
    assert_eq!(underflows, 0);
    assert_complex_close(
        answer,
        Complex32 {
            re: 0.355_028_06,
            im: 0.0,
        },
        1.0e-5,
    );
}
