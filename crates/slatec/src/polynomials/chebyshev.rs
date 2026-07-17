//! Checked Chebyshev-series evaluation over the SLATEC `[-1, 1]` contract.

use slatec_sys::families::special_polynomials as raw;

use crate::special::{SpecialFunctionError, runtime};

fn validate(
    function: &'static str,
    x: f64,
    coefficient_count: usize,
) -> Result<(), SpecialFunctionError> {
    if !x.is_finite() || !(-1.0..=1.0).contains(&x) {
        return Err(SpecialFunctionError::Domain {
            function,
            argument: "x",
            value: x,
        });
    }
    if coefficient_count == 0 || coefficient_count > 1000 {
        return Err(SpecialFunctionError::Domain {
            function,
            argument: "coefficients.len()",
            value: coefficient_count as f64,
        });
    }
    Ok(())
}

/// Evaluates a Chebyshev series at `x` in `[-1, 1]` using SLATEC `DCSEVL`.
pub fn chebyshev_series(x: f64, coefficients: &[f64]) -> Result<f64, SpecialFunctionError> {
    validate("chebyshev_series", x, coefficients.len())?;
    let _guard = runtime::lock_fnlib();
    let (mut x, mut n) = (
        x,
        runtime::integer(
            "chebyshev_series",
            "coefficients.len()",
            i32::try_from(coefficients.len()).map_err(|_| {
                SpecialFunctionError::IntegerOverflow {
                    function: "chebyshev_series",
                    argument: "coefficients.len()",
                }
            })?,
        )?,
    );
    // Safety: x and coefficient count meet DCSEVL's documented contract;
    // coefficients remain immutably borrowed for the call, and DCSEVL only
    // reads this array through the validated profile ABI.
    Ok(unsafe { raw::dcsevl(&mut x, coefficients.as_ptr().cast_mut(), &mut n) })
}

#[cfg(feature = "special-f32")]
/// Evaluates a single-precision Chebyshev series on `[-1, 1]` using `CSEVL`.
pub fn chebyshev_series_f32(x: f32, coefficients: &[f32]) -> Result<f32, SpecialFunctionError> {
    validate("chebyshev_series_f32", f64::from(x), coefficients.len())?;
    let _guard = runtime::lock_fnlib();
    let (mut x, mut n) = (
        x,
        runtime::integer(
            "chebyshev_series_f32",
            "coefficients.len()",
            i32::try_from(coefficients.len()).map_err(|_| {
                SpecialFunctionError::IntegerOverflow {
                    function: "chebyshev_series_f32",
                    argument: "coefficients.len()",
                }
            })?,
        )?,
    );
    // Safety: x and count meet CSEVL's documented contract; coefficients are
    // immutably borrowed and CSEVL only reads the validated raw array ABI.
    Ok(unsafe { raw::csevl(&mut x, coefficients.as_ptr().cast_mut(), &mut n) })
}
