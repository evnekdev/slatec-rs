//! <!-- slatec-safe-example -->
//! A known complex frequency bin using the safe in-place FFTPACK plan.
//!
//! Requires `std,external-backend,fftpack-complex`. `CFFTF1` uses the negative
//! exponent, so a positive complex exponential appears in its matching bin.

use num_complex::Complex32;
use slatec::transforms::fft::complex::ComplexFftPlan32;

fn main() {
    let length = 8;
    let bin = 3;
    let mut values = (0..length)
        .map(|index| {
            let angle = 2.0 * core::f32::consts::PI * bin as f32 * index as f32 / length as f32;
            Complex32::new(angle.cos(), angle.sin())
        })
        .collect::<Vec<_>>();
    let mut plan = ComplexFftPlan32::new(length).expect("valid complex FFT plan");
    plan.forward(&mut values).expect("CFFTF1 forward transform");
    assert!((values[bin].re - length as f32).abs() < 1.0e-4);
    assert!(values[bin].im.abs() < 1.0e-4);
}
