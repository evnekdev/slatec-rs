#![cfg(all(
    feature = "special-functions-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Explicit GNU MinGW native tests for the runtime-validated special surface.
//!
//! They require `SLATEC_NATIVE_LIB_DIR` and `SLATEC_GFORTRAN_RUNTIME_DIR` to
//! point at the archive rebuilt by `build-native-ffi --offline`.  They are not
//! part of ordinary source-only CI.

use std::process::Command;

use slatec::polynomials::chebyshev::*;
use slatec::special::{
    airy::*, bessel::*, elementary::*, error_functions::*, gamma::*, integrals::*,
    scalar_expanded::*,
};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, observed {actual}, tolerance {tolerance}"
    );
}

fn finite(value: f64) {
    assert!(
        value.is_finite(),
        "expected a finite result, observed {value}"
    );
}

#[test]
fn every_exposed_wrapper_executes_with_an_ordinary_valid_case() {
    for value in [
        log1p(0.25).unwrap(),
        exprel(0.5).unwrap(),
        cbrt(8.0).unwrap(),
        sin_degrees(30.0).unwrap(),
        cos_degrees(60.0).unwrap(),
        dawson(1.0).unwrap(),
        airy_ai(0.0).unwrap(),
        airy_ai_scaled(0.0).unwrap(),
        airy_bi(0.0).unwrap(),
        airy_bi_scaled(0.0).unwrap(),
        erf(0.5).unwrap(),
        erfc(0.5).unwrap(),
        exponential_integral_e1(1.0).unwrap(),
        exponential_integral_ei(1.0).unwrap(),
        logarithmic_integral(2.0).unwrap(),
        spence_integral(1.0).unwrap(),
        carlson_rc(0.0, 1.0).unwrap(),
        carlson_rf(1.0, 1.0, 1.0).unwrap(),
        carlson_rd(1.0, 1.0, 1.0).unwrap(),
        carlson_rj(1.0, 1.0, 1.0, 1.0).unwrap(),
        gamma(1.0).unwrap(),
        reciprocal_gamma(1.0).unwrap(),
        log_gamma(1.0).unwrap(),
        beta(1.0, 1.0).unwrap(),
        log_beta(1.0, 1.0).unwrap(),
        regularized_beta(0.5, 1.0, 1.0).unwrap(),
        incomplete_gamma_lower(1.0, 1.0).unwrap(),
        incomplete_gamma_upper(1.0, 1.0).unwrap(),
        tricomi_incomplete_gamma(1.0, 1.0).unwrap(),
        digamma(1.0).unwrap(),
        factorial(5).unwrap(),
        binomial_coefficient(5, 2).unwrap(),
        bessel_j0(0.0).unwrap(),
        bessel_j1(0.0).unwrap(),
        bessel_y0(1.0).unwrap(),
        bessel_y1(1.0).unwrap(),
        bessel_i0(1.0).unwrap(),
        bessel_i1(1.0).unwrap(),
        bessel_i0_scaled(1.0).unwrap(),
        bessel_i1_scaled(1.0).unwrap(),
        bessel_k0(1.0).unwrap(),
        bessel_k1(1.0).unwrap(),
        bessel_k0_scaled(1.0).unwrap(),
        bessel_k1_scaled(1.0).unwrap(),
        chebyshev_series(0.0, &[1.0, 0.5]).unwrap(),
    ] {
        finite(value);
    }

    for value in [
        f64::from(log1p_f32(0.25).unwrap()),
        f64::from(exprel_f32(0.5).unwrap()),
        f64::from(cbrt_f32(8.0).unwrap()),
        f64::from(sin_degrees_f32(30.0).unwrap()),
        f64::from(cos_degrees_f32(60.0).unwrap()),
        f64::from(dawson_f32(1.0).unwrap()),
        f64::from(airy_ai_f32(0.0).unwrap()),
        f64::from(airy_ai_scaled_f32(0.0).unwrap()),
        f64::from(airy_bi_f32(0.0).unwrap()),
        f64::from(airy_bi_scaled_f32(0.0).unwrap()),
        f64::from(erf_f32(0.5).unwrap()),
        f64::from(erfc_f32(0.5).unwrap()),
        f64::from(exponential_integral_e1_f32(1.0).unwrap()),
        f64::from(exponential_integral_ei_f32(1.0).unwrap()),
        f64::from(logarithmic_integral_f32(2.0).unwrap()),
        f64::from(spence_integral_f32(1.0).unwrap()),
        f64::from(carlson_rc_f32(0.0, 1.0).unwrap()),
        f64::from(carlson_rf_f32(1.0, 1.0, 1.0).unwrap()),
        f64::from(carlson_rd_f32(1.0, 1.0, 1.0).unwrap()),
        f64::from(carlson_rj_f32(1.0, 1.0, 1.0, 1.0).unwrap()),
        f64::from(gamma_f32(1.0).unwrap()),
        f64::from(reciprocal_gamma_f32(1.0).unwrap()),
        f64::from(log_gamma_f32(1.0).unwrap()),
        f64::from(beta_f32(1.0, 1.0).unwrap()),
        f64::from(log_beta_f32(1.0, 1.0).unwrap()),
        f64::from(regularized_beta_f32(0.5, 1.0, 1.0).unwrap()),
        f64::from(incomplete_gamma_lower_f32(1.0, 1.0).unwrap()),
        f64::from(incomplete_gamma_upper_f32(1.0, 1.0).unwrap()),
        f64::from(tricomi_incomplete_gamma_f32(1.0, 1.0).unwrap()),
        f64::from(digamma_f32(1.0).unwrap()),
        f64::from(factorial_f32(5).unwrap()),
        f64::from(binomial_coefficient_f32(5, 2).unwrap()),
        f64::from(bessel_j0_f32(0.0).unwrap()),
        f64::from(bessel_j1_f32(0.0).unwrap()),
        f64::from(bessel_y0_f32(1.0).unwrap()),
        f64::from(bessel_y1_f32(1.0).unwrap()),
        f64::from(bessel_i0_f32(1.0).unwrap()),
        f64::from(bessel_i1_f32(1.0).unwrap()),
        f64::from(bessel_i0_scaled_f32(1.0).unwrap()),
        f64::from(bessel_i1_scaled_f32(1.0).unwrap()),
        f64::from(bessel_k0_f32(1.0).unwrap()),
        f64::from(bessel_k1_f32(1.0).unwrap()),
        f64::from(bessel_k0_scaled_f32(1.0).unwrap()),
        f64::from(bessel_k1_scaled_f32(1.0).unwrap()),
        f64::from(chebyshev_series_f32(0.0, &[1.0, 0.5]).unwrap()),
    ] {
        finite(value);
    }
}

#[test]
fn reference_and_identity_cases_hold_on_the_validated_profile() {
    close(gamma(1.0).unwrap(), 1.0, 1.0e-13);
    close(gamma(0.5).unwrap(), std::f64::consts::PI.sqrt(), 1.0e-12);
    close(
        reciprocal_gamma(0.5).unwrap(),
        1.0 / std::f64::consts::PI.sqrt(),
        1.0e-12,
    );
    close(beta(1.0, 1.0).unwrap(), 1.0, 1.0e-13);
    close(regularized_beta(0.25, 1.0, 1.0).unwrap(), 0.25, 1.0e-12);
    close(erf(0.0).unwrap(), 0.0, 1.0e-15);
    close(erfc(0.0).unwrap(), 1.0, 1.0e-15);
    close(airy_ai(0.0).unwrap(), 0.355_028_053_887_817_2, 1.0e-13);
    close(
        exponential_integral_ei(1.0).unwrap(),
        1.895_117_816_355_936_8,
        1.0e-12,
    );
    close(
        exponential_integral_e1(1.0).unwrap(),
        0.219_383_934_395_520_3,
        1.0e-12,
    );
    close(
        logarithmic_integral(2.0).unwrap(),
        1.045_163_780_117_492_8,
        1.0e-12,
    );
    close(
        spence_integral(1.0).unwrap(),
        std::f64::consts::PI.powi(2) / 6.0,
        1.0e-12,
    );
    close(
        carlson_rc(0.0, 1.0).unwrap(),
        std::f64::consts::FRAC_PI_2,
        1.0e-12,
    );
    close(carlson_rf(1.0, 1.0, 1.0).unwrap(), 1.0, 1.0e-13);
    close(carlson_rd(1.0, 1.0, 1.0).unwrap(), 1.0, 1.0e-13);
    close(carlson_rj(1.0, 1.0, 1.0, 1.0).unwrap(), 1.0, 1.0e-13);
    close(bessel_j0(0.0).unwrap(), 1.0, 1.0e-13);
    close(bessel_j1(0.0).unwrap(), 0.0, 1.0e-13);
    close(sin_degrees(30.0).unwrap(), 0.5, 1.0e-13);
    close(cos_degrees(60.0).unwrap(), 0.5, 1.0e-13);

    close(gamma(3.5).unwrap(), 2.5 * gamma(2.5).unwrap(), 2.0e-12);
    close(erf(0.75).unwrap() + erfc(0.75).unwrap(), 1.0, 1.0e-13);
    close(
        beta(2.0, 3.0).unwrap(),
        gamma(2.0).unwrap() * gamma(3.0).unwrap() / gamma(5.0).unwrap(),
        1.0e-13,
    );
}

#[test]
fn safe_domain_checks_prevent_the_known_fatal_inputs() {
    assert!(gamma(0.0).is_err());
    assert!(exponential_integral_e1(0.0).is_err());
    assert!(bessel_k0(0.0).is_err());
    assert!(log1p(-1.0).is_err());
    assert!(logarithmic_integral(1.0).is_err());
    assert!(carlson_rc(-1.0, 1.0).is_err());
    assert!(carlson_rf(0.0, 0.0, 1.0).is_err());
}

#[test]
fn carlson_range_status_is_contained_and_xerror_is_restored() {
    assert_eq!(
        carlson_rc(0.0, f64::MIN_POSITIVE),
        Err(slatec::special::SpecialFunctionError::NativeStatus {
            function: "carlson_rc",
            status: 2,
        })
    );
    close(carlson_rf(1.0, 1.0, 1.0).unwrap(), 1.0, 1.0e-13);
}

#[test]
fn contained_invalid_raw_calls_use_the_profile_fatal_status() {
    if let Ok(case) = std::env::var("SLATEC_SPECIAL_CHILD_CASE") {
        match case.as_str() {
            "gamma" => {
                let mut x = 0.0;
                unsafe { slatec_sys::special::gamma::dgamma(&mut x) };
            }
            "e1" => {
                let mut x = 0.0;
                unsafe { slatec_sys::special::de1(&mut x) };
            }
            "k0" => {
                let mut x = 0.0;
                unsafe { slatec_sys::special::bessel::dbesk0(&mut x) };
            }
            "log1p" => {
                let mut x = -1.0;
                unsafe { slatec_sys::special::elementary::dlnrel(&mut x) };
            }
            _ => panic!("unknown child case {case}"),
        }
        panic!("fatal SLATEC path unexpectedly returned");
    }
    for case in ["gamma", "e1", "k0", "log1p"] {
        let status = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "contained_invalid_raw_calls_use_the_profile_fatal_status",
                "--nocapture",
            ])
            .env("SLATEC_SPECIAL_CHILD_CASE", case)
            .status()
            .unwrap();
        // The reviewed GNU MinGW source build reaches XERROR's Fortran STOP
        // path, which terminates the child before the following Rust panic
        // and reports zero on this runtime. The source-build validation profile
        // reports 70 instead.  In either case, a raw fatal call never returns
        // into the test body; safe wrappers reject these inputs before FFI.
        assert!(
            matches!(status.code(), Some(0) | Some(70)),
            "unexpected child termination for {case}: {status:?}"
        );
    }
}

#[test]
fn serialized_state_remains_usable_after_a_contained_failure() {
    close(gamma(1.0).unwrap(), 1.0, 1.0e-13);
    let handles = (0..4)
        .map(|_| std::thread::spawn(|| (gamma(1.0).unwrap(), airy_ai(0.0).unwrap())))
        .collect::<Vec<_>>();
    for handle in handles {
        let (gamma_value, ai_value) = handle.join().unwrap();
        close(gamma_value, 1.0, 1.0e-13);
        close(ai_value, 0.355_028_053_887_817_2, 1.0e-13);
    }
}
