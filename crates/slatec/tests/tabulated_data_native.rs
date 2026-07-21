use slatec::interpolation::tabulated::{TabulatedData, TabulatedDataError};
use slatec::quadrature::{IntegrationError, integrate_tabulated, integrate_tabulated_f32};

#[test]
fn polynomial_interpolation_evaluation_derivatives_and_taylor_coefficients_match_a_quadratic() {
    let data =
        TabulatedData::<f64>::from_samples(vec![0.0_f64, 1.0, 2.0], vec![1.0, 4.0, 9.0]).unwrap();
    let polynomial = data.interpolating_polynomial().unwrap();
    assert_eq!(polynomial.degree(), 2);
    assert!((polynomial.evaluate(1.5).unwrap() - 6.25).abs() < 1.0e-12);
    let evaluation = polynomial.evaluate_with_derivatives(1.5, 3).unwrap();
    assert!((evaluation.value - 6.25).abs() < 1.0e-12);
    assert!((evaluation.derivatives[0] - 5.0).abs() < 1.0e-12);
    assert!((evaluation.derivatives[1] - 2.0).abs() < 1.0e-12);
    assert_eq!(evaluation.derivatives[2], 0.0);
    let coefficients = polynomial.taylor_coefficients_at(1.0).unwrap();
    assert_eq!(coefficients, vec![4.0, 4.0, 1.0]);

    let single =
        TabulatedData::<f32>::from_samples(vec![0.0_f32, 1.0, 2.0], vec![1.0, 4.0, 9.0]).unwrap();
    let single_polynomial = single.interpolating_polynomial().unwrap();
    let single_evaluation = single_polynomial.evaluate_with_derivatives(1.5, 2).unwrap();
    assert!((single_evaluation.value - 6.25).abs() < 1.0e-5);
    assert!((single_evaluation.derivatives[0] - 5.0).abs() < 1.0e-5);
    assert!((single_evaluation.derivatives[1] - 2.0).abs() < 1.0e-5);
}

#[test]
fn overlapping_parabola_integration_matches_the_tabulated_quadratic() {
    let double =
        TabulatedData::<f64>::from_samples(vec![0.0_f64, 0.5, 1.0], vec![0.0, 0.25, 1.0]).unwrap();
    let single =
        TabulatedData::<f32>::from_samples(vec![0.0_f32, 0.5, 1.0], vec![0.0, 0.25, 1.0]).unwrap();
    assert!((integrate_tabulated(&double, 0.0..=1.0).unwrap() - 1.0 / 3.0).abs() < 1.0e-12);
    assert!((integrate_tabulated_f32(&single, 0.0..=1.0).unwrap() - 1.0 / 3.0).abs() < 1.0e-6);
}

#[test]
fn safe_tabulated_integration_matches_the_reviewed_raw_entry() {
    let data =
        TabulatedData::<f64>::from_samples(vec![0.0_f64, 0.5, 1.0], vec![0.0, 0.25, 1.0]).unwrap();
    let safe = integrate_tabulated(&data, 0.0..=1.0).unwrap();

    let mut abscissas = data.abscissas().to_vec();
    let mut values = data.values().to_vec();
    let mut count = 3;
    let mut lower = 0.0;
    let mut upper = 1.0;
    let mut raw = 0.0;
    let mut status = 0;
    // SAFETY: these private arrays meet DAVINT's reviewed finite, strictly
    // increasing `X(N)`/`Y(N)` contract and every scalar is valid output
    // storage for this exact call.
    unsafe {
        slatec_sys::quadrature::davint(
            abscissas.as_mut_ptr(),
            values.as_mut_ptr(),
            &mut count,
            &mut lower,
            &mut upper,
            &mut raw,
            &mut status,
        );
    }
    assert_eq!(status, 1);
    assert_eq!(safe, raw);
}

#[test]
fn validation_prevents_native_invalid_input_paths() {
    assert!(matches!(
        TabulatedData::<f64>::from_samples(vec![0.0, 1.0], vec![0.0, f64::NAN]),
        Err(TabulatedDataError::NonFiniteValue { index: 1 })
    ));
    let data =
        TabulatedData::<f64>::from_samples(vec![0.0_f64, 1.0, 2.0], vec![0.0, 1.0, 4.0]).unwrap();
    assert_eq!(
        integrate_tabulated(&data, 1.0..=0.0),
        Err(IntegrationError::InvalidBounds)
    );
    assert_eq!(
        integrate_tabulated(&data, 0.0..=0.5),
        Err(IntegrationError::InsufficientTabulatedPoints { found: 1 })
    );
    let polynomial = data.interpolating_polynomial().unwrap();
    assert_eq!(
        polynomial.evaluate(f64::INFINITY),
        Err(TabulatedDataError::NonFiniteQuery)
    );
}
