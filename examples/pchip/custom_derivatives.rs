// slatec-safe-example
//! Exact derivative construction without alteration, then `PCHFE` evaluation.
//! Requires `std,external-backend,pchip`.

use slatec::pchip::{EndpointCondition, PiecewiseCubicHermite};

fn main() {
    let custom =
        PiecewiseCubicHermite::from_derivatives(vec![0.0_f64, 1.0], vec![0.0, 1.0], vec![1.0, 1.0])
            .expect("custom derivatives");
    assert!((custom.evaluate(0.75).expect("PCHFE") - 0.75).abs() < 1.0e-12);
    let spline = PiecewiseCubicHermite::spline(
        &[0.0, 1.0, 2.0],
        &[0.0, 1.0, 0.0],
        EndpointCondition::NotAKnot,
        EndpointCondition::NotAKnot,
    )
    .expect("PCHSP");
    assert!((spline.evaluate(1.0).expect("knot value") - 1.0).abs() < 1.0e-12);
}
