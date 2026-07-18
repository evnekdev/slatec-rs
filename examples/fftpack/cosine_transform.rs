// slatec-safe-example
//! The full cosine transform `COSTI/COST` from real FFTPACK.
//!
//! Requires `std,external-backend,fftpack-real`. `COST` is self-inverse up to
//! the native factor `2 * (N - 1)`.

use slatec::fftpack::CosineTransformPlan;

fn main() {
    let original = vec![0.5_f32, 1.0, -0.5, 0.25];
    let mut values = original.clone();
    let mut plan = CosineTransformPlan::new(values.len()).expect("valid plan");
    plan.transform(&mut values).expect("COST");
    plan.transform(&mut values).expect("COST inverse scale");
    for (actual, expected) in values.iter().zip(original) {
        assert!((*actual - 6.0 * expected).abs() < 1.0e-4);
    }
}
