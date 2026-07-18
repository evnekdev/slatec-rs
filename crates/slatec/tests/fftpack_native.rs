#![cfg(feature = "fftpack-real-native-tests")]

use slatec::fftpack::{
    CosineTransformPlan, EasyRealFftPlan, FftError, QuarterWaveCosinePlan, QuarterWaveSinePlan,
    RealFftPlan, SineTransformPlan,
};

const PI: f32 = core::f32::consts::PI;

fn assert_close(actual: &[f32], expected: &[f32], tolerance: f32) {
    assert_eq!(actual.len(), expected.len());
    for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
        let scale = 1.0_f32.max(actual.abs()).max(expected.abs());
        assert!(
            (actual - expected).abs() <= tolerance * scale,
            "index {index}: actual {actual}, expected {expected}, tolerance {tolerance}"
        );
    }
}

fn sample(length: usize) -> Vec<f32> {
    (0..length)
        .map(|index| {
            let index = index as f32;
            0.25 + (0.37 * index).sin() - 0.2 * (0.19 * index).cos()
        })
        .collect()
}

fn input_cases(length: usize) -> Vec<Vec<f32>> {
    let mut impulse = vec![0.0; length];
    impulse[0] = 1.0;
    let denominator = length as f32;
    vec![
        sample(length),
        impulse,
        vec![0.25; length],
        (0..length)
            .map(|index| if index % 2 == 0 { 1.0 } else { -1.0 })
            .collect(),
        (0..length)
            .map(|index| (2.0 * PI * index as f32 / denominator).sin())
            .collect(),
        (0..length)
            .map(|index| (2.0 * PI * index as f32 / denominator).cos())
            .collect(),
    ]
}

fn rfft_reference(values: &[f32]) -> Vec<f32> {
    let length = values.len();
    let mut output = vec![0.0; length];
    output[0] = values.iter().sum();
    for harmonic in 1..=((length - 1) / 2) {
        let mut cosine = 0.0;
        let mut negative_sine = 0.0;
        for (index, &value) in values.iter().enumerate() {
            let angle = 2.0 * PI * harmonic as f32 * index as f32 / length as f32;
            cosine += value * angle.cos();
            negative_sine -= value * angle.sin();
        }
        output[2 * harmonic - 1] = cosine;
        output[2 * harmonic] = negative_sine;
    }
    if length % 2 == 0 {
        output[length - 1] = values
            .iter()
            .enumerate()
            .map(|(index, &value)| if index % 2 == 0 { value } else { -value })
            .sum();
    }
    output
}

fn sint_reference(values: &[f32]) -> Vec<f32> {
    let length = values.len();
    (1..=length)
        .map(|output_index| {
            (1..=length)
                .map(|input_index| {
                    2.0 * values[input_index - 1]
                        * (input_index as f32 * output_index as f32 * PI / (length + 1) as f32)
                            .sin()
                })
                .sum()
        })
        .collect()
}

fn cost_reference(values: &[f32]) -> Vec<f32> {
    let length = values.len();
    (0..length)
        .map(|output_index| {
            let endpoints = values[0]
                + if output_index % 2 == 0 {
                    values[length - 1]
                } else {
                    -values[length - 1]
                };
            endpoints
                + (1..length - 1)
                    .map(|harmonic| {
                        2.0 * values[harmonic]
                            * (harmonic as f32 * output_index as f32 * PI / (length - 1) as f32)
                                .cos()
                    })
                    .sum::<f32>()
        })
        .collect()
}

fn cosqf_reference(values: &[f32]) -> Vec<f32> {
    let length = values.len();
    (1..=length)
        .map(|output_index| {
            values[0]
                + (1..length)
                    .map(|harmonic| {
                        2.0 * values[harmonic]
                            * ((2 * output_index - 1) as f32 * harmonic as f32 * PI
                                / (2 * length) as f32)
                                .cos()
                    })
                    .sum::<f32>()
        })
        .collect()
}

fn cosqb_reference(values: &[f32]) -> Vec<f32> {
    let length = values.len();
    (0..length)
        .map(|output_index| {
            (1..=length)
                .map(|harmonic| {
                    // COSQB's executable source uses 4, despite the old
                    // prologue displaying 2.  This agrees with its stated
                    // 4*N composition relation and every small-N path.
                    4.0 * values[harmonic - 1]
                        * ((2 * harmonic - 1) as f32 * output_index as f32 * PI
                            / (2 * length) as f32)
                            .cos()
                })
                .sum()
        })
        .collect()
}

fn sinqf_reference(values: &[f32]) -> Vec<f32> {
    let length = values.len();
    (1..=length)
        .map(|output_index| {
            let endpoint = if output_index % 2 == 0 {
                -values[length - 1]
            } else {
                values[length - 1]
            };
            endpoint
                + (1..length)
                    .map(|harmonic| {
                        2.0 * values[harmonic - 1]
                            * ((2 * output_index - 1) as f32 * harmonic as f32 * PI
                                / (2 * length) as f32)
                                .sin()
                    })
                    .sum::<f32>()
        })
        .collect()
}

fn sinqb_reference(values: &[f32]) -> Vec<f32> {
    let length = values.len();
    (1..=length)
        .map(|output_index| {
            (1..=length)
                .map(|harmonic| {
                    4.0 * values[harmonic - 1]
                        * ((2 * harmonic - 1) as f32 * output_index as f32 * PI
                            / (2 * length) as f32)
                            .sin()
                })
                .sum()
        })
        .collect()
}

#[test]
fn periodic_real_fft_matches_direct_oracle_and_round_trips() {
    for length in 1..=10 {
        let mut plan = RealFftPlan::new(length).expect("valid plan");
        for original in input_cases(length) {
            let mut transformed = original.clone();
            plan.forward(&mut transformed).expect("forward transform");
            assert_close(&transformed, &rfft_reference(&original), 2.0e-5);

            let spectrum = plan.spectrum(&transformed).expect("matching length");
            assert_close(&[spectrum.dc()], &[transformed[0]], 0.0);
            if length > 2 {
                assert!(spectrum.harmonic(1).is_some());
            }
            assert_eq!(spectrum.nyquist().is_some(), length % 2 == 0);

            plan.backward(&mut transformed).expect("backward transform");
            let expected = original
                .iter()
                .map(|value| value * length as f32)
                .collect::<Vec<_>>();
            assert_close(&transformed, &expected, 2.0e-5);
        }
    }
}

#[test]
fn easy_real_fft_matches_series_coefficients_and_synthesis() {
    for length in 1..=10 {
        let mut plan = EasyRealFftPlan::new(length).expect("valid plan");
        for input in input_cases(length) {
            let spectrum = plan.forward(&input).expect("easy forward transform");
            assert_close(
                &[spectrum.mean],
                &[input.iter().sum::<f32>() / length as f32],
                2.0e-5,
            );
            assert_eq!(spectrum.cosine.len(), length / 2);
            assert_eq!(spectrum.sine.len(), length / 2);
            for harmonic in 1..=(length / 2) {
                let cosine = input
                    .iter()
                    .enumerate()
                    .map(|(index, &value)| {
                        value * (2.0 * PI * harmonic as f32 * index as f32 / length as f32).cos()
                    })
                    .sum::<f32>();
                let sine = input
                    .iter()
                    .enumerate()
                    .map(|(index, &value)| {
                        value * (2.0 * PI * harmonic as f32 * index as f32 / length as f32).sin()
                    })
                    .sum::<f32>();
                let expected_cosine = if length % 2 == 0 && harmonic == length / 2 {
                    cosine / length as f32
                } else {
                    2.0 * cosine / length as f32
                };
                let expected_sine = if length % 2 == 0 && harmonic == length / 2 {
                    0.0
                } else {
                    2.0 * sine / length as f32
                };
                assert_close(&[spectrum.cosine[harmonic - 1]], &[expected_cosine], 2.0e-5);
                assert_close(&[spectrum.sine[harmonic - 1]], &[expected_sine], 2.0e-5);
            }
            let mut output = vec![0.0; length];
            plan.backward(&spectrum, &mut output)
                .expect("easy backward transform");
            assert_close(&output, &input, 2.0e-5);
        }
    }
}

#[test]
fn sine_and_cosine_transforms_match_direct_oracles_and_reuse_plans() {
    for length in 1..=10 {
        let mut plan = SineTransformPlan::new(length).expect("valid sine plan");
        for original in input_cases(length) {
            let mut transformed = original.clone();
            plan.transform(&mut transformed).expect("sine transform");
            assert_close(&transformed, &sint_reference(&original), 3.0e-5);
            plan.transform(&mut transformed)
                .expect("sine inverse transform");
            let expected = original
                .iter()
                .map(|value| value * (2 * (length + 1)) as f32)
                .collect::<Vec<_>>();
            assert_close(&transformed, &expected, 4.0e-5);
        }
    }

    for length in 2..=10 {
        let mut plan = CosineTransformPlan::new(length).expect("valid cosine plan");
        for original in input_cases(length) {
            let mut transformed = original.clone();
            plan.transform(&mut transformed).expect("cosine transform");
            assert_close(&transformed, &cost_reference(&original), 3.0e-5);
            plan.transform(&mut transformed)
                .expect("cosine inverse transform");
            let expected = original
                .iter()
                .map(|value| value * (2 * (length - 1)) as f32)
                .collect::<Vec<_>>();
            assert_close(&transformed, &expected, 4.0e-5);
        }
    }
}

#[test]
fn quarter_wave_transforms_match_direct_oracles_and_round_trip() {
    for length in 1..=10 {
        let mut cosine_plan = QuarterWaveCosinePlan::new(length).expect("valid cosine plan");
        let mut sine_plan = QuarterWaveSinePlan::new(length).expect("valid sine plan");
        for original in input_cases(length) {
            let mut cosine_forward = original.clone();
            cosine_plan
                .forward(&mut cosine_forward)
                .expect("quarter cosine forward");
            assert_close(&cosine_forward, &cosqf_reference(&original), 4.0e-5);
            cosine_plan
                .backward(&mut cosine_forward)
                .expect("quarter cosine backward");
            let expected = original
                .iter()
                .map(|value| value * (4 * length) as f32)
                .collect::<Vec<_>>();
            assert_close(&cosine_forward, &expected, 5.0e-5);

            let mut cosine_backward = original.clone();
            cosine_plan
                .backward(&mut cosine_backward)
                .expect("quarter cosine backward direct");
            assert_close(&cosine_backward, &cosqb_reference(&original), 4.0e-5);

            let mut sine_forward = original.clone();
            sine_plan
                .forward(&mut sine_forward)
                .expect("quarter sine forward");
            assert_close(&sine_forward, &sinqf_reference(&original), 4.0e-5);
            sine_plan
                .backward(&mut sine_forward)
                .expect("quarter sine backward");
            assert_close(&sine_forward, &expected, 5.0e-5);

            let mut sine_backward = original.clone();
            sine_plan
                .backward(&mut sine_backward)
                .expect("quarter sine backward direct");
            assert_close(&sine_backward, &sinqb_reference(&original), 4.0e-5);
        }
    }
}

#[test]
fn exact_length_is_required() {
    let mut plan = RealFftPlan::new(3).expect("valid plan");
    assert_eq!(
        plan.forward(&mut [1.0, 2.0]),
        Err(FftError::LengthMismatch {
            expected: 3,
            actual: 2
        })
    );
}
