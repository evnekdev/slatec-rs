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
        Complex32::new(-2.5, 1.5),
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
    }
}

#[test]
fn owned_companion_qr_result_matches_the_reviewed_raw_driver() {
    let coefficients = [1.0_f32, -1.0, 1.0, -1.0];
    let safe = real_polynomial_roots_with_method(&coefficients, PolynomialRootMethod::CompanionQr)
        .unwrap();
    let mut raw_coefficients = coefficients;
    let mut raw_roots = [Complex32::new(0.0, 0.0); 3];
    let mut workspace = [0.0_f32; 15]; // NDEG * (NDEG + 2), with NDEG = 3.
    let mut degree = 3;
    let mut error = 0;
    with_native_lock(|| {
        // SAFETY: this probe holds the production process-wide native lock
        // and its dimensions and private writable buffers meet RPQR79's
        // reviewed raw contract.
        unsafe {
            slatec_sys::roots::complex::rpqr79(
                &mut degree,
                raw_coefficients.as_mut_ptr(),
                raw_roots.as_mut_ptr().cast(),
                &mut error,
                workspace.as_mut_ptr(),
            );
        }
    });
    assert_eq!(error, 0);
    for root in raw_roots {
        assert!(contains_close(safe.roots(), root, 2.0e-5));
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
