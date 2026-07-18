// slatec-safe-example
//! Classify a non-optimal `DSPLP` iteration-limit return safely.
//!
//! Requires `std,external-backend,optimization-linear-programming-in-memory`.
//! The objective is linear; a returned progress iterate is not a least-squares
//! solution and is never reported as optimal.

use slatec::linear_programming::{
    LinearProgram, LpBound, LpOptions, LpStatus, LpValidation, SparseColumns,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matrix =
        SparseColumns::<f64>::new(2, 2, vec![0, 1, 2], vec![0, 1], vec![1.0, 1.0])?;
    let problem = LinearProgram::<f64>::new(
        vec![-1.0, -1.0],
        matrix,
        vec![LpBound::Upper(1.0), LpBound::Upper(1.0)],
        vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
    )?;
    let result = problem.solve_with_options(LpOptions {
        maximum_iterations: Some(1),
        validation: LpValidation::NativeOnly,
        ..LpOptions::default()
    })?;
    assert_eq!(result.status, LpStatus::IterationLimit);
    assert!(result.solution.is_none());
    assert!(result.progress.is_some());
    Ok(())
}
