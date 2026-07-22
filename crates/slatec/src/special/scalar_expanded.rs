//! Additional real scalar special functions with reviewed native contracts.
//!
//! This module exposes the logarithmic and Spence integrals plus Carlson's
//! symmetric elliptic integrals.  Every call is serialized: the selected
//! native routines use `SAVE`d initialization and reach SLATEC's process-wide
//! XERROR runtime.  The Carlson wrappers preserve their native `IER` outcome
//! as [`SpecialFunctionError::NativeStatus`] after scoped XERROR restoration.

use slatec_sys::{FortranInteger, special as raw};

use super::{SpecialFunctionError, runtime};

const LOG_INTEGRAL_MIN_F64: f64 = 2.061_153_622_438_558e-9;
const LOG_INTEGRAL_MAX_F64: f64 = 4.851_651_954_097_903e8;
const LOG_INTEGRAL_MIN_F32: f32 = 2.061_153_7e-9;
const LOG_INTEGRAL_MAX_F32: f32 = 4.851_652e8;

fn logarithmic_integral_input(function: &'static str, x: f64) -> Result<(), SpecialFunctionError> {
    if x.is_finite() && x >= LOG_INTEGRAL_MIN_F64 && x <= LOG_INTEGRAL_MAX_F64 && x != 1.0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument: "x",
            value: x,
        })
    }
}

#[cfg(feature = "special-f32")]
fn logarithmic_integral_input_f32(
    function: &'static str,
    x: f32,
) -> Result<(), SpecialFunctionError> {
    if x.is_finite() && x >= LOG_INTEGRAL_MIN_F32 && x <= LOG_INTEGRAL_MAX_F32 && x != 1.0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument: "x",
            value: f64::from(x),
        })
    }
}

fn finite(
    function: &'static str,
    argument: &'static str,
    value: f64,
) -> Result<(), SpecialFunctionError> {
    runtime::finite(function, argument, value)
}

#[cfg(feature = "special-f32")]
fn finite_f32(
    function: &'static str,
    argument: &'static str,
    value: f32,
) -> Result<(), SpecialFunctionError> {
    runtime::finite_f32(function, argument, value)
}

fn nonnegative(
    function: &'static str,
    argument: &'static str,
    value: f64,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value,
        })
    }
}

fn positive(
    function: &'static str,
    argument: &'static str,
    value: f64,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value,
        })
    }
}

#[cfg(feature = "special-f32")]
fn nonnegative_f32(
    function: &'static str,
    argument: &'static str,
    value: f32,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value: f64::from(value),
        })
    }
}

#[cfg(feature = "special-f32")]
fn positive_f32(
    function: &'static str,
    argument: &'static str,
    value: f32,
) -> Result<(), SpecialFunctionError> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument,
            value: f64::from(value),
        })
    }
}

fn status(function: &'static str, ier: FortranInteger) -> Result<(), SpecialFunctionError> {
    if ier == 0 {
        Ok(())
    } else {
        Err(SpecialFunctionError::NativeStatus {
            function,
            status: ier,
        })
    }
}

/// Logarithmic integral \(\operatorname{li}(x)=\operatorname{Ei}(\ln x)\)
/// using SLATEC `DLI`.
///
/// The real domain excludes `x <= 0` and the logarithmic singularity at
/// `x = 1`.  This wrapper also keeps `ln(x)` inside the reviewed `DEI`
/// envelope `[-20, 20]`; inputs outside it return [`SpecialFunctionError::Domain`]
/// before native execution.
pub fn logarithmic_integral(x: f64) -> Result<f64, SpecialFunctionError> {
    logarithmic_integral_input("logarithmic_integral", x)?;
    let _runtime = runtime::lock_fnlib();
    let mut x = x;
    // Safety: `x` is in the reviewed DLI/DEI envelope, and the pointer is a
    // valid mutable scalar under the validated GNU Fortran ABI.
    Ok(unsafe { raw::dli(&mut x) })
}

/// Spence's integral \(\int_0^x-\ln(1-t)/t\,dt\) using SLATEC `DSPENC`.
///
/// The native definition has a real continuation for every finite real `x`;
/// `x = 1` returns \(\pi^2/6\).  The result is not an automatically scaled
/// dilogarithm and no scaling is applied by this wrapper.
pub fn spence_integral(x: f64) -> Result<f64, SpecialFunctionError> {
    finite("spence_integral", "x", x)?;
    let _runtime = runtime::lock_fnlib();
    let mut x = x;
    // Safety: DSPENC accepts each finite real scalar and uses no caller-owned
    // workspace.  The process-global FNLIB state is serialized.
    Ok(unsafe { raw::dspenc(&mut x) })
}

/// Carlson's degenerate symmetric elliptic integral \(R_C(x,y)\), using
/// SLATEC `DRC`.
///
/// The mathematical precondition is `x >= 0`, `y > 0`.  Native range
/// screening is retained: statuses 1, 2, and 3 report invalid signs, a
/// too-small combination, and an excessive magnitude respectively.
pub fn carlson_rc(x: f64, y: f64) -> Result<f64, SpecialFunctionError> {
    nonnegative("carlson_rc", "x", x)?;
    positive("carlson_rc", "y", y)?;
    let _runtime = runtime::lock_fnlib();
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let (mut x, mut y, mut ier) = (x, y, 0);
    // Safety: pointers reference distinct initialized scalars.  Scoped
    // XERROR converts DRC's documented level-one IER path into a return.
    let value = unsafe { raw::drc(&mut x, &mut y, &mut ier) };
    status("carlson_rc", ier)?;
    Ok(value)
}

/// Carlson's symmetric elliptic integral \(R_F(x,y,z)\), using SLATEC
/// `DRF`.
///
/// All arguments must be nonnegative and no pair may be simultaneously zero.
/// Native range statuses are returned as [`SpecialFunctionError::NativeStatus`].
pub fn carlson_rf(x: f64, y: f64, z: f64) -> Result<f64, SpecialFunctionError> {
    nonnegative("carlson_rf", "x", x)?;
    nonnegative("carlson_rf", "y", y)?;
    nonnegative("carlson_rf", "z", z)?;
    if x + y == 0.0 || x + z == 0.0 || y + z == 0.0 {
        return Err(SpecialFunctionError::Domain {
            function: "carlson_rf",
            argument: "x, y, z",
            value: 0.0,
        });
    }
    let _runtime = runtime::lock_fnlib();
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let (mut x, mut y, mut z, mut ier) = (x, y, z, 0);
    // Safety: the checked scalars and output status have the reviewed ABI.
    let value = unsafe { raw::drf(&mut x, &mut y, &mut z, &mut ier) };
    status("carlson_rf", ier)?;
    Ok(value)
}

/// Carlson's symmetric elliptic integral \(R_D(x,y,z)\), using SLATEC
/// `DRD`.
///
/// `x` and `y` must be nonnegative, `z` must be positive, and `x + y` must
/// be positive.  Native range statuses remain explicit rather than being
/// folded into a numerical result.
pub fn carlson_rd(x: f64, y: f64, z: f64) -> Result<f64, SpecialFunctionError> {
    nonnegative("carlson_rd", "x", x)?;
    nonnegative("carlson_rd", "y", y)?;
    positive("carlson_rd", "z", z)?;
    if x + y == 0.0 {
        return Err(SpecialFunctionError::Domain {
            function: "carlson_rd",
            argument: "x, y",
            value: 0.0,
        });
    }
    let _runtime = runtime::lock_fnlib();
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let (mut x, mut y, mut z, mut ier) = (x, y, z, 0);
    // Safety: all native scalar pointers are valid and the level-one XERROR
    // status path is contained by the scoped restoration guard.
    let value = unsafe { raw::drd(&mut x, &mut y, &mut z, &mut ier) };
    status("carlson_rd", ier)?;
    Ok(value)
}

/// Carlson's symmetric elliptic integral \(R_J(x,y,z,p)\), using SLATEC
/// `DRJ`.
///
/// `x`, `y`, and `z` must be nonnegative, `p` must be positive, and no pair
/// among `x`, `y`, and `z` may be simultaneously zero.
pub fn carlson_rj(x: f64, y: f64, z: f64, p: f64) -> Result<f64, SpecialFunctionError> {
    nonnegative("carlson_rj", "x", x)?;
    nonnegative("carlson_rj", "y", y)?;
    nonnegative("carlson_rj", "z", z)?;
    positive("carlson_rj", "p", p)?;
    if x + y == 0.0 || x + z == 0.0 || y + z == 0.0 {
        return Err(SpecialFunctionError::Domain {
            function: "carlson_rj",
            argument: "x, y, z",
            value: 0.0,
        });
    }
    let _runtime = runtime::lock_fnlib();
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let (mut x, mut y, mut z, mut p, mut ier) = (x, y, z, p, 0);
    // Safety: the reviewed DRJ ABI uses four scalar inputs and one status
    // output; all point to valid local storage for the complete call.
    let value = unsafe { raw::drj(&mut x, &mut y, &mut z, &mut p, &mut ier) };
    status("carlson_rj", ier)?;
    Ok(value)
}

#[cfg(feature = "special-f32")]
/// Single-precision logarithmic integral using SLATEC `ALI`.
pub fn logarithmic_integral_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    logarithmic_integral_input_f32("logarithmic_integral_f32", x)?;
    let _runtime = runtime::lock_fnlib();
    let mut x = x;
    // Safety: the scalar is within the reviewed ALI/EI envelope.
    Ok(unsafe { raw::ali(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Single-precision Spence's integral using SLATEC `SPENC`.
pub fn spence_integral_f32(x: f32) -> Result<f32, SpecialFunctionError> {
    finite_f32("spence_integral_f32", "x", x)?;
    let _runtime = runtime::lock_fnlib();
    let mut x = x;
    // Safety: SPENC receives one valid finite scalar pointer.
    Ok(unsafe { raw::spenc(&mut x) })
}

#[cfg(feature = "special-f32")]
/// Single-precision Carlson \(R_C(x,y)\) using SLATEC `RC`.
pub fn carlson_rc_f32(x: f32, y: f32) -> Result<f32, SpecialFunctionError> {
    nonnegative_f32("carlson_rc_f32", "x", x)?;
    positive_f32("carlson_rc_f32", "y", y)?;
    let _runtime = runtime::lock_fnlib();
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let (mut x, mut y, mut ier) = (x, y, 0);
    // Safety: reviewed RC scalar/status ABI with scoped XERROR restoration.
    let value = unsafe { raw::rc(&mut x, &mut y, &mut ier) };
    status("carlson_rc_f32", ier)?;
    Ok(value)
}

#[cfg(feature = "special-f32")]
/// Single-precision Carlson \(R_F(x,y,z)\) using SLATEC `RF`.
pub fn carlson_rf_f32(x: f32, y: f32, z: f32) -> Result<f32, SpecialFunctionError> {
    nonnegative_f32("carlson_rf_f32", "x", x)?;
    nonnegative_f32("carlson_rf_f32", "y", y)?;
    nonnegative_f32("carlson_rf_f32", "z", z)?;
    if x + y == 0.0 || x + z == 0.0 || y + z == 0.0 {
        return Err(SpecialFunctionError::Domain {
            function: "carlson_rf_f32",
            argument: "x, y, z",
            value: 0.0,
        });
    }
    let _runtime = runtime::lock_fnlib();
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let (mut x, mut y, mut z, mut ier) = (x, y, z, 0);
    // Safety: reviewed RF scalar/status ABI with scoped XERROR restoration.
    let value = unsafe { raw::rf(&mut x, &mut y, &mut z, &mut ier) };
    status("carlson_rf_f32", ier)?;
    Ok(value)
}

#[cfg(feature = "special-f32")]
/// Single-precision Carlson \(R_D(x,y,z)\) using SLATEC `RD`.
pub fn carlson_rd_f32(x: f32, y: f32, z: f32) -> Result<f32, SpecialFunctionError> {
    nonnegative_f32("carlson_rd_f32", "x", x)?;
    nonnegative_f32("carlson_rd_f32", "y", y)?;
    positive_f32("carlson_rd_f32", "z", z)?;
    if x + y == 0.0 {
        return Err(SpecialFunctionError::Domain {
            function: "carlson_rd_f32",
            argument: "x, y",
            value: 0.0,
        });
    }
    let _runtime = runtime::lock_fnlib();
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let (mut x, mut y, mut z, mut ier) = (x, y, z, 0);
    // Safety: reviewed RD scalar/status ABI with scoped XERROR restoration.
    let value = unsafe { raw::rd(&mut x, &mut y, &mut z, &mut ier) };
    status("carlson_rd_f32", ier)?;
    Ok(value)
}

#[cfg(feature = "special-f32")]
/// Single-precision Carlson \(R_J(x,y,z,p)\) using SLATEC `RJ`.
pub fn carlson_rj_f32(x: f32, y: f32, z: f32, p: f32) -> Result<f32, SpecialFunctionError> {
    nonnegative_f32("carlson_rj_f32", "x", x)?;
    nonnegative_f32("carlson_rj_f32", "y", y)?;
    nonnegative_f32("carlson_rj_f32", "z", z)?;
    positive_f32("carlson_rj_f32", "p", p)?;
    if x + y == 0.0 || x + z == 0.0 || y + z == 0.0 {
        return Err(SpecialFunctionError::Domain {
            function: "carlson_rj_f32",
            argument: "x, y, z",
            value: 0.0,
        });
    }
    let _runtime = runtime::lock_fnlib();
    let _xerror = crate::runtime::permit_recoverable_native_statuses();
    let (mut x, mut y, mut z, mut p, mut ier) = (x, y, z, p, 0);
    // Safety: reviewed RJ scalar/status ABI with scoped XERROR restoration.
    let value = unsafe { raw::rj(&mut x, &mut y, &mut z, &mut p, &mut ier) };
    status("carlson_rj_f32", ier)?;
    Ok(value)
}
