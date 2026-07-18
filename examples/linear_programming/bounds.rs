// slatec-safe-example
//! Row, range, and fixed-variable bounds with safe `DSPLP`.
//!
//! Requires `std,external-backend,optimization-linear-programming-in-memory`.
//! The objective is linear; no residual norm is minimized.

use slatec::linear_programming::{LinearProgram, LpBound, LpStatus, SparseColumns};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // minimize x + 3y, subject to x + y = 3, 0 <= x <= 5, y = 1.
    let matrix =
        SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0])?;
    let problem = LinearProgram::<f64>::new(
        vec![1.0, 3.0],
        matrix,
        vec![LpBound::Fixed(3.0)],
        vec![
            LpBound::Range {
                lower: 0.0,
                upper: 5.0,
            },
            LpBound::Fixed(1.0),
        ],
    )?;
    let result = problem.solve()?;
    assert_eq!(result.status, LpStatus::Optimal);
    let solution = result.solution.expect("optimal status has a solution");
    assert!((solution.variables[0] - 2.0).abs() < 1.0e-9);
    assert!((solution.row_activities[0] - 3.0).abs() < 1.0e-9);
    Ok(())
}
