//! Deterministic compact evidence for reviewed `WNNLS` and `DWNNLS` wrappers.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Summary returned after generating constrained linear least-squares metadata.
#[derive(Debug)]
pub struct ResultSummary {
    /// Shared selected-corpus snapshot identity.
    pub snapshot_id: String,
    /// Stable hash of generated compact outputs.
    pub semantic_hash: String,
    /// Number of candidate records.
    pub candidate_count: usize,
    /// Number of public safe wrapper records.
    pub wrapper_count: usize,
}

/// Generates compact `WNNLS`/`DWNNLS` inventory and validation metadata.
pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-linear-least-squares-api requires --offline".to_owned(),
        ));
    }
    let runtime = read(&runtime_profile_dir.join("manifest.json"))?;
    for name in [
        "abi_validated",
        "machine_constants_validated",
        "legacy_error_behavior_validated",
        "fnlib_initialization_validated",
    ] {
        if runtime["validation"][name].as_bool() != Some(true) {
            return Err(CorpusError::Verification(format!(
                "linear least-squares runtime profile lacks {name}=true"
            )));
        }
    }
    let inventory = read(&ffi_dir.join("interface-inventory.json"))?;
    let selected = read(&selected_corpus_dir.join("manifest.json"))?;
    let snapshot = string(&runtime, "snapshot_id")?;
    if string(&inventory, "snapshot_id")? != snapshot
        || string(&selected, "snapshot_id")? != snapshot
    {
        return Err(CorpusError::Verification(
            "linear least-squares runtime, FFI, and selected-corpus snapshots differ".to_owned(),
        ));
    }
    let columns = inventory["columns"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("raw FFI inventory lacks columns".to_owned()))?;
    let records = inventory["records"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("raw FFI inventory lacks records".to_owned()))?;
    let index = |name: &str| {
        columns
            .iter()
            .position(|value| value.as_str() == Some(name))
            .ok_or_else(|| CorpusError::Verification(format!("raw FFI inventory lacks {name}")))
    };
    let (name_ix, id_ix, symbol_ix, subset_ix, path_ix) = (
        index("normalized_name")?,
        index("program_unit_id")?,
        index("observed_raw_symbol")?,
        index("source_subset")?,
        index("source_path")?,
    );
    let mut available = BTreeMap::new();
    for row in records.iter().filter_map(Value::as_array) {
        if let Some(name) = row.get(name_ix).and_then(Value::as_str) {
            available.insert(name, row);
        }
    }
    let mut candidates = Vec::new();
    let mut wrappers = Vec::new();
    for (routine, precision, safe_path) in [
        (
            "DWNNLS",
            "f64",
            "slatec::linear_least_squares::solve_nonnegative_least_squares",
        ),
        (
            "WNNLS",
            "f32",
            "slatec::linear_least_squares::solve_nonnegative_least_squares_f32",
        ),
    ] {
        let row = available.get(routine).ok_or_else(|| {
            CorpusError::Verification(format!(
                "linear least-squares candidate {routine} is absent from validated FFI inventory"
            ))
        })?;
        let value = |ix: usize| {
            row.get(ix).and_then(Value::as_str).ok_or_else(|| {
                CorpusError::Verification(format!("invalid FFI record for {routine}"))
            })
        };
        let (id, symbol, subset, path) = (
            value(id_ix)?,
            value(symbol_ix)?,
            value(subset_ix)?,
            value(path_ix)?,
        );
        candidates.push(json!([
            routine,
            id,
            symbol,
            subset,
            path,
            precision,
            "constrained_linear_least_squares_driver",
            "W(MDW,N+1), ME, MA, N, L, PRGOPT, X, RNORM, MODE, IWORK, WORK",
            "included",
            "reviewed_raw_abi_runtime_profile_native_validation"
        ]));
        wrappers.push(json!([
            safe_path,
            routine,
            symbol,
            id,
            precision,
            "E_x_equals_f_and_minimize_norm_Ax_minus_b_with_free_then_nonnegative_native_partition",
            "caller_constraints_stably_permuted_to_native_partition_and_solution_restored",
            "W[MDW*(N+1)]; WORK[ME+MA+5*N]; IWORK[ME+MA+N]",
            "process_global_saved_machine_and_legacy_error_runtime_serialized",
            "native_execution_passed",
            "independent_small_active_set_reference",
            "included"
        ]));
    }
    candidates.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    wrappers.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    let deferred = vec![
        json!([
            "SBOLS",
            "bounded_linear_least_squares",
            "general_bound_model_and_workspace_contract_out_of_scope",
            "deferred"
        ]),
        json!([
            "DBOLS",
            "bounded_linear_least_squares",
            "general_bound_model_and_workspace_contract_out_of_scope",
            "deferred"
        ]),
        json!([
            "SBOCLS",
            "combined_constrained_least_squares",
            "broader_equality_bound_contract_out_of_scope",
            "deferred"
        ]),
        json!([
            "DBOCLS",
            "combined_constrained_least_squares",
            "broader_equality_bound_contract_out_of_scope",
            "deferred"
        ]),
        json!([
            "LSEI",
            "general_equality_inequality_least_squares",
            "inequality_and_workspace_contract_out_of_scope",
            "deferred"
        ]),
        json!([
            "DLSEI",
            "general_equality_inequality_least_squares",
            "inequality_and_workspace_contract_out_of_scope",
            "deferred"
        ]),
        json!([
            "SPLP",
            "linear_programming",
            "not_a_least_squares_problem",
            "deferred"
        ]),
        json!([
            "DSPLP",
            "linear_programming",
            "not_a_least_squares_problem",
            "deferred"
        ]),
    ];
    let files = [
        (
            "linear-least-squares-candidate-index.json",
            json!({"schema_id":"slatec.safe-linear-least-squares.candidate-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","role","interface","review_state","reason"],"records":candidates}),
        ),
        (
            "nonnegative-least-squares-wrapper-index.json",
            json!({"schema_id":"slatec.safe-linear-least-squares.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","mathematical_model","variable_partition","workspace_formula","runtime_policy","native_test_status","numerical_reference_type","review_state"],"records":wrappers}),
        ),
        (
            "nonnegative-least-squares-status-map.json",
            json!({"schema_id":"slatec.safe-linear-least-squares.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["mode","safe_status","meaning","policy"],"records":[[0,"Converged","native active-set iteration completed","ok"],[1,"MaximumIterations","native 3*(N-L) iteration bound reached","ok_with_approximate_solution"],[2,"not_returned_after_safe_validation","usage_or_storage_error","native_contract_violation"]]}),
        ),
        (
            "nonnegative-least-squares-matrix-layout.json",
            json!({"schema_id":"slatec.safe-linear-least-squares.matrix-layout","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_argument","safe_source","layout_or_transformation"],"records":[["W(MDW,N+1)","owned internal buffer","column_major; equality rows first; least-squares rows second; final column RHS"],["L","VariableConstraint slice","stable permutation: free variables first, nonnegative variables last"],["X","owned native solution","inverse permutation restores caller order"]]}),
        ),
        (
            "nonnegative-least-squares-workspace.json",
            json!({"schema_id":"slatec.safe-linear-least-squares.workspace","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["raw_routine","augmented_matrix","floating_workspace","integer_workspace","checked_arithmetic","ownership"],"records":[["WNNLS","(ME+MA)*(N+1) f32","ME+MA+5*N f32","ME+MA+N INTEGER","checked_add_and_checked_mul","allocated internally"],["DWNNLS","(ME+MA)*(N+1) f64","ME+MA+5*N f64","ME+MA+N INTEGER","checked_add_and_checked_mul","allocated internally"]]}),
        ),
        (
            "nonnegative-least-squares-deferred.json",
            json!({"schema_id":"slatec.safe-linear-least-squares.deferred","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["raw_routine","role","reason","review_state"],"records":deferred}),
        ),
    ];
    fs::create_dir_all(output_dir)?;
    let mut bytes = Vec::new();
    for (name, value) in files {
        let encoded = compact(&value)?;
        fs::write(output_dir.join(name), &encoded)?;
        bytes.extend_from_slice(&encoded);
    }
    let summary = format!(
        "# Safe weighted nonnegative linear least squares\n\n- Snapshot: `{snapshot}`\n- Reviewed public wrappers: 2 (`DWNNLS`, `WNNLS`).\n- Contract: exact equality block `E x = f`, least-squares block `min ||A x-b||₂`, with a native free/nonnegative variable partition.\n- Program options: safe wrapper supplies `PRGOPT(1)=1`; WNNLS historical weighting is internal equality handling, not a public user-weight API.\n- Storage: immutable caller matrices are copied to owned column-major `W(MDW,N+1)` before native mutation.\n- Runtime: calls serialize saved machine-constant and legacy-error support for `ffi-profile-gnu-mingw-x86_64`.\n- Deferred: bounded, broader constrained, and linear-programming drivers remain out of scope.\n"
    );
    fs::write(
        output_dir.join("nonnegative-least-squares-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot,
        semantic_hash: hash::bytes(&bytes),
        candidate_count: 2,
        wrapper_count: 2,
    })
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
fn compact(value: &Value) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(value)?)
}
