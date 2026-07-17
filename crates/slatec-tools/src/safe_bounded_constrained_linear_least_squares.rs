//! Deterministic compact evidence for reviewed `SBOCLS` and `DBOCLS` wrappers.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Summary returned after generating bounded constrained least-squares metadata.
#[derive(Debug)]
pub struct ResultSummary {
    /// Shared selected-corpus snapshot identity.
    pub snapshot_id: String,
    /// Stable hash of all compact generated outputs.
    pub semantic_hash: String,
    /// Number of reviewed candidate records.
    pub candidate_count: usize,
    /// Number of public wrapper records.
    pub wrapper_count: usize,
    /// Number of explicit deferred records.
    pub deferred_count: usize,
}

/// Generates compact offline evidence for the safe bounded constrained wrappers.
pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-bounded-constrained-linear-least-squares-api requires --offline"
                .to_owned(),
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
                "bounded constrained least-squares runtime profile lacks {name}=true"
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
            "bounded constrained least-squares runtime, FFI, and selected-corpus snapshots differ"
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
            "DBOCLS",
            "f64",
            "slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares",
        ),
        (
            "SBOCLS",
            "f32",
            "slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32",
        ),
    ] {
        let row = available.get(routine).ok_or_else(|| {
            CorpusError::Verification(format!(
                "bounded constrained least-squares candidate {routine} is absent from validated FFI inventory"
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
            "bounded_constraint_expression_least_squares_driver",
            "W(MDW,NCOLS+MCON+1), BL, BU, IND, IOPT, X, RNORMC, RNORM, MODE, RW, IW",
            "reviewed_raw_abi_runtime_profile_native_validation",
            "included"
        ]));
        wrappers.push(json!([
            safe_path,
            routine,
            symbol,
            unit,
            precision,
            "minimize_norm_E_x_minus_f_subject_to_bounds_on_x_and_C_x",
            "owned_W_rows_C_then_E; BL_BU_IND_for_x_then_auxiliary_y_equal_Cx",
            "MDW=MCON+max(MROWS,NCOLS); W=MDW*(NCOLS+MCON+1); X=2*(NCOLS+MCON)+2; RW=6*NCOLS+5*MCON; IW=2*(NCOLS+MCON); IOPT=17",
            "process_global_saved_state_and_level_one_XERROR_serialized",
            "native_execution_passed",
            "small_active_set_and_overlap_cross_checks",
            "included"
        ]));
    }
    candidates.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    wrappers.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    let deferred = vec![
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
            "SBOCLS_option_language",
            "legacy_options",
            "only_default_safe_filter_is_validated",
            "deferred"
        ]),
        json!([
            "SBOCLS_active_sets",
            "native_internal_state",
            "no_stable_public_output_contract",
            "deferred"
        ]),
        json!([
            "dual_multipliers",
            "optimization_diagnostics",
            "not_returned_by_driver",
            "deferred"
        ]),
        json!([
            "constrained_covariance",
            "statistical_diagnostics",
            "separate_contract_and_scaling_review_required",
            "deferred"
        ]),
    ];
    let files = [
        (
            "bounded-constrained-least-squares-candidate-index.json",
            json!({"schema_id":"slatec.safe-bounded-constrained-least-squares.candidate-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","role","interface","review_state","inclusion"],"records":candidates}),
        ),
        (
            "bounded-constrained-least-squares-wrapper-index.json",
            json!({"schema_id":"slatec.safe-bounded-constrained-least-squares.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","mathematical_model","block_and_bound_policy","workspace_formula","runtime_policy","native_test_status","numerical_reference_type","review_state"],"records":wrappers}),
        ),
        (
            "bounded-constrained-least-squares-status-map.json",
            json!({"schema_id":"slatec.safe-bounded-constrained-least-squares.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_mode","safe_status_or_error","meaning","policy"],"records":[[">=0","Converged","native bounded constrained iteration completed","ok"],[-22,"MaximumIterations","subsidiary bounded iteration limit reached","ok_with_approximate_solution"],[-57,"NativeContractViolation","DBOCLS/SBOCLS input or option contract error range -57..-41","error"],[-38,"NativeContractViolation","DBOLSM/SBOLSM internal contract error range -38..-23","error"],[-19,"NativeContractViolation","DBOLS/SBOLS internal contract error range -19..-2","error"]]}),
        ),
        (
            "bounded-constrained-least-squares-block-layout.json",
            json!({"schema_id":"slatec.safe-bounded-constrained-least-squares.block-layout","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_rows_or_columns","safe_input","meaning","transformation"],"records":[["rows 1..MCON","constraints.matrix","C","copied first into W columns 1..NCOLS"],["rows MCON+1..MCON+MROWS","objective_matrix/objective_rhs","E and f","copied after C; f is W column NCOLS+1"],["columns NCOLS+1..NCOLS+MCON","internal auxiliary variables","y=Cx","native-owned columns; bounds copied after original variable bounds"]]}),
        ),
        (
            "bounded-constrained-least-squares-bound-encoding.json",
            json!({"schema_id":"slatec.safe-bounded-constrained-least-squares.bound-encoding","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_variant","native_IND","BL","BU","meaning"],"records":[["Unbounded",4,"ignored","ignored","no bound"],["Lower(value)",1,"value","ignored","z>=BL"],["Upper(value)",2,"ignored","value","z<=BU"],["Between{lower,upper}",3,"lower","upper","BL<=z<=BU"],["Fixed(value)",3,"value","value","equality bound; used for Cx equality"]]}),
        ),
        (
            "bounded-constrained-least-squares-options.json",
            json!({"schema_id":"slatec.safe-bounded-constrained-least-squares.options","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_control","native_IOPT","policy"],"records":[["BoundedConstrainedLeastSquaresOptions","[99, padding through element 17]","default filter only; padding is allocated because the driver writes nested option arrays"],["rank/scaling/preprocessing/accumulation","not_exposed","legacy mutable recursive option language deferred"]]}),
        ),
        (
            "bounded-constrained-least-squares-workspace.json",
            json!({"schema_id":"slatec.safe-bounded-constrained-least-squares.workspace","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["raw_routine","augmented_matrix","bounds_and_types","output","floating_workspace","integer_workspace","options","ownership"],"records":[["SBOCLS","(MCON+max(MROWS,NCOLS))*(NCOLS+MCON+1) f32","NCOLS+MCON","2*(NCOLS+MCON)+2 f32","6*NCOLS+5*MCON f32","2*(NCOLS+MCON) INTEGER","17 INTEGER","allocated and checked internally"],["DBOCLS","(MCON+max(MROWS,NCOLS))*(NCOLS+MCON+1) f64","NCOLS+MCON","2*(NCOLS+MCON)+2 f64","6*NCOLS+5*MCON f64","2*(NCOLS+MCON) INTEGER","17 INTEGER","allocated and checked internally"]]}),
        ),
        (
            "bounded-constrained-least-squares-deferred.json",
            json!({"schema_id":"slatec.safe-bounded-constrained-least-squares.deferred","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["item","role","reason","review_state"],"records":deferred}),
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
        "# Safe bounded constrained linear least squares\n\n- Snapshot: `{snapshot}`.\n- Reviewed wrappers: `DBOCLS` (f64) and `SBOCLS` (f32).\n- Contract: minimize `||E x-f||â‚‚` while both original variables `x` and constraint expressions `y=Cx` have closed bounds. A fixed `y` bound represents an equality; intervals represent constraint ranges.\n- Storage: `C` rows precede objective rows in owned column-major `W`; auxiliary `y` columns are native-owned.\n- Workspace: all source formulas, including 17 writable `IOPT` entries, use checked arithmetic.\n- Runtime: saved native state and documented level-one XERROR behavior are serialized and restored.\n- Deferred: raw option-array language, active-set/dual diagnostics, constrained covariance, sparse/warm-start work, and linear programming.\n"
    );
    fs::write(
        output_dir.join("bounded-constrained-least-squares-validation-summary.md"),
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
