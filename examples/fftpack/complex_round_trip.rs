//! <!-- slatec-safe-example -->
//! A reusable in-place complex FFTPACK plan using `CFFTI1/CFFTF1/CFFTB1`.
//!
//! Requires `std,external-backend,fftpack-complex`. FFTPACK is unnormalised,
//! so backward(forward(x)) is `N * x`.

use num_complex::Complex32;
use slatec::transforms::fft::complex::ComplexFftPlan32;

fn main() {
    let original = vec![
        Complex32::new(1.0, 0.0),
        Complex32::new(0.0, 1.0),
        Complex32::new(-1.0, 0.0),
        Complex32::new(0.0, -1.0),
    ];
    let mut values = original.clone();
    let mut plan = ComplexFftPlan32::new(values.len()).expect("valid complex FFT plan");
    plan.forward(&mut values).expect("CFFTF1 forward transform");
    plan.backward(&mut values)
        .expect("CFFTB1 backward transform");
    for (actual, expected) in values.iter().zip(&original) {
        assert!((actual.re - 4.0 * expected.re).abs() < 1.0e-4);
        assert!((actual.im - 4.0 * expected.im).abs() < 1.0e-4);
    }
}
