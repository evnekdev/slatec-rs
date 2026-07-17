//! Deterministic metadata for reviewed `LSEI` and `DLSEI` safe wrappers.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Concise result from constrained linear least-squares metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus snapshot used by all emitted records.
    pub snapshot_id: String,
    /// Hash over the compact deterministic outputs.
    pub semantic_hash: String,
    /// Reviewed public candidate count.
    pub candidate_count: usize,
    /// Public safe wrapper count.
    pub wrapper_count: usize,
    /// Explicitly deferred driver count.
    pub deferred_count: usize,
}

/// Generates metadata only when the matching runtime and raw-FFI snapshots are
/// validated and the caller requested offline operation.
pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-constrained-linear-least-squares-api requires --offline".to_owned(),
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
                "constrained linear least-squares runtime profile lacks {name}=true"
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
            "constrained least-squares runtime, FFI, and selected-corpus snapshots differ"
                .to_owned(),
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
    for (routine, precision, path) in [
        (
            "DLSEI",
            "f64",
            "slatec::constrained_least_squares::solve_constrained_least_squares",
        ),
        (
            "LSEI",
            "f32",
            "slatec::constrained_least_squares::solve_constrained_least_squares_f32",
        ),
    ] {
        let row = available.get(routine).ok_or_else(|| {
            CorpusError::Verification(format!("constrained least-squares candidate {routine} is absent from validated FFI inventory"))
        })?;
        let value = |ix: usize| {
            row.get(ix).and_then(Value::as_str).ok_or_else(|| {
                CorpusError::Verification(format!("invalid FFI record for {routine}"))
            })
        };
        let (id, symbol, subset, source) = (
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
            source,
            precision,
            "general_equality_inequality_constrained_linear_least_squares_driver",
            "W(MDW,N+1), ME, MA, MG, N, PRGOPT, X, RNORME, RNORML, MODE, WS, IP",
            "reviewed_raw_abi_runtime_profile_native_validation",
            "included"
        ]));
        wrappers.push(json!([
            path,
            routine,
            symbol,
            id,
            precision,
            "minimize_norm_Ax_minus_b_subject_to_Ex_equals_f_and_Gx_greater_equal_h",
            "owned_column_major_W_with_rows_E_then_A_then_G",
            "WS=2*(ME+N)+max(MA+MG,N)+(MG+2)*(N+7);IP=MG+2*N+2",
            "shared_native_lock_and_scoped_XERROR_restore",
            "analytic_small_KKT_and_active_set_reference",
            "included"
        ]));
    }
    candidates.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    wrappers.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    let deferred = vec![
        json!([
            "SBOCLS",
            "combined_bound_and_equality_driver",
            "simultaneous_variable_bounds_out_of_scope",
            "deferred"
        ]),
        json!([
            "DBOCLS",
            "combined_bound_and_equality_driver",
            "simultaneous_variable_bounds_out_of_scope",
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
        json!([
            "LSEI_covariance_option",
            "constrained_covariance",
            "separate_statistical_scaling_and_layout_review_required",
            "deferred"
        ]),
        json!([
            "LSEI_active_set_and_duals",
            "native_internal_state",
            "no_documented_public_output_contract",
            "deferred"
        ]),
    ];
    let files = [
        (
            "constrained-least-squares-candidate-index.json",
            json!({"schema_id":"slatec.safe-constrained-least-squares.candidate-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","role","interface","review_state","inclusion"],"records":candidates}),
        ),
        (
            "constrained-least-squares-wrapper-index.json",
            json!({"schema_id":"slatec.safe-constrained-least-squares.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","mathematical_model","block_layout","workspace_formula","runtime_policy","numerical_reference_type","review_state"],"records":wrappers}),
        ),
        (
            "constrained-least-squares-status-map.json",
            json!({"schema_id":"slatec.safe-constrained-least-squares.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_mode","safe_status_or_error","meaning","policy"],"records":[[0,"Converged","all equality and inequality constraints compatible","ok"],[1,"EqualityConstraintsContradictory","generalized-inverse equality residual is defined","ok_with_structured_status"],[2,"InequalityConstraintsInfeasible","no native solution outputs defined","error"],[3,"ConstraintsInfeasible","equality and inequality sets have no common point","error"],[4,"NativeContractViolation","usage/options/workspace error","error"]]}),
        ),
        (
            "constrained-least-squares-block-layout.json",
            json!({"schema_id":"slatec.safe-constrained-least-squares.block-layout","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_rows","safe_input","relation","transformation"],"records":[["1..ME","equalities","E*x=f","copied first into W"],["ME+1..ME+MA","objective","A*x approximately equals b","copied second into W"],["ME+MA+1..ME+MA+MG","inequalities","G*x>=h","copied last into W without sign reversal"],["N+1","all_rhs","F,B,H","owned final W column"]]}),
        ),
        (
            "constrained-least-squares-options.json",
            json!({"schema_id":"slatec.safe-constrained-least-squares.options","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_control","native_PRGOPT","policy"],"records":[["rank_tolerance=None","[1]","source default sqrt(machine_epsilon)"],["rank_tolerance=Some(t)","[4,4,t,7,5,t,1]","finite nonnegative t; source clamps to machine epsilon"],["covariance/scaling/raw_option_links","not_exposed","deferred"]]}),
        ),
        (
            "constrained-least-squares-workspace.json",
            json!({"schema_id":"slatec.safe-constrained-least-squares.workspace","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["raw_routine","augmented_matrix","floating_workspace","integer_workspace","ownership"],"records":[["LSEI","(ME+MA+MG)*(N+1) f32","2*(ME+N)+max(MA+MG,N)+(MG+2)*(N+7)","MG+2*N+2 INTEGER","allocated_and_checked_internally"],["DLSEI","(ME+MA+MG)*(N+1) f64","2*(ME+N)+max(MA+MG,N)+(MG+2)*(N+7)","MG+2*N+2 INTEGER","allocated_and_checked_internally"]]}),
        ),
        (
            "constrained-least-squares-deferred.json",
            json!({"schema_id":"slatec.safe-constrained-least-squares.deferred","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["item","role","reason","review_state"],"records":deferred}),
        ),
    ];
    fs::create_dir_all(output_dir)?;
    let mut bytes = Vec::new();
    for (name, value) in files {
        let encoded = serde_json::to_vec(&value)?;
        fs::write(output_dir.join(name), &encoded)?;
        bytes.extend_from_slice(&encoded);
    }
    let summary = format!(
        "# Safe equality and inequality constrained linear least squares\n\n- Snapshot: `{snapshot}`.\n- Reviewed wrappers: `DLSEI` (f64) and `LSEI` (f32).\n- Contract: minimize `||A x-b||₂` subject to `E x=f` and `G x≥h`; native rows are copied in that order.\n- Status: MODE 0 and 1 yield defined results; MODE 2 and 3 are structured infeasibility errors; MODE 4 is a contained contract violation.\n- Ranks: `IP(1)` is equality rank and `IP(2)` is reduced-objective rank.\n- Deferred: covariance option, combined bounds, duals, active-set state, linear programming, sparse/warm-start APIs.\n"
    );
    fs::write(
        output_dir.join("constrained-least-squares-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot,
        semantic_hash: hash::bytes(&bytes),
        candidate_count: 2,
        wrapper_count: 2,
        deferred_count: 6,
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
