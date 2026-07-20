//! `banded-linear-systems` uses compact general-band storage, not a dense matrix.
//! slatec-safe-example: factorization, reciprocal conditioning, and scaled determinant.
use slatec::linear_algebra::banded::BandMatrixRef;

fn main() {
    // A tridiagonal 4x4 matrix: rows are superdiagonal, diagonal, subdiagonal.
    let storage: [f64; 12] = [0., 2., -1., -1., 2., -1., -1., 2., -1., -1., 2., 0.];
    let matrix = BandMatrixRef::from_compact_storage(&storage, 4, 4, 1, 1).unwrap();
    let (lu, reciprocal_condition) = matrix.factorize_with_condition_estimate().unwrap();
    assert!(reciprocal_condition.value() > 0.0);
    let determinant = lu.scaled_determinant().unwrap();
    assert!(determinant.mantissa().is_finite());
    let mut rhs = [1., 0., 0., 1.];
    lu.solve_in_place(&mut rhs).unwrap();
    assert!(rhs.iter().all(|value| (value - 1.0_f64).abs() < 1.0e-12));
}
