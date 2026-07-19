//! Solve a known Cartesian Poisson problem with the safe FISHPACK facade.
// slatec-safe-example

use slatec::differential_equations::pde::{AxisBoundary, CartesianHelmholtz2d, Grid2, UniformAxis};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = UniformAxis::new(-0.5, 1.0, 8)?;
    let y = UniformAxis::new(0.0, 1.0, 6)?;
    let solution = |x: f32, y: f32| 1.0 + x * x + 2.0 * y * y + 3.0 * x * y;
    // The quadratic is exact under the documented centered second-difference stencil.
    let nx = x.nodes()?;
    let ny = y.nodes()?;
    let rhs = Grid2::new(nx, ny, vec![6.0; nx * ny])?;
    let x_lower = (0..y.nodes()?)
        .map(|j| solution(x.lower(), y.lower() + j as f32 * y.spacing()))
        .collect();
    let x_upper_derivative = (0..y.nodes()?)
        .map(|j| 2.0 * x.upper() + 3.0 * (y.lower() + j as f32 * y.spacing()))
        .collect();
    let y_lower_derivative = (0..x.nodes()?)
        .map(|i| 3.0 * (x.lower() + i as f32 * x.spacing()))
        .collect();
    let y_upper = (0..x.nodes()?)
        .map(|i| solution(x.lower() + i as f32 * x.spacing(), y.upper()))
        .collect();

    let result = CartesianHelmholtz2d::new(
        x,
        y,
        0.0,
        rhs,
        AxisBoundary::DirichletNeumann {
            lower: x_lower,
            upper_derivative: x_upper_derivative,
        },
        AxisBoundary::NeumannDirichlet {
            lower_derivative: y_lower_derivative,
            upper: y_upper,
        },
    )?
    .solve()?;

    let mut max_error: f32 = 0.0;
    for j in 0..result.values().ny() {
        for i in 0..result.values().nx() {
            let expected = solution(
                x.lower() + i as f32 * x.spacing(),
                y.lower() + j as f32 * y.spacing(),
            );
            max_error = max_error.max((result.values()[(i, j)] - expected).abs());
        }
    }
    println!(
        "HWSCRT Cartesian solve: {}x{} nodes, max error={max_error:.3e}, perturbation={:?}, uniqueness={:?}",
        result.values().nx(),
        result.values().ny(),
        result.perturbation(),
        result.uniqueness(),
    );
    Ok(())
}
