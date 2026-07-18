// slatec-safe-example
//! Check independently recomputed `DSPLP` primal-dual diagnostics.
//!
//! Requires `std,external-backend,optimization-linear-programming-in-memory`.
//! The checked objective is linear, rather than a least-squares norm.

use slatec::linear_programming::{LinearProgram, LpBound, LpStatus, SparseColumns};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matrix =
        SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0])?;
    let problem = LinearProgram::<f64>::new(
        vec![1.0, 1.0],
        matrix,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
    )?;
    let result = problem.solve()?;
    assert_eq!(result.status, LpStatus::Optimal);
    let diagnostics = &result.solution.expect("optimal status has a solution").diagnostics;
    assert!(diagnostics.maximum_variable_bound_violation <= diagnostics.absolute_tolerance);
    assert!(diagnostics.maximum_dual_feasibility_violation <= diagnostics.absolute_tolerance);
    assert!(diagnostics.primal_dual_objective_gap.unwrap() <= diagnostics.absolute_tolerance);
    Ok(())
}
