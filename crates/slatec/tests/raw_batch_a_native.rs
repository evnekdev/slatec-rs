//! Representative native smoke coverage for the Batch A raw ABI classes.
//!
//! This is intentionally a small risk-based sample. It proves calls through
//! scalar returns, scalar outputs, status-bearing arrays, leading dimensions,
//! and caller-owned workspace without claiming numerical validation for the
//! full Batch A corpus.

#![cfg(all(
    feature = "raw-canonical-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

use slatec_sys::FortranInteger;

#[test]
fn scalar_real_and_double_functions_return_plausible_values() {
    slatec_src::ensure_linked();
    let mut single = 1.5_f32;
    let mut double = 1.5_f64;
    let single_result = unsafe { slatec_sys::special::acosh(&mut single) };
    let double_result = unsafe { slatec_sys::special::dacosh(&mut double) };
    assert!((single_result - 1.5_f32.acosh()).abs() < 1.0e-5);
    assert!((double_result - 1.5_f64.acosh()).abs() < 1.0e-12);
}

#[test]
fn scalar_subroutine_writes_outputs() {
    slatec_src::ensure_linked();
    let mut x = 2.0_f32;
    let mut logarithm = f32::NAN;
    let mut sign = f32::NAN;
    unsafe { slatec_sys::special::algams(&mut x, &mut logarithm, &mut sign) };
    assert!(logarithm.abs() < 1.0e-6);
    assert_eq!(sign, 1.0);
}

#[test]
fn status_bearing_array_subroutine_integrates_a_parabola() {
    slatec_src::ensure_linked();
    let mut x = [0.0_f64, 1.0, 2.0];
    let mut y = [0.0_f64, 1.0, 4.0];
    let mut n: FortranInteger = 3;
    let mut lower = 0.0_f64;
    let mut upper = 2.0_f64;
    let mut answer = f64::NAN;
    let mut status: FortranInteger = -1;
    unsafe {
        slatec_sys::quadrature::davint(
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            &mut n,
            &mut lower,
            &mut upper,
            &mut answer,
            &mut status,
        )
    };
    assert_eq!(status, 1);
    assert!((answer - 8.0 / 3.0).abs() < 1.0e-12);
}

#[test]
fn leading_dimension_matrix_factorization_writes_pivots() {
    slatec_src::ensure_linked();
    // Fortran column-major matrix [[2, 1], [1, 3]].
    let mut matrix = [2.0_f64, 1.0, 1.0, 3.0];
    let mut leading_dimension: FortranInteger = 2;
    let mut order: FortranInteger = 2;
    let mut pivots = [0_i32; 2];
    let mut status: FortranInteger = -1;
    unsafe {
        slatec_sys::linear_algebra::dense::dgefa(
            matrix.as_mut_ptr(),
            &mut leading_dimension,
            &mut order,
            pivots.as_mut_ptr(),
            &mut status,
        )
    };
    assert_eq!(status, 0);
    assert!(pivots.iter().all(|pivot| *pivot >= 1));
}

#[test]
fn workspace_sequence_output_subroutine_writes_derivatives() {
    slatec_src::ensure_linked();
    let mut boundary = [2_i32, 2_i32];
    let values = [0.0_f32, 0.0_f32];
    let mut count: FortranInteger = 3;
    let abscissas = [0.0_f32, 1.0, 2.0];
    let ordinates = [0.0_f32, 1.0, 4.0];
    let mut derivatives = [f32::NAN; 3];
    let mut increment: FortranInteger = 1;
    let mut workspace = [0.0_f32; 6];
    let mut workspace_length: FortranInteger = 6;
    let mut status: FortranInteger = -1;
    unsafe {
        slatec_sys::interpolation::pchsp(
            boundary.as_mut_ptr(),
            values.as_ptr(),
            &mut count,
            abscissas.as_ptr(),
            ordinates.as_ptr(),
            derivatives.as_mut_ptr(),
            &mut increment,
            workspace.as_mut_ptr(),
            &mut workspace_length,
            &mut status,
        )
    };
    assert_eq!(status, 0);
    assert!(derivatives.iter().all(|value| value.is_finite()));
}
