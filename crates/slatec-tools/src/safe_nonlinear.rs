//! Deterministic compact inventory for the reviewed nonlinear easy drivers.
//!
//! The generator records only identity, reviewed interface facts, and validation
//! status. It never copies selected Fortran source, native workspaces, or logs.

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
    source: &'static str,
    precision: &'static str,
    interface_kind: &'static str,
    callback_signature: &'static str,
    mutation: &'static str,
    workspace: &'static str,
    disposition: &'static str,
    reason: &'static str,
}

const CANDIDATES: &[Candidate] = &[
    Candidate {
        source: "SNSQE",
        precision: "f32",
        interface_kind: "easy_driver_finite_difference_system",
        callback_signature: "FCN(N,X,FVEC,IFLAG); vector_by_reference",
        mutation: "X,FVEC,INFO,WA mutable; JAC ABI placeholder unused when IOPT=2",
        workspace: "(3*N*N + 13*N)/2 f32 elements",
        disposition: "included",
        reason: "reviewed_raw_declaration_runtime_profile_and_native_reference_required",
    },
    Candidate {
        source: "DNSQE",
        precision: "f64",
        interface_kind: "easy_driver_finite_difference_system",
        callback_signature: "FCN(N,X,FVEC,IFLAG); vector_by_reference",
        mutation: "X,FVEC,INFO,WA mutable; JAC ABI placeholder unused when IOPT=2",
        workspace: "(3*N*N + 13*N)/2 f64 elements",
        disposition: "included",
        reason: "reviewed_raw_declaration_runtime_profile_and_native_reference_required",
    },
    Candidate {
        source: "SNSQ",
        precision: "f32",
        interface_kind: "expert_system_driver",
        callback_signature: "residual_and_optional_jacobian_callbacks",
        mutation: "scaling, state, callbacks, work arrays, diagnostics",
        workspace: "caller-managed expert workspace",
        disposition: "covered_by_nonlinear_expert",
        reason: "reviewed_by_dedicated_expert_nonlinear_inventory",
    },
    Candidate {
        source: "DNSQ",
        precision: "f64",
        interface_kind: "expert_system_driver",
        callback_signature: "residual_and_optional_jacobian_callbacks",
        mutation: "scaling, state, callbacks, work arrays, diagnostics",
        workspace: "caller-managed expert workspace",
        disposition: "covered_by_nonlinear_expert",
        reason: "reviewed_by_dedicated_expert_nonlinear_inventory",
    },
    Candidate {
        source: "SOS",
        precision: "f32",
        interface_kind: "nonlinear_least_squares",
        callback_signature: "residual_and_optional_jacobian_callbacks",
        mutation: "vectors, Jacobian, controls, work arrays",
        workspace: "caller-managed expert workspace",
        disposition: "deferred",
        reason: "nonlinear_least_squares_and_jacobian_contract_out_of_scope",
    },
    Candidate {
        source: "DSOS",
        precision: "f64",
        interface_kind: "nonlinear_least_squares",
        callback_signature: "residual_and_optional_jacobian_callbacks",
        mutation: "vectors, Jacobian, controls, work arrays",
        workspace: "caller-managed expert workspace",
        disposition: "deferred",
        reason: "nonlinear_least_squares_and_jacobian_contract_out_of_scope",
    },
    Candidate {
        source: "CHKDER",
        precision: "f32",
        interface_kind: "Jacobian_checker",
        callback_signature: "none",
        mutation: "vectors and dense Jacobian arrays",
        workspace: "caller arrays",
        disposition: "covered_by_nonlinear_jacobian_check",
        reason: "reviewed_by_dedicated_expert_nonlinear_inventory",
    },
    Candidate {
        source: "DCKDER",
        precision: "f64",
        interface_kind: "Jacobian_checker",
        callback_signature: "none",
        mutation: "vectors and dense Jacobian arrays",
        workspace: "caller arrays",
        disposition: "covered_by_nonlinear_jacobian_check",
        reason: "reviewed_by_dedicated_expert_nonlinear_inventory",
    },
];

/// Result of generating nonlinear safe-API compact metadata.
#[derive(Debug)]
pub struct SafeNonlinearResult {
    /// Selected complete-corpus snapshot identity.
    pub snapshot_id: String,
    /// Stable hash of every generated nonlinear output.
    pub semantic_hash: String,
    /// Number of classified nonlinear candidates.
    pub candidate_count: usize,
    /// Number of included public wrappers.
    pub wrapper_count: usize,
    /// Number of deterministic deferred records.
    pub deferred_count: usize,
}

/// Generates compact nonlinear easy-driver metadata using committed evidence.
pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<SafeNonlinearResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-nonlinear-api requires --offline".to_owned(),
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
            "nonlinear runtime, FFI, and selected-corpus snapshots differ".to_owned(),
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

    let mut candidate_records = Vec::new();
    let mut wrappers = Vec::new();
    let mut deferred = Vec::new();
    let mut seen = BTreeSet::new();
    for candidate in CANDIDATES {
        let row = available.get(candidate.source).ok_or_else(|| {
            CorpusError::Verification(format!(
                "nonlinear candidate {} is absent from the selected FFI inventory",
                candidate.source
            ))
        })?;
        let id = value(row, id_ix)?;
        let subset = value(row, subset_ix)?;
        let path = value(row, path_ix)?;
        let symbol = value(row, symbol_ix)?;
        if id.is_empty() || symbol.is_empty() || !seen.insert(candidate.source) {
            return Err(CorpusError::Verification(format!(
                "nonlinear candidate {} has invalid compact identity",
                candidate.source
            )));
        }
        candidate_records.push(json!([
            candidate.source,
            id,
            symbol,
            subset,
            path,
            candidate.precision,
            candidate.interface_kind,
            candidate.callback_signature,
            candidate.mutation,
            candidate.workspace,
            candidate.disposition,
            candidate.reason,
        ]));
        if candidate.disposition == "included" {
            let safe_path = if candidate.source == "DNSQE" {
                "slatec::nonlinear::solve_system"
            } else {
                "slatec::nonlinear::solve_system_f32"
            };
            wrappers.push(json!([
                safe_path,
                candidate.source,
                symbol,
                id,
                candidate.precision,
                "shared_scoped_thread_local_vector_trampoline",
                "IOPT=2 finite_difference; JAC never invoked",
                candidate.workspace,
                "INFO_1_to_4",
                "serialized_process_native_lock",
                "native_execution_passed",
                "analytic_system_reference",
                "included",
            ]));
        } else if candidate.disposition == "deferred" {
            deferred.push(json!([
                candidate.source,
                symbol,
                candidate.interface_kind,
                candidate.reason,
                "manual_review_required",
            ]));
        }
    }
    candidate_records.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    wrappers.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    deferred.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));

    let mut outputs = BTreeMap::new();
    outputs.insert("nonlinear-candidate-index.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear.candidate-index", "schema_version":"1.0.0",
        "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","interface_kind","callback_signature","mutation","workspace_policy","disposition","reason"],
        "records":candidate_records,
    }))?);
    outputs.insert("nonlinear-wrapper-index.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear.wrapper-index", "schema_version":"1.0.0",
        "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","callback_policy","jacobian_policy","workspace_formula","status_mapping","concurrency_policy","native_test_status","numerical_reference_type","review_state"],
        "records":wrappers,
    }))?);
    outputs.insert("nonlinear-status-map.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear.status-map", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["info","safe_status","meaning","result_policy"],
        "records":[
            [1,"Converged","relative error between successive iterates is at most TOL","ok"],
            [2,"MaximumFunctionEvaluations","finite-difference callback budget 200*(N+1) reached","ok_with_status"],
            [3,"ToleranceTooSmall","TOL is below working precision","ok_with_status"],
            [4,"SlowProgress","native progress criterion failed; SNSQE/DNSQE map INFO=5 to 4","ok_with_status"]
        ],
    }))?);
    outputs.insert("nonlinear-callback-policy.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear.callback-policy", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["family","callback_context","pointer_policy","panic_policy","non_finite_policy","cancellation_policy","nested_policy","concurrency_policy"],
        "records":[["nonlinear_easy","shared_scoped_thread_local_vector_slot","N_and_pointer_ranges_checked_before_slice_construction","caught_before_fortran_unwind; finite_zero_residual_sentinel","first_non_finite_residual_index_recorded; finite_zero_residual_sentinel","not_exposed_negative_IFLAG_reaches_fatal_XERROR","cross_family_rejected","serialized_process_native_lock"]],
    }))?);
    outputs.insert("nonlinear-deferred.json", compact(&json!({
        "schema_id":"slatec.safe-nonlinear.deferred", "schema_version":"1.0.0", "snapshot_id":snapshot,
        "columns":["raw_routine","raw_symbol","interface_kind","reason","review_state"], "records":deferred,
    }))?);
    let semantic_hash = semantic_hash(&outputs);
    outputs.insert("nonlinear-validation-summary.md", format!(
        "# Safe nonlinear easy-driver validation\n\n- Snapshot: `{snapshot}`\n- Profile: `{PROFILE}`\n- Classified candidates: {}\n- Reviewed safe wrappers: {}\n- Deferred candidates: {}\n- Jacobian policy: `IOPT = 2` finite differences only; the ABI `JAC` argument is never called\n- Workspace formula: `(3*N*N + 13*N)/2` checked elements\n- Callback policy: shared scoped vector trampoline; panic and non-finite results are recorded while finite sentinel residuals preserve in-process containment\n- Cancellation policy: not exposed; the historical negative-`IFLAG` path reaches fatal XERROR for this profile\n- Concurrency policy: native calls serialize; cross-family callback nesting is rejected\n- Semantic hash: `{semantic_hash}`\n\nThe original SLATEC Fortran routines remain the numerical implementation. Detailed native evidence remains ignored.\n",
        CANDIDATES.len(), wrappers.len(), deferred.len()
    ).into_bytes());
    let total = outputs.values().map(Vec::len).sum::<usize>();
    if total > MAX_TOTAL_BYTES {
        return Err(CorpusError::Policy(format!(
            "safe nonlinear metadata is {total} bytes, above {MAX_TOTAL_BYTES}"
        )));
    }
    promote(output_dir, &outputs)?;
    Ok(SafeNonlinearResult {
        snapshot_id: snapshot,
        semantic_hash,
        candidate_count: CANDIDATES.len(),
        wrapper_count: wrappers.len(),
        deferred_count: deferred.len(),
    })
}

fn require_runtime_gate(runtime: &Value) -> Result<()> {
    let validation = runtime["validation"].as_object().ok_or_else(|| {
        CorpusError::Verification("runtime profile lacks validation gates".to_owned())
    })?;
    for gate in [
        "abi_validated",
        "machine_constants_validated",
        "legacy_error_behavior_validated",
        "fnlib_initialization_validated",
    ] {
        if validation.get(gate).and_then(Value::as_bool) != Some(true) {
            return Err(CorpusError::Verification(format!(
                "runtime-profile gate {gate} is not true"
            )));
        }
    }
    Ok(())
}

fn read(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path)?).map_err(CorpusError::from)
}

fn string(value: &Value, field: &str) -> Result<String> {
    value[field]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("missing {field}")))
}

fn value(row: &[Value], index: usize) -> Result<&str> {
    row.get(index).and_then(Value::as_str).ok_or_else(|| {
        CorpusError::Verification("nonlinear inventory field is not a string".to_owned())
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
    use super::*;

    #[test]
    fn inventory_keeps_easy_and_expert_drivers_distinct() {
        assert_eq!(CANDIDATES.len(), 8);
        assert_eq!(
            CANDIDATES
                .iter()
                .filter(|candidate| candidate.disposition == "included")
                .count(),
            2
        );
        assert!(CANDIDATES.iter().any(|candidate| candidate.source == "DNSQ"
            && candidate.disposition == "covered_by_nonlinear_expert"));
    }

    #[test]
    fn runtime_gate_requires_every_layer() {
        let valid = json!({"validation":{
            "abi_validated":true, "machine_constants_validated":true,
            "legacy_error_behavior_validated":true, "fnlib_initialization_validated":true
        }});
        assert!(require_runtime_gate(&valid).is_ok());
        let mut invalid = valid;
        invalid["validation"]["fnlib_initialization_validated"] = json!(false);
        assert!(require_runtime_gate(&invalid).is_err());
    }

    #[test]
    fn committed_inputs_regenerate_identically_in_separate_roots() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let temporary = tempfile::tempdir().unwrap();
        let first = temporary.path().join("first/safe-api");
        let second = temporary.path().join("second/safe-api");
        let regenerate = |output: &Path| {
            generate(
                &root.join("generated/runtime-profile"),
                &root.join("generated/ffi"),
                &root.join("generated/selected-corpus"),
                output,
                true,
            )
            .unwrap()
        };
        let first_result = regenerate(&first);
        let second_result = regenerate(&second);
        assert_eq!(first_result.semantic_hash, second_result.semantic_hash);
        for name in [
            "nonlinear-candidate-index.json",
            "nonlinear-wrapper-index.json",
            "nonlinear-status-map.json",
            "nonlinear-callback-policy.json",
            "nonlinear-deferred.json",
            "nonlinear-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.join(name)).unwrap(),
                fs::read(second.join(name)).unwrap()
            );
        }
    }
}
