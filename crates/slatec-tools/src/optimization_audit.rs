//! Deterministic inventory of reviewed SLATEC optimization candidates.
//!
//! The inventory deliberately separates current public safe families from the
//! remaining SPLP pair. It is audit evidence only: it never emits a binding,
//! source closure, feature, or wrapper record.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

#[derive(Clone, Copy)]
struct Candidate {
    name: &'static str,
    pair: &'static str,
    precision: &'static str,
    family: &'static str,
    role: &'static str,
    model: &'static str,
    derivative: &'static str,
    callback: &'static str,
    state: &'static str,
    io: &'static str,
    status: &'static str,
    workspace: &'static str,
    disposition: &'static str,
    reason: &'static str,
}

// Discovery is corpus-first: each entry is verified against the committed
// complete selected-provider/FFI inventories before it can be emitted. The
// list records all public optimization drivers and adjacent public support
// routines found during the source-prologue search, plus the two misleading
// minimization-named PDE subsidiaries that search also finds.
const CANDIDATES: &[Candidate] = &[
    Candidate {
        name: "SPLP",
        pair: "DSPLP",
        precision: "f32",
        family: "sparse_linear_programming",
        role: "public_driver",
        model: "minimize_c_transpose_x_subject_to_Ax_equals_w_with_bounds_on_x_and_w",
        derivative: "none",
        callback: "USRMT: IFLAG initialization/entry/end protocol; one-based I,J; INDCAT assign_or_accumulate",
        state: "COMMON_LA05DD_and_legacy_XERROR",
        io: "conditional_PRWVIR_paging_to_SOPENM_SCLOSM_direct_access_file_STATUS_KEEP",
        status: "INFO=1 optimum; -1 infeasible; -2 no_finite_solution; -3 both; -4..-29 contract_or_numerical",
        workspace: "LW>=4*N+8*M+LAMAT+LBM; LIW>=N+11*M+LAMAT+2*LBM; defaults LAMAT=4*N+7,LBM=8*M",
        disposition: "deferred",
        reason: "conditional_global_Fortran_unit_paging_has_no_safe_lifecycle_cleanup_or_reliable_io_failure_status",
    },
    Candidate {
        name: "DSPLP",
        pair: "SPLP",
        precision: "f64",
        family: "sparse_linear_programming",
        role: "public_driver",
        model: "minimize_c_transpose_x_subject_to_Ax_equals_w_with_bounds_on_x_and_w",
        derivative: "none",
        callback: "DUSRMT: IFLAG initialization/entry/end protocol; one-based I,J; INDCAT assign_or_accumulate",
        state: "COMMON_LA05DD_and_legacy_XERROR",
        io: "conditional_DPRWVR_paging_to_SOPENM_SCLOSM_direct_access_file_STATUS_KEEP",
        status: "INFO=1 optimum; -1 infeasible; -2 no_finite_solution; -3 both; -4..-29 contract_or_numerical",
        workspace: "LW>=4*N+8*M+LAMAT+LBM; LIW>=N+11*M+LAMAT+2*LBM; defaults LAMAT=4*N+7,LBM=8*M",
        disposition: "deferred",
        reason: "conditional_global_Fortran_unit_paging_has_no_safe_lifecycle_cleanup_or_reliable_io_failure_status",
    },
    Candidate {
        name: "SNLS1",
        pair: "DNLS1",
        precision: "f32",
        family: "nonlinear_least_squares",
        role: "public_driver",
        model: "minimize_half_sum_of_squared_residuals",
        derivative: "finite_difference_or_analytic_dense_jacobian",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none_when_NPRINT_zero",
        status: "INFO=1..8",
        workspace: "FJAC(M,N), four_N_vectors, QTF, IPVT",
        disposition: "already_safe",
        reason: "existing_least_squares_nonlinear_expert_feature",
    },
    Candidate {
        name: "DNLS1",
        pair: "SNLS1",
        precision: "f64",
        family: "nonlinear_least_squares",
        role: "public_driver",
        model: "minimize_half_sum_of_squared_residuals",
        derivative: "finite_difference_or_analytic_dense_jacobian",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none_when_NPRINT_zero",
        status: "INFO=1..8",
        workspace: "FJAC(M,N), four_N_vectors, QTF, IPVT",
        disposition: "already_safe",
        reason: "existing_least_squares_nonlinear_expert_feature",
    },
    Candidate {
        name: "SNLS1E",
        pair: "DNLS1E",
        precision: "f32",
        family: "nonlinear_least_squares",
        role: "public_driver",
        model: "minimize_half_sum_of_squared_residuals",
        derivative: "finite_difference",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none_when_NPRINT_zero",
        status: "INFO=1..4",
        workspace: "IW(N), W=N*(M+5*N)+M",
        disposition: "already_safe",
        reason: "existing_least_squares_nonlinear_easy_feature",
    },
    Candidate {
        name: "DNLS1E",
        pair: "SNLS1E",
        precision: "f64",
        family: "nonlinear_least_squares",
        role: "public_driver",
        model: "minimize_half_sum_of_squared_residuals",
        derivative: "finite_difference",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none_when_NPRINT_zero",
        status: "INFO=1..4",
        workspace: "IW(N), W=N*(M+5*N)+M",
        disposition: "already_safe",
        reason: "existing_least_squares_nonlinear_easy_feature",
    },
    Candidate {
        name: "SCOV",
        pair: "DCOV",
        precision: "f32",
        family: "nonlinear_least_squares_support",
        role: "public_support",
        model: "covariance_after_nonlinear_least_squares",
        derivative: "finite_difference_or_analytic_dense_jacobian",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "INFO=1_or_2",
        workspace: "four_N_vectors_and_R(LDR,N)",
        disposition: "already_safe",
        reason: "existing_least_squares_covariance_feature",
    },
    Candidate {
        name: "DCOV",
        pair: "SCOV",
        precision: "f64",
        family: "nonlinear_least_squares_support",
        role: "public_support",
        model: "covariance_after_nonlinear_least_squares",
        derivative: "finite_difference_or_analytic_dense_jacobian",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "INFO=1_or_2",
        workspace: "four_N_vectors_and_R(LDR,N)",
        disposition: "already_safe",
        reason: "existing_least_squares_covariance_feature",
    },
    Candidate {
        name: "SNSQ",
        pair: "DNSQ",
        precision: "f32",
        family: "nonlinear_equations",
        role: "public_driver",
        model: "solve_F_x_equals_zero_not_objective_minimization",
        derivative: "finite_difference_or_analytic_dense_jacobian",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none_when_NPRINT_zero",
        status: "INFO=1..5",
        workspace: "FJAC(N,N), R(N*(N+1)/2), four_N_vectors",
        disposition: "already_safe",
        reason: "existing_nonlinear_expert_feature_not_an_optimizer",
    },
    Candidate {
        name: "DNSQ",
        pair: "SNSQ",
        precision: "f64",
        family: "nonlinear_equations",
        role: "public_driver",
        model: "solve_F_x_equals_zero_not_objective_minimization",
        derivative: "finite_difference_or_analytic_dense_jacobian",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none_when_NPRINT_zero",
        status: "INFO=1..5",
        workspace: "FJAC(N,N), R(N*(N+1)/2), four_N_vectors",
        disposition: "already_safe",
        reason: "existing_nonlinear_expert_feature_not_an_optimizer",
    },
    Candidate {
        name: "SNSQE",
        pair: "DNSQE",
        precision: "f32",
        family: "nonlinear_equations",
        role: "public_driver",
        model: "solve_F_x_equals_zero_not_objective_minimization",
        derivative: "finite_difference",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none_when_NPRINT_zero",
        status: "INFO=1..5",
        workspace: "IW(N), W=N*(3*N+13)/2",
        disposition: "already_safe",
        reason: "existing_nonlinear_easy_feature_not_an_optimizer",
    },
    Candidate {
        name: "DNSQE",
        pair: "SNSQE",
        precision: "f64",
        family: "nonlinear_equations",
        role: "public_driver",
        model: "solve_F_x_equals_zero_not_objective_minimization",
        derivative: "finite_difference",
        callback: "FCN with mutable IFLAG",
        state: "legacy_XERROR_and_machine_constants",
        io: "none_when_NPRINT_zero",
        status: "INFO=1..5",
        workspace: "IW(N), W=N*(3*N+13)/2",
        disposition: "already_safe",
        reason: "existing_nonlinear_easy_feature_not_an_optimizer",
    },
    Candidate {
        name: "CHKDER",
        pair: "DCKDER",
        precision: "f32",
        family: "derivative_checking",
        role: "public_support",
        model: "check_jacobian_for_nonlinear_equations",
        derivative: "user_jacobian",
        callback: "none",
        state: "none",
        io: "none",
        status: "MODE_protocol",
        workspace: "caller_vectors",
        disposition: "already_safe",
        reason: "existing_nonlinear_jacobian_check_feature",
    },
    Candidate {
        name: "DCKDER",
        pair: "CHKDER",
        precision: "f64",
        family: "derivative_checking",
        role: "public_support",
        model: "check_jacobian_for_nonlinear_equations",
        derivative: "user_jacobian",
        callback: "none",
        state: "none",
        io: "none",
        status: "MODE_protocol",
        workspace: "caller_vectors",
        disposition: "already_safe",
        reason: "existing_nonlinear_jacobian_check_feature",
    },
    Candidate {
        name: "WNNLS",
        pair: "DWNNLS",
        precision: "f32",
        family: "constrained_linear_least_squares",
        role: "public_driver",
        model: "minimize_norm_Ax_minus_b_subject_to_equalities_and_nonnegative_selection",
        derivative: "none",
        callback: "none",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "MODE=0_or_1",
        workspace: "WORK=ME+MA+5*N; IWORK=ME+MA+N",
        disposition: "already_safe",
        reason: "existing_least_squares_linear_nonnegative_feature",
    },
    Candidate {
        name: "DWNNLS",
        pair: "WNNLS",
        precision: "f64",
        family: "constrained_linear_least_squares",
        role: "public_driver",
        model: "minimize_norm_Ax_minus_b_subject_to_equalities_and_nonnegative_selection",
        derivative: "none",
        callback: "none",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "MODE=0_or_1",
        workspace: "WORK=ME+MA+5*N; IWORK=ME+MA+N",
        disposition: "already_safe",
        reason: "existing_least_squares_linear_nonnegative_feature",
    },
    Candidate {
        name: "SBOLS",
        pair: "DBOLS",
        precision: "f32",
        family: "bounded_linear_least_squares",
        role: "public_driver",
        model: "minimize_norm_Ax_minus_b_subject_to_variable_bounds",
        derivative: "none",
        callback: "none",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "MODE>=0_or_-22",
        workspace: "W=M*(N+1); RW=5*N; IW=2*N",
        disposition: "already_safe",
        reason: "existing_least_squares_linear_bounded_feature",
    },
    Candidate {
        name: "DBOLS",
        pair: "SBOLS",
        precision: "f64",
        family: "bounded_linear_least_squares",
        role: "public_driver",
        model: "minimize_norm_Ax_minus_b_subject_to_variable_bounds",
        derivative: "none",
        callback: "none",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "MODE>=0_or_-22",
        workspace: "W=M*(N+1); RW=5*N; IW=2*N",
        disposition: "already_safe",
        reason: "existing_least_squares_linear_bounded_feature",
    },
    Candidate {
        name: "LSEI",
        pair: "DLSEI",
        precision: "f32",
        family: "constrained_linear_least_squares",
        role: "public_driver",
        model: "minimize_norm_Ax_minus_b_subject_to_equalities_and_inequalities",
        derivative: "none",
        callback: "none",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "MODE=0..4",
        workspace: "W=MDW*(N+1); WS and IP source_sized",
        disposition: "already_safe",
        reason: "existing_least_squares_linear_constrained_feature",
    },
    Candidate {
        name: "DLSEI",
        pair: "LSEI",
        precision: "f64",
        family: "constrained_linear_least_squares",
        role: "public_driver",
        model: "minimize_norm_Ax_minus_b_subject_to_equalities_and_inequalities",
        derivative: "none",
        callback: "none",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "MODE=0..4",
        workspace: "W=MDW*(N+1); WS and IP source_sized",
        disposition: "already_safe",
        reason: "existing_least_squares_linear_constrained_feature",
    },
    Candidate {
        name: "SBOCLS",
        pair: "DBOCLS",
        precision: "f32",
        family: "bounded_constrained_linear_least_squares",
        role: "public_driver",
        model: "minimize_norm_Ex_minus_f_subject_to_bounds_on_x_and_Cx",
        derivative: "none",
        callback: "none",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "MODE>=0_or_-22",
        workspace: "W=MDW*(N+MCON+1); RW=6*N+5*MCON; IW=2*(N+MCON)",
        disposition: "already_safe",
        reason: "existing_least_squares_linear_bounded_constrained_feature",
    },
    Candidate {
        name: "DBOCLS",
        pair: "SBOCLS",
        precision: "f64",
        family: "bounded_constrained_linear_least_squares",
        role: "public_driver",
        model: "minimize_norm_Ex_minus_f_subject_to_bounds_on_x_and_Cx",
        derivative: "none",
        callback: "none",
        state: "legacy_XERROR_and_machine_constants",
        io: "none",
        status: "MODE>=0_or_-22",
        workspace: "W=MDW*(N+MCON+1); RW=6*N+5*MCON; IW=2*(N+MCON)",
        disposition: "already_safe",
        reason: "existing_least_squares_linear_bounded_constrained_feature",
    },
    Candidate {
        name: "DOGLEG",
        pair: "DDOGLG",
        precision: "f32",
        family: "trust_region_support",
        role: "subsidiary",
        model: "dogleg_step_for_nonlinear_equations",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "no_public_status",
        workspace: "caller_vectors",
        disposition: "not_public",
        reason: "subsidiary_of_existing_nonlinear_equation_drivers",
    },
    Candidate {
        name: "DDOGLG",
        pair: "DOGLEG",
        precision: "f64",
        family: "trust_region_support",
        role: "subsidiary",
        model: "dogleg_step_for_nonlinear_equations",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "no_public_status",
        workspace: "caller_vectors",
        disposition: "not_public",
        reason: "subsidiary_of_existing_nonlinear_equation_drivers",
    },
    Candidate {
        name: "LMPAR",
        pair: "DMPAR",
        precision: "f32",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "Levenberg_Marquardt_parameter_step",
        derivative: "none",
        callback: "none",
        state: "machine_constants",
        io: "none",
        status: "caller_owned_par_and_error_bound",
        workspace: "QR_and_four_N_vectors_owned_by_caller",
        disposition: "not_public",
        reason: "subsidiary_of_existing_nonlinear_least_squares_drivers",
    },
    Candidate {
        name: "DMPAR",
        pair: "LMPAR",
        precision: "f64",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "Levenberg_Marquardt_parameter_step",
        derivative: "none",
        callback: "none",
        state: "machine_constants",
        io: "none",
        status: "caller_owned_par_and_error_bound",
        workspace: "QR_and_four_N_vectors_owned_by_caller",
        disposition: "not_public",
        reason: "subsidiary_of_existing_nonlinear_least_squares_drivers",
    },
    Candidate {
        name: "QRFAC",
        pair: "DQRFAC",
        precision: "f32",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "pivoted_QR_factorization",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "no_public_status",
        workspace: "RDIAG_ACNORM_WA_and_IPVT_owned_by_caller",
        disposition: "not_public",
        reason: "subsidiary_of_existing_nonlinear_least_squares_drivers",
    },
    Candidate {
        name: "DQRFAC",
        pair: "QRFAC",
        precision: "f64",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "pivoted_QR_factorization",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "no_public_status",
        workspace: "RDIAG_ACNORM_WA_and_IPVT_owned_by_caller",
        disposition: "not_public",
        reason: "subsidiary_of_existing_nonlinear_least_squares_drivers",
    },
    Candidate {
        name: "QRSOLV",
        pair: "none",
        precision: "f32",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "damped_QR_linear_subproblem",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "no_public_status",
        workspace: "WA_and_SDIAG_owned_by_caller",
        disposition: "not_public",
        reason: "single_precision_MINPACK_subsidiary_with_no_standalone_driver_contract",
    },
    Candidate {
        name: "FDJAC3",
        pair: "DFDJC3",
        precision: "f32",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "forward_difference_dense_Jacobian",
        derivative: "finite_difference",
        callback: "FCN with mutable IFLAG",
        state: "machine_constants",
        io: "none",
        status: "callback_IFLAG_only",
        workspace: "WA(N)_and_FJAC(M,N)_owned_by_caller",
        disposition: "not_public",
        reason: "callback_subsidiary_of_existing_nonlinear_least_squares_drivers",
    },
    Candidate {
        name: "DFDJC3",
        pair: "FDJAC3",
        precision: "f64",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "forward_difference_dense_Jacobian",
        derivative: "finite_difference",
        callback: "FCN with mutable IFLAG",
        state: "machine_constants",
        io: "none",
        status: "callback_IFLAG_only",
        workspace: "WA(N)_and_FJAC(M,N)_owned_by_caller",
        disposition: "not_public",
        reason: "callback_subsidiary_of_existing_nonlinear_least_squares_drivers",
    },
    Candidate {
        name: "ENORM",
        pair: "DENORM",
        precision: "f32",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "stable_Euclidean_norm",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "function_result",
        workspace: "input_vector_only",
        disposition: "not_public",
        reason: "numeric_support_not_an_optimization_driver",
    },
    Candidate {
        name: "DENORM",
        pair: "ENORM",
        precision: "f64",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "stable_Euclidean_norm",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "function_result",
        workspace: "input_vector_only",
        disposition: "not_public",
        reason: "numeric_support_not_an_optimization_driver",
    },
    Candidate {
        name: "RWUPDT",
        pair: "DWUPDT",
        precision: "f32",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "QR_rank_one_update",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "no_public_status",
        workspace: "caller_matrices_and_vectors",
        disposition: "not_public",
        reason: "subsidiary_of_existing_nonlinear_least_squares_drivers",
    },
    Candidate {
        name: "DWUPDT",
        pair: "RWUPDT",
        precision: "f64",
        family: "nonlinear_least_squares_support",
        role: "subsidiary",
        model: "QR_rank_one_update",
        derivative: "none",
        callback: "none",
        state: "none",
        io: "none",
        status: "no_public_status",
        workspace: "caller_matrices_and_vectors",
        disposition: "not_public",
        reason: "subsidiary_of_existing_nonlinear_least_squares_drivers",
    },
    Candidate {
        name: "MINSOL",
        pair: "none",
        precision: "f32",
        family: "pde_normalization",
        role: "subsidiary",
        model: "weighted_least_squares_normalization_of_PDE_solution",
        derivative: "none",
        callback: "none",
        state: "COMMON_SPLPCM",
        io: "none",
        status: "no_public_status",
        workspace: "caller_arrays",
        disposition: "not_public",
        reason: "minimization_named_PDE_subsidiary_not_a_general_optimizer",
    },
    Candidate {
        name: "MINSO4",
        pair: "none",
        precision: "f32",
        family: "pde_normalization",
        role: "subsidiary",
        model: "weighted_least_squares_normalization_of_PDE_solution",
        derivative: "none",
        callback: "none",
        state: "COMMON_SPL4",
        io: "none",
        status: "no_public_status",
        workspace: "caller_arrays",
        disposition: "not_public",
        reason: "minimization_named_PDE_subsidiary_not_a_general_optimizer",
    },
];

/// Summary returned after generating the optimization audit.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus snapshot.
    pub snapshot_id: String,
    /// Stable hash of all generated records.
    pub semantic_hash: String,
    /// Number of inventoried records.
    pub candidate_count: usize,
    /// The sole recommendation classification.
    pub recommendation: &'static str,
}

/// Generates the audit-only optimization inventory.
pub fn generate(
    ffi_dir: &Path,
    selected_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-optimization-audit requires --offline".to_owned(),
        ));
    }
    let inventory = read(&ffi_dir.join("interface-inventory.json"))?;
    let selected = read(&selected_dir.join("selected-providers.json"))?;
    let snapshot = string(&selected, "snapshot_id")?;
    let records = ffi_records(&inventory)?;
    validate(&records)?;
    let mut candidates = Vec::new();
    for candidate in CANDIDATES {
        let row = records.get(candidate.name).ok_or_else(|| {
            CorpusError::Verification(format!(
                "optimization candidate {} missing from FFI inventory",
                candidate.name
            ))
        })?;
        candidates.push(json!([
            candidate.name,
            candidate.pair,
            candidate.precision,
            row.0,
            row.1,
            row.2,
            candidate.family,
            candidate.role,
            candidate.model,
            candidate.derivative,
            candidate.callback,
            candidate.disposition,
            candidate.reason
        ]));
    }
    candidates.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    let families = family_rows();
    let callbacks = CANDIDATES
        .iter()
        .filter(|c| c.callback != "none")
        .map(|c| {
            json!([
                c.name,
                c.callback,
                "panic_must_be_caught_before_crossing_Fortran_boundary",
                if c.disposition == "already_safe" {
                    "contained_by_existing_wrapper"
                } else {
                    "not_exposed"
                }
            ])
        })
        .collect::<Vec<_>>();
    let globals = CANDIDATES
        .iter()
        .filter(|c| c.state != "none")
        .map(|c| {
            json!([
                c.name,
                c.state,
                "serialize_with_native_runtime_lock",
                if c.name == "SPLP" || c.name == "DSPLP" {
                    "lock_insufficient_due_to_external_file"
                } else {
                    "existing_wrapper_scope_or_not_public"
                }
            ])
        })
        .collect::<Vec<_>>();
    let io = CANDIDATES
        .iter()
        .filter(|c| c.io != "none")
        .map(|c| {
            json!([
                c.name,
                c.io,
                if c.name == "SPLP" || c.name == "DSPLP" {
                    "unsafe_conditional_global_Fortran_unit_paging"
                } else {
                    "controlled_by_existing_wrapper"
                }
            ])
        })
        .collect::<Vec<_>>();
    let status = CANDIDATES
        .iter()
        .map(|c| {
            json!([
                c.name,
                c.status,
                if c.disposition == "deferred" {
                    "not_exposed"
                } else {
                    c.disposition
                }
            ])
        })
        .collect::<Vec<_>>();
    let workspace = CANDIDATES
        .iter()
        .map(|c| {
            json!([
                c.name,
                c.workspace,
                if c.name == "SPLP" || c.name == "DSPLP" {
                    "source_formula_known_but_not_safe_due_to_IO"
                } else {
                    "existing_or_subsidiary"
                }
            ])
        })
        .collect::<Vec<_>>();
    let closure = vec![
        json!([
            "SPLP/DSPLP",
            "SPLPMN/DPLPMN; paging; LA05; XERROR; machine constants",
            "large; conditional external paging and global unit lifecycle; no source closure added",
            "deferred"
        ]),
        json!([
            "SNLS1/DNLS1 and easy/covariance variants",
            "MINPACK-derived LMPAR/DMPAR, QRFAC/DQRFAC, QRSOLV, FDJAC3/DFDJC3, RWUPDT/DWUPDT, ENORM/DENORM, trust-region, XERROR",
            "existing reviewed closures",
            "already_safe"
        ]),
        json!([
            "linear least-squares drivers",
            "dense active-set/elimination and XERROR subsidiaries",
            "existing reviewed closures",
            "already_safe"
        ]),
        json!([
            "DOGLEG/DDOGLG and MINSOL/MINSO4",
            "nonlinear/PDE parents",
            "not standalone public closures",
            "not_public"
        ]),
    ];
    let selection = json!({"schema_id":"slatec.optimization.selection","schema_version":"1.1.0","snapshot_id":snapshot,"recommendation":"no_optimization_family_yet","recommended_routines":[],"reason":"the selected corpus has no standalone scalar-minimization, bounded-multivariate-minimization, or nonlinear-programming driver; the only remaining public general optimizer is SPLP/DSPLP, whose conditional paging uses a global Fortran unit without a safe lifecycle or reliable recovery contract","future_requirements":["do_not_substitute_an_IO_shim_without_a_new_source-contract_decision","retain_existing_callback_runtime_lock_and_XERROR_scopes","use_slices_or_lightweight_views_and_owned_native_mutable_storage","keep_ecosystem_adapters_optional_and_separate"]});
    let files = [
        (
            "optimization-candidate-index.json",
            json!({"schema_id":"slatec.optimization.candidate-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"discovery":"selected-corpus public/provider inventory cross-checked with source-prologue optimization/minimization terms and existing safe-family inventory","columns":["routine","precision_pair","precision","program_unit_id","source_subset","source_path","family","role","mathematical_problem","derivative_requirement","callback_protocol","disposition","reason"],"records":candidates}),
        ),
        (
            "optimization-family-index.json",
            json!({"schema_id":"slatec.optimization.family-index","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["family","routines","public_status","suitability"],"records":families}),
        ),
        (
            "optimization-callback-audit.json",
            json!({"schema_id":"slatec.optimization.callback-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","callback_abi_and_control","panic_policy","public_policy"],"records":callbacks}),
        ),
        (
            "optimization-global-state-audit.json",
            json!({"schema_id":"slatec.optimization.global-state-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","state","serialization","result"],"records":globals}),
        ),
        (
            "optimization-io-audit.json",
            json!({"schema_id":"slatec.optimization.io-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","io_or_printing","safety_result"],"records":io}),
        ),
        (
            "optimization-status-audit.json",
            json!({"schema_id":"slatec.optimization.status-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","native_status_contract","audit_disposition"],"records":status}),
        ),
        (
            "optimization-workspace-audit.json",
            json!({"schema_id":"slatec.optimization.workspace-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","workspace_contract","audit_result"],"records":workspace}),
        ),
        (
            "optimization-source-closure-audit.json",
            json!({"schema_id":"slatec.optimization.source-closure-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["family","closure","result","disposition"],"records":closure}),
        ),
        ("optimization-selection.json", selection),
    ];
    fs::create_dir_all(output_dir)?;
    let mut bytes = Vec::new();
    for (name, value) in files {
        let encoded = serde_json::to_vec(&value)?;
        fs::write(output_dir.join(name), &encoded)?;
        bytes.extend_from_slice(&encoded);
    }
    let summary = format!(
        "# SLATEC optimization-family audit\n\n- Snapshot: `{snapshot}`.\n- Discovery found no standalone scalar-minimization, vector-bound nonlinear-minimization, or nonlinear-programming driver in the selected corpus.\n- Existing safe families cover nonlinear least squares, covariance, nonlinear equation support, derivative checking, and constrained linear least squares.\n- The only remaining public general optimizer is `SPLP`/`DSPLP`; both remain deferred because conditional paging uses a process-global direct-access Fortran unit without a safe lifecycle or recovery contract. The caller is not required to pre-open unit 1; option 54 selects it when paging is activated.\n- Recommendation: **no optimization family yet**. A future wrapper must retain panic-contained callbacks, scoped XERROR control, serialized native calls, and slices/lightweight views with privately owned mutable native storage; optional nalgebra/ndarray/faer adapters must remain separate.\n- No public optimization feature, raw declaration, provider closure, native source, or translated algorithm is added.\n"
    );
    fs::write(
        output_dir.join("optimization-audit-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot,
        semantic_hash: hash::bytes(&bytes),
        candidate_count: CANDIDATES.len(),
        recommendation: "no_optimization_family_yet",
    })
}

fn ffi_records(inventory: &Value) -> Result<BTreeMap<String, (String, String, String)>> {
    let cols = inventory["columns"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("FFI columns missing".to_owned()))?;
    let ix = |field: &str| {
        cols.iter()
            .position(|v| v.as_str() == Some(field))
            .ok_or_else(|| CorpusError::Verification(format!("FFI field {field} missing")))
    };
    let (name, id, subset, path) = (
        ix("normalized_name")?,
        ix("program_unit_id")?,
        ix("source_subset")?,
        ix("source_path")?,
    );
    let rows = inventory["records"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("FFI records missing".to_owned()))?;
    let mut result = BTreeMap::new();
    for row in rows.iter().filter_map(Value::as_array) {
        if let (Some(name), Some(id), Some(subset), Some(path)) = (
            row.get(name).and_then(Value::as_str),
            row.get(id).and_then(Value::as_str),
            row.get(subset).and_then(Value::as_str),
            row.get(path).and_then(Value::as_str),
        ) {
            result.insert(
                name.to_owned(),
                (id.to_owned(), subset.to_owned(), path.to_owned()),
            );
        }
    }
    Ok(result)
}
fn read(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path)?).map_err(Into::into)
}
fn string(value: &Value, field: &str) -> Result<String> {
    value[field]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("missing string {field}")))
}
fn validate(records: &BTreeMap<String, (String, String, String)>) -> Result<()> {
    let mut names = BTreeSet::new();
    for c in CANDIDATES {
        if !names.insert(c.name) {
            return Err(CorpusError::Verification(format!(
                "duplicate optimization candidate {}",
                c.name
            )));
        }
        if !records.contains_key(c.name) {
            return Err(CorpusError::Verification(format!(
                "candidate {} absent from selected FFI inventory",
                c.name
            )));
        }
        if c.pair != "none"
            && CANDIDATES
                .iter()
                .find(|other| other.name == c.pair)
                .map(|other| other.pair)
                != Some(c.name)
        {
            return Err(CorpusError::Verification(format!(
                "unpaired optimization candidate {}",
                c.name
            )));
        }
    }
    Ok(())
}
fn family_rows() -> Vec<Value> {
    vec![
        json!([
            "sparse_linear_programming",
            "SPLP/DSPLP",
            "remaining_public_general_optimizer",
            "unsafe_conditional_global_Fortran_unit_paging"
        ]),
        json!([
            "nonlinear_least_squares",
            "SNLS1/DNLS1; SNLS1E/DNLS1E; SCOV/DCOV",
            "already_safe",
            "not_a_remaining_family"
        ]),
        json!([
            "nonlinear_equations_and_derivative_checking",
            "SNSQ/DNSQ; SNSQE/DNSQE; CHKDER/DCKDER",
            "already_safe",
            "not_objective_minimization"
        ]),
        json!([
            "constrained_linear_least_squares",
            "WNNLS/DWNNLS; SBOLS/DBOLS; LSEI/DLSEI; SBOCLS/DBOCLS",
            "already_safe",
            "not_a_remaining_family"
        ]),
        json!([
            "trust_region_numeric_and_PDE_support",
            "DOGLEG/DDOGLG; LMPAR/DMPAR; QRFAC/DQRFAC; QRSOLV; FDJAC3/DFDJC3; RWUPDT/DWUPDT; ENORM/DENORM; MINSOL/MINSO4",
            "subsidiary_only",
            "not_standalone_optimization"
        ]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    fn root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }
    #[test]
    fn public_drivers_are_classified_and_pairs_are_symmetric() {
        let names = CANDIDATES.iter().map(|c| c.name).collect::<BTreeSet<_>>();
        for name in [
            "SPLP", "DSPLP", "SNLS1", "DNLS1", "SNLS1E", "DNLS1E", "SCOV", "DCOV", "SNSQ", "DNSQ",
            "SNSQE", "DNSQE", "CHKDER", "DCKDER", "WNNLS", "DWNNLS", "SBOLS", "DBOLS", "LSEI",
            "DLSEI", "SBOCLS", "DBOCLS",
        ] {
            assert!(names.contains(name));
        }
        assert!(CANDIDATES.iter().all(|c| {
            c.pair == "none"
                || CANDIDATES
                    .iter()
                    .any(|p| p.name == c.pair && p.pair == c.name)
        }));
    }
    #[test]
    fn lp_is_marked_io_and_not_selected() {
        for name in ["SPLP", "DSPLP"] {
            let c = CANDIDATES.iter().find(|c| c.name == name).unwrap();
            assert!(c.io.contains("conditional"));
            assert!(!c.io.contains("mandatory"));
            assert_eq!(c.disposition, "deferred");
        }
    }
    #[test]
    fn callbacks_and_globals_are_recorded() {
        assert!(
            CANDIDATES
                .iter()
                .filter(|c| c.callback != "none")
                .all(|c| !c.callback.is_empty())
        );
        assert!(CANDIDATES.iter().any(|c| c.state.contains("COMMON_LA05DD")));
    }
    #[test]
    fn generation_is_deterministic() {
        let root = root();
        let one = tempdir().unwrap();
        let two = tempdir().unwrap();
        let a = generate(
            &root.join("generated/ffi"),
            &root.join("generated/selected-corpus"),
            one.path(),
            true,
        )
        .unwrap();
        let b = generate(
            &root.join("generated/ffi"),
            &root.join("generated/selected-corpus"),
            two.path(),
            true,
        )
        .unwrap();
        assert_eq!(a.semantic_hash, b.semantic_hash);
        for name in [
            "optimization-candidate-index.json",
            "optimization-family-index.json",
            "optimization-callback-audit.json",
            "optimization-global-state-audit.json",
            "optimization-io-audit.json",
            "optimization-status-audit.json",
            "optimization-workspace-audit.json",
            "optimization-source-closure-audit.json",
            "optimization-selection.json",
            "optimization-audit-summary.md",
        ] {
            assert_eq!(
                fs::read(one.path().join(name)).unwrap(),
                fs::read(two.path().join(name)).unwrap()
            );
        }
    }
}
