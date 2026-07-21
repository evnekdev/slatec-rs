//! Checked tabulated samples and their SLATEC polynomial interpolants.
//!
//! `TabulatedData` owns finite, strictly increasing abscissas and matching
//! finite ordinates. `TabulatedData::interpolating_polynomial` constructs the
//! Newton-form interpolant with `POLINT`/`DPLINT`; the resulting
//! `InterpolatingPolynomial` keeps the native interpolation state private
//! and evaluates it with `POLYVL`/`DPOLVL`.  It can also produce a Taylor
//! expansion through `POLCOF`/`DPOLCF`.
//!
//! The representation is intended for interpolation of a modest number of
//! distinct samples.  A global polynomial can be poorly conditioned for a
//! large or widely spaced data set; use the checked PCHIP, B-spline, or
//! piecewise-polynomial APIs when their local representation is more suitable.
//! Every native call is serialized through the process-wide runtime lock and
//! keeps all native workspaces private.

use alloc::vec;
use alloc::vec::Vec;

use slatec_core::to_fortran_integer;
use slatec_sys::FortranInteger;

use crate::runtime::lock_native;

/// Error from checked tabulated-data construction or polynomial evaluation.
#[derive(Clone, Debug, PartialEq)]
pub enum TabulatedDataError {
    /// Fewer than two paired samples were supplied.
    TooFewSamples,
    /// The abscissa and ordinate slices have different lengths.
    LengthMismatch {
        /// Number of supplied abscissas.
        abscissas: usize,
        /// Number of supplied ordinates.
        values: usize,
    },
    /// An abscissa is NaN or infinite.
    NonFiniteAbscissa {
        /// Zero-based sample index.
        index: usize,
    },
    /// An ordinate is NaN or infinite.
    NonFiniteValue {
        /// Zero-based sample index.
        index: usize,
    },
    /// The latter abscissa of an adjacent pair is not strictly greater.
    AbscissasNotStrictlyIncreasing {
        /// Zero-based index of the latter abscissa.
        index: usize,
    },
    /// An evaluation or Taylor-expansion centre is NaN or infinite.
    NonFiniteQuery,
    /// A count cannot fit the reviewed Fortran `INTEGER` ABI.
    IntegerOverflow,
    /// A private native workspace could not be allocated.
    AllocationFailed,
    /// A reviewed native routine returned an unexpected status.
    NativeContractViolation {
        /// Original SLATEC routine that contradicted its reviewed contract.
        routine: &'static str,
        /// Native status value, where the routine reports one.
        status: FortranInteger,
    },
}

impl core::fmt::Display for TabulatedDataError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooFewSamples => write!(formatter, "at least two tabulated samples are required"),
            Self::LengthMismatch { abscissas, values } => write!(
                formatter,
                "tabulated abscissa count {abscissas} does not match value count {values}"
            ),
            Self::NonFiniteAbscissa { index } => {
                write!(
                    formatter,
                    "tabulated abscissa at index {index} is not finite"
                )
            }
            Self::NonFiniteValue { index } => {
                write!(formatter, "tabulated value at index {index} is not finite")
            }
            Self::AbscissasNotStrictlyIncreasing { index } => write!(
                formatter,
                "tabulated abscissa at index {index} is not strictly increasing"
            ),
            Self::NonFiniteQuery => write!(formatter, "polynomial query is not finite"),
            Self::IntegerOverflow => {
                write!(formatter, "tabulated-data count exceeds Fortran INTEGER")
            }
            Self::AllocationFailed => write!(
                formatter,
                "tabulated-data private workspace allocation failed"
            ),
            Self::NativeContractViolation { routine, status } => write!(
                formatter,
                "{routine} returned unexpected native status {status}"
            ),
        }
    }
}

impl std::error::Error for TabulatedDataError {}

/// Owned, validated samples at strictly increasing real abscissas.
///
/// The data is retained exactly as supplied after validation; it is never
/// sorted, deduplicated, resampled, or extrapolated.  Construct it once and
/// reuse it for tabulated integration or a global polynomial interpolant.
#[derive(Clone, Debug, PartialEq)]
pub struct TabulatedData<T> {
    abscissas: Vec<T>,
    values: Vec<T>,
}

impl<T> TabulatedData<T> {
    /// Returns the retained strictly increasing abscissas.
    #[must_use]
    pub fn abscissas(&self) -> &[T] {
        &self.abscissas
    }

    /// Returns the retained ordinates paired with [`Self::abscissas`].
    #[must_use]
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// Returns the number of paired samples.
    #[must_use]
    pub fn len(&self) -> usize {
        self.abscissas.len()
    }

    /// Always returns `false`; construction requires at least two samples.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

/// A value and its requested first derivatives from an interpolating polynomial.
#[derive(Clone, Debug, PartialEq)]
pub struct PolynomialEvaluation<T> {
    /// Interpolating-polynomial value at the query point.
    pub value: T,
    /// First through requested-order derivatives at the query point.
    ///
    /// The derivative at index zero is the first derivative.  Requests above
    /// the polynomial degree are returned as zero, following `POLYVL`/`DPOLVL`.
    pub derivatives: Vec<T>,
}

/// An owned Newton-form polynomial interpolant of checked tabulated samples.
///
/// Its source samples and the private divided-difference coefficients are kept
/// together because `POLYVL`/`DPOLVL` require both to remain unchanged after
/// `POLINT`/`DPLINT` constructs the representation.
#[derive(Clone, Debug, PartialEq)]
pub struct InterpolatingPolynomial<T> {
    abscissas: Vec<T>,
    coefficients: Vec<T>,
}

impl<T> InterpolatingPolynomial<T> {
    /// Returns the degree of this interpolant.
    #[must_use]
    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    /// Returns the number of original interpolation samples.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.coefficients.len()
    }
}

fn native_integer(value: usize) -> Result<FortranInteger, TabulatedDataError> {
    to_fortran_integer(value).map_err(|_| TabulatedDataError::IntegerOverflow)
}

fn zeroed<T: Default>(length: usize) -> Result<Vec<T>, TabulatedDataError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| TabulatedDataError::AllocationFailed)?;
    values.resize_with(length, T::default);
    Ok(values)
}

fn workspace_len(length: usize) -> Result<usize, TabulatedDataError> {
    length
        .checked_mul(2)
        .ok_or(TabulatedDataError::IntegerOverflow)
}

macro_rules! impl_tabulated_precision {
    ($scalar:ty, $polint:path, $polyvl:path, $polcof:path) => {
        impl TabulatedData<$scalar> {
            /// Creates checked, owned samples for tabulated numerical operations.
            ///
            /// Abscissas and ordinates must have equal length, contain only
            /// finite values, and have at least two strictly increasing
            /// abscissas.  The inputs are retained without sorting or copying.
            pub fn from_samples(
                abscissas: Vec<$scalar>,
                values: Vec<$scalar>,
            ) -> Result<Self, TabulatedDataError> {
                if abscissas.len() != values.len() {
                    return Err(TabulatedDataError::LengthMismatch {
                        abscissas: abscissas.len(),
                        values: values.len(),
                    });
                }
                if abscissas.len() < 2 {
                    return Err(TabulatedDataError::TooFewSamples);
                }
                for (index, value) in abscissas.iter().enumerate() {
                    if !value.is_finite() {
                        return Err(TabulatedDataError::NonFiniteAbscissa { index });
                    }
                    if index > 0 && *value <= abscissas[index - 1] {
                        return Err(TabulatedDataError::AbscissasNotStrictlyIncreasing { index });
                    }
                }
                if let Some((index, _)) = values
                    .iter()
                    .enumerate()
                    .find(|(_, value)| !value.is_finite())
                {
                    return Err(TabulatedDataError::NonFiniteValue { index });
                }
                native_integer(abscissas.len())?;
                Ok(Self { abscissas, values })
            }

            /// Produces this data set's global Newton-form interpolating polynomial.
            ///
            /// This owns the divided-difference representation created by
            /// `POLINT`/`DPLINT`.  It is a global polynomial through every
            /// sample, not a piecewise interpolant or a smoothing fit.
            pub fn interpolating_polynomial(
                &self,
            ) -> Result<InterpolatingPolynomial<$scalar>, TabulatedDataError> {
                let mut coefficients = self.values.clone();
                let mut abscissas = self.abscissas.clone();
                let mut count = native_integer(self.len())?;
                let _native = lock_native();
                // SAFETY: `TabulatedData` validates exact equal-length finite
                // storage with distinct increasing abscissas. `POLINT`/
                // `DPLINT` mutates only the private coefficients buffer.
                unsafe {
                    $polint(
                        &mut count,
                        abscissas.as_mut_ptr(),
                        self.values.as_ptr().cast_mut(),
                        coefficients.as_mut_ptr(),
                    );
                }
                Ok(InterpolatingPolynomial {
                    abscissas,
                    coefficients,
                })
            }
        }

        impl InterpolatingPolynomial<$scalar> {
            /// Evaluates the interpolating polynomial at a finite point.
            ///
            /// This calls `POLYVL`/`DPOLVL` with derivative count zero and
            /// allocates no caller-visible workspace.
            pub fn evaluate(&self, point: $scalar) -> Result<$scalar, TabulatedDataError> {
                Ok(self.evaluate_with_derivatives(point, 0)?.value)
            }

            /// Evaluates the polynomial and its first `derivative_count` derivatives.
            ///
            /// The returned derivative vector is in ascending derivative
            /// order. Requests above the polynomial degree are represented by
            /// trailing zero values exactly as the reviewed native routine
            /// specifies. Private native workspace has `2 * sample_count()`
            /// elements when at least one derivative is requested.
            pub fn evaluate_with_derivatives(
                &self,
                point: $scalar,
                derivative_count: usize,
            ) -> Result<PolynomialEvaluation<$scalar>, TabulatedDataError> {
                if !point.is_finite() {
                    return Err(TabulatedDataError::NonFiniteQuery);
                }
                let mut derivative_count_native = native_integer(derivative_count)?;
                let mut point = point;
                let mut value = <$scalar>::default();
                let mut derivatives = zeroed(derivative_count)?;
                let mut derivative_placeholder = <$scalar>::default();
                let mut workspace = if derivative_count == 0 {
                    vec![<$scalar>::default()]
                } else {
                    zeroed(workspace_len(self.sample_count())?)?
                };
                let mut count = native_integer(self.sample_count())?;
                let mut status = 0;
                let derivative_pointer = if derivative_count == 0 {
                    &mut derivative_placeholder
                } else {
                    derivatives.as_mut_ptr()
                };
                let _native = lock_native();
                // SAFETY: the interpolant owns unchanged matching `X` and `C`
                // arrays from `POLINT`/`DPLINT`; every scalar and private
                // workspace pointer is valid for the source-documented extent.
                unsafe {
                    $polyvl(
                        &mut derivative_count_native,
                        &mut point,
                        &mut value,
                        derivative_pointer,
                        &mut count,
                        self.abscissas.as_ptr().cast_mut(),
                        self.coefficients.as_ptr().cast_mut(),
                        workspace.as_mut_ptr(),
                        &mut status,
                    );
                }
                if status != 1 {
                    return Err(TabulatedDataError::NativeContractViolation {
                        routine: stringify!($polyvl),
                        status,
                    });
                }
                Ok(PolynomialEvaluation { value, derivatives })
            }

            /// Returns Taylor coefficients about a finite expansion centre.
            ///
            /// The returned vector `d` represents
            /// `p(z) = d[0] + d[1] * (z - centre) + ...`; it is produced by
            /// `POLCOF`/`DPOLCF`, not by a Rust-side coefficient conversion.
            pub fn taylor_coefficients_at(
                &self,
                centre: $scalar,
            ) -> Result<Vec<$scalar>, TabulatedDataError> {
                if !centre.is_finite() {
                    return Err(TabulatedDataError::NonFiniteQuery);
                }
                let mut centre = centre;
                let mut count = native_integer(self.sample_count())?;
                let mut coefficients = zeroed(self.sample_count())?;
                let mut workspace = zeroed(workspace_len(self.sample_count())?)?;
                let _native = lock_native();
                // SAFETY: the stored Newton representation is unchanged since
                // `POLINT`/`DPLINT`; output and work arrays have the exact
                // source-documented lengths `N` and `2*N`.
                unsafe {
                    $polcof(
                        &mut centre,
                        &mut count,
                        self.abscissas.as_ptr().cast_mut(),
                        self.coefficients.as_ptr().cast_mut(),
                        coefficients.as_mut_ptr(),
                        workspace.as_mut_ptr(),
                    );
                }
                Ok(coefficients)
            }
        }
    };
}

impl_tabulated_precision!(
    f32,
    slatec_sys::interpolation::polint,
    slatec_sys::interpolation::polyvl,
    slatec_sys::interpolation::polcof
);
impl_tabulated_precision!(
    f64,
    slatec_sys::interpolation::dplint,
    slatec_sys::interpolation::dpolvl,
    slatec_sys::interpolation::dpolcf
);

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::{TabulatedData, TabulatedDataError};

    #[test]
    fn rejects_mismatched_non_finite_and_unordered_samples() {
        assert!(matches!(
            TabulatedData::<f64>::from_samples(vec![0.0], vec![1.0]),
            Err(TabulatedDataError::TooFewSamples)
        ));
        assert!(matches!(
            TabulatedData::<f64>::from_samples(vec![0.0, 1.0], vec![1.0]),
            Err(TabulatedDataError::LengthMismatch { .. })
        ));
        assert!(matches!(
            TabulatedData::<f64>::from_samples(vec![0.0, f64::NAN], vec![0.0, 1.0]),
            Err(TabulatedDataError::NonFiniteAbscissa { index: 1 })
        ));
        assert!(matches!(
            TabulatedData::<f64>::from_samples(vec![0.0, 0.0], vec![0.0, 1.0]),
            Err(TabulatedDataError::AbscissasNotStrictlyIncreasing { index: 1 })
        ));
    }
}
