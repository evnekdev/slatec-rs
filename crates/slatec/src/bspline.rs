//! Safe owned scalar B-splines backed by the reviewed SLATEC B-spline package.
//!
//! [`BSpline`] represents the native scalar form
//! `sum_j coefficients[j] * B_j,order(x)`. It owns its nondecreasing knot
//! vector and coefficient vector, and retains the B-spline *order* (one more
//! than the polynomial degree). `f32` calls `BVALU` and `BSQAD`; `f64` calls
//! `DBVALU` and `DBSQAD`.
//!
//! This is intentionally a narrow representation API. It neither constructs
//! knots from interpolation data nor exposes basis functions, tensor-product
//! splines, smoothing, NURBS, arbitrary strides, or matrix adapters. Native
//! calls are serialized through the process-wide runtime lock because the
//! reachable XERROR subsystem is process-global. Inputs are prevalidated so
//! that reviewed level-two `XERMSG` paths are unreachable in safe calls; the
//! XERROR control setting is nevertheless scoped and restored around every
//! native call.

use alloc::vec::Vec;
use core::convert::TryFrom;
use core::ops::RangeInclusive;

use slatec_sys::FortranInteger;

use crate::runtime::{lock_native, permit_recoverable_native_statuses};

/// An error from B-spline validation, allocation, or a contradicted native contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BSplineError {
    /// The order is zero or cannot fit SLATEC's `INTEGER` ABI.
    InvalidOrder,
    /// There are fewer coefficients than the B-spline order requires.
    TooFewCoefficients,
    /// The knot length is not exactly `coefficients.len() + order`.
    KnotCountMismatch {
        /// The required number of knots.
        expected: usize,
        /// The supplied number of knots.
        actual: usize,
    },
    /// A knot, coefficient, or query was NaN or infinite.
    NonFiniteInput,
    /// The knot vector decreases at the named zero-based position.
    KnotsNotNondecreasing {
        /// The index of the latter knot in the decreasing adjacent pair.
        index: usize,
    },
    /// One identical-knot run exceeds the B-spline order.
    ExcessiveKnotMultiplicity {
        /// The first zero-based index of the invalid equal-knot run.
        index: usize,
    },
    /// The basic B-spline domain has no positive width.
    EmptyDomain,
    /// An evaluation or integration endpoint is outside the basic knot domain.
    OutOfDomain,
    /// The requested derivative is not below the B-spline order.
    DerivativeOrderTooHigh {
        /// The requested derivative order.
        requested: usize,
        /// The maximum supported derivative order.
        maximum: usize,
    },
    /// A caller-provided output buffer does not match the query length.
    OutputLengthMismatch {
        /// Number of query points.
        expected: usize,
        /// Number of output elements.
        actual: usize,
    },
    /// `BSQAD`/`DBSQAD` support orders at most twenty.
    IntegrationOrderTooHigh {
        /// The B-spline order.
        order: usize,
        /// The native integration limit.
        maximum: usize,
    },
    /// A dimension or exact workspace formula does not fit the native ABI.
    DimensionOverflow,
    /// A fallible allocation for private native work storage failed.
    AllocationFailed,
    /// Native behavior contradicted the reviewed B-spline contract.
    NativeContractViolation {
        /// A short explanation of the impossible native behavior.
        detail: &'static str,
    },
}

/// Controls handling of a finite query outside the B-spline basic domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extrapolation {
    /// Reject the query before native entry.
    Error,
    /// Evaluate the appropriate endpoint limit instead of extrapolating.
    ///
    /// This is Rust-side clamping, not a native extrapolation mode. A query
    /// below the domain uses the left endpoint and a query above it uses the
    /// right endpoint.
    Clamp,
}

/// An owned univariate scalar B-spline in the exact reviewed SLATEC storage format.
///
/// The spline owns a nondecreasing knot vector of length `N + K`, a
/// coefficient vector of length `N`, and the order `K`. Its basic closed
/// domain is `knots[K - 1]..=knots[N]`; at an interior knot the native
/// evaluator takes the right limit, while at the right endpoint it takes the
/// left limit. No vectors are sorted, merged, or altered after construction.
#[derive(Clone, Debug, PartialEq)]
pub struct BSpline<T> {
    knots: Vec<T>,
    coefficients: Vec<T>,
    order: usize,
}

impl<T> BSpline<T> {
    /// Returns the owned nondecreasing knot sequence.
    #[must_use]
    pub fn knots(&self) -> &[T] {
        &self.knots
    }

    /// Returns the B-spline coefficients in native order.
    #[must_use]
    pub fn coefficients(&self) -> &[T] {
        &self.coefficients
    }

    /// Returns the B-spline order `K`.
    #[must_use]
    pub const fn order(&self) -> usize {
        self.order
    }

    /// Returns the polynomial degree, equal to `order() - 1`.
    #[must_use]
    pub const fn degree(&self) -> usize {
        self.order - 1
    }

    /// Returns the number of coefficients.
    #[must_use]
    pub fn coefficient_count(&self) -> usize {
        self.coefficients.len()
    }

    /// Returns the closed basic domain on which SLATEC evaluates this spline.
    #[must_use]
    pub fn domain(&self) -> RangeInclusive<T>
    where
        T: Copy,
    {
        self.knots[self.order - 1]..=self.knots[self.coefficients.len()]
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

trait BSplineScalar: sealed::Sealed + Copy + Default + PartialOrd {
    /// Returns whether the scalar is finite.
    fn finite(self) -> bool;

    /// Calls `BVALU` or `DBVALU`.
    ///
    /// # Safety
    ///
    /// Every pointer must satisfy the complete reviewed native contract.
    #[allow(clippy::too_many_arguments)]
    unsafe fn evaluate_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: &FortranInteger,
        order: &FortranInteger,
        derivative_order: &FortranInteger,
        point: &Self,
        interval_state: &mut FortranInteger,
        workspace: *mut Self,
    ) -> Self;

    /// Calls `BSQAD` or `DBSQAD`.
    ///
    /// # Safety
    ///
    /// Every pointer must satisfy the complete reviewed native contract.
    #[allow(clippy::too_many_arguments)]
    unsafe fn integrate_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: &FortranInteger,
        order: &FortranInteger,
        lower: &Self,
        upper: &Self,
        integral: &mut Self,
        workspace: *mut Self,
    );
}

impl BSplineScalar for f32 {
    fn finite(self) -> bool {
        self.is_finite()
    }

    unsafe fn evaluate_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: &FortranInteger,
        order: &FortranInteger,
        derivative_order: &FortranInteger,
        point: &Self,
        interval_state: &mut FortranInteger,
        workspace: *mut Self,
    ) -> Self {
        // SAFETY: upheld by this trait's safety contract.
        unsafe {
            slatec_sys::bspline::bvalu(
                knots,
                coefficients,
                coefficient_count,
                order,
                derivative_order,
                point,
                interval_state,
                workspace,
            )
        }
    }

    unsafe fn integrate_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: &FortranInteger,
        order: &FortranInteger,
        lower: &Self,
        upper: &Self,
        integral: &mut Self,
        workspace: *mut Self,
    ) {
        // SAFETY: upheld by this trait's safety contract.
        unsafe {
            slatec_sys::bspline::bsqad(
                knots,
                coefficients,
                coefficient_count,
                order,
                lower,
                upper,
                integral,
                workspace,
            )
        }
    }
}

impl BSplineScalar for f64 {
    fn finite(self) -> bool {
        self.is_finite()
    }

    unsafe fn evaluate_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: &FortranInteger,
        order: &FortranInteger,
        derivative_order: &FortranInteger,
        point: &Self,
        interval_state: &mut FortranInteger,
        workspace: *mut Self,
    ) -> Self {
        // SAFETY: upheld by this trait's safety contract.
        unsafe {
            slatec_sys::bspline::dbvalu(
                knots,
                coefficients,
                coefficient_count,
                order,
                derivative_order,
                point,
                interval_state,
                workspace,
            )
        }
    }

    unsafe fn integrate_native(
        knots: *const Self,
        coefficients: *const Self,
        coefficient_count: &FortranInteger,
        order: &FortranInteger,
        lower: &Self,
        upper: &Self,
        integral: &mut Self,
        workspace: *mut Self,
    ) {
        // SAFETY: upheld by this trait's safety contract.
        unsafe {
            slatec_sys::bspline::dbsqad(
                knots,
                coefficients,
                coefficient_count,
                order,
                lower,
                upper,
                integral,
                workspace,
            )
        }
    }
}

#[allow(private_bounds)]
impl<T: BSplineScalar> BSpline<T> {
    fn from_parts_impl(
        knots: Vec<T>,
        coefficients: Vec<T>,
        order: usize,
    ) -> Result<Self, BSplineError> {
        validate_parts(&knots, &coefficients, order)?;
        Ok(Self {
            knots,
            coefficients,
            order,
        })
    }

    fn evaluate_impl(&self, point: T) -> Result<T, BSplineError> {
        self.evaluate_with_extrapolation_impl(point, Extrapolation::Error)
    }

    fn evaluate_with_extrapolation_impl(
        &self,
        point: T,
        extrapolation: Extrapolation,
    ) -> Result<T, BSplineError> {
        self.evaluate_derivative_with_extrapolation(point, 0, extrapolation)
    }

    fn derivative_impl(&self, point: T, derivative_order: usize) -> Result<T, BSplineError> {
        self.evaluate_derivative_with_extrapolation(point, derivative_order, Extrapolation::Error)
    }

    fn derivative_with_extrapolation_impl(
        &self,
        point: T,
        derivative_order: usize,
        extrapolation: Extrapolation,
    ) -> Result<T, BSplineError> {
        self.evaluate_derivative_with_extrapolation(point, derivative_order, extrapolation)
    }

    fn evaluate_into_impl(&self, points: &[T], output: &mut [T]) -> Result<(), BSplineError> {
        self.evaluate_into_with_extrapolation_impl(points, output, Extrapolation::Error)
    }

    fn evaluate_into_with_extrapolation_impl(
        &self,
        points: &[T],
        output: &mut [T],
        extrapolation: Extrapolation,
    ) -> Result<(), BSplineError> {
        if points.len() != output.len() {
            return Err(BSplineError::OutputLengthMismatch {
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

    fn integrate_impl(&self, lower: T, upper: T) -> Result<T, BSplineError> {
        if self.order > 20 {
            return Err(BSplineError::IntegrationOrderTooHigh {
                order: self.order,
                maximum: 20,
            });
        }
        let lower = self.validate_in_domain(lower, Extrapolation::Error)?;
        let upper = self.validate_in_domain(upper, Extrapolation::Error)?;
        let mut workspace = zeroed::<T>(workspace_len(self.order)?)?;
        let coefficient_count = native_len(self.coefficients.len())?;
        let order = native_len(self.order)?;
        let mut integral = T::default();
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        // SAFETY: construction and endpoint preflight establish the exact
        // BSQAD/DBSQAD contract, including N >= K, T length N+K, K <= 20,
        // 3*K scratch values, and finite in-domain limits.
        unsafe {
            T::integrate_native(
                self.knots.as_ptr(),
                self.coefficients.as_ptr(),
                &coefficient_count,
                &order,
                &lower,
                &upper,
                &mut integral,
                workspace.as_mut_ptr(),
            )
        };
        Ok(integral)
    }

    fn evaluate_derivative_with_extrapolation(
        &self,
        point: T,
        derivative_order: usize,
        extrapolation: Extrapolation,
    ) -> Result<T, BSplineError> {
        if derivative_order >= self.order {
            return Err(BSplineError::DerivativeOrderTooHigh {
                requested: derivative_order,
                maximum: self.order - 1,
            });
        }
        let point = self.validate_in_domain(point, extrapolation)?;
        let mut workspace = zeroed::<T>(workspace_len(self.order)?)?;
        let coefficient_count = native_len(self.coefficients.len())?;
        let order = native_len(self.order)?;
        let derivative_order = native_len(derivative_order)?;
        // BVALU/DBVALU require `INBV=1` on the first call. Keeping it local
        // prevents any native search state from escaping the invocation.
        let mut interval_state = 1;
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        // SAFETY: construction and query preflight establish BVALU/DBVALU's
        // exact input contract. The private work array has length 3*K and all
        // pointers remain live for the complete native call.
        let value = unsafe {
            T::evaluate_native(
                self.knots.as_ptr(),
                self.coefficients.as_ptr(),
                &coefficient_count,
                &order,
                &derivative_order,
                &point,
                &mut interval_state,
                workspace.as_mut_ptr(),
            )
        };
        if interval_state < 1
            || usize::try_from(interval_state)
                .ok()
                .is_some_and(|value| value > self.coefficients.len())
        {
            return Err(BSplineError::NativeContractViolation {
                detail: "BVALU returned an invalid interval-search state",
            });
        }
        Ok(value)
    }

    fn validate_in_domain(
        &self,
        point: T,
        extrapolation: Extrapolation,
    ) -> Result<T, BSplineError> {
        if !point.finite() {
            return Err(BSplineError::NonFiniteInput);
        }
        let first = self.knots[self.order - 1];
        let last = self.knots[self.coefficients.len()];
        if point < first {
            return match extrapolation {
                Extrapolation::Error => Err(BSplineError::OutOfDomain),
                Extrapolation::Clamp => Ok(first),
            };
        }
        if point > last {
            return match extrapolation {
                Extrapolation::Error => Err(BSplineError::OutOfDomain),
                Extrapolation::Clamp => Ok(last),
            };
        }
        Ok(point)
    }
}

macro_rules! impl_public_bspline_precision {
    ($scalar:ty) => {
        impl BSpline<$scalar> {
            /// Constructs a B-spline from exact native-format parts.
            ///
            /// `knots` must have length `coefficients.len() + order`, be
            /// finite and nondecreasing, and have no equal-knot run longer
            /// than `order`. Coefficients must be finite, and the basic
            /// domain `knots[order - 1]..=knots[coefficients.len()]` must
            /// have positive width. Inputs are retained as supplied: this
            /// method performs no sorting, knot insertion, duplicate merging,
            /// interpolation fitting, or coefficient conversion.
            pub fn from_parts(
                knots: Vec<$scalar>,
                coefficients: Vec<$scalar>,
                order: usize,
            ) -> Result<Self, BSplineError> {
                Self::from_parts_impl(knots, coefficients, order)
            }

            /// Evaluates the spline value at an in-domain point.
            ///
            /// The native evaluator uses right limiting values at interior
            /// knots and the left limiting value at the right endpoint. It
            /// does not extrapolate.
            pub fn evaluate(&self, point: $scalar) -> Result<$scalar, BSplineError> {
                self.evaluate_impl(point)
            }

            /// Evaluates the spline under an explicit out-of-domain policy.
            ///
            /// [`Extrapolation::Clamp`] changes only the query point in Rust
            /// before calling SLATEC; it never requests native extrapolation.
            pub fn evaluate_with_extrapolation(
                &self,
                point: $scalar,
                extrapolation: Extrapolation,
            ) -> Result<$scalar, BSplineError> {
                self.evaluate_with_extrapolation_impl(point, extrapolation)
            }

            /// Evaluates one derivative of order strictly below `order()`.
            ///
            /// A derivative order of zero is the spline value. SLATEC's
            /// native derivative mode is used directly.
            pub fn derivative(
                &self,
                point: $scalar,
                derivative_order: usize,
            ) -> Result<$scalar, BSplineError> {
                self.derivative_impl(point, derivative_order)
            }

            /// Evaluates one derivative under an explicit out-of-domain policy.
            pub fn derivative_with_extrapolation(
                &self,
                point: $scalar,
                derivative_order: usize,
                extrapolation: Extrapolation,
            ) -> Result<$scalar, BSplineError> {
                self.derivative_with_extrapolation_impl(point, derivative_order, extrapolation)
            }

            /// Evaluates in-domain points into a caller-provided output slice.
            ///
            /// This preserves query order and does not allocate output. Each
            /// query uses fresh private native interval and work storage.
            pub fn evaluate_into(
                &self,
                points: &[$scalar],
                output: &mut [$scalar],
            ) -> Result<(), BSplineError> {
                self.evaluate_into_impl(points, output)
            }

            /// Evaluates values into a caller buffer under an explicit policy.
            pub fn evaluate_into_with_extrapolation(
                &self,
                points: &[$scalar],
                output: &mut [$scalar],
                extrapolation: Extrapolation,
            ) -> Result<(), BSplineError> {
                self.evaluate_into_with_extrapolation_impl(points, output, extrapolation)
            }

            /// Integrates the spline over an in-domain interval.
            ///
            /// Reversed finite limits are accepted and preserve SLATEC's
            /// signed integral convention. Native integration supports orders
            /// through twenty; evaluation remains available for larger orders.
            pub fn integrate(
                &self,
                lower: $scalar,
                upper: $scalar,
            ) -> Result<$scalar, BSplineError> {
                self.integrate_impl(lower, upper)
            }
        }
    };
}

impl_public_bspline_precision!(f32);
impl_public_bspline_precision!(f64);

fn validate_parts<T: BSplineScalar>(
    knots: &[T],
    coefficients: &[T],
    order: usize,
) -> Result<(), BSplineError> {
    if order == 0 {
        return Err(BSplineError::InvalidOrder);
    }
    native_len(order).map_err(|_| BSplineError::InvalidOrder)?;
    if coefficients.len() < order {
        return Err(BSplineError::TooFewCoefficients);
    }
    native_len(coefficients.len())?;
    let expected_knots = coefficients
        .len()
        .checked_add(order)
        .ok_or(BSplineError::DimensionOverflow)?;
    if knots.len() != expected_knots {
        return Err(BSplineError::KnotCountMismatch {
            expected: expected_knots,
            actual: knots.len(),
        });
    }
    native_len(knots.len())?;
    if knots.iter().any(|&value| !value.finite())
        || coefficients.iter().any(|&value| !value.finite())
    {
        return Err(BSplineError::NonFiniteInput);
    }
    let mut run_start = 0usize;
    let mut run_length = 1usize;
    for (index, pair) in knots.windows(2).enumerate() {
        if pair[1] < pair[0] {
            return Err(BSplineError::KnotsNotNondecreasing { index: index + 1 });
        }
        if pair[1] == pair[0] {
            run_length = run_length
                .checked_add(1)
                .ok_or(BSplineError::DimensionOverflow)?;
            if run_length > order {
                return Err(BSplineError::ExcessiveKnotMultiplicity { index: run_start });
            }
        } else {
            run_start = index + 1;
            run_length = 1;
        }
    }
    if knots[coefficients.len()] <= knots[order - 1] {
        return Err(BSplineError::EmptyDomain);
    }
    workspace_len(order)?;
    Ok(())
}

fn workspace_len(order: usize) -> Result<usize, BSplineError> {
    order.checked_mul(3).ok_or(BSplineError::DimensionOverflow)
}

fn native_len(length: usize) -> Result<FortranInteger, BSplineError> {
    FortranInteger::try_from(length).map_err(|_| BSplineError::DimensionOverflow)
}

fn zeroed<T: Copy + Default>(length: usize) -> Result<Vec<T>, BSplineError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| BSplineError::AllocationFailed)?;
    values.resize(length, T::default());
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn validates_native_storage_contract() {
        let spline =
            BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2).unwrap();
        assert_eq!(spline.order(), 2);
        assert_eq!(spline.degree(), 1);
        assert_eq!(spline.domain(), 0.0..=1.0);
        assert_eq!(
            BSpline::<f64>::from_parts(vec![0.0], vec![1.0], 1),
            Err(BSplineError::KnotCountMismatch {
                expected: 2,
                actual: 1
            })
        );
    }

    #[test]
    fn rejects_invalid_parts_before_native_entry() {
        assert_eq!(
            BSpline::<f32>::from_parts(vec![0.0], vec![1.0], 0),
            Err(BSplineError::InvalidOrder)
        );
        assert_eq!(
            BSpline::<f32>::from_parts(vec![0.0, 0.0, 0.0, 1.0], vec![1.0, 2.0], 2),
            Err(BSplineError::ExcessiveKnotMultiplicity { index: 0 })
        );
        assert_eq!(
            BSpline::<f32>::from_parts(vec![0.0, f32::NAN, 1.0, 1.0], vec![0.0, 1.0], 2),
            Err(BSplineError::NonFiniteInput)
        );
    }

    #[test]
    fn validates_derivative_and_batch_shapes_before_native_entry() {
        let spline =
            BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2).unwrap();
        assert_eq!(
            spline.derivative(0.5, 2),
            Err(BSplineError::DerivativeOrderTooHigh {
                requested: 2,
                maximum: 1
            })
        );
        let mut out = [0.0];
        assert_eq!(
            spline.evaluate_into(&[0.25, 0.5], &mut out),
            Err(BSplineError::OutputLengthMismatch {
                expected: 2,
                actual: 1
            })
        );
        assert_eq!(
            spline.evaluate_with_extrapolation(-1.0, Extrapolation::Error),
            Err(BSplineError::OutOfDomain)
        );
    }

    #[test]
    fn scoped_xerror_control_is_restored_after_evaluation_and_preflight_rejection() {
        let _runtime = lock_native();
        let mut before = 0;
        // SAFETY: the test holds the same process-wide runtime lock as the
        // safe facade, so it can observe XERROR's process-global flag.
        unsafe { slatec_sys::legacy_error::xgetf(&mut before) };

        let spline =
            BSpline::<f64>::from_parts(vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 1.0], 2).unwrap();
        assert!((spline.evaluate(0.25).unwrap() - 0.25).abs() < 1.0e-12);
        assert_eq!(spline.evaluate(-0.1), Err(BSplineError::OutOfDomain));

        let mut after = 0;
        // SAFETY: still protected by the process-wide native runtime lock.
        unsafe { slatec_sys::legacy_error::xgetf(&mut after) };
        assert_eq!(after, before);
    }
}
