//! Link-retention probe for the narrow nonlinear easy-driver family.

use slatec::nonlinear::{NonlinearOptions, solve_system};

fn main() {
    let _ = solve_system(
        &[1.0],
        |x, residual| {
            residual[0] = x[0] * x[0] - 2.0;
        },
        NonlinearOptions::default(),
    );
}
