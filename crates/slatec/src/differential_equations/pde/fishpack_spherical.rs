//! Checked one-shot FISHPACK workflows in spherical coordinates.
//!
//! The source drivers retain work data for faster repeated calls.  This
//! module deliberately always requests fresh initialization and owns a fresh
//! workspace, so no FISHPACK continuation state or common-block cache leaks
//! into the public API.  Every native call remains process-serialized.

use alloc::vec::Vec;
use core::convert::TryFrom;

use slatec_sys::FortranInteger;

use crate::runtime::lock_native;

use super::fishpack_cylindrical_polar::{
    CoordinateBoundary, CurvilinearPdeError, CurvilinearPdeSolution, FishpackGrid2,
    NativeCurvilinearPdeStatus,
};

// This is the literal returned by the selected source's `PIMACH`, rounded to
// the reviewed IEEE single-precision profile.  Full-domain constructors use
// it rather than a caller-supplied approximation so the source's exact pole
// equality tests hold on that profile.
pub(super) const SLATEC_PI: f32 = 3.141_592_653_589_79_f32;

/// A checked colatitude interval in radians.
///
/// Colatitude is measured from the north pole.  The interval can include zero
/// and the source-accurate SLATEC value of pi, but all other endpoints must be
/// strictly inside that range.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColatitudeAxis {
    lower: f32,
    upper: f32,
    panels: usize,
}

impl ColatitudeAxis {
    /// Creates a finite increasing colatitude interval with at least five panels.
    pub fn new(lower: f32, upper: f32, panels: usize) -> Result<Self, CurvilinearPdeError> {
        validate_colatitude(lower, upper, panels)?;
        Ok(Self {
            lower,
            upper,
            panels,
        })
    }

    /// Creates the full north-pole-to-south-pole interval using SLATEC's pi.
    pub fn full_sphere(panels: usize) -> Result<Self, CurvilinearPdeError> {
        Self::new(0.0, SLATEC_PI, panels)
    }

    /// Returns the lower colatitude in radians.
    #[must_use]
    pub fn lower(self) -> f32 {
        self.lower
    }

    /// Returns the upper colatitude in radians.
    #[must_use]
    pub fn upper(self) -> f32 {
        self.upper
    }

    /// Returns the number of equal panels.
    #[must_use]
    pub fn panels(self) -> usize {
        self.panels
    }

    pub(super) fn nodes(self) -> Result<usize, CurvilinearPdeError> {
        self.panels
            .checked_add(1)
            .ok_or(CurvilinearPdeError::DimensionOverflow)
    }

    pub(super) fn includes_north_pole(self) -> bool {
        self.lower == 0.0
    }

    pub(super) fn includes_south_pole(self) -> bool {
        self.upper == SLATEC_PI
    }
}

/// A checked longitude interval in radians on the unit sphere.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LongitudeAxis {
    lower: f32,
    upper: f32,
    panels: usize,
}

impl LongitudeAxis {
    /// Creates a finite increasing longitude interval with at least five panels.
    pub fn new(lower: f32, upper: f32, panels: usize) -> Result<Self, CurvilinearPdeError> {
        let two_pi = 2.0 * SLATEC_PI;
        if !lower.is_finite()
            || !upper.is_finite()
            || lower < 0.0
            || upper > two_pi
            || lower >= upper
        {
            return Err(CurvilinearPdeError::InvalidAxis);
        }
        if panels < 5 {
            return Err(CurvilinearPdeError::GridTooSmall { panels, minimum: 5 });
        }
        Ok(Self {
            lower,
            upper,
            panels,
        })
    }

    /// Creates the full zero-to-two-pi longitude interval using SLATEC's pi.
    pub fn full_circle(panels: usize) -> Result<Self, CurvilinearPdeError> {
        Self::new(0.0, 2.0 * SLATEC_PI, panels)
    }

    /// Returns the lower longitude.
    #[must_use]
    pub fn lower(self) -> f32 {
        self.lower
    }

    /// Returns the upper longitude.
    #[must_use]
    pub fn upper(self) -> f32 {
        self.upper
    }

    /// Returns the number of longitude panels.
    #[must_use]
    pub fn panels(self) -> usize {
        self.panels
    }

    fn nodes(self) -> Result<usize, CurvilinearPdeError> {
        self.panels
            .checked_add(1)
            .ok_or(CurvilinearPdeError::DimensionOverflow)
    }
}

/// A staggered colatitude interval whose unknowns exclude both endpoints.
///
/// The `points` unknowns are at `lower + (i + 0.5) * (upper - lower) /
/// points`.  Zero and the source-accurate SLATEC value of pi remain available
/// as boundary endpoints for the source-defined pole regularity modes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaggeredColatitudeAxis {
    lower: f32,
    upper: f32,
    points: usize,
}

impl StaggeredColatitudeAxis {
    /// Creates a finite increasing colatitude interval with at least three unknowns.
    pub fn new(lower: f32, upper: f32, points: usize) -> Result<Self, CurvilinearPdeError> {
        validate_staggered_colatitude(lower, upper, points)?;
        Ok(Self {
            lower,
            upper,
            points,
        })
    }

    /// Creates the full north-pole-to-south-pole interval using SLATEC's pi.
    pub fn full_sphere(points: usize) -> Result<Self, CurvilinearPdeError> {
        Self::new(0.0, SLATEC_PI, points)
    }

    /// Returns the lower colatitude endpoint in radians.
    #[must_use]
    pub fn lower(self) -> f32 {
        self.lower
    }

    /// Returns the upper colatitude endpoint in radians.
    #[must_use]
    pub fn upper(self) -> f32 {
        self.upper
    }

    /// Returns the number of staggered colatitude unknowns.
    #[must_use]
    pub fn points(self) -> usize {
        self.points
    }

    pub(super) fn includes_north_pole(self) -> bool {
        self.lower == 0.0
    }

    pub(super) fn includes_south_pole(self) -> bool {
        self.upper == SLATEC_PI
    }
}

/// A staggered longitude interval whose unknowns exclude both endpoints.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StaggeredLongitudeAxis {
    lower: f32,
    upper: f32,
    points: usize,
}

impl StaggeredLongitudeAxis {
    /// Creates a finite increasing longitude interval with at least three unknowns.
    pub fn new(lower: f32, upper: f32, points: usize) -> Result<Self, CurvilinearPdeError> {
        let two_pi = 2.0 * SLATEC_PI;
        if !lower.is_finite()
            || !upper.is_finite()
            || lower < 0.0
            || upper > two_pi
            || lower >= upper
        {
            return Err(CurvilinearPdeError::InvalidAxis);
        }
        if points < 3 {
            return Err(CurvilinearPdeError::GridTooSmall {
                panels: points,
                minimum: 3,
            });
        }
        Ok(Self {
            lower,
            upper,
            points,
        })
    }

    /// Creates the full zero-to-two-pi longitude interval using SLATEC's pi.
    pub fn full_circle(points: usize) -> Result<Self, CurvilinearPdeError> {
        Self::new(0.0, 2.0 * SLATEC_PI, points)
    }

    /// Returns the lower longitude endpoint in radians.
    #[must_use]
    pub fn lower(self) -> f32 {
        self.lower
    }

    /// Returns the upper longitude endpoint in radians.
    #[must_use]
    pub fn upper(self) -> f32 {
        self.upper
    }

    /// Returns the number of staggered longitude unknowns.
    #[must_use]
    pub fn points(self) -> usize {
        self.points
    }
}

/// Colatitude boundary conditions for FISHPACK's spherical drivers.
///
/// Derivatives are with respect to increasing colatitude, not outward normal.
/// `NorthPole*` and `SouthPole*` are source-defined regularity modes; they do
/// not accept arbitrary point values at a coordinate singularity.  Every
/// supplied vector has one entry for each radial or longitude node.
#[derive(Clone, Debug, PartialEq)]
pub enum ColatitudeBoundary {
    /// Prescribe values at both colatitude endpoints.
    Dirichlet {
        /// Values at the lower colatitude.
        lower: Vec<f32>,
        /// Values at the upper colatitude.
        upper: Vec<f32>,
    },
    /// Prescribe a lower value and upper increasing-colatitude derivative.
    DirichletNeumann {
        /// Values at the lower colatitude.
        lower: Vec<f32>,
        /// Derivatives at the upper colatitude.
        upper_derivative: Vec<f32>,
    },
    /// Prescribe increasing-colatitude derivatives at both endpoints.
    Neumann {
        /// Derivatives at the lower colatitude.
        lower_derivative: Vec<f32>,
        /// Derivatives at the upper colatitude.
        upper_derivative: Vec<f32>,
    },
    /// Prescribe a lower derivative and upper value.
    NeumannDirichlet {
        /// Derivatives at the lower colatitude.
        lower_derivative: Vec<f32>,
        /// Values at the upper colatitude.
        upper: Vec<f32>,
    },
    /// Use north-pole regularity and prescribe the upper value.
    NorthPoleDirichlet {
        /// Values at the upper colatitude.
        upper: Vec<f32>,
    },
    /// Use north-pole regularity and prescribe the upper derivative.
    NorthPoleNeumann {
        /// Derivatives at the upper colatitude.
        upper_derivative: Vec<f32>,
    },
    /// Prescribe the lower value and use south-pole regularity.
    SouthPoleDirichlet {
        /// Values at the lower colatitude.
        lower: Vec<f32>,
    },
    /// Prescribe the lower derivative and use south-pole regularity.
    SouthPoleNeumann {
        /// Derivatives at the lower colatitude.
        lower_derivative: Vec<f32>,
    },
    /// Use source-defined regularity at both poles.
    BothPoles,
}

impl ColatitudeBoundary {
    pub(super) fn code(&self) -> FortranInteger {
        match self {
            Self::Dirichlet { .. } => 1,
            Self::DirichletNeumann { .. } => 2,
            Self::Neumann { .. } => 3,
            Self::NeumannDirichlet { .. } => 4,
            Self::NorthPoleDirichlet { .. } => 5,
            Self::NorthPoleNeumann { .. } => 6,
            Self::SouthPoleDirichlet { .. } => 7,
            Self::SouthPoleNeumann { .. } => 8,
            Self::BothPoles => 9,
        }
    }

    pub(super) fn lower_value(&self) -> Option<&[f32]> {
        match self {
            Self::Dirichlet { lower, .. }
            | Self::DirichletNeumann { lower, .. }
            | Self::SouthPoleDirichlet { lower } => Some(lower),
            Self::Neumann { .. }
            | Self::NeumannDirichlet { .. }
            | Self::NorthPoleDirichlet { .. }
            | Self::NorthPoleNeumann { .. }
            | Self::SouthPoleNeumann { .. }
            | Self::BothPoles => None,
        }
    }

    pub(super) fn upper_value(&self) -> Option<&[f32]> {
        match self {
            Self::Dirichlet { upper, .. }
            | Self::NeumannDirichlet { upper, .. }
            | Self::NorthPoleDirichlet { upper } => Some(upper),
            Self::DirichletNeumann { .. }
            | Self::Neumann { .. }
            | Self::NorthPoleNeumann { .. }
            | Self::SouthPoleDirichlet { .. }
            | Self::SouthPoleNeumann { .. }
            | Self::BothPoles => None,
        }
    }

    pub(super) fn lower_derivative(&self) -> Option<&[f32]> {
        match self {
            Self::Neumann {
                lower_derivative, ..
            }
            | Self::NeumannDirichlet {
                lower_derivative, ..
            }
            | Self::SouthPoleNeumann { lower_derivative } => Some(lower_derivative),
            Self::Dirichlet { .. }
            | Self::DirichletNeumann { .. }
            | Self::NorthPoleDirichlet { .. }
            | Self::NorthPoleNeumann { .. }
            | Self::SouthPoleDirichlet { .. }
            | Self::BothPoles => None,
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
            | Self::NorthPoleNeumann { upper_derivative } => Some(upper_derivative),
            Self::Dirichlet { .. }
            | Self::NeumannDirichlet { .. }
            | Self::NorthPoleDirichlet { .. }
            | Self::SouthPoleDirichlet { .. }
            | Self::SouthPoleNeumann { .. }
            | Self::BothPoles => None,
        }
    }

    pub(super) fn lower_data(&self) -> Option<&[f32]> {
        self.lower_value().or_else(|| self.lower_derivative())
    }

    pub(super) fn upper_data(&self) -> Option<&[f32]> {
        self.upper_value().or_else(|| self.upper_derivative())
    }

    pub(super) fn unspecified_pole(&self) -> bool {
        self.code() >= 5
    }

    pub(super) fn validate(
        &self,
        axis: ColatitudeAxis,
        expected: usize,
    ) -> Result<(), CurvilinearPdeError> {
        self.validate_with_poles(
            axis.includes_north_pole(),
            axis.includes_south_pole(),
            expected,
        )
    }

    pub(super) fn validate_staggered(
        &self,
        axis: StaggeredColatitudeAxis,
        expected: usize,
    ) -> Result<(), CurvilinearPdeError> {
        self.validate_with_poles(
            axis.includes_north_pole(),
            axis.includes_south_pole(),
            expected,
        )
    }

    fn validate_with_poles(
        &self,
        includes_north_pole: bool,
        includes_south_pole: bool,
        expected: usize,
    ) -> Result<(), CurvilinearPdeError> {
        for values in [
            self.lower_value(),
            self.upper_value(),
            self.lower_derivative(),
            self.upper_derivative(),
        ]
        .into_iter()
        .flatten()
        {
            validate_values(values, expected, "colatitude")?;
        }
        let code = self.code();
        if matches!(code, 5 | 6 | 9) && !includes_north_pole
            || matches!(code, 7 | 8 | 9) && !includes_south_pole
            || includes_north_pole && matches!(code, 3 | 4 | 8)
            || includes_south_pole && matches!(code, 2 | 3 | 6)
        {
            return Err(CurvilinearPdeError::UnsupportedBoundaryCombination {
                routine: "spherical FISHPACK driver",
            });
        }
        Ok(())
    }
}

/// An owned Helmholtz problem on the surface of the unit sphere.
///
/// `HWSSSP` approximates `(sin(theta))^-1 (sin(theta) u_theta)_theta +
/// (sin(theta))^-2 u_phi_phi + lambda u = rhs`.  The first grid coordinate is
/// colatitude and the second is longitude.
#[derive(Clone, Debug, PartialEq)]
pub struct SphereSurfaceHelmholtz2d {
    colatitude: ColatitudeAxis,
    longitude: LongitudeAxis,
    coefficient: f32,
    rhs: FishpackGrid2,
    colatitude_boundary: ColatitudeBoundary,
    longitude_boundary: CoordinateBoundary,
}

impl SphereSurfaceHelmholtz2d {
    /// Validates and owns a unit-sphere surface problem.
    pub fn new(
        colatitude: ColatitudeAxis,
        longitude: LongitudeAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        colatitude_boundary: ColatitudeBoundary,
        longitude_boundary: CoordinateBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        if !coefficient.is_finite() {
            return Err(CurvilinearPdeError::NonFiniteInput {
                field: "Helmholtz coefficient",
            });
        }
        let first_nodes = colatitude.nodes()?;
        let second_nodes = longitude.nodes()?;
        validate_grid(&rhs, first_nodes, second_nodes)?;
        colatitude_boundary.validate(colatitude, second_nodes)?;
        longitude_boundary.validate(first_nodes)?;
        if colatitude_boundary.unspecified_pole()
            && matches!(
                longitude_boundary,
                CoordinateBoundary::Dirichlet { .. }
                    | CoordinateBoundary::DirichletNeumann { .. }
                    | CoordinateBoundary::NeumannDirichlet { .. }
            )
        {
            return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HWSSSP" });
        }
        if matches!(longitude_boundary, CoordinateBoundary::Periodic)
            && (0..first_nodes).any(|first| rhs.get(first, 0) != rhs.get(first, second_nodes - 1))
        {
            return Err(CurvilinearPdeError::InconsistentPeriodicRightHandSide);
        }
        Ok(Self {
            colatitude,
            longitude,
            coefficient,
            rhs,
            colatitude_boundary,
            longitude_boundary,
        })
    }

    /// Solves through the reviewed `HWSSSP` driver.
    pub fn solve(self) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        let first_nodes = self.colatitude.nodes()?;
        let second_nodes = self.longitude.nodes()?;
        let mut m = as_fortran(self.colatitude.panels())?;
        let mut n = as_fortran(self.longitude.panels())?;
        let mut idimf = as_fortran(first_nodes)?;
        let mut mbdcnd = self.colatitude_boundary.code();
        let mut nbdcnd = self.longitude_boundary.code();
        let mut ts = self.colatitude.lower();
        let mut tf = self.colatitude.upper();
        let mut ps = self.longitude.lower();
        let mut pf = self.longitude.upper();
        let mut coefficient = self.coefficient;
        let mut values = self.rhs.values().to_vec();
        apply_surface_edges(
            first_nodes,
            second_nodes,
            &mut values,
            &self.colatitude_boundary,
            &self.longitude_boundary,
        );
        let dummy = [0.0_f32];
        let bdts = self
            .colatitude_boundary
            .lower_derivative()
            .unwrap_or(&dummy);
        let bdtf = self
            .colatitude_boundary
            .upper_derivative()
            .unwrap_or(&dummy);
        let bdps = self.longitude_boundary.lower_derivative().unwrap_or(&dummy);
        let bdpf = self.longitude_boundary.upper_derivative().unwrap_or(&dummy);
        let workspace_length = surface_workspace_len(first_nodes, second_nodes)?;
        let mut workspace = zeroed(workspace_length)?;
        let mut perturbation = 0.0;
        let mut ierror = 0;
        let _native = lock_native();
        // SAFETY: owned buffers match the reviewed `HWSSSP` dimensions and
        // source-defined boundary data extents, and no pointer escapes this
        // serialized call.
        unsafe {
            slatec_sys::pde::fishpack::hwsssp(
                &mut ts,
                &mut tf,
                &mut m,
                &mut mbdcnd,
                bdts.as_ptr().cast_mut(),
                bdtf.as_ptr().cast_mut(),
                &mut ps,
                &mut pf,
                &mut n,
                &mut nbdcnd,
                bdps.as_ptr().cast_mut(),
                bdpf.as_ptr().cast_mut(),
                &mut coefficient,
                values.as_mut_ptr(),
                &mut idimf,
                &mut perturbation,
                &mut ierror,
                workspace.as_mut_ptr(),
            );
        }
        spherical_result(
            "HWSSSP",
            first_nodes,
            second_nodes,
            values,
            perturbation,
            ierror,
            workspace,
            9,
        )
    }
}

/// An owned staggered-grid Helmholtz problem on the surface of the unit sphere.
///
/// `HSTSSP` samples both coordinates at open-interval staggered points.  Its
/// right-hand side therefore contains only unknown-point values; all endpoint
/// values and increasing-coordinate derivatives are supplied by the boundary
/// objects rather than stored in [`FishpackGrid2`].
#[derive(Clone, Debug, PartialEq)]
pub struct StaggeredSphereSurfaceHelmholtz2d {
    colatitude: StaggeredColatitudeAxis,
    longitude: StaggeredLongitudeAxis,
    coefficient: f32,
    rhs: FishpackGrid2,
    colatitude_boundary: ColatitudeBoundary,
    longitude_boundary: CoordinateBoundary,
}

impl StaggeredSphereSurfaceHelmholtz2d {
    /// Validates and owns a staggered unit-sphere surface problem.
    pub fn new(
        colatitude: StaggeredColatitudeAxis,
        longitude: StaggeredLongitudeAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        colatitude_boundary: ColatitudeBoundary,
        longitude_boundary: CoordinateBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        if !coefficient.is_finite() {
            return Err(CurvilinearPdeError::NonFiniteInput {
                field: "Helmholtz coefficient",
            });
        }
        let first_points = colatitude.points();
        let second_points = longitude.points();
        validate_grid(&rhs, first_points, second_points)?;
        colatitude_boundary.validate_staggered(colatitude, second_points)?;
        longitude_boundary.validate(first_points)?;
        if colatitude_boundary.unspecified_pole()
            && matches!(
                longitude_boundary,
                CoordinateBoundary::Dirichlet { .. }
                    | CoordinateBoundary::DirichletNeumann { .. }
                    | CoordinateBoundary::NeumannDirichlet { .. }
            )
        {
            return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HSTSSP" });
        }
        Ok(Self {
            colatitude,
            longitude,
            coefficient,
            rhs,
            colatitude_boundary,
            longitude_boundary,
        })
    }

    /// Solves through the reviewed single-precision `HSTSSP` driver.
    pub fn solve(self) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        let first_points = self.colatitude.points();
        let second_points = self.longitude.points();
        let mut m = as_fortran(first_points)?;
        let mut n = as_fortran(second_points)?;
        let mut idimf = as_fortran(first_points)?;
        let mut mbdcnd = self.colatitude_boundary.code();
        let mut nbdcnd = self.longitude_boundary.code();
        let mut a = self.colatitude.lower();
        let mut b = self.colatitude.upper();
        let mut c = self.longitude.lower();
        let mut d = self.longitude.upper();
        let mut coefficient = self.coefficient;
        let mut values = self.rhs.values().to_vec();
        let dummy = [0.0_f32];
        let bda = self.colatitude_boundary.lower_data().unwrap_or(&dummy);
        let bdb = self.colatitude_boundary.upper_data().unwrap_or(&dummy);
        let bdc = self.longitude_boundary.lower_data().unwrap_or(&dummy);
        let bdd = self.longitude_boundary.upper_data().unwrap_or(&dummy);
        let mut workspace = zeroed(staggered_surface_workspace_len(
            first_points,
            second_points,
        )?)?;
        let mut perturbation = 0.0;
        let mut ierror = 0;
        let _native = lock_native();
        // SAFETY: each scalar and private buffer matches the source-reviewed
        // `HSTSSP` contract.  The owned `F(IDIMF, N)` storage has
        // `IDIMF == M`, boundary arrays have the documented opposite-axis
        // lengths when used, and every pointer remains live for this
        // serialized native call.
        unsafe {
            slatec_sys::pde::fishpack::hstssp(
                &mut a,
                &mut b,
                &mut m,
                &mut mbdcnd,
                bda.as_ptr().cast_mut(),
                bdb.as_ptr().cast_mut(),
                &mut c,
                &mut d,
                &mut n,
                &mut nbdcnd,
                bdc.as_ptr().cast_mut(),
                bdd.as_ptr().cast_mut(),
                &mut coefficient,
                values.as_mut_ptr(),
                &mut idimf,
                &mut perturbation,
                &mut ierror,
                workspace.as_mut_ptr(),
            );
        }
        spherical_result(
            "HSTSSP",
            first_points,
            second_points,
            values,
            perturbation,
            ierror,
            workspace,
            14,
        )
    }
}

fn validate_colatitude(lower: f32, upper: f32, panels: usize) -> Result<(), CurvilinearPdeError> {
    if !lower.is_finite()
        || !upper.is_finite()
        || lower < 0.0
        || upper > SLATEC_PI
        || lower >= upper
    {
        return Err(CurvilinearPdeError::InvalidAxis);
    }
    if panels < 5 {
        return Err(CurvilinearPdeError::GridTooSmall { panels, minimum: 5 });
    }
    Ok(())
}

fn validate_staggered_colatitude(
    lower: f32,
    upper: f32,
    points: usize,
) -> Result<(), CurvilinearPdeError> {
    if !lower.is_finite()
        || !upper.is_finite()
        || lower < 0.0
        || upper > SLATEC_PI
        || lower >= upper
    {
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

fn validate_grid(
    rhs: &FishpackGrid2,
    first_nodes: usize,
    second_nodes: usize,
) -> Result<(), CurvilinearPdeError> {
    if rhs.first_nodes() != first_nodes || rhs.second_nodes() != second_nodes {
        return Err(CurvilinearPdeError::InvalidGridShape {
            expected: first_nodes
                .checked_mul(second_nodes)
                .ok_or(CurvilinearPdeError::DimensionOverflow)?,
            actual: rhs.values().len(),
        });
    }
    if rhs.values().iter().any(|value| !value.is_finite()) {
        return Err(CurvilinearPdeError::NonFiniteInput {
            field: "right-hand side",
        });
    }
    Ok(())
}

fn validate_values(
    values: &[f32],
    expected: usize,
    axis: &'static str,
) -> Result<(), CurvilinearPdeError> {
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
    Ok(())
}

fn apply_surface_edges(
    first_nodes: usize,
    second_nodes: usize,
    values: &mut [f32],
    first: &ColatitudeBoundary,
    second: &CoordinateBoundary,
) {
    apply_first_edges(
        first_nodes,
        second_nodes,
        values,
        first.lower_value(),
        first.upper_value(),
    );
    apply_second_edges(
        first_nodes,
        second_nodes,
        values,
        second.lower_value(),
        second.upper_value(),
    );
}

fn apply_first_edges(
    first_nodes: usize,
    second_nodes: usize,
    values: &mut [f32],
    lower: Option<&[f32]>,
    upper: Option<&[f32]>,
) {
    if let Some(lower) = lower {
        for second in 0..second_nodes {
            values[second * first_nodes] = lower[second];
        }
    }
    if let Some(upper) = upper {
        for second in 0..second_nodes {
            values[second * first_nodes + first_nodes - 1] = upper[second];
        }
    }
}

fn apply_second_edges(
    first_nodes: usize,
    second_nodes: usize,
    values: &mut [f32],
    lower: Option<&[f32]>,
    upper: Option<&[f32]>,
) {
    if let Some(lower) = lower {
        values[..first_nodes].copy_from_slice(lower);
    }
    if let Some(upper) = upper {
        values[(second_nodes - 1) * first_nodes..].copy_from_slice(upper);
    }
}

fn as_fortran(value: usize) -> Result<FortranInteger, CurvilinearPdeError> {
    FortranInteger::try_from(value).map_err(|_| CurvilinearPdeError::DimensionOverflow)
}

fn surface_workspace_len(
    first_nodes: usize,
    second_nodes: usize,
) -> Result<usize, CurvilinearPdeError> {
    let log2_second = (usize::BITS - 1 - second_nodes.leading_zeros()) as usize;
    second_nodes
        .checked_mul(4)
        .and_then(|value| {
            log2_second
                .checked_add(16)
                .and_then(|factor| factor.checked_mul(first_nodes))
                .and_then(|other| value.checked_add(other))
        })
        .ok_or(CurvilinearPdeError::DimensionOverflow)
}

fn staggered_surface_workspace_len(
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

fn zeroed(length: usize) -> Result<Vec<f32>, CurvilinearPdeError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| CurvilinearPdeError::AllocationFailed)?;
    values.resize(length, 0.0);
    Ok(values)
}

fn spherical_result(
    routine: &'static str,
    first_nodes: usize,
    second_nodes: usize,
    values: Vec<f32>,
    perturbation: f32,
    ierror: FortranInteger,
    workspace: Vec<f32>,
    warning_code: FortranInteger,
) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
    if ierror != 0 && ierror != warning_code {
        return Err(CurvilinearPdeError::NativeFailure {
            routine,
            code: ierror,
        });
    }
    if !perturbation.is_finite() {
        return Err(CurvilinearPdeError::NativeFailure {
            routine,
            code: ierror,
        });
    }
    let reported = workspace[0];
    if !reported.is_finite() || reported < 1.0 || reported > workspace.len() as f32 {
        return Err(CurvilinearPdeError::InconsistentNativeWorkspace {
            reported,
            allocated: workspace.len(),
        });
    }
    let values = FishpackGrid2::new(first_nodes, second_nodes, values)?;
    Ok(CurvilinearPdeSolution::from_native(
        values,
        perturbation,
        if ierror == 0 {
            NativeCurvilinearPdeStatus::Success
        } else {
            NativeCurvilinearPdeStatus::CoefficientWarning { code: ierror }
        },
    ))
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::super::fishpack_cylindrical_polar::{CoordinateBoundary, FishpackGrid2};
    use super::{ColatitudeAxis, ColatitudeBoundary, LongitudeAxis, SphereSurfaceHelmholtz2d};

    #[cfg(feature = "fishpack-spherical-native-tests")]
    use super::{
        StaggeredColatitudeAxis, StaggeredLongitudeAxis, StaggeredSphereSurfaceHelmholtz2d,
    };

    #[cfg(feature = "fishpack-spherical-native-tests")]
    #[test]
    fn native_surface_constant_solution_is_manufactured() {
        let colatitude = ColatitudeAxis::new(0.5, 2.5, 6).unwrap();
        let longitude = LongitudeAxis::new(0.0, 1.0, 6).unwrap();
        let surface = SphereSurfaceHelmholtz2d::new(
            colatitude,
            longitude,
            0.0,
            FishpackGrid2::zeros(7, 7).unwrap(),
            ColatitudeBoundary::Dirichlet {
                lower: vec![1.0; 7],
                upper: vec![1.0; 7],
            },
            CoordinateBoundary::Periodic,
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            surface
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 1.0e-4)
        );
    }

    #[cfg(feature = "fishpack-spherical-native-tests")]
    #[test]
    fn native_full_sphere_uses_source_accurate_pole_endpoints() {
        let surface = SphereSurfaceHelmholtz2d::new(
            ColatitudeAxis::full_sphere(6).unwrap(),
            LongitudeAxis::full_circle(6).unwrap(),
            0.0,
            FishpackGrid2::zeros(7, 7).unwrap(),
            ColatitudeBoundary::BothPoles,
            CoordinateBoundary::Periodic,
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            surface
                .values()
                .values()
                .iter()
                .all(|value| value.abs() < 1.0e-5)
        );
    }

    #[cfg(feature = "fishpack-spherical-native-tests")]
    #[test]
    fn native_staggered_surface_constant_solution_is_manufactured() {
        let surface = StaggeredSphereSurfaceHelmholtz2d::new(
            StaggeredColatitudeAxis::new(0.5, 2.5, 6).unwrap(),
            StaggeredLongitudeAxis::new(0.0, 1.0, 6).unwrap(),
            0.0,
            FishpackGrid2::zeros(6, 6).unwrap(),
            ColatitudeBoundary::Dirichlet {
                lower: vec![1.0; 6],
                upper: vec![1.0; 6],
            },
            CoordinateBoundary::Periodic,
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            surface
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 1.0e-4)
        );
    }

    #[cfg(feature = "fishpack-spherical-native-tests")]
    #[test]
    fn native_surface_coefficient_pole_modes_and_singular_perturbation_are_verified() {
        let coefficient = 1.25;
        let north = SphereSurfaceHelmholtz2d::new(
            ColatitudeAxis::new(0.0, 2.0, 12).unwrap(),
            LongitudeAxis::full_circle(12).unwrap(),
            coefficient,
            FishpackGrid2::new(13, 13, vec![coefficient; 169]).unwrap(),
            ColatitudeBoundary::NorthPoleDirichlet {
                upper: vec![1.0; 13],
            },
            CoordinateBoundary::Periodic,
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            north
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 3.0e-4)
        );

        let south = SphereSurfaceHelmholtz2d::new(
            ColatitudeAxis::new(1.0, core::f32::consts::PI, 12).unwrap(),
            LongitudeAxis::full_circle(12).unwrap(),
            coefficient,
            FishpackGrid2::new(13, 13, vec![coefficient; 169]).unwrap(),
            ColatitudeBoundary::SouthPoleDirichlet {
                lower: vec![1.0; 13],
            },
            CoordinateBoundary::Periodic,
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            south
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 3.0e-4)
        );

        let singular = SphereSurfaceHelmholtz2d::new(
            ColatitudeAxis::full_sphere(12).unwrap(),
            LongitudeAxis::full_circle(12).unwrap(),
            0.0,
            FishpackGrid2::new(13, 13, vec![0.75; 169]).unwrap(),
            ColatitudeBoundary::BothPoles,
            CoordinateBoundary::Periodic,
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!((singular.perturbation() - 0.75).abs() < 3.0e-4);
    }

    #[test]
    fn full_domain_constructors_keep_source_pi_boundary_exact() {
        let colatitude = ColatitudeAxis::full_sphere(6).unwrap();
        let longitude = LongitudeAxis::full_circle(6).unwrap();
        assert_eq!(colatitude.lower(), 0.0);
        assert!(colatitude.upper().is_finite());
        assert_eq!(longitude.lower(), 0.0);
        assert!(longitude.upper() > colatitude.upper());
    }
}
