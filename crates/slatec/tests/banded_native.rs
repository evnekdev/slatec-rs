#![cfg(feature = "banded-linear-systems-native-tests")]

use slatec::linear_algebra::banded::{BandElementRef, BandError, BandMatrixRef};

fn tridiagonal64() -> Vec<f64> {
    // Compact `(kl+ku+1) x n` column-major storage; main diagonal is row 1.
    vec![0., 2., -1., -1., 2., -1., -1., 2., -1., -1., 2., 0.]
}

fn assert_close(values: &[f64], expected: &[f64]) {
    assert!(
        values
            .iter()
            .zip(expected)
            .all(|(actual, target)| (actual - target).abs() < 1.0e-12)
    );
}

#[test]
fn solves_and_reuses_a_pivoted_tridiagonal_factorization() {
    let storage = tridiagonal64();
    let a = BandMatrixRef::from_compact_storage(&storage, 4, 4, 1, 1).unwrap();
    let lu = a.factorize().unwrap();
    let mut rhs = vec![1., 0., 0., 1.];
    lu.solve_in_place(&mut rhs).unwrap();
    assert_close(&rhs, &[1., 1., 1., 1.]);
    let mut second = vec![2., 0., 0., 2.];
    lu.solve_in_place(&mut second).unwrap();
    assert_close(&second, &[2., 2., 2., 2.]);
}

#[test]
fn transpose_and_storage_semantics_are_checked() {
    let values = tridiagonal64();
    let a = BandMatrixRef::from_compact_storage(&values, 4, 4, 1, 1).unwrap();
    assert!(matches!(a.get(0, 3), Some(BandElementRef::StructuralZero)));
    assert!(a.get(4, 0).is_none());
    let lu = a.factorize().unwrap();
    let mut rhs = vec![1., 0., 0., 1.];
    lu.solve_transpose_in_place(&mut rhs).unwrap();
    assert_close(&rhs, &[1., 1., 1., 1.]);
}

#[test]
fn reports_singular_and_invalid_inputs() {
    let singular = [0.0_f32, 0.0];
    let a = BandMatrixRef::from_compact_storage(&singular, 2, 2, 0, 0).unwrap();
    assert!(matches!(
        a.factorize(),
        Err(BandError::Singular { pivot: 1 })
    ));
    assert!(matches!(
        BandMatrixRef::<f64>::from_compact_storage(&[], 0, 0, 0, 0),
        Err(BandError::InvalidDimensions)
    ));
}

fn assert_scaled64(
    actual: slatec::linear_algebra::banded::ScaledDeterminant<f64>,
    mantissa: f64,
    exponent: i32,
) {
    assert!((actual.mantissa() - mantissa).abs() < 1.0e-12, "{actual:?}");
    assert_eq!(actual.exponent10(), exponent);
    assert!(
        (actual.mantissa() == 0.0)
            || (actual.mantissa().abs() >= 1.0 && actual.mantissa().abs() < 10.0)
    );
}

#[test]
fn estimates_reciprocal_one_norm_condition_while_factoring() {
    let diagonal = [2.0_f64, 8.0];
    let matrix = BandMatrixRef::from_compact_storage(&diagonal, 2, 2, 0, 0).unwrap();
    let (lu, estimate) = matrix.factorize_with_condition_estimate().unwrap();
    // LINPACK's reciprocal estimator is intentionally approximate: for this
    // diagonal matrix the reviewed implementation returns 5/17 rather than
    // the exact reciprocal condition 1/4. Check the stable qualitative band,
    // not equality with a different estimator.
    assert!(estimate.value() > 0.2 && estimate.value() < 0.4);
    assert!(estimate.value() >= 0.0 && estimate.value() <= 1.0);
    let mut rhs = [2.0, 8.0];
    lu.solve_in_place(&mut rhs).unwrap();
    assert_close(&rhs, &[1.0, 1.0]);

    let near_singular = [1.0_f64, 1.0e-12];
    let matrix = BandMatrixRef::from_compact_storage(&near_singular, 2, 2, 0, 0).unwrap();
    let (_, near_estimate) = matrix.factorize_with_condition_estimate().unwrap();
    assert!(near_estimate.value() >= 0.0 && near_estimate.value() < estimate.value());

    let identity = [1.0_f32, 1.0, 1.0];
    let matrix = BandMatrixRef::from_compact_storage(&identity, 3, 3, 0, 0).unwrap();
    let (_, estimate) = matrix.factorize_with_condition_estimate().unwrap();
    assert!((estimate.value() - 1.0).abs() < 1.0e-6);

    let singular = [0.0_f64, 0.0];
    let matrix = BandMatrixRef::from_compact_storage(&singular, 2, 2, 0, 0).unwrap();
    assert!(matches!(
        matrix.factorize_with_condition_estimate(),
        Err(BandError::Singular { pivot: 1 })
    ));
}

#[test]
fn returns_scaled_determinants_without_consuming_factors() {
    let diagonal = [2.0_f64, 8.0];
    let matrix = BandMatrixRef::from_compact_storage(&diagonal, 2, 2, 0, 0).unwrap();
    let lu = matrix.factorize().unwrap();
    assert_scaled64(lu.scaled_determinant().unwrap(), 1.6, 1);
    assert_scaled64(lu.scaled_determinant().unwrap(), 1.6, 1);

    let pivoted = [0.0_f64, 0.0, 2.0, 1.0, 3.0, 0.0];
    let matrix = BandMatrixRef::from_compact_storage(&pivoted, 2, 2, 1, 1).unwrap();
    let lu = matrix.factorize().unwrap();
    assert_scaled64(lu.scaled_determinant().unwrap(), -2.0, 0);

    let large = [1.0e20_f64, 1.0e20];
    let matrix = BandMatrixRef::from_compact_storage(&large, 2, 2, 0, 0).unwrap();
    assert_scaled64(
        matrix.factorize().unwrap().scaled_determinant().unwrap(),
        1.0,
        40,
    );

    let small = [1.0e-20_f64, 1.0e-20];
    let matrix = BandMatrixRef::from_compact_storage(&small, 2, 2, 0, 0).unwrap();
    assert_scaled64(
        matrix.factorize().unwrap().scaled_determinant().unwrap(),
        1.0,
        -40,
    );

    let f32_diagonal = [2.0_f32, 8.0];
    let matrix = BandMatrixRef::from_compact_storage(&f32_diagonal, 2, 2, 0, 0).unwrap();
    let determinant = matrix.factorize().unwrap().scaled_determinant().unwrap();
    assert!((determinant.mantissa() - 1.6).abs() < 1.0e-5);
    assert_eq!(determinant.exponent10(), 1);
}
