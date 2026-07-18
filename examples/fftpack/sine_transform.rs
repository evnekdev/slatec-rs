// slatec-safe-example
//! The real full sine transform `SINTI/SINT`, not a complex FFT.
//!
//! Requires `std,external-backend,fftpack-real`. The native transform is
//! unnormalised: applying `SINT` twice multiplies by `2 * (N + 1)`.

use slatec::fftpack::SineTransformPlan;

fn main() {
    let original = vec![1.0_f32, -0.5, 0.25];
    let mut values = original.clone();
    let mut plan = SineTransformPlan::new(values.len()).expect("valid plan");
    plan.transform(&mut values).expect("SINT");
    plan.transform(&mut values).expect("SINT inverse scale");
    for (actual, expected) in values.iter().zip(original) {
        assert!((*actual - 8.0 * expected).abs() < 1.0e-4);
    }
}
