#![cfg(feature = "fishpack-cartesian-2d-native-tests")]

use std::sync::{Arc, Barrier};

use slatec::differential_equations::pde::{
    AxisBoundary, CartesianHelmholtz2d, Grid2, NativePdeStatus, SolutionUniqueness, UniformAxis,
};
use slatec::native_serialization_test_support;

const TOLERANCE: f32 = 3.0e-4;

fn axes() -> (UniformAxis, UniformAxis) {
    (
        UniformAxis::new(-0.5, 1.0, 6).unwrap(),
        UniformAxis::new(-0.25, 0.75, 5).unwrap(),
    )
}

fn coordinates(axis: UniformAxis, index: usize) -> f32 {
    axis.lower() + index as f32 * axis.spacing()
}

fn grid_from(x: UniformAxis, y: UniformAxis, mut value: impl FnMut(f32, f32) -> f32) -> Grid2 {
    let nx = x.nodes().unwrap();
    let ny = y.nodes().unwrap();
    let mut values = Vec::with_capacity(nx * ny);
    for j in 0..ny {
        for i in 0..nx {
            values.push(value(coordinates(x, i), coordinates(y, j)));
        }
    }
    Grid2::new(nx, ny, values).unwrap()
}

fn dirichlet_problem(coefficient: f32) -> (CartesianHelmholtz2d, Grid2) {
    let (x, y) = axes();
    let solution = |x: f32, y: f32| 1.0 + 2.0 * x * x + 3.0 * y * y + 0.5 * x * y;
    let rhs = grid_from(x, y, |x, y| 10.0 + coefficient * solution(x, y));
    let expected = grid_from(x, y, solution);
    let x_lower = (0..y.nodes().unwrap())
        .map(|j| solution(x.lower(), coordinates(y, j)))
        .collect();
    let x_upper = (0..y.nodes().unwrap())
        .map(|j| solution(x.upper(), coordinates(y, j)))
        .collect();
    let y_lower = (0..x.nodes().unwrap())
        .map(|i| solution(coordinates(x, i), y.lower()))
        .collect();
    let y_upper = (0..x.nodes().unwrap())
        .map(|i| solution(coordinates(x, i), y.upper()))
        .collect();
    (
        CartesianHelmholtz2d::new(
            x,
            y,
            coefficient,
            rhs,
            AxisBoundary::Dirichlet {
                lower: x_lower,
                upper: x_upper,
            },
            AxisBoundary::Dirichlet {
                lower: y_lower,
                upper: y_upper,
            },
        )
        .unwrap(),
        expected,
    )
}

fn assert_grid_close(actual: &Grid2, expected: &Grid2, tolerance: f32) {
    assert_eq!((actual.nx(), actual.ny()), (expected.nx(), expected.ny()));
    for (index, (actual, expected)) in actual.values().iter().zip(expected.values()).enumerate() {
        assert!(
            (actual - expected).abs() <= tolerance,
            "index={index}, actual={actual}, expected={expected}, tolerance={tolerance}"
        );
    }
}

#[test]
fn dirichlet_poisson_matches_an_exact_discrete_quadratic_on_asymmetric_grid() {
    let (problem, expected) = dirichlet_problem(0.0);
    let solution = problem.solve().unwrap();
    assert_eq!(solution.native_status(), NativePdeStatus::Success);
    assert_eq!(solution.uniqueness(), SolutionUniqueness::Unique);
    assert_eq!(solution.perturbation(), None);
    assert_grid_close(solution.values(), &expected, TOLERANCE);
}

#[test]
fn dirichlet_helmholtz_matches_an_exact_discrete_quadratic() {
    let (problem, expected) = dirichlet_problem(-1.25);
    let solution = problem.solve().unwrap();
    assert_eq!(solution.native_status(), NativePdeStatus::Success);
    assert_eq!(solution.uniqueness(), SolutionUniqueness::Unique);
    assert_grid_close(solution.values(), &expected, TOLERANCE);
}

#[test]
fn mixed_value_derivative_conditions_use_increasing_coordinate_derivatives() {
    let (x, y) = axes();
    let value = |x: f32, y: f32| 1.0 + x * x + 2.0 * y * y + 3.0 * x * y;
    let rhs = grid_from(x, y, |_, _| 6.0);
    let expected = grid_from(x, y, value);
    let x_lower = (0..y.nodes().unwrap())
        .map(|j| value(x.lower(), coordinates(y, j)))
        .collect();
    let x_upper_derivative = (0..y.nodes().unwrap())
        .map(|j| 2.0 * x.upper() + 3.0 * coordinates(y, j))
        .collect();
    let y_lower_derivative = (0..x.nodes().unwrap())
        .map(|i| 4.0 * y.lower() + 3.0 * coordinates(x, i))
        .collect();
    let y_upper = (0..x.nodes().unwrap())
        .map(|i| value(coordinates(x, i), y.upper()))
        .collect();
    let solution = CartesianHelmholtz2d::new(
        x,
        y,
        0.0,
        rhs,
        AxisBoundary::DirichletNeumann {
            lower: x_lower,
            upper_derivative: x_upper_derivative,
        },
        AxisBoundary::NeumannDirichlet {
            lower_derivative: y_lower_derivative,
            upper: y_upper,
        },
    )
    .unwrap()
    .solve()
    .unwrap();
    assert_eq!(solution.uniqueness(), SolutionUniqueness::Unique);
    assert_grid_close(solution.values(), &expected, TOLERANCE);
}

#[test]
fn periodic_problem_preserves_layout_and_is_correct_up_to_a_constant() {
    let (x, y) = axes();
    let nx = x.nodes().unwrap();
    let ny = y.nodes().unwrap();
    let unique_x = x.intervals();
    let unique_y = y.intervals();
    let value = |i: usize, j: usize| {
        let x_phase = 2.0 * core::f32::consts::PI * (i % unique_x) as f32 / unique_x as f32;
        let y_phase = 2.0 * core::f32::consts::PI * (j % unique_y) as f32 / unique_y as f32;
        x_phase.sin() + 0.35 * (2.0 * y_phase).cos()
    };
    let mut rhs_values = vec![0.0; nx * ny];
    for j in 0..unique_y {
        for i in 0..unique_x {
            let west = value((i + unique_x - 1) % unique_x, j);
            let east = value((i + 1) % unique_x, j);
            let south = value(i, (j + unique_y - 1) % unique_y);
            let north = value(i, (j + 1) % unique_y);
            rhs_values[j * nx + i] = (west - 2.0 * value(i, j) + east) / x.spacing().powi(2)
                + (south - 2.0 * value(i, j) + north) / y.spacing().powi(2);
        }
    }
    for j in 0..unique_y {
        rhs_values[j * nx + unique_x] = rhs_values[j * nx];
    }
    for i in 0..nx {
        rhs_values[unique_y * nx + i] = rhs_values[i];
    }
    let expected = Grid2::new(
        nx,
        ny,
        (0..ny)
            .flat_map(|j| (0..nx).map(move |i| value(i, j)))
            .collect(),
    )
    .unwrap();
    let solution = CartesianHelmholtz2d::new(
        x,
        y,
        0.0,
        Grid2::new(nx, ny, rhs_values).unwrap(),
        AxisBoundary::Periodic,
        AxisBoundary::Periodic,
    )
    .unwrap()
    .solve()
    .unwrap();
    assert_eq!(
        solution.uniqueness(),
        SolutionUniqueness::DefinedUpToAdditiveConstant
    );
    assert!(solution.perturbation().unwrap().abs() <= TOLERANCE);
    let offset = solution.values()[(0, 0)] - expected[(0, 0)];
    for (actual, expected) in solution.values().values().iter().zip(expected.values()) {
        assert!((actual - expected - offset).abs() <= TOLERANCE);
    }
    for j in 0..ny {
        assert!((solution.values()[(0, j)] - solution.values()[(nx - 1, j)]).abs() <= TOLERANCE);
    }
    for i in 0..nx {
        assert!((solution.values()[(i, 0)] - solution.values()[(i, ny - 1)]).abs() <= TOLERANCE);
    }
}

#[test]
fn incompatible_neumann_poisson_reports_the_native_rhs_perturbation() {
    let (x, y) = axes();
    let rhs = grid_from(x, y, |_, _| 1.0);
    let solution = CartesianHelmholtz2d::new(
        x,
        y,
        0.0,
        rhs,
        AxisBoundary::Neumann {
            lower_derivative: vec![0.0; y.nodes().unwrap()],
            upper_derivative: vec![0.0; y.nodes().unwrap()],
        },
        AxisBoundary::Neumann {
            lower_derivative: vec![0.0; x.nodes().unwrap()],
            upper_derivative: vec![0.0; x.nodes().unwrap()],
        },
    )
    .unwrap()
    .solve()
    .unwrap();
    assert_eq!(
        solution.uniqueness(),
        SolutionUniqueness::DefinedUpToAdditiveConstant
    );
    assert!(solution.rhs_was_perturbed());
    assert!((solution.perturbation().unwrap() - 1.0).abs() <= TOLERANCE);
    assert!(
        solution
            .values()
            .values()
            .iter()
            .all(|value| value.abs() <= TOLERANCE)
    );
}

#[test]
fn positive_coefficient_preserves_hwscrt_warning_status() {
    let (problem, _) = dirichlet_problem(1.0);
    let solution = problem.solve().unwrap();
    assert_eq!(
        solution.native_status(),
        NativePdeStatus::PositiveCoefficientMayNotHaveSolution
    );
    assert_eq!(solution.native_status().code(), 6);
}

#[test]
fn concurrent_solves_share_the_process_global_native_lock() {
    let (problem, expected) = dirichlet_problem(-0.75);
    native_serialization_test_support::reset();
    let barrier = Arc::new(Barrier::new(5));
    let mut workers = Vec::new();
    for _ in 0..4 {
        let barrier = Arc::clone(&barrier);
        let problem = problem.clone();
        let expected = expected.clone();
        workers.push(std::thread::spawn(move || {
            barrier.wait();
            let solution = problem.solve().unwrap();
            assert_grid_close(solution.values(), &expected, TOLERANCE);
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

#[cfg(feature = "fftpack-real")]
#[test]
fn coexists_with_the_existing_real_fftpack_profile() {
    use slatec::fftpack::RealFftPlan;

    let (problem, expected) = dirichlet_problem(-0.5);
    let solution = problem.solve().unwrap();
    assert_grid_close(solution.values(), &expected, TOLERANCE);

    let mut values = [1.0_f32, 0.0, -1.0, 0.0];
    let mut plan = RealFftPlan::new(values.len()).unwrap();
    plan.forward(&mut values).unwrap();
    plan.backward(&mut values).unwrap();
    assert!((values[0] - 4.0).abs() <= TOLERANCE);
    assert!(values[1].abs() <= TOLERANCE);
    assert!((values[2] + 4.0).abs() <= TOLERANCE);
    assert!(values[3].abs() <= TOLERANCE);
}
