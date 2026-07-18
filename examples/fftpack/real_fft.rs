// slatec-safe-example
//! A packed periodic real Fourier transform using SLATEC `RFFTI/RFFTF/RFFTB`.
//!
//! Requires `std,external-backend,fftpack-real` and a reviewed linked SLATEC
//! backend. FFTPACK keeps its native unnormalised scaling: backward(forward(x))
//! is `N * x`.

use slatec::fftpack::{EasyRealFftPlan, RealFftPlan};

fn main() {
    let original = vec![1.0_f32, 0.0, -1.0, 0.0];
    let mut values = original.clone();
    let mut plan = RealFftPlan::new(values.len()).expect("valid plan");
    plan.forward(&mut values).expect("forward RFFTF");
    let spectrum = plan.spectrum(&values).expect("native packed spectrum");
    assert!((spectrum.dc()).abs() < 1.0e-5);
    assert!(spectrum.harmonic(1).is_some());
    assert!(spectrum.nyquist().is_some());
    plan.backward(&mut values).expect("backward RFFTB");
    for (actual, expected) in values.iter().zip(original) {
        assert!((*actual - 4.0 * expected).abs() < 1.0e-4);
    }

    let easy = EasyRealFftPlan::new(4).expect("valid EZFFT plan");
    let coefficients = easy.forward(&[1.0, 0.0, -1.0, 0.0]).expect("EZFFTF");
    let mut reconstructed = [0.0; 4];
    easy.backward(coefficients, &mut reconstructed)
        .expect("EZFFTB");
    assert!((reconstructed[0] - 1.0).abs() < 1.0e-4);
}
