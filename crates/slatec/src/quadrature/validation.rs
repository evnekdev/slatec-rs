#![allow(dead_code)] // Narrow quadrature features intentionally prune helpers.

use alloc::vec::Vec;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use super::{IntegrationError, IntegrationOptions};

pub(crate) struct Workspace<T> {
    pub limit: FortranInteger,
    pub lenw: FortranInteger,
    pub iwork: Vec<FortranInteger>,
    pub work: Vec<T>,
}

pub(crate) fn finite_bounds<T: Into<f64> + Copy>(
    lower: T,
    upper: T,
) -> Result<(), IntegrationError> {
    if lower.into().is_finite() && upper.into().is_finite() {
        Ok(())
    } else {
        Err(IntegrationError::InvalidBounds)
    }
}

pub(crate) fn principal_value_bounds<T: Into<f64> + Copy>(
    lower: T,
    upper: T,
    singularity: T,
) -> Result<(), IntegrationError> {
    finite_bounds(lower, upper)?;
    let (lower, upper, singularity) = (lower.into(), upper.into(), singularity.into());
    if !singularity.is_finite() {
        return Err(IntegrationError::InvalidBounds);
    }
    let minimum = lower.min(upper);
    let maximum = lower.max(upper);
    if singularity > minimum && singularity < maximum {
        Ok(())
    } else {
        Err(IntegrationError::InvalidBounds)
    }
}

fn validate_tolerances(
    absolute: f64,
    relative: f64,
    minimum_relative: f64,
) -> Result<(), IntegrationError> {
    if !absolute.is_finite()
        || !relative.is_finite()
        || absolute < 0.0
        || relative < 0.0
        || (absolute == 0.0 && relative < minimum_relative)
    {
        Err(IntegrationError::InvalidTolerance)
    } else {
        Ok(())
    }
}

pub(crate) fn standard_tolerances_f64(
    absolute: f64,
    relative: f64,
) -> Result<(f64, f64), IntegrationError> {
    validate_tolerances(absolute, relative, (50.0 * f64::EPSILON).max(0.5e-28))?;
    Ok((absolute, relative))
}

pub(crate) fn standard_tolerances_f32(
    absolute: f64,
    relative: f64,
) -> Result<(f32, f32), IntegrationError> {
    validate_tolerances(
        absolute,
        relative,
        (50.0 * f64::from(f32::EPSILON)).max(0.5e-28),
    )?;
    if absolute > f64::from(f32::MAX) || relative > f64::from(f32::MAX) {
        return Err(IntegrationError::InvalidTolerance);
    }
    let absolute = absolute as f32;
    let relative = relative as f32;
    validate_tolerances(
        f64::from(absolute),
        f64::from(relative),
        (50.0 * f64::from(f32::EPSILON)).max(0.5e-28),
    )?;
    Ok((absolute, relative))
}

pub(crate) fn positive_tolerance_f64(value: f64) -> Result<f64, IntegrationError> {
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(IntegrationError::InvalidTolerance)
    }
}

pub(crate) fn positive_tolerance_f32(value: f64) -> Result<f32, IntegrationError> {
    let value = positive_tolerance_f64(value)?;
    if value > f64::from(f32::MAX) {
        return Err(IntegrationError::InvalidTolerance);
    }
    let value = value as f32;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err(IntegrationError::InvalidTolerance)
    }
}

pub(crate) fn workspace_f64(
    options: IntegrationOptions,
) -> Result<(Workspace<f64>, f64, f64), IntegrationError> {
    validate_tolerances(
        options.absolute_tolerance,
        options.relative_tolerance,
        (50.0 * f64::EPSILON).max(0.5e-28),
    )?;
    Ok((
        workspace(options.limit)?,
        options.absolute_tolerance,
        options.relative_tolerance,
    ))
}

pub(crate) fn workspace_f32(
    options: IntegrationOptions,
) -> Result<(Workspace<f32>, f32, f32), IntegrationError> {
    validate_tolerances(
        options.absolute_tolerance,
        options.relative_tolerance,
        (50.0 * f64::from(f32::EPSILON)).max(0.5e-28),
    )?;
    if options.absolute_tolerance > f64::from(f32::MAX)
        || options.relative_tolerance > f64::from(f32::MAX)
    {
        return Err(IntegrationError::InvalidTolerance);
    }
    let absolute = options.absolute_tolerance as f32;
    let relative = options.relative_tolerance as f32;
    validate_tolerances(
        f64::from(absolute),
        f64::from(relative),
        (50.0 * f64::from(f32::EPSILON)).max(0.5e-28),
    )?;
    Ok((workspace(options.limit)?, absolute, relative))
}

pub(crate) fn allocated_workspace<T: Default + Clone>(
    integer_count: usize,
    work_count: usize,
    moment_workspace: bool,
) -> Result<(Vec<FortranInteger>, Vec<T>), IntegrationError> {
    let error = if moment_workspace {
        IntegrationError::MomentWorkspaceTooLarge
    } else {
        IntegrationError::WorkspaceTooLarge
    };
    let mut iwork = Vec::new();
    iwork
        .try_reserve_exact(integer_count)
        .map_err(|_| error.clone())?;
    iwork.resize(integer_count, 0);
    let mut work = Vec::new();
    work.try_reserve_exact(work_count)
        .map_err(|_| error.clone())?;
    work.resize(work_count, T::default());
    Ok((iwork, work))
}

pub(crate) fn to_fortran(
    value: usize,
    moment_workspace: bool,
) -> Result<FortranInteger, IntegrationError> {
    to_fortran_integer(value).map_err(|_| {
        if moment_workspace {
            IntegrationError::MomentWorkspaceTooLarge
        } else {
            IntegrationError::IntegerOverflow
        }
    })
}

fn workspace<T: Default + Clone>(limit: usize) -> Result<Workspace<T>, IntegrationError> {
    if limit == 0 {
        return Err(IntegrationError::WorkspaceTooLarge);
    }
    let work_len = limit
        .checked_mul(4)
        .ok_or(IntegrationError::WorkspaceTooLarge)?;
    let limit_integer = to_fortran_integer(limit).map_err(|_| IntegrationError::IntegerOverflow)?;
    let lenw_integer =
        to_fortran_integer(work_len).map_err(|_| IntegrationError::IntegerOverflow)?;
    let mut iwork = Vec::new();
    iwork
        .try_reserve_exact(limit)
        .map_err(|_| IntegrationError::WorkspaceTooLarge)?;
    iwork.resize(limit, 0);
    let mut work = Vec::new();
    work.try_reserve_exact(work_len)
        .map_err(|_| IntegrationError::WorkspaceTooLarge)?;
    work.resize(work_len, T::default());
    Ok(Workspace {
        limit: limit_integer,
        lenw: lenw_integer,
        iwork,
        work,
    })
}

pub(crate) fn checked_add(
    left: usize,
    right: usize,
    moment_workspace: bool,
) -> Result<usize, IntegrationError> {
    left.checked_add(right).ok_or(if moment_workspace {
        IntegrationError::MomentWorkspaceTooLarge
    } else {
        IntegrationError::WorkspaceTooLarge
    })
}

pub(crate) fn checked_mul(
    left: usize,
    right: usize,
    moment_workspace: bool,
) -> Result<usize, IntegrationError> {
    left.checked_mul(right).ok_or(if moment_workspace {
        IntegrationError::MomentWorkspaceTooLarge
    } else {
        IntegrationError::WorkspaceTooLarge
    })
}

pub(crate) fn status(status: FortranInteger) -> Result<(), IntegrationError> {
    match status {
        0 => Ok(()),
        1 => Err(IntegrationError::MaximumSubdivisions),
        2 => Err(IntegrationError::RoundoffDetected),
        3 => Err(IntegrationError::BadIntegrandBehavior),
        4 => Err(IntegrationError::NonConvergence),
        5 => Err(IntegrationError::DivergentOrSlowlyConvergent),
        6 => Err(IntegrationError::NativeContractViolation),
        value => Err(IntegrationError::NativeStatus(value)),
    }
}

pub(crate) fn output_count(
    value: FortranInteger,
    upper: Option<usize>,
) -> Result<usize, IntegrationError> {
    let value = usize::try_from(value).map_err(|_| IntegrationError::NativeContractViolation)?;
    if upper.is_some_and(|upper| value > upper) {
        return Err(IntegrationError::NativeContractViolation);
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quadrature::{IntegrationOptions, IntegrationRule};

    fn options() -> IntegrationOptions {
        IntegrationOptions {
            absolute_tolerance: 0.0,
            relative_tolerance: 1.0e-8,
            limit: 50,
            rule: IntegrationRule::Points21,
        }
    }

    #[test]
    fn validates_tolerances_and_workspace_arithmetic() {
        assert_eq!(workspace_f64(options()).unwrap().0.work.len(), 200);
        let mut invalid = options();
        invalid.relative_tolerance = 0.0;
        assert!(matches!(
            workspace_f64(invalid),
            Err(IntegrationError::InvalidTolerance)
        ));
        invalid = options();
        invalid.absolute_tolerance = -1.0;
        assert!(matches!(
            workspace_f64(invalid),
            Err(IntegrationError::InvalidTolerance)
        ));
        invalid = options();
        invalid.limit = usize::MAX;
        assert!(workspace_f64(invalid).is_err());

        invalid = options();
        invalid.absolute_tolerance = 1.0e-100;
        invalid.relative_tolerance = 0.0;
        assert!(matches!(
            workspace_f32(invalid),
            Err(IntegrationError::InvalidTolerance)
        ));
    }

    #[test]
    fn maps_status_and_validates_native_indices() {
        assert_eq!(status(0), Ok(()));
        assert_eq!(status(1), Err(IntegrationError::MaximumSubdivisions));
        assert_eq!(status(6), Err(IntegrationError::NativeContractViolation));
        assert_eq!(output_count(3, Some(4)), Ok(3));
        assert!(output_count(-1, None).is_err());
        assert!(output_count(5, Some(4)).is_err());
    }

    #[test]
    fn accepts_reversed_finite_bounds_and_checks_principal_value_location() {
        assert!(finite_bounds(1.0, -1.0).is_ok());
        assert!(principal_value_bounds(1.0, -1.0, 0.0).is_ok());
        assert!(principal_value_bounds(-1.0, 1.0, 1.0).is_err());
        assert!(finite_bounds(f64::INFINITY, 1.0).is_err());
    }
}
