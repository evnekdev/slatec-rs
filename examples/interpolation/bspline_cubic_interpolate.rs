//! Cubic interpolation with the reviewed `BINT4`/`DBINT4` constructors.
// slatec-safe-example: cubic B-spline interpolation

use slatec::interpolation::bspline::{BSpline, CubicBoundaryCondition, CubicKnotPlacement};

fn main() -> Result<(), slatec::interpolation::bspline::BSplineError> {
    let nodes = [0.0_f64, 0.5, 1.5, 2.0];
    let values = nodes.map(|x| x * x * x - 2.0 * x + 1.0);
    let spline = BSpline::<f64>::interpolate_cubic(
        &nodes,
        &values,
        CubicBoundaryCondition::FirstDerivative(-2.0),
        CubicBoundaryCondition::FirstDerivative(10.0),
        CubicKnotPlacement::EndpointMultiplicity,
    )?;

    let value = spline.evaluate(1.25)?;
    let slope = spline.derivative(1.25, 1)?;
    println!("value={value}, slope={slope}, knots={:?}", spline.knots());
    Ok(())
}
