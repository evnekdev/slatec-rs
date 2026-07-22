//! Checked axisymmetric spherical FISHPACK workflows.
//!
//! These are deliberately separate from the unit-sphere surface workflows:
//! `HWSCSP` and `HSTCSP` retain a radial coordinate and solve the
//! axisymmetric three-dimensional equation after a longitude Fourier mode has
//! been selected.  Calls use fresh owned workspaces and are process-serialized
//! because the underlying `BLKTRI` implementation has legacy saved state.

use alloc::vec::Vec;
use core::convert::TryFrom;

use slatec_sys::FortranInteger;

use crate::runtime::lock_native;

use super::fishpack_cylindrical_polar::{
    CurvilinearPdeError, CurvilinearPdeSolution, FishpackGrid2, NativeCurvilinearPdeStatus,
    RadialAxis, RadialBoundary, StaggeredRadialAxis,
};
use super::fishpack_spherical::{
    ColatitudeAxis, ColatitudeBoundary, SLATEC_PI, StaggeredColatitudeAxis,
};

/// An owned centered-grid axisymmetric spherical Helmholtz problem.
///
/// `HWSCSP` approximates
/// `(r^-2)(r^2 u_r)_r + (r^2 sin(theta))^-1 (sin(theta) u_theta)_theta +
/// lambda/(r sin(theta))^2 u = rhs` on a colatitude-by-radius grid.  There is
/// no longitude coordinate: this is not the unit-sphere surface solver.
#[derive(Clone, Debug, PartialEq)]
pub struct AxisymmetricSphericalHelmholtz2d {
    colatitude: ColatitudeAxis,
    radius: RadialAxis,
    coefficient: f32,
    rhs: FishpackGrid2,
    colatitude_boundary: ColatitudeBoundary,
    radial_boundary: RadialBoundary,
}

impl AxisymmetricSphericalHelmholtz2d {
    /// Validates and owns a centered-grid axisymmetric spherical problem.
    ///
    /// Colatitude derivatives use increasing colatitude; radial derivatives
    /// use increasing radius.  Pole and origin variants represent the
    /// source-defined regularity conditions rather than arbitrary values at a
    /// coordinate singularity.
    pub fn new(
        colatitude: ColatitudeAxis,
        radius: RadialAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        colatitude_boundary: ColatitudeBoundary,
        radial_boundary: RadialBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        validate_centered(
            colatitude,
            radius,
            coefficient,
            &rhs,
            &colatitude_boundary,
            &radial_boundary,
        )?;
        Ok(Self {
            colatitude,
            radius,
            coefficient,
            rhs,
            colatitude_boundary,
            radial_boundary,
        })
    }

    /// Solves through the reviewed single-precision `HWSCSP` driver.
    pub fn solve(self) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        let first_nodes = self.colatitude.nodes()?;
        let second_nodes = self.radius.nodes()?;
        let mut intl = 0;
        let mut m = as_fortran(self.colatitude.panels())?;
        let mut n = as_fortran(self.radius.panels())?;
        let mut idimf = as_fortran(first_nodes)?;
        let mut mbdcnd = self.colatitude_boundary.code();
        let mut nbdcnd = self.radial_boundary.code();
        let mut ts = self.colatitude.lower();
        let mut tf = self.colatitude.upper();
        let mut rs = self.radius.lower();
        let mut rf = self.radius.upper();
        let mut coefficient = self.coefficient;
        let mut values = self.rhs.values().to_vec();
        apply_centered_edges(
            first_nodes,
            second_nodes,
            &mut values,
            &self.colatitude_boundary,
            &self.radial_boundary,
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
        let bdrs = self.radial_boundary.lower_derivative().unwrap_or(&dummy);
        let bdrf = self.radial_boundary.upper_derivative().unwrap_or(&dummy);
        let workspace_length = centered_workspace_len(
            self.colatitude.panels(),
            self.radius.panels(),
            self.radial_boundary.code(),
        )?;
        let mut workspace = zeroed(workspace_length)?;
        let mut perturbation = 0.0;
        let mut ierror = 0;
        let _native = lock_native();
        // SAFETY: all scalar, grid, boundary, and workspace buffers are
        // owned, non-null, source-sized, and live for this serialized
        // `HWSCSP` call.  The routine does not retain any caller pointer.
        unsafe {
            slatec_sys::pde::fishpack::hwscsp(
                &mut intl,
                &mut ts,
                &mut tf,
                &mut m,
                &mut mbdcnd,
                bdts.as_ptr().cast_mut(),
                bdtf.as_ptr().cast_mut(),
                &mut rs,
                &mut rf,
                &mut n,
                &mut nbdcnd,
                bdrs.as_ptr().cast_mut(),
                bdrf.as_ptr().cast_mut(),
                &mut coefficient,
                values.as_mut_ptr(),
                &mut idimf,
                &mut perturbation,
                &mut ierror,
                workspace.as_mut_ptr(),
            );
        }
        native_result(
            "HWSCSP",
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

/// An owned staggered-grid axisymmetric spherical Helmholtz problem.
///
/// `HSTCSP` samples colatitude and radius at open-interval points.  Endpoint
/// data remain private owned buffers in the boundary objects; the RHS grid is
/// therefore exactly `colatitude.points() × radius.points()`.
#[derive(Clone, Debug, PartialEq)]
pub struct StaggeredAxisymmetricSphericalHelmholtz2d {
    colatitude: StaggeredColatitudeAxis,
    radius: StaggeredRadialAxis,
    coefficient: f32,
    rhs: FishpackGrid2,
    colatitude_boundary: ColatitudeBoundary,
    radial_boundary: RadialBoundary,
}

impl StaggeredAxisymmetricSphericalHelmholtz2d {
    /// Validates and owns a staggered-grid axisymmetric spherical problem.
    pub fn new(
        colatitude: StaggeredColatitudeAxis,
        radius: StaggeredRadialAxis,
        coefficient: f32,
        rhs: FishpackGrid2,
        colatitude_boundary: ColatitudeBoundary,
        radial_boundary: RadialBoundary,
    ) -> Result<Self, CurvilinearPdeError> {
        validate_staggered(
            colatitude,
            radius,
            coefficient,
            &rhs,
            &colatitude_boundary,
            &radial_boundary,
        )?;
        Ok(Self {
            colatitude,
            radius,
            coefficient,
            rhs,
            colatitude_boundary,
            radial_boundary,
        })
    }

    /// Solves through the reviewed single-precision `HSTCSP` driver.
    pub fn solve(self) -> Result<CurvilinearPdeSolution, CurvilinearPdeError> {
        let first_points = self.colatitude.points();
        let second_points = self.radius.points();
        let mut intl = 0;
        let mut m = as_fortran(first_points)?;
        let mut n = as_fortran(second_points)?;
        let mut idimf = as_fortran(first_points)?;
        let mut mbdcnd = self.colatitude_boundary.code();
        let mut nbdcnd = self.radial_boundary.code();
        let mut a = self.colatitude.lower();
        let mut b = self.colatitude.upper();
        let mut c = self.radius.lower();
        let mut d = self.radius.upper();
        let mut coefficient = self.coefficient;
        let mut values = self.rhs.values().to_vec();
        let dummy = [0.0_f32];
        let bda = self.colatitude_boundary.lower_data().unwrap_or(&dummy);
        let bdb = self.colatitude_boundary.upper_data().unwrap_or(&dummy);
        let bdc = self.radial_boundary.lower_data().unwrap_or(&dummy);
        let bdd = self.radial_boundary.upper_data().unwrap_or(&dummy);
        let workspace_length = staggered_workspace_len(first_points, second_points)?;
        let mut workspace = zeroed(workspace_length)?;
        let mut perturbation = 0.0;
        let mut ierror = 0;
        let _native = lock_native();
        // SAFETY: the owned arrays satisfy HSTCSP's `BDA(N)`, `BDB(N)`,
        // `BDC(M)`, `BDD(M)`, `F(IDIMF,N)`, and workspace contracts.  Native
        // state is serialized and no pointer escapes this synchronous call.
        unsafe {
            slatec_sys::pde::fishpack::hstcsp(
                &mut intl,
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
        native_result(
            "HSTCSP",
            first_points,
            second_points,
            values,
            perturbation,
            ierror,
            workspace,
            10,
        )
    }
}

fn validate_centered(
    colatitude: ColatitudeAxis,
    radius: RadialAxis,
    coefficient: f32,
    rhs: &FishpackGrid2,
    colatitude_boundary: &ColatitudeBoundary,
    radial_boundary: &RadialBoundary,
) -> Result<(), CurvilinearPdeError> {
    if !coefficient.is_finite() {
        return Err(CurvilinearPdeError::NonFiniteInput {
            field: "Helmholtz coefficient",
        });
    }
    if radius.panels() < 5 {
        return Err(CurvilinearPdeError::GridTooSmall {
            panels: radius.panels(),
            minimum: 5,
        });
    }
    let first_nodes = colatitude.nodes()?;
    let second_nodes = radius.nodes()?;
    validate_grid(rhs, first_nodes, second_nodes)?;
    colatitude_boundary.validate(colatitude, second_nodes)?;
    radial_boundary.validate(first_nodes)?;
    validate_centered_combination(
        colatitude,
        radius,
        coefficient,
        colatitude_boundary,
        radial_boundary,
    )?;
    validate_corners(
        colatitude_boundary,
        radial_boundary,
        first_nodes,
        second_nodes,
    )
}

fn validate_staggered(
    colatitude: StaggeredColatitudeAxis,
    radius: StaggeredRadialAxis,
    coefficient: f32,
    rhs: &FishpackGrid2,
    colatitude_boundary: &ColatitudeBoundary,
    radial_boundary: &RadialBoundary,
) -> Result<(), CurvilinearPdeError> {
    if !coefficient.is_finite() {
        return Err(CurvilinearPdeError::NonFiniteInput {
            field: "Helmholtz coefficient",
        });
    }
    if colatitude.points() < 5 {
        return Err(CurvilinearPdeError::GridTooSmall {
            panels: colatitude.points(),
            minimum: 5,
        });
    }
    if radius.points() < 5 {
        return Err(CurvilinearPdeError::GridTooSmall {
            panels: radius.points(),
            minimum: 5,
        });
    }
    validate_grid(rhs, colatitude.points(), radius.points())?;
    colatitude_boundary.validate_staggered(colatitude, radius.points())?;
    radial_boundary.validate(colatitude.points())?;
    let colatitude_code = colatitude_boundary.code();
    if colatitude.includes_north_pole() && !matches!(colatitude_code, 5 | 6 | 9)
        || colatitude.includes_south_pole() && !matches!(colatitude_code, 7..=9)
    {
        return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HSTCSP" });
    }
    if radial_boundary.is_axis() && radius.lower() != 0.0 {
        return Err(CurvilinearPdeError::AxisBoundaryRequiresZeroRadius);
    }
    if radial_boundary.is_axis() && matches!(colatitude_code, 1 | 2 | 4 | 5 | 7) {
        return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HSTCSP" });
    }
    if radial_boundary.is_axis() && coefficient != 0.0 {
        return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HSTCSP" });
    }
    Ok(())
}

fn validate_centered_combination(
    colatitude: ColatitudeAxis,
    radius: RadialAxis,
    coefficient: f32,
    colatitude_boundary: &ColatitudeBoundary,
    radial_boundary: &RadialBoundary,
) -> Result<(), CurvilinearPdeError> {
    let colatitude_code = colatitude_boundary.code();
    if colatitude_boundary.unspecified_pole() && coefficient != 0.0 {
        return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HWSCSP" });
    }
    if radial_boundary.is_axis() && radius.lower() != 0.0 {
        return Err(CurvilinearPdeError::AxisBoundaryRequiresZeroRadius);
    }
    if radial_boundary.is_axis() && coefficient != 0.0 {
        return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HWSCSP" });
    }
    if radial_boundary.is_axis() && matches!(colatitude_code, 1 | 2 | 4 | 5 | 7) {
        return Err(CurvilinearPdeError::UnsupportedBoundaryCombination { routine: "HWSCSP" });
    }
    // The two pole tests above are already enforced by ColatitudeBoundary;
    // keep this assertion explicit because HWSCSP compares against PIMACH.
    if colatitude.includes_south_pole() && colatitude.upper() != SLATEC_PI {
        return Err(CurvilinearPdeError::InvalidAxis);
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

fn validate_corners(
    colatitude: &ColatitudeBoundary,
    radial: &RadialBoundary,
    first_nodes: usize,
    second_nodes: usize,
) -> Result<(), CurvilinearPdeError> {
    let theta_lower = colatitude.lower_value();
    let theta_upper = colatitude.upper_value();
    let radius_lower = radial.lower_value();
    let radius_upper = radial.upper_value();
    if theta_lower
        .zip(radius_lower)
        .is_some_and(|(theta, radius)| theta[0] != radius[0])
        || theta_lower
            .zip(radius_upper)
            .is_some_and(|(theta, radius)| theta[second_nodes - 1] != radius[0])
        || theta_upper
            .zip(radius_lower)
            .is_some_and(|(theta, radius)| theta[0] != radius[first_nodes - 1])
        || theta_upper
            .zip(radius_upper)
            .is_some_and(|(theta, radius)| theta[second_nodes - 1] != radius[first_nodes - 1])
    {
        return Err(CurvilinearPdeError::InconsistentCornerValues);
    }
    Ok(())
}

fn apply_centered_edges(
    first_nodes: usize,
    second_nodes: usize,
    values: &mut [f32],
    colatitude: &ColatitudeBoundary,
    radial: &RadialBoundary,
) {
    if let Some(lower) = colatitude.lower_value() {
        for second in 0..second_nodes {
            values[second * first_nodes] = lower[second];
        }
    }
    if let Some(upper) = colatitude.upper_value() {
        for second in 0..second_nodes {
            values[second * first_nodes + first_nodes - 1] = upper[second];
        }
    }
    if let Some(lower) = radial.lower_value() {
        values[..first_nodes].copy_from_slice(lower);
    }
    if let Some(upper) = radial.upper_value() {
        values[(second_nodes - 1) * first_nodes..].copy_from_slice(upper);
    }
}

fn as_fortran(value: usize) -> Result<FortranInteger, CurvilinearPdeError> {
    FortranInteger::try_from(value).map_err(|_| CurvilinearPdeError::DimensionOverflow)
}

fn centered_workspace_len(
    colatitude_panels: usize,
    radial_panels: usize,
    radial_boundary: FortranInteger,
) -> Result<usize, CurvilinearPdeError> {
    let radial_unknowns = match radial_boundary {
        1 | 5 => radial_panels.checked_sub(1),
        2 | 4 | 6 => Some(radial_panels),
        3 => radial_panels.checked_add(1),
        _ => None,
    }
    .ok_or(CurvilinearPdeError::DimensionOverflow)?;
    let k = (usize::BITS - radial_unknowns.leading_zeros()) as usize;
    let l = 1_usize
        .checked_shl((k + 1) as u32)
        .ok_or(CurvilinearPdeError::DimensionOverflow)?;
    k.checked_sub(2)
        .and_then(|value| value.checked_mul(l))
        .and_then(|value| value.checked_add(k))
        .and_then(|value| {
            colatitude_panels
                .checked_add(radial_panels)
                .and_then(|sum| sum.checked_mul(5))
                .and_then(|other| value.checked_add(other))
        })
        .and_then(|value| {
            radial_panels
                .checked_mul(2)
                .zip(colatitude_panels.checked_mul(6))
                .map(|(radial, colatitude)| radial.max(colatitude))
                .and_then(|other| value.checked_add(other))
        })
        .and_then(|value| value.checked_add(23))
        .ok_or(CurvilinearPdeError::DimensionOverflow)
}

fn staggered_workspace_len(
    colatitude_points: usize,
    radial_points: usize,
) -> Result<usize, CurvilinearPdeError> {
    let k = (usize::BITS - radial_points.leading_zeros()) as usize;
    let l = 1_usize
        .checked_shl((k + 1) as u32)
        .ok_or(CurvilinearPdeError::DimensionOverflow)?;
    k.checked_sub(2)
        .and_then(|value| value.checked_mul(l))
        .and_then(|value| value.checked_add(k))
        .and_then(|value| {
            radial_points
                .checked_mul(2)
                .zip(colatitude_points.checked_mul(6))
                .map(|(radial, colatitude)| radial.max(colatitude))
                .and_then(|other| value.checked_add(other))
        })
        .and_then(|value| {
            radial_points
                .checked_add(colatitude_points)
                .and_then(|sum| sum.checked_mul(4))
                .and_then(|other| value.checked_add(other))
        })
        .and_then(|value| value.checked_add(5))
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

#[allow(clippy::too_many_arguments)]
fn native_result(
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
    Ok(CurvilinearPdeSolution::from_native(
        FishpackGrid2::new(first_nodes, second_nodes, values)?,
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

    use super::super::fishpack_cylindrical_polar::{FishpackGrid2, RadialAxis, RadialBoundary};
    use super::super::fishpack_spherical::{ColatitudeAxis, ColatitudeBoundary};
    use super::AxisymmetricSphericalHelmholtz2d;

    #[cfg(feature = "fishpack-spherical-native-tests")]
    use super::{
        StaggeredAxisymmetricSphericalHelmholtz2d, StaggeredColatitudeAxis, StaggeredRadialAxis,
    };

    #[cfg(feature = "fishpack-spherical-native-tests")]
    #[test]
    fn native_axisymmetric_constant_solution_is_manufactured() {
        let problem = AxisymmetricSphericalHelmholtz2d::new(
            ColatitudeAxis::full_sphere(6).unwrap(),
            RadialAxis::new(1.0, 2.0, 6).unwrap(),
            0.0,
            FishpackGrid2::zeros(7, 7).unwrap(),
            ColatitudeBoundary::BothPoles,
            RadialBoundary::Dirichlet {
                lower: vec![1.0; 7],
                upper: vec![1.0; 7],
            },
        )
        .unwrap();
        let result = problem.solve().unwrap();
        assert!(
            result
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 5.0e-4)
        );
    }

    #[cfg(feature = "fishpack-spherical-native-tests")]
    #[test]
    fn native_staggered_axisymmetric_constant_solution_is_manufactured() {
        let problem = StaggeredAxisymmetricSphericalHelmholtz2d::new(
            StaggeredColatitudeAxis::full_sphere(6).unwrap(),
            StaggeredRadialAxis::new(1.0, 2.0, 6).unwrap(),
            0.0,
            FishpackGrid2::zeros(6, 6).unwrap(),
            ColatitudeBoundary::BothPoles,
            RadialBoundary::Dirichlet {
                lower: vec![1.0; 6],
                upper: vec![1.0; 6],
            },
        )
        .unwrap();
        let result = problem.solve().unwrap();
        assert!(
            result
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 5.0e-4)
        );
    }

    #[cfg(feature = "fishpack-spherical-native-tests")]
    #[test]
    fn native_axisymmetric_coefficient_origin_and_singular_modes_are_verified() {
        let coefficient = 0.5;
        let mut coefficient_rhs = vec![0.0; 121];
        for radial in 0..=10 {
            let radius = 1.0 + radial as f32 / 10.0;
            for colatitude in 0..=10 {
                let theta = 0.5 + 2.0 * colatitude as f32 / 10.0;
                coefficient_rhs[colatitude + 11 * radial] =
                    coefficient / (radius * theta.sin()).powi(2);
            }
        }
        let coefficient_problem = AxisymmetricSphericalHelmholtz2d::new(
            ColatitudeAxis::new(0.5, 2.5, 10).unwrap(),
            RadialAxis::new(1.0, 2.0, 10).unwrap(),
            coefficient,
            FishpackGrid2::new(11, 11, coefficient_rhs).unwrap(),
            ColatitudeBoundary::Dirichlet {
                lower: vec![1.0; 11],
                upper: vec![1.0; 11],
            },
            RadialBoundary::Dirichlet {
                lower: vec![1.0; 11],
                upper: vec![1.0; 11],
            },
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            coefficient_problem
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 6.0e-4)
        );

        let origin_problem = AxisymmetricSphericalHelmholtz2d::new(
            ColatitudeAxis::full_sphere(10).unwrap(),
            RadialAxis::new(0.0, 2.0, 10).unwrap(),
            0.0,
            FishpackGrid2::zeros(11, 11).unwrap(),
            ColatitudeBoundary::BothPoles,
            RadialBoundary::AxisDirichlet {
                outer: vec![1.0; 11],
            },
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!(
            origin_problem
                .values()
                .values()
                .iter()
                .all(|value| (*value - 1.0).abs() < 8.0e-4)
        );

        let singular = AxisymmetricSphericalHelmholtz2d::new(
            ColatitudeAxis::full_sphere(8).unwrap(),
            RadialAxis::new(1.0, 2.0, 8).unwrap(),
            0.0,
            FishpackGrid2::new(9, 9, vec![1.5; 81]).unwrap(),
            ColatitudeBoundary::BothPoles,
            RadialBoundary::Neumann {
                lower_derivative: vec![0.0; 9],
                upper_derivative: vec![0.0; 9],
            },
        )
        .unwrap()
        .solve()
        .unwrap();
        assert!((singular.perturbation() - 1.5).abs() < 4.0e-4);
    }
}
