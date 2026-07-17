//! Deterministic metadata for expert nonlinear solvers and Jacobian checking.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const MAX_TOTAL_BYTES: usize = 128 * 1024;

#[derive(Clone, Copy)]
struct Candidate {
    name: &'static str,
    precision: &'static str,
    role: &'static str,
    callback: &'static str,
    jacobian_mode: &'static str,
    arrays: &'static str,
    workspace: &'static str,
    dependencies: &'static str,
    disposition: &'static str,
    reason: &'static str,
}

const CANDIDATES: &[Candidate] = &[
    candidate(
        "SNSQ",
        "f32",
        "expert_driver",
        "FCN_and_JAC",
        "IOPT_1_user_dense_or_IOPT_2_native_finite_difference",
        "X,FVEC,FJAC,DIAG,R,QTF,WA1_to_WA4",
        "FJAC=N*N;R=N*(N+1)/2;DIAG,QTF,WA1_to_WA4=N",
        "R1MACH,R1MPYQ,R1UPDT,DOGLEG,ENORM,FDJAC1,QFORM,QRFAC,XERMSG",
        "included",
        "reviewed_raw_and_native_validation",
    ),
    candidate(
        "DNSQ",
        "f64",
        "expert_driver",
        "FCN_and_JAC",
        "IOPT_1_user_dense_or_IOPT_2_native_finite_difference",
        "X,FVEC,FJAC,DIAG,R,QTF,WA1_to_WA4",
        "FJAC=N*N;R=N*(N+1)/2;DIAG,QTF,WA1_to_WA4=N",
        "D1MACH,D1MPYQ,D1UPDT,DDOGLG,DENORM,DFDJC1,DQFORM,DQRFAC,XERMSG",
        "included",
        "reviewed_raw_and_native_validation",
    ),
    candidate(
        "CHKDER",
        "f32",
        "jacobian_checker",
        "none",
        "two_stage_MODE_1_then_MODE_2",
        "X,FVEC,FJAC,XP,FVECP,ERR",
        "FJAC=N*N;five_N_vectors",
        "R1MACH",
        "included",
        "callback_free_alloc_only_checked_wrapper",
    ),
    candidate(
        "DCKDER",
        "f64",
        "jacobian_checker",
        "none",
        "two_stage_MODE_1_then_MODE_2",
        "X,FVEC,FJAC,XP,FVECP,ERR",
        "FJAC=N*N;five_N_vectors",
        "D1MACH",
        "included",
        "callback_free_alloc_only_checked_wrapper",
    ),
    candidate(
        "SNSQE",
        "f32",
        "easy_driver",
        "FCN_and_unused_JAC",
        "IOPT_2_only_in_safe_easy_API",
        "X,FVEC,WA",
        "(3*N*N+13*N)/2",
        "SNSQ,XERMSG",
        "existing_safe_api",
        "covered_by_nonlinear_easy",
    ),
    candidate(
        "DNSQE",
        "f64",
        "easy_driver",
        "FCN_and_unused_JAC",
        "IOPT_2_only_in_safe_easy_API",
        "X,FVEC,WA",
        "(3*N*N+13*N)/2",
        "DNSQ,XERMSG",
        "existing_safe_api",
        "covered_by_nonlinear_easy",
    ),
    candidate(
        "SOS",
        "f32",
        "nonlinear_least_squares",
        "residual_and_Jacobian",
        "expert_least_squares",
        "dense_Jacobian_and_work_arrays",
        "caller_managed_expert",
        "MINPACK_least_squares_closure",
        "deferred",
        "nonlinear_least_squares_out_of_scope",
    ),
    candidate(
        "DSOS",
        "f64",
        "nonlinear_least_squares",
        "residual_and_Jacobian",
        "expert_least_squares",
        "dense_Jacobian_and_work_arrays",
        "caller_managed_expert",
        "MINPACK_least_squares_closure",
        "deferred",
        "nonlinear_least_squares_out_of_scope",
    ),
];

const SUBSIDIARIES: &[(&str, &str, &str)] = &[
    ("R1MACH", "f32", "machine_precision"),
    ("D1MACH", "f64", "machine_precision"),
    ("R1MPYQ", "f32", "rank_one_Q_update"),
    ("D1MPYQ", "f64", "rank_one_Q_update"),
    ("R1UPDT", "f32", "rank_one_QR_update"),
    ("D1UPDT", "f64", "rank_one_QR_update"),
    ("DOGLEG", "f32", "dogleg_step"),
    ("DDOGLG", "f64", "dogleg_step"),
    ("ENORM", "f32", "euclidean_norm"),
    ("DENORM", "f64", "euclidean_norm"),
    ("FDJAC1", "f32", "finite_difference_Jacobian"),
    ("DFDJC1", "f64", "finite_difference_Jacobian"),
    ("QFORM", "f32", "orthogonal_factor"),
    ("DQFORM", "f64", "orthogonal_factor"),
    ("QRFAC", "f32", "QR_factorization"),
    ("DQRFAC", "f64", "QR_factorization"),
    ("XERMSG", "shared", "legacy_error_reporting"),
];

#[allow(clippy::too_many_arguments)]
const fn candidate(
    name: &'static str,
    precision: &'static str,
    role: &'static str,
    callback: &'static str,
    jacobian_mode: &'static str,
    arrays: &'static str,
    workspace: &'static str,
    dependencies: &'static str,
    disposition: &'static str,
    reason: &'static str,
) -> Candidate {
    Candidate {
        name,
        precision,
        role,
        callback,
        jacobian_mode,
        arrays,
        workspace,
        dependencies,
        disposition,
        reason,
    }
}

/// Result of deterministic expert nonlinear metadata generation.
pub struct SafeNonlinearExpertResult {
    /// Selected complete-corpus snapshot.
    pub snapshot_id: String,
    /// Hash over all compact semantic outputs.
    pub semantic_hash: String,
    /// Top-level and subsidiary candidate count.
    pub candidate_count: usize,
    /// Public safe wrapper count.
    pub wrapper_count: usize,
    /// Explicit deferred item count.
    pub deferred_count: usize,
}

/// Generates compact expert nonlinear and Jacobian-check metadata.
pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<SafeNonlinearExpertResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-nonlinear-expert-api requires --offline".to_owned(),
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
            "expert nonlinear runtime, FFI, and selected-corpus snapshots differ".to_owned(),
        ));
    }
    let columns = inventory["columns"]
        .as_array()
        .ok_or_else(|| bad("columns"))?;
    let records = inventory["records"]
        .as_array()
        .ok_or_else(|| bad("records"))?;
    let index = |name: &str| -> Result<usize> {
        columns
            .iter()
            .position(|value| value.as_str() == Some(name))
            .ok_or_else(|| bad(name))
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
    let identity = |name: &str| -> Result<[String; 5]> {
        let row = available.get(name).ok_or_else(|| {
            CorpusError::Verification(format!("expert nonlinear candidate {name} is absent"))
        })?;
        Ok([
            field(row, id_ix)?.to_owned(),
            field(row, symbol_ix)?.to_owned(),
            field(row, subset_ix)?.to_owned(),
            field(row, path_ix)?.to_owned(),
            name.to_owned(),
        ])
    };

    let mut seen = BTreeSet::new();
    let mut candidate_rows = Vec::new();
    for candidate in CANDIDATES {
        let [id, symbol, subset, path, name] = identity(candidate.name)?;
        if !seen.insert(name.clone()) {
            return Err(bad("duplicate candidate"));
        }
        candidate_rows.push(json!([
            name,
            id,
            symbol,
            subset,
            path,
            candidate.precision,
            candidate.role,
            candidate.callback,
            candidate.jacobian_mode,
            candidate.arrays,
            candidate.workspace,
            candidate.dependencies,
            candidate.disposition,
            candidate.reason
        ]));
    }
    for (name, precision, role) in SUBSIDIARIES {
        let [id, symbol, subset, path, name] = identity(name)?;
        if !seen.insert(name.clone()) {
            return Err(bad("duplicate subsidiary"));
        }
        candidate_rows.push(json!([
            name,
            id,
            symbol,
            subset,
            path,
            precision,
            role,
            "none",
            "subsidiary",
            "internal_arrays",
            "dependency_defined",
            "transitive_expert_dependency",
            "subsidiary",
            "selected_by_compiler_observed_dependency_closure"
        ]));
    }
    candidate_rows.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));

    let wrappers = json!([
        [
            "slatec::nonlinear::solve_system_expert",
            "DNSQ",
            "f64",
            "finite_difference",
            "std",
            "FJAC=N*N;R=N*(N+1)/2;6*N",
            "native_passed"
        ],
        [
            "slatec::nonlinear::solve_system_expert_f32",
            "SNSQ",
            "f32",
            "finite_difference",
            "std",
            "FJAC=N*N;R=N*(N+1)/2;6*N",
            "native_passed"
        ],
        [
            "slatec::nonlinear::solve_system_with_jacobian",
            "DNSQ",
            "f64",
            "user_dense_Jacobian",
            "std",
            "FJAC=N*N;R=N*(N+1)/2;6*N",
            "native_passed"
        ],
        [
            "slatec::nonlinear::solve_system_with_jacobian_f32",
            "SNSQ",
            "f32",
            "user_dense_Jacobian",
            "std",
            "FJAC=N*N;R=N*(N+1)/2;6*N",
            "native_passed"
        ],
    ]);
    let checker_rows = json!([
        [
            "slatec::nonlinear::check_jacobian",
            "DCKDER",
            "f64",
            "alloc",
            "two_stage_MODE_1_MODE_2",
            "N*N+5*N",
            "native_passed",
            "0.5_upstream_heuristic"
        ],
        [
            "slatec::nonlinear::check_jacobian_f32",
            "CHKDER",
            "f32",
            "alloc",
            "two_stage_MODE_1_MODE_2",
            "N*N+5*N",
            "native_passed",
            "0.5_upstream_heuristic"
        ],
    ]);
    let deferred = json!([
        [
            "SOS",
            "nonlinear_least_squares_and_expert_Jacobian_contract_out_of_scope"
        ],
        [
            "DSOS",
            "nonlinear_least_squares_and_expert_Jacobian_contract_out_of_scope"
        ],
        [
            "NPRINT_observer",
            "shared_mutable_callback_control_channel_not_validated"
        ],
        [
            "callback_cancellation",
            "negative_IFLAG_reaches_fatal_legacy_XERROR"
        ],
        ["packed_QR_diagnostics", "no_reviewed_safe_public_use_case"],
        ["persistent_solver", "stateful_lifecycle_not_in_scope"],
        [
            "sparse_user_Jacobian",
            "SNSQ_DNSQ_user_Jacobian_storage_is_dense"
        ],
    ]);

    let mut outputs = BTreeMap::new();
    outputs.insert("nonlinear-expert-candidate-index.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear-expert.candidates","schema_version":"1.0.0",
        "snapshot_id":snapshot,"profile":PROFILE,
        "columns":["routine","program_unit_id","symbol","subset","path","precision","role","callbacks","jacobian_mode","mutable_arrays","workspace","dependencies","disposition","reason"],
        "records":candidate_rows,
    }))?);
    outputs.insert("nonlinear-expert-wrapper-index.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear-expert.wrappers","schema_version":"1.0.0",
        "snapshot_id":snapshot,"columns":["safe_path","raw_routine","precision","jacobian_policy","capability","workspace_formula","native_test"],"records":wrappers,
    }))?);
    outputs.insert("nonlinear-expert-status-map.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear-expert.status","schema_version":"1.0.0","snapshot_id":snapshot,
        "columns":["info","status","meaning","result_policy"],"records":[
            [1,"Converged","relative_iterate_error_at_most_XTOL","ok"],
            [2,"MaximumFunctionEvaluations","NFEV_reached_MAXFEV","ok_with_status"],
            [3,"ToleranceTooSmall","no_further_working_precision_improvement","ok_with_status"],
            [4,"SlowProgressJacobianEvaluations","insufficient_progress_across_five_Jacobian_evaluations","ok_with_status"],
            [5,"SlowProgressIterations","insufficient_progress_across_ten_iterations","ok_with_status"]
        ],
    }))?);
    outputs.insert("nonlinear-expert-callback-policy.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear-expert.callbacks","schema_version":"1.0.0","snapshot_id":snapshot,
        "residual":"checked_N_nonnull_nonoverlap;panic_and_nonfinite_contained_with_finite_sentinel",
        "jacobian":"checked_N_LDFJAC_nonnull_nonoverlap;logical_entries_initialized_NaN_then_required_finite",
        "roles":["residual","jacobian"],"nprint":0,"cancellation":"not_exposed",
        "nested":"all_callback_families_rejected","concurrency":"serialized_process_native_lock",
    }))?);
    outputs.insert("nonlinear-expert-workspace.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear-expert.workspace","schema_version":"1.0.0","snapshot_id":snapshot,
        "records":[
            ["FJAC","N*N","column_major_LDFJAC=N"],["R","N*(N+1)/2","packed_rowwise_native_output"],
            ["DIAG","N","automatic_or_user_copy"],["QTF","N","hidden"],
            ["WA1","N","hidden"],["WA2","N","hidden"],["WA3","N","hidden"],["WA4","N","hidden"]
        ],"checked_total_hidden_elements":"(3*N*N+13*N)/2",
    }))?);
    outputs.insert("nonlinear-jacobian-check-index.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear.jacobian-check","schema_version":"1.0.0","snapshot_id":snapshot,
        "columns":["safe_path","raw_routine","precision","capability","mode_policy","workspace","native_test","score_policy"],"records":checker_rows,
    }))?);
    outputs.insert("nonlinear-expert-deferred.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear-expert.deferred","schema_version":"1.0.0","snapshot_id":snapshot,
        "columns":["item","reason"],"records":deferred,
    }))?);
    let semantic_hash = semantic_hash(&outputs);
    outputs.insert("nonlinear-expert-validation-summary.md", format!(
        "# Expert nonlinear validation\n\n- Snapshot: `{snapshot}`\n- Profile: `{PROFILE}`\n- Expert wrappers: 4 (`SNSQ`/`DNSQ`, finite-difference and dense user Jacobian)\n- Jacobian-check wrappers: 2 (`CHKDER`/`DCKDER`)\n- Candidate and subsidiary records: {}\n- Native status distinctions: `INFO=1..5` retained\n- Evaluation counters: native `NFEV`/`NJEV` cross-checked with contained callback counts\n- Workspace: exact checked `FJAC`, packed `R`, `DIAG`, `QTF`, and four work vectors\n- Cancellation and `NPRINT` observer callbacks: deferred\n- Semantic hash: `{semantic_hash}`\n\nNo numerical algorithm or selected Fortran source is copied into the safe API.\n",
        candidate_rows.len()
    ).into_bytes());
    let total = outputs.values().map(Vec::len).sum::<usize>();
    if total > MAX_TOTAL_BYTES {
        return Err(CorpusError::Policy(format!(
            "expert nonlinear metadata is {total} bytes"
        )));
    }
    promote(output_dir, &outputs)?;
    Ok(SafeNonlinearExpertResult {
        snapshot_id: snapshot,
        semantic_hash,
        candidate_count: CANDIDATES.len() + SUBSIDIARIES.len(),
        wrapper_count: 6,
        deferred_count: deferred.as_array().map_or(0, Vec::len),
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
    serde_json::from_slice(&fs::read(path)?).map_err(Into::into)
}
fn string(value: &Value, name: &str) -> Result<String> {
    value[name]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| bad(name))
}
fn field(row: &[Value], index: usize) -> Result<&str> {
    row.get(index)
        .and_then(Value::as_str)
        .ok_or_else(|| bad("inventory field"))
}
fn bad(name: &str) -> CorpusError {
    CorpusError::Verification(format!("invalid expert nonlinear metadata: {name}"))
}
fn compact(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn semantic_hash(outputs: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut bytes = Vec::new();
    for (name, value) in outputs {
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(value);
        bytes.push(0);
    }
    hash::bytes(&bytes)
}
fn promote(output: &Path, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    fs::create_dir_all(output)?;
    for (name, bytes) in files {
        let temporary = output.join(format!(".{name}.tmp"));
        fs::write(&temporary, bytes)?;
        let destination = output.join(name);
        if destination.exists() {
            fs::remove_file(&destination)?;
        }
        fs::rename(temporary, destination)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_separates_expert_checkers_and_deferred_least_squares() {
        assert_eq!(
            CANDIDATES
                .iter()
                .filter(|item| item.disposition == "included")
                .count(),
            4
        );
        assert!(
            CANDIDATES
                .iter()
                .any(|item| item.name == "DSOS" && item.disposition == "deferred")
        );
        assert!(SUBSIDIARIES.iter().any(|item| item.0 == "DFDJC1"));
    }

    #[test]
    fn committed_inputs_regenerate_identically_in_separate_roots() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let temporary = tempfile::tempdir().unwrap();
        let first = temporary.path().join("first");
        let second = temporary.path().join("second");
        let run = |output: &Path| {
            generate(
                &root.join("generated/runtime-profile"),
                &root.join("generated/ffi"),
                &root.join("generated/selected-corpus"),
                output,
                true,
            )
            .unwrap()
        };
        assert_eq!(run(&first).semantic_hash, run(&second).semantic_hash);
        for name in [
            "nonlinear-expert-candidate-index.json",
            "nonlinear-expert-wrapper-index.json",
            "nonlinear-expert-status-map.json",
            "nonlinear-expert-callback-policy.json",
            "nonlinear-expert-workspace.json",
            "nonlinear-jacobian-check-index.json",
            "nonlinear-expert-deferred.json",
            "nonlinear-expert-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.join(name)).unwrap(),
                fs::read(second.join(name)).unwrap()
            );
        }
    }
}
