//! Manufactured discrete solution for the safe structured FISHPACK `POIS3D` API.
// slatec-safe-example

use slatec::differential_equations::pde::{
    CyclicAxisCoefficients, Grid3, Pois3dProblem, ThirdAxisOperator, TransverseBoundary,
};

fn main() {
    let (l, m, n) = (3, 4, 5);
    let c1 = 0.75;
    let c2 = 1.25;
    let third = CyclicAxisCoefficients::new(0.5, -5.0).expect("finite cyclic coefficients");
    let mut exact = Grid3::zeros(l, m, n).expect("checked grid");
    for k in 0..n {
        for j in 0..m {
            for i in 0..l {
                exact[(i, j, k)] = (i as f32 + 1.0) * (j as f32 + 2.0) + k as f32;
            }
        }
    }
    let rhs = apply_periodic_operator(&exact, c1, c2, third);
    let rhs_for_residual = rhs.clone();
    let solution = Pois3dProblem::new(
        TransverseBoundary::Periodic,
        TransverseBoundary::Periodic,
        c1,
        c2,
        ThirdAxisOperator::Cyclic(third),
        rhs,
    )
    .expect("valid structured system")
    .solve()
    .expect("nonsingular manufactured system");
    let max_error = solution
        .values()
        .iter()
        .zip(exact.values())
        .map(|(actual, expected)| (actual - expected).abs())
        .fold(0.0_f32, f32::max);
    let max_residual = apply_periodic_operator(&solution, c1, c2, third)
        .values()
        .iter()
        .zip(rhs_for_residual.values())
        .map(|(actual, expected)| (actual - expected).abs())
        .fold(0.0_f32, f32::max);
    println!(
        "POIS3D {l}x{m}x{n}, periodic/periodic/cyclic; max error {max_error:.3e}, residual {max_residual:.3e}"
    );
}

fn apply_periodic_operator(
    values: &Grid3,
    c1: f32,
    c2: f32,
    third: CyclicAxisCoefficients,
) -> Grid3 {
    let mut rhs = Grid3::zeros(values.nx(), values.ny(), values.nz()).expect("checked grid");
    for k in 0..values.nz() {
        for j in 0..values.ny() {
            for i in 0..values.nx() {
                let left = values[((i + values.nx() - 1) % values.nx(), j, k)];
                let right = values[((i + 1) % values.nx(), j, k)];
                let lower = values[(i, (j + values.ny() - 1) % values.ny(), k)];
                let upper = values[(i, (j + 1) % values.ny(), k)];
                let previous = values[(i, j, (k + values.nz() - 1) % values.nz())];
                let next = values[(i, j, (k + 1) % values.nz())];
                let centre = values[(i, j, k)];
                rhs[(i, j, k)] = c1 * (left - 2.0 * centre + right)
                    + c2 * (lower - 2.0 * centre + upper)
                    + third.off_diagonal() * previous
                    + third.diagonal() * centre
                    + third.off_diagonal() * next;
            }
        }
    }
    rhs
}
