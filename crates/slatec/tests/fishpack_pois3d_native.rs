#![cfg(feature = "fishpack-pois3d-native-tests")]

use std::sync::{Arc, Barrier};

use slatec::differential_equations::pde::{
    CyclicAxisCoefficients, Grid3, Pois3dProblem, ThirdAxisOperator, TransverseBoundary,
    TridiagonalAxisCoefficients,
};
use slatec::native_serialization_test_support;
use slatec_sys::FortranInteger;

const TOLERANCE: f32 = 4.0e-4;

fn exact_grid(l: usize, m: usize, n: usize) -> Grid3 {
    let mut values = Grid3::zeros(l, m, n).unwrap();
    for k in 0..n {
        for j in 0..m {
            for i in 0..l {
                values[(i, j, k)] = 0.25 + 0.4 * i as f32 - 0.3 * j as f32
                    + 0.2 * k as f32
                    + 0.1 * (i * j) as f32
                    + 0.05 * (j * k) as f32;
            }
        }
    }
    values
}

fn coefficients(operator: &ThirdAxisOperator, n: usize) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    match operator {
        ThirdAxisOperator::Cyclic(coefficients) => (
            vec![coefficients.off_diagonal(); n],
            vec![coefficients.diagonal(); n],
            vec![coefficients.off_diagonal(); n],
        ),
        ThirdAxisOperator::Tridiagonal(coefficients) => (
            coefficients.lower().to_vec(),
            coefficients.diagonal().to_vec(),
            coefficients.upper().to_vec(),
        ),
    }
}

fn first_neighbor(
    values: &Grid3,
    boundary: TransverseBoundary,
    i: usize,
    j: usize,
    k: usize,
    lower: bool,
) -> f32 {
    if lower && i > 0 {
        return values[(i - 1, j, k)];
    }
    if !lower && i + 1 < values.nx() {
        return values[(i + 1, j, k)];
    }
    match (boundary, lower) {
        (TransverseBoundary::Periodic, true) => values[(values.nx() - 1, j, k)],
        (TransverseBoundary::Periodic, false) => values[(0, j, k)],
        (TransverseBoundary::ZeroBoth | TransverseBoundary::ZeroLowerReflectUpper, true) => 0.0,
        (TransverseBoundary::ZeroBoth | TransverseBoundary::ReflectLowerZeroUpper, false) => 0.0,
        (TransverseBoundary::ReflectBoth | TransverseBoundary::ReflectLowerZeroUpper, true) => {
            values[(1, j, k)]
        }
        (TransverseBoundary::ReflectBoth | TransverseBoundary::ZeroLowerReflectUpper, false) => {
            values[(values.nx() - 2, j, k)]
        }
    }
}

fn second_neighbor(
    values: &Grid3,
    boundary: TransverseBoundary,
    i: usize,
    j: usize,
    k: usize,
    lower: bool,
) -> f32 {
    if lower && j > 0 {
        return values[(i, j - 1, k)];
    }
    if !lower && j + 1 < values.ny() {
        return values[(i, j + 1, k)];
    }
    match (boundary, lower) {
        (TransverseBoundary::Periodic, true) => values[(i, values.ny() - 1, k)],
        (TransverseBoundary::Periodic, false) => values[(i, 0, k)],
        (TransverseBoundary::ZeroBoth | TransverseBoundary::ZeroLowerReflectUpper, true) => 0.0,
        (TransverseBoundary::ZeroBoth | TransverseBoundary::ReflectLowerZeroUpper, false) => 0.0,
        (TransverseBoundary::ReflectBoth | TransverseBoundary::ReflectLowerZeroUpper, true) => {
            values[(i, 1, k)]
        }
        (TransverseBoundary::ReflectBoth | TransverseBoundary::ZeroLowerReflectUpper, false) => {
            values[(i, values.ny() - 2, k)]
        }
    }
}

fn apply_operator(
    values: &Grid3,
    l_boundary: TransverseBoundary,
    m_boundary: TransverseBoundary,
    c1: f32,
    c2: f32,
    operator: &ThirdAxisOperator,
) -> Grid3 {
    let (lower, diagonal, upper) = coefficients(operator, values.nz());
    let mut rhs = Grid3::zeros(values.nx(), values.ny(), values.nz()).unwrap();
    for k in 0..values.nz() {
        for j in 0..values.ny() {
            for i in 0..values.nx() {
                let previous = if k == 0 {
                    match operator {
                        ThirdAxisOperator::Cyclic(_) => values[(i, j, values.nz() - 1)],
                        ThirdAxisOperator::Tridiagonal(_) => 0.0,
                    }
                } else {
                    values[(i, j, k - 1)]
                };
                let next = if k + 1 == values.nz() {
                    match operator {
                        ThirdAxisOperator::Cyclic(_) => values[(i, j, 0)],
                        ThirdAxisOperator::Tridiagonal(_) => 0.0,
                    }
                } else {
                    values[(i, j, k + 1)]
                };
                let centre = values[(i, j, k)];
                rhs[(i, j, k)] = c1
                    * (first_neighbor(values, l_boundary, i, j, k, true) - 2.0 * centre
                        + first_neighbor(values, l_boundary, i, j, k, false))
                    + c2 * (second_neighbor(values, m_boundary, i, j, k, true) - 2.0 * centre
                        + second_neighbor(values, m_boundary, i, j, k, false))
                    + lower[k] * previous
                    + diagonal[k] * centre
                    + upper[k] * next;
            }
        }
    }
    rhs
}

fn problem_for(
    l_boundary: TransverseBoundary,
    m_boundary: TransverseBoundary,
    operator: ThirdAxisOperator,
) -> (Pois3dProblem, Grid3) {
    let exact = exact_grid(3, 4, 5);
    let c1 = 0.75;
    let c2 = 1.25;
    let rhs = apply_operator(&exact, l_boundary, m_boundary, c1, c2, &operator);
    (
        Pois3dProblem::new(l_boundary, m_boundary, c1, c2, operator, rhs).unwrap(),
        exact,
    )
}

fn assert_grid_close(actual: &Grid3, expected: &Grid3) {
    assert_eq!(
        (actual.nx(), actual.ny(), actual.nz()),
        (expected.nx(), expected.ny(), expected.nz())
    );
    for (index, (actual, expected)) in actual.values().iter().zip(expected.values()).enumerate() {
        assert!(
            (actual - expected).abs() <= TOLERANCE,
            "index={index}, actual={actual}, expected={expected}"
        );
    }
}

#[test]
fn manufactured_solutions_cover_the_transverse_modes_and_cyclic_axis() {
    let cyclic = || ThirdAxisOperator::Cyclic(CyclicAxisCoefficients::new(0.5, -7.0).unwrap());
    for (l_boundary, m_boundary) in [
        (TransverseBoundary::Periodic, TransverseBoundary::Periodic),
        (TransverseBoundary::ZeroBoth, TransverseBoundary::ZeroBoth),
        (
            TransverseBoundary::ZeroLowerReflectUpper,
            TransverseBoundary::ReflectLowerZeroUpper,
        ),
        (
            TransverseBoundary::ReflectBoth,
            TransverseBoundary::ReflectBoth,
        ),
    ] {
        let (problem, expected) = problem_for(l_boundary, m_boundary, cyclic());
        assert_grid_close(&problem.solve().unwrap(), &expected);
    }
}

#[test]
fn manufactured_solution_covers_a_noncyclic_third_axis() {
    let operator = ThirdAxisOperator::Tridiagonal(
        TridiagonalAxisCoefficients::new(
            vec![0.0, 0.3, 0.4, 0.25, 0.2],
            vec![-7.0, -6.5, -7.25, -6.75, -7.5],
            vec![0.2, 0.35, 0.45, 0.3, 0.0],
        )
        .unwrap(),
    );
    let (problem, expected) = problem_for(
        TransverseBoundary::ZeroLowerReflectUpper,
        TransverseBoundary::ReflectBoth,
        operator,
    );
    assert_grid_close(&problem.solve().unwrap(), &expected);
}

fn linear_index(i: usize, j: usize, k: usize, l: usize, m: usize) -> usize {
    k * l * m + j * l + i
}

fn add_first_matrix_neighbor(
    matrix: &mut [Vec<f32>],
    row: usize,
    boundary: TransverseBoundary,
    i: usize,
    j: usize,
    k: usize,
    l: usize,
    m: usize,
    lower: bool,
    coefficient: f32,
) {
    let neighbor = if lower && i > 0 {
        Some((i - 1, j, k))
    } else if !lower && i + 1 < l {
        Some((i + 1, j, k))
    } else {
        match (boundary, lower) {
            (TransverseBoundary::Periodic, true) => Some((l - 1, j, k)),
            (TransverseBoundary::Periodic, false) => Some((0, j, k)),
            (TransverseBoundary::ReflectBoth | TransverseBoundary::ReflectLowerZeroUpper, true) => {
                Some((1, j, k))
            }
            (
                TransverseBoundary::ReflectBoth | TransverseBoundary::ZeroLowerReflectUpper,
                false,
            ) => Some((l - 2, j, k)),
            _ => None,
        }
    };
    if let Some((ni, nj, nk)) = neighbor {
        matrix[row][linear_index(ni, nj, nk, l, m)] += coefficient;
    }
}

fn add_second_matrix_neighbor(
    matrix: &mut [Vec<f32>],
    row: usize,
    boundary: TransverseBoundary,
    i: usize,
    j: usize,
    k: usize,
    l: usize,
    m: usize,
    lower: bool,
    coefficient: f32,
) {
    let neighbor = if lower && j > 0 {
        Some((i, j - 1, k))
    } else if !lower && j + 1 < m {
        Some((i, j + 1, k))
    } else {
        match (boundary, lower) {
            (TransverseBoundary::Periodic, true) => Some((i, m - 1, k)),
            (TransverseBoundary::Periodic, false) => Some((i, 0, k)),
            (TransverseBoundary::ReflectBoth | TransverseBoundary::ReflectLowerZeroUpper, true) => {
                Some((i, 1, k))
            }
            (
                TransverseBoundary::ReflectBoth | TransverseBoundary::ZeroLowerReflectUpper,
                false,
            ) => Some((i, m - 2, k)),
            _ => None,
        }
    };
    if let Some((ni, nj, nk)) = neighbor {
        matrix[row][linear_index(ni, nj, nk, l, m)] += coefficient;
    }
}

fn dense_oracle(
    rhs: &Grid3,
    l_boundary: TransverseBoundary,
    m_boundary: TransverseBoundary,
    c1: f32,
    c2: f32,
    operator: &ThirdAxisOperator,
) -> Grid3 {
    let (l, m, n) = (rhs.nx(), rhs.ny(), rhs.nz());
    let count = l * m * n;
    let (lower, diagonal, upper) = coefficients(operator, n);
    let mut matrix = vec![vec![0.0; count]; count];
    let mut result = rhs.values().to_vec();
    for k in 0..n {
        for j in 0..m {
            for i in 0..l {
                let row = linear_index(i, j, k, l, m);
                matrix[row][row] += -2.0 * c1 - 2.0 * c2 + diagonal[k];
                add_first_matrix_neighbor(&mut matrix, row, l_boundary, i, j, k, l, m, true, c1);
                add_first_matrix_neighbor(&mut matrix, row, l_boundary, i, j, k, l, m, false, c1);
                add_second_matrix_neighbor(&mut matrix, row, m_boundary, i, j, k, l, m, true, c2);
                add_second_matrix_neighbor(&mut matrix, row, m_boundary, i, j, k, l, m, false, c2);
                if k > 0 {
                    matrix[row][linear_index(i, j, k - 1, l, m)] += lower[k];
                } else if matches!(operator, ThirdAxisOperator::Cyclic(_)) {
                    matrix[row][linear_index(i, j, n - 1, l, m)] += lower[k];
                }
                if k + 1 < n {
                    matrix[row][linear_index(i, j, k + 1, l, m)] += upper[k];
                } else if matches!(operator, ThirdAxisOperator::Cyclic(_)) {
                    matrix[row][linear_index(i, j, 0, l, m)] += upper[k];
                }
            }
        }
    }
    for pivot in 0..count {
        let best = (pivot..count)
            .max_by(|&left, &right| {
                matrix[left][pivot]
                    .abs()
                    .total_cmp(&matrix[right][pivot].abs())
            })
            .unwrap();
        matrix.swap(pivot, best);
        result.swap(pivot, best);
        assert!(matrix[pivot][pivot].abs() > 1.0e-6);
        let diagonal = matrix[pivot][pivot];
        for column in pivot..count {
            matrix[pivot][column] /= diagonal;
        }
        result[pivot] /= diagonal;
        for row in 0..count {
            if row == pivot {
                continue;
            }
            let factor = matrix[row][pivot];
            if factor == 0.0 {
                continue;
            }
            for column in pivot..count {
                matrix[row][column] -= factor * matrix[pivot][column];
            }
            result[row] -= factor * result[pivot];
        }
    }
    Grid3::new(l, m, n, result).unwrap()
}

#[test]
fn native_solution_matches_an_independent_dense_system_oracle() {
    let operator = ThirdAxisOperator::Tridiagonal(
        TridiagonalAxisCoefficients::new(
            vec![0.0, 0.4, 0.2, 0.3, 0.25],
            vec![-8.0, -7.5, -8.5, -7.75, -8.25],
            vec![0.2, 0.25, 0.45, 0.2, 0.0],
        )
        .unwrap(),
    );
    let l_boundary = TransverseBoundary::ReflectLowerZeroUpper;
    let m_boundary = TransverseBoundary::ZeroLowerReflectUpper;
    let exact = exact_grid(3, 4, 5);
    let rhs = apply_operator(&exact, l_boundary, m_boundary, 0.75, 1.25, &operator);
    let oracle = dense_oracle(&rhs, l_boundary, m_boundary, 0.75, 1.25, &operator);
    let native = Pois3dProblem::new(l_boundary, m_boundary, 0.75, 1.25, operator, rhs)
        .unwrap()
        .solve()
        .unwrap();
    assert_grid_close(&native, &oracle);
    assert_grid_close(&native, &exact);
}

#[test]
fn raw_contract_reports_the_documented_coefficient_statuses() {
    let l: FortranInteger = 3;
    let m: FortranInteger = 3;
    let n: FortranInteger = 3;
    let lperod = 0;
    let mperod = 0;
    let c1 = 1.0;
    let c2 = 1.0;
    let ldimf = l;
    let mdimf = m;
    let mut f = vec![0.0_f32; 27];
    let mut work = vec![0.0_f32; 128];
    let mut a = vec![1.0_f32, 2.0, 1.0];
    let mut b = vec![-7.0_f32; 3];
    let mut c = vec![1.0_f32; 3];
    let nperod = 0;
    let mut ierror = 0;
    // SAFETY: every scalar and buffer has the reviewed raw ABI extent. The
    // intentionally inconsistent cyclic arrays make POIS3D stop at IERROR=9
    // before attempting a numerical solve.
    unsafe {
        slatec_sys::pde::fishpack::pois3d(
            &lperod,
            &l,
            &c1,
            &mperod,
            &m,
            &c2,
            &nperod,
            &n,
            a.as_mut_ptr(),
            b.as_mut_ptr(),
            c.as_mut_ptr(),
            &ldimf,
            &mdimf,
            f.as_mut_ptr(),
            &mut ierror,
            work.as_mut_ptr(),
        );
    }
    assert_eq!(ierror, 9);

    a.copy_from_slice(&[1.0, 1.0, 1.0]);
    c.copy_from_slice(&[1.0, 1.0, 1.0]);
    let nperod = 1;
    ierror = 0;
    // SAFETY: this call has the same valid storage, but intentionally violates
    // only the documented noncyclic endpoint condition.
    unsafe {
        slatec_sys::pde::fishpack::pois3d(
            &lperod,
            &l,
            &c1,
            &mperod,
            &m,
            &c2,
            &nperod,
            &n,
            a.as_mut_ptr(),
            b.as_mut_ptr(),
            c.as_mut_ptr(),
            &ldimf,
            &mdimf,
            f.as_mut_ptr(),
            &mut ierror,
            work.as_mut_ptr(),
        );
    }
    assert_eq!(ierror, 10);
}

#[test]
fn concurrent_pois3d_solves_are_serialized() {
    let (problem, expected) = problem_for(
        TransverseBoundary::ReflectBoth,
        TransverseBoundary::ZeroBoth,
        ThirdAxisOperator::Cyclic(CyclicAxisCoefficients::new(0.5, -7.0).unwrap()),
    );
    native_serialization_test_support::reset();
    let barrier = Arc::new(Barrier::new(5));
    let mut workers = Vec::new();
    for _ in 0..4 {
        let barrier = Arc::clone(&barrier);
        let problem = problem.clone();
        let expected = expected.clone();
        workers.push(std::thread::spawn(move || {
            barrier.wait();
            assert_grid_close(&problem.solve().unwrap(), &expected);
        }));
    }
    barrier.wait();
    for worker in workers {
        worker.join().unwrap();
    }
    let snapshot = native_serialization_test_support::snapshot();
    assert_eq!(snapshot.active, 0);
    assert_eq!(snapshot.maximum_active, 1);
}

#[cfg(all(feature = "fishpack-cartesian-2d", feature = "fftpack-real"))]
#[test]
fn coexists_under_the_same_lock_with_m1_and_real_fftpack() {
    use slatec::differential_equations::pde::{
        AxisBoundary, CartesianHelmholtz2d, Grid2, UniformAxis,
    };
    use slatec::fftpack::RealFftPlan;

    let (m2, expected) = problem_for(
        TransverseBoundary::Periodic,
        TransverseBoundary::ReflectBoth,
        ThirdAxisOperator::Cyclic(CyclicAxisCoefficients::new(0.5, -7.0).unwrap()),
    );
    let x = UniformAxis::new(0.0, 1.0, 4).unwrap();
    let y = UniformAxis::new(0.0, 1.0, 4).unwrap();
    let m1 = CartesianHelmholtz2d::new(
        x,
        y,
        -1.0,
        Grid2::zeros(5, 5).unwrap(),
        AxisBoundary::Dirichlet {
            lower: vec![0.0; 5],
            upper: vec![0.0; 5],
        },
        AxisBoundary::Dirichlet {
            lower: vec![0.0; 5],
            upper: vec![0.0; 5],
        },
    )
    .unwrap();
    native_serialization_test_support::reset();
    let first = std::thread::spawn(move || assert_grid_close(&m2.solve().unwrap(), &expected));
    let second = std::thread::spawn(move || {
        let solution = m1.solve().unwrap();
        assert!(
            solution
                .values()
                .values()
                .iter()
                .all(|value| value.abs() <= TOLERANCE)
        );
    });
    let third = std::thread::spawn(move || {
        let mut values = [1.0_f32, 0.0, -1.0, 0.0];
        let mut plan = RealFftPlan::new(values.len()).unwrap();
        plan.forward(&mut values).unwrap();
        plan.backward(&mut values).unwrap();
        assert!((values[0] - 4.0).abs() <= TOLERANCE);
    });
    first.join().unwrap();
    second.join().unwrap();
    third.join().unwrap();
    assert_eq!(
        native_serialization_test_support::snapshot().maximum_active,
        1
    );
}

#[cfg(feature = "fftpack-complex")]
#[test]
fn coexists_under_the_same_lock_with_complex_fftpack() {
    use num_complex::Complex32;
    use slatec::transforms::fft::complex::ComplexFftPlan32;

    let (m2, expected) = problem_for(
        TransverseBoundary::ZeroLowerReflectUpper,
        TransverseBoundary::ReflectBoth,
        ThirdAxisOperator::Cyclic(CyclicAxisCoefficients::new(0.5, -7.0).unwrap()),
    );
    native_serialization_test_support::reset();
    let first = std::thread::spawn(move || assert_grid_close(&m2.solve().unwrap(), &expected));
    let second = std::thread::spawn(move || {
        let mut values = [Complex32::new(1.0, 0.0), Complex32::new(0.0, 1.0)];
        let mut plan = ComplexFftPlan32::new(values.len()).unwrap();
        plan.forward(&mut values).unwrap();
        plan.backward(&mut values).unwrap();
        assert!((values[0].re - 2.0).abs() <= TOLERANCE);
    });
    first.join().unwrap();
    second.join().unwrap();
    assert_eq!(
        native_serialization_test_support::snapshot().maximum_active,
        1
    );
}
