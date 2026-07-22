//! Checked owned facades for centered-grid cylindrical and polar FISHPACK solves.
//!
//! `HWSCYL` and `HWSPLR` have the same storage and workspace ABI, but their
//! radial and second-coordinate boundary restrictions differ.  The public
//! types make those restrictions explicit and never expose FISHPACK leading
//! dimensions, dummy arrays, or reusable native work storage.

use alloc::vec::Vec;
use core::convert::TryFrom;
use core::fmt;
use core::ops::{Index, IndexMut};

use slatec_sys::FortranInteger;

use crate::runtime::lock_native;

/// A finite, strictly increasing uniformly spaced coordinate axis.
///
/// `panels` is the number of equal intervals, so the axis has `panels + 1`
/// nodes.  Centered FISHPACK cylindrical and polar drivers require at least
/// four panels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CoordinateAxis {
    lower: f32,
    upper: f32,
    panels: usize,
}

impl CoordinateAxis {
    /// Creates a checked coordinate axis.
    pub fn new(lower: f32, upper: f32, panels: usize) -> Result<Self, CurvilinearPdeError> {
        validate_axis(lower, upper, panels, false)?;
        Ok(Self {
            lower,
            upper,
            panels,
        })
    }

    /// Returns the lower coordinate.
    #[must_use]
    pub fn lower(self) -> f32 {
        self.lower
    }

    /// Returns the upper coordinate.
    #[must_use]
    pub fn upper(self) -> f32 {
        self.upper
    }

    /// Returns the number of panels.
    #[must_use]
    pub fn panels(self) -> usize {
        self.panels
    }

    /// Returns the number of grid nodes, including both endpoints.
    pub fn nodes(self) -> Result<usize, CurvilinearPdeError> {
        self.panels
            .checked_add(1)
            .ok_or(CurvilinearPdeError::DimensionOverflow)
    }
}

/// A finite, non-negative radial axis with a strictly positive span.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RadialAxis(CoordinateAxis);

impl RadialAxis {
    /// Creates a checked radial axis.  Its lower coordinate may be zero.
    pub fn new(lower: f32, upper: f32, panels: usize) -> Result<Self, CurvilinearPdeError> {
        validate_axis(lower, upper, panels, true)?;
        Ok(Self(CoordinateAxis {
            lower,
            upper,
            panels,
        }))
    }

    /// Returns the radial lower coordinate.
    #[must_use]
    pub fn lower(self) -> f32 {
        self.0.lower
    }

    /// Returns the radial upper coordinate.
    #[must_use]
    pub fn upper(self) -> f32 {
        self.0.upper
    }

    /// Returns the number of radial panels.
    #[must_use]
    pub fn panels(self) -> usize {
        self.0.panels
    }

    /// Returns the radial node count, including endpoints.
    pub fn nodes(self) -> Result<usize, CurvilinearPdeError> {
        self.0.nodes()
    }
}

/// A finite, strictly increasing coordinate interval sampled at staggered points.
///
/// `points` is the number of unknowns in the open interval.  The grid is
/// `lower + (i + 0.5) * (upper - lower) / points` for zero-based `i`, so it
/// contains neither endpoint.  The staggered FISHPACK drivers require at
/// least three points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaggeredCoordinateAxis {
    lower: f32,
    upper: f32,
    points: usize,
}

impl StaggeredCoordinateAxis {
    /// Creates a checked staggered coordinate interval.
    pub fn new(lower: f32, upper: f32, points: usize) -> Result<Self, CurvilinearPdeError> {
        validate_staggered_axis(lower, upper, points, false)?;
        Ok(Self {
            lower,
            upper,
            points,
        })
    }

    /// Returns the lower endpoint, which is not itself a grid point.
    #[must_use]
    pub fn lower(self) -> f32 {
        self.lower
    }

    /// Returns the upper endpoint, which is not itself a grid point.
    #[must_use]
    pub fn upper(self) -> f32 {
        self.upper
    }

    /// Returns the number of staggered unknowns in the interval.
    #[must_use]
    pub fn points(self) -> usize {
        self.points
    }
}

/// A non-negative staggered radial interval.
///
/// Even when the lower endpoint is zero, the first grid point is strictly
/// positive.  [`RadialBoundary::AxisDirichlet`] and
/// [`RadialBoundary::AxisNeumann`] represent the source-defined regularity
/// condition at that omitted endpoint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaggeredRadialAxis(StaggeredCoordinateAxis);

impl StaggeredRadialAxis {
    /// Creates a checked staggered radial interval.
    pub fn new(lower: f32, upper: f32, points: usize) -> Result<Self, CurvilinearPdeError> {
        validate_staggered_axis(lower, upper, points, true)?;
        Ok(Self(StaggeredCoordinateAxis {
            lower,
            upper,
            points,
        }))
    }

    /// Returns the radial lower endpoint.
    #[must_use]
    pub fn lower(self) -> f32 {
        self.0.lower
    }

    /// Returns the radial upper endpoint.
    #[must_use]
    pub fn upper(self) -> f32 {
        self.0.upper
    }

    /// Returns the number of staggered radial unknowns.
    #[must_use]
    pub fn points(self) -> usize {
        self.0.points
    }
}

/// An owned first-coordinate-fast two-dimensional FISHPACK grid.
///
/// `values[second * first_nodes + first]` is the value at the corresponding
/// grid node.  This is the contiguous representation of Fortran
/// `F(IDIMF, second_nodes)` when the private `IDIMF == first_nodes`.
#[derive(Clone, Debug, PartialEq)]
pub struct FishpackGrid2 {
    values: Vec<f32>,
    first_nodes: usize,
    second_nodes: usize,
}

impl FishpackGrid2 {
    /// Creates a grid from first-coordinate-fast values.
    pub fn new(
        first_nodes: usize,
        second_nodes: usize,
        values: Vec<f32>,
    ) -> Result<Self, CurvilinearPdeError> {
        let expected = area(first_nodes, second_nodes)?;
        if values.len() != expected {
            return Err(CurvilinearPdeError::InvalidGridShape {
                expected,
                actual: values.len(),
            });
        }
        Ok(Self {
            values,
            first_nodes,
            second_nodes,
        })
    }

    /// Allocates a zero-filled grid with checked dimensions.
    pub fn zeros(first_nodes: usize, second_nodes: usize) -> Result<Self, CurvilinearPdeError> {
        let length = area(first_nodes, second_nodes)?;
        Ok(Self {
            values: zeroed(length)?,
            first_nodes,
            second_nodes,
        })
    }

    /// Returns the number of nodes on the first coordinate.
    #[must_use]
    pub fn first_nodes(&self) -> usize {
        self.first_nodes
    }

    /// Returns the number of nodes on the second coordinate.
    #[must_use]
    pub fn second_nodes(&self) -> usize {
        self.second_nodes
    }

    /// Returns the contiguous first-coordinate-fast values.
    #[must_use]
    pub fn values(&self) -> &[f32] {
        &self.values
    }

    /// Returns the mutable contiguous first-coordinate-fast values.
    #[must_use]
    pub fn values_mut(&mut self) -> &mut [f32] {
        &mut self.values
    }

    /// Returns the value at `(first, second)` when it is in range.
    #[must_use]
    pub fn get(&self, first: usize, second: usize) -> Option<&f32> {
        self.offset(first, second)
            .map(|offset| &self.values[offset])
    }

    /// Returns the mutable value at `(first, second)` when it is in range.
    pub fn get_mut(&mut self, first: usize, second: usize) -> Option<&mut f32> {
        self.offset(first, second)
            .map(|offset| &mut self.values[offset])
    }

    fn offset(&self, first: usize, second: usize) -> Option<usize> {
        (first < self.first_nodes && second < self.second_nodes)
            .then_some(second * self.first_nodes + first)
    }
}

impl Index<(usize, usize)> for FishpackGrid2 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1)
            .expect("FishpackGrid2 indices satisfy the documented dimensions")
    }
}

impl IndexMut<(usize, usize)> for FishpackGrid2 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1)
            .expect("FishpackGrid2 indices satisfy the documented dimensions")
    }
}

/// Boundary conditions for a regular axial or angular coordinate.
///
/// Derivatives use the increasing-coordinate direction.  Every supplied edge
/// contains all nodes of the other coordinate, corners included.
#[derive(Clone, Debug, PartialEq)]
pub enum CoordinateBoundary {
    /// Identify the lower and upper coordinate planes periodically.
    Periodic,
    /// Prescribe values at both endpoints.
    Dirichlet {
        /// Values at the lower endpoint.
        lower: Vec<f32>,
        /// Values at the upper endpoint.
        upper: Vec<f32>,
    },
    /// Prescribe a lower value and upper increasing-coordinate derivative.
    DirichletNeumann {
        /// Values at the lower endpoint.
        lower: Vec<f32>,
        /// Increasing-coordinate derivatives at the upper endpoint.
        upper_derivative: Vec<f32>,
    },
    /// Prescribe increasing-coordinate derivatives at both endpoints.
    Neumann {
        /// Increasing-coordinate derivatives at the lower endpoint.
        lower_derivative: Vec<f32>,
        /// Increasing-coordinate derivatives at the upper endpoint.
        upper_derivative: Vec<f32>,
    },
    /// Prescribe a lower increasing-coordinate derivative and upper value.
    NeumannDirichlet {
        /// Increasing-coordinate derivatives at the lower endpoint.
        lower_derivative: Vec<f32>,
        /// Values at the upper endpoint.
        upper: Vec<f32>,
    },
}

/// Boundary conditions for the radial coordinate.
///
/// `Axis*` variants represent the documented `R = 0` unspecified-axis modes;
/// they are not a request for an arbitrary value at the origin.
#[derive(Clone, Debug, PartialEq)]
pub enum RadialBoundary {
    /// Prescribe values at both radial endpoints.
    Dirichlet {
        /// Values at the inner radius.
        lower: Vec<f32>,
        /// Values at the outer radius.
        upper: Vec<f32>,
    },
    /// Prescribe a lower value and upper radial derivative.
    DirichletNeumann {
        /// Values at the inner radius.
        lower: Vec<f32>,
        /// Radial derivatives at the outer radius.
        upper_derivative: Vec<f32>,
    },
    /// Prescribe radial derivatives at both endpoints.
    Neumann {
        /// Radial derivatives at the inner radius.
        lower_derivative: Vec<f32>,
        /// Radial derivatives at the outer radius.
        upper_derivative: Vec<f32>,
    },
    /// Prescribe a lower radial derivative and upper value.
    NeumannDirichlet {
        /// Radial derivatives at the inner radius.
        lower_derivative: Vec<f32>,
        /// Values at the outer radius.
        upper: Vec<f32>,
    },
    /// Use FISHPACK's unspecified-axis condition at `R = 0` and prescribe the outer value.
    AxisDirichlet {
        /// Values at the outer radius.
        outer: Vec<f32>,
    },
    /// Use FISHPACK's unspecified-axis condition at `R = 0` and prescribe the outer derivative.
    AxisNeumann {
        /// Radial derivatives at the outer radius.
        outer_derivative: Vec<f32>,
    },
}

/// A checked centered-grid cylindrical Helmholtz problem.
///
/// The native operation is `HWSCYL`, which approximates
/// `(1/r)(r u_r)_r + u_zz + lambda/r^2 u = rhs` on an `r × z` grid.
#[derive(Clone, Debug, PartialEq)]
pub struct CylindricalHelmholtz2d(CenteredProblem);

impl CylindricalHelmholtz2d {
    /// Validates and owns a centered-grid cylindrical problem.
    pub fn new(
        radius: RadialAxis,
        axial: CoordinateAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        radial_boundary: RadialBoundary,
        axial_boundary: CoordinateBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        CenteredProblem::new(
            ProblemKind::Cylindrical,
            radius,
            axial,
            coefficient,
            rhs,
            radial_boundary,
            axial_boundary,
        )
        .map(Self)
    }

    /// Solves through the reviewed single-precision `HWSCYL` driver.
    pub fn solve(self) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        self.0.solve(slatec_sys::pde::fishpack::hwscyl)
    }
}

/// A checked centered-grid polar Helmholtz problem.
///
/// The native operation is `HWSPLR`, which approximates
/// `(1/r)(r u_r)_r + (1/r^2)u_theta_theta + lambda u = rhs`.
#[derive(Clone, Debug, PartialEq)]
pub struct PolarHelmholtz2d(CenteredProblem);

impl PolarHelmholtz2d {
    /// Validates and owns a centered-grid polar problem.
    pub fn new(
        radius: RadialAxis,
        angle: CoordinateAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        radial_boundary: RadialBoundary,
        angular_boundary: CoordinateBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        CenteredProblem::new(
            ProblemKind::Polar,
            radius,
            angle,
            coefficient,
            rhs,
            radial_boundary,
            angular_boundary,
        )
        .map(Self)
    }

    /// Solves through the reviewed single-precision `HWSPLR` driver.
    pub fn solve(self) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        self.0.solve(slatec_sys::pde::fishpack::hwsplr)
    }
}

/// A checked staggered-grid cylindrical Helmholtz problem.
///
/// The native operation is `HSTCYL`, which samples both coordinates at
/// open-interval staggered points.  Endpoint values and derivatives belong in
/// the boundary objects; unlike [`CylindricalHelmholtz2d`], they are not
/// stored in the right-hand-side grid.
#[derive(Clone, Debug, PartialEq)]
pub struct StaggeredCylindricalHelmholtz2d(StaggeredProblem);

impl StaggeredCylindricalHelmholtz2d {
    /// Validates and owns a staggered cylindrical problem.
    pub fn new(
        radius: StaggeredRadialAxis,
        axial: StaggeredCoordinateAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        radial_boundary: RadialBoundary,
        axial_boundary: CoordinateBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        StaggeredProblem::new(
            ProblemKind::StaggeredCylindrical,
            radius,
            axial,
            coefficient,
            rhs,
            radial_boundary,
            axial_boundary,
        )
        .map(Self)
    }

    /// Solves through the reviewed single-precision `HSTCYL` driver.
    pub fn solve(self) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        self.0.solve(slatec_sys::pde::fishpack::hstcyl)
    }
}

/// A checked staggered-grid polar Helmholtz problem.
///
/// The native operation is `HSTPLR`.  Its unknowns are at open-interval
/// radial and angular points, while endpoint data remain private native
/// boundary buffers owned by this problem.
#[derive(Clone, Debug, PartialEq)]
pub struct StaggeredPolarHelmholtz2d(StaggeredProblem);

impl StaggeredPolarHelmholtz2d {
    /// Validates and owns a staggered polar problem.
    pub fn new(
        radius: StaggeredRadialAxis,
        angle: StaggeredCoordinateAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        radial_boundary: RadialBoundary,
        angular_boundary: CoordinateBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        StaggeredProblem::new(
            ProblemKind::StaggeredPolar,
            radius,
            angle,
            coefficient,
            rhs,
            radial_boundary,
            angular_boundary,
        )
        .map(Self)
    }

    /// Solves through the reviewed single-precision `HSTPLR` driver.
    pub fn solve(self) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        self.0.solve(slatec_sys::pde::fishpack::hstplr)
    }
}

/// An owned solution returned by a centered curvilinear FISHPACK driver.
#[derive(Clone, Debug, PartialEq)]
pub struct CurvilinearPdeSolution {
    values: FishpackGrid2,
    perturbation: f32,
    native_status: NativeCurvilinearPdeStatus,
}

impl CurvilinearPdeSolution {
    #[cfg(feature = "fishpack-spherical")]
    pub(super) fn from_native(
        values: FishpackGrid2,
        perturbation: f32,
        native_status: NativeCurvilinearPdeStatus,
    ) -> Self {
        Self {
            values,
            perturbation,
            native_status,
        }
    }

    /// Returns the solution on every grid node, including boundary nodes.
    #[must_use]
    pub fn values(&self) -> &FishpackGrid2 {
        &self.values
    }

    /// Consumes this result and returns the owned solution grid.
    #[must_use]
    pub fn into_values(self) -> FishpackGrid2 {
        self.values
    }

    /// Returns FISHPACK's compatibility correction subtracted from the RHS.
    ///
    /// A nonzero value means the selected singular Poisson system was solved
    /// after the documented correction; callers can compare its magnitude to
    /// the original RHS before accepting the result.
    #[must_use]
    pub fn perturbation(&self) -> f32 {
        self.perturbation
    }

    /// Returns the native completion status.
    #[must_use]
    pub fn native_status(&self) -> NativeCurvilinearPdeStatus {
        self.native_status
    }
}

/// The native completion state after a checked centered-grid solve.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeCurvilinearPdeStatus {
    /// The driver returned `IERROR = 0`.
    Success,
    /// The driver returned its documented attempted-solve coefficient warning.
    ///
    /// The meaning of the coefficient warning differs between the cylindrical
    /// and polar source prologues; the safe API preserves the exact code
    /// without over-generalizing that mathematical condition.
    CoefficientWarning {
        /// Exact native `IERROR` warning code.
        code: i32,
    },
}

impl NativeCurvilinearPdeStatus {
    /// Returns the exact native `IERROR` code.
    #[must_use]
    pub fn code(self) -> i32 {
        match self {
            Self::Success => 0,
            Self::CoefficientWarning { code } => code,
        }
    }
}

/// Validation, allocation, or unexpected native failure for these facades.
#[derive(Clone, Debug, PartialEq)]
pub enum CurvilinearPdeError {
    /// An axis endpoint was non-finite, unordered, or an invalid radial lower bound.
    InvalidAxis,
    /// A grid axis has fewer than the documented four panels.
    GridTooSmall {
        /// Requested panel count.
        panels: usize,
        /// Documented lower bound.
        minimum: usize,
    },
    /// A checked dimension or workspace computation overflowed Rust or Fortran storage.
    DimensionOverflow,
    /// The supplied RHS grid did not have the exact node dimensions.
    InvalidGridShape {
        /// Required element count.
        expected: usize,
        /// Supplied element count.
        actual: usize,
    },
    /// An input scalar, RHS, or boundary vector contains a non-finite value.
    NonFiniteInput {
        /// Human-readable input category.
        field: &'static str,
    },
    /// A supplied derivative or value edge did not span the other coordinate.
    InvalidBoundaryLength {
        /// Affected coordinate category.
        axis: &'static str,
        /// Required edge length.
        expected: usize,
        /// Supplied edge length.
        actual: usize,
    },
    /// Two Dirichlet edge values assigned the same corner inconsistently.
    InconsistentCornerValues,
    /// Periodic duplicate endpoint RHS samples differed.
    InconsistentPeriodicRightHandSide,
    /// The selected driver forbids the supplied radial/second-coordinate boundary pairing.
    UnsupportedBoundaryCombination {
        /// FISHPACK driver rejecting this pairing.
        routine: &'static str,
    },
    /// An unspecified radial-axis boundary was used away from `R = 0`.
    AxisBoundaryRequiresZeroRadius,
    /// A lower radial derivative was requested at `R = 0`.
    RadialDerivativeAtAxis,
    /// `HWSCYL` requires a zero coefficient for its unspecified-axis modes.
    AxisBoundaryRequiresZeroCoefficient,
    /// An owned workspace allocation failed.
    AllocationFailed,
    /// The native driver reported an invalid workspace requirement in `W(1)`.
    InconsistentNativeWorkspace {
        /// Value reported in native `W(1)`.
        reported: f32,
        /// Private workspace allocation length.
        allocated: usize,
    },
    /// A prevalidated call returned an unexpected `IERROR` code.
    NativeFailure {
        /// FISHPACK driver returning the code.
        routine: &'static str,
        /// Exact native `IERROR` value.
        code: i32,
    },
}

impl fmt::Display for CurvilinearPdeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAxis => formatter.write_str("axis endpoints are invalid"),
            Self::GridTooSmall { panels, minimum } => {
                write!(
                    formatter,
                    "axis has {panels} panels; at least {minimum} are required"
                )
            }
            Self::DimensionOverflow => formatter.write_str("PDE dimension or workspace overflowed"),
            Self::InvalidGridShape { expected, actual } => {
                write!(formatter, "grid has {actual} values; expected {expected}")
            }
            Self::NonFiniteInput { field } => write!(formatter, "{field} must be finite"),
            Self::InvalidBoundaryLength {
                axis,
                expected,
                actual,
            } => write!(
                formatter,
                "{axis} boundary has length {actual}; expected {expected}"
            ),
            Self::InconsistentCornerValues => {
                formatter.write_str("two prescribed boundary values disagree at a corner")
            }
            Self::InconsistentPeriodicRightHandSide => {
                formatter.write_str("periodic duplicate endpoint RHS samples must agree")
            }
            Self::UnsupportedBoundaryCombination { routine } => {
                write!(
                    formatter,
                    "the supplied boundary combination is not accepted by {routine}"
                )
            }
            Self::AxisBoundaryRequiresZeroRadius => formatter
                .write_str("an unspecified axis boundary requires a radial lower endpoint of zero"),
            Self::RadialDerivativeAtAxis => {
                formatter.write_str("a radial derivative boundary cannot be used at r = 0")
            }
            Self::AxisBoundaryRequiresZeroCoefficient => {
                formatter.write_str("HWSCYL requires a zero coefficient with an unspecified axis")
            }
            Self::AllocationFailed => formatter.write_str("PDE workspace allocation failed"),
            Self::InconsistentNativeWorkspace {
                reported,
                allocated,
            } => write!(
                formatter,
                "native workspace report {reported} is invalid for allocation {allocated}"
            ),
            Self::NativeFailure { routine, code } => {
                write!(
                    formatter,
                    "{routine} returned unexpected native error code {code}"
                )
            }
        }
    }
}

impl std::error::Error for CurvilinearPdeError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProblemKind {
    Cylindrical,
    Polar,
    StaggeredCylindrical,
    StaggeredPolar,
}

#[derive(Clone, Debug, PartialEq)]
struct CenteredProblem {
    kind: ProblemKind,
    radius: RadialAxis,
    second: CoordinateAxis,
    coefficient: f32,
    rhs: FishpackGrid2,
    radial_boundary: RadialBoundary,
    second_boundary: CoordinateBoundary,
}

#[derive(Clone, Debug, PartialEq)]
struct StaggeredProblem {
    kind: ProblemKind,
    radius: StaggeredRadialAxis,
    second: StaggeredCoordinateAxis,
    coefficient: f32,
    rhs: FishpackGrid2,
    radial_boundary: RadialBoundary,
    second_boundary: CoordinateBoundary,
}

type CenteredDriver = unsafe extern "C" fn(
    *mut f32,
    *mut f32,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
    *mut FortranInteger,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut f32,
    *mut FortranInteger,
    *mut f32,
    *mut FortranInteger,
    *mut f32,
);

impl CenteredProblem {
    fn new(
        kind: ProblemKind,
        radius: RadialAxis,
        second: CoordinateAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        radial_boundary: RadialBoundary,
        second_boundary: CoordinateBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        if !coefficient.is_finite() {
            return Err(CurvilinearPdeError::NonFiniteInput {
                field: "Helmholtz coefficient",
            });
        }
        let first_nodes = radius.nodes()?;
        let second_nodes = second.nodes()?;
        let expected = area(first_nodes, second_nodes)?;
        if rhs.first_nodes != first_nodes || rhs.second_nodes != second_nodes {
            return Err(CurvilinearPdeError::InvalidGridShape {
                expected,
                actual: area(rhs.first_nodes, rhs.second_nodes)?,
            });
        }
        if rhs.values.iter().any(|value| !value.is_finite()) {
            return Err(CurvilinearPdeError::NonFiniteInput {
                field: "right-hand side",
            });
        }
        radial_boundary.validate(second_nodes)?;
        second_boundary.validate(first_nodes)?;
        validate_radial_boundary(
            kind,
            radius,
            coefficient,
            &radial_boundary,
            &second_boundary,
        )?;
        validate_periodic_rhs(&rhs, &second_boundary)?;
        validate_corners(
            &radial_boundary,
            &second_boundary,
            first_nodes,
            second_nodes,
        )?;
        Ok(Self {
            kind,
            radius,
            second,
            coefficient,
            rhs,
            radial_boundary,
            second_boundary,
        })
    }

    fn solve(self, driver: CenteredDriver) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        let first_nodes = self.radius.nodes()?;
        let second_nodes = self.second.nodes()?;
        let mut first_panels = FortranInteger::try_from(self.radius.panels())
            .map_err(|_| CurvilinearPdeError::DimensionOverflow)?;
        let mut second_panels = FortranInteger::try_from(self.second.panels())
            .map_err(|_| CurvilinearPdeError::DimensionOverflow)?;
        let mut first_dimension = FortranInteger::try_from(first_nodes)
            .map_err(|_| CurvilinearPdeError::DimensionOverflow)?;
        let mut radial_code = self.radial_boundary.code();
        let mut second_code = self.second_boundary.code();
        let mut lower_radius = self.radius.lower();
        let mut upper_radius = self.radius.upper();
        let mut lower_second = self.second.lower();
        let mut upper_second = self.second.upper();
        let mut coefficient = self.coefficient;
        let mut values = self.rhs.values;
        apply_dirichlet_edges(
            first_nodes,
            second_nodes,
            &mut values,
            &self.radial_boundary,
            &self.second_boundary,
        );
        let dummy = [0.0_f32];
        let lower_radial_derivative = self.radial_boundary.lower_derivative().unwrap_or(&dummy);
        let upper_radial_derivative = self.radial_boundary.upper_derivative().unwrap_or(&dummy);
        let lower_second_derivative = self.second_boundary.lower_derivative().unwrap_or(&dummy);
        let upper_second_derivative = self.second_boundary.upper_derivative().unwrap_or(&dummy);
        let workspace_length = workspace_len(first_nodes, second_nodes)?;
        let mut workspace = zeroed(workspace_length)?;
        let mut perturbation = 0.0;
        let mut native_code = 0;
        let _native = lock_native();
        // SAFETY: both reviewed drivers use the same GNU-Fortran ABI and
        // documented F(IDIMF, N+1) layout.  All private buffers are
        // contiguous, non-null, correctly sized, and remain live for the
        // entire native call.  Only the owned RHS/solution and workspace are
        // mutable native arrays.
        unsafe {
            driver(
                &mut lower_radius,
                &mut upper_radius,
                &mut first_panels,
                &mut radial_code,
                lower_radial_derivative.as_ptr().cast_mut(),
                upper_radial_derivative.as_ptr().cast_mut(),
                &mut lower_second,
                &mut upper_second,
                &mut second_panels,
                &mut second_code,
                lower_second_derivative.as_ptr().cast_mut(),
                upper_second_derivative.as_ptr().cast_mut(),
                &mut coefficient,
                values.as_mut_ptr(),
                &mut first_dimension,
                &mut perturbation,
                &mut native_code,
                workspace.as_mut_ptr(),
            );
        }
        if native_code != 0 && native_code != 11 {
            return Err(CurvilinearPdeError::NativeFailure {
                routine: self.kind.routine(),
                code: native_code,
            });
        }
        if !perturbation.is_finite() {
            return Err(CurvilinearPdeError::NativeFailure {
                routine: self.kind.routine(),
                code: native_code,
            });
        }
        let reported = workspace[0];
        if !reported.is_finite() || reported < 1.0 || reported > workspace_length as f32 {
            return Err(CurvilinearPdeError::InconsistentNativeWorkspace {
                reported,
                allocated: workspace_length,
            });
        }
        Ok(CurvilinearPdeSolution {
            values: FishpackGrid2 {
                values,
                first_nodes,
                second_nodes,
            },
            perturbation,
            native_status: if native_code == 0 {
                NativeCurvilinearPdeStatus::Success
            } else {
                NativeCurvilinearPdeStatus::CoefficientWarning { code: native_code }
            },
        })
    }
}

impl StaggeredProblem {
    fn new(
        kind: ProblemKind,
        radius: StaggeredRadialAxis,
        second: StaggeredCoordinateAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        radial_boundary: RadialBoundary,
        second_boundary: CoordinateBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        if !coefficient.is_finite() {
            return Err(CurvilinearPdeError::NonFiniteInput {
                field: "Helmholtz coefficient",
            });
        }
        let first_points = radius.points();
        let second_points = second.points();
        let expected = area(first_points, second_points)?;
        if rhs.first_nodes != first_points || rhs.second_nodes != second_points {
            return Err(CurvilinearPdeError::InvalidGridShape {
                expected,
                actual: area(rhs.first_nodes, rhs.second_nodes)?,
            });
        }
        if rhs.values.iter().any(|value| !value.is_finite()) {
            return Err(CurvilinearPdeError::NonFiniteInput {
                field: "right-hand side",
            });
        }
        radial_boundary.validate(second_points)?;
        second_boundary.validate(first_points)?;
        validate_staggered_radial_boundary(
            kind,
            radius,
            coefficient,
            &radial_boundary,
            &second_boundary,
        )?;
        Ok(Self {
            kind,
            radius,
            second,
            coefficient,
            rhs,
            radial_boundary,
            second_boundary,
        })
    }

    fn solve(self, driver: CenteredDriver) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        let first_points = self.radius.points();
        let second_points = self.second.points();
        let mut first_points_native = FortranInteger::try_from(first_points)
            .map_err(|_| CurvilinearPdeError::DimensionOverflow)?;
        let mut second_points_native = FortranInteger::try_from(second_points)
            .map_err(|_| CurvilinearPdeError::DimensionOverflow)?;
        let mut first_dimension = FortranInteger::try_from(first_points)
            .map_err(|_| CurvilinearPdeError::DimensionOverflow)?;
        let mut radial_code = self.radial_boundary.code();
        let mut second_code = self.second_boundary.code();
        let mut lower_radius = self.radius.lower();
        let mut upper_radius = self.radius.upper();
        let mut lower_second = self.second.lower();
        let mut upper_second = self.second.upper();
        let mut coefficient = self.coefficient;
        let mut values = self.rhs.values;
        let dummy = [0.0_f32];
        let lower_radial = self.radial_boundary.lower_data().unwrap_or(&dummy);
        let upper_radial = self.radial_boundary.upper_data().unwrap_or(&dummy);
        let lower_second_data = self.second_boundary.lower_data().unwrap_or(&dummy);
        let upper_second_data = self.second_boundary.upper_data().unwrap_or(&dummy);
        let workspace_length = staggered_workspace_len(first_points, second_points)?;
        let mut workspace = zeroed(workspace_length)?;
        let mut perturbation = 0.0;
        let mut native_code = 0;
        let _native = lock_native();
        // SAFETY: both reviewed staggered drivers share this source-verified
        // GNU-Fortran ABI.  Private buffers are non-null and exactly sized for
        // their documented M, N, and IDIMF contracts; no pointer is retained.
        unsafe {
            driver(
                &mut lower_radius,
                &mut upper_radius,
                &mut first_points_native,
                &mut radial_code,
                lower_radial.as_ptr().cast_mut(),
                upper_radial.as_ptr().cast_mut(),
                &mut lower_second,
                &mut upper_second,
                &mut second_points_native,
                &mut second_code,
                lower_second_data.as_ptr().cast_mut(),
                upper_second_data.as_ptr().cast_mut(),
                &mut coefficient,
                values.as_mut_ptr(),
                &mut first_dimension,
                &mut perturbation,
                &mut native_code,
                workspace.as_mut_ptr(),
            );
        }
        if native_code != 0 && native_code != 11 {
            return Err(CurvilinearPdeError::NativeFailure {
                routine: self.kind.routine(),
                code: native_code,
            });
        }
        if !perturbation.is_finite() {
            return Err(CurvilinearPdeError::NativeFailure {
                routine: self.kind.routine(),
                code: native_code,
            });
        }
        let reported = workspace[0];
        if !reported.is_finite() || reported < 1.0 || reported > workspace_length as f32 {
            return Err(CurvilinearPdeError::InconsistentNativeWorkspace {
                reported,
                allocated: workspace_length,
            });
        }
        Ok(CurvilinearPdeSolution {
            values: FishpackGrid2 {
                values,
                first_nodes: first_points,
                second_nodes: second_points,
            },
            perturbation,
            native_status: if native_code == 0 {
                NativeCurvilinearPdeStatus::Success
            } else {
                NativeCurvilinearPdeStatus::CoefficientWarning { code: native_code }
            },
        })
    }
}

impl ProblemKind {
    fn routine(self) -> &'static str {
        match self {
            Self::Cylindrical => "HWSCYL",
            Self::Polar => "HWSPLR",
            Self::StaggeredCylindrical => "HSTCYL",
            Self::StaggeredPolar => "HSTPLR",
        }
    }
}

impl CoordinateBoundary {
    pub(super) fn code(&self) -> FortranInteger {
        match self {
            Self::Periodic => 0,
            Self::Dirichlet { .. } => 1,
            Self::DirichletNeumann { .. } => 2,
            Self::Neumann { .. } => 3,
            Self::NeumannDirichlet { .. } => 4,
        }
    }

    pub(super) fn lower_value(&self) -> Option<&[f32]> {
        match self {
            Self::Dirichlet { lower, .. } | Self::DirichletNeumann { lower, .. } => Some(lower),
            Self::Periodic | Self::Neumann { .. } | Self::NeumannDirichlet { .. } => None,
        }
    }

    pub(super) fn upper_value(&self) -> Option<&[f32]> {
        match self {
            Self::Dirichlet { upper, .. } | Self::NeumannDirichlet { upper, .. } => Some(upper),
            Self::Periodic | Self::DirichletNeumann { .. } | Self::Neumann { .. } => None,
        }
    }

    pub(super) fn lower_derivative(&self) -> Option<&[f32]> {
        match self {
            Self::Neumann {
                lower_derivative, ..
            }
            | Self::NeumannDirichlet {
                lower_derivative, ..
            } => Some(lower_derivative),
            Self::Periodic | Self::Dirichlet { .. } | Self::DirichletNeumann { .. } => None,
        }
    }

    pub(super) fn upper_derivative(&self) -> Option<&[f32]> {
        match self {
            Self::DirichletNeumann {
                upper_derivative, ..
            }
            | Self::Neumann {
                upper_derivative, ..
            } => Some(upper_derivative),
            Self::Periodic | Self::Dirichlet { .. } | Self::NeumannDirichlet { .. } => None,
        }
    }

    pub(super) fn lower_data(&self) -> Option<&[f32]> {
        self.lower_value().or_else(|| self.lower_derivative())
    }

    pub(super) fn upper_data(&self) -> Option<&[f32]> {
        self.upper_value().or_else(|| self.upper_derivative())
    }

    pub(super) fn validate(&self, expected: usize) -> Result<(), CurvilinearPdeError> {
        validate_edges(
            [
                self.lower_value(),
                self.upper_value(),
                self.lower_derivative(),
                self.upper_derivative(),
            ],
            expected,
            "second-coordinate",
        )
    }
}

impl RadialBoundary {
    pub(super) fn code(&self) -> FortranInteger {
        match self {
            Self::Dirichlet { .. } => 1,
            Self::DirichletNeumann { .. } => 2,
            Self::Neumann { .. } => 3,
            Self::NeumannDirichlet { .. } => 4,
            Self::AxisDirichlet { .. } => 5,
            Self::AxisNeumann { .. } => 6,
        }
    }

    pub(super) fn lower_value(&self) -> Option<&[f32]> {
        match self {
            Self::Dirichlet { lower, .. } | Self::DirichletNeumann { lower, .. } => Some(lower),
            Self::Neumann { .. }
            | Self::NeumannDirichlet { .. }
            | Self::AxisDirichlet { .. }
            | Self::AxisNeumann { .. } => None,
        }
    }

    pub(super) fn upper_value(&self) -> Option<&[f32]> {
        match self {
            Self::Dirichlet { upper, .. }
            | Self::NeumannDirichlet { upper, .. }
            | Self::AxisDirichlet { outer: upper } => Some(upper),
            Self::DirichletNeumann { .. } | Self::Neumann { .. } | Self::AxisNeumann { .. } => None,
        }
    }

    pub(super) fn lower_derivative(&self) -> Option<&[f32]> {
        match self {
            Self::Neumann {
                lower_derivative, ..
            }
            | Self::NeumannDirichlet {
                lower_derivative, ..
            } => Some(lower_derivative),
            Self::Dirichlet { .. }
            | Self::DirichletNeumann { .. }
            | Self::AxisDirichlet { .. }
            | Self::AxisNeumann { .. } => None,
        }
    }

    pub(super) fn upper_derivative(&self) -> Option<&[f32]> {
        match self {
            Self::DirichletNeumann {
                upper_derivative, ..
            }
            | Self::Neumann {
                upper_derivative, ..
            }
            | Self::AxisNeumann {
                outer_derivative: upper_derivative,
            } => Some(upper_derivative),
            Self::Dirichlet { .. } | Self::NeumannDirichlet { .. } | Self::AxisDirichlet { .. } => {
                None
            }
        }
    }

    pub(super) fn lower_data(&self) -> Option<&[f32]> {
        self.lower_value().or_else(|| self.lower_derivative())
    }

    pub(super) fn upper_data(&self) -> Option<&[f32]> {
        self.upper_value().or_else(|| self.upper_derivative())
    }

    pub(super) fn is_axis(&self) -> bool {
        matches!(self, Self::AxisDirichlet { .. } | Self::AxisNeumann { .. })
    }

    pub(super) fn validate(&self, expected: usize) -> Result<(), CurvilinearPdeError> {
        validate_edges(
            [
                self.lower_value(),
                self.upper_value(),
                self.lower_derivative(),
                self.upper_derivative(),
            ],
            expected,
            "radial",
        )
    }
}

fn validate_axis(
    lower: f32,
    upper: f32,
    panels: usize,
    radial: bool,
) -> Result<(), CurvilinearPdeError> {
    if !lower.is_finite() || !upper.is_finite() || lower >= upper || (radial && lower < 0.0) {
        return Err(CurvilinearPdeError::InvalidAxis);
    }
    if panels < 4 {
        return Err(CurvilinearPdeError::GridTooSmall { panels, minimum: 4 });
    }
    Ok(())
}

fn validate_staggered_axis(
    lower: f32,
    upper: f32,
    points: usize,
    radial: bool,
) -> Result<(), CurvilinearPdeError> {
    if !lower.is_finite() || !upper.is_finite() || lower >= upper || (radial && lower < 0.0) {
        return Err(CurvilinearPdeError::InvalidAxis);
    }
    if points < 3 {
        return Err(CurvilinearPdeError::GridTooSmall {
            panels: points,
            minimum: 3,
        });
    }
    Ok(())
}

fn validate_edges(
    edges: [Option<&[f32]>; 4],
    expected: usize,
    axis: &'static str,
) -> Result<(), CurvilinearPdeError> {
    for values in edges.into_iter().flatten() {
        if values.len() != expected {
            return Err(CurvilinearPdeError::InvalidBoundaryLength {
                axis,
                expected,
                actual: values.len(),
            });
        }
        if values.iter().any(|value| !value.is_finite()) {
            return Err(CurvilinearPdeError::NonFiniteInput {
                field: "boundary data",
            });
        }
    }
    Ok(())
}

fn validate_radial_boundary(
    kind: ProblemKind,
    radius: RadialAxis,
    coefficient: f32,
    radial: &RadialBoundary,
    second: &CoordinateBoundary,
) -> Result<(), CurvilinearPdeError> {
    if radial.is_axis() && radius.lower() != 0.0 {
        return Err(CurvilinearPdeError::AxisBoundaryRequiresZeroRadius);
    }
    if radius.lower() == 0.0
        && matches!(
            radial,
            RadialBoundary::Neumann { .. } | RadialBoundary::NeumannDirichlet { .. }
        )
    {
        return Err(CurvilinearPdeError::RadialDerivativeAtAxis);
    }
    if kind == ProblemKind::Cylindrical && radial.is_axis() && coefficient != 0.0 {
        return Err(CurvilinearPdeError::AxisBoundaryRequiresZeroCoefficient);
    }
    if kind == ProblemKind::Polar
        && radial.is_axis()
        && !matches!(
            second,
            CoordinateBoundary::Periodic | CoordinateBoundary::Neumann { .. }
        )
    {
        return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HWSPLR" });
    }
    Ok(())
}

fn validate_staggered_radial_boundary(
    kind: ProblemKind,
    radius: StaggeredRadialAxis,
    coefficient: f32,
    radial: &RadialBoundary,
    second: &CoordinateBoundary,
) -> Result<(), CurvilinearPdeError> {
    if radial.is_axis() && radius.lower() != 0.0 {
        return Err(CurvilinearPdeError::AxisBoundaryRequiresZeroRadius);
    }
    if radius.lower() > 0.0 && radial.is_axis() {
        return Err(CurvilinearPdeError::AxisBoundaryRequiresZeroRadius);
    }
    match kind {
        ProblemKind::StaggeredCylindrical => {
            if radius.lower() == 0.0 && !radial.is_axis() {
                return Err(CurvilinearPdeError::UnsupportedBoundaryCombination {
                    routine: "HSTCYL",
                });
            }
            if radial.is_axis() && coefficient != 0.0 {
                return Err(CurvilinearPdeError::AxisBoundaryRequiresZeroCoefficient);
            }
        }
        ProblemKind::StaggeredPolar => {
            if radius.lower() == 0.0
                && matches!(
                    radial,
                    RadialBoundary::Neumann { .. } | RadialBoundary::NeumannDirichlet { .. }
                )
            {
                return Err(CurvilinearPdeError::RadialDerivativeAtAxis);
            }
            if radial.is_axis()
                && !matches!(
                    second,
                    CoordinateBoundary::Periodic | CoordinateBoundary::Neumann { .. }
                )
            {
                return Err(CurvilinearPdeError::UnsupportedBoundaryCombination {
                    routine: "HSTPLR",
                });
            }
        }
        ProblemKind::Cylindrical | ProblemKind::Polar => {
            unreachable!("centered kinds use their own validator")
        }
    }
    Ok(())
}

fn validate_periodic_rhs(
    grid: &FishpackGrid2,
    boundary: &CoordinateBoundary,
) -> Result<(), CurvilinearPdeError> {
    if matches!(boundary, CoordinateBoundary::Periodic)
        && (0..grid.first_nodes)
            .any(|first| grid[(first, 0)] != grid[(first, grid.second_nodes - 1)])
    {
        return Err(CurvilinearPdeError::InconsistentPeriodicRightHandSide);
    }
    Ok(())
}

fn validate_corners(
    radial: &RadialBoundary,
    second: &CoordinateBoundary,
    first_nodes: usize,
    second_nodes: usize,
) -> Result<(), CurvilinearPdeError> {
    for (radial_value, second_value) in [
        (
            radial.lower_value().map(|values| values[0]),
            second.lower_value().map(|values| values[0]),
        ),
        (
            radial.upper_value().map(|values| values[0]),
            second.lower_value().map(|values| values[first_nodes - 1]),
        ),
        (
            radial.lower_value().map(|values| values[second_nodes - 1]),
            second.upper_value().map(|values| values[0]),
        ),
        (
            radial.upper_value().map(|values| values[second_nodes - 1]),
            second.upper_value().map(|values| values[first_nodes - 1]),
        ),
    ] {
        if let (Some(radial_value), Some(second_value)) = (radial_value, second_value) {
            if radial_value != second_value {
                return Err(CurvilinearPdeError::InconsistentCornerValues);
            }
        }
    }
    Ok(())
}

fn apply_dirichlet_edges(
    first_nodes: usize,
    second_nodes: usize,
    values: &mut [f32],
    radial: &RadialBoundary,
    second: &CoordinateBoundary,
) {
    if let Some(lower) = radial.lower_value() {
        for second_index in 0..second_nodes {
            values[second_index * first_nodes] = lower[second_index];
        }
    }
    if let Some(upper) = radial.upper_value() {
        for second_index in 0..second_nodes {
            values[second_index * first_nodes + first_nodes - 1] = upper[second_index];
        }
    }
    if let Some(lower) = second.lower_value() {
        values[..first_nodes].copy_from_slice(lower);
    }
    if let Some(upper) = second.upper_value() {
        values[(second_nodes - 1) * first_nodes..].copy_from_slice(upper);
    }
}

fn workspace_len(first_nodes: usize, second_nodes: usize) -> Result<usize, CurvilinearPdeError> {
    let log2_second = (usize::BITS - 1 - second_nodes.leading_zeros()) as usize;
    second_nodes
        .checked_mul(4)
        .and_then(|first| {
            log2_second
                .checked_add(13)
                .and_then(|factor| factor.checked_mul(first_nodes))
                .and_then(|second| first.checked_add(second))
        })
        .ok_or(CurvilinearPdeError::DimensionOverflow)
}

fn staggered_workspace_len(
    first_points: usize,
    second_points: usize,
) -> Result<usize, CurvilinearPdeError> {
    let log2_second = (usize::BITS - 1 - second_points.leading_zeros()) as usize;
    first_points
        .checked_mul(13)
        .and_then(|value| {
            second_points
                .checked_mul(4)
                .and_then(|other| value.checked_add(other))
        })
        .and_then(|value| {
            first_points
                .checked_mul(log2_second)
                .and_then(|other| value.checked_add(other))
        })
        .ok_or(CurvilinearPdeError::DimensionOverflow)
}

fn area(first: usize, second: usize) -> Result<usize, CurvilinearPdeError> {
    first
        .checked_mul(second)
        .ok_or(CurvilinearPdeError::DimensionOverflow)
}

fn zeroed(length: usize) -> Result<Vec<f32>, CurvilinearPdeError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| CurvilinearPdeError::AllocationFailed)?;
    values.resize(length, 0.0);
    Ok(values)
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::{
        CoordinateAxis, CoordinateBoundary, CurvilinearPdeError, CylindricalHelmholtz2d,
        FishpackGrid2, PolarHelmholtz2d, RadialAxis, RadialBoundary,
    };

    #[cfg(feature = "fishpack-cylindrical-polar-native-tests")]
    use super::{
        StaggeredCoordinateAxis, StaggeredCylindricalHelmholtz2d, StaggeredPolarHelmholtz2d,
        StaggeredRadialAxis,
    };

    #[test]
    fn radial_axis_and_boundary_restrictions_are_checked() {
        assert!(matches!(
            RadialAxis::new(-1.0, 1.0, 4),
            Err(CurvilinearPdeError::InvalidAxis)
        ));
        let radius = RadialAxis::new(0.0, 1.0, 4).unwrap();
        let second = CoordinateAxis::new(0.0, 1.0, 4).unwrap();
        let grid = FishpackGrid2::zeros(5, 5).unwrap();
        assert!(matches!(
            CylindricalHelmholtz2d::new(
                radius,
                second,
                1.0,
                grid,
                RadialBoundary::AxisDirichlet {
                    outer: vec![0.0; 5]
                },
                CoordinateBoundary::Periodic,
            ),
            Err(CurvilinearPdeError::AxisBoundaryRequiresZeroCoefficient)
        ));
    }

    #[test]
    fn polar_axis_requires_periodic_or_derivative_angle_boundary() {
        let radius = RadialAxis::new(0.0, 1.0, 4).unwrap();
        let angle = CoordinateAxis::new(0.0, 1.0, 4).unwrap();
        let grid = FishpackGrid2::zeros(5, 5).unwrap();
        assert!(matches!(
            PolarHelmholtz2d::new(
                radius,
                angle,
                0.0,
                grid,
                RadialBoundary::AxisDirichlet {
                    outer: vec![0.0; 5]
                },
                CoordinateBoundary::Dirichlet {
                    lower: vec![0.0; 5],
                    upper: vec![0.0; 5]
                },
            ),
            Err(CurvilinearPdeError::UnsupportedBoundaryCombination { .. })
        ));
    }

    #[test]
    fn grid_uses_first_coordinate_fast_layout() {
        let grid = FishpackGrid2::new(3, 2, vec![0.0, 1.0, 2.0, 10.0, 11.0, 12.0]).unwrap();
        assert_eq!(grid[(2, 0)], 2.0);
        assert_eq!(grid[(1, 1)], 11.0);
    }

    #[cfg(feature = "fishpack-cylindrical-polar-native-tests")]
    #[test]
    fn native_cylindrical_and_polar_constant_solutions_are_manufactured() {
        let radius = RadialAxis::new(1.0, 2.0, 8).unwrap();
        let second = CoordinateAxis::new(0.0, 1.0, 8).unwrap();
        let zeros = || FishpackGrid2::zeros(9, 9).unwrap();
        let radial = || RadialBoundary::Dirichlet {
            lower: vec![1.0; 9],
            upper: vec![1.0; 9],
        };
        let cylindrical = CylindricalHelmholtz2d::new(
            radius,
            second,
            0.0,
            zeros(),
            radial(),
            CoordinateBoundary::Dirichlet {
                lower: vec![1.0; 9],
                upper: vec![1.0; 9],
            },
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            cylindrical
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 1.0e-4)
        );

        let polar = PolarHelmholtz2d::new(
            radius,
            second,
            0.0,
            zeros(),
            radial(),
            CoordinateBoundary::Periodic,
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            polar
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 1.0e-4)
        );
    }

    #[cfg(feature = "fishpack-cylindrical-polar-native-tests")]
    #[test]
    fn native_staggered_cylindrical_and_polar_constant_solutions_are_manufactured() {
        let radius = StaggeredRadialAxis::new(1.0, 2.0, 8).unwrap();
        let second = StaggeredCoordinateAxis::new(0.0, 1.0, 8).unwrap();
        let zeros = || FishpackGrid2::zeros(8, 8).unwrap();
        let radial = || RadialBoundary::Dirichlet {
            lower: vec![1.0; 8],
            upper: vec![1.0; 8],
        };
        let cylindrical = StaggeredCylindricalHelmholtz2d::new(
            radius,
            second,
            0.0,
            zeros(),
            radial(),
            CoordinateBoundary::Dirichlet {
                lower: vec![1.0; 8],
                upper: vec![1.0; 8],
            },
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            cylindrical
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 1.0e-4)
        );

        let polar = StaggeredPolarHelmholtz2d::new(
            radius,
            second,
            0.0,
            zeros(),
            radial(),
            CoordinateBoundary::Periodic,
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            polar
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 1.0e-4)
        );
    }
}
