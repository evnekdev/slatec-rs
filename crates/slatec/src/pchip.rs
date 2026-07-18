//! Safe owned PCHIP and piecewise-cubic Hermite interpolants.
//!
//! A [`PiecewiseCubicHermite`](crate::pchip::PiecewiseCubicHermite) stores exactly the knot values and first
//! derivatives used by SLATEC `PCHFD`/`DPCHFD`.  It is not a general spline
//! abstraction: its constructors use `PCHIM`/`DPCHIM`, `PCHIC`/`DPCHIC`, or
//! `PCHSP`/`DPCHSP`, and evaluation and integration use the original SLATEC
//! PCHIP implementation.  Knots are copied once into the owned curve; caller
//! buffers are never reordered, sorted, or merged.
//!
//! The public methods use contiguous storage only (`INCFD = 1`).  They hold
//! the process-wide native-runtime lock through the complete XERROR scope and
//! native call.  This protects process-global XERROR state and PCHIP's
//! DATA/SAVE constants; it does not claim parallel native execution.

use alloc::vec::Vec;
use core::convert::TryFrom;

use slatec_sys::{FortranInteger, FortranLogical};

use crate::runtime::{lock_native, permit_recoverable_native_statuses};

/// An error from validation, allocation, or an unexpected native PCHIP path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PchipError {
    /// Fewer than two knots were supplied.
    TooFewPoints,
    /// Knots, values, derivatives, points, or output slices have inconsistent lengths.
    LengthMismatch,
    /// An input scalar is NaN or infinite where the reviewed native contract requires finiteness.
    NonFiniteInput,
    /// Knots are not strictly increasing.
    KnotsNotStrictlyIncreasing,
    /// A typed endpoint condition contained a non-finite required value.
    InvalidEndpointCondition,
    /// A switch policy did not contain the required finite positive deviation limit.
    InvalidSwitchPolicy,
    /// An evaluation or integration request lies outside the knot domain under [`Extrapolation::Reject`].
    OutOfDomain,
    /// A dimension or exact workspace formula does not fit the native integer ABI.
    DimensionOverflow,
    /// Allocation of curve-owned data or native scratch storage failed.
    AllocationFailed,
    /// The native routine returned a documented failure that prevalidation could not rule out.
    NativeFailure {
        /// The reviewed SLATEC program unit.
        routine: &'static str,
        /// The original negative `IERR` status.
        code: FortranInteger,
    },
    /// A native output contradicted the reviewed PCHIP contract.
    NativeContractViolation {
        /// Short description of the violated contract.
        detail: &'static str,
    },
}

/// Controls whether a query may use the endpoint cubic outside the knot domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Extrapolation {
    /// Reject any point outside the closed knot domain before native entry.
    Reject,
    /// Preserve `PCHFE`/`PCHFD`/`PCHIA` endpoint-cubic extrapolation.
    Allow,
}

/// The endpoint policy accepted by SLATEC `PCHSP`/`DPCHSP`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointCondition<T> {
    /// Use PCHSP's not-a-knot condition, with its documented small-`N` fallback.
    NotAKnot,
    /// Prescribe the first derivative at the endpoint.
    FirstDerivative(T),
    /// Prescribe the second derivative at the endpoint; zero is the natural condition.
    SecondDerivative(T),
    /// Use PCHSP's three-point endpoint difference formula.
    ThreePoint,
    /// Use PCHSP's four-point endpoint difference formula.
    FourPoint,
}

/// A boundary policy accepted by SLATEC `PCHIC`/`DPCHIC`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MonotoneEndpointCondition<T> {
    /// Use the same default endpoint condition as `PCHIM`.
    Default,
    /// Prescribe a first derivative, optionally adjusting it to preserve endpoint monotonicity.
    FirstDerivative {
        /// Requested derivative.
        value: T,
        /// Whether PCHIC may change the requested value to preserve monotonicity.
        adjust_for_monotonicity: bool,
    },
    /// Prescribe a second derivative, optionally adjusting the resulting first derivative.
    SecondDerivative {
        /// Requested second derivative.
        value: T,
        /// Whether PCHIC may change the resulting first derivative for monotonicity.
        adjust_for_monotonicity: bool,
    },
    /// Use PCHIC's three-point endpoint formula.
    ThreePoint {
        /// Whether PCHIC may adjust the result for monotonicity.
        adjust_for_monotonicity: bool,
    },
    /// Use PCHIC's four-point endpoint formula.
    FourPoint {
        /// Whether PCHIC may adjust the result for monotonicity.
        adjust_for_monotonicity: bool,
    },
    /// Require PCHIC's second-derivative-continuous endpoint formula.
    SecondDerivativeContinuous {
        /// Whether PCHIC may adjust the result for monotonicity.
        adjust_for_monotonicity: bool,
    },
}

/// PCHIC's treatment of a change in data monotonicity direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchPointPolicy<T> {
    /// Set the derivative to zero at every switch, forcing a local extremum.
    ForceExtrema,
    /// Limit excursion near each switch to the supplied positive local-data multiple.
    LimitDeviation(T),
    /// Use the PCHIC three-point switch formula without an excursion limit.
    Unrestricted,
}

/// Information returned by native derivative construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstructionReport {
    /// No native warning was produced.
    None,
    /// `PCHIM` found this many changes in data monotonicity direction.
    MonotonicitySwitches(usize),
    /// `PCHIC` adjusted one or both requested endpoint derivatives.
    EndpointAdjusted {
        /// Whether the left endpoint was adjusted.
        beginning: bool,
        /// Whether the right endpoint was adjusted.
        end: bool,
    },
}

/// A value and first derivative evaluated at one point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HermiteEvaluation<T> {
    /// The interpolated value.
    pub value: T,
    /// The interpolated first derivative.
    pub first_derivative: T,
}

/// Valid output accompanied by the number of endpoint-cubic extrapolations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvaluationReport {
    /// Number of queried points outside the closed knot domain.
    pub extrapolated_points: usize,
}

/// A definite-integral result with explicit endpoint extrapolation information.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IntegrationReport<T> {
    /// The signed definite integral.
    pub value: T,
    /// Whether the lower bound lay outside the closed knot domain.
    pub lower_extrapolated: bool,
    /// Whether the upper bound lay outside the closed knot domain.
    pub upper_extrapolated: bool,
}

/// An owned piecewise-cubic Hermite curve represented by knots, values, and first derivatives.
///
/// The curve is defined interval-by-interval by the native SLATEC Hermite
/// representation.  `f32` uses `PCHIM`, `PCHIC`, `PCHSP`, `PCHFD`, and
/// `PCHIA`; `f64` uses the `D`-prefixed counterparts.  Inputs are copied into
/// owned contiguous vectors and later passed directly to the native routines.
#[derive(Clone, Debug, PartialEq)]
pub struct PiecewiseCubicHermite<T> {
    knots: Vec<T>,
    values: Vec<T>,
    derivatives: Vec<T>,
    construction: ConstructionReport,
}

impl<T> PiecewiseCubicHermite<T> {
    /// Returns the strictly increasing interpolation knots.
    #[must_use]
    pub fn knots(&self) -> &[T] {
        &self.knots
    }

    /// Returns the values stored at the knots.
    #[must_use]
    pub fn values(&self) -> &[T] {
        &self.values
    }

    /// Returns the first derivatives stored at the knots.
    #[must_use]
    pub fn derivatives(&self) -> &[T] {
        &self.derivatives
    }

    /// Returns the native construction warning information, if any.
    #[must_use]
    pub fn construction_report(&self) -> &ConstructionReport {
        &self.construction
    }

    /// Returns the number of knots.
    #[must_use]
    pub fn len(&self) -> usize {
        self.knots.len()
    }

    /// Returns whether this curve has no knots.
    ///
    /// Valid PCHIP curves contain at least two knots, so this always returns
    /// `false`; it is provided for collection-style API consistency.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        false
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// Internal scalar support for the two reviewed native PCHIP precisions.
///
/// This sealed trait is public only because methods on
/// [`PiecewiseCubicHermite`] are generic; downstream implementations are not
/// possible or required.
#[doc(hidden)]
pub trait PchipScalar: sealed::Sealed + Copy + Default + PartialOrd + core::fmt::Debug {
    /// True when the scalar is finite.
    fn finite(self) -> bool;
    /// The additive identity for the reviewed scalar type.
    fn zero() -> Self;
    /// Negative one for the reviewed scalar type.
    fn negative_one() -> Self;
    /// Calls the reviewed monotone derivative constructor.
    ///
    /// # Safety
    ///
    /// Every pointer must be non-null and valid for the exact native `PCHIM`
    /// or `DPCHIM` argument contract, including the `N`-element arrays.
    unsafe fn monotone(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        ierr: &mut FortranInteger,
    );
    /// Calls the reviewed controlled-monotone derivative constructor.
    ///
    /// # Safety
    ///
    /// Every pointer must be non-null and valid for the exact native `PCHIC`
    /// or `DPCHIC` argument contract, including its `2 * (N - 1)` workspace.
    unsafe fn controlled(
        ic: *const FortranInteger,
        vc: *const Self,
        switch: *const Self,
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        workspace: *mut Self,
        workspace_len: &mut FortranInteger,
        ierr: &mut FortranInteger,
    );
    /// Calls the reviewed spline derivative constructor.
    ///
    /// # Safety
    ///
    /// Every pointer must be non-null and valid for the exact native `PCHSP`
    /// or `DPCHSP` argument contract, including its `2 * N` workspace.
    unsafe fn spline(
        ic: *const FortranInteger,
        vc: *const Self,
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        workspace: *mut Self,
        workspace_len: &mut FortranInteger,
        ierr: &mut FortranInteger,
    );
    /// Evaluates values only without allocating a derivative output buffer.
    ///
    /// # Safety
    ///
    /// Every pointer must be non-null and valid for the exact native `PCHFE`
    /// or `DPCHFE` argument contract, including `count` point and output slots.
    unsafe fn evaluate(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        count: &mut FortranInteger,
        points: *const Self,
        values: *mut Self,
        ierr: &mut FortranInteger,
    );
    /// Evaluates values and derivatives.
    ///
    /// # Safety
    ///
    /// Every pointer must be non-null and valid for the exact native `PCHFD`
    /// or `DPCHFD` contract; its value and derivative outputs must be disjoint.
    unsafe fn evaluate_derivative(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        count: &mut FortranInteger,
        points: *const Self,
        values: *mut Self,
        derivatives: *mut Self,
        ierr: &mut FortranInteger,
    );
    /// Integrates the Hermite curve.
    ///
    /// # Safety
    ///
    /// Every pointer must be non-null and valid for the exact native `PCHIA`
    /// or `DPCHIA` argument contract.
    unsafe fn integrate(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        lower: &Self,
        upper: &Self,
        ierr: &mut FortranInteger,
    ) -> Self;
}

impl PchipScalar for f32 {
    fn finite(self) -> bool {
        self.is_finite()
    }
    fn zero() -> Self {
        0.0
    }
    fn negative_one() -> Self {
        -1.0
    }
    unsafe fn monotone(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        ierr: &mut FortranInteger,
    ) {
        unsafe { slatec_sys::pchip::pchim(n, x, f, d, incfd, ierr) }
    }
    unsafe fn controlled(
        ic: *const FortranInteger,
        vc: *const Self,
        switch: *const Self,
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        workspace: *mut Self,
        workspace_len: &mut FortranInteger,
        ierr: &mut FortranInteger,
    ) {
        unsafe {
            slatec_sys::pchip::pchic(
                ic,
                vc,
                switch,
                n,
                x,
                f,
                d,
                incfd,
                workspace,
                workspace_len,
                ierr,
            )
        }
    }
    unsafe fn spline(
        ic: *const FortranInteger,
        vc: *const Self,
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        workspace: *mut Self,
        workspace_len: &mut FortranInteger,
        ierr: &mut FortranInteger,
    ) {
        unsafe {
            slatec_sys::pchip::pchsp(ic, vc, n, x, f, d, incfd, workspace, workspace_len, ierr)
        }
    }
    unsafe fn evaluate(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        count: &mut FortranInteger,
        points: *const Self,
        values: *mut Self,
        ierr: &mut FortranInteger,
    ) {
        unsafe { slatec_sys::pchip::pchfe(n, x, f, d, incfd, skip, count, points, values, ierr) }
    }
    unsafe fn evaluate_derivative(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        count: &mut FortranInteger,
        points: *const Self,
        values: *mut Self,
        derivatives: *mut Self,
        ierr: &mut FortranInteger,
    ) {
        unsafe {
            slatec_sys::pchip::pchfd(
                n,
                x,
                f,
                d,
                incfd,
                skip,
                count,
                points,
                values,
                derivatives,
                ierr,
            )
        }
    }
    unsafe fn integrate(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        lower: &Self,
        upper: &Self,
        ierr: &mut FortranInteger,
    ) -> Self {
        unsafe { slatec_sys::pchip::pchia(n, x, f, d, incfd, skip, lower, upper, ierr) }
    }
}

impl PchipScalar for f64 {
    fn finite(self) -> bool {
        self.is_finite()
    }
    fn zero() -> Self {
        0.0
    }
    fn negative_one() -> Self {
        -1.0
    }
    unsafe fn monotone(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        ierr: &mut FortranInteger,
    ) {
        unsafe { slatec_sys::pchip::dpchim(n, x, f, d, incfd, ierr) }
    }
    unsafe fn controlled(
        ic: *const FortranInteger,
        vc: *const Self,
        switch: *const Self,
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        workspace: *mut Self,
        workspace_len: &mut FortranInteger,
        ierr: &mut FortranInteger,
    ) {
        unsafe {
            slatec_sys::pchip::dpchic(
                ic,
                vc,
                switch,
                n,
                x,
                f,
                d,
                incfd,
                workspace,
                workspace_len,
                ierr,
            )
        }
    }
    unsafe fn spline(
        ic: *const FortranInteger,
        vc: *const Self,
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *mut Self,
        incfd: &mut FortranInteger,
        workspace: *mut Self,
        workspace_len: &mut FortranInteger,
        ierr: &mut FortranInteger,
    ) {
        unsafe {
            slatec_sys::pchip::dpchsp(ic, vc, n, x, f, d, incfd, workspace, workspace_len, ierr)
        }
    }
    unsafe fn evaluate(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        count: &mut FortranInteger,
        points: *const Self,
        values: *mut Self,
        ierr: &mut FortranInteger,
    ) {
        unsafe { slatec_sys::pchip::dpchfe(n, x, f, d, incfd, skip, count, points, values, ierr) }
    }
    unsafe fn evaluate_derivative(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        count: &mut FortranInteger,
        points: *const Self,
        values: *mut Self,
        derivatives: *mut Self,
        ierr: &mut FortranInteger,
    ) {
        unsafe {
            slatec_sys::pchip::dpchfd(
                n,
                x,
                f,
                d,
                incfd,
                skip,
                count,
                points,
                values,
                derivatives,
                ierr,
            )
        }
    }
    unsafe fn integrate(
        n: &mut FortranInteger,
        x: *const Self,
        f: *const Self,
        d: *const Self,
        incfd: &mut FortranInteger,
        skip: &mut FortranLogical,
        lower: &Self,
        upper: &Self,
        ierr: &mut FortranInteger,
    ) -> Self {
        unsafe { slatec_sys::pchip::dpchia(n, x, f, d, incfd, skip, lower, upper, ierr) }
    }
}

impl<T: PchipScalar> PiecewiseCubicHermite<T> {
    /// Builds a monotonicity-preserving PCHIP curve with `PCHIM`/`DPCHIM`.
    ///
    /// On monotone input, each cubic interval is monotone.  For data with a
    /// direction switch, PCHIM forces an extremum at that knot; the switch
    /// count is available from [`Self::construction_report`].
    pub fn monotone(knots: &[T], values: &[T]) -> Result<Self, PchipError> {
        validate_knots_values(knots, values)?;
        let mut derivatives = zeroed_like(values)?;
        let mut n = native_len(knots.len())?;
        let mut incfd = 1;
        let mut ierr = 0;
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        // SAFETY: contiguous validated input slices, a same-length owned output,
        // and the reviewed `INCFD=1` ABI contract are live through this call.
        unsafe {
            T::monotone(
                &mut n,
                knots.as_ptr(),
                values.as_ptr(),
                derivatives.as_mut_ptr(),
                &mut incfd,
                &mut ierr,
            )
        };
        if ierr < 0 {
            return Err(native_failure("PCHIM/DPCHIM", ierr));
        }
        Ok(Self {
            knots: copy_slice(knots)?,
            values: copy_slice(values)?,
            derivatives,
            construction: if ierr == 0 {
                ConstructionReport::None
            } else {
                ConstructionReport::MonotonicitySwitches(ierr as usize)
            },
        })
    }

    /// Creates a curve from caller-supplied first derivatives without changing them.
    pub fn from_derivatives(
        knots: Vec<T>,
        values: Vec<T>,
        derivatives: Vec<T>,
    ) -> Result<Self, PchipError> {
        validate_full(&knots, &values, &derivatives)?;
        Ok(Self {
            knots,
            values,
            derivatives,
            construction: ConstructionReport::None,
        })
    }

    /// Builds a PCHIC curve with typed endpoint and switch policies.
    pub fn monotone_with_conditions(
        knots: &[T],
        values: &[T],
        beginning: MonotoneEndpointCondition<T>,
        end: MonotoneEndpointCondition<T>,
        switch_policy: SwitchPointPolicy<T>,
    ) -> Result<Self, PchipError> {
        validate_knots_values(knots, values)?;
        let (left_code, left_value) = monotone_endpoint(beginning)?;
        let (right_code, right_value) = monotone_endpoint(end)?;
        let switch = switch_value(switch_policy)?;
        let mut derivatives = zeroed_like(values)?;
        let mut workspace = zeroed::<T>(
            knots
                .len()
                .checked_sub(1)
                .and_then(|v| v.checked_mul(2))
                .ok_or(PchipError::DimensionOverflow)?,
        )?;
        let mut n = native_len(knots.len())?;
        let mut workspace_len = native_len(workspace.len())?;
        let mut incfd = 1;
        let mut ierr = 0;
        let ic = [left_code, right_code];
        let vc = [left_value, right_value];
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        // SAFETY: all arrays meet PCHIC's exact contiguous `INCFD=1` and
        // `NWK=2*(N-1)` contract; the curve owns no aliased mutable storage.
        unsafe {
            T::controlled(
                ic.as_ptr(),
                vc.as_ptr(),
                &switch,
                &mut n,
                knots.as_ptr(),
                values.as_ptr(),
                derivatives.as_mut_ptr(),
                &mut incfd,
                workspace.as_mut_ptr(),
                &mut workspace_len,
                &mut ierr,
            )
        };
        if ierr < 0 {
            return Err(native_failure("PCHIC/DPCHIC", ierr));
        }
        Ok(Self {
            knots: copy_slice(knots)?,
            values: copy_slice(values)?,
            derivatives,
            construction: endpoint_report(ierr),
        })
    }

    /// Builds the cubic-spline Hermite representation produced by `PCHSP`/`DPCHSP`.
    pub fn spline(
        knots: &[T],
        values: &[T],
        beginning: EndpointCondition<T>,
        end: EndpointCondition<T>,
    ) -> Result<Self, PchipError> {
        validate_knots_values(knots, values)?;
        let (left_code, left_value) = spline_endpoint(beginning)?;
        let (right_code, right_value) = spline_endpoint(end)?;
        let mut derivatives = zeroed_like(values)?;
        let mut workspace = zeroed::<T>(
            knots
                .len()
                .checked_mul(2)
                .ok_or(PchipError::DimensionOverflow)?,
        )?;
        let mut n = native_len(knots.len())?;
        let mut workspace_len = native_len(workspace.len())?;
        let mut incfd = 1;
        let mut ierr = 0;
        let ic = [left_code, right_code];
        let vc = [left_value, right_value];
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        // SAFETY: all input and owned scratch buffers meet the reviewed
        // PCHSP `NWK=2*N`, `INCFD=1` storage contract.
        unsafe {
            T::spline(
                ic.as_ptr(),
                vc.as_ptr(),
                &mut n,
                knots.as_ptr(),
                values.as_ptr(),
                derivatives.as_mut_ptr(),
                &mut incfd,
                workspace.as_mut_ptr(),
                &mut workspace_len,
                &mut ierr,
            )
        };
        if ierr < 0 {
            return Err(native_failure("PCHSP/DPCHSP", ierr));
        }
        Ok(Self {
            knots: copy_slice(knots)?,
            values: copy_slice(values)?,
            derivatives,
            construction: ConstructionReport::None,
        })
    }

    /// Evaluates one in-domain value with `PCHFE`/`DPCHFE`.
    pub fn evaluate(&self, point: T) -> Result<T, PchipError> {
        let mut output = [point];
        self.evaluate_with_policy_into(
            core::slice::from_ref(&point),
            &mut output,
            Extrapolation::Reject,
        )?;
        Ok(output[0])
    }

    /// Evaluates in-domain values into a caller-provided output buffer without allocation.
    pub fn evaluate_into(
        &self,
        points: &[T],
        output: &mut [T],
    ) -> Result<EvaluationReport, PchipError> {
        self.evaluate_with_policy_into(points, output, Extrapolation::Reject)
    }

    /// Evaluates values under an explicit endpoint-cubic extrapolation policy.
    pub fn evaluate_with_policy_into(
        &self,
        points: &[T],
        output: &mut [T],
        extrapolation: Extrapolation,
    ) -> Result<EvaluationReport, PchipError> {
        if points.len() != output.len() {
            return Err(PchipError::LengthMismatch);
        }
        let count = validate_points(self, points, extrapolation)?;
        if points.is_empty() {
            return Ok(EvaluationReport {
                extrapolated_points: 0,
            });
        }
        let mut n = native_len(self.len())?;
        let mut native_count = native_len(points.len())?;
        let mut incfd = 1;
        let mut skip: FortranLogical = 1;
        let mut ierr = 0;
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        // SAFETY: curve invariants were checked at construction; `points` and
        // `output` are exact-length, non-overlapping Rust slices for PCHFE.
        unsafe {
            T::evaluate(
                &mut n,
                self.knots.as_ptr(),
                self.values.as_ptr(),
                self.derivatives.as_ptr(),
                &mut incfd,
                &mut skip,
                &mut native_count,
                points.as_ptr(),
                output.as_mut_ptr(),
                &mut ierr,
            )
        };
        evaluation_status(ierr, count)
    }

    /// Evaluates one in-domain value and first derivative with `PCHFD`/`DPCHFD`.
    pub fn evaluate_with_derivative(&self, point: T) -> Result<HermiteEvaluation<T>, PchipError> {
        let mut values = [point];
        let mut derivatives = [point];
        self.evaluate_with_derivatives_into(
            core::slice::from_ref(&point),
            &mut values,
            &mut derivatives,
        )?;
        Ok(HermiteEvaluation {
            value: values[0],
            first_derivative: derivatives[0],
        })
    }

    /// Evaluates in-domain values and first derivatives into caller-provided buffers.
    pub fn evaluate_with_derivatives_into(
        &self,
        points: &[T],
        values: &mut [T],
        derivatives: &mut [T],
    ) -> Result<EvaluationReport, PchipError> {
        self.evaluate_with_derivatives_with_policy_into(
            points,
            values,
            derivatives,
            Extrapolation::Reject,
        )
    }

    /// Evaluates values and first derivatives under an explicit extrapolation policy.
    pub fn evaluate_with_derivatives_with_policy_into(
        &self,
        points: &[T],
        values: &mut [T],
        derivatives: &mut [T],
        extrapolation: Extrapolation,
    ) -> Result<EvaluationReport, PchipError> {
        if points.len() != values.len() || points.len() != derivatives.len() {
            return Err(PchipError::LengthMismatch);
        }
        let count = validate_points(self, points, extrapolation)?;
        if points.is_empty() {
            return Ok(EvaluationReport {
                extrapolated_points: 0,
            });
        }
        let mut n = native_len(self.len())?;
        let mut native_count = native_len(points.len())?;
        let mut incfd = 1;
        let mut skip: FortranLogical = 1;
        let mut ierr = 0;
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        // SAFETY: exact-length disjoint mutable output slices and immutable
        // curve/point storage meet the reviewed PCHFD ABI contract.
        unsafe {
            T::evaluate_derivative(
                &mut n,
                self.knots.as_ptr(),
                self.values.as_ptr(),
                self.derivatives.as_ptr(),
                &mut incfd,
                &mut skip,
                &mut native_count,
                points.as_ptr(),
                values.as_mut_ptr(),
                derivatives.as_mut_ptr(),
                &mut ierr,
            )
        };
        evaluation_status(ierr, count)
    }

    /// Integrates over an in-domain interval with `PCHIA`/`DPCHIA`.
    pub fn integrate(&self, lower: T, upper: T) -> Result<T, PchipError> {
        Ok(self
            .integrate_with_policy(lower, upper, Extrapolation::Reject)?
            .value)
    }

    /// Integrates under an explicit endpoint-cubic extrapolation policy.
    pub fn integrate_with_policy(
        &self,
        lower: T,
        upper: T,
        extrapolation: Extrapolation,
    ) -> Result<IntegrationReport<T>, PchipError> {
        if !lower.finite() || !upper.finite() {
            return Err(PchipError::NonFiniteInput);
        }
        let lower_outside = lower < self.knots[0] || lower > self.knots[self.len() - 1];
        let upper_outside = upper < self.knots[0] || upper > self.knots[self.len() - 1];
        if extrapolation == Extrapolation::Reject && (lower_outside || upper_outside) {
            return Err(PchipError::OutOfDomain);
        }
        let mut n = native_len(self.len())?;
        let mut incfd = 1;
        let mut skip: FortranLogical = 1;
        let mut ierr = 0;
        let _native = lock_native();
        let _xerror = permit_recoverable_native_statuses();
        // SAFETY: a previously validated curve and finite scalar limits meet
        // the PCHIA contract. The native function returns its scalar by value.
        let value = unsafe {
            T::integrate(
                &mut n,
                self.knots.as_ptr(),
                self.values.as_ptr(),
                self.derivatives.as_ptr(),
                &mut incfd,
                &mut skip,
                &lower,
                &upper,
                &mut ierr,
            )
        };
        if ierr < 0 {
            return Err(native_failure("PCHIA/DPCHIA", ierr));
        }
        if ierr > 3 {
            return Err(PchipError::NativeContractViolation {
                detail: "PCHIA returned an undocumented warning status",
            });
        }
        Ok(IntegrationReport {
            value,
            lower_extrapolated: ierr == 1 || ierr == 3,
            upper_extrapolated: ierr == 2 || ierr == 3,
        })
    }
}

fn copy_slice<T: Copy>(values: &[T]) -> Result<Vec<T>, PchipError> {
    let mut copied = Vec::new();
    copied
        .try_reserve_exact(values.len())
        .map_err(|_| PchipError::AllocationFailed)?;
    copied.extend_from_slice(values);
    Ok(copied)
}

fn zeroed<T: Copy + Default>(length: usize) -> Result<Vec<T>, PchipError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| PchipError::AllocationFailed)?;
    values.resize(length, T::default());
    Ok(values)
}

fn zeroed_like<T: Copy + Default>(values: &[T]) -> Result<Vec<T>, PchipError> {
    zeroed(values.len())
}

fn native_len(length: usize) -> Result<FortranInteger, PchipError> {
    FortranInteger::try_from(length).map_err(|_| PchipError::DimensionOverflow)
}

fn validate_knots_values<T: PchipScalar>(knots: &[T], values: &[T]) -> Result<(), PchipError> {
    if knots.len() < 2 {
        return Err(PchipError::TooFewPoints);
    }
    if knots.len() != values.len() {
        return Err(PchipError::LengthMismatch);
    }
    native_len(knots.len())?;
    for (&knot, &value) in knots.iter().zip(values) {
        if !knot.finite() || !value.finite() {
            return Err(PchipError::NonFiniteInput);
        }
    }
    if knots.windows(2).any(|pair| !(pair[1] > pair[0])) {
        return Err(PchipError::KnotsNotStrictlyIncreasing);
    }
    Ok(())
}

fn validate_full<T: PchipScalar>(
    knots: &[T],
    values: &[T],
    derivatives: &[T],
) -> Result<(), PchipError> {
    validate_knots_values(knots, values)?;
    if derivatives.len() != knots.len() {
        return Err(PchipError::LengthMismatch);
    }
    if derivatives.iter().any(|value| !value.finite()) {
        return Err(PchipError::NonFiniteInput);
    }
    Ok(())
}

fn validate_points<T: PchipScalar>(
    curve: &PiecewiseCubicHermite<T>,
    points: &[T],
    extrapolation: Extrapolation,
) -> Result<usize, PchipError> {
    native_len(points.len())?;
    let first = curve.knots[0];
    let last = curve.knots[curve.len() - 1];
    let mut count = 0usize;
    for &point in points {
        if !point.finite() {
            return Err(PchipError::NonFiniteInput);
        }
        if point < first || point > last {
            count = count.checked_add(1).ok_or(PchipError::DimensionOverflow)?;
        }
    }
    if extrapolation == Extrapolation::Reject && count > 0 {
        return Err(PchipError::OutOfDomain);
    }
    Ok(count)
}

fn spline_endpoint<T: PchipScalar>(
    condition: EndpointCondition<T>,
) -> Result<(FortranInteger, T), PchipError> {
    match condition {
        EndpointCondition::NotAKnot => Ok((0, T::zero())),
        EndpointCondition::FirstDerivative(value) => finite_endpoint(1, value),
        EndpointCondition::SecondDerivative(value) => finite_endpoint(2, value),
        EndpointCondition::ThreePoint => Ok((3, T::zero())),
        EndpointCondition::FourPoint => Ok((4, T::zero())),
    }
}

fn monotone_endpoint<T: PchipScalar>(
    condition: MonotoneEndpointCondition<T>,
) -> Result<(FortranInteger, T), PchipError> {
    let zero = T::zero();
    match condition {
        MonotoneEndpointCondition::Default => Ok((0, zero)),
        MonotoneEndpointCondition::FirstDerivative {
            value,
            adjust_for_monotonicity,
        } => finite_endpoint(if adjust_for_monotonicity { -1 } else { 1 }, value),
        MonotoneEndpointCondition::SecondDerivative {
            value,
            adjust_for_monotonicity,
        } => finite_endpoint(if adjust_for_monotonicity { -2 } else { 2 }, value),
        MonotoneEndpointCondition::ThreePoint {
            adjust_for_monotonicity,
        } => Ok((if adjust_for_monotonicity { -3 } else { 3 }, zero)),
        MonotoneEndpointCondition::FourPoint {
            adjust_for_monotonicity,
        } => Ok((if adjust_for_monotonicity { -4 } else { 4 }, zero)),
        MonotoneEndpointCondition::SecondDerivativeContinuous {
            adjust_for_monotonicity,
        } => Ok((if adjust_for_monotonicity { -5 } else { 5 }, zero)),
    }
}

fn finite_endpoint<T: PchipScalar>(
    code: FortranInteger,
    value: T,
) -> Result<(FortranInteger, T), PchipError> {
    if value.finite() {
        Ok((code, value))
    } else {
        Err(PchipError::InvalidEndpointCondition)
    }
}

fn switch_value<T: PchipScalar>(policy: SwitchPointPolicy<T>) -> Result<T, PchipError> {
    match policy {
        SwitchPointPolicy::ForceExtrema => Ok(T::zero()),
        SwitchPointPolicy::LimitDeviation(value) => {
            if value.finite() && value > T::zero() {
                Ok(value)
            } else {
                Err(PchipError::InvalidSwitchPolicy)
            }
        }
        SwitchPointPolicy::Unrestricted => Ok(T::negative_one()),
    }
}

fn endpoint_report(status: FortranInteger) -> ConstructionReport {
    match status {
        0 => ConstructionReport::None,
        1 => ConstructionReport::EndpointAdjusted {
            beginning: true,
            end: false,
        },
        2 => ConstructionReport::EndpointAdjusted {
            beginning: false,
            end: true,
        },
        3 => ConstructionReport::EndpointAdjusted {
            beginning: true,
            end: true,
        },
        _ => ConstructionReport::None,
    }
}

fn evaluation_status(
    status: FortranInteger,
    expected_extrapolations: usize,
) -> Result<EvaluationReport, PchipError> {
    if status < 0 {
        return Err(native_failure("PCHFE/PCHFD", status));
    }
    let returned = usize::try_from(status).map_err(|_| PchipError::NativeContractViolation {
        detail: "negative PCHIP extrapolation count",
    })?;
    if returned < expected_extrapolations {
        return Err(PchipError::NativeContractViolation {
            detail: "PCHIP missed a preflighted extrapolation point",
        });
    }
    Ok(EvaluationReport {
        extrapolated_points: returned,
    })
}

fn native_failure(routine: &'static str, code: FortranInteger) -> PchipError {
    PchipError::NativeFailure { routine, code }
}
