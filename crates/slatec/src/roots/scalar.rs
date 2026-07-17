use crate::callback_runtime::{self, CallbackFailure, F32Callback, F64Callback};
use slatec_sys::FortranInteger;

use super::{RootBracket, RootError, RootOptions, RootResult, RootStatus};

enum RootStage<T> {
    Endpoint {
        estimate: T,
        status: RootStatus,
    },
    NoSignChange,
    Native {
        estimate: T,
        other_endpoint: T,
        status: FortranInteger,
    },
}

fn callback_failure(failure: CallbackFailure) -> RootError {
    match failure {
        CallbackFailure::Panicked => RootError::CallbackPanicked,
        CallbackFailure::NonFinite => RootError::CallbackReturnedNonFinite,
        CallbackFailure::InvalidPointer => RootError::NativeContractViolation {
            detail: "native callback argument was null",
        },
    }
}

fn opposite_sign_f64(left: f64, right: f64) -> bool {
    left.is_sign_positive() != right.is_sign_positive()
}

fn opposite_sign_f32(left: f32, right: f32) -> bool {
    left.is_sign_positive() != right.is_sign_positive()
}

fn validate_f64(
    bracket: RootBracket<f64>,
    options: RootOptions<f64>,
) -> Result<(f64, f64, f64), RootError> {
    if !bracket.lower.is_finite() {
        return Err(RootError::NonFiniteEndpoint { argument: "lower" });
    }
    if !bracket.upper.is_finite() {
        return Err(RootError::NonFiniteEndpoint { argument: "upper" });
    }
    if bracket.lower == bracket.upper {
        return Err(RootError::InvalidBracket);
    }
    for (argument, value) in [
        ("relative", options.relative_tolerance),
        ("absolute", options.absolute_tolerance),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(RootError::InvalidTolerance { argument });
        }
    }
    let minimum = bracket.lower.min(bracket.upper);
    let maximum = bracket.lower.max(bracket.upper);
    let guess = match options.initial_guess {
        Some(value) if value.is_finite() && value > minimum && value < maximum => value,
        Some(_) => return Err(RootError::InvalidInitialGuess),
        // The FZERO prologue recommends B or C when no better interior guess
        // is known. C is therefore the deterministic safe default.
        None => bracket.upper,
    };
    Ok((
        guess,
        options.relative_tolerance,
        options.absolute_tolerance,
    ))
}

fn validate_f32(
    bracket: RootBracket<f32>,
    options: RootOptions<f32>,
) -> Result<(f32, f32, f32), RootError> {
    if !bracket.lower.is_finite() {
        return Err(RootError::NonFiniteEndpoint { argument: "lower" });
    }
    if !bracket.upper.is_finite() {
        return Err(RootError::NonFiniteEndpoint { argument: "upper" });
    }
    if bracket.lower == bracket.upper {
        return Err(RootError::InvalidBracket);
    }
    for (argument, value) in [
        ("relative", options.relative_tolerance),
        ("absolute", options.absolute_tolerance),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(RootError::InvalidTolerance { argument });
        }
    }
    let minimum = bracket.lower.min(bracket.upper);
    let maximum = bracket.lower.max(bracket.upper);
    let guess = match options.initial_guess {
        Some(value) if value.is_finite() && value > minimum && value < maximum => value,
        Some(_) => return Err(RootError::InvalidInitialGuess),
        None => bracket.upper,
    };
    Ok((
        guess,
        options.relative_tolerance,
        options.absolute_tolerance,
    ))
}

fn result_f64(stage: RootStage<f64>, evaluations: usize) -> Result<RootResult<f64>, RootError> {
    match stage {
        RootStage::Endpoint { estimate, status } => Ok(RootResult {
            estimate,
            bracket: RootBracket {
                lower: estimate,
                upper: estimate,
            },
            evaluations,
            status,
        }),
        RootStage::NoSignChange => Err(RootError::NoSignChange),
        RootStage::Native {
            estimate,
            other_endpoint,
            status,
        } => Ok(RootResult {
            estimate,
            bracket: RootBracket {
                lower: estimate,
                upper: other_endpoint,
            },
            evaluations,
            status: native_status(status)?,
        }),
    }
}

fn result_f32(stage: RootStage<f32>, evaluations: usize) -> Result<RootResult<f32>, RootError> {
    match stage {
        RootStage::Endpoint { estimate, status } => Ok(RootResult {
            estimate,
            bracket: RootBracket {
                lower: estimate,
                upper: estimate,
            },
            evaluations,
            status,
        }),
        RootStage::NoSignChange => Err(RootError::NoSignChange),
        RootStage::Native {
            estimate,
            other_endpoint,
            status,
        } => Ok(RootResult {
            estimate,
            bracket: RootBracket {
                lower: estimate,
                upper: other_endpoint,
            },
            evaluations,
            status: native_status(status)?,
        }),
    }
}

fn native_status(status: FortranInteger) -> Result<RootStatus, RootError> {
    match status {
        1 => Ok(RootStatus::Converged),
        2 => Ok(RootStatus::EndpointRoot),
        3 => Ok(RootStatus::PossibleSingularity),
        4 => Ok(RootStatus::NoSignChange),
        5 => Ok(RootStatus::MaximumEvaluations),
        value => Err(RootError::NativeStatus(value)),
    }
}

fn stage_f64(
    callback: F64Callback,
    lower: f64,
    upper: f64,
    guess: f64,
    relative: f64,
    absolute: f64,
) -> RootStage<f64> {
    let lower_value = callback.call(lower);
    if callback.failed() {
        return RootStage::NoSignChange;
    }
    if lower_value == 0.0 {
        return RootStage::Endpoint {
            estimate: lower,
            status: RootStatus::LowerEndpoint,
        };
    }
    let upper_value = callback.call(upper);
    if callback.failed() {
        return RootStage::NoSignChange;
    }
    if upper_value == 0.0 {
        return RootStage::Endpoint {
            estimate: upper,
            status: RootStatus::UpperEndpoint,
        };
    }
    if !opposite_sign_f64(lower_value, upper_value) {
        return RootStage::NoSignChange;
    }
    let mut lower = lower;
    let mut upper = upper;
    let mut guess = guess;
    let mut relative = relative;
    let mut absolute = absolute;
    let mut status = 0;
    // SAFETY: the shared scoped callback stays installed for the call; all
    // scalar pointers are valid and mutable; inputs were validated against the
    // reviewed DFZERO contract and the GNU MinGW raw ABI profile.
    unsafe {
        slatec_sys::roots::dfzero(
            callback.ffi(),
            &mut lower,
            &mut upper,
            &mut guess,
            &mut relative,
            &mut absolute,
            &mut status,
        );
    }
    RootStage::Native {
        estimate: lower,
        other_endpoint: upper,
        status,
    }
}

fn stage_f32(
    callback: F32Callback,
    lower: f32,
    upper: f32,
    guess: f32,
    relative: f32,
    absolute: f32,
) -> RootStage<f32> {
    let lower_value = callback.call(lower);
    if callback.failed() {
        return RootStage::NoSignChange;
    }
    if lower_value == 0.0 {
        return RootStage::Endpoint {
            estimate: lower,
            status: RootStatus::LowerEndpoint,
        };
    }
    let upper_value = callback.call(upper);
    if callback.failed() {
        return RootStage::NoSignChange;
    }
    if upper_value == 0.0 {
        return RootStage::Endpoint {
            estimate: upper,
            status: RootStatus::UpperEndpoint,
        };
    }
    if !opposite_sign_f32(lower_value, upper_value) {
        return RootStage::NoSignChange;
    }
    let mut lower = lower;
    let mut upper = upper;
    let mut guess = guess;
    let mut relative = relative;
    let mut absolute = absolute;
    let mut status = 0;
    // SAFETY: see stage_f64 for the reviewed FZERO invariants.
    unsafe {
        slatec_sys::roots::fzero(
            callback.ffi(),
            &mut lower,
            &mut upper,
            &mut guess,
            &mut relative,
            &mut absolute,
            &mut status,
        );
    }
    RootStage::Native {
        estimate: lower,
        other_endpoint: upper,
        status,
    }
}

/// Finds a bracketed double-precision scalar root with the original `DFZERO`.
///
/// The supplied endpoints are evaluated before native entry. Exact endpoint
/// roots return immediately; otherwise the safe API requires an opposite sign.
/// `DFZERO` has a fixed internal limit of more than 500 callback evaluations,
/// so there is intentionally no misleading caller-controlled limit option.
pub fn find_root<F>(
    function: F,
    bracket: RootBracket<f64>,
    options: RootOptions<f64>,
) -> Result<RootResult<f64>, RootError>
where
    F: FnMut(f64) -> f64,
{
    let (guess, relative, absolute) = validate_f64(bracket, options)?;
    let invocation = callback_runtime::with_f64(function, |callback| {
        stage_f64(
            callback,
            bracket.lower,
            bracket.upper,
            guess,
            relative,
            absolute,
        )
    })
    .map_err(|_| RootError::NestedNativeCallback)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    result_f64(invocation.value, invocation.evaluations)
}

/// Finds a bracketed single-precision scalar root with the original `FZERO`.
pub fn find_root_f32<F>(
    function: F,
    bracket: RootBracket<f32>,
    options: RootOptions<f32>,
) -> Result<RootResult<f32>, RootError>
where
    F: FnMut(f32) -> f32,
{
    let (guess, relative, absolute) = validate_f32(bracket, options)?;
    let invocation = callback_runtime::with_f32(function, |callback| {
        stage_f32(
            callback,
            bracket.lower,
            bracket.upper,
            guess,
            relative,
            absolute,
        )
    })
    .map_err(|_| RootError::NestedNativeCallback)?;
    if let Some(failure) = invocation.failure {
        return Err(callback_failure(failure));
    }
    result_f32(invocation.value, invocation.evaluations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_brackets_tolerances_and_guess_without_native_linking() {
        let bracket = RootBracket {
            lower: 1.0,
            upper: 2.0,
        };
        assert_eq!(
            validate_f64(bracket, RootOptions::default()).unwrap().0,
            2.0
        );
        assert!(matches!(
            validate_f64(
                RootBracket {
                    lower: 1.0,
                    upper: 1.0
                },
                RootOptions::default()
            ),
            Err(RootError::InvalidBracket)
        ));
        assert!(matches!(
            validate_f64(
                bracket,
                RootOptions {
                    initial_guess: Some(1.0),
                    ..RootOptions::default()
                }
            ),
            Err(RootError::InvalidInitialGuess)
        ));
        assert!(opposite_sign_f64(-1.0, 1.0));
        assert!(!opposite_sign_f64(1.0, 1.0));
        // Exact signed zeros are handled as endpoint roots before the
        // multiplication-free sign comparison is reached.
        assert_eq!(-0.0_f64, 0.0);
    }

    #[test]
    fn maps_each_documented_native_status() {
        assert_eq!(native_status(1), Ok(RootStatus::Converged));
        assert_eq!(native_status(2), Ok(RootStatus::EndpointRoot));
        assert_eq!(native_status(3), Ok(RootStatus::PossibleSingularity));
        assert_eq!(native_status(4), Ok(RootStatus::NoSignChange));
        assert_eq!(native_status(5), Ok(RootStatus::MaximumEvaluations));
    }
}
