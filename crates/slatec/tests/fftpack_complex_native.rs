#![cfg(all(
    feature = "fftpack-complex-native-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

use core::mem::{align_of, size_of};

use num_complex::Complex32;
use slatec::transforms::fft::FftError;
use slatec::transforms::fft::complex::ComplexFftPlan32;

const PI: f32 = core::f32::consts::PI;

fn assert_close(actual: &[Complex32], expected: &[Complex32], tolerance: f32) {
    assert_eq!(actual.len(), expected.len());
    for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
        let scale = 1.0_f32
            .max(actual.re.abs())
            .max(actual.im.abs())
            .max(expected.re.abs())
            .max(expected.im.abs());
        assert!(
            (actual.re - expected.re).abs() <= tolerance * scale
                && (actual.im - expected.im).abs() <= tolerance * scale,
            "index {index}: actual {actual:?}, expected {expected:?}, tolerance {tolerance}"
        );
    }
}

fn direct_dft(input: &[Complex32], sign: f32) -> Vec<Complex32> {
    let length = input.len();
    (0..length)
        .map(|output_index| {
            input
                .iter()
                .enumerate()
                .fold(Complex32::new(0.0, 0.0), |sum, (input_index, &value)| {
                    let angle =
                        sign * 2.0 * PI * output_index as f32 * input_index as f32 / length as f32;
                    sum + value * Complex32::new(angle.cos(), angle.sin())
                })
        })
        .collect()
}

fn sample(length: usize) -> Vec<Complex32> {
    (0..length)
        .map(|index| {
            let x = index as f32;
            Complex32::new(0.2 + (0.37 * x).sin(), -0.1 + (0.19 * x).cos())
        })
        .collect()
}

#[test]
fn public_layout_matches_the_reviewed_interleaved_words() {
    assert_eq!(size_of::<Complex32>(), 2 * size_of::<f32>());
    assert_eq!(align_of::<Complex32>(), align_of::<f32>());
    let values = [Complex32::new(1.25, -2.5), Complex32::new(3.0, 4.5)];
    // SAFETY: this test verifies num-complex's documented repr(C) two-f32
    // layout, the same contract isolated by the production wrapper.
    let words = unsafe { core::slice::from_raw_parts(values.as_ptr().cast::<f32>(), 4) };
    assert_eq!(words, &[1.25, -2.5, 3.0, 4.5]);
}

#[test]
fn complex_fft_matches_direct_dft_and_round_trips() {
    for length in [2_usize, 3, 8, 9, 12] {
        let mut plan = ComplexFftPlan32::new(length).expect("valid plan");
        let mut impulse = vec![Complex32::new(0.0, 0.0); length];
        impulse[0] = Complex32::new(1.0, 0.0);
        let mut transformed_impulse = impulse.clone();
        let pointer = transformed_impulse.as_ptr();
        plan.forward(&mut transformed_impulse)
            .expect("forward impulse");
        assert_eq!(pointer, transformed_impulse.as_ptr());
        assert_close(
            &transformed_impulse,
            &vec![Complex32::new(1.0, 0.0); length],
            3.0e-5,
        );

        let constant = vec![Complex32::new(0.25, -0.5); length];
        let mut transformed_constant = constant.clone();
        plan.forward(&mut transformed_constant)
            .expect("forward constant");
        assert_close(&transformed_constant, &direct_dft(&constant, -1.0), 3.0e-5);

        let original = sample(length);
        let mut transformed = original.clone();
        plan.forward(&mut transformed).expect("forward DFT");
        assert_close(&transformed, &direct_dft(&original, -1.0), 4.0e-5);
        plan.backward(&mut transformed).expect("backward DFT");
        let scaled = original
            .iter()
            .map(|value| *value * length as f32)
            .collect::<Vec<_>>();
        assert_close(&transformed, &scaled, 5.0e-5);
    }
}

#[test]
fn complex_exponential_establishes_forward_sign_and_frequency_bin() {
    let length = 9;
    let frequency = 2;
    let input = (0..length)
        .map(|index| {
            let angle = 2.0 * PI * frequency as f32 * index as f32 / length as f32;
            Complex32::new(angle.cos(), angle.sin())
        })
        .collect::<Vec<_>>();
    let mut transformed = input;
    let mut plan = ComplexFftPlan32::new(length).unwrap();
    plan.forward(&mut transformed).unwrap();
    for (index, value) in transformed.iter().enumerate() {
        let expected = if index == frequency {
            Complex32::new(length as f32, 0.0)
        } else {
            Complex32::new(0.0, 0.0)
        };
        assert_close(
            core::slice::from_ref(value),
            core::slice::from_ref(&expected),
            5.0e-5,
        );
    }
}

#[test]
fn exact_length_and_dimension_validation_prevent_native_entry() {
    let mut plan = ComplexFftPlan32::new(3).unwrap();
    assert_eq!(
        plan.forward(&mut [Complex32::new(0.0, 0.0); 2]),
        Err(FftError::LengthMismatch {
            expected: 3,
            actual: 2
        })
    );
    assert!(matches!(
        ComplexFftPlan32::new(usize::MAX),
        Err(FftError::DimensionOverflow)
    ));
    assert!(matches!(
        ComplexFftPlan32::new(1),
        Err(FftError::InvalidLength {
            length: 1,
            minimum: 2
        })
    ));
}

#[test]
fn non_finite_values_are_sized_safely_and_do_not_escape_native_control_flow() {
    let mut plan = ComplexFftPlan32::new(2).unwrap();
    let mut values = [
        Complex32::new(f32::NAN, 0.0),
        Complex32::new(1.0, f32::INFINITY),
    ];
    plan.forward(&mut values).unwrap();
    assert!(
        values
            .iter()
            .any(|value| value.re.is_nan() || value.im.is_nan())
    );
}
