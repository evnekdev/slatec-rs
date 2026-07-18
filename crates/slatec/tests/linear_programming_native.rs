#![cfg(feature = "optimization-linear-programming-in-memory-native-tests")]

use std::fs;
use std::process::Command;

use slatec::linear_programming::{
    LinearProgram, LpBound, LpError, LpOptions, LpStatus, SparseColumns,
};

fn bounded_f64() -> LinearProgram<f64> {
    let matrix =
        SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0]).unwrap();
    LinearProgram::<f64>::new(
        vec![1.0, 2.0],
        matrix,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
    )
    .unwrap()
}

#[test]
fn splp_and_dsplp_recompute_known_optimum() {
    let matrix =
        SparseColumns::<f32>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0]).unwrap();
    let single = LinearProgram::<f32>::new(
        vec![1.0, 2.0],
        matrix,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
    )
    .unwrap()
    .solve()
    .unwrap();
    assert_eq!(single.status, LpStatus::Optimal);
    assert!((single.solution.unwrap().objective_value - 1.0).abs() < 1.0e-4);

    let double = bounded_f64().solve().unwrap();
    assert_eq!(double.status, LpStatus::Optimal);
    let solution = double.solution.unwrap();
    assert!((solution.objective_value - 1.0).abs() < 1.0e-10);
    assert!((solution.row_activities[0] - 1.0).abs() < 1.0e-10);
    assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
}

#[test]
fn bounds_equality_infeasible_and_no_finite_solution_are_distinct() {
    let equality_matrix =
        SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0]).unwrap();
    let equality = LinearProgram::<f64>::new(
        vec![1.0, 3.0],
        equality_matrix,
        vec![LpBound::Fixed(3.0)],
        vec![
            LpBound::Range {
                lower: 0.0,
                upper: 5.0,
            },
            LpBound::Fixed(1.0),
        ],
    )
    .unwrap()
    .solve()
    .unwrap();
    assert_eq!(equality.status, LpStatus::Optimal);
    let solution = equality.solution.unwrap();
    assert!((solution.variables[0] - 2.0_f64).abs() < 1.0e-10);
    assert!((solution.row_activities[0] - 3.0).abs() < 1.0e-10);

    let one = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
    let infeasible = LinearProgram::<f64>::new(
        vec![1.0],
        one,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Fixed(0.0)],
    )
    .unwrap()
    .solve()
    .unwrap();
    assert_eq!(infeasible.status, LpStatus::Infeasible);
    assert!(infeasible.solution.is_none());

    // The first column is structurally empty, so minimizing -x is unbounded.
    let empty_column = SparseColumns::<f64>::new(1, 2, vec![0, 0, 1], vec![0], vec![1.0]).unwrap();
    let unbounded = LinearProgram::<f64>::new(
        vec![-1.0, 0.0],
        empty_column,
        vec![LpBound::Fixed(0.0)],
        vec![LpBound::Lower(0.0), LpBound::Fixed(0.0)],
    )
    .unwrap()
    .solve()
    .unwrap();
    assert_eq!(unbounded.status, LpStatus::NoFiniteSolution);
    assert!(unbounded.solution.is_none());
}

#[test]
fn exact_capacity_boundary_and_recovery_are_preflighted() {
    let problem = bounded_f64();
    assert!(
        problem
            .solve_with_options(LpOptions {
                maximum_matrix_entries: Some(2),
            })
            .is_ok()
    );
    assert_eq!(
        problem.solve_with_options(LpOptions {
            maximum_matrix_entries: Some(1),
        }),
        Err(LpError::PagingRequired {
            required_matrix_storage: 2,
            available_matrix_storage: 1,
        })
    );
    assert_eq!(problem.solve().unwrap().status, LpStatus::Optimal);
    assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
}

#[test]
fn independent_threads_succeed_under_global_serialization() {
    let workers = (0..4)
        .map(|_| std::thread::spawn(|| bounded_f64().solve().unwrap().status))
        .collect::<Vec<_>>();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), LpStatus::Optimal);
    }
    assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
}

#[test]
fn solve_creates_no_current_or_temporary_directory_artifact() {
    const CHILD: &str = "SLATEC_LP_NO_FILE_CHILD";
    if std::env::var_os(CHILD).is_some() {
        let current = std::env::current_dir().unwrap();
        let configured_temp = std::env::temp_dir();
        assert_eq!(fs::read_dir(&current).unwrap().count(), 0);
        assert_eq!(fs::read_dir(&configured_temp).unwrap().count(), 0);
        assert_eq!(bounded_f64().solve().unwrap().status, LpStatus::Optimal);
        assert_eq!(fs::read_dir(&current).unwrap().count(), 0);
        assert_eq!(fs::read_dir(&configured_temp).unwrap().count(), 0);
        assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
        return;
    }

    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "slatec-lp-no-files-{}-{unique}",
        std::process::id()
    ));
    let current = root.join("current");
    let configured_temp = root.join("temp");
    fs::create_dir_all(&current).unwrap();
    fs::create_dir(&configured_temp).unwrap();
    let status = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("solve_creates_no_current_or_temporary_directory_artifact")
        .arg("--nocapture")
        .current_dir(&current)
        .env(CHILD, "1")
        .env("TMP", &configured_temp)
        .env("TEMP", &configured_temp)
        .status()
        .unwrap();
    assert!(status.success());
    assert_eq!(fs::read_dir(&current).unwrap().count(), 0);
    assert_eq!(fs::read_dir(&configured_temp).unwrap().count(), 0);
    fs::remove_dir_all(&root).unwrap();
}
