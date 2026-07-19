//! `banded-linear-systems` uses compact general-band storage, not a dense matrix.
use slatec::linear_algebra::banded::BandMatrixRef;

fn main() {
    // A tridiagonal 4x4 matrix: rows are superdiagonal, diagonal, subdiagonal.
    let storage: [f64; 12] = [0., 2., -1., -1., 2., -1., -1., 2., -1., -1., 2., 0.];
    let matrix = BandMatrixRef::from_compact_storage(&storage, 4, 4, 1, 1).unwrap();
    let lu = matrix.factorize().unwrap();
    let mut rhs = [1., 0., 0., 1.];
    lu.solve_in_place(&mut rhs).unwrap();
    assert_eq!(rhs, [1., 1., 1., 1.]);
}
