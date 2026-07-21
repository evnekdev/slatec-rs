// slatec-safe-example
//! Solve the positive unit-circle system with the safe `DSOS` wrapper.

use slatec::nonlinear::{SystemOptions, solve_scalar_equations};

fn main() {
    let report = solve_scalar_equations(&[0.8, 0.6], SystemOptions::default(), |x, equation| {
        if equation == 0 {
            x[0] * x[0] + x[1] * x[1] - 1.0
        } else {
            x[0] - x[1]
        }
    })
    .expect("scalar equations converge");
    let expected = 2.0_f64.sqrt() / 2.0;
    assert!((report.solution[0] - expected).abs() < 1.0e-6);
    assert!((report.solution[1] - expected).abs() < 1.0e-6);
}
