#![cfg(feature = "optimization-linear-programming-in-memory-native-tests")]

use std::fs;
use std::process::Command;

use slatec::linear_programming::{
    LinearProgram, LpBasisPosition, LpBound, LpError, LpNonbasicPosition, LpOptions,
    LpPricingStrategy, LpStatus, LpValidation, SparseColumns,
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
    let single_solution = single.solution.unwrap();
    assert!((single_solution.objective_value - 1.0).abs() < 1.0e-4);
    assert!((single_solution.dual.row_multipliers[0] - 1.0).abs() < 1.0e-4);
    assert!((single_solution.dual.reduced_costs[1] - 1.0).abs() < 1.0e-4);
    assert!(
        single_solution
            .diagnostics
            .maximum_dual_feasibility_violation
            < 1.0e-4
    );

    let double = bounded_f64().solve().unwrap();
    assert_eq!(double.status, LpStatus::Optimal);
    let solution = double.solution.unwrap();
    assert!((solution.objective_value - 1.0).abs() < 1.0e-10);
    assert!((solution.row_activities[0] - 1.0).abs() < 1.0e-10);
    assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
}

#[test]
fn optimal_dual_basis_and_kkt_diagnostics_have_native_meaning() {
    let result = bounded_f64().solve().unwrap();
    let solution = result.solution.unwrap();
    // For min x + 2y, x + y >= 1, x,y >= 0, the native convention is
    // L=c^T x-y^T(Ax-w), so y=1 and c-A^T y=(0,1).
    assert!((solution.dual.row_multipliers[0] - 1.0).abs() < 1.0e-10);
    assert!((solution.dual.reduced_costs[0]).abs() < 1.0e-10);
    assert!((solution.dual.reduced_costs[1] - 1.0).abs() < 1.0e-10);
    assert_eq!(
        solution.basis.variable_positions[1],
        LpBasisPosition::Nonbasic(LpNonbasicPosition::LowerBound)
    );
    assert_eq!(
        solution.basis.row_positions[0],
        LpBasisPosition::Nonbasic(LpNonbasicPosition::LowerBound)
    );
    let diagnostics = solution.diagnostics;
    assert!(diagnostics.maximum_variable_bound_violation < 1.0e-10);
    assert!(diagnostics.maximum_row_bound_violation < 1.0e-10);
    assert!(diagnostics.maximum_row_activity_residual < 1.0e-10);
    assert!(diagnostics.maximum_reduced_cost_residual < 1.0e-10);
    assert!(diagnostics.maximum_dual_feasibility_violation < 1.0e-10);
    assert!(diagnostics.maximum_complementary_slackness_residual < 1.0e-10);
    assert!((diagnostics.dual_objective.unwrap() - 1.0).abs() < 1.0e-10);
    assert!(diagnostics.primal_dual_objective_gap.unwrap() < 1.0e-10);
}

#[test]
fn lower_upper_ranged_fixed_and_free_bound_duals_are_checked() {
    // The empty column lets the upper bound determine the objective. The
    // fixed row remains a valid equation in the native A*x=w formulation.
    let upper_matrix = SparseColumns::<f64>::new(1, 1, vec![0, 0], vec![], vec![]).unwrap();
    let upper = LinearProgram::<f64>::new(
        vec![-1.0],
        upper_matrix,
        vec![LpBound::Fixed(0.0)],
        vec![LpBound::Upper(1.0)],
    )
    .unwrap()
    .solve()
    .unwrap()
    .solution
    .unwrap();
    assert!((upper.variables[0] - 1.0).abs() < 1.0e-10);
    assert!((upper.dual.reduced_costs[0] + 1.0).abs() < 1.0e-10);
    assert_eq!(
        upper.basis.variable_positions[0],
        LpBasisPosition::Nonbasic(LpNonbasicPosition::UpperBound)
    );

    let range_matrix = SparseColumns::<f64>::new(1, 1, vec![0, 0], vec![], vec![]).unwrap();
    let ranged = LinearProgram::<f64>::new(
        vec![-1.0],
        range_matrix,
        vec![LpBound::Free],
        vec![LpBound::Range {
            lower: -2.0,
            upper: 3.0,
        }],
    )
    .unwrap()
    .solve()
    .unwrap()
    .solution
    .unwrap();
    assert!((ranged.variables[0] - 3.0).abs() < 1.0e-10);
    assert_eq!(
        ranged.basis.variable_positions[0],
        LpBasisPosition::Nonbasic(LpNonbasicPosition::UpperBound)
    );

    let fixed_matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
    let fixed = LinearProgram::<f64>::new(
        vec![3.0],
        fixed_matrix,
        vec![LpBound::Fixed(2.0)],
        vec![LpBound::Fixed(2.0)],
    )
    .unwrap()
    .solve()
    .unwrap()
    .solution
    .unwrap();
    assert_eq!(
        fixed.basis.variable_positions[0],
        LpBasisPosition::Nonbasic(LpNonbasicPosition::Fixed)
    );
    assert!(fixed.diagnostics.maximum_dual_feasibility_violation < 1.0e-10);
}

#[test]
fn row_upper_multiplier_and_multiple_optima_satisfy_kkt() {
    let row_upper_matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
    let row_upper = LinearProgram::<f64>::new(
        vec![-1.0],
        row_upper_matrix,
        vec![LpBound::Upper(1.0)],
        vec![LpBound::Free],
    )
    .unwrap()
    .solve()
    .unwrap()
    .solution
    .unwrap();
    assert!((row_upper.row_activities[0] - 1.0).abs() < 1.0e-10);
    assert!(row_upper.dual.row_multipliers[0] < 0.0);

    let multiple_matrix =
        SparseColumns::<f64>::new(1, 2, vec![0, 1, 2], vec![0, 0], vec![1.0, 1.0]).unwrap();
    let multiple = LinearProgram::<f64>::new(
        vec![1.0, 1.0],
        multiple_matrix,
        vec![LpBound::Lower(1.0)],
        vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
    )
    .unwrap()
    .solve()
    .unwrap()
    .solution
    .unwrap();
    assert!((multiple.objective_value - 1.0).abs() < 1.0e-10);
    assert!(multiple.diagnostics.primal_dual_objective_gap.unwrap() < 1.0e-10);
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
                maximum_resident_nonzeros: Some(2),
                ..LpOptions::default()
            })
            .is_ok()
    );
    assert_eq!(
        problem.solve_with_options(LpOptions {
            maximum_resident_nonzeros: Some(1),
            ..LpOptions::default()
        }),
        Err(LpError::PagingRequired {
            required_nonzeros: 2,
            required_lamat: 10,
            maximum_resident_nonzeros: 1,
        })
    );
    assert_eq!(problem.solve().unwrap().status, LpStatus::Optimal);
    assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
}

#[test]
fn reviewed_controls_reject_invalid_values_and_preserve_paging_policy() {
    let problem = bounded_f64();
    assert!(matches!(
        problem.solve_with_options(LpOptions {
            maximum_iterations: Some(0),
            ..LpOptions::default()
        }),
        Err(LpError::InvalidInput(_))
    ));
    assert!(matches!(
        problem.solve_with_options(LpOptions {
            feasibility_tolerance: Some(f64::NAN),
            ..LpOptions::default()
        }),
        Err(LpError::InvalidInput(_))
    ));
    assert!(matches!(
        problem.solve_with_options(LpOptions {
            maximum_iterations: Some(usize::MAX),
            ..LpOptions::default()
        }),
        Err(LpError::InvalidInput(_))
    ));
    let result = problem
        .solve_with_options(LpOptions {
            feasibility_tolerance: Some(1.0e-9),
            absolute_feasibility_tolerance: Some(1.0e-10),
            pricing_strategy: LpPricingStrategy::MinimumReducedCost,
            validation: LpValidation::NativeOnly,
            ..LpOptions::default()
        })
        .unwrap();
    assert_eq!(result.status, LpStatus::Optimal);
    assert_eq!(slatec_src::lp_forbidden_io_entries(), (0, 0, 0));
}

#[test]
fn tiny_iteration_limit_returns_labelled_nonoptimal_progress() {
    // Two independently bounded directions require more than one simplex
    // improvement from the all-lower-bound starting point.
    let matrix =
        SparseColumns::<f64>::new(2, 2, vec![0, 1, 2], vec![0, 1], vec![1.0, 1.0]).unwrap();
    let problem = LinearProgram::<f64>::new(
        vec![-1.0, -1.0],
        matrix,
        vec![LpBound::Upper(1.0), LpBound::Upper(1.0)],
        vec![LpBound::Lower(0.0), LpBound::Lower(0.0)],
    )
    .unwrap();
    let limited = problem
        .solve_with_options(LpOptions {
            maximum_iterations: Some(1),
            validation: LpValidation::NativeOnly,
            ..LpOptions::default()
        })
        .unwrap();
    assert_eq!(limited.status, LpStatus::IterationLimit);
    let progress = limited.progress.unwrap();
    assert!(progress.objective_value <= 0.0);
    assert!(limited.solution.is_none());
    let exact_boundary = problem
        .solve_with_options(LpOptions {
            maximum_iterations: Some(2),
            ..LpOptions::default()
        })
        .unwrap();
    assert_eq!(exact_boundary.status, LpStatus::Optimal);
    assert_eq!(problem.solve().unwrap().status, LpStatus::Optimal);
}

#[test]
fn deterministic_generated_one_variable_oracles_match_primal_and_dual_objectives() {
    for lower in -3_i32..=3 {
        for cost in 1_i32..=3 {
            let matrix = SparseColumns::<f64>::new(1, 1, vec![0, 1], vec![0], vec![1.0]).unwrap();
            let solution = LinearProgram::<f64>::new(
                vec![f64::from(cost)],
                matrix,
                vec![LpBound::Fixed(f64::from(lower))],
                vec![LpBound::Free],
            )
            .unwrap()
            .solve()
            .unwrap()
            .solution
            .unwrap();
            assert!((solution.variables[0] - f64::from(lower)).abs() < 1.0e-10);
            assert!((solution.objective_value - f64::from(cost * lower)).abs() < 1.0e-10);
            assert!(solution.diagnostics.primal_dual_objective_gap.unwrap() < 1.0e-10);
        }
    }
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
