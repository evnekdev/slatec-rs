#![cfg(all(
    feature = "special-airy",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

//! Native numerical regression coverage for the reviewed real FNLIB Airy API.
//!
//! The constants are independent high-accuracy reference values (DLMF 9.2 and
//! 9.7 values, rounded only for the stated tolerances).  Run with the narrow
//! `source-build,special-airy` feature combination on GNU MinGW.

use slatec::special::airy::{
    airy_ai, airy_ai_f32, airy_ai_scaled, airy_ai_scaled_f32, airy_bi, airy_bi_f32, airy_bi_scaled,
    airy_bi_scaled_f32,
};

fn close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected:.17e}, observed {actual:.17e}, tolerance {tolerance:.1e}"
    );
}

fn derivative(function: impl Fn(f64) -> f64, x: f64) -> f64 {
    let h = 1.0e-4;
    (function(x + h) - function(x - h)) / (2.0 * h)
}

#[test]
fn known_values_cover_negative_zero_positive_and_large_arguments() {
    // Values at -1, 0, 1, and 5 exercise both oscillatory and exponential
    // regions without relying on another wrapper around SLATEC as the oracle.
    for (x, ai, bi) in [
        (-1.0, 0.535_560_883_292_352_1, 0.103_997_389_496_944_6),
        (0.0, 0.355_028_053_887_817_2, 0.614_926_627_446_000_7),
        (1.0, 0.135_292_416_312_881_4, 1.207_423_594_952_871_3),
        (5.0, 1.083_444_281_360_744_2e-4, 657.792_044_171_171_2),
    ] {
        close(airy_ai(x).unwrap(), ai, 2.0e-12 * ai.abs().max(1.0));
        close(airy_bi(x).unwrap(), bi, 2.0e-12 * bi.abs().max(1.0));
    }

    // The scaled forms are specifically the usable representation at the
    // positive end of the documented safe envelope.
    assert!(airy_ai_scaled(20.0).unwrap().is_finite());
    assert!(airy_bi_scaled(20.0).unwrap().is_finite());
    assert!(airy_ai(-20.0).unwrap().is_finite());
    assert!(airy_bi(-20.0).unwrap().is_finite());
}

#[test]
fn scaled_relations_and_single_precision_calls_hold() {
    for x in [-5.0_f64, 0.0, 1.0, 5.0] {
        let scale = if x <= 0.0 {
            1.0
        } else {
            (2.0 * x * x.sqrt() / 3.0).exp()
        };
        close(
            airy_ai_scaled(x).unwrap(),
            scale * airy_ai(x).unwrap(),
            3.0e-12 * airy_ai_scaled(x).unwrap().abs().max(1.0),
        );
        close(
            airy_bi_scaled(x).unwrap(),
            airy_bi(x).unwrap() / scale,
            3.0e-12 * airy_bi_scaled(x).unwrap().abs().max(1.0),
        );
    }

    // The f32 calls exercise AI/AIE/BI/BIE at representative signed inputs.
    for x in [-1.0_f32, 0.0, 1.0, 5.0] {
        assert!(airy_ai_f32(x).unwrap().is_finite());
        assert!(airy_ai_scaled_f32(x).unwrap().is_finite());
        assert!(airy_bi_f32(x).unwrap().is_finite());
        assert!(airy_bi_scaled_f32(x).unwrap().is_finite());
    }
}

#[test]
fn airy_equation_and_wronskian_hold_without_derivative_entry_points() {
    // FNLIB has no separately promoted real derivative driver.  Centre
    // differences provide an independent consistency check of the public
    // value API against y'' - x y = 0 and Ai Bi' - Ai' Bi = 1 / pi.
    let h = 2.0e-3;
    for x in [-1.0, 0.0, 1.0] {
        let ai = airy_ai(x).unwrap();
        let bi = airy_bi(x).unwrap();
        let ai_second = (airy_ai(x + h).unwrap() - 2.0 * ai + airy_ai(x - h).unwrap()) / h.powi(2);
        let bi_second = (airy_bi(x + h).unwrap() - 2.0 * bi + airy_bi(x - h).unwrap()) / h.powi(2);
        close(ai_second - x * ai, 0.0, 2.0e-6);
        close(bi_second - x * bi, 0.0, 2.0e-6);

        let wronskian = ai * derivative(|value| airy_bi(value).unwrap(), x)
            - derivative(|value| airy_ai(value).unwrap(), x) * bi;
        close(wronskian, 1.0 / std::f64::consts::PI, 3.0e-8);
    }
}
