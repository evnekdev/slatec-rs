//! Narrow link probe for the reviewed PCHIP source closure.

use slatec::pchip::PiecewiseCubicHermite;

fn main() {
    let curve =
        PiecewiseCubicHermite::<f64>::monotone(&[0.0, 1.0], &[0.0, 1.0]).expect("valid PCHIP data");
    let _ = curve.evaluate(0.5).expect("PCHFD");
}
