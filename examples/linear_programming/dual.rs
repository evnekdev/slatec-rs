// slatec-safe-example
//! Inspect source-audited dual output from the in-memory `DSPLP` wrapper.
//!
//! Requires `std,external-backend,optimization-linear-programming-in-memory`.
//! This is linear programming, not a least-squares residual minimization.

use slatec::linear_programming::{LinearProgram, LpBound, LpStatus, SparseColumns};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // min x + 2y, x + y >= 1, x,y >= 0.  The native minimization
    // convention gives row multiplier 1 and reduced costs (0, 1).
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
    assert!((solution.dual.row_multipliers[0] - 1.0).abs() < 1.0e-9);
    assert!((solution.dual.reduced_costs[1] - 1.0).abs() < 1.0e-9);
    assert!(solution.diagnostics.primal_dual_objective_gap.unwrap() < 1.0e-9);
    Ok(())
}
