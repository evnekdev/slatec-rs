//! Deterministic compact evidence for reviewed `SBOLS` and `DBOLS` wrappers.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Summary returned after generating bounded linear least-squares metadata.
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
    /// Number of explicit deferral records.
    pub deferred_count: usize,
}

/// Generates compact `SBOLS`/`DBOLS` inventory and validation metadata.
pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-bounded-linear-least-squares-api requires --offline".to_owned(),
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
                "bounded linear least-squares runtime profile lacks {name}=true"
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
            "bounded linear least-squares runtime, FFI, and selected-corpus snapshots differ"
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
    for (routine, precision, safe_path) in [
        (
            "DBOLS",
            "f64",
            "slatec::bounded_least_squares::solve_bounded_least_squares",
        ),
        (
            "SBOLS",
            "f32",
            "slatec::bounded_least_squares::solve_bounded_least_squares_f32",
        ),
    ] {
        let row = available.get(routine).ok_or_else(|| {
            CorpusError::Verification(format!(
                "bounded linear least-squares candidate {routine} is absent from validated FFI inventory"
            ))
        })?;
        let field = |ix: usize| {
            row.get(ix).and_then(Value::as_str).ok_or_else(|| {
                CorpusError::Verification(format!("invalid FFI record for {routine}"))
            })
        };
        let (unit, symbol, subset, path) = (
            field(id_ix)?,
            field(symbol_ix)?,
            field(subset_ix)?,
            field(path_ix)?,
        );
        candidates.push(json!([
            routine,
            unit,
            symbol,
            subset,
            path,
            precision,
            "bounded_dense_linear_least_squares_driver",
            "W(MDW,NCOLS+1), BL, BU, IND, IOPT, X, RNORM, MODE, RW, IW",
            "included",
            "reviewed_raw_abi_runtime_profile_native_validation"
        ]));
        wrappers.push(json!([
            safe_path,
            routine,
            symbol,
            unit,
            precision,
            "minimize_norm_Ax_minus_b_subject_to_closed_per_variable_bounds",
            "VariableBounds_to_BL_BU_IND; immutable input copied to owned_column_major_W",
            "W=M*(N+1); BL/BU/IND=N; RW=5*N; IW=2*N; IOPT=1_or_3_or_6_or_8",
            "process_global_saved_state_and_XERROR_level_one_messages_serialized",
            "native_execution_passed",
            "independent_small_active_set_reference",
            "included"
        ]));
    }
    candidates.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    wrappers.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    let deferred = vec![
        json!([
            "SBOCLS",
            "combined_bound_equality_least_squares",
            "general_linear_equalities_out_of_scope",
            "deferred"
        ]),
        json!([
            "DBOCLS",
            "combined_bound_equality_least_squares",
            "general_linear_equalities_out_of_scope",
            "deferred"
        ]),
        json!([
            "LSEI",
            "general_equality_inequality_least_squares",
            "inequality_workspace_contract_out_of_scope",
            "deferred"
        ]),
        json!([
            "DLSEI",
            "general_equality_inequality_least_squares",
            "inequality_workspace_contract_out_of_scope",
            "deferred"
        ]),
        json!([
            "SPLP",
            "linear_programming",
            "not_a_bounded_linear_least_squares_API",
            "deferred"
        ]),
        json!([
            "DSPLP",
            "linear_programming",
            "not_a_bounded_linear_least_squares_API",
            "deferred"
        ]),
        json!([
            "DBOLSM",
            "expert_bounded_least_squares_kernel",
            "raw_option_language_and_mutable_workspaces_not_public",
            "deferred"
        ]),
        json!([
            "SBOLSM",
            "expert_bounded_least_squares_kernel",
            "raw_option_language_and_mutable_workspaces_not_public",
            "deferred"
        ]),
    ];
    let files = [
        (
            "bounded-least-squares-candidate-index.json",
            json!({"schema_id":"slatec.safe-bounded-linear-least-squares.candidate-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","role","interface","review_state","reason"],"records":candidates}),
        ),
        (
            "bounded-least-squares-wrapper-index.json",
            json!({"schema_id":"slatec.safe-bounded-linear-least-squares.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","mathematical_model","bound_and_storage_policy","workspace_formula","runtime_policy","native_test_status","numerical_reference_type","review_state"],"records":wrappers}),
        ),
        (
            "bounded-least-squares-status-map.json",
            json!({"schema_id":"slatec.safe-bounded-linear-least-squares.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_mode","safe_status_or_error","meaning","policy"],"records":[[">=0","Converged","native active-set iteration completed; positive private count is not exposed","ok"],[-22,"MaximumIterations","DBOLSM/SBOLSM iteration limit reached","ok_with_approximate_solution"],["-2..-17","NativeContractViolation","safe wrapper precondition or option violation","error"],["-23..-38","NativeContractViolation","native subsidiary contract violation","error"]]}),
        ),
        (
            "bounded-least-squares-bound-encoding.json",
            json!({"schema_id":"slatec.safe-bounded-linear-least-squares.bound-encoding","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_variant","native_IND","BL","BU","meaning"],"records":[["Unbounded",4,"ignored","ignored","no bound"],["Lower(value)",1,"value","ignored","x>=BL"],["Upper(value)",2,"ignored","value","x<=BU"],["Between{lower,upper}",3,"lower","upper","BL<=x<=BU"],["Fixed(value)",3,"value","value","validated equal closed bounds"]]}),
        ),
        (
            "bounded-least-squares-options.json",
            json!({"schema_id":"slatec.safe-bounded-linear-least-squares.options","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_control","native_IOPT_protocol","default_or_policy"],"records":[["scaling=Nominal","[99]","native nominal scaling"],["scaling=EuclideanColumns","[3,2,99]","native option 3"],["scaling=Identity","[3,3,99]","native option 3"],["maximum_iterations=Some(limit)","[5,4,99,4,limit,99] or combined with scaling","positive checked INTEGER; DBOLSM option 4"],["rank_tolerance_and_other_raw_options","not_exposed","expert option language deferred"]]}),
        ),
        (
            "bounded-least-squares-workspace.json",
            json!({"schema_id":"slatec.safe-bounded-linear-least-squares.workspace","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["raw_routine","augmented_matrix","bounds_and_types","floating_workspace","integer_workspace","checked_arithmetic","ownership"],"records":[["SBOLS","M*(N+1) f32","BL,BU,IND: N","5*N f32","2*N INTEGER","checked_add_and_checked_mul","allocated internally"],["DBOLS","M*(N+1) f64","BL,BU,IND: N","5*N f64","2*N INTEGER","checked_add_and_checked_mul","allocated internally"]]}),
        ),
        (
            "bounded-least-squares-deferred.json",
            json!({"schema_id":"slatec.safe-bounded-linear-least-squares.deferred","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["raw_routine","role","reason","review_state"],"records":deferred}),
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
        "# Safe bounded linear least squares\n\n- Snapshot: `{snapshot}`\n- Reviewed public wrappers: 2 (`DBOLS`, `SBOLS`).\n- Contract: minimize `||A x-b||₂` subject to closed per-variable bounds; native `IND` codes are hidden behind `VariableBounds`.\n- Storage: caller matrix and right-hand side are copied into owned column-major `W(M,N+1)` because the native driver overwrites it.\n- Options: typed scaling and positive iteration limit only; raw option-array language is deferred.\n- Runtime: calls serialize saved DBOLS/SBOLS and legacy XERROR state; only recoverable level-one iteration-limit behavior is scoped.\n- Rank: the driver uses a private numerical dependence test and exposes no stable rank output; success does not claim a unique minimizer.\n- Deferred: combined equality-bound, general inequality, and linear-programming drivers.\n"
    );
    fs::write(
        output_dir.join("bounded-least-squares-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot,
        semantic_hash: hash::bytes(&bytes),
        candidate_count: 2,
        wrapper_count: 2,
        deferred_count: 8,
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
