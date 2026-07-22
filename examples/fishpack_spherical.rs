//! Manufactured constant solution for FISHPACK's unit-sphere surface driver.
// slatec-safe-example

use slatec::differential_equations::pde::{
    AxisymmetricSphericalHelmholtz2d, ColatitudeAxis, ColatitudeBoundary, CoordinateBoundary,
    FishpackGrid2, LongitudeAxis, RadialAxis, RadialBoundary, SphereSurfaceHelmholtz2d,
    StaggeredAxisymmetricSphericalHelmholtz2d, StaggeredColatitudeAxis,
    StaggeredLongitudeAxis, StaggeredRadialAxis, StaggeredSphereSurfaceHelmholtz2d,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solution = SphereSurfaceHelmholtz2d::new(
        ColatitudeAxis::new(0.5, 2.5, 8)?,
        LongitudeAxis::full_circle(8)?,
        0.0,
        FishpackGrid2::zeros(9, 9)?,
        ColatitudeBoundary::Dirichlet {
            lower: vec![1.0; 9],
            upper: vec![1.0; 9],
        },
        CoordinateBoundary::Periodic,
    )?
    .solve()?;

    assert!(solution
        .values()
        .values()
        .iter()
        .all(|value| (*value - 1.0).abs() < 1.0e-4));

    let staggered = StaggeredSphereSurfaceHelmholtz2d::new(
        StaggeredColatitudeAxis::new(0.5, 2.5, 8)?,
        StaggeredLongitudeAxis::full_circle(8)?,
        0.0,
        FishpackGrid2::zeros(8, 8)?,
        ColatitudeBoundary::Dirichlet {
            lower: vec![1.0; 8],
            upper: vec![1.0; 8],
        },
        CoordinateBoundary::Periodic,
    )?
    .solve()?;
    assert!(staggered
        .values()
        .values()
        .iter()
        .all(|value| (*value - 1.0).abs() < 1.0e-4));

    let axisymmetric = AxisymmetricSphericalHelmholtz2d::new(
        ColatitudeAxis::full_sphere(8)?,
        RadialAxis::new(1.0, 2.0, 8)?,
        0.0,
        FishpackGrid2::zeros(9, 9)?,
        ColatitudeBoundary::BothPoles,
        RadialBoundary::Dirichlet {
            lower: vec![1.0; 9],
            upper: vec![1.0; 9],
        },
    )?
    .solve()?;
    assert!(axisymmetric
        .values()
        .values()
        .iter()
        .all(|value| (*value - 1.0).abs() < 5.0e-4));

    let staggered_axisymmetric = StaggeredAxisymmetricSphericalHelmholtz2d::new(
        StaggeredColatitudeAxis::full_sphere(8)?,
        StaggeredRadialAxis::new(1.0, 2.0, 8)?,
        0.0,
        FishpackGrid2::zeros(8, 8)?,
        ColatitudeBoundary::BothPoles,
        RadialBoundary::Dirichlet {
            lower: vec![1.0; 8],
            upper: vec![1.0; 8],
        },
    )?
    .solve()?;
    assert!(staggered_axisymmetric
        .values()
        .values()
        .iter()
        .all(|value| (*value - 1.0).abs() < 5.0e-4));
    Ok(())
}
