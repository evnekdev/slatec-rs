// slatec-safe-example
//! Quarter-wave sine and cosine transforms from `SINQ*` and `COSQ*`.
//!
//! Requires `std,external-backend,fftpack-real`. Both forward/backward pairs
//! preserve FFTPACK's native `4 * N` composition scale.

use slatec::fftpack::{QuarterWaveCosinePlan, QuarterWaveSinePlan};

fn main() {
    let original = vec![0.25_f32, -0.5, 1.0];
    let mut sine = original.clone();
    let mut sine_plan = QuarterWaveSinePlan::new(sine.len()).expect("valid plan");
    sine_plan.forward(&mut sine).expect("SINQF");
    sine_plan.backward(&mut sine).expect("SINQB");

    let mut cosine = original.clone();
    let mut cosine_plan = QuarterWaveCosinePlan::new(cosine.len()).expect("valid plan");
    cosine_plan.forward(&mut cosine).expect("COSQF");
    cosine_plan.backward(&mut cosine).expect("COSQB");

    for ((sine, cosine), expected) in sine.iter().zip(&cosine).zip(original) {
        assert!((*sine - 12.0 * expected).abs() < 1.0e-4);
        assert!((*cosine - 12.0 * expected).abs() < 1.0e-4);
    }
}
