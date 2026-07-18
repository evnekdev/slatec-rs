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
    let projections_value = read_value(repo_path(SAFE_API).join("per-wrapper-native-state.json"))?;
    let projections = projections_value["records"]
        .as_array()
        .ok_or_else(|| policy("per-wrapper native-state index lacks records"))?;
    let projection_by_path = projections
        .iter()
        .filter_map(|record| {
            record["safe_function"]
                .as_str()
                .map(|path| (path.to_owned(), record))
        })
        .collect::<BTreeMap<_, _>>();
    let mutable_state = mutable_global_state_records(&functions)?;
    let concurrency = concurrency_records(
        &functions,
        &mutable_state.routine_sources,
        &projection_by_path,
    )?;
    let relaxation = relaxation_records(projections)?;
    let relaxation_roadmap = relaxation_roadmap(&relaxation);
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
                "columns":["safe_path","fortran_routine","feature","concurrency_class","lock_scope","current_dispatch","callback_dispatch","nested_callback_policy","xerror_policy","native_state_evidence","mutable_global_state_record_ids","external_io_evidence","send_policy","sync_policy","relaxation_gate","review_state","rust_api_concurrency","native_routine_reentrancy","provider_runtime_thread_safety","exact_object_closure","projection_evidence","backend_concurrency_evidence"],
                "records":concurrency,
            }),
        ),
        (
            "concurrency-relaxation-candidates.json",
            json!({
                "schema_id":"slatec.runtime.concurrency-relaxation-candidates",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "behavior_change":"none; roadmap only; global runtime lock remains unchanged",
                "columns":["safe_function","native_entry_points","feature","current_class","outcome","best_possible_class_from_slatec_source","remaining_blockers","evidence"],
                "records":relaxation,
            }),
        ),
        (
            "mutable-global-state-index.json",
            json!({
                "schema_id":"slatec.runtime.mutable-global-state-index",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "coverage":"all_current_safe_records_reference_complete_hash_verified_source_closure_compiler_storage_object_symbol_COMMON_XERROR_and_provider-runtime evidence; no record is promoted to ParallelSafe",
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
        ("concurrency-relaxation-roadmap.md", relaxation_roadmap),
        ("storage-interoperability-summary.md", storage_summary),
    ] {
        fs::write(output_dir.join(name), text.as_bytes())?;
        semantic.extend_from_slice(text.as_bytes());
    }
    let blas1 = crate::blas1_concurrency::generate(output_dir)?;
    semantic.extend_from_slice(blas1.semantic_hash.as_bytes());

    Ok(ResultSummary {
        function_count: functions.len(),
        semantic_hash: hash::bytes(&semantic),
    })
}

fn concurrency_records(
    functions: &[Value],
    routine_sources: &BTreeMap<String, String>,
    projections: &BTreeMap<String, &Value>,
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
            "saved_IER_XERROR_and_callback_context; complete_native_stress_validation_confirms_serialized_execution"
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
            "not_candidate_for_ParallelSafe: saved_IER_XERROR_callback_context_and_provider_runtime evidence require process serialization"
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
        let projection = projections
            .get(safe_path)
            .ok_or_else(|| policy("safe function lacks an exact native-state projection"))?;
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
            projection["rust_api_concurrency"],
            projection["native_routine_reentrancy"],
            projection["provider_runtime_thread_safety"],
            projection["object_closure"],
            "generated/safe-api/per-wrapper-native-state.json",
            if is_qualified_blas1_candidate(feature, routine) {
                "generated/safe-api/backend-concurrency-index.json: ParallelSafe only for source-build-gnu-mingw-reviewed; system/external/unknown remain BackendDependent"
            } else {
                "not_in_BLAS_Level_1_backend_qualification_scope"
            },
        ]));
    }
    output.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    Ok(output)
}

fn relaxation_records(projections: &[Value]) -> Result<Vec<Value>> {
    let mut output = Vec::with_capacity(projections.len());
    for projection in projections {
        let safe = string(projection, "safe_function")?;
        let feature = string(projection, "feature")?;
        let saved = projection["saved_mutable_locals"]
            .as_array()
            .is_some_and(|records| !records.is_empty());
        let common = projection["common_blocks"]
            .as_array()
            .is_some_and(|records| !records.is_empty());
        let xerror = projection["xerror_state"]
            .as_array()
            .is_some_and(|records| !records.is_empty());
        let io = projection["fortran_io"]
            .as_array()
            .is_some_and(|records| !records.is_empty());
        let callback = projection["callback_state"]
            .as_array()
            .is_some_and(|records| records.iter().any(|record| record != "none"));
        let unresolved = projection["source_object_unresolved"]
            .as_array()
            .is_some_and(|records| !records.is_empty());
        let outcome = if saved || common {
            "not_candidate_mutable_native_state"
        } else if io {
            "not_candidate_fortran_io"
        } else if xerror {
            "not_candidate_xerror"
        } else if callback {
            "not_candidate_callback_runtime"
        } else if unresolved || projection["feature_closure_mismatch"] == true {
            "insufficient_evidence"
        } else if feature.starts_with("blas-") || feature == "nonlinear-jacobian-check" {
            "candidate_backend_dependent_parallel"
        } else {
            "candidate_parallel_safe_after_provider_audit"
        };
        output.push(json!([
            safe,
            projection["native_entry_points"],
            feature,
            projection["current_class"],
            outcome,
            projection["best_possible_class_from_slatec_source"],
            projection["remaining_blockers"],
            "generated/safe-api/per-wrapper-native-state.json; generated/safe-api/native-state-crosscheck.json; generated/safe-api/fortran-scanner-validation.json"
        ]));
    }
    output.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    Ok(output)
}

fn relaxation_roadmap(records: &[Value]) -> String {
    let mut counts = BTreeMap::<&str, usize>::new();
    for record in records {
        *counts
            .entry(record[4].as_str().unwrap_or("insufficient_evidence"))
            .or_default() += 1;
    }
    let mut text = String::from(
        "# Concurrency relaxation roadmap\n\nThis report changes no runtime behavior. Every hosted native wrapper remains protected by the process-wide reentrant lock, and `BackendDependent` remains a provider qualification rather than a parallel-safety promise. Independent Rust buffers and `Send` values do not prove native reentrancy.\n\n## Outcomes\n\n",
    );
    for (outcome, count) in counts {
        text.push_str(&format!("- `{outcome}`: {count} wrappers\n"));
    }
    text.push_str(
        "\n## Required evidence\n\nThe 28 BLAS Level 1 records are now qualified `ParallelSafe` only for the exact hash-verified `source-build-gnu-mingw-reviewed` backend and only for independent calls with non-overlapping mutable storage. Their existing direct, `core`-capable dispatch is unchanged. System archives, external linkage, named-but-unselected vendor libraries, and unknown providers remain `BackendDependent`. Every callback, XERROR, ODE, and solver family retains process-global serialization.\n\nAny other future relaxation must have a complete logical-statement source scan, GNU Fortran oracle agreement, bidirectional source/object reconciliation, an exact retained object closure, no reachable XERROR or Fortran-I/O state, and a documented compiler/provider/runtime concurrency contract. Family locks are considered only where mutable state is proved family-local.\n\nStorage layout remains orthogonal: packed storage uses `matrixpacked`, conventional rectangular storage may use standard dense crates, and exact-layout slices/views remain valid without implicit repacking or transpose. The existing LP paging and unit-lifecycle deferral is unchanged.\n",
    );
    text
}

fn is_qualified_blas1_candidate(feature: &str, routine: &str) -> bool {
    feature == "blas-level1"
        && matches!(
            routine,
            "SASUM"
                | "DASUM"
                | "SAXPY"
                | "DAXPY"
                | "SCOPY"
                | "DCOPY"
                | "SDOT"
                | "DDOT"
                | "SSCAL"
                | "DSCAL"
                | "SSWAP"
                | "DSWAP"
                | "ISAMAX"
                | "IDAMAX"
        )
}

fn mutable_state_ids(
    feature: &str,
    domain: &str,
    ode: bool,
    backend_dependent: bool,
    routine_source: Option<&String>,
) -> Vec<String> {
    let mut ids = vec![
        "native-origin-source-closure-complete".to_owned(),
        "native-origin-compiler-storage-model".to_owned(),
        "native-origin-writable-symbol-inspection".to_owned(),
        "native-origin-common-block-audit".to_owned(),
        "xerror-global-state-audit".to_owned(),
    ];
    ids.push(if ode {
        "native-ode-serialization-validation".to_owned()
    } else {
        "native-non-ode-stress-status".to_owned()
    });
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
    } else if !backend_dependent
        && matches!(
            domain,
            "quadrature" | "roots" | "nonlinear" | "least squares"
        )
    {
        ids.push("rust-callback-thread-local-registry".to_owned());
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

struct NativeSourceStatus {
    source_file: String,
    storage: String,
    mutable: bool,
    status: String,
}

fn native_origin_source_statuses() -> Result<BTreeMap<String, NativeSourceStatus>> {
    let status = read_value(repo_path(
        "generated/safe-api/native-origin-audit-status.json",
    ))?;
    for field in [
        "source_closure",
        "object_inspection",
        "compiler_storage_model",
    ] {
        if status[field].as_str() != Some("complete_mutable_state_found") {
            return Err(policy("native-origin audit evidence is incomplete"));
        }
    }
    if status["common_blocks"].as_str().is_none() || status["xerror"].as_str().is_none() {
        return Err(policy(
            "native-origin audit lacks COMMON or XERROR evidence",
        ));
    }
    let index = read_value(repo_path(
        "generated/safe-api/native-source-scan-index.json",
    ))?;
    let columns = index["columns"]
        .as_array()
        .ok_or_else(|| policy("native source scan lacks columns"))?;
    let position = |name: &str| {
        columns
            .iter()
            .position(|column| column.as_str() == Some(name))
            .ok_or_else(|| policy("native source scan lacks expected column"))
    };
    let id = position("id")?;
    let source_file = position("source_file")?;
    let state = position("status")?;
    let findings = position("finding_count")?;
    let mut output = BTreeMap::new();
    for row in index["records"]
        .as_array()
        .ok_or_else(|| policy("native source scan lacks records"))?
    {
        let row = row
            .as_array()
            .ok_or_else(|| policy("native source scan record is not an array"))?;
        let audit_status = value_string(row, state, "native source scan status")?;
        if !matches!(
            audit_status.as_str(),
            "complete_no_mutable_state_found" | "complete_mutable_state_found"
        ) {
            return Err(policy("native source scan record is incomplete"));
        }
        let finding_count = row
            .get(findings)
            .and_then(Value::as_u64)
            .ok_or_else(|| policy("native source scan finding count is not unsigned"))?;
        output.insert(
            value_string(row, id, "native source scan id")?,
            NativeSourceStatus {
                source_file: value_string(row, source_file, "native source scan path")?,
                storage: format!(
                    "complete_source_and_object_scan:{}:{}",
                    value_string(row, source_file, "native source scan path")?,
                    finding_count
                ),
                mutable: audit_status == "complete_mutable_state_found",
                status: audit_status,
            },
        );
    }
    Ok(output)
}

fn mutable_global_state_records(functions: &[Value]) -> Result<MutableStateAudit> {
    let native_sources = native_origin_source_statuses()?;
    let mut records = vec![
        json!([
            "native-origin-source-closure-complete",
            "all_selected_native_closures",
            "hash_verified_transitive_source_closure",
            "PROVIDER",
            true,
            "explicit",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "generated/safe-api/native-origin-audit-status.json: complete_mutable_state_found",
                "generated/safe-api/native-source-origin-index.json: all 330 provider sources are hash verified through the reviewed acquisition receipt; three project machine-constant sources are compiled separately"
            ]
        ]),
        json!([
            "native-origin-compiler-storage-model",
            "GNU_Fortran_14_2_0_x86_64_w64_mingw32",
            "ordinary_locals_automatic_DATA_and_SAVE_locals_persistent",
            "COMPILER",
            true,
            "compile_time",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "generated/safe-api/fortran-storage-model.json",
                "exact flags -x f77 -std=legacy -ffixed-line-length-none -c; no -fno-automatic or -frecursive",
                "storage probe: ordinary local has no writable symbol; DATA local has local writable .bss symbol"
            ]
        ]),
        json!([
            "native-origin-writable-symbol-inspection",
            "all_selected_native_objects",
            "COFF_data_and_bss_symbols",
            "OBJECT",
            true,
            "compile_time",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "generated/safe-api/native-writable-symbol-index.json",
                "every selected source and project profile object is compiled and inspected; unknown writable symbols remain conservative"
            ]
        ]),
        json!([
            "native-origin-common-block-audit",
            "all_selected_native_closures",
            "COMMON_block_ownership_and_layout",
            "COMMON",
            false,
            "explicit",
            "process",
            "read_only",
            "wrapper_lock",
            "complete_no_mutable_state_found",
            [
                "generated/safe-api/common-block-index.json: complete_no_mutable_state_found for the current selected source closure"
            ]
        ]),
        json!([
            "native-ode-serialization-validation",
            "SDRIV3_DDRIV3",
            "native_entry_active_counter",
            "OTHER",
            true,
            "explicit",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "crates/slatec/tests/ode_sdrive_native.rs: native concurrency test records a maximum of one active native entry under multiple Rust threads",
                "global serialization is a race-prevention policy, not reentrancy proof"
            ]
        ]),
        json!([
            "native-non-ode-stress-status",
            "non_ODE_safe_wrappers",
            "routine_specific_parallel_native_stress_status",
            "OTHER",
            false,
            "explicit",
            "process",
            "read_only",
            "unknown",
            "provider_unknown",
            [
                "This milestone executes instrumented native concurrency stress only for SDRIV3/DDRIV3.",
                "No non-ODE routine is promoted to ParallelSafe; hosted calls remain SerializedGlobal and no_std provider calls remain BackendDependent."
            ]
        ]),
        json!([
            "xerror-global-state-audit",
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
                "generated/safe-api/xerror-state-index.json: J4SAVE IPARAM and XERSVE tables are saved process-global state",
                "reviewed selected closures use XERMSG; hosted wrappers that scope XERROR hold lock_native for the full restoration interval"
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
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "src/ddstp.f:89 DATA IER /.FALSE./",
                "DDNTL resets IER false and sets it true on callback, factorization, singular-diagonal, and related failure paths; DDPST resets and sets it on factorization or USERS failure",
                "generated/safe-api/native-writable-symbol-index.json: ddstp object emits local writable ier.0 in .bss; local linkage is one shared process-image instance"
            ]
        ]),
        json!([
            "sdstp-data-ier",
            "SDSTP",
            "IER",
            "DATA",
            true,
            "compile_time",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "src/sdstp.f:88 DATA IER /.FALSE./",
                "SDNTL resets IER false and sets it true on callback, factorization, singular-diagonal, and related failure paths; SDPST resets and sets it on factorization or USERS failure",
                "generated/safe-api/native-writable-symbol-index.json: sdstp object emits local writable ier.0 in .bss; local linkage is one shared process-image instance"
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
    let source_by_routine = routine_source_ids(&interface, &source_paths)?;
    let safe_routines = functions
        .iter()
        .map(|function| string(function, "fortran_routine").map(str::to_owned))
        .collect::<Result<std::collections::BTreeSet<_>>>()?;
    let mut routine_sources = BTreeMap::new();
    for (routine, source_id) in source_by_routine {
        if !safe_routines.contains(&routine) {
            continue;
        }
        let source_path = source_paths
            .get(&source_id)
            .ok_or_else(|| policy("safe routine source lacks a selected source path"))?;
        let reviewed_id = if native_sources.contains_key(&source_id) {
            source_id
        } else {
            native_sources
                .iter()
                .find(|(_, status)| status.source_file == source_path.as_str())
                .map(|(id, _)| id.clone())
                .ok_or_else(|| {
                    policy("safe routine source is absent from the native-origin source closure")
                })?
        };
        routine_sources.insert(routine, reviewed_id);
    }
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
            if let Some(audited) = native_sources.get(source_id) {
                records.push(json!([
                    format!("source-static-scan-{source_id}"),
                    audited.source_file,
                    audited.storage,
                    "OTHER",
                    audited.mutable,
                    "explicit",
                    "process",
                    if audited.mutable { "read_write" } else { "read_only" },
                    "wrapper_lock",
                    audited.status,
                    [format!("selected overlay source id {source_id}"), format!("generated/safe-api/native-source-scan-index.json: {}", audited.status), "complete source and object evidence is required by the native-origin audit gate"]
                ]));
                continue;
            }
            return Err(policy(&format!(
                "family source closure references unknown selected source {source_id}"
            )));
        };
        let audited = native_sources.get(source_id).ok_or_else(|| {
            policy(&format!(
                "selected safe source {source_id} is absent from the complete native-origin scan"
            ))
        })?;
        records.push(json!([
            format!("source-static-scan-{source_id}"),
            path,
            audited.storage,
            "OTHER",
            audited.mutable,
            "explicit",
            "process",
            if audited.mutable {
                "read_write"
            } else {
                "read_only"
            },
            "wrapper_lock",
            audited.status,
            [
                format!("selected source id {source_id}; path {path}"),
                format!(
                    "generated/safe-api/native-source-scan-index.json: {}",
                    audited.status
                ),
                "complete source and object evidence is required by the native-origin audit gate"
            ]
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
        "# Runtime concurrency audit\n\n- Classified safe functions: {} ({rendered}).\n- `SerializedGlobal` records enter the reentrant-per-thread, process-wide native runtime guard. This permits a non-callback native helper inside an active hosted call, but callback-based nested native operations remain rejected.\n- `BackendDependent` records preserve existing `no_std`/`alloc` capability and do not make a Rust thread-safety claim. They require provider provenance, source-level reentrancy evidence, and parallel stress tests before any narrower class is adopted.\n- Rust ownership safety, exact retained-native-closure reentrancy, and provider/runtime thread safety are separate fields; independent buffers do not prove native reentrancy.\n- The origin audit hash-verifies and scans 330 provider sources, compiles and inspects their objects plus three profile objects, records no selected-closure COMMON block, and separately preserves 172 COMMON declarations from the 1,452-file reviewed corpus. GNU Fortran parse-tree validation reports no selected-source scanner disagreement.\n- `SDRIV3`/`DDRIV3` sessions remain `SerializedGlobal`: `SDSTP`/`DDSTP` emit saved writable `IER` storage, XERROR is process-global, callback dispatch is scoped, and provider/runtime reentrancy is not a public guarantee. Cross-family native instrumentation observes at most one active hosted native lock scope.\n",
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
            "concurrency-relaxation-candidates.json",
            "concurrency-relaxation-roadmap.md",
            "function-family-ownership.json",
            "backend-concurrency-index.json",
            "blas1-concurrency-evidence.json",
            "blas1-provider-qualification.json",
            "blas1-concurrency-audit-summary.md",
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
        assert!(concurrency.iter().all(|record| {
            record[16].is_string()
                && record[17].is_string()
                && record[18].is_string()
                && record[19]
                    .as_array()
                    .is_some_and(|objects| !objects.is_empty())
        }));
        let candidates =
            records(first.path().join("concurrency-relaxation-candidates.json")).unwrap();
        assert_eq!(candidates.len(), one.function_count);
        assert!(candidates.iter().all(|record| {
            matches!(
                record[4].as_str(),
                Some(
                    "not_candidate_mutable_native_state"
                        | "not_candidate_xerror"
                        | "not_candidate_callback_runtime"
                        | "not_candidate_fortran_io"
                        | "candidate_for_family_lock"
                        | "candidate_backend_dependent_parallel"
                        | "candidate_parallel_safe_after_provider_audit"
                        | "insufficient_evidence"
                )
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
            "native-origin-source-closure-complete",
            "native-origin-compiler-storage-model",
            "native-origin-writable-symbol-inspection",
            "native-origin-common-block-audit",
            "xerror-global-state-audit",
            "native-ode-serialization-validation",
            "ddstp-data-ier",
            "sdstp-data-ier",
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
