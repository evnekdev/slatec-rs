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
