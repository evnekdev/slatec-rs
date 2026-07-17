//! Deterministic compact inventory for reviewed nonlinear least-squares drivers.
//!
//! This generator records interface and validation facts only. It never copies
//! selected Fortran source, compiled objects, native archives, or verbose logs.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const MAX_TOTAL_BYTES: usize = 64 * 1024;

#[derive(Clone, Copy)]
struct Candidate {
    name: &'static str,
    precision: &'static str,
    role: &'static str,
    callback: &'static str,
    workspace: &'static str,
    disposition: &'static str,
    reason: &'static str,
}

const CANDIDATES: &[Candidate] = &[
    Candidate {
        name: "DNLS1E",
        precision: "f64",
        role: "public_easy_driver",
        callback: "FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC); residual_only_when_IOPT_1",
        workspace: "IW[N]; WA[N*(M+5)+M]",
        disposition: "included",
        reason: "reviewed_raw_abi_runtime_profile_native_fit_validation",
    },
    Candidate {
        name: "SNLS1E",
        precision: "f32",
        role: "public_easy_driver",
        callback: "FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC); residual_only_when_IOPT_1",
        workspace: "IW[N]; WA[N*(M+5)+M]",
        disposition: "included",
        reason: "reviewed_raw_abi_runtime_profile_native_fit_validation",
    },
    Candidate {
        name: "DNLS1",
        precision: "f64",
        role: "expert_driver",
        callback: "FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC); IOPT=1_residual_or_IOPT=2_dense_analytic_Jacobian",
        workspace: "FJAC[M*N]; IPVT[N]; DIAG[N]; QTF[N]; WA1[N]; WA2[N]; WA3[N]; WA4[M]",
        disposition: "included",
        reason: "reviewed_raw_abi_runtime_profile_native_expert_fit_validation",
    },
    Candidate {
        name: "SNLS1",
        precision: "f32",
        role: "expert_driver",
        callback: "FCN(IFLAG,M,N,X,FVEC,FJAC,LDFJAC); IOPT=1_residual_or_IOPT=2_dense_analytic_Jacobian",
        workspace: "FJAC[M*N]; IPVT[N]; DIAG[N]; QTF[N]; WA1[N]; WA2[N]; WA3[N]; WA4[M]",
        disposition: "included",
        reason: "reviewed_raw_abi_runtime_profile_native_expert_fit_validation",
    },
    Candidate {
        name: "DCOV",
        precision: "f64",
        role: "covariance_helper",
        callback: "residual_and_optional_analytic_Jacobian",
        workspace: "expert_caller_workspace",
        disposition: "deferred",
        reason: "covariance_estimation_requires_separate_statistical_contract_review",
    },
    Candidate {
        name: "SCOV",
        precision: "f32",
        role: "covariance_helper",
        callback: "residual_and_optional_analytic_Jacobian",
        workspace: "expert_caller_workspace",
        disposition: "deferred",
        reason: "covariance_estimation_requires_separate_statistical_contract_review",
    },
    Candidate {
        name: "DFDJC3",
        precision: "f64",
        role: "finite_difference_subsidiary",
        callback: "residual_callback",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_DNLS1_IOPT_1_dependency_closure",
    },
    Candidate {
        name: "FDJAC3",
        precision: "f32",
        role: "finite_difference_subsidiary",
        callback: "residual_callback",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_SNLS1_IOPT_1_dependency_closure",
    },
    Candidate {
        name: "DMPAR",
        precision: "f64",
        role: "Levenberg_Marquardt_parameter_subsidiary",
        callback: "none",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_DNLS1_dependency_closure",
    },
    Candidate {
        name: "LMPAR",
        precision: "f32",
        role: "Levenberg_Marquardt_parameter_subsidiary",
        callback: "none",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_SNLS1_dependency_closure",
    },
    Candidate {
        name: "DQRFAC",
        precision: "f64",
        role: "QR_factorization_subsidiary",
        callback: "none",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_DNLS1_dependency_closure",
    },
    Candidate {
        name: "QRFAC",
        precision: "f32",
        role: "QR_factorization_subsidiary",
        callback: "none",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_SNLS1_dependency_closure",
    },
    Candidate {
        name: "DQRSLV",
        precision: "f64",
        role: "QR_solve_subsidiary",
        callback: "none",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_DMPAR_dependency_closure",
    },
    Candidate {
        name: "QRSOLV",
        precision: "f32",
        role: "QR_solve_subsidiary",
        callback: "none",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_LMPAR_dependency_closure",
    },
    Candidate {
        name: "DWUPDT",
        precision: "f64",
        role: "rank_one_update_subsidiary",
        callback: "none",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_DNLS1_dependency_closure",
    },
    Candidate {
        name: "RWUPDT",
        precision: "f32",
        role: "rank_one_update_subsidiary",
        callback: "none",
        workspace: "native_internal",
        disposition: "subsidiary",
        reason: "selected_by_SNLS1_dependency_closure",
    },
    Candidate {
        name: "DENORM",
        precision: "f64",
        role: "Euclidean_norm_subsidiary",
        callback: "none",
        workspace: "none",
        disposition: "subsidiary",
        reason: "selected_by_double_precision_least_squares_dependency_closure",
    },
    Candidate {
        name: "ENORM",
        precision: "f32",
        role: "Euclidean_norm_subsidiary",
        callback: "none",
        workspace: "none",
        disposition: "subsidiary",
        reason: "selected_by_single_precision_least_squares_dependency_closure",
    },
];

/// Result from generating nonlinear least-squares safe-API evidence.
#[derive(Debug)]
pub struct SafeLeastSquaresResult {
    /// Selected complete-corpus snapshot identity.
    pub snapshot_id: String,
    /// Stable hash of all compact generated outputs.
    pub semantic_hash: String,
    /// Number of classified public and subsidiary records.
    pub candidate_count: usize,
    /// Number of included safe functions.
    pub wrapper_count: usize,
    /// Number of explicitly deferred public candidates.
    pub deferred_count: usize,
}

/// Generates compact nonlinear least-squares metadata from committed evidence.
pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<SafeLeastSquaresResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-least-squares-api requires --offline".to_owned(),
        ));
    }
    let runtime = read(&runtime_profile_dir.join("manifest.json"))?;
    require_runtime_gate(&runtime)?;
    let inventory = read(&ffi_dir.join("interface-inventory.json"))?;
    let selected = read(&selected_corpus_dir.join("manifest.json"))?;
    let snapshot = string(&runtime, "snapshot_id")?;
    if string(&inventory, "snapshot_id")? != snapshot
        || string(&selected, "snapshot_id")? != snapshot
    {
        return Err(CorpusError::Verification(
            "least-squares runtime, FFI, and selected-corpus snapshots differ".to_owned(),
        ));
    }
    let columns = inventory["columns"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("raw FFI inventory lacks columns".to_owned()))?;
    let records = inventory["records"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("raw FFI inventory lacks records".to_owned()))?;
    let index = |name: &str| -> Result<usize> {
        columns
            .iter()
            .position(|value| value.as_str() == Some(name))
            .ok_or_else(|| CorpusError::Verification(format!("raw FFI inventory lacks {name}")))
    };
    let (id_ix, name_ix, subset_ix, path_ix, symbol_ix) = (
        index("program_unit_id")?,
        index("normalized_name")?,
        index("source_subset")?,
        index("source_path")?,
        index("observed_raw_symbol")?,
    );
    let available = records
        .iter()
        .filter_map(Value::as_array)
        .filter_map(|row| Some((row.get(name_ix)?.as_str()?, row)))
        .collect::<BTreeMap<_, _>>();

    let mut candidates = Vec::new();
    let mut wrappers = Vec::new();
    let mut deferred = Vec::new();
    let mut seen = BTreeSet::new();
    for candidate in CANDIDATES {
        let row = available.get(candidate.name).ok_or_else(|| {
            CorpusError::Verification(format!(
                "least-squares candidate {} is absent from the selected FFI inventory",
                candidate.name
            ))
        })?;
        let (id, subset, path, symbol) = (
            value(row, id_ix)?,
            value(row, subset_ix)?,
            value(row, path_ix)?,
            value(row, symbol_ix)?,
        );
        if id.is_empty() || symbol.is_empty() || !seen.insert(candidate.name) {
            return Err(CorpusError::Verification(format!(
                "least-squares candidate {} has invalid compact identity",
                candidate.name
            )));
        }
        candidates.push(json!([
            candidate.name,
            id,
            symbol,
            subset,
            path,
            candidate.precision,
            candidate.role,
            candidate.callback,
            candidate.workspace,
            candidate.disposition,
            candidate.reason
        ]));
        if candidate.disposition == "included" {
            if candidate.role == "public_easy_driver" {
                let safe_path = if candidate.precision == "f64" {
                    "slatec::least_squares::least_squares"
                } else {
                    "slatec::least_squares::least_squares_f32"
                };
                wrappers.push(json!([
                    safe_path,
                    candidate.name,
                    symbol,
                    id,
                    candidate.precision,
                    "M_residuals_N_parameters_residual_only_callback",
                    "IOPT=1_forward_difference",
                    "TOL_to_FTOL_and_XTOL; GTOL=0; native_MAXFEV=200*(N+1)",
                    "IW[N]; WA[N*(M+5)+M]",
                    "shared_scoped_thread_local_least_squares_slot",
                    "serialized_process_native_lock",
                    "native_execution_passed",
                    "analytic_and_noisy_overdetermined_fits",
                    "included"
                ]));
            } else {
                let (finite_difference_path, analytic_path) = if candidate.precision == "f64" {
                    (
                        "slatec::least_squares::least_squares_expert",
                        "slatec::least_squares::least_squares_with_jacobian",
                    )
                } else {
                    (
                        "slatec::least_squares::least_squares_expert_f32",
                        "slatec::least_squares::least_squares_with_jacobian_f32",
                    )
                };
                for (safe_path, jacobian_policy) in [
                    (finite_difference_path, "IOPT=1_forward_difference"),
                    (analytic_path, "IOPT=2_dense_analytic_Jacobian"),
                ] {
                    wrappers.push(json!([
                        safe_path,
                        candidate.name,
                        symbol,
                        id,
                        candidate.precision,
                        "M_residuals_N_parameters_LDFJAC=M",
                        jacobian_policy,
                        "FTOL_XTOL_GTOL_MAXFEV_EPSFCN_MODE_DIAG_FACTOR_are_checked",
                        "FJAC[M*N]; IPVT[N]; DIAG[N]; QTF[N]; WA1[N]; WA2[N]; WA3[N]; WA4[M]",
                        "shared_scoped_thread_local_expert_least_squares_slot",
                        "serialized_process_native_lock_with_scoped_XGETF_XSETF_level_one_recovery",
                        "native_execution_passed",
                        "overdetermined_linear_and_exponential_fits",
                        "included"
                    ]));
                }
            }
        } else if candidate.disposition == "deferred" {
            deferred.push(json!([
                candidate.name,
                symbol,
                candidate.role,
                candidate.reason,
                "manual_review_required"
            ]));
        }
    }
    candidates.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    wrappers.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    deferred.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    let easy_wrappers = wrappers
        .iter()
        .filter(|row| row[1].as_str().is_some_and(|name| name.ends_with('E')))
        .cloned()
        .collect::<Vec<_>>();
    let expert_wrappers = wrappers
        .iter()
        .filter(|row| {
            row[1]
                .as_str()
                .is_some_and(|name| matches!(name, "DNLS1" | "SNLS1"))
        })
        .cloned()
        .collect::<Vec<_>>();
    let expert_candidates = candidates
        .iter()
        .filter(|row| {
            row[0]
                .as_str()
                .is_some_and(|name| !matches!(name, "DNLS1E" | "SNLS1E"))
        })
        .cloned()
        .collect::<Vec<_>>();
    let mut expert_deferred = deferred
        .iter()
        .filter(|row| {
            row[0]
                .as_str()
                .is_some_and(|name| matches!(name, "DCOV" | "SCOV"))
        })
        .cloned()
        .collect::<Vec<_>>();
    expert_deferred.extend([
        json!([
            "DNLS1/SNLS1 IOPT=3",
            "",
            "row_oriented_analytic_Jacobian_mode",
            "requires_a_distinct_callback_contract_and_callback_order_review",
            "deferred"
        ]),
        json!([
            "DNLS1/SNLS1 NPRINT",
            "",
            "legacy_observer_callback",
            "print_callback_uses_the_mutable_IFLAG_channel_and_needs_a_separate_reentrancy_design",
            "deferred"
        ]),
        json!([
            "DNLS1/SNLS1 negative_IFLAG",
            "",
            "callback_cancellation",
            "contained_profile_probes_show_the_legacy_fatal_error_path_rather_than_a_safe_return",
            "deferred"
        ]),
        json!([
            "SCOV/DCOV policy",
            "",
            "covariance_interpretation",
            "normalization_rank_deficiency_and_confidence_interval_contracts_need_separate_review",
            "deferred"
        ]),
    ]);
    expert_deferred.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    let mut outputs = BTreeMap::new();
    outputs.insert("least-squares-candidate-index.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.candidate-index", "schema_version":"1.0.0", "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","role","callback_signature","workspace_policy","disposition","reason"], "records":candidates,
    }))?);
    outputs.insert("least-squares-easy-wrapper-index.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.easy-wrapper-index", "schema_version":"1.0.0", "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","dimensions","jacobian_policy","tolerance_policy","workspace_formula","callback_policy","runtime_serialization","native_test_status","numerical_reference_type","review_state"], "records":easy_wrappers,
    }))?);
    outputs.insert("least-squares-status-map.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.status-map", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["info","safe_status","meaning","result_policy"], "records":[
            [1,"ConvergedResidual","actual and predicted residual reductions are small","ok"],
            [2,"ConvergedParameters","relative parameter change is small","ok"],
            [3,"ConvergedResidualAndParameters","both residual and parameter tests passed","ok"],
            [4,"ConvergedOrthogonality","residual is orthogonal to Jacobian columns","ok"],
            [5,"MaximumEvaluations","fixed IOPT=1 callback budget 200*(N+1) reached","ok_with_status"],
            [6,"ResidualToleranceTooSmall","working precision prevents further residual reduction","ok_with_status"],
            [7,"ParameterToleranceTooSmall","working precision prevents further parameter improvement","ok_with_status"],
            [8,"not_exposed","easy driver maps INFO=8 to 4 before return","not_returned_by_easy_driver"]
        ],
    }))?);
    outputs.insert("least-squares-callback-policy.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.callback-policy", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["family","callback_context","pointer_policy","panic_policy","non_finite_policy","cancellation_policy","nested_policy","concurrency_policy"],
        "records":[["nonlinear_least_squares_easy","shared_scoped_thread_local_least_squares_slot","M_N_and_disjoint_parameter_residual_ranges_checked_before_slice_construction","caught_before_fortran_unwind; finite_zero_residual_sentinel","first_non_finite_residual_index_recorded; finite_zero_residual_sentinel","not_exposed_negative_IFLAG_reaches_legacy_fatal_path","cross_family_rejected","serialized_process_native_lock_with_scoped_XGETF_XSETF_level_one_recovery"]],
    }))?);
    outputs.insert("least-squares-workspace.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.workspace", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["raw_routine","iopt","integer_workspace","floating_workspace","checked_arithmetic","ownership"],
        "records":[["SNLS1E",1,"N INTEGER elements","N*(M+5)+M f32 elements","checked_add_and_checked_mul","allocated internally"],["DNLS1E",1,"N INTEGER elements","N*(M+5)+M f64 elements","checked_add_and_checked_mul","allocated internally"]],
    }))?);
    outputs.insert("least-squares-deferred.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.deferred", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["raw_routine","raw_symbol","role","reason","review_state"], "records":deferred,
    }))?);
    outputs.insert("least-squares-expert-candidate-index.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.expert-candidate-index", "schema_version":"1.0.0", "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","role","callback_signature","workspace_policy","disposition","reason"], "records":expert_candidates,
    }))?);
    outputs.insert("least-squares-expert-wrapper-index.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.expert-wrapper-index", "schema_version":"1.0.0", "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","dimensions","jacobian_policy","tolerance_policy","workspace_formula","callback_policy","runtime_serialization","native_test_status","numerical_reference_type","review_state"], "records":expert_wrappers,
    }))?);
    outputs.insert("least-squares-expert-status-map.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.expert-status-map", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["info","safe_status","meaning","result_policy"], "records":[
            [1,"ConvergedResidual","actual and predicted residual reductions satisfy FTOL","ok"],
            [2,"ConvergedParameters","relative parameter change satisfies XTOL","ok"],
            [3,"ConvergedResidualAndParameters","both FTOL and XTOL tests passed","ok"],
            [4,"ConvergedOrthogonality","residual is orthogonal to Jacobian columns according to GTOL","ok"],
            [5,"MaximumEvaluations","MAXFEV residual-callback budget reached","ok_with_status"],
            [6,"ResidualToleranceTooSmall","FTOL cannot improve at working precision","ok_with_status"],
            [7,"ParameterToleranceTooSmall","XTOL cannot improve at working precision","ok_with_status"],
            [8,"GradientToleranceTooSmall","GTOL cannot improve at working precision","ok_with_status"]
        ],
    }))?);
    outputs.insert("least-squares-expert-callback-policy.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.expert-callback-policy", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["family","callback_context","roles","pointer_policy","panic_policy","non_finite_policy","cancellation_policy","nested_policy","concurrency_policy"],
        "records":[["nonlinear_least_squares_expert","shared_scoped_thread_local_expert_least_squares_slot","IOPT=1_residual; IOPT=2_dense_analytic_Jacobian","M_N_LDFJAC_and_disjoint_parameter_residual_Jacobian_ranges_checked_before_slice_construction","caught_before_fortran_unwind; finite_zero_output_sentinel","first_non_finite_residual_or_Jacobian_index_recorded; finite_zero_output_sentinel","not_exposed_negative_IFLAG_reaches_legacy_fatal_path","cross_family_rejected","serialized_process_native_lock_with_scoped_XGETF_XSETF_level_one_recovery"]],
    }))?);
    outputs.insert("least-squares-expert-workspace.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.expert-workspace", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["raw_routine","iopt","ldfjac","integer_workspace","floating_workspace","checked_arithmetic","ownership"],
        "records":[["SNLS1",1,"M","IPVT[N]","FJAC[M*N]; DIAG[N]; QTF[N]; WA1[N]; WA2[N]; WA3[N]; WA4[M]","checked_add_and_checked_mul","allocated internally"],["SNLS1",2,"M","IPVT[N]","FJAC[M*N]; DIAG[N]; QTF[N]; WA1[N]; WA2[N]; WA3[N]; WA4[M]","checked_add_and_checked_mul","allocated internally"],["DNLS1",1,"M","IPVT[N]","FJAC[M*N]; DIAG[N]; QTF[N]; WA1[N]; WA2[N]; WA3[N]; WA4[M]","checked_add_and_checked_mul","allocated internally"],["DNLS1",2,"M","IPVT[N]","FJAC[M*N]; DIAG[N]; QTF[N]; WA1[N]; WA2[N]; WA3[N]; WA4[M]","checked_add_and_checked_mul","allocated internally"]],
    }))?);
    outputs.insert("least-squares-expert-scaling.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.expert-scaling", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["safe_scaling","mode","diag_policy","validation","native_effect"],
        "records":[["Automatic",1,"DIAG[N] initialized to one and may be updated by native scaling","none","native internal scaling"],["User(&[T])",2,"caller values copied into DIAG[N]","length=N; finite; strictly_positive","fixed user variable scaling"]],
    }))?);
    outputs.insert("least-squares-expert-deferred.json", compact(&json!({
        "schema_id":"slatec.safe-least-squares.expert-deferred", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["raw_routine","raw_symbol","role","reason","review_state"], "records":expert_deferred,
    }))?);
    let semantic_hash = semantic_hash(&outputs);
    outputs.insert("least-squares-validation-summary.md", format!(
        "# Safe nonlinear least-squares easy-driver validation\n\n- Snapshot: `{snapshot}`\n- Profile: `{PROFILE}`\n- Classified public and subsidiary records: {}\n- Easy safe wrappers: {} (`SNLS1E`, `DNLS1E`)\n- Expert safe wrappers: {} (`SNLS1`, `DNLS1`, finite-difference and dense analytic Jacobian modes)\n- Deferred public routines: {} (`SCOV`, `DCOV`)\n- Dimensions: native `M >= N` is prevalidated; residual and parameter dimensions remain distinct\n- Expert workspace: checked `FJAC[M*N]`, `IPVT[N]`, `DIAG[N]`, `QTF[N]`, `WA1..WA3[N]`, and `WA4[M]` allocations\n- Legacy errors: scoped `XGETF`/`XSETF(0)` lets documented level-one numerical completion statuses return, then restores the prior process-global control\n- State: callback calls serialize; nested callback-based families are rejected\n- Semantic hash: `{semantic_hash}`\n\nThe original SLATEC Fortran routines remain the numerical implementation.\n",
        CANDIDATES.len(), easy_wrappers.len(), expert_wrappers.len(), deferred.len()
    ).into_bytes());
    outputs.insert("least-squares-expert-validation-summary.md", format!(
        "# Safe expert nonlinear least-squares validation\n\n- Snapshot: `{snapshot}`\n- Profile: `{PROFILE}`\n- Expert candidate and subsidiary records: {}\n- Expert safe wrappers: {} (`SNLS1`, `DNLS1`; `IOPT = 1` finite differences and `IOPT = 2` dense analytic Jacobians)\n- Deferred records: {} (`SCOV`/`DCOV`, `IOPT = 3`, observer callbacks, cancellation, and covariance policy)\n- Dimensions: `M >= N`, `LDFJAC = M`, and all `M*N` storage arithmetic are checked\n- Controls: checked `FTOL`, `XTOL`, `GTOL`, `MAXFEV`, `EPSFCN`, `MODE`/`DIAG`, and `FACTOR`; `NPRINT = 0`\n- Counts: native `NFEV` and `NJEV` are retained and checked against contained callback counts\n- State: native calls serialize; panic, non-finite output, and nested callback entry are contained; scoped `XGETF`/`XSETF(0)` is restored on every Rust return path\n- Semantic hash: `{semantic_hash}`\n\nThe original SLATEC Fortran routines remain the numerical implementation.\n",
        expert_candidates.len(), expert_wrappers.len(), expert_deferred.len()
    ).into_bytes());
    let total = outputs.values().map(Vec::len).sum::<usize>();
    if total > MAX_TOTAL_BYTES {
        return Err(CorpusError::Policy(format!(
            "least-squares metadata is {total} bytes"
        )));
    }
    promote(output_dir, &outputs)?;
    Ok(SafeLeastSquaresResult {
        snapshot_id: snapshot,
        semantic_hash,
        candidate_count: CANDIDATES.len(),
        wrapper_count: wrappers.len(),
        deferred_count: deferred.len(),
    })
}

fn require_runtime_gate(runtime: &Value) -> Result<()> {
    for gate in [
        "abi_validated",
        "machine_constants_validated",
        "legacy_error_behavior_validated",
        "fnlib_initialization_validated",
    ] {
        if runtime["validation"][gate].as_bool() != Some(true) {
            return Err(CorpusError::Verification(format!(
                "runtime-profile gate {gate} is not true"
            )));
        }
    }
    Ok(())
}
fn read(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn string(value: &Value, field: &str) -> Result<String> {
    value[field]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("missing {field}")))
}
fn value(row: &[Value], index: usize) -> Result<&str> {
    row.get(index).and_then(Value::as_str).ok_or_else(|| {
        CorpusError::Verification("least-squares inventory field is not a string".to_owned())
    })
}
fn compact(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn semantic_hash(outputs: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut bytes = Vec::new();
    for (name, content) in outputs {
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(content);
        bytes.push(0);
    }
    hash::bytes(&bytes)
}
fn promote(output_dir: &Path, outputs: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    for (name, bytes) in outputs {
        let temporary = output_dir.join(format!(".{name}.tmp"));
        fs::write(&temporary, bytes)?;
        let destination = output_dir.join(name);
        if destination.exists() {
            fs::remove_file(&destination)?;
        }
        fs::rename(temporary, destination)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::CANDIDATES;

    #[test]
    fn easy_and_expert_drivers_are_the_only_public_inclusions() {
        assert_eq!(
            CANDIDATES
                .iter()
                .filter(|candidate| candidate.disposition == "included")
                .count(),
            4
        );
        assert_eq!(
            CANDIDATES
                .iter()
                .filter(|candidate| candidate.disposition == "deferred")
                .count(),
            2
        );
    }
}
