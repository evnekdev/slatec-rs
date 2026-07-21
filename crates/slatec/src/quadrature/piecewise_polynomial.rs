//! Multiplicative quadrature over the checked SLATEC PP representation.

use core::ops::RangeInclusive;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::interpolation::piecewise_polynomial::PiecewisePolynomial;

use super::IntegrationError;
use super::callback;

/// Exact completion status reported by `DPFQAD`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PiecewiseQuadratureStatus {
    /// Every subinterval met the requested tolerance.
    Converged,
    /// At least one subinterval did not meet the requested tolerance.
    ToleranceNotMet,
}

/// Result of a `DPFQAD` multiplicative PP integral.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PiecewiseQuadratureResult {
    /// Integral of the callback times the requested PP derivative.
    pub value: f64,
    /// Exact reviewed native completion state.
    pub status: PiecewiseQuadratureStatus,
}

fn integer(value: usize) -> Result<FortranInteger, IntegrationError> {
    to_fortran_integer(value).map_err(|_| IntegrationError::IntegerOverflow)
}

fn validate_inputs(
    polynomial: &PiecewisePolynomial<f64>,
    derivative_order: usize,
    lower: f64,
    upper: f64,
    tolerance: f64,
) -> Result<(), IntegrationError> {
    if derivative_order >= polynomial.order() {
        return Err(IntegrationError::InvalidDerivativeOrder);
    }
    if !lower.is_finite() || !upper.is_finite() {
        return Err(IntegrationError::InvalidBounds);
    }
    if !tolerance.is_finite() || tolerance <= 0.0 || tolerance > 0.1 {
        return Err(IntegrationError::InvalidTolerance);
    }
    Ok(())
}

/// Integrates a scalar callback times a derivative of a checked double-
/// precision piecewise polynomial using original SLATEC `DPFQAD`.
///
/// `derivative_order = 0` selects the polynomial itself. The supplied PP
/// object owns strictly increasing breakpoints and a checked column-major
/// coefficient representation, so no raw coefficient dimensions or native
/// work arrays are exposed. The interval may extend beyond the PP domain when
/// the multiplicative callback is defined there, matching the source contract.
///
/// The callback is synchronous and is serialized with every other hosted
/// callback-bearing SLATEC operation. A panic or non-finite callback value is
/// reported as [`IntegrationError`] after native return and cannot unwind
/// through Fortran.
///
/// # Example
///
/// ```no_run
/// # fn example() -> Result<(), slatec::quadrature::IntegrationError> {
/// use slatec::interpolation::piecewise_polynomial::PiecewisePolynomial;
/// use slatec::quadrature::integrate_piecewise_polynomial;
///
/// let constant = PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0], 1)
///     .expect("valid constant PP representation");
/// let result = integrate_piecewise_polynomial(&constant, 0, 0.0..=1.0, 1.0e-8, |x| x * x)?;
/// assert!((result.value - 1.0 / 3.0).abs() < 1.0e-6);
/// # Ok(())
/// # }
/// ```
pub fn integrate_piecewise_polynomial<F>(
    polynomial: &PiecewisePolynomial<f64>,
    derivative_order: usize,
    interval: RangeInclusive<f64>,
    tolerance: f64,
    function: F,
) -> Result<PiecewiseQuadratureResult, IntegrationError>
where
    F: FnMut(f64) -> f64,
{
    let mut lower = *interval.start();
    let mut upper = *interval.end();
    validate_inputs(polynomial, derivative_order, lower, upper, tolerance)?;
    let mut leading_dimension = integer(polynomial.order())?;
    let mut piece_count = integer(polynomial.piece_count())?;
    let mut order = integer(polynomial.order())?;
    let mut derivative_order = integer(derivative_order)?;
    let mut integral = 0.0;
    let mut status = 0;
    let mut tolerance = tolerance;

    let ((), failure) = callback::with_f64(function, |callback| {
        // SAFETY: PiecewisePolynomial validates its owned PP representation;
        // the checked integer conversions and callback scope satisfy DPFQAD.
        unsafe {
            slatec_sys::quadrature::dpfqad(
                callback,
                &mut leading_dimension,
                polynomial
                    .coefficients_for_piece(0)
                    .expect("checked PP always has the first coefficient")
                    .as_ptr()
                    .cast_mut(),
                polynomial.breakpoints().as_ptr().cast_mut(),
                &mut piece_count,
                &mut order,
                &mut derivative_order,
                &mut lower,
                &mut upper,
                &mut tolerance,
                &mut integral,
                &mut status,
            );
        }
    })?;
    if let Some(failure) = failure {
        return Err(callback::failure_error(failure));
    }
    if !integral.is_finite() {
        return Err(IntegrationError::NativeContractViolation);
    }
    let status = match status {
        1 => PiecewiseQuadratureStatus::Converged,
        2 => PiecewiseQuadratureStatus::ToleranceNotMet,
        value => return Err(IntegrationError::NativeStatus(value)),
    };
    Ok(PiecewiseQuadratureResult {
        value: integral,
        status,
    })
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::validate_inputs;
    use crate::interpolation::piecewise_polynomial::PiecewisePolynomial;
    use crate::quadrature::IntegrationError;

    #[test]
    fn validates_derivative_order_and_tolerance_before_native_entry() {
        let polynomial =
            PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0], 1).unwrap();
        assert_eq!(
            validate_inputs(&polynomial, 1, 0.0, 1.0, 1.0e-8),
            Err(IntegrationError::InvalidDerivativeOrder)
        );
        assert_eq!(
            validate_inputs(&polynomial, 0, 0.0, 1.0, 0.0),
            Err(IntegrationError::InvalidTolerance)
        );
    }
}
