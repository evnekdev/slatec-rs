//! Deterministic, evidence-labelled concurrency and storage policy metadata.
//!
//! This is an architecture audit rather than a numerical wrapper generator.
//! It deliberately records uncertainty as `BackendDependent` instead of
//! promoting an unproven native call to a thread-safe API contract.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const SAFE_API: &str = "generated/safe-api";

/// Summary returned after generating runtime and storage-policy metadata.
#[derive(Debug)]
pub struct ResultSummary {
    /// Number of safe public functions classified by the audit.
    pub function_count: usize,
    /// Stable hash of the emitted files.
    pub semantic_hash: String,
}

/// Generates the library-wide runtime, storage, interoperability, and LP
/// paging audit files.  Inputs are the reviewed safe-API indexes already in
/// the repository; the command never downloads or builds native sources.
pub fn generate(output_dir: &Path) -> Result<ResultSummary> {
    let functions = records(repo_path(SAFE_API).join("function-index.json"))?;
    let mappings = records(repo_path(SAFE_API).join("fortran-argument-map.json"))?;
    let argument_index = read_value(repo_path("generated/ffi-inventory/argument-index.json"))?;
    let selected = read_value(repo_path("generated/selected-corpus/manifest.json"))?;
    let snapshot = selected["snapshot_id"]
        .as_str()
        .ok_or_else(|| policy("selected corpus lacks snapshot_id"))?;

    let arguments = argument_lookup(&argument_index)?;
    let mutable_state = mutable_global_state_records()?;
    let concurrency = concurrency_records(&functions, &mutable_state.routine_sources)?;
    let storage = storage_records(&mappings, &arguments)?;
    let concurrency_summary = concurrency_summary(&concurrency);
    let storage_summary = storage_summary(&storage);
    let lp_policy = lp_paging_policy(snapshot);
    let matrixpacked = matrixpacked_compatibility(snapshot);
    let runtime_policy = runtime_policy(snapshot, functions.len(), storage.len());

    fs::create_dir_all(output_dir)?;
    let files = [
        (
            "concurrency-index.json",
            json!({
                "schema_id":"slatec.runtime.concurrency-index",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "columns":["safe_path","fortran_routine","feature","concurrency_class","lock_scope","current_dispatch","callback_dispatch","nested_callback_policy","xerror_policy","native_state_evidence","mutable_global_state_record_ids","external_io_evidence","send_policy","sync_policy","relaxation_gate","review_state"],
                "records":concurrency,
            }),
        ),
        (
            "mutable-global-state-index.json",
            json!({
                "schema_id":"slatec.runtime.mutable-global-state-index",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "coverage":"all_current_safe_records_reference_at_least_one_finding_or_an_explicit_incomplete_closure_result; the available offline source cache is incomplete, so no closure is marked free_of_mutable_static_storage",
                "columns":["id","routine","symbol_or_storage","origin","mutable","initialization","scope","access","synchronization","reentrancy_effect","evidence"],
                "records":mutable_state.records,
            }),
        ),
        (
            "storage-contract-index.json",
            json!({
                "schema_id":"slatec.runtime.storage-contract-index",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "columns":["safe_path","fortran_routine","fortran_argument","rust_argument","declared_type","array_rank","argument_role","native_layout","safe_representation","current_transfer","implicit_rearrangement","validation_requirement","review_state"],
                "records":storage,
            }),
        ),
        ("matrixpacked-compatibility.json", matrixpacked),
        ("lp-paging-policy.json", lp_policy),
        ("runtime-storage-policy.json", runtime_policy),
    ];
    let mut semantic = Vec::new();
    for (name, value) in files {
        let encoded = serde_json::to_vec(&value)?;
        fs::write(output_dir.join(name), &encoded)?;
        semantic.extend_from_slice(&encoded);
    }
    for (name, text) in [
        ("concurrency-audit-summary.md", concurrency_summary),
        ("storage-interoperability-summary.md", storage_summary),
    ] {
        fs::write(output_dir.join(name), text.as_bytes())?;
        semantic.extend_from_slice(text.as_bytes());
    }

    Ok(ResultSummary {
        function_count: functions.len(),
        semantic_hash: hash::bytes(&semantic),
    })
}

fn concurrency_records(
    functions: &[Value],
    routine_sources: &BTreeMap<String, String>,
) -> Result<Vec<Value>> {
    let mut output = Vec::with_capacity(functions.len());
    for function in functions {
        let safe_path = string(function, "rust_path")?;
        let routine = string(function, "fortran_routine")?;
        let feature = string(function, "feature")?;
        let domain = string(function, "domain")?;
        let backend_dependent =
            feature.starts_with("blas-") || feature == "nonlinear-jacobian-check";
        let ode = feature == "ode-sdrive-expert";
        let callback = callback_dispatch(domain, feature);
        let class = if backend_dependent {
            "BackendDependent"
        } else {
            "SerializedGlobal"
        };
        let lock_scope = if backend_dependent {
            "none"
        } else {
            "process_global"
        };
        let dispatch = if backend_dependent {
            "direct_reviewed_raw_ffi; no_std_or_alloc_capability"
        } else {
            "crate::runtime::lock_native"
        };
        let xerror = if matches!(feature, "ode-sdrive-expert") {
            "scoped_XGETF_XSETF_restore"
        } else if feature.starts_with("least-squares") {
            "scoped_XGETF_XSETF_restore_where_reviewed"
        } else if backend_dependent {
            "native_default_not_scoped"
        } else {
            "native_default_or_checked_precondition; no_general_interception_claim"
        };
        let global = if backend_dependent {
            "source_level_reentrancy_not_proven; safe_wrapper_exposes_no_global_dispatch"
        } else if ode {
            "SDRIVE_callback_context_and_XERROR; serialized_pending_complete_native_stress_validation"
        } else if callback != "none" {
            "thread_local_callback_registration_and_legacy_runtime; serialized"
        } else {
            "selected_SLATEC_FNLIB_or_driver_state_may_include_SAVE_COMMON_or_XERROR; serialized_conservatively"
        };
        let external_io = if ode {
            "no_mandatory_file_protocol_in_reviewed_SDRIV3_DDRIV3_scope"
        } else {
            "not_a_public_resource_contract; source-closure-specific audit_required_before_relaxation"
        };
        let relaxation = if backend_dependent {
            "require_provider_source_provenance_and_parallel_stress_test_before_threadsafe_claim"
        } else if ode {
            "not_candidate_until_complete_source_closure_build_callback_reentry_and_parallel_stress_tests_pass"
        } else {
            "require_complete_source_closure_audit_XERROR_audit_and_parallel_stress_test"
        };
        let state_ids = mutable_state_ids(
            feature,
            domain,
            ode,
            backend_dependent,
            routine_sources.get(routine),
        );
        output.push(json!([
            safe_path,
            routine,
            feature,
            class,
            lock_scope,
            dispatch,
            callback,
            nested_policy(callback, ode),
            xerror,
            global,
            state_ids,
            external_io,
            if ode {
                "Send_if_callback_and_error_are_Send; otherwise_not_Send"
            } else {
                "not_part_of_function_contract"
            },
            if ode {
                "not_Sync"
            } else {
                "not_part_of_function_contract"
            },
            relaxation,
            "reviewed_policy_record",
        ]));
    }
    output.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    Ok(output)
}

fn mutable_state_ids(
    feature: &str,
    domain: &str,
    ode: bool,
    backend_dependent: bool,
    routine_source: Option<&String>,
) -> Vec<String> {
    let mut ids = vec!["closure-incomplete-selected-provider".to_owned()];
    if backend_dependent && feature.starts_with("blas-") {
        ids.push("provider-blas-runtime-unresolved".to_owned());
    }
    if ode {
        ids.extend([
            "sdriv3-ddriv3-xerror-control".to_owned(),
            "sdriv3-ddriv3-callback-context".to_owned(),
            "ddstp-data-ier".to_owned(),
            "sdstp-data-ier".to_owned(),
        ]);
    } else if !backend_dependent {
        ids.push("legacy-xerror-process-state".to_owned());
        if matches!(
            domain,
            "quadrature" | "roots" | "nonlinear" | "least squares"
        ) {
            ids.push("rust-callback-thread-local-registry".to_owned());
        }
    }
    if let Some(routine_source) = routine_source {
        // Direct routine source records are evidence for the entry point;
        // the closure-level result above covers every transitive source.
        ids.push(format!("source-static-scan-{routine_source}"));
    }
    ids
}

struct MutableStateAudit {
    records: Vec<Value>,
    routine_sources: BTreeMap<String, String>,
}

fn mutable_global_state_records() -> Result<MutableStateAudit> {
    let mut records = vec![
        json!([
            "closure-incomplete-selected-provider",
            "all_selected_native_closures",
            "complete_transitive_source_and_binary_state_scan",
            "PROVIDER",
            true,
            "unknown",
            "process",
            "unknown",
            "unknown",
            "unknown",
            [
                "The locally available verified cache lacks src/ddcor.f; an attempted composed cache then lacked lin/dgbfa.f. No complete selected closure was compiled or symbol-inspected."
            ]
        ]),
        json!([
            "legacy-xerror-process-state",
            "XGETF/XSETF/XERMSG_runtime",
            "legacy_error_control_flag_and_message_runtime",
            "RUNTIME",
            true,
            "explicit",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "crates/slatec/src/runtime.rs: permit_recoverable_native_statuses must execute under lock_native",
                "reviewed SLATEC drivers use XERMSG"
            ]
        ]),
        json!([
            "rust-callback-thread-local-registry",
            "safe_callback_runtime",
            "ACTIVE_* thread-local callback slots",
            "OTHER",
            true,
            "explicit",
            "family",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "crates/slatec/src/callback_runtime.rs: thread_local ACTIVE_* registrations",
                "scoped installers reject nested callback-based native use"
            ]
        ]),
        json!([
            "sdriv3-ddriv3-xerror-control",
            "SDRIV3/DDRIV3_transitive_XERROR",
            "XGETF_XSETF_control_flag",
            "RUNTIME",
            true,
            "explicit",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "crates/slatec/src/ode.rs: scoped recovery guard",
                "src/sdriv3.f and src/ddriv3.f list XERMSG in their subsidiary contracts"
            ]
        ]),
        json!([
            "sdriv3-ddriv3-callback-context",
            "SDRIV3/DDRIV3_safe_trampoline",
            "ODE_ACTIVE_CONTEXT thread-local slot",
            "OTHER",
            true,
            "explicit",
            "family",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "crates/slatec/src/ode.rs: per-call callback context and reentry flag",
                "context is cleared on every native return path"
            ]
        ]),
        json!([
            "ddstp-data-ier",
            "DDSTP",
            "IER",
            "DATA",
            true,
            "compile_time",
            "routine",
            "read_write",
            "wrapper_lock",
            "initialization_race",
            [
                "src/ddstp.f:89 DATA IER /.FALSE./",
                "src/ddstp.f:101 and 127 pass IER to DDNTL/DDPST; initialized local storage is conservatively treated as persistent until compiler/object inspection proves otherwise",
                "generated/providers/provider-index.json: the selected GNU Fortran build uses -std=legacy -ffixed-line-length-none -c and contains no automatic-storage proof"
            ]
        ]),
        json!([
            "sdstp-data-ier",
            "SDSTP",
            "IER",
            "DATA",
            true,
            "compile_time",
            "routine",
            "read_write",
            "wrapper_lock",
            "initialization_race",
            [
                "src/sdstp.f:88 DATA IER /.FALSE./",
                "src/sdstp.f:100 and 126 pass IER to SDNTL/SDPST; initialized local storage is conservatively treated as persistent until compiler/object inspection proves otherwise",
                "generated/providers/provider-index.json: the selected GNU Fortran build uses -std=legacy -ffixed-line-length-none -c and contains no automatic-storage proof"
            ]
        ]),
        json!([
            "provider-blas-runtime-unresolved",
            "BLAS_level1_level2_level3_provider",
            "linked_provider_static_locals_common_blocks_and_runtime_state",
            "PROVIDER",
            true,
            "unknown",
            "process",
            "unknown",
            "unknown",
            "unknown",
            [
                "BLAS safe APIs preserve no_std capability and do not take the hosted runtime lock",
                "no complete provider source-plus-binary inspection is available in this audit"
            ]
        ]),
        json!([
            "fortran-runtime-connected-units",
            "Fortran_runtime",
            "connected_units_and_direct_access_IO_bookkeeping",
            "RUNTIME",
            true,
            "runtime",
            "process",
            "read_write",
            "unknown",
            "unsafe",
            [
                "Fortran connected units are process runtime state",
                "SPLP/DSPLP paging remains deferred and is separately described in lp-paging-policy.json"
            ]
        ]),
        json!([
            "splp-dsplp-la05dd-common",
            "SPLP/DSPLP",
            "LA05DD named COMMON block",
            "COMMON",
            true,
            "first_call",
            "library",
            "read_write",
            "none",
            "unsafe",
            [
                "src/dplpdm.f:39, src/dplpmn.f:152, and src/la05ad.f:61 declare COMMON /LA05DD/",
                "The deferred LP family shares mutable factorization state; it is not a candidate for a parallel-safe classification even apart from conditional direct-access paging."
            ]
        ]),
        json!([
            "dprwvr-prwvir-saved-constants",
            "DPRWVR/PRWVIR",
            "ZERO_ONE initialized SAVEd constants",
            "SAVE",
            false,
            "compile_time",
            "routine",
            "read_only",
            "none",
            "benign_after_init",
            [
                "src/dprwvr.f and src/prwvir.f: SAVE ZERO,ONE; DATA ZERO,ONE",
                "Read-only initialized constants are recorded for completeness but are not themselves a concurrency hazard."
            ]
        ]),
    ];
    let source_files = read_value(repo_path("generated/ffi/selected-source-files.json"))?;
    let closures = read_value(repo_path(
        "crates/slatec-src/metadata/family-source-closure.json",
    ))?;
    let interface = read_value(repo_path("generated/ffi/interface-inventory.json"))?;
    let source_paths = selected_source_paths(&source_files)?;
    let routine_sources = routine_source_ids(&interface, &source_paths)?;
    let mut included = closures["families"]
        .as_object()
        .ok_or_else(|| policy("family source closure lacks families"))?
        .values()
        .flat_map(|sources| sources.as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<std::collections::BTreeSet<_>>();
    // A public raw routine occasionally lives in a physical source file which
    // is not named by a family closure record. Include those direct owners as
    // well so every safe-call state reference resolves to a real audit row.
    included.extend(routine_sources.values().cloned());
    for source_id in &included {
        let Some(path) = source_paths.get(source_id) else {
            return Err(policy(
                "family source closure references unknown selected source",
            ));
        };
        records.push(json!([
            format!("source-static-scan-{source_id}"),
            path,
            "explicit_static_storage_scan_not_completed_from_retained_complete_source_text",
            "OTHER",
            true,
            "unknown",
            "process",
            "unknown",
            "unknown",
            "unknown",
            [format!("selected source id {source_id}; path {path}"), "This deterministic policy audit retains no complete text-and-object scan result for this source. It is deliberately not classified as free of COMMON/SAVE/DATA/BLOCK_DATA/EQUIVALENCE or compiler-generated static storage."]
        ]));
    }
    records.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    Ok(MutableStateAudit {
        records,
        routine_sources,
    })
}

fn selected_source_paths(source_files: &Value) -> Result<BTreeMap<String, String>> {
    let columns = source_files["columns"]
        .as_array()
        .ok_or_else(|| policy("selected source files lacks columns"))?;
    let position = |name: &str| {
        columns
            .iter()
            .position(|column| column.as_str() == Some(name))
            .ok_or_else(|| policy("selected source files missing expected column"))
    };
    let id = position("source_id")?;
    let path = position("source_path")?;
    let mut output = BTreeMap::new();
    for record in source_files["records"]
        .as_array()
        .ok_or_else(|| policy("selected source files lacks records"))?
    {
        let row = record
            .as_array()
            .ok_or_else(|| policy("selected source file record is not an array"))?;
        output.insert(
            value_string(row, id, "selected source id")?,
            value_string(row, path, "selected source path")?,
        );
    }
    Ok(output)
}

fn routine_source_ids(
    interface: &Value,
    source_paths: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>> {
    let columns = interface["columns"]
        .as_array()
        .ok_or_else(|| policy("interface inventory lacks columns"))?;
    let position = |name: &str| {
        columns
            .iter()
            .position(|column| column.as_str() == Some(name))
            .ok_or_else(|| policy("interface inventory missing expected column"))
    };
    let routine = position("normalized_name")?;
    let path = position("source_path")?;
    let mut source_by_path = BTreeMap::new();
    for (id, source_path) in source_paths {
        source_by_path.insert(source_path.as_str(), id.as_str());
    }
    let mut output = BTreeMap::new();
    for record in interface["records"]
        .as_array()
        .ok_or_else(|| policy("interface inventory lacks records"))?
    {
        let row = record
            .as_array()
            .ok_or_else(|| policy("interface record is not an array"))?;
        let source_path = value_string(row, path, "interface source path")?;
        if let Some(source_id) = source_by_path.get(source_path.as_str()) {
            output.insert(
                value_string(row, routine, "interface routine")?,
                (*source_id).to_owned(),
            );
        }
    }
    Ok(output)
}

fn callback_dispatch(domain: &str, feature: &str) -> &'static str {
    if feature == "ode-sdrive-expert" {
        "session_scoped_thread_local_context"
    } else if feature == "nonlinear-jacobian-check" {
        "rust_callbacks_only; no_native_callback"
    } else if matches!(
        domain,
        "quadrature" | "roots" | "nonlinear" | "least squares"
    ) {
        "scoped_thread_local_callback_registry"
    } else {
        "none"
    }
}

fn nested_policy(callback: &str, ode: bool) -> &'static str {
    if ode {
        "reject_same_session_reentry; global_lock_is_reentrant_only_for_non_callback_calls"
    } else if callback != "none" && callback != "rust_callbacks_only; no_native_callback" {
        "reject_nested_callback_based_native_operation"
    } else {
        "not_applicable"
    }
}

#[derive(Clone, Debug)]
struct ArgumentInfo {
    name: String,
    declared_type: String,
    dimensions: Vec<Value>,
    external: bool,
}

fn argument_lookup(index: &Value) -> Result<BTreeMap<String, ArgumentInfo>> {
    let columns = index["columns"]
        .as_array()
        .ok_or_else(|| policy("argument index lacks columns"))?;
    let position = |name: &str| {
        columns
            .iter()
            .position(|column| column.as_str() == Some(name))
            .ok_or_else(|| policy("argument index missing expected column"))
    };
    let id = position("id")?;
    let name = position("normalized_name")?;
    let declared_type = position("declared_type")?;
    let dimensions = position("dimensions")?;
    let external = position("is_external")?;
    let rows = index["records"]
        .as_array()
        .ok_or_else(|| policy("argument index lacks records"))?;
    let mut output = BTreeMap::new();
    for row in rows {
        let values = row
            .as_array()
            .ok_or_else(|| policy("argument index record is not an array"))?;
        let argument_id = value_string(values, id, "argument id")?;
        output.insert(
            argument_id,
            ArgumentInfo {
                name: value_string(values, name, "argument name")?,
                declared_type: values
                    .get(declared_type)
                    .and_then(Value::as_str)
                    .unwrap_or("UNKNOWN")
                    .to_owned(),
                dimensions: values
                    .get(dimensions)
                    .and_then(Value::as_array)
                    .cloned()
                    .ok_or_else(|| policy("argument dimensions are not an array"))?,
                external: values
                    .get(external)
                    .and_then(Value::as_bool)
                    .ok_or_else(|| policy("argument external marker is not bool"))?,
            },
        );
    }
    Ok(output)
}

fn storage_records(
    mappings: &[Value],
    arguments: &BTreeMap<String, ArgumentInfo>,
) -> Result<Vec<Value>> {
    let interface = read_value(repo_path("generated/ffi/interface-inventory.json"))?;
    let units = unit_argument_ids(&interface)?;
    let mut output = Vec::new();
    for mapping in mappings {
        let path = string(mapping, "rust_path")?;
        let routine = string(mapping, "fortran_routine")?;
        let unit = string(mapping, "program_unit_id")?;
        let argument_ids = units
            .get(unit)
            .ok_or_else(|| policy("safe argument mapping references unknown program unit"))?;
        let mapped = mapping["arguments"]
            .as_array()
            .ok_or_else(|| policy("safe argument mapping lacks arguments"))?;
        let mut by_fortran = BTreeMap::new();
        for item in mapped {
            let fortran = string(item, "fortran")?;
            by_fortran.insert(fortran.to_ascii_uppercase(), item);
        }
        for id in argument_ids {
            let argument = arguments
                .get(id)
                .ok_or_else(|| policy("interface inventory references unknown argument"))?;
            let map = by_fortran.get(&argument.name).copied();
            let rust = map
                .and_then(|value| value["rust"].as_str())
                .unwrap_or("internal_or_unexposed");
            let transformation = map
                .and_then(|value| value["transformation"].as_str())
                .unwrap_or("internal");
            let role = storage_role(argument);
            output.push(json!([
                path,
                routine,
                argument.name,
                rust,
                argument.declared_type,
                argument.dimensions.len(),
                role,
                native_layout(argument, role),
                safe_representation(transformation),
                transfer(transformation, role),
                "prohibited",
                validation(argument, role),
                "reviewed_mapping_and_inventory_join",
            ]));
        }
    }
    output.sort_by(|left, right| {
        left[0]
            .as_str()
            .cmp(&right[0].as_str())
            .then_with(|| left[2].as_str().cmp(&right[2].as_str()))
    });
    Ok(output)
}

fn unit_argument_ids(interface: &Value) -> Result<BTreeMap<String, Vec<String>>> {
    let columns = interface["columns"]
        .as_array()
        .ok_or_else(|| policy("interface inventory lacks columns"))?;
    let unit = columns
        .iter()
        .position(|column| column.as_str() == Some("program_unit_id"))
        .ok_or_else(|| policy("interface inventory lacks program_unit_id"))?;
    let arguments = columns
        .iter()
        .position(|column| column.as_str() == Some("argument_ids"))
        .ok_or_else(|| policy("interface inventory lacks argument_ids"))?;
    let mut output = BTreeMap::new();
    for record in interface["records"]
        .as_array()
        .ok_or_else(|| policy("interface inventory lacks records"))?
    {
        let values = record
            .as_array()
            .ok_or_else(|| policy("interface record is not an array"))?;
        let ids = values
            .get(arguments)
            .and_then(Value::as_array)
            .ok_or_else(|| policy("interface argument ids are not an array"))?
            .iter()
            .map(|id| {
                id.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| policy("interface argument id is not a string"))
            })
            .collect::<Result<Vec<_>>>()?;
        output.insert(value_string(values, unit, "program unit id")?, ids);
    }
    Ok(output)
}

fn storage_role(argument: &ArgumentInfo) -> &'static str {
    let name = argument.name.as_str();
    if argument.external {
        "callback"
    } else if matches!(
        name,
        "WORK" | "IWORK" | "WA" | "WA1" | "WA2" | "WA3" | "WA4" | "RWORK" | "IWA" | "IW"
    ) {
        "opaque_workspace"
    } else if argument.dimensions.len() >= 2 {
        "matrix"
    } else if argument.dimensions.len() == 1 {
        "vector"
    } else {
        "scalar"
    }
}

fn native_layout(argument: &ArgumentInfo, role: &str) -> &'static str {
    match role {
        "callback" => "Fortran_external_callback_abi",
        "opaque_workspace" => "opaque_native_work_array",
        "matrix" => "Fortran_column_major",
        "vector"
            if matches!(
                argument.name.as_str(),
                "DX" | "DY" | "SX" | "SY" | "X" | "Y"
            ) =>
        {
            "Fortran_vector_with_possible_increment"
        }
        "vector" => "Fortran_contiguous_vector",
        _ => "Fortran_scalar_by_reference",
    }
}

fn safe_representation(transformation: &str) -> &'static str {
    match transformation {
        "pass" => "borrowed_validated_input_or_output",
        "owned" => "owned_native_storage_same_logical_layout",
        "internal" => "internally_allocated_or_derived_native_storage",
        "callback" => "contained_Rust_callback_trampoline",
        "inferred" => "convenience_wrapper_derives_native_control",
        _ => "unmapped_internal_storage",
    }
}

fn transfer(transformation: &str, role: &str) -> &'static str {
    match (transformation, role) {
        (_, "callback") => "no_data_layout_conversion",
        (_, "opaque_workspace") => "internal_allocation_only",
        ("owned", "matrix") => "preservation_copy_in_Fortran_column_major_order",
        ("owned", _) => "owned_copy_or_reconstruction_without_layout_reordering",
        ("pass", _) => "zero_copy_after_validation",
        ("inferred", _) => "no_buffer_conversion",
        _ => "internal_native_preparation",
    }
}

fn validation(argument: &ArgumentInfo, role: &str) -> &'static str {
    match role {
        "callback" => "scoped_registration_pointer_and_output_protocol",
        "opaque_workspace" => "checked_documented_length_and_integer_conversion",
        "matrix" => "logical_dimensions_leading_dimension_and_slice_length",
        "vector" => "logical_length_stride_and_slice_length",
        _ if argument.declared_type == "CHARACTER" => "selector_character_length_and_value",
        _ => "finite_or_domain_contract_and_integer_conversion_as_applicable",
    }
}

fn matrixpacked_compatibility(snapshot: &str) -> Value {
    json!({
        "schema_id":"slatec.runtime.matrixpacked-compatibility",
        "schema_version":"1.0.0",
        "snapshot_id":snapshot,
        "sources":[
            "https://crates.io/crates/matrixpacked/0.1.0",
            "https://docs.rs/matrixpacked/0.1.0/matrixpacked/",
            "https://docs.rs/nalgebra/0.35.0/nalgebra/"
        ],
        "columns":["ecosystem","published_version","item","physical_layout","current_safe_api_compatibility","adapter_policy","evidence"],
        "records":[
            ["matrixpacked","0.1.0","PackedLower","LAPACK_packed_lower_columns","not_accepted_directly","future_explicit_adapter_only_after_exact_routine_contract_review","packed_triangle_is_not_a_full_rectangular_Fortran_matrix"],
            ["matrixpacked","0.1.0","PackedUpper","LAPACK_packed_upper_columns","not_accepted_directly","future_explicit_adapter_only_after_exact_routine_contract_review","packed_triangle_is_not_a_full_rectangular_Fortran_matrix"],
            ["matrixpacked","0.1.0","PackedSymmetric_or_PackedHermitian","packed_triangle_with_structure_semantics","not_accepted_directly","no_implicit_expansion_or_repacking","structure_and_conjugation_require_routine_specific_review"],
            ["nalgebra","0.35.0","Matrix_and_MatrixView","column_major_owned_or_strided_view","not_accepted_directly","future_adapter_must_validate_contiguity_mutability_and_leading_dimension","view_stride_is_not_equivalent_to_a_contiguous_Fortran_matrix"],
            ["slatec","current","slice_and_checked_view_contract","explicit_column_major_or_vector_stride_per_function","supported","no_hidden_layout_conversion; preservation_copies_only_when_native_mutation_or_workspace_requires","storage-contract-index.json_is_the_per_argument_source_of_truth"]
        ]
    })
}

fn lp_paging_policy(snapshot: &str) -> Value {
    json!({
        "schema_id":"slatec.runtime.lp-paging-policy",
        "schema_version":"1.0.0",
        "snapshot_id":snapshot,
        "status":"reviewed_deferred_no_public_ffi_or_feature",
        "columns":["subject","single_precision","double_precision","source_evidence","policy"],
        "records":[
            ["mathematical_model","SPLP_minimize_c_transpose_x_subject_to_Ax_equals_w_with_typed_x_and_w_bounds","DSPLP_minimize_c_transpose_x_subject_to_Ax_equals_w_with_typed_x_and_w_bounds","src/splp.f_and_src/dsplp.f_prologues","remain_deferred; no_safe_wrapper"],
            ["matrix_callback","USRMAT_one_based_sequential_sparse_column_data","DUSRMT_one_based_sequential_sparse_column_data","src/splp.f_and_src/dsplp.f_DATTRV_description","no_public_callback_abi"],
            ["paging_subsidiary","PRWVIR_single_precision_counterpart; no_SPRWVR_program_unit_in_selected_source","DPRWVR","src/prwvir.f_and_src/dprwvr.f","paging_is_not_a_safe_public_resource_protocol"],
            ["paging_activation","only_when_save_restore_or_matrix_storage_exceeds_high_speed_memory","only_when_save_restore_or_matrix_storage_exceeds_high_speed_memory","src/splp.f_and_src/dsplp.f_Input_Output_files_required","no_claim_that_caller_must_preopen_a_matrix_file"],
            ["unit_selection","option_KEY_54_selects_sparse_page_Fortran_unit_default_1","option_KEY_54_selects_sparse_page_Fortran_unit_default_1","src/splp.f_and_src/dsplp.f_option_54","unit_is_process_global_legacy_resource"],
            ["open_close_behavior","PRWVIR_calls_SOPENM_when_first_page_write_triggers_paging; SCLOSM_is_option_controlled","DPRWVR_calls_SOPENM_when_first_page_write_triggers_paging; SCLOSM_is_option_controlled","src/prwvir.f_src/dprwvr.f_src/sopenm.f_src/sclosm.f","solver_does_not_require_the_caller_to_preopen_unit_1; direct_access_file_can_be_activated_by_paging; no_safe_lifecycle_guarantee"],
            ["save_restore","unit_2_is_only_for_explicit_save_restore_option","unit_2_is_only_for_explicit_save_restore_option","src/splp.f_and_src/dsplp.f_Input_Output_files_required","disabled_in_any_future_default_safe_configuration"],
            ["printing","KEY_51_enables_legacy_output_unit","KEY_51_enables_legacy_output_unit","src/splp.f_and_src/dsplp.f_option_51","must_remain_off_by_default_if_ever_wrapped"]
        ]
    })
}

fn runtime_policy(snapshot: &str, functions: usize, storage_records: usize) -> Value {
    json!({
        "schema_id":"slatec.runtime.storage-policy",
        "schema_version":"1.0.0",
        "snapshot_id":snapshot,
        "safe_function_count":functions,
        "storage_record_count":storage_records,
        "policies":{
            "hosted_native_default":"SerializedGlobal through crate::runtime::lock_native for safe APIs that require std; no routine is advertised thread-safe without source and stress evidence",
            "no_std_alloc_boundary":"BLAS and nonlinear-jacobian-check retain their existing no_std or alloc capability and are BackendDependent rather than claimed thread-safe",
            "callback":"thread-local registrations are scoped; callback-based nested native operations are rejected; Rust panics never unwind through Fortran",
            "xerror":"only specifically reviewed recoverable native status paths temporarily change XERROR; guards restore the prior setting",
            "storage":"native layout is explicit per argument; implicit transpose, densification, packing, repacking, and arbitrary-stride conversion are prohibited",
            "interoperability":"the core API remains slices, Vec, and lightweight checked views; any matrixpacked, nalgebra, ndarray, or faer adapter is optional and must be explicit",
            "linear_programming":"SPLP and DSPLP remain reviewed but deferred; no FFI, provider source closure, feature, or I/O shim is added"
        }
    })
}

fn concurrency_summary(records: &[Value]) -> String {
    let mut classes = BTreeMap::<&str, usize>::new();
    for record in records {
        *classes
            .entry(record[3].as_str().unwrap_or("invalid"))
            .or_default() += 1;
    }
    let rendered = classes
        .iter()
        .map(|(class, count)| format!("`{class}`: {count}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "# Runtime concurrency audit\n\n- Classified safe functions: {} ({rendered}).\n- `SerializedGlobal` records enter the reentrant-per-thread, process-wide native runtime guard. This permits a non-callback native helper inside an active hosted call, but callback-based nested native operations remain rejected.\n- `BackendDependent` records preserve existing `no_std`/`alloc` capability and do not make a Rust thread-safety claim. They require provider provenance, source-level reentrancy evidence, and parallel stress tests before any narrower class is adopted.\n- `SDRIV3`/`DDRIV3` sessions remain `SerializedGlobal`: the session callback context and scoped XERROR state make a per-family or lock-free claim premature.\n- The complete SDRIVE source closure could not be rebuilt from the available offline cache: `DDCOR` was missing first, and the composed local cache also lacked LINPACK `DGBFA`. No source was downloaded, copied into the repository, or treated as a successful native validation.\n",
        records.len()
    )
}

fn storage_summary(records: &[Value]) -> String {
    let matrices = records
        .iter()
        .filter(|record| record[6].as_str() == Some("matrix"))
        .count();
    format!(
        "# Storage interoperability policy\n\n- Audited native arguments: {}; matrix arguments: {matrices}.\n- The core API accepts slices and its existing checked views. A matrix is passed only in the documented Fortran column-major layout with the documented leading-dimension contract.\n- No implicit transpose, dense-to-packed conversion, packed-to-dense expansion, CSR/CSC conversion, arbitrary-stride materialization, or hidden repacking is permitted. An owned preservation copy is allowed only when a reviewed native routine mutates its storage or requires an opaque workspace, and it preserves the logical column-major order.\n- `matrixpacked` 0.1.0 and `nalgebra` 0.35.0 are audited for possible future optional adapters only. Neither is accepted by the current core API. A future adapter must validate exact layout, contiguity/stride, mutability, scalar support, and native leading dimensions; packed triangular storage cannot be passed to a routine expecting a full rectangular matrix.\n- `ndarray` and `faer` remain possible optional integrations, but no core dependency or compatibility promise is introduced.\n",
        records.len()
    )
}

fn records(path: impl AsRef<Path>) -> Result<Vec<Value>> {
    let value = read_value(path)?;
    value["records"]
        .as_array()
        .cloned()
        .ok_or_else(|| policy("metadata file lacks records"))
}

fn read_value(path: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn repo_path(relative: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value[key]
        .as_str()
        .ok_or_else(|| policy("expected metadata string"))
}

fn value_string(values: &[Value], index: usize, label: &str) -> Result<String> {
    values
        .get(index)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("{label} is not a string")))
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Verification(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic_and_classifies_every_safe_function() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let one = generate(first.path()).unwrap();
        let two = generate(second.path()).unwrap();
        assert_eq!(one.function_count, two.function_count);
        assert_eq!(one.semantic_hash, two.semantic_hash);
        for name in [
            "concurrency-index.json",
            "concurrency-audit-summary.md",
            "mutable-global-state-index.json",
            "storage-contract-index.json",
            "storage-interoperability-summary.md",
            "matrixpacked-compatibility.json",
            "lp-paging-policy.json",
            "runtime-storage-policy.json",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
        let concurrency = records(first.path().join("concurrency-index.json")).unwrap();
        assert_eq!(concurrency.len(), one.function_count);
        assert!(concurrency.iter().all(|record| {
            matches!(
                record[3].as_str(),
                Some("SerializedGlobal" | "BackendDependent")
            )
        }));
        let global = records(first.path().join("mutable-global-state-index.json")).unwrap();
        let ids = global
            .iter()
            .filter_map(|record| record[0].as_str())
            .collect::<std::collections::BTreeSet<_>>();
        assert!(concurrency.iter().all(|record| {
            record[10].as_array().is_some_and(|references| {
                references
                    .iter()
                    .all(|id| id.as_str().is_some_and(|id| ids.contains(id)))
            })
        }));
    }

    #[test]
    fn callbacks_storage_and_ode_records_have_required_policy_fields() {
        let temp = tempfile::tempdir().unwrap();
        generate(temp.path()).unwrap();
        let concurrency = records(temp.path().join("concurrency-index.json")).unwrap();
        for record in concurrency.iter().filter(|record| {
            !matches!(
                record[6].as_str(),
                Some("none" | "rust_callbacks_only; no_native_callback")
            )
        }) {
            assert_ne!(record[7].as_str(), Some("not_applicable"));
        }
        let ode = concurrency
            .iter()
            .filter(|record| record[2].as_str() == Some("ode-sdrive-expert"))
            .collect::<Vec<_>>();
        assert_eq!(ode.len(), 2);
        assert!(ode.iter().all(|record| {
            record[3].as_str() == Some("SerializedGlobal")
                && record[4].as_str() == Some("process_global")
                && record[8].as_str() == Some("scoped_XGETF_XSETF_restore")
        }));
        let storage = records(temp.path().join("storage-contract-index.json")).unwrap();
        assert!(!storage.is_empty());
        assert!(
            storage
                .iter()
                .all(|record| record[10].as_str() == Some("prohibited"))
        );
    }

    #[test]
    fn mutable_static_findings_are_conservative_and_complete_for_sdrive() {
        let temp = tempfile::tempdir().unwrap();
        generate(temp.path()).unwrap();
        let state = records(temp.path().join("mutable-global-state-index.json")).unwrap();
        let by_id = |id: &str| state.iter().find(|record| record[0].as_str() == Some(id));
        for id in [
            "closure-incomplete-selected-provider",
            "ddstp-data-ier",
            "sdstp-data-ier",
            "legacy-xerror-process-state",
            "rust-callback-thread-local-registry",
        ] {
            assert!(by_id(id).is_some(), "missing state finding {id}");
        }
        for record in &state {
            assert_eq!(record.as_array().map(Vec::len), Some(11));
            assert!(
                record[10]
                    .as_array()
                    .is_some_and(|evidence| !evidence.is_empty())
            );
        }
    }

    #[test]
    fn lp_remains_deferred_without_a_mandatory_preopen_claim() {
        let temp = tempfile::tempdir().unwrap();
        generate(temp.path()).unwrap();
        let lp = read_value(temp.path().join("lp-paging-policy.json")).unwrap();
        assert_eq!(
            lp["status"].as_str(),
            Some("reviewed_deferred_no_public_ffi_or_feature")
        );
        let rendered = serde_json::to_string(&lp).unwrap();
        assert!(rendered.contains("no_claim_that_caller_must_preopen_a_matrix_file"));
        assert!(!rendered.contains("mandatory_external_file"));
    }
}
