// slatec-safe-example
//! Classify an infeasible linear program with `DSPLP`.
//!
//! Requires `std,external-backend,optimization-linear-programming-in-memory`.
//! This is an LP feasibility diagnosis, not constrained least squares.

use slatec::linear_programming::{LinearProgram, LpBound, LpStatus, SparseColumns};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // x is fixed at zero while the row requires x >= 1.
    let matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0])?;
    let problem = LinearProgram::<f64>::new(
        vec![1.0],
        matrix,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Fixed(0.0)],
    )?;
    let result = problem.solve()?;
    assert_eq!(result.status, LpStatus::Infeasible);
    assert!(result.solution.is_none());
    Ok(())
}
