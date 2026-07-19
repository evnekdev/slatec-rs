//! Narrow source-link probe for the safe complex FFTPACK plan API.

use num_complex::Complex32;
use slatec::transforms::fft::complex::ComplexFftPlan32;

fn main() {
    let mut plan = ComplexFftPlan32::new(1).expect("valid complex FFT plan");
    let mut values = [Complex32::new(1.0, 0.0)];
    plan.forward(&mut values).expect("forward transform");
}
