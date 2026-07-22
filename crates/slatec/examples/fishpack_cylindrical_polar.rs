//! Manufactured constant solutions for the centered FISHPACK cylindrical and polar drivers.
// slatec-safe-example

use slatec::differential_equations::pde::{
    CoordinateAxis, CoordinateBoundary, CylindricalHelmholtz2d, FishpackGrid2, PolarHelmholtz2d,
    RadialAxis, RadialBoundary,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let radius = RadialAxis::new(1.0, 2.0, 8)?;
    let second_axis = CoordinateAxis::new(0.0, 1.0, 8)?;
    let rhs = || FishpackGrid2::zeros(9, 9);
    let radial_boundary = || RadialBoundary::Dirichlet {
        lower: vec![1.0; 9],
        upper: vec![1.0; 9],
    };

    let cylindrical = CylindricalHelmholtz2d::new(
        radius,
        second_axis,
        0.0,
        rhs()?,
        radial_boundary(),
        CoordinateBoundary::Dirichlet {
            lower: vec![1.0; 9],
            upper: vec![1.0; 9],
        },
    )?
    .solve()?;
    assert!(
        cylindrical
            .values()
            .values()
            .iter()
            .all(|value| (*value - 1.0).abs() < 1.0e-4)
    );

    let polar = PolarHelmholtz2d::new(
        radius,
        second_axis,
        0.0,
        rhs()?,
        radial_boundary(),
        CoordinateBoundary::Periodic,
    )?
    .solve()?;
    assert!(
        polar
            .values()
            .values()
            .iter()
            .all(|value| (*value - 1.0).abs() < 1.0e-4)
    );
    Ok(())
}
