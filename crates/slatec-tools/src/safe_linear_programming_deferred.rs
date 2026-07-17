//! Deterministic audit metadata for the reviewed but unsupported SPLP family.
//!
//! This module intentionally emits no wrapper, raw-FFI, provider, feature, or
//! source-closure record. Its output preserves the audit decision that the
//! original drivers' mandatory external paging protocol cannot provide the
//! safe ownership and recovery guarantees required by this crate.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Summary returned after generating the SPLP/DSPLP deferral audit metadata.
#[derive(Debug)]
pub struct ResultSummary {
    /// Shared selected-corpus snapshot identity.
    pub snapshot_id: String,
    /// Stable hash of every generated audit file.
    pub semantic_hash: String,
    /// Number of reviewed deferred driver candidates.
    pub candidate_count: usize,
}

/// Generates compact offline metadata for the unsupported SPLP/DSPLP drivers.
pub fn generate(
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-linear-programming-deferred-metadata requires --offline".to_owned(),
        ));
    }

    let inventory = read(&ffi_dir.join("interface-inventory.json"))?;
    let selected = read(&selected_corpus_dir.join("manifest.json"))?;
    let snapshot = string(&selected, "snapshot_id")?;
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
    let mut by_name = BTreeMap::new();
    for row in records.iter().filter_map(Value::as_array) {
        if let Some(name) = row.get(name_ix).and_then(Value::as_str) {
            by_name.insert(name, row);
        }
    }

    let mut candidates = Vec::new();
    for (routine, precision) in [("DSPLP", "f64"), ("SPLP", "f32")] {
        let row = by_name.get(routine).ok_or_else(|| {
            CorpusError::Verification(format!(
                "LP candidate {routine} is absent from FFI inventory"
            ))
        })?;
        let field = |ix: usize| {
            row.get(ix).and_then(Value::as_str).ok_or_else(|| {
                CorpusError::Verification(format!("invalid FFI inventory record for {routine}"))
            })
        };
        candidates.push(json!([
            routine,
            field(id_ix)?,
            field(symbol_ix)?,
            field(subset_ix)?,
            field(path_ix)?,
            precision,
            "sparse_linear_programming_driver",
            "minimize_c_transpose_x_subject_to_Ax_equals_w_with_bounds_on_x_and_w",
            "reviewed_source_contract_no_public_raw_declaration",
            "deferred"
        ]));
    }
    candidates.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));

    let files = [
        (
            "linear-programming-candidate-index.json",
            json!({"schema_id":"slatec.safe-linear-programming.candidate-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","role","mathematical_model","review_state","disposition"],"records":candidates}),
        ),
        (
            "linear-programming-mathematical-model.json",
            json!({"schema_id":"slatec.safe-linear-programming.mathematical-model","schema_version":"1.0.0","snapshot_id":snapshot,"model":"minimize c^T x subject to A x = w","columns":["concept","native_representation","meaning"],"records":[["objective","COSTS(1..NVARS)","no objective constant; minimization is default and option key 50 selects maximization"],["variables","x(1..NVARS), BL/BU/IND(1..NVARS)","IND 1 lower, 2 upper, 3 two-sided (equal endpoints allowed), 4 free"],["row activities","w(1..MRELAS)=A*x, BL/BU/IND(NVARS+1..NVARS+MRELAS)","same typed bound encoding; equal two-sided bounds encode equality rows"],["matrix","A(MRELAS,NVARS)","sparse entries are supplied through the external matrix callback protocol"]]}),
        ),
        (
            "linear-programming-sparse-protocol.json",
            json!({"schema_id":"slatec.safe-linear-programming.sparse-protocol","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["precision","default_callback","protocol","indexing","ordering_and_duplicates","safety_disposition"],"records":[["f32","USRMT","IFLAG(1)=1 initializes; calls with IFLAG(1)=2 yield one (I,J,AIJ,INDCAT) entry; IFLAG(1)=3 terminates","one-based row and column indices","entries may be supplied in convenient order; INDCAT 0 assigns and 1 accumulates; the built-in USRMT stream assigns, so a safe API would require deterministic duplicate policy","deferred before any Rust callback or matrix view is exposed"],["f64","DUSRMT","IFLAG(1)=1 initializes; calls with IFLAG(1)=2 yield one (I,J,AIJ,INDCAT) entry; IFLAG(1)=3 terminates","one-based row and column indices","entries may be supplied in convenient order; INDCAT 0 assigns and 1 accumulates; the built-in DUSRMT stream assigns, so a safe API would require deterministic duplicate policy","deferred before any Rust callback or matrix view is exposed"]]}),
        ),
        (
            "linear-programming-deferred.json",
            json!({"schema_id":"slatec.safe-linear-programming.deferred","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["item","role","reason","review_state"],"records":[["DSPLP","f64_sparse_linear_programming_driver","mandatory_DPRWVR_to_SOPENM_SCLOSM_global_direct_access_file_has_no_safe_ownership_cleanup_or_reliable_io_failure_protocol","reviewed_and_deferred"],["SPLP","f32_sparse_linear_programming_driver","mandatory_PRWVIR_to_SOPENM_SCLOSM_global_direct_access_file_has_no_safe_ownership_cleanup_or_reliable_io_failure_protocol","reviewed_and_deferred"],["optimization-linear-programming_feature","public_capability","no_safe_driver_is_available_so_no_feature_or_provider_closure_is_advertised","deferred"],["native_io_shim","substitution","explicitly_not_implemented_to_preserve_the_reviewed_original_source_contract","deferred"]]}),
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
        "# Reviewed but deferred SLATEC linear programming\n\n- Snapshot: `{snapshot}`.\n- Candidates: `SPLP` (f32) and `DSPLP` (f64); neither has a raw declaration, provider feature, source closure, safe wrapper, or example.\n- Model: minimize `c^T x` subject to `A x = w`, with `IND`-encoded lower, upper, two-sided/fixed, or free bounds on both `x` and the row activities `w`.\n- Sparse protocol: `USRMT`/`DUSRMT` is a one-based external callback protocol, not a modern owned sparse-matrix ABI.\n- Blocking dependency: the single path uses `PRWVIR -> SOPENM/SCLOSM`; the double path uses `DPRWVR -> SOPENM/SCLOSM`. They open a process-global direct-access Fortran unit and close it with `STATUS='KEEP'`.\n- Decision: a runtime lock can serialize access, but cannot give Rust filename ownership, deterministic cleanup, or a reliable post-I/O-failure native state. No native I/O shim is substituted.\n"
    );
    fs::write(
        output_dir.join("linear-programming-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot,
        semantic_hash: hash::bytes(&bytes),
        candidate_count: 2,
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
