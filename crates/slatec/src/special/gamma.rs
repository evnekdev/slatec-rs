//! Gamma, beta, and related scalar functions with documented fatal domains
//! rejected before the original FNLIB implementation is entered.

use slatec_sys::generated::scalar_functions as raw;

use super::{SpecialFunctionError, runtime};

fn gamma_argument(function: &'static str, x: f64) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive(function, "x", x, 30.0)
}

fn beta_arguments(function: &'static str, a: f64, b: f64) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive(function, "a", a, 30.0)?;
    runtime::bounded_positive(function, "b", b, 30.0)
}

fn incomplete_gamma_arguments(
    function: &'static str,
    a: f64,
    x: f64,
) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive(function, "a", a, 30.0)?;
    if x.is_finite() && (0.0..=30.0).contains(&x) {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument: "x",
            value: x,
        })
    }
}

/// Gamma for `0 < x <= 30`, using SLATEC `DGAMMA`.
pub fn gamma(x: f64) -> Result<f64, SpecialFunctionError> {
    gamma_argument("gamma", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: poles and conservative overflow inputs are rejected, the FNLIB
    // global state is serialized, and the scalar ABI/profile is validated.
    Ok(unsafe { raw::dgamma(&mut x) })
}

/// Reciprocal gamma for `0 < x <= 30`, using SLATEC `DGAMR`.
pub fn reciprocal_gamma(x: f64) -> Result<f64, SpecialFunctionError> {
    gamma_argument("reciprocal_gamma", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: DGAMR's legacy XERROR control manipulation is serialized; x is
    // in a checked non-pole domain and the raw scalar ABI is validated.
    Ok(unsafe { raw::dgamr(&mut x) })
}

/// Logarithm of the absolute gamma value for `0 < x <= 30`, using `DLNGAM`.
pub fn log_gamma(x: f64) -> Result<f64, SpecialFunctionError> {
    gamma_argument("log_gamma", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: x excludes DLngam's pole/error cases; runtime and ABI validated.
    Ok(unsafe { raw::dlngam(&mut x) })
}

/// Complete beta for positive parameters within the conservative safe range.
pub fn beta(a: f64, b: f64) -> Result<f64, SpecialFunctionError> {
    beta_arguments("beta", a, b)?;
    let _guard = runtime::lock_fnlib();
    let (mut a, mut b) = (a, b);
    // Safety: positive bounded arguments avoid DBETA's fatal parameter path;
    // the raw scalar pointers and serialized FNLIB state are valid.
    Ok(unsafe { raw::dbeta(&mut a, &mut b) })
}

/// Natural logarithm of beta for positive parameters, using SLATEC `DLBETA`.
pub fn log_beta(a: f64, b: f64) -> Result<f64, SpecialFunctionError> {
    beta_arguments("log_beta", a, b)?;
    let _guard = runtime::lock_fnlib();
    let (mut a, mut b) = (a, b);
    // Safety: checked positive parameters and validated FNLIB ABI/runtime.
    Ok(unsafe { raw::dlbeta(&mut a, &mut b) })
}

/// Regularized incomplete beta ratio for `0 <= x <= 1` and positive shapes.
pub fn regularized_beta(x: f64, a: f64, b: f64) -> Result<f64, SpecialFunctionError> {
    runtime::closed_unit("regularized_beta", "x", x)?;
    beta_arguments("regularized_beta", a, b)?;
    let _guard = runtime::lock_fnlib();
    let (mut x, mut a, mut b) = (x, a, b);
    // Safety: DBETAI's documented parameter domain was checked; raw pointers,
    // FNLIB initialization, and profile ABI were explicitly validated.
    Ok(unsafe { raw::dbetai(&mut x, &mut a, &mut b) })
}

/// Lower incomplete gamma integral for positive `a` and nonnegative `x`.
pub fn incomplete_gamma_lower(a: f64, x: f64) -> Result<f64, SpecialFunctionError> {
    incomplete_gamma_arguments("incomplete_gamma_lower", a, x)?;
    let _guard = runtime::lock_fnlib();
    let (mut a, mut x) = (a, x);
    // Safety: DGAMI's documented fatal domains are excluded and the validated
    // FNLIB state and raw scalar pointer ABI are used.
    Ok(unsafe { raw::dgami(&mut a, &mut x) })
}

/// Complementary incomplete gamma integral for positive `a` and nonnegative `x`.
pub fn incomplete_gamma_upper(a: f64, x: f64) -> Result<f64, SpecialFunctionError> {
    incomplete_gamma_arguments("incomplete_gamma_upper", a, x)?;
    let _guard = runtime::lock_fnlib();
    let (mut a, mut x) = (a, x);
    // Safety: conservative DGAMIC parameters avoid its documented singular
    // cases; runtime state and the raw ABI are validated.
    Ok(unsafe { raw::dgamic(&mut a, &mut x) })
}

/// Tricomi's incomplete gamma form for positive `a` and `0 < x <= 30`.
pub fn tricomi_incomplete_gamma(a: f64, x: f64) -> Result<f64, SpecialFunctionError> {
    incomplete_gamma_arguments("tricomi_incomplete_gamma", a, x)?;
    if x == 0.0 {
        return Err(SpecialFunctionError::Domain {
            function: "tricomi_incomplete_gamma",
            argument: "x",
            value: x,
        });
    }
    let _guard = runtime::lock_fnlib();
    let (mut a, mut x) = (a, x);
    // Safety: singular zero input is excluded; validated FNLIB and raw ABI.
    Ok(unsafe { raw::dgamit(&mut a, &mut x) })
}

/// Digamma for `0 < x <= 30`, using SLATEC `DPSI`.
pub fn digamma(x: f64) -> Result<f64, SpecialFunctionError> {
    gamma_argument("digamma", x)?;
    let _guard = runtime::lock_fnlib();
    let mut x = x;
    // Safety: poles are excluded; serialized FNLIB state and scalar ABI valid.
    Ok(unsafe { raw::dpsi(&mut x) })
}

/// Factorial for `0 <= n <= 170`, using SLATEC `DFAC`.
pub fn factorial(n: i32) -> Result<f64, SpecialFunctionError> {
    if !(0..=170).contains(&n) {
        return Err(SpecialFunctionError::Domain {
            function: "factorial",
            argument: "n",
            value: f64::from(n),
        });
    }
    let _guard = runtime::lock_fnlib();
    let mut n = runtime::integer("factorial", "n", n)?;
    // Safety: n is bounded and ABI-representable; DFAC profile/runtime valid.
    Ok(unsafe { raw::dfac(&mut n) })
}

/// Binomial coefficient for `0 <= m <= n <= 60`, using SLATEC `DBINOM`.
pub fn binomial_coefficient(n: i32, m: i32) -> Result<f64, SpecialFunctionError> {
    if n < 0 || m < 0 || m > n || n > 60 {
        return Err(SpecialFunctionError::Domain {
            function: "binomial_coefficient",
            argument: "n_or_m",
            value: f64::from(n.max(m)),
        });
    }
    let _guard = runtime::lock_fnlib();
    let (mut n, mut m) = (
        runtime::integer("binomial_coefficient", "n", n)?,
        runtime::integer("binomial_coefficient", "m", m)?,
    );
    // Safety: ordered, bounded integer inputs and validated FNLIB ABI/runtime.
    Ok(unsafe { raw::dbinom(&mut n, &mut m) })
}

#[cfg(feature = "special-functions-f32")]
fn gamma_argument_f32(function: &'static str, x: f32) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive_f32(function, "x", x, 30.0)
}

#[cfg(feature = "special-functions-f32")]
fn beta_arguments_f32(function: &'static str, a: f32, b: f32) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive_f32(function, "a", a, 30.0)?;
    runtime::bounded_positive_f32(function, "b", b, 30.0)
}

#[cfg(feature = "special-functions-f32")]
fn incomplete_gamma_arguments_f32(
    function: &'static str,
    a: f32,
    x: f32,
) -> Result<(), SpecialFunctionError> {
    runtime::bounded_positive_f32(function, "a", a, 30.0)?;
    if x.is_finite() && (0.0..=30.0).contains(&x) {
        Ok(())
    } else {
        Err(SpecialFunctionError::Domain {
            function,
            argument: "x",
            value: f64::from(x),
        })
    }
}

#[cfg(feature = "special-functions-f32")]
macro_rules! unary_f32 {
    ($name:ident, $raw:ident, $doc:literal) => {
        #[doc = $doc]
        pub fn $name(x: f32) -> Result<f32, SpecialFunctionError> {
            gamma_argument_f32(stringify!($name), x)?;
            let _guard = runtime::lock_fnlib();
            let mut x = x;
            // Safety: checked non-pole domain, valid scalar pointer, and
            // serialized corrected FNLIB runtime for the supported profile.
            Ok(unsafe { raw::$raw(&mut x) })
        }
    };
}

#[cfg(feature = "special-functions-f32")]
unary_f32!(
    gamma_f32,
    gamma,
    "Single-precision gamma for `0 < x <= 30`, using `GAMMA`."
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    reciprocal_gamma_f32,
    gamr,
    "Single-precision reciprocal gamma, using `GAMR`."
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    log_gamma_f32,
    alngam,
    "Single-precision log absolute gamma, using `ALNGAM`."
);
#[cfg(feature = "special-functions-f32")]
unary_f32!(
    digamma_f32,
    psi,
    "Single-precision digamma for positive x, using `PSI`."
);

#[cfg(feature = "special-functions-f32")]
/// Single-precision complete beta for positive bounded parameters.
pub fn beta_f32(a: f32, b: f32) -> Result<f32, SpecialFunctionError> {
    beta_arguments_f32("beta_f32", a, b)?;
    let _guard = runtime::lock_fnlib();
    let (mut a, mut b) = (a, b);
    // Safety: checked positive parameters, valid pointers, validated runtime.
    Ok(unsafe { raw::beta(&mut a, &mut b) })
}

#[cfg(feature = "special-functions-f32")]
/// Single-precision natural logarithm of beta for positive parameters.
pub fn log_beta_f32(a: f32, b: f32) -> Result<f32, SpecialFunctionError> {
    beta_arguments_f32("log_beta_f32", a, b)?;
    let _guard = runtime::lock_fnlib();
    let (mut a, mut b) = (a, b);
    // Safety: checked positive parameters, valid pointers, validated runtime.
    Ok(unsafe { raw::albeta(&mut a, &mut b) })
}

#[cfg(feature = "special-functions-f32")]
/// Single-precision regularized incomplete beta ratio on its documented domain.
pub fn regularized_beta_f32(x: f32, a: f32, b: f32) -> Result<f32, SpecialFunctionError> {
    runtime::closed_unit_f32("regularized_beta_f32", "x", x)?;
    beta_arguments_f32("regularized_beta_f32", a, b)?;
    let _guard = runtime::lock_fnlib();
    let (mut x, mut a, mut b) = (x, a, b);
    // Safety: BETAI's documented parameter domain, runtime, and ABI verified.
    Ok(unsafe { raw::betai(&mut x, &mut a, &mut b) })
}

#[cfg(feature = "special-functions-f32")]
macro_rules! incomplete_gamma_f32 {
    ($name:ident, $raw:ident, $doc:literal, $nonzero:expr) => {
        #[doc = $doc]
        pub fn $name(a: f32, x: f32) -> Result<f32, SpecialFunctionError> {
            incomplete_gamma_arguments_f32(stringify!($name), a, x)?;
            if $nonzero && x == 0.0 {
                return Err(SpecialFunctionError::Domain {
                    function: stringify!($name),
                    argument: "x",
                    value: 0.0,
                });
            }
            let _guard = runtime::lock_fnlib();
            let (mut a, mut x) = (a, x);
            // Safety: documented fatal parameters are checked and the scalar
            // pointers use the corrected serialized FNLIB profile.
            Ok(unsafe { raw::$raw(&mut a, &mut x) })
        }
    };
}

#[cfg(feature = "special-functions-f32")]
incomplete_gamma_f32!(
    incomplete_gamma_lower_f32,
    gami,
    "Single-precision lower incomplete gamma using `GAMI`.",
    false
);
#[cfg(feature = "special-functions-f32")]
incomplete_gamma_f32!(
    incomplete_gamma_upper_f32,
    gamic,
    "Single-precision complementary incomplete gamma using `GAMIC`.",
    false
);
#[cfg(feature = "special-functions-f32")]
incomplete_gamma_f32!(
    tricomi_incomplete_gamma_f32,
    gamit,
    "Single-precision Tricomi incomplete gamma using `GAMIT`.",
    true
);

#[cfg(feature = "special-functions-f32")]
/// Single-precision factorial for `0 <= n <= 33`, using SLATEC `FAC`.
pub fn factorial_f32(n: i32) -> Result<f32, SpecialFunctionError> {
    if !(0..=33).contains(&n) {
        return Err(SpecialFunctionError::Domain {
            function: "factorial_f32",
            argument: "n",
            value: f64::from(n),
        });
    }
    let _guard = runtime::lock_fnlib();
    let mut n = runtime::integer("factorial_f32", "n", n)?;
    // Safety: bounded ABI-representable integer and validated FNLIB runtime.
    Ok(unsafe { raw::fac(&mut n) })
}

#[cfg(feature = "special-functions-f32")]
/// Single-precision binomial coefficient for `0 <= m <= n <= 30`.
pub fn binomial_coefficient_f32(n: i32, m: i32) -> Result<f32, SpecialFunctionError> {
    if n < 0 || m < 0 || m > n || n > 30 {
        return Err(SpecialFunctionError::Domain {
            function: "binomial_coefficient_f32",
            argument: "n_or_m",
            value: f64::from(n.max(m)),
        });
    }
    let _guard = runtime::lock_fnlib();
    let (mut n, mut m) = (
        runtime::integer("binomial_coefficient_f32", "n", n)?,
        runtime::integer("binomial_coefficient_f32", "m", m)?,
    );
    // Safety: checked integer inputs and the validated FNLIB ABI/runtime.
    Ok(unsafe { raw::binom(&mut n, &mut m) })
}
