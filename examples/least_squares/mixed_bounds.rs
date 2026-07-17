//! Solve a mixed-bound single-precision problem with original `SBOLS`.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-linear-bounded`.

// slatec-safe-example: mixed bounded linear least squares

use slatec::bounded_least_squares::{
    BoundedLeastSquaresOptions, BoundedLeastSquaresProblem, VariableBounds,
    solve_bounded_least_squares_f32,
};
use slatec::linear_least_squares::MatrixRef;

fn main() -> Result<(), slatec::bounded_least_squares::BoundedLeastSquaresError> {
    let matrix = MatrixRef::column_major(&[1.0_f32, 0.0, 0.0, 1.0], 2, 2, 2)?;
    let fit = solve_bounded_least_squares_f32(
        BoundedLeastSquaresProblem {
            matrix,
            rhs: &[3.0, -2.0],
            bounds: &[
                VariableBounds::Upper(2.0),
                VariableBounds::Between {
                    lower: -1.0,
                    upper: 1.0,
                },
            ],
        },
        BoundedLeastSquaresOptions::default(),
    )?;
    assert!((fit.solution[0] - 2.0).abs() < 2.0e-4);
    assert!((fit.solution[1] + 1.0).abs() < 2.0e-4);
    Ok(())
}
