//! Safe real univariate piecewise polynomials backed by SLATEC PP form.
//!
//! A [`PiecewisePolynomial`](crate::interpolation::piecewise_polynomial::PiecewisePolynomial)
//! owns strictly increasing breakpoints `XI` and a
//! column-major coefficient matrix `C`. For piece `j`, whose local coordinate
//! is `dx = x - XI[j]`, its `order()` coefficients are the right Taylor
//! derivatives `C(1,j), .., C(K,j)`, so its value is
//! `sum_i C(i,j) * dx^(i-1) / (i-1)!`. Thus `order()` is the number of
//! coefficients per piece and `degree()` is `order() - 1`; coefficients are
//! neither ordinary descending powers nor pre-divided factorial coefficients.
//!
//! `PPVAL`/`DPPVAL` select the right piece at interior breakpoints and the
//! final piece at the right endpoint. This API rejects extrapolation by
//! default;
//! [`Extrapolation::Clamp`](crate::interpolation::piecewise_polynomial::Extrapolation::Clamp)
//! is a Rust-side endpoint clamp and never
//! asks SLATEC to extrapolate. `PPQAD`/`DPPQAD` perform exact integration of
//! the PP representation. When the `bspline` feature is also enabled,
//! `BSpline` converts exactly through `BSPPP`/`DBSPPP`.
//!
//! All native calls are serialized through the process-wide runtime lock and
//! use scoped XERROR restoration. The initial surface is real, univariate,
//! allocation-owning, and intentionally does not provide PP-to-B-spline,
//! PCHIP, multidimensional, fitting, or generic spline abstractions.

#![cfg(feature = "piecewise-polynomial")]

use alloc::vec::Vec;
use core::convert::TryFrom;
use core::ops::RangeInclusive;

use slatec_sys::FortranInteger;

use crate::runtime::{lock_native, permit_recoverable_native_statuses};

/// An error from PP-form validation, allocation, or a contradicted native contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PiecewisePolynomialError {
    /// Fewer than two breakpoints were supplied.
    TooFewBreakpoints,
    /// The polynomial order was zero or cannot fit SLATEC's integer ABI.
    InvalidOrder,
    /// A breakpoint, coefficient, or query was NaN or infinite.
    NonFiniteInput,
    /// The latter breakpoint of an adjacent pair was not strictly greater.
    BreakpointsNotStrictlyIncreasing {
        /// Zero-based index of the latter invalid breakpoint.
        index: usize,
    },
    /// The flat coefficient storage does not equal `order * piece_count`.
    CoefficientCountMismatch {
        /// Required coefficient count.
        expected: usize,
        /// Supplied coefficient count.
        actual: usize,
    },
    /// A query or integration bound was outside the closed PP domain.
    OutOfDomain,
    /// The requested derivative order is not below the PP order.
    DerivativeOrderTooHigh {
        /// Requested derivative order.
        requested: usize,
        /// Largest supported derivative order.
        maximum: usize,
    },
    /// A caller-provided output slice has the wrong length.
    OutputLengthMismatch {
        /// Number of query points.
        expected: usize,
        /// Output slice length.
        actual: usize,
    },
    /// A dimension or native workspace formula overflowed Rust or Fortran integers.
    DimensionOverflow,
    /// Allocation of private conversion storage failed.
    AllocationFailed,
    /// Native output contradicted the reviewed PP-form contract.
    NativeContractViolation {
        /// Concise explanation of the violated postcondition.
        detail: &'static str,
    },
}

/// Controls handling of an otherwise finite query outside the PP domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extrapolation {
    /// Reject out-of-domain inputs before native entry.
    Error,
    /// Clamp the query coordinate to the nearest domain endpoint in Rust.
    ///
    /// This does not select SLATEC's native extrapolation behavior and does
    /// not clamp the returned value.
    Clamp,
}

/// An owned scalar univariate SLATEC piecewise polynomial.
///
/// `breakpoints` has `piece_count() + 1` strictly increasing finite values.
/// `coefficients_for_piece(j)` exposes `order()` right Taylor derivatives for
/// the interval `breakpoints[j]..breakpoints[j + 1]`, in ascending derivative
/// order. The representation is passed to SLATEC unchanged: no breakpoints
/// are sorted or merged and no coefficients are reshaped.
#[derive(Clone, Debug, PartialEq)]
pub struct PiecewisePolynomial<T> {
    breakpoints: Vec<T>,
    coefficients: Vec<T>,
    order: usize,
}

impl<T> PiecewisePolynomial<T> {
    /// Returns the PP order, the number of coefficients per polynomial piece.
    #[must_use]
    pub const fn order(&self) -> usize {
        self.order
    }

    /// Returns the polynomial degree, `order() - 1`.
    #[must_use]
    pub const fn degree(&self) -> usize {
        self.order - 1
    }

    /// Returns the number of polynomial pieces.
    #[must_use]
    pub fn piece_count(&self) -> usize {
        self.breakpoints.len() - 1
    }

    /// Returns the strictly increasing PP breakpoint vector.
    #[must_use]
    pub fn breakpoints(&self) -> &[T] {
        &self.breakpoints
    }

    /// Returns one piece's coefficients in ascending right-Taylor order.
    ///
    /// For local `dx = x - breakpoints()[index]`, the returned slice contains
    /// `f(left), f'(left), ..., f^(degree)(left)`, and the represented value
    /// divides the coefficient at index `i` by `i!` when forming its power.
    #[must_use]
    pub fn coefficients_for_piece(&self, index: usize) -> Option<&[T]> {
        let start = index.checked_mul(self.order)?;
        let end = start.checked_add(self.order)?;
        self.coefficients.get(start..end)
    }

    /// Returns the closed PP evaluation domain.
    #[must_use]
    pub fn domain(&self) -> RangeInclusive<T>
    where
        T: Copy,
    {
        self.breakpoints[0]..=self.breakpoints[self.piece_count()]
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

trait PpScalar: sealed::Sealed + Copy + Default + PartialOrd {
    fn finite(self) -> bool;

    unsafe fn evaluate_native(
        leading_dimension: *const FortranInteger,
        coefficients: *const Self,
        breakpoints: *const Self,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        derivative_order: *const FortranInteger,
        point: *const Self,
        interval_state: *mut FortranInteger,
    ) -> Self;

    unsafe fn integrate_native(
        leading_dimension: *const FortranInteger,
        coefficients: *const Self,
        breakpoints: *const Self,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        lower: *const Self,
        upper: *const Self,
        integral: *mut Self,
    );

    #[cfg(feature = "bspline")]
    unsafe fn convert_bspline_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        leading_dimension: *const FortranInteger,
        pp_coefficients: *mut Self,
        breakpoints: *mut Self,
        piece_count: *mut FortranInteger,
        workspace: *mut Self,
    );
}

impl PpScalar for f32 {
    fn finite(self) -> bool {
        self.is_finite()
    }

    unsafe fn evaluate_native(
        leading_dimension: *const FortranInteger,
        coefficients: *const Self,
        breakpoints: *const Self,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        derivative_order: *const FortranInteger,
        point: *const Self,
        interval_state: *mut FortranInteger,
    ) -> Self {
        unsafe {
            slatec_sys::piecewise_polynomial::ppval(
                leading_dimension,
                coefficients,
                breakpoints,
                piece_count,
                order,
                derivative_order,
                point,
                interval_state,
            )
        }
    }

    unsafe fn integrate_native(
        leading_dimension: *const FortranInteger,
        coefficients: *const Self,
        breakpoints: *const Self,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        lower: *const Self,
        upper: *const Self,
        integral: *mut Self,
    ) {
        unsafe {
            slatec_sys::piecewise_polynomial::ppqad(
                leading_dimension,
                coefficients,
                breakpoints,
                piece_count,
                order,
                lower,
                upper,
                integral,
            )
        }
    }

    #[cfg(feature = "bspline")]
    unsafe fn convert_bspline_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        leading_dimension: *const FortranInteger,
        pp_coefficients: *mut Self,
        breakpoints: *mut Self,
        piece_count: *mut FortranInteger,
        workspace: *mut Self,
    ) {
        unsafe {
            slatec_sys::piecewise_polynomial::bsppp(
                knots,
                coefficients,
                coefficient_count,
                order,
                leading_dimension,
                pp_coefficients,
                breakpoints,
                piece_count,
                workspace,
            )
        }
    }
}

impl PpScalar for f64 {
    fn finite(self) -> bool {
        self.is_finite()
    }

    unsafe fn evaluate_native(
        leading_dimension: *const FortranInteger,
        coefficients: *const Self,
        breakpoints: *const Self,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        derivative_order: *const FortranInteger,
        point: *const Self,
        interval_state: *mut FortranInteger,
    ) -> Self {
        unsafe {
            slatec_sys::piecewise_polynomial::dppval(
                leading_dimension,
                coefficients,
                breakpoints,
                piece_count,
                order,
                derivative_order,
                point,
                interval_state,
            )
        }
    }

    unsafe fn integrate_native(
        leading_dimension: *const FortranInteger,
        coefficients: *const Self,
        breakpoints: *const Self,
        piece_count: *const FortranInteger,
        order: *const FortranInteger,
        lower: *const Self,
        upper: *const Self,
        integral: *mut Self,
    ) {
        unsafe {
            slatec_sys::piecewise_polynomial::dppqad(
                leading_dimension,
                coefficients,
                breakpoints,
                piece_count,
                order,
                lower,
                upper,
                integral,
            )
        }
    }

    #[cfg(feature = "bspline")]
    unsafe fn convert_bspline_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: *const FortranInteger,
        order: *const FortranInteger,
        leading_dimension: *const FortranInteger,
        pp_coefficients: *mut Self,
        breakpoints: *mut Self,
        piece_count: *mut FortranInteger,
        workspace: *mut Self,
    ) {
        unsafe {
            slatec_sys::piecewise_polynomial::dbsppp(
                knots,
                coefficients,
                coefficient_count,
                order,
                leading_dimension,
                pp_coefficients,
                breakpoints,
                piece_count,
                workspace,
            )
        }
    }
}

#[allow(private_bounds)]
impl<T: PpScalar> PiecewisePolynomial<T> {
    fn from_parts_impl(
        breakpoints: Vec<T>,
        coefficients: Vec<T>,
        order: usize,
    ) -> Result<Self, PiecewisePolynomialError> {
        validate_parts(&breakpoints, &coefficients, order)?;
        Ok(Self {
            breakpoints,
            coefficients,
            order,
        })
    }

    fn evaluate_impl(&self, point: T) -> Result<T, PiecewisePolynomialError> {
        self.evaluate_with_extrapolation_impl(point, Extrapolation::Error)
    }

    fn evaluate_with_extrapolation_impl(
        &self,
        point: T,
        extrapolation: Extrapolation,
    ) -> Result<T, PiecewisePolynomialError> {
        self.derivative_with_extrapolation_impl(point, 0, extrapolation)
    }

    fn derivative_impl(
        &self,
        point: T,
        derivative_order: usize,
    ) -> Result<T, PiecewisePolynomialError> {
        self.derivative_with_extrapolation_impl(point, derivative_order, Extrapolation::Error)
    }

    fn derivative_with_extrapolation_impl(
        &self,
        point: T,
        derivative_order: usize,
        extrapolation: Extrapolation,
    ) -> Result<T, PiecewisePolynomialError> {
        if derivative_order >= self.order {
            return Err(PiecewisePolynomialError::DerivativeOrderTooHigh {
                requested: derivative_order,
                maximum: self.order - 1,
            });
        }
        let point = self.validate_domain(point, extrapolation)?;
        let leading_dimension = native_len(self.order)?;
        let piece_count = native_len(self.piece_count())?;
        let order = native_len(self.order)?;
        let derivative_order = native_len(derivative_order)?;
        // PPVAL/DPPVAL require INPPV=1 before the first query. Retaining no
        // cache makes immutable methods independent and avoids search state.
        let mut interval_state = 1;
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        let value = unsafe {
            T::evaluate_native(
                &leading_dimension,
                self.coefficients.as_ptr(),
                self.breakpoints.as_ptr(),
                &piece_count,
                &order,
                &derivative_order,
                &point,
                &mut interval_state,
            )
        };
        if interval_state < 1
            || usize::try_from(interval_state)
                .ok()
                .is_some_and(|index| index > self.piece_count())
        {
            return Err(PiecewisePolynomialError::NativeContractViolation {
                detail: "PPVAL returned an invalid interval-search state",
            });
        }
        Ok(value)
    }

    fn evaluate_into_impl(
        &self,
        points: &[T],
        output: &mut [T],
        extrapolation: Extrapolation,
    ) -> Result<(), PiecewisePolynomialError> {
        if points.len() != output.len() {
            return Err(PiecewisePolynomialError::OutputLengthMismatch {
                expected: points.len(),
                actual: output.len(),
            });
        }
        native_len(points.len())?;
        for (&point, value) in points.iter().zip(output.iter_mut()) {
            *value = self.evaluate_with_extrapolation_impl(point, extrapolation)?;
        }
        Ok(())
    }

    fn integrate_impl(
        &self,
        lower: T,
        upper: T,
        extrapolation: Extrapolation,
    ) -> Result<T, PiecewisePolynomialError> {
        let lower = self.validate_domain(lower, extrapolation)?;
        let upper = self.validate_domain(upper, extrapolation)?;
        let leading_dimension = native_len(self.order)?;
        let piece_count = native_len(self.piece_count())?;
        let order = native_len(self.order)?;
        let mut integral = T::default();
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        unsafe {
            T::integrate_native(
                &leading_dimension,
                self.coefficients.as_ptr(),
                self.breakpoints.as_ptr(),
                &piece_count,
                &order,
                &lower,
                &upper,
                &mut integral,
            )
        };
        Ok(integral)
    }

    fn validate_domain(
        &self,
        point: T,
        extrapolation: Extrapolation,
    ) -> Result<T, PiecewisePolynomialError> {
        if !point.finite() {
            return Err(PiecewisePolynomialError::NonFiniteInput);
        }
        let first = self.breakpoints[0];
        let last = self.breakpoints[self.piece_count()];
        if point < first {
            return match extrapolation {
                Extrapolation::Error => Err(PiecewisePolynomialError::OutOfDomain),
                Extrapolation::Clamp => Ok(first),
            };
        }
        if point > last {
            return match extrapolation {
                Extrapolation::Error => Err(PiecewisePolynomialError::OutOfDomain),
                Extrapolation::Clamp => Ok(last),
            };
        }
        Ok(point)
    }

    #[cfg(feature = "bspline")]
    fn from_bspline_parts(
        knots: &[T],
        coefficients: &[T],
        order: usize,
    ) -> Result<Self, PiecewisePolynomialError> {
        let coefficient_count = coefficients.len();
        let maximum_pieces = coefficient_count
            .checked_sub(order)
            .and_then(|value| value.checked_add(1))
            .ok_or(PiecewisePolynomialError::DimensionOverflow)?;
        let breakpoint_capacity = maximum_pieces
            .checked_add(1)
            .ok_or(PiecewisePolynomialError::DimensionOverflow)?;
        let coefficient_capacity = order
            .checked_mul(maximum_pieces)
            .ok_or(PiecewisePolynomialError::DimensionOverflow)?;
        let workspace_capacity = order
            .checked_mul(
                coefficient_count
                    .checked_add(3)
                    .ok_or(PiecewisePolynomialError::DimensionOverflow)?,
            )
            .ok_or(PiecewisePolynomialError::DimensionOverflow)?;
        let mut pp_coefficients = zeroed::<T>(coefficient_capacity)?;
        let mut breakpoints = zeroed::<T>(breakpoint_capacity)?;
        let mut workspace = zeroed::<T>(workspace_capacity)?;
        let coefficient_count_native = native_len(coefficient_count)?;
        let order_native = native_len(order)?;
        let leading_dimension = order_native;
        let mut piece_count = 0;
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        unsafe {
            T::convert_bspline_native(
                knots.as_ptr(),
                coefficients.as_ptr(),
                &coefficient_count_native,
                &order_native,
                &leading_dimension,
                pp_coefficients.as_mut_ptr(),
                breakpoints.as_mut_ptr(),
                &mut piece_count,
                workspace.as_mut_ptr(),
            )
        };
        let piece_count = usize::try_from(piece_count).map_err(|_| {
            PiecewisePolynomialError::NativeContractViolation {
                detail: "BSPPP returned a negative piece count",
            }
        })?;
        if piece_count == 0 || piece_count > maximum_pieces {
            return Err(PiecewisePolynomialError::NativeContractViolation {
                detail: "BSPPP returned an out-of-range piece count",
            });
        }
        let used_coefficients = order.checked_mul(piece_count).ok_or(
            PiecewisePolynomialError::NativeContractViolation {
                detail: "BSPPP result dimensions overflowed",
            },
        )?;
        let used_breakpoints = piece_count.checked_add(1).ok_or(
            PiecewisePolynomialError::NativeContractViolation {
                detail: "BSPPP breakpoint count overflowed",
            },
        )?;
        pp_coefficients.truncate(used_coefficients);
        breakpoints.truncate(used_breakpoints);
        Self::from_parts_impl(breakpoints, pp_coefficients, order).map_err(|_| {
            PiecewisePolynomialError::NativeContractViolation {
                detail: "BSPPP returned an invalid PP representation",
            }
        })
    }
}

macro_rules! impl_public_precision {
    ($scalar:ty) => {
        impl PiecewisePolynomial<$scalar> {
            /// Constructs a PP-form curve from exact SLATEC-format owned parts.
            ///
            /// `coefficients` is flat column-major `C(order, piece_count)`:
            /// each consecutive piece slice holds right Taylor derivatives in
            /// ascending derivative order. This constructor never sorts,
            /// merges, or otherwise changes the supplied representation.
            pub fn from_parts(
                breakpoints: Vec<$scalar>,
                coefficients: Vec<$scalar>,
                order: usize,
            ) -> Result<Self, PiecewisePolynomialError> {
                Self::from_parts_impl(breakpoints, coefficients, order)
            }

            /// Evaluates the PP-form curve at an in-domain point.
            ///
            /// At an interior breakpoint this uses the right piece; at the
            /// final breakpoint it uses the final piece's left endpoint.
            pub fn evaluate(&self, point: $scalar) -> Result<$scalar, PiecewisePolynomialError> {
                self.evaluate_impl(point)
            }

            /// Evaluates under an explicit Rust-side out-of-domain policy.
            pub fn evaluate_with_extrapolation(
                &self,
                point: $scalar,
                extrapolation: Extrapolation,
            ) -> Result<$scalar, PiecewisePolynomialError> {
                self.evaluate_with_extrapolation_impl(point, extrapolation)
            }

            /// Evaluates a derivative with order strictly below `order()`.
            pub fn derivative(
                &self,
                point: $scalar,
                derivative_order: usize,
            ) -> Result<$scalar, PiecewisePolynomialError> {
                self.derivative_impl(point, derivative_order)
            }

            /// Evaluates a derivative under an explicit out-of-domain policy.
            pub fn derivative_with_extrapolation(
                &self,
                point: $scalar,
                derivative_order: usize,
                extrapolation: Extrapolation,
            ) -> Result<$scalar, PiecewisePolynomialError> {
                self.derivative_with_extrapolation_impl(point, derivative_order, extrapolation)
            }

            /// Evaluates points into an exact-length caller-provided buffer.
            ///
            /// Query order is preserved. Each point uses a fresh native
            /// interval-search state, so no mutable cache escapes `&self`.
            pub fn evaluate_into(
                &self,
                points: &[$scalar],
                output: &mut [$scalar],
            ) -> Result<(), PiecewisePolynomialError> {
                self.evaluate_into_impl(points, output, Extrapolation::Error)
            }

            /// Batch-evaluates under an explicit out-of-domain policy.
            pub fn evaluate_into_with_extrapolation(
                &self,
                points: &[$scalar],
                output: &mut [$scalar],
                extrapolation: Extrapolation,
            ) -> Result<(), PiecewisePolynomialError> {
                self.evaluate_into_impl(points, output, extrapolation)
            }

            /// Integrates the PP representation exactly over an in-domain interval.
            ///
            /// Reversed limits return the negated integral and equal limits
            /// return zero, following `PPQAD`/`DPPQAD`.
            pub fn integrate(
                &self,
                lower: $scalar,
                upper: $scalar,
            ) -> Result<$scalar, PiecewisePolynomialError> {
                self.integrate_impl(lower, upper, Extrapolation::Error)
            }

            /// Integrates under an explicit Rust-side out-of-domain policy.
            pub fn integrate_with_extrapolation(
                &self,
                lower: $scalar,
                upper: $scalar,
                extrapolation: Extrapolation,
            ) -> Result<$scalar, PiecewisePolynomialError> {
                self.integrate_impl(lower, upper, extrapolation)
            }
        }
    };
}

impl_public_precision!(f32);
impl_public_precision!(f64);

#[cfg(feature = "bspline")]
macro_rules! impl_bspline_conversion {
    ($scalar:ty) => {
        impl crate::bspline::BSpline<$scalar> {
            /// Converts this B-spline exactly to its SLATEC PP representation.
            ///
            /// `BSPPP`/`DBSPPP` remove repeated knot values when forming PP
            /// breakpoints, retain right Taylor derivatives for each remaining
            /// interval, and preserve the B-spline's closed basic domain. No
            /// interpolation fit or numerical approximation is performed.
            pub fn to_piecewise_polynomial(
                &self,
            ) -> Result<PiecewisePolynomial<$scalar>, PiecewisePolynomialError> {
                let (knots, coefficients, order) = self.native_parts();
                PiecewisePolynomial::from_bspline_parts(knots, coefficients, order)
            }
        }
    };
}

#[cfg(feature = "bspline")]
impl_bspline_conversion!(f32);
#[cfg(feature = "bspline")]
impl_bspline_conversion!(f64);

fn validate_parts<T: PpScalar>(
    breakpoints: &[T],
    coefficients: &[T],
    order: usize,
) -> Result<(), PiecewisePolynomialError> {
    if order == 0 || native_len(order).is_err() {
        return Err(PiecewisePolynomialError::InvalidOrder);
    }
    if breakpoints.len() < 2 {
        return Err(PiecewisePolynomialError::TooFewBreakpoints);
    }
    let piece_count = breakpoints.len() - 1;
    native_len(piece_count)?;
    let expected = order
        .checked_mul(piece_count)
        .ok_or(PiecewisePolynomialError::DimensionOverflow)?;
    if coefficients.len() != expected {
        return Err(PiecewisePolynomialError::CoefficientCountMismatch {
            expected,
            actual: coefficients.len(),
        });
    }
    for (index, &breakpoint) in breakpoints.iter().enumerate() {
        if !breakpoint.finite() {
            return Err(PiecewisePolynomialError::NonFiniteInput);
        }
        if index > 0 && !(breakpoint > breakpoints[index - 1]) {
            return Err(PiecewisePolynomialError::BreakpointsNotStrictlyIncreasing { index });
        }
    }
    if coefficients
        .iter()
        .any(|&coefficient| !coefficient.finite())
    {
        return Err(PiecewisePolynomialError::NonFiniteInput);
    }
    Ok(())
}

fn native_len(length: usize) -> Result<FortranInteger, PiecewisePolynomialError> {
    FortranInteger::try_from(length).map_err(|_| PiecewisePolynomialError::DimensionOverflow)
}

#[cfg(feature = "bspline")]
fn zeroed<T: Copy + Default>(length: usize) -> Result<Vec<T>, PiecewisePolynomialError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| PiecewisePolynomialError::AllocationFailed)?;
    values.resize(length, T::default());
    Ok(values)
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;

    #[test]
    fn validates_native_pp_storage_before_entry() {
        assert_eq!(
            PiecewisePolynomial::<f64>::from_parts(vec![0.0], vec![], 1),
            Err(PiecewisePolynomialError::TooFewBreakpoints)
        );
        assert_eq!(
            PiecewisePolynomial::<f64>::from_parts(vec![0.0, 1.0], vec![1.0], 0),
            Err(PiecewisePolynomialError::InvalidOrder)
        );
        assert_eq!(
            PiecewisePolynomial::<f64>::from_parts(vec![0.0, 0.0], vec![1.0], 1),
            Err(PiecewisePolynomialError::BreakpointsNotStrictlyIncreasing { index: 1 })
        );
    }
}
