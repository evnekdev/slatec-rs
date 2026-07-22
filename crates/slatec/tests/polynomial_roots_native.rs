#![cfg(all(
    feature = "roots-polynomial",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native regressions for reviewed `CPZERO`, `RPZERO`, `CPQR79`, and `RPQR79`.

use num_complex::Complex32;
use slatec::native_lock_test_support::with_native_lock;
use slatec::roots::{
    PolynomialRootError, PolynomialRootMethod, PolynomialRootStatus, complex_polynomial_roots,
    complex_polynomial_roots_with_method, real_polynomial_roots, real_polynomial_roots_with_method,
};

fn contains_close(roots: &[Complex32], expected: Complex32, tolerance: f32) -> bool {
    roots.iter().any(|root| {
        let difference = *root - expected;
        difference.re * difference.re + difference.im * difference.im <= tolerance * tolerance
    })
}

fn assert_same_roots(left: &[Complex32], right: &[Complex32], tolerance: f32) {
    assert_eq!(left.len(), right.len());
    for root in left {
        assert!(
            contains_close(right, *root, tolerance),
            "missing matching root {root:?} in {right:?}"
        );
    }
}

fn real_residual(coefficients: &[f32], point: Complex32) -> Complex32 {
    coefficients
        .iter()
        .fold(Complex32::new(0.0, 0.0), |value, coefficient| {
            value * point + Complex32::new(*coefficient, 0.0)
        })
}

fn complex_residual(coefficients: &[Complex32], point: Complex32) -> Complex32 {
    coefficients
        .iter()
        .fold(Complex32::new(0.0, 0.0), |value, coefficient| {
            value * point + *coefficient
        })
}

fn assert_small_residuals(
    roots: &[Complex32],
    residual: impl Fn(Complex32) -> Complex32,
    tolerance: f32,
) {
    for root in roots {
        let value = residual(*root);
        assert!(
            value.re * value.re + value.im * value.im <= tolerance * tolerance,
            "residual {value:?} at root {root:?} exceeds {tolerance}"
        );
    }
}

#[test]
fn real_coefficients_return_one_real_and_two_imaginary_roots_with_both_methods() {
    // (x - 1) * (x^2 + 1), in the descending-power layout required by SLATEC.
    let coefficients = [1.0_f32, -1.0, 1.0, -1.0];
    let automatic = real_polynomial_roots(&coefficients).unwrap();
    assert!(contains_close(
        automatic.roots(),
        Complex32::new(1.0, 0.0),
        2.0e-3
    ));
    for method in [
        PolynomialRootMethod::Iterative,
        PolynomialRootMethod::CompanionQr,
    ] {
        let result = real_polynomial_roots_with_method(&coefficients, method).unwrap();
        assert_eq!(result.degree(), 3);
        assert!(matches!(
            result.status(),
            PolynomialRootStatus::Converged | PolynomialRootStatus::IterationLimitReached
        ));
        assert!(contains_close(
            result.roots(),
            Complex32::new(1.0, 0.0),
            2.0e-3
        ));
        assert!(contains_close(
            result.roots(),
            Complex32::new(0.0, 1.0),
            2.0e-3
        ));
        assert!(contains_close(
            result.roots(),
            Complex32::new(0.0, -1.0),
            2.0e-3
        ));
        assert_small_residuals(
            result.roots(),
            |root| real_residual(&coefficients, root),
            2.0e-3,
        );
        if result.status() == PolynomialRootStatus::Converged {
            assert_eq!(
                result.error_bounds().is_some(),
                method == PolynomialRootMethod::Iterative
            );
        }
    }
}

#[test]
fn complex_coefficients_return_the_manufactured_roots() {
    // (z - (1 + i)) * (z - (-2 + 0.5i)).
    let coefficients = [
        Complex32::new(1.0, 0.0),
        Complex32::new(1.0, -1.5),
        Complex32::new(-2.5, -1.5),
    ];
    let automatic = complex_polynomial_roots(&coefficients).unwrap();
    assert!(contains_close(
        automatic.roots(),
        Complex32::new(1.0, 1.0),
        3.0e-3
    ));
    for method in [
        PolynomialRootMethod::Iterative,
        PolynomialRootMethod::CompanionQr,
    ] {
        let result = complex_polynomial_roots_with_method(&coefficients, method).unwrap();
        assert!(contains_close(
            result.roots(),
            Complex32::new(1.0, 1.0),
            3.0e-3
        ));
        assert!(contains_close(
            result.roots(),
            Complex32::new(-2.0, 0.5),
            3.0e-3
        ));
        assert_small_residuals(
            result.roots(),
            |root| complex_residual(&coefficients, root),
            3.0e-3,
        );
    }
}

#[test]
fn all_safe_methods_match_their_reviewed_raw_drivers() {
    let coefficients = [1.0_f32, -1.0, 1.0, -1.0];
    let complex_coefficients = [
        Complex32::new(1.0, 0.0),
        Complex32::new(1.0, -1.5),
        Complex32::new(-2.5, -1.5),
    ];

    let safe_real_iterative = real_polynomial_roots(&coefficients).unwrap();
    let mut raw_real_coefficients = coefficients;
    let mut raw_real_iterative = [Complex32::new(0.0, 0.0); 3];
    let mut raw_real_workspace = [Complex32::new(0.0, 0.0); 24];
    let mut raw_real_bounds = [0.0_f32; 3];
    let mut real_degree = 3;
    let mut real_flag = 0;
    with_native_lock(|| {
        // SAFETY: the probe supplies RPZERO's reviewed degree, copied
        // descending coefficients, roots, workspace, and bounds extents.
        unsafe {
            slatec_sys::roots::complex::rpzero(
                &mut real_degree,
                raw_real_coefficients.as_mut_ptr(),
                raw_real_iterative.as_mut_ptr().cast(),
                raw_real_workspace.as_mut_ptr().cast(),
                &mut real_flag,
                raw_real_bounds.as_mut_ptr(),
            );
        }
    });
    assert_eq!(real_flag, 0);
    assert_same_roots(safe_real_iterative.roots(), &raw_real_iterative, 2.0e-5);

    let safe_complex_iterative = complex_polynomial_roots(&complex_coefficients).unwrap();
    let mut raw_complex_coefficients = complex_coefficients;
    let mut raw_complex_iterative = [Complex32::new(0.0, 0.0); 2];
    let mut raw_complex_workspace = [Complex32::new(0.0, 0.0); 12];
    let mut raw_complex_bounds = [0.0_f32; 2];
    let mut complex_degree = 2;
    let mut complex_flag = 0;
    with_native_lock(|| {
        // SAFETY: the probe supplies CPZERO's reviewed degree, copied
        // descending coefficients, roots, workspace, and bounds extents.
        unsafe {
            slatec_sys::roots::complex::cpzero(
                &mut complex_degree,
                raw_complex_coefficients.as_mut_ptr().cast(),
                raw_complex_iterative.as_mut_ptr().cast(),
                raw_complex_workspace.as_mut_ptr().cast(),
                &mut complex_flag,
                raw_complex_bounds.as_mut_ptr(),
            );
        }
    });
    assert_eq!(complex_flag, 0);
    assert_same_roots(
        safe_complex_iterative.roots(),
        &raw_complex_iterative,
        2.0e-5,
    );

    let safe_real_qr =
        real_polynomial_roots_with_method(&coefficients, PolynomialRootMethod::CompanionQr)
            .unwrap();
    let mut raw_real_qr_coefficients = coefficients;
    let mut raw_real_qr = [Complex32::new(0.0, 0.0); 3];
    let mut raw_real_qr_workspace = [0.0_f32; 15];
    let mut real_qr_degree = 3;
    let mut real_qr_error = 0;
    with_native_lock(|| {
        // SAFETY: this probe holds the production process-wide native lock
        // and its dimensions and private writable buffers meet RPQR79's
        // reviewed raw contract.
        unsafe {
            slatec_sys::roots::complex::rpqr79(
                &mut real_qr_degree,
                raw_real_qr_coefficients.as_mut_ptr(),
                raw_real_qr.as_mut_ptr().cast(),
                &mut real_qr_error,
                raw_real_qr_workspace.as_mut_ptr(),
            );
        }
    });
    assert_eq!(real_qr_error, 0);
    assert_same_roots(safe_real_qr.roots(), &raw_real_qr, 2.0e-5);

    let safe_complex_qr = complex_polynomial_roots_with_method(
        &complex_coefficients,
        PolynomialRootMethod::CompanionQr,
    )
    .unwrap();
    let mut raw_complex_qr_coefficients = complex_coefficients;
    let mut raw_complex_qr = [Complex32::new(0.0, 0.0); 2];
    let mut raw_complex_qr_workspace = [0.0_f32; 12];
    let mut complex_qr_degree = 2;
    let mut complex_qr_error = 0;
    with_native_lock(|| {
        // SAFETY: this probe supplies CPQR79's reviewed complex coefficient,
        // root, and real workspace extents under the production native lock.
        unsafe {
            slatec_sys::roots::complex::cpqr79(
                &mut complex_qr_degree,
                raw_complex_qr_coefficients.as_mut_ptr().cast(),
                raw_complex_qr.as_mut_ptr().cast(),
                &mut complex_qr_error,
                raw_complex_qr_workspace.as_mut_ptr(),
            );
        }
    });
    assert_eq!(complex_qr_error, 0);
    assert_same_roots(safe_complex_qr.roots(), &raw_complex_qr, 2.0e-5);
}

#[test]
fn clustered_real_roots_remain_within_the_source_supported_tolerance() {
    // (x - 1)^2 * (x + 2): a repeated root plus a separated root. The
    // comparison allows the documented iterative-limit partial-result status.
    let coefficients = [1.0_f32, 0.0, -3.0, 2.0];
    for method in [
        PolynomialRootMethod::Iterative,
        PolynomialRootMethod::CompanionQr,
    ] {
        let result = real_polynomial_roots_with_method(&coefficients, method).unwrap();
        assert!(contains_close(
            result.roots(),
            Complex32::new(-2.0, 0.0),
            2.0e-3
        ));
        assert!(
            result
                .roots()
                .iter()
                .filter(|root| {
                    let difference = **root - Complex32::new(1.0, 0.0);
                    difference.re * difference.re + difference.im * difference.im
                        <= 1.5e-2_f32.powi(2)
                })
                .count()
                >= 2
        );
        assert_small_residuals(
            result.roots(),
            |root| real_residual(&coefficients, root),
            2.0e-2,
        );
    }
}

#[test]
fn polynomial_preflight_rejects_invalid_leading_and_nonfinite_coefficients() {
    assert_eq!(
        real_polynomial_roots_with_method(&[0.0, 1.0], PolynomialRootMethod::Iterative),
        Err(PolynomialRootError::ZeroLeadingCoefficient)
    );
    assert_eq!(
        complex_polynomial_roots_with_method(
            &[Complex32::new(1.0, f32::NAN), Complex32::new(0.0, 0.0)],
            PolynomialRootMethod::CompanionQr,
        ),
        Err(PolynomialRootError::NonFiniteCoefficient { index: 0 })
    );
}
