//! Narrow link probe for the safe in-memory `DSPLP` surface.

use slatec::linear_programming::{LinearProgram, LpBound, LpStatus, SparseColumns};

fn main() {
    let matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
    let problem = LinearProgram::<f64>::new(
        vec![1.0],
        matrix,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Lower(0.0)],
    )
    .unwrap();
    assert_eq!(problem.solve().unwrap().status, LpStatus::Optimal);
}
