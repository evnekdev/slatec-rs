//! Fit with a fixed variable using equal closed `DBOLS` bounds.
//!
//! Required features: `std`, one native backend, and
//! `least-squares-linear-bounded`. Equal endpoints are an audited `IND=3`
//! representation, not a general equality-constraint interface.

// slatec-safe-example: fixed bounded linear least squares

use slatec::bounded_least_squares::{
    BoundedLeastSquaresOptions, BoundedLeastSquaresProblem, VariableBounds,
    solve_bounded_least_squares,
};
use slatec::linear_least_squares::MatrixRef;

fn main() -> Result<(), slatec::bounded_least_squares::BoundedLeastSquaresError> {
    let matrix = MatrixRef::column_major(&[1.0_f64, 0.0, 0.0, 1.0], 2, 2, 2)?;
    let fit = solve_bounded_least_squares(
        BoundedLeastSquaresProblem {
            matrix,
            rhs: &[7.0, 2.0],
            bounds: &[VariableBounds::Fixed(3.0), VariableBounds::Unbounded],
        },
        BoundedLeastSquaresOptions::default(),
    )?;
    assert!((fit.solution[0] - 3.0).abs() < 1.0e-12);
    assert!((fit.solution[1] - 2.0).abs() < 1.0e-12);
    assert!((fit.residual_norm - 4.0).abs() < 1.0e-12);
    Ok(())
}
