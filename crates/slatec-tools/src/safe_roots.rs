//! Deterministic compact inventory for the reviewed safe scalar-root surface.
//!
//! This generator keeps the required scalar FZERO pair distinct from complex
//! polynomial and nonlinear-system candidates. It emits structural evidence
//! only: no copied source, native output, or workspace contents.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

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
    argument_shape: &'static str,
    mutable_inputs: &'static str,
    workspace_policy: &'static str,
    status_output: &'static str,
    machine_dependency: &'static str,
    runtime_dependency: &'static str,
    disposition: &'static str,
    reason: &'static str,
}

macro_rules! scalar {
    ($source:literal, $precision:literal) => {
        Candidate {
            source: $source,
            precision: $precision,
            interface_kind: "scalar_bracketed_callback",
            callback_signature: "scalar_by_reference_return_scalar",
            argument_shape: "scalar_function,b,c,r,re,ae,iflag",
            mutable_inputs: "b,c,r; re,ae and iflag passed by reference",
            workspace_policy: "no caller workspace; fixed internal evaluation limit >500",
            status_output: "iflag_1_to_5",
            machine_dependency: "R1MACH_or_D1MACH",
            runtime_dependency: "runtime_independent_machine_constants",
            disposition: "included",
            reason: "reviewed_raw_declaration_and_native_reference_passed",
        }
    };
}

macro_rules! polynomial {
    ($source:literal, $kind:literal, $shape:literal, $workspace:literal, $reason:literal) => {
        Candidate {
            source: $source,
            precision: "f32_complex",
            interface_kind: $kind,
            callback_signature: "none",
            argument_shape: $shape,
            mutable_inputs: "coefficient/output/work arrays require a dedicated contract audit",
            workspace_policy: $workspace,
            status_output: "routine_specific_error_flag",
            machine_dependency: "unverified_for_safe_surface",
            runtime_dependency: "legacy_error_state_possible",
            disposition: "deferred",
            reason: $reason,
        }
    };
}

macro_rules! nonlinear {
    ($source:literal, $precision:literal, $kind:literal) => {
        Candidate {
            source: $source,
            precision: $precision,
            interface_kind: $kind,
            callback_signature: "residual_and_optional_jacobian_callbacks",
            argument_shape: "mutable_vector_state,dense_column_major_jacobian,controls,workspaces",
            mutable_inputs: "state,scaling,callback_control_flags,and_work_arrays",
            workspace_policy: "caller_or_session_managed_workspaces",
            status_output: "solver_specific_info",
            machine_dependency: "unreviewed_for_safe_surface",
            runtime_dependency: "process_global_error_and_callback_state",
            disposition: "deferred",
            reason: "nonlinear_system_architecture_out_of_scope",
        }
    };
}

const CANDIDATES: &[Candidate] = &[
    scalar!("FZERO", "f32"),
    scalar!("DFZERO", "f64"),
    polynomial!(
        "RPQR79",
        "real_polynomial_complex_roots",
        "degree,real_coefficients,complex_roots,error,real_workspace",
        "NDEG*(NDEG+2) real values",
        "no_existing_safe_complex_array_policy_or_residual_validation"
    ),
    polynomial!(
        "CPQR79",
        "complex_polynomial_complex_roots",
        "degree,complex_coefficients,complex_roots,error,real_workspace",
        "2*NDEG*(NDEG+1) real values",
        "no_existing_safe_complex_array_policy_or_residual_validation"
    ),
    polynomial!(
        "RPZERO",
        "real_polynomial_iterative_roots",
        "degree,real_coefficients,complex_initial_roots,complex_workspace,error,bounds",
        "complex_6*(N+1) temporary storage plus N real bounds",
        "mutable_initial_estimate_and_complex_workspace_contract_unvalidated"
    ),
    polynomial!(
        "CPZERO",
        "complex_polynomial_iterative_roots",
        "degree,complex_coefficients,complex_initial_roots,complex_workspace,error,bounds",
        "complex_4*(N+1) temporary storage plus N real bounds",
        "mutable_initial_estimate_and_complex_workspace_contract_unvalidated"
    ),
    nonlinear!("SNSQ", "f32", "nonlinear_system"),
    nonlinear!("DNSQ", "f64", "nonlinear_system"),
    nonlinear!("SNSQE", "f32", "nonlinear_system_expert"),
    nonlinear!("DNSQE", "f64", "nonlinear_system_expert"),
    nonlinear!("SOS", "f32", "nonlinear_system_optional_jacobian"),
    nonlinear!("DSOS", "f64", "nonlinear_system_optional_jacobian"),
    Candidate {
        source: "CHKDER",
        precision: "f32",
        interface_kind: "nonlinear_derivative_checker",
        callback_signature: "none",
        argument_shape: "vectors_and_dense_jacobian",
        mutable_inputs: "vectors_and_work_arrays",
        workspace_policy: "caller-managed arrays",
        status_output: "mode_dependent",
        machine_dependency: "unreviewed_for_safe_surface",
        runtime_dependency: "runtime_independent",
        disposition: "deferred",
        reason: "nonlinear_system_architecture_out_of_scope",
    },
    Candidate {
        source: "DCKDER",
        precision: "f64",
        interface_kind: "nonlinear_derivative_checker",
        callback_signature: "none",
        argument_shape: "vectors_and_dense_jacobian",
        mutable_inputs: "vectors_and_work_arrays",
        workspace_policy: "caller-managed arrays",
        status_output: "mode_dependent",
        machine_dependency: "unreviewed_for_safe_surface",
        runtime_dependency: "runtime_independent",
        disposition: "deferred",
        reason: "nonlinear_system_architecture_out_of_scope",
    },
];

#[derive(Debug)]
pub struct SafeRootsResult {
    pub snapshot_id: String,
    pub semantic_hash: String,
    pub candidate_count: usize,
    pub wrapper_count: usize,
    pub deferred_count: usize,
    pub output_dir: PathBuf,
}

pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<SafeRootsResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-roots-api requires --offline".to_owned(),
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
            "root runtime, FFI, and selected-corpus snapshots differ".to_owned(),
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
        .filter_map(|row| {
            let name = row.get(name_ix)?.as_str()?;
            Some((name, row))
        })
        .collect::<BTreeMap<_, _>>();

    let mut candidate_records = Vec::new();
    let mut wrappers = Vec::new();
    let mut deferred = Vec::new();
    let mut seen = BTreeSet::new();
    for candidate in CANDIDATES {
        let row = available.get(candidate.source).ok_or_else(|| {
            CorpusError::Verification(format!(
                "root candidate {} is absent from the selected FFI inventory",
                candidate.source
            ))
        })?;
        let id = value(row, id_ix)?;
        let subset = value(row, subset_ix)?;
        let path = value(row, path_ix)?;
        let symbol = value(row, symbol_ix)?;
        if id.is_empty() || symbol.is_empty() || !seen.insert(candidate.source) {
            return Err(CorpusError::Verification(format!(
                "root candidate {} has invalid compact identity",
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
            candidate.argument_shape,
            candidate.mutable_inputs,
            candidate.workspace_policy,
            candidate.status_output,
            candidate.machine_dependency,
            candidate.runtime_dependency,
            candidate.disposition,
            candidate.reason,
        ]));
        if candidate.disposition == "included" {
            let safe_path = if candidate.source == "DFZERO" {
                "slatec::roots::find_root"
            } else {
                "slatec::roots::find_root_f32"
            };
            wrappers.push(json!([
                safe_path,
                candidate.source,
                symbol,
                id,
                candidate.precision,
                "thread_local_scoped_trampoline",
                "b,c,r,re,ae,iflag mutable scalar references",
                candidate.status_output,
                candidate.workspace_policy,
                "serialized_process_native_lock",
                "native_reference_passed",
                "analytic_root_reference",
                "included",
            ]));
        } else {
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
    outputs.insert(
        "root-candidate-index.json",
        compact(&json!({
            "schema_id":"slatec.safe-roots.candidate-index", "schema_version":"1.0.0",
            "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
            "columns":["raw_routine","program_unit_id","raw_symbol","source_subset","source_path","precision","interface_kind","callback_signature","argument_shape","mutable_inputs","workspace_policy","status_output","machine_dependency","runtime_dependency","disposition","reason"],
            "records":candidate_records,
        }))?,
    );
    outputs.insert(
        "root-wrapper-index.json",
        compact(&json!({
            "schema_id":"slatec.safe-roots.wrapper-index", "schema_version":"1.0.0",
            "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
            "columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","callback_policy","input_mutation","status_mapping","workspace_policy","concurrency_policy","native_test_status","numerical_reference_type","review_state"],
            "records":wrappers,
        }))?,
    );
    outputs.insert(
        "root-status-map.json",
        compact(&json!({
            "schema_id":"slatec.safe-roots.status-map", "schema_version":"1.0.0",
            "snapshot_id":snapshot,
            "columns":["iflag","safe_status","meaning","result_policy"],
            "records":[
                [1,"Converged","bracket collapsed with decreasing function magnitude","ok"],
                [2,"EndpointRoot","native current endpoint is exactly zero","ok"],
                [3,"PossibleSingularity","sign-changing bracket collapsed with increasing magnitude","ok_with_warning_status"],
                [4,"NoSignChange","native iteration collapsed without a sign change","ok_with_warning_status"],
                [5,"MaximumEvaluations","fixed native limit greater than 500 function evaluations reached","ok_with_warning_status"],
            ],
        }))?,
    );
    outputs.insert(
        "root-callback-policy.json",
        compact(&json!({
            "schema_id":"slatec.safe-roots.callback-policy", "schema_version":"1.0.0",
            "snapshot_id":snapshot,
            "columns":["family","callback_context","panic_policy","non_finite_policy","nested_policy","concurrency_policy","evaluation_count"],
            "records":[["scalar_roots","shared_scoped_thread_local_slot","caught_before_fortran_unwind","recorded_and_reported_after_return","cross_family_rejected","serialized_process_native_lock","trampoline_invocation_count_including_safe_endpoint_checks"]],
        }))?,
    );
    outputs.insert(
        "root-deferred.json",
        compact(&json!({
            "schema_id":"slatec.safe-roots.deferred", "schema_version":"1.0.0",
            "snapshot_id":snapshot,
            "columns":["raw_routine","raw_symbol","interface_kind","reason","review_state"],
            "records":deferred,
        }))?,
    );
    let semantic_hash = semantic_hash(&outputs);
    outputs.insert(
        "root-validation-summary.md",
        format!(
            "# Safe scalar-root validation\n\n- Snapshot: `{snapshot}`\n- Profile: `{PROFILE}`\n- Classified candidates: {}\n- Reviewed safe wrappers: {}\n- Deferred candidates: {}\n- Callback policy: shared scoped thread-local trampoline; panics and non-finite results are contained\n- Concurrency policy: root and quadrature native calls serialize; nested callback calls are rejected\n- Polynomial policy: complex-array candidates remain deferred pending a complete safe complex/workspace/residual contract\n- Semantic hash: `{semantic_hash}`\n\nThe original SLATEC Fortran routines remain the numerical implementation. Detailed native evidence remains ignored.\n",
            CANDIDATES.len(),
            wrappers.len(),
            deferred.len(),
        )
        .into_bytes(),
    );
    let total = outputs.values().map(Vec::len).sum::<usize>();
    if total > MAX_TOTAL_BYTES {
        return Err(CorpusError::Policy(format!(
            "safe root metadata is {total} bytes, above {MAX_TOTAL_BYTES}"
        )));
    }
    promote(output_dir, &outputs)?;
    Ok(SafeRootsResult {
        snapshot_id: snapshot,
        semantic_hash,
        candidate_count: CANDIDATES.len(),
        wrapper_count: wrappers.len(),
        deferred_count: deferred.len(),
        output_dir: output_dir.to_owned(),
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
    row.get(index)
        .and_then(Value::as_str)
        .ok_or_else(|| CorpusError::Verification("root inventory field is not a string".to_owned()))
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
    fn inventory_keeps_scalar_and_deferred_families_distinct() {
        assert_eq!(CANDIDATES.len(), 14);
        assert_eq!(
            CANDIDATES
                .iter()
                .filter(|candidate| candidate.disposition == "included")
                .count(),
            2
        );
        assert!(CANDIDATES.iter().any(|candidate| {
            candidate.source == "CPQR79"
                && candidate.reason
                    == "no_existing_safe_complex_array_policy_or_residual_validation"
        }));
    }

    #[test]
    fn runtime_gate_requires_every_layer() {
        let valid = json!({"validation":{
            "abi_validated":true, "machine_constants_validated":true,
            "legacy_error_behavior_validated":true, "fnlib_initialization_validated":true
        }});
        assert!(require_runtime_gate(&valid).is_ok());
        let mut invalid = valid;
        invalid["validation"]["machine_constants_validated"] = json!(false);
        assert!(require_runtime_gate(&invalid).is_err());
    }

    #[test]
    fn committed_inputs_regenerate_identically_in_separate_roots() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
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
            "root-candidate-index.json",
            "root-wrapper-index.json",
            "root-status-map.json",
            "root-callback-policy.json",
            "root-deferred.json",
            "root-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.join(name)).unwrap(),
                fs::read(second.join(name)).unwrap()
            );
        }
    }
}
