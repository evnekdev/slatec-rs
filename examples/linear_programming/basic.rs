// slatec-safe-example
//! A sparse linear objective solved by `DSPLP`, not a least-squares fit.
//!
//! Requires `std,external-backend,optimization-linear-programming-in-memory`
//! and a reviewed linked backend.

use slatec::linear_programming::{LinearProgram, LpBound, LpStatus, SparseColumns};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // minimize x + 2y, subject to x + y >= 1 and x,y >= 0.
    let matrix =
        SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0])?;
    let problem = LinearProgram::<f64>::new(
        vec![1.0, 2.0],
        matrix,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
    )?;
    let result = problem.solve()?;
    assert_eq!(result.status, LpStatus::Optimal);
    let solution = result.solution.expect("optimal status has a solution");
    assert!((solution.objective_value - 1.0).abs() < 1.0e-9);
    assert!(solution.row_activities[0] >= 1.0 - 1.0e-9);
    Ok(())
}
