//! Owned single-precision polynomial roots.
//!
//! The reviewed SLATEC drivers accept coefficients in descending powers and
//! return complex roots. [`PolynomialRootMethod::Iterative`] uses
//! `CPZERO`/`RPZERO` with automatic native initial estimates; it preserves the
//! documented best current roots when its iteration limit is reached.
//! [`PolynomialRootMethod::CompanionQr`] uses `CPQR79`/`RPQR79`; its source
//! does not document partial roots on nonconvergence, so that condition is an
//! error instead.

use alloc::vec::Vec;
use core::convert::TryFrom;
use core::fmt;
use core::mem::{align_of, size_of};

use num_complex::Complex32;
use slatec_sys::{Complex32 as NativeComplex32, FortranInteger};

use crate::runtime::lock_native;

const _: () = assert!(size_of::<Complex32>() == size_of::<NativeComplex32>());
const _: () = assert!(align_of::<Complex32>() == align_of::<NativeComplex32>());

/// The reviewed SLATEC polynomial-root driver to invoke.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialRootMethod {
    /// `CPZERO` or `RPZERO`: automatic simultaneous iteration with an error
    /// bound for each root when it converges.
    Iterative,
    /// `CPQR79` or `RPQR79`: companion-matrix QR iteration.
    CompanionQr,
}

/// Completion state for an owned polynomial-root result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolynomialRootStatus {
    /// The selected native algorithm completed normally.
    Converged,
    /// `CPZERO`/`RPZERO` reached its documented `25 * degree` iteration
    /// limit. The contained roots are the native driver's best estimates;
    /// error bounds are not available for this status.
    IterationLimitReached,
}

/// An owned result from a reviewed single-precision polynomial-root driver.
///
/// Roots remain in the native driver's order; they are neither sorted nor
/// paired by this safe facade. Coefficients and every native workspace are
/// copied or allocated privately, so no caller storage is mutated or retained.
#[derive(Clone, Debug, PartialEq)]
pub struct PolynomialRoots {
    roots: Vec<Complex32>,
    error_bounds: Option<Vec<f32>>,
    method: PolynomialRootMethod,
    status: PolynomialRootStatus,
}

impl PolynomialRoots {
    /// Returns the roots in the original driver's output order.
    #[must_use]
    pub fn roots(&self) -> &[Complex32] {
        &self.roots
    }

    /// Returns the native per-root error bounds from `CPZERO`/`RPZERO`.
    ///
    /// The companion-QR drivers do not provide bounds. Iterative results that
    /// reached their documented iteration limit preserve roots but provide no
    /// bounds, exactly as the source prologue specifies.
    #[must_use]
    pub fn error_bounds(&self) -> Option<&[f32]> {
        self.error_bounds.as_deref()
    }

    /// Returns the selected original SLATEC algorithm.
    #[must_use]
    pub const fn method(&self) -> PolynomialRootMethod {
        self.method
    }

    /// Returns the native completion state.
    #[must_use]
    pub const fn status(&self) -> PolynomialRootStatus {
        self.status
    }

    /// Returns the polynomial degree, equal to the number of returned roots.
    #[must_use]
    pub fn degree(&self) -> usize {
        self.roots.len()
    }
}

/// Failure while validating or invoking a polynomial-root driver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PolynomialRootError {
    /// Fewer than two coefficients were supplied, so no positive-degree
    /// polynomial was defined.
    TooFewCoefficients,
    /// A coefficient has a NaN or infinite real or imaginary component.
    NonFiniteCoefficient {
        /// Zero-based coefficient index in descending-power order.
        index: usize,
    },
    /// The leading coefficient is exactly zero, which the reviewed drivers
    /// reject rather than silently reducing the degree.
    ZeroLeadingCoefficient,
    /// Degree or an exact native workspace formula exceeded Rust or SLATEC
    /// representable bounds.
    DimensionOverflow,
    /// Allocation of owned coefficient, output, or workspace storage failed.
    AllocationFailed,
    /// `CPQR79`/`RPQR79` exhausted its 30 QR iterations on at least one
    /// companion-matrix eigenvalue. Its source does not document partial roots.
    CompanionQrNoConvergence,
    /// A preflighted input-status error was nevertheless returned by native
    /// code, or native output violated its reviewed contract.
    NativeContractViolation {
        /// Short stable explanation of the contradicted contract.
        detail: &'static str,
    },
}

impl fmt::Display for PolynomialRootError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooFewCoefficients => {
                write!(formatter, "a polynomial needs at least two coefficients")
            }
            Self::NonFiniteCoefficient { index } => {
                write!(formatter, "polynomial coefficient {index} is not finite")
            }
            Self::ZeroLeadingCoefficient => {
                write!(
                    formatter,
                    "the leading polynomial coefficient must be nonzero"
                )
            }
            Self::DimensionOverflow => {
                write!(
                    formatter,
                    "polynomial degree or native workspace is too large"
                )
            }
            Self::AllocationFailed => {
                write!(formatter, "could not allocate polynomial-root storage")
            }
            Self::CompanionQrNoConvergence => {
                write!(
                    formatter,
                    "the companion-matrix QR iteration did not converge"
                )
            }
            Self::NativeContractViolation { detail } => {
                write!(
                    formatter,
                    "native polynomial-root contract was violated: {detail}"
                )
            }
        }
    }
}

impl std::error::Error for PolynomialRootError {}

/// Finds complex roots of a real-coefficient polynomial using `RPZERO`.
///
/// `coefficients` are finite descending-power coefficients, beginning with
/// the nonzero leading coefficient. `RPZERO` creates its own initial root
/// estimates; this safe API deliberately does not expose the source's
/// distinct-and-separated estimate protocol. On its documented iteration
/// limit, the returned [`PolynomialRoots`] retains the best current roots and
/// reports [`PolynomialRootStatus::IterationLimitReached`].
pub fn real_polynomial_roots(coefficients: &[f32]) -> Result<PolynomialRoots, PolynomialRootError> {
    real_polynomial_roots_with_method(coefficients, PolynomialRootMethod::Iterative)
}

/// Finds complex roots of a real-coefficient polynomial with an explicit
/// reviewed SLATEC algorithm.
pub fn real_polynomial_roots_with_method(
    coefficients: &[f32],
    method: PolynomialRootMethod,
) -> Result<PolynomialRoots, PolynomialRootError> {
    let degree = validate_real_coefficients(coefficients)?;
    match method {
        PolynomialRootMethod::Iterative => real_iterative(coefficients, degree),
        PolynomialRootMethod::CompanionQr => real_companion_qr(coefficients, degree),
    }
}

/// Finds roots of a complex-coefficient polynomial using `CPZERO`.
///
/// `coefficients` are finite descending-power coefficients, beginning with
/// the nonzero leading coefficient. The result owns its roots and, when the
/// source documents them, native error bounds.
pub fn complex_polynomial_roots(
    coefficients: &[Complex32],
) -> Result<PolynomialRoots, PolynomialRootError> {
    complex_polynomial_roots_with_method(coefficients, PolynomialRootMethod::Iterative)
}

/// Finds roots of a complex-coefficient polynomial with an explicit reviewed
/// SLATEC algorithm.
pub fn complex_polynomial_roots_with_method(
    coefficients: &[Complex32],
    method: PolynomialRootMethod,
) -> Result<PolynomialRoots, PolynomialRootError> {
    let degree = validate_complex_coefficients(coefficients)?;
    match method {
        PolynomialRootMethod::Iterative => complex_iterative(coefficients, degree),
        PolynomialRootMethod::CompanionQr => complex_companion_qr(coefficients, degree),
    }
}

fn real_iterative(
    coefficients: &[f32],
    degree: usize,
) -> Result<PolynomialRoots, PolynomialRootError> {
    let mut coefficients = copied(coefficients)?;
    let mut roots = zeroed(degree)?;
    let mut workspace = zeroed(iterative_real_workspace_len(degree)?)?;
    let mut bounds = zeroed(degree)?;
    let mut degree_native = native_len(degree)?;
    // Passing zero asks RPZERO to construct the source-documented automatic
    // initial estimates. The same scalar returns its diagnostic on exit.
    let mut flag = 0;
    {
        let _native = lock_native();
        // SAFETY: preflight proves a positive f32 degree, finite descending
        // coefficients with a nonzero leader, roots[N], workspace[6*(N+1)],
        // and bounds[N]. All storage is private and stays live for RPZERO.
        unsafe {
            slatec_sys::roots::complex::rpzero(
                &mut degree_native,
                coefficients.as_mut_ptr(),
                native_complex_mut(&mut roots),
                native_complex_mut(&mut workspace),
                &mut flag,
                bounds.as_mut_ptr(),
            );
        }
    }
    iterative_result(roots, bounds, flag, PolynomialRootMethod::Iterative)
}

fn complex_iterative(
    coefficients: &[Complex32],
    degree: usize,
) -> Result<PolynomialRoots, PolynomialRootError> {
    let mut coefficients = copied(coefficients)?;
    let mut roots = zeroed(degree)?;
    let mut workspace = zeroed(iterative_complex_workspace_len(degree)?)?;
    let mut bounds = zeroed(degree)?;
    let mut degree_native = native_len(degree)?;
    let mut flag = 0;
    {
        let _native = lock_native();
        // SAFETY: preflight proves CPZERO's documented positive degree,
        // finite nonzero-leading coefficients, roots[N], temporary
        // workspace[4*(N+1)], and bounds[N] requirements. The num-complex
        // layout checks above prove the reviewed GNU Fortran COMPLEX layout.
        unsafe {
            slatec_sys::roots::complex::cpzero(
                &mut degree_native,
                native_complex_mut(&mut coefficients),
                native_complex_mut(&mut roots),
                native_complex_mut(&mut workspace),
                &mut flag,
                bounds.as_mut_ptr(),
            );
        }
    }
    iterative_result(roots, bounds, flag, PolynomialRootMethod::Iterative)
}

fn real_companion_qr(
    coefficients: &[f32],
    degree: usize,
) -> Result<PolynomialRoots, PolynomialRootError> {
    let mut coefficients = copied(coefficients)?;
    let mut roots = zeroed(degree)?;
    let mut workspace = zeroed(companion_real_workspace_len(degree)?)?;
    let mut degree_native = native_len(degree)?;
    let mut error = 0;
    {
        let _native = lock_native();
        // SAFETY: preflight establishes RPQR79's coefficient, root, and
        // N*(N+2) real-workspace requirements. Input coefficients are copied
        // because the raw declaration is mutable despite source input intent.
        unsafe {
            slatec_sys::roots::complex::rpqr79(
                &mut degree_native,
                coefficients.as_mut_ptr(),
                native_complex_mut(&mut roots),
                &mut error,
                workspace.as_mut_ptr(),
            );
        }
    }
    companion_result(roots, error)
}

fn complex_companion_qr(
    coefficients: &[Complex32],
    degree: usize,
) -> Result<PolynomialRoots, PolynomialRootError> {
    let mut coefficients = copied(coefficients)?;
    let mut roots = zeroed(degree)?;
    let mut workspace = zeroed(companion_complex_workspace_len(degree)?)?;
    let mut degree_native = native_len(degree)?;
    let mut error = 0;
    {
        let _native = lock_native();
        // SAFETY: preflight establishes CPQR79's coefficient, root, and
        // 2*N*(N+1) real-workspace requirements. The layout checks above
        // prove the num-complex representation used for GNU Fortran COMPLEX.
        unsafe {
            slatec_sys::roots::complex::cpqr79(
                &mut degree_native,
                native_complex_mut(&mut coefficients),
                native_complex_mut(&mut roots),
                &mut error,
                workspace.as_mut_ptr(),
            );
        }
    }
    companion_result(roots, error)
}

fn iterative_result(
    roots: Vec<Complex32>,
    bounds: Vec<f32>,
    flag: FortranInteger,
    method: PolynomialRootMethod,
) -> Result<PolynomialRoots, PolynomialRootError> {
    match flag {
        0 => {
            validate_roots(&roots)?;
            if bounds
                .iter()
                .any(|bound| !bound.is_finite() || *bound < 0.0)
            {
                return Err(PolynomialRootError::NativeContractViolation {
                    detail: "CPZERO/RPZERO returned an invalid error bound",
                });
            }
            Ok(PolynomialRoots {
                roots,
                error_bounds: Some(bounds),
                method,
                status: PolynomialRootStatus::Converged,
            })
        }
        1 => Err(PolynomialRootError::NativeContractViolation {
            detail: "CPZERO/RPZERO rejected prevalidated polynomial input",
        }),
        2 => {
            validate_roots(&roots)?;
            Ok(PolynomialRoots {
                roots,
                error_bounds: None,
                method,
                status: PolynomialRootStatus::IterationLimitReached,
            })
        }
        _ => Err(PolynomialRootError::NativeContractViolation {
            detail: "CPZERO/RPZERO returned an undocumented diagnostic",
        }),
    }
}

fn companion_result(
    roots: Vec<Complex32>,
    error: FortranInteger,
) -> Result<PolynomialRoots, PolynomialRootError> {
    match error {
        0 => {
            validate_roots(&roots)?;
            Ok(PolynomialRoots {
                roots,
                error_bounds: None,
                method: PolynomialRootMethod::CompanionQr,
                status: PolynomialRootStatus::Converged,
            })
        }
        1 => Err(PolynomialRootError::CompanionQrNoConvergence),
        2 | 3 => Err(PolynomialRootError::NativeContractViolation {
            detail: "CPQR79/RPQR79 rejected prevalidated polynomial input",
        }),
        _ => Err(PolynomialRootError::NativeContractViolation {
            detail: "CPQR79/RPQR79 returned an undocumented diagnostic",
        }),
    }
}

fn validate_real_coefficients(coefficients: &[f32]) -> Result<usize, PolynomialRootError> {
    if coefficients.len() < 2 {
        return Err(PolynomialRootError::TooFewCoefficients);
    }
    for (index, coefficient) in coefficients.iter().enumerate() {
        if !coefficient.is_finite() {
            return Err(PolynomialRootError::NonFiniteCoefficient { index });
        }
    }
    if coefficients[0] == 0.0 {
        return Err(PolynomialRootError::ZeroLeadingCoefficient);
    }
    let degree = coefficients.len() - 1;
    native_len(degree)?;
    Ok(degree)
}

fn validate_complex_coefficients(coefficients: &[Complex32]) -> Result<usize, PolynomialRootError> {
    if coefficients.len() < 2 {
        return Err(PolynomialRootError::TooFewCoefficients);
    }
    for (index, coefficient) in coefficients.iter().enumerate() {
        if !coefficient.re.is_finite() || !coefficient.im.is_finite() {
            return Err(PolynomialRootError::NonFiniteCoefficient { index });
        }
    }
    if coefficients[0].re == 0.0 && coefficients[0].im == 0.0 {
        return Err(PolynomialRootError::ZeroLeadingCoefficient);
    }
    let degree = coefficients.len() - 1;
    native_len(degree)?;
    Ok(degree)
}

fn validate_roots(roots: &[Complex32]) -> Result<(), PolynomialRootError> {
    if roots
        .iter()
        .any(|root| !root.re.is_finite() || !root.im.is_finite())
    {
        return Err(PolynomialRootError::NativeContractViolation {
            detail: "native polynomial-root output was not finite",
        });
    }
    Ok(())
}

fn iterative_real_workspace_len(degree: usize) -> Result<usize, PolynomialRootError> {
    degree
        .checked_add(1)
        .and_then(|count| count.checked_mul(6))
        .ok_or(PolynomialRootError::DimensionOverflow)
}

fn iterative_complex_workspace_len(degree: usize) -> Result<usize, PolynomialRootError> {
    degree
        .checked_add(1)
        .and_then(|count| count.checked_mul(4))
        .ok_or(PolynomialRootError::DimensionOverflow)
}

fn companion_real_workspace_len(degree: usize) -> Result<usize, PolynomialRootError> {
    degree
        .checked_add(2)
        .and_then(|offset| degree.checked_mul(offset))
        .ok_or(PolynomialRootError::DimensionOverflow)
}

fn companion_complex_workspace_len(degree: usize) -> Result<usize, PolynomialRootError> {
    degree
        .checked_add(1)
        .and_then(|offset| degree.checked_mul(offset))
        .and_then(|words| words.checked_mul(2))
        .ok_or(PolynomialRootError::DimensionOverflow)
}

fn native_len(length: usize) -> Result<FortranInteger, PolynomialRootError> {
    FortranInteger::try_from(length).map_err(|_| PolynomialRootError::DimensionOverflow)
}

fn zeroed<T: Copy + Default>(length: usize) -> Result<Vec<T>, PolynomialRootError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| PolynomialRootError::AllocationFailed)?;
    values.resize(length, T::default());
    Ok(values)
}

fn copied<T: Copy>(values: &[T]) -> Result<Vec<T>, PolynomialRootError> {
    let mut copied = Vec::new();
    copied
        .try_reserve_exact(values.len())
        .map_err(|_| PolynomialRootError::AllocationFailed)?;
    copied.extend_from_slice(values);
    Ok(copied)
}

fn native_complex_mut(values: &mut [Complex32]) -> *mut NativeComplex32 {
    values.as_mut_ptr().cast()
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    #[test]
    fn validates_real_and_complex_coefficients_before_native_entry() {
        assert_eq!(
            real_polynomial_roots(&[1.0]),
            Err(PolynomialRootError::TooFewCoefficients)
        );
        assert_eq!(
            real_polynomial_roots(&[0.0, 1.0]),
            Err(PolynomialRootError::ZeroLeadingCoefficient)
        );
        assert_eq!(
            complex_polynomial_roots(&[Complex32::new(f32::NAN, 0.0), Complex32::new(1.0, 0.0)]),
            Err(PolynomialRootError::NonFiniteCoefficient { index: 0 })
        );
    }

    #[test]
    fn exact_workspace_formulas_are_checked() {
        assert_eq!(iterative_real_workspace_len(2), Ok(18));
        assert_eq!(iterative_complex_workspace_len(2), Ok(12));
        assert_eq!(companion_real_workspace_len(2), Ok(8));
        assert_eq!(companion_complex_workspace_len(2), Ok(12));
    }

    #[test]
    fn documented_nonconverged_statuses_preserve_only_supported_results() {
        let roots = vec![Complex32::new(1.0, 0.0), Complex32::new(-2.0, 0.0)];
        let iterative = iterative_result(
            roots.clone(),
            vec![0.0, 0.0],
            2,
            PolynomialRootMethod::Iterative,
        )
        .expect("the iterative driver documents its best current roots");
        assert_eq!(
            iterative.status(),
            PolynomialRootStatus::IterationLimitReached
        );
        assert_eq!(iterative.roots(), roots);
        assert_eq!(iterative.error_bounds(), None);

        assert_eq!(
            companion_result(roots, 1),
            Err(PolynomialRootError::CompanionQrNoConvergence)
        );
    }
}
