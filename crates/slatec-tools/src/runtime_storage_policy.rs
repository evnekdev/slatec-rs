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
    let mut projections = projections_value["records"]
        .as_array()
        .ok_or_else(|| policy("per-wrapper native-state index lacks records"))?
        .clone();
    // The PCHIP closure has focused source/object evidence independent of the
    // prior broad provider audit.  Merge those conservative projections here
    // so the global policy cannot accidentally omit a newly safe public API.
    projections.extend(crate::safe_pchip::native_state_projections()?);
    projections.extend(crate::safe_banded::native_state_projections()?);
    projections.extend(crate::safe_bspline::native_state_projections()?);
    projections.extend(crate::safe_piecewise_polynomial::native_state_projections()?);
    projections.extend(crate::safe_fftpack_complex::native_state_projections()?);
    projections.extend(crate::safe_fishpack::native_state_projections()?);
    projections.extend(crate::safe_pois3d::native_state_projections()?);
    projections.extend(crate::safe_dassl::native_state_projections()?);
    projections.extend(crate::safe_special::native_state_projections()?);
    projections.extend(crate::safe_callback_drivers::native_state_projections()?);
    // Focused projections are appended after a prior broad audit.  Replace a
    // stale broad record with the focused record for the same safe function,
    // rather than emitting duplicate roadmap/concurrency rows.
    let mut projections_by_path = BTreeMap::new();
    for projection in projections {
        projections_by_path.insert(string(&projection, "safe_function")?.to_owned(), projection);
    }
    let projections = projections_by_path.into_values().collect::<Vec<_>>();
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
    let relaxation = relaxation_records(&projections)?;
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
        let dassl = feature == "dassl";
        let session_callback = ode || dassl;
        let lp = feature == "optimization-linear-programming-in-memory";
        let fftpack_real = feature == "fftpack-real";
        let fftpack_complex = feature == "fftpack-complex";
        let fftpack = fftpack_real || fftpack_complex;
        let pchip = feature == "pchip";
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
        let xerror = if lp {
            "scoped_XGETF_XSETF_and_XGETUA_XSETUA_restore"
        } else if session_callback || pchip {
            "scoped_XGETF_XSETF_restore"
        } else if feature.starts_with("least-squares") {
            "scoped_XGETF_XSETF_restore_where_reviewed"
        } else if fftpack_complex {
            "not_reachable_in_exact_complex_FFTPACK_closure"
        } else if fftpack_real {
            "not_reachable_in_exact_real_FFTPACK_closure"
        } else if backend_dependent {
            "native_default_not_scoped"
        } else {
            "native_default_or_checked_precondition; no_general_interception_claim"
        };
        let global = if lp {
            "LA05DD_or_LA05DS_COMMON_saved_LA05_state_XERROR_and_thread_local_sparse_callback; serialized"
        } else if backend_dependent {
            "source_level_reentrancy_not_proven; safe_wrapper_exposes_no_global_dispatch"
        } else if ode {
            "saved_IER_XERROR_and_callback_context; complete_native_stress_validation_confirms_serialized_execution"
        } else if dassl {
            "saved_DASSL_DATA_state_XERROR_and_callback_context; serialized"
        } else if fftpack_complex {
            "CFFTI1_NTRYH_is_SAVE_DATA_and_emitted_as_writable_local_data; no_source_writer_was_found_but_calls_remain_serialized_conservatively"
        } else if fftpack_real {
            "RFFTI1_and_EZFFT1_NTRYH_are_SAVE_DATA_tables_emitted_as_writable_local_data; no_source_writer_was_found_but_calls_remain_serialized_conservatively"
        } else if pchip {
            "PCHIP_SAVE_DATA_constants_and_process_global_XERROR; serialized"
        } else if callback != "none" {
            "thread_local_callback_registration_and_legacy_runtime; serialized"
        } else {
            "selected_SLATEC_FNLIB_or_driver_state_may_include_SAVE_COMMON_or_XERROR; serialized_conservatively"
        };
        let external_io = if lp {
            "paging_and_open_close_implementations_excluded; no_IO_forbidden_entry_symbols; valid_calls_preflight_resident_capacity"
        } else if ode {
            "no_mandatory_file_protocol_in_reviewed_SDRIV3_DDRIV3_scope"
        } else if dassl {
            "no_OPEN_CLOSE_or_retained_external_file_protocol_in_reviewed_DASSL_scope"
        } else if fftpack_complex {
            "none_in_exact_complex_FFTPACK_closure"
        } else if fftpack_real {
            "none_in_exact_real_FFTPACK_closure"
        } else if pchip {
            "none_in_exact_PCHIP_closure"
        } else {
            "not_a_public_resource_contract; source-closure-specific audit_required_before_relaxation"
        };
        let relaxation = if lp {
            "not_candidate_for_ParallelSafe: mutable_LA05_COMMON_saved_state_XERROR_callback_dispatch_and_provider_runtime_require_process_serialization"
        } else if backend_dependent {
            "require_provider_source_provenance_and_parallel_stress_test_before_threadsafe_claim"
        } else if ode {
            "not_candidate_for_ParallelSafe: saved_IER_XERROR_callback_context_and_provider_runtime evidence require process serialization"
        } else if dassl {
            "not_candidate_for_ParallelSafe: DASSL_SAVE_DATA_XERROR_callback_context_and_provider_runtime evidence require process serialization"
        } else if fftpack_complex {
            "not_candidate_mutable_native_state: CFFTI1_NTRYH_is_SAVEd_DATA_storage_with_a_writable_object_symbol; no_parallel_native_stress_evidence"
        } else if fftpack_real {
            "not_candidate_mutable_native_state: RFFTI1_and_EZFFT1_NTRYH_are_SAVEd_DATA_tables_with_writable_object_symbols; no_parallel_native_stress_evidence"
        } else if pchip {
            "not_candidate_mutable_native_state: PCHIP_SAVE_DATA_storage_and_XERROR_require_process_serialization"
        } else {
            "require_complete_source_closure_audit_XERROR_audit_and_parallel_stress_test"
        };
        let state_ids = mutable_state_ids(
            feature,
            domain,
            ode,
            dassl,
            backend_dependent,
            fftpack,
            routine_sources.get(routine),
        );
        let projection = projections.get(safe_path).ok_or_else(|| {
            CorpusError::Verification(format!(
                "safe function lacks an exact native-state projection: {safe_path}"
            ))
        })?;
        output.push(json!([
            safe_path,
            routine,
            feature,
            class,
            lock_scope,
            dispatch,
            callback,
            nested_policy(callback, session_callback),
            xerror,
            global,
            state_ids,
            external_io,
            if dassl {
                "Send_if_residual_is_Send"
            } else if ode {
                "Send_if_callback_and_error_are_Send; otherwise_not_Send"
            } else if lp {
                "owned_problem_values_may_be_Send; solve_remains_process_serialized"
            } else {
                "not_part_of_function_contract"
            },
            if session_callback {
                "not_Sync"
            } else if lp {
                "immutable_problem_may_be_Sync; native_entries_remain_serialized"
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
        "\n## Required evidence\n\nThe 28 BLAS Level 1 records are now qualified `ParallelSafe` only for the exact hash-verified `source-build-gnu-mingw-reviewed` backend and only for independent calls with non-overlapping mutable storage. Their existing direct, `core`-capable dispatch is unchanged. System archives, external linkage, named-but-unselected vendor libraries, and unknown providers remain `BackendDependent`. Every callback, XERROR, ODE, LP, and solver family retains process-global serialization.\n\nAny other future relaxation must have a complete logical-statement source scan, GNU Fortran oracle agreement, bidirectional source/object reconciliation, an exact retained object closure, no reachable XERROR or Fortran-I/O state, and a documented compiler/provider/runtime concurrency contract. Family locks are considered only where mutable state is proved family-local.\n\nStorage layout remains orthogonal: packed storage uses `matrixpacked`, conventional rectangular storage may use standard dense crates, and exact-layout slices/views remain valid without implicit repacking or transpose. LP external paging and unit lifecycle remain deferred; the current safe LP subset preflights resident capacity and contains no file-I/O implementation.\n",
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
    dassl: bool,
    backend_dependent: bool,
    fftpack: bool,
    routine_source: Option<&String>,
) -> Vec<String> {
    let mut ids = vec![
        "native-origin-source-closure-complete".to_owned(),
        "native-origin-compiler-storage-model".to_owned(),
        "native-origin-writable-symbol-inspection".to_owned(),
        "native-origin-common-block-audit".to_owned(),
        "xerror-global-state-audit".to_owned(),
    ];
    ids.push(if ode || dassl {
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
    } else if dassl {
        ids.extend([
            "dassl-xerror-control".to_owned(),
            "dassl-callback-context".to_owned(),
            "dassl-data-saved-state".to_owned(),
        ]);
    } else if !backend_dependent
        && matches!(
            domain,
            "quadrature" | "roots" | "nonlinear" | "least squares" | "linear programming"
        )
    {
        ids.push("rust-callback-thread-local-registry".to_owned());
    }
    if fftpack {
        if feature == "fftpack-complex" {
            ids.push("cffti1-data-ntryh".to_owned());
        } else {
            ids.extend([
                "rffti1-data-ntryh".to_owned(),
                "ezfft1-data-ntryh".to_owned(),
            ]);
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
    // PCHIP was added after the last broad native-origin audit.  Its narrow
    // closure is nevertheless fully source-reviewed and every PCHIP object
    // was inspected with the same GNU MinGW command recorded by that audit.
    // Merge this conservative, family-local evidence rather than pretending
    // that an unrelated broad provider build scanned these sources.  Any
    // later broad audit may replace these records by using the same source
    // identifiers; the first complete record remains authoritative.
    let pchip_closure = read_value(repo_path(
        "crates/slatec-src/metadata/pchip-source-closure.json",
    ))?;
    let pchip_sources = pchip_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("PCHIP closure lacks source records"))?;
    for source in pchip_sources {
        let id = string(source, "id")?.to_owned();
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: string(source, "path")?.to_owned(),
            storage: format!(
                "focused_pchip_source_and_object_audit:{}:all_32_PCHIP_objects_inspected",
                string(source, "path")?
            ),
            // The closure is intentionally conservative.  Several numerical
            // units have saved DATA values and every exposed error path can
            // reach XERROR, so no individual PCHIP entry can be a parallel
            // candidate even when its own object has no named writable symbol.
            mutable: true,
            status: "complete_mutable_state_found".to_owned(),
        });
    }
    // The B-spline representation profile is likewise focused after the
    // prior broad archive audit. Its complete small closure was source-
    // reviewed and linked with the reviewed compiler profile; XERROR and the
    // initialized BSQAD/DBSQAD quadrature tables retain the conservative
    // serialization classification.
    let bspline_closure = read_value(repo_path(
        "crates/slatec-src/metadata/bspline-source-closure.json",
    ))?;
    let bspline_sources = bspline_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("B-spline closure lacks source records"))?;
    for source in bspline_sources {
        let id = string(source, "id")?.to_owned();
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: string(source, "path")?.to_owned(),
            storage: format!(
                "focused_bspline_source_and_link_audit:{}:BVALU_BSQAD_and_XERROR_closure_inspected",
                string(source, "path")?
            ),
            mutable: true,
            status: "complete_mutable_state_found".to_owned(),
        });
    }
    // PP evaluation and B-spline conversion use a focused closure that was
    // inspected with the same reviewed compiler profile.  The PP numerical
    // routines themselves have no persistent numerical storage, but their
    // reachable XERROR path remains a process-global serialization blocker.
    let pp_closure = read_value(repo_path(
        "crates/slatec-src/metadata/piecewise-polynomial-source-closure.json",
    ))?;
    let pp_sources = pp_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("piecewise-polynomial closure lacks source records"))?;
    for source in pp_sources {
        let id = string(source, "id")?.to_owned();
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: string(source, "path")?.to_owned(),
            storage: format!(
                "focused_piecewise_polynomial_source_and_link_audit:{}:PP_BSPPP_and_XERROR_closure_inspected",
                string(source, "path")?
            ),
            mutable: true,
            status: "complete_mutable_state_found".to_owned(),
        });
    }
    // The scalar-expanded special profile was added after the previously
    // committed broad audit.  It is intentionally conservative pending that
    // audit's regeneration: source review and the GNU MinGW link probe show
    // the closure is complete, while every wrapper remains serialized because
    // it reaches SAVEd/DATA coefficient state and, where documented, XERROR.
    let special_closure = read_value(repo_path(
        "crates/slatec-src/metadata/special-scalar-expanded-source-closure.json",
    ))?;
    let special_sources = special_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("scalar-expanded closure lacks source records"))?;
    for source in special_sources {
        let id = string(source, "id")?.to_owned();
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: string(source, "path")?.to_owned(),
            storage: format!(
                "focused_special_source_and_link_audit:{}:SAVE_DATA_and_XERROR_conservative",
                string(source, "path")?
            ),
            mutable: true,
            status: "complete_mutable_state_found".to_owned(),
        });
    }
    // Callback-driver wrappers were reviewed after the broad native-origin
    // archive audit. Their exact provider closures are hash guarded in the
    // family manifest and were source-built and link-tested as a focused set.
    // Keep the conservative serialized status until a later broad scan records
    // the same entries, rather than accepting a missing ownership projection.
    for source in crate::safe_callback_drivers::focused_native_sources()? {
        let id = string(&source, "id")?.to_owned();
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: string(&source, "path")?.to_owned(),
            storage: format!(
                "focused_callback_driver_source_build_and_link_audit:{}:XERROR_and_callback_dispatch_serialized",
                string(&source, "path")?
            ),
            mutable: true,
            status: "complete_mutable_state_found".to_owned(),
        });
    }
    // General-band diagnostics extend the existing narrow banded closure.
    // Focused source review covers both precisions, the required BLAS entries,
    // and the newly selected GBCO/GBDI roots.  They have no persistent
    // numerical storage, error runtime, callback, or I/O path; the API stays
    // serialized until a provider/runtime concurrency qualification exists.
    let banded_closure = read_value(repo_path(
        "crates/slatec-src/metadata/banded-linear-systems-source-closure.json",
    ))?;
    let banded_sources = banded_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("banded closure lacks source records"))?;
    for source in banded_sources {
        let id = string(source, "id")?.to_owned();
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: string(source, "path")?.to_owned(),
            storage: format!(
                "focused_banded_source_and_link_audit:{}:no_COMMON_SAVE_DATA_XERROR_or_IO",
                string(source, "path")?
            ),
            mutable: false,
            status: "complete_no_mutable_state_found".to_owned(),
        });
    }
    // The broad native-origin retry is recorded separately.  Until it emits a
    // completed archive report containing DASSL, retain focused closure
    // evidence instead of allowing a missing record to imply reentrancy.
    let dassl_closure = read_value(repo_path(
        "crates/slatec-src/metadata/dassl-source-closure.json",
    ))?;
    let dassl_sources = dassl_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("DASSL closure lacks source records"))?;
    for source in dassl_sources {
        let id = string(source, "id")?.to_owned();
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: string(source, "path")?.to_owned(),
            storage: format!(
                "focused_dassl_source_and_object_audit:{}:DASSL_closure_inspected",
                string(source, "path")?
            ),
            mutable: true,
            status: "complete_mutable_state_found".to_owned(),
        });
    }
    // The standard complex FFTPACK closure is deliberately narrow and uses
    // only real-array entry points.  Its focused source/object audit found
    // CFFTI1's SAVEd DATA factor table in writable local storage; the remaining
    // CFFTF1/CFFTB1 and PASS* units have no persistent native state.  Keeping
    // each source record here lets the global policy project this exact closure
    // without accidentally borrowing state from unrelated fishfft routines.
    let fftpack_complex_closure = read_value(repo_path(
        "crates/slatec-src/metadata/fftpack-complex-source-closure.json",
    ))?;
    let fftpack_complex_sources = fftpack_complex_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("complex FFTPACK closure lacks source records"))?;
    for source in fftpack_complex_sources {
        let id = string(source, "id")?.to_owned();
        let path = string(source, "path")?;
        let has_saved_table = path == "cffti1.f";
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: path.to_owned(),
            storage: if has_saved_table {
                "focused_fftpack_complex_source_and_object_audit:cffti1.f:NTRYH_SAVE_DATA_writable_local".to_owned()
            } else {
                format!(
                    "focused_fftpack_complex_source_and_object_audit:{path}:no_persistent_native_storage"
                )
            },
            mutable: has_saved_table,
            status: if has_saved_table {
                "complete_mutable_state_found".to_owned()
            } else {
                "complete_no_mutable_state_found".to_owned()
            },
        });
    }
    // HWSCRT's Cartesian FISHPACK closure is likewise narrow and has focused
    // source plus GNU MinGW link evidence.  It contains no mutable COMMON,
    // SAVE, DATA, XERROR, Fortran I/O, or callback path, including through
    // GENBUN and the direct subsidiary solvers.  Retain the runtime lock until
    // provider/runtime reentrancy is qualified independently.
    let fishpack_closure = read_value(repo_path(
        "crates/slatec-src/metadata/fishpack-cartesian-2d-source-closure.json",
    ))?;
    let fishpack_sources = fishpack_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("Cartesian FISHPACK closure lacks source records"))?;
    for source in fishpack_sources {
        let id = string(source, "id")?.to_owned();
        let path = string(source, "path")?;
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: path.to_owned(),
            storage: format!(
                "focused_fishpack_cartesian_source_and_link_audit:{path}:no_COMMON_SAVE_DATA_XERROR_or_IO"
            ),
            mutable: false,
            status: "complete_no_mutable_state_found".to_owned(),
        });
    }
    let pois3d_closure = read_value(repo_path(
        "crates/slatec-src/metadata/fishpack-pois3d-source-closure.json",
    ))?;
    let pois3d_sources = pois3d_closure["sources"]
        .as_array()
        .ok_or_else(|| policy("POIS3D closure lacks source records"))?;
    for source in pois3d_sources {
        let id = string(source, "id")?.to_owned();
        let path = string(source, "path")?;
        output.entry(id).or_insert(NativeSourceStatus {
            source_file: path.to_owned(),
            storage: format!(
                "focused_fishpack_pois3d_source_and_link_audit:{path}:no_COMMON_SAVE_statement_DATA_XERROR_or_IO"
            ),
            mutable: false,
            status: "complete_no_mutable_state_found".to_owned(),
        });
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
            "dassl-xerror-control",
            "SDASSL_DDASSL_transitive_XERROR",
            "XGETF_XSETF_control_flag",
            "RUNTIME",
            true,
            "explicit",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "crates/slatec/src/dassl.rs: scoped permit_recoverable_native_statuses guard",
                "SDASSL/DDASSL source and the reviewed closure reach XERMSG on invalid-contract paths"
            ]
        ]),
        json!([
            "dassl-callback-context",
            "SDASSL_DDASSL_safe_trampoline",
            "ACTIVE_DASSL_CONTEXT thread-local slot",
            "OTHER",
            true,
            "explicit",
            "family",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "crates/slatec/src/dassl.rs: per-call residual context is installed only while the process runtime lock is held",
                "the installer rejects nested callback-based native DASSL invocation and clears every exit path"
            ]
        ]),
        json!([
            "dassl-data-saved-state",
            "SDAINI_DDAINI_SDASTP_DDASTP",
            "MAXIT_MJAC_DAMP_XRATE",
            "DATA",
            true,
            "compile_time",
            "process",
            "read_write",
            "wrapper_lock",
            "unsafe",
            [
                "src/sdaini.f and src/ddaini.f: DATA MAXIT/10/,MJAC/5/,DAMP/.75/",
                "src/sdastp.f and src/ddastp.f: DATA MAXIT/4/,XRATE/.25/",
                "initialized DATA locals have saved process-image lifetime under the reviewed GNU MinGW storage model; serialization does not establish reentrancy"
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
            "rffti1-data-ntryh",
            "RFFTI1",
            "NTRYH initialized factor-order table",
            "DATA",
            true,
            "compile_time",
            "process",
            "read_only",
            "wrapper_lock",
            "benign_after_init",
            [
                "fishfft/rffti1.f: SAVE NTRYH; DATA NTRYH /4,2,3,5/",
                "generated/safe-api/native-writable-symbol-index.json: rffti1 object emits local writable ntryh.0 in .data",
                "No executable writer is present in the complete reviewed real FFTPACK closure; local linkage is nevertheless one process-image instance."
            ]
        ]),
        json!([
            "ezfft1-data-ntryh",
            "EZFFT1",
            "NTRYH initialized factor-order table",
            "DATA",
            true,
            "compile_time",
            "process",
            "read_only",
            "wrapper_lock",
            "benign_after_init",
            [
                "fishfft/ezfft1.f: SAVE NTRYH; DATA NTRYH /4,2,3,5/",
                "generated/safe-api/native-writable-symbol-index.json: ezfft1 object emits local writable ntryh.0 in .data",
                "No executable writer is present in the complete reviewed real FFTPACK closure; local linkage is nevertheless one process-image instance."
            ]
        ]),
        json!([
            "cffti1-data-ntryh",
            "CFFTI1",
            "NTRYH initialized factor-order table",
            "SAVE+DATA",
            true,
            "compile_time",
            "process",
            "read_only",
            "wrapper_lock",
            "benign_after_init",
            [
                "fishfft/cffti1.f: SAVE NTRYH; DATA NTRYH /3,4,2,5/",
                "focused complex FFT closure audit: no executable source writer is present",
                "The local symbol has process-image lifetime, so the hosted lock remains in force pending an independent parallel audit."
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
                "SPLP/DSPLP safe calls exclude real paging and file-I/O implementations; XERROR still owns process-global output-unit state"
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
                "The safe in-memory LP family shares mutable factorization state; it remains SerializedGlobal independently of the excluded paging path."
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
    let source_paths = selected_source_paths(&source_files)?;
    let mut closures = read_value(repo_path(
        "crates/slatec-src/metadata/family-source-closure.json",
    ))?;
    // Linkage metadata may retain a compiler-observed `unclassified` bucket
    // for non-public source units. It is not a safe API family and must not
    // widen this audit beyond reviewed public closures.
    closures["families"]
        .as_object_mut()
        .ok_or_else(|| policy("family source closure lacks families"))?
        .remove("unclassified");
    let mut overlay_aliases = BTreeMap::new();
    for (family, file) in [
        ("ode-sdrive-expert", "ode-sdrive-source-closure.json"),
        ("dassl", "dassl-source-closure.json"),
        (
            "optimization-linear-programming-in-memory",
            "lp-in-memory-source-closure.json",
        ),
        ("fftpack-real", "fftpack-real-source-closure.json"),
        ("fftpack-complex", "fftpack-complex-source-closure.json"),
        (
            "fishpack-cartesian-2d",
            "fishpack-cartesian-2d-source-closure.json",
        ),
        ("fishpack-pois3d", "fishpack-pois3d-source-closure.json"),
        (
            "banded-linear-systems",
            "banded-linear-systems-source-closure.json",
        ),
        ("pchip", "pchip-source-closure.json"),
        ("bspline", "bspline-source-closure.json"),
        (
            "piecewise-polynomial",
            "piecewise-polynomial-source-closure.json",
        ),
        (
            "special-scalar-expanded",
            "special-scalar-expanded-source-closure.json",
        ),
    ] {
        let families = closures["families"]
            .as_object_mut()
            .ok_or_else(|| policy("family source closure lacks families"))?;
        if families.contains_key(family) {
            continue;
        }
        let overlay = read_value(repo_path(format!("crates/slatec-src/metadata/{file}")))?;
        overlay_aliases.extend(overlay_source_aliases(&overlay, &source_paths)?);
        families.insert(family.to_owned(), overlay["source_ids"].clone());
    }
    let interface = read_value(repo_path("generated/ffi/interface-inventory.json"))?;
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
        .map(|source_id| {
            overlay_aliases
                .get(source_id)
                .filter(|selected_id| native_sources.contains_key(selected_id.as_str()))
                .cloned()
                .unwrap_or_else(|| source_id.to_owned())
        })
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
                    native_storage_access(source_id, audited),
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
            native_storage_access(source_id, audited),
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

fn native_storage_access(source_id: &str, audited: &NativeSourceStatus) -> &'static str {
    if source_id == "selected-source-43ec770eeca689dc" {
        // CFFTI1's `NTRYH` lives in writable `.data` because it is DATA/SAVE,
        // but this exact source closure only reads the table.
        "read_only"
    } else if audited.mutable {
        "read_write"
    } else {
        "read_only"
    }
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

fn overlay_source_aliases(
    overlay: &Value,
    source_paths: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>> {
    let by_path = source_paths
        .iter()
        .map(|(id, path)| (path.as_str(), id.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut aliases = BTreeMap::new();
    for source in overlay["sources"]
        .as_array()
        .ok_or_else(|| policy("source closure overlay lacks sources"))?
    {
        let id = source["id"]
            .as_str()
            .ok_or_else(|| policy("source closure overlay source lacks id"))?;
        let path = source["path"]
            .as_str()
            .ok_or_else(|| policy("source closure overlay source lacks path"))?;
        if let Some(selected_id) = by_path.get(path) {
            aliases.insert(id.to_owned(), (*selected_id).to_owned());
        }
    }
    Ok(aliases)
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
    if matches!(feature, "ode-sdrive-expert" | "dassl") {
        "session_scoped_thread_local_context"
    } else if feature == "nonlinear-jacobian-check" {
        "rust_callbacks_only; no_native_callback"
    } else if feature == "optimization-linear-programming-in-memory" {
        "scoped_thread_local_sparse_column_context"
    } else if matches!(
        domain,
        "quadrature" | "roots" | "nonlinear" | "least squares"
    ) {
        "scoped_thread_local_callback_registry"
    } else {
        "none"
    }
}

fn nested_policy(callback: &str, session_callback: bool) -> &'static str {
    if session_callback {
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

pub(crate) fn lp_paging_policy(snapshot: &str) -> Value {
    json!({
        "schema_id":"slatec.runtime.lp-paging-policy",
        "schema_version":"2.0.0",
        "snapshot_id":snapshot,
        "status":"safe_in_memory_only; external_paging_deferred",
        "columns":["subject","single_precision","double_precision","source_evidence","policy"],
        "records":[
            ["mathematical_model","SPLP_minimize_c_transpose_x_subject_to_Ax_equals_w_with_typed_x_and_w_bounds","DSPLP_minimize_c_transpose_x_subject_to_Ax_equals_w_with_typed_x_and_w_bounds","src/splp.f_and_src/dsplp.f_prologues","safe_in_memory_wrapper"],
            ["matrix_callback","USRMAT_one_based_sequential_sparse_column_data","DUSRMT_one_based_sequential_sparse_column_data","src/splp.f_and_src/dsplp.f_DATTRV_description","private_contained_trampoline_over_validated_owned_CSC"],
            ["paging_activation","save_restore_or_matrix_staging_exceeds_LAMAT","save_restore_or_matrix_staging_exceeds_LAMAT","src/splp.f_src/dsplp.f_and_DPLPMN_SPLPMN","save_restore_disabled_and_LAMAT_preflighted"],
            ["resident_capacity","max(N+7,N+NNZ+6)","max(N+7,N+NNZ+6)","DPLPMN_SPLPMN_matrix_staging","PagingRequired_returned_before_FFI_if_the_public_limit_is_too_small"],
            ["unit_selection","option_KEY_54_not_emitted","option_KEY_54_not_emitted","internal_PRGOPT_builder","no_public_Fortran_unit_or_filesystem_resource"],
            ["paging_implementations","PRWVIR_PRWPGE_SOPENM_SCLOSM_excluded","DPRWVR_DPRWPG_SOPENM_SCLOSM_excluded","lp-in-memory-source-closure.json","ABI_forbidden_entry_symbols_do_no_IO_and_any_entry_discards_the_native_result"],
            ["save_restore","disabled","disabled","internal_PRGOPT_builder","not_public"],
            ["printing","KEY_51_fixed_zero","KEY_51_fixed_zero","internal_PRGOPT_builder","legacy_printing_disabled"]
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
            "linear_programming":"SPLP and DSPLP expose only the validated in-memory sparse subset; paging implementations, option key 54, save/restore, printing, and filesystem resources remain unavailable"
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
        "# Runtime concurrency audit\n\n- Classified safe functions: {} ({rendered}).\n- `SerializedGlobal` records enter the reentrant-per-thread, process-wide native runtime guard. This permits a non-callback native helper inside an active hosted call, but callback-based nested native operations remain rejected.\n- `BackendDependent` records preserve existing `no_std`/`alloc` capability and do not make a Rust thread-safety claim. They require provider provenance, source-level reentrancy evidence, and parallel stress tests before any narrower class is adopted.\n- Rust ownership safety, exact retained-native-closure reentrancy, and provider/runtime thread safety are separate fields; independent buffers do not prove native reentrancy.\n- The origin audit hash-verifies, scans, compiles, and inspects every selected base and overlay source. It records selected LP COMMON blocks and preserves full-corpus COMMON findings separately; any parser disagreement or source/object mismatch remains conservatively unresolved rather than weakening serialization.\n- `SDRIV3`/`DDRIV3` and the safe in-memory `SPLP`/`DSPLP` subset remain `SerializedGlobal`: mutable saved/COMMON storage, XERROR, callback dispatch, and provider/runtime uncertainty require the lock. Cross-family native instrumentation observes at most one active hosted native lock scope.\n",
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
        assert_eq!(ode.len(), 8);
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
    fn lp_exposes_only_the_in_memory_subset_and_keeps_paging_deferred() {
        let temp = tempfile::tempdir().unwrap();
        generate(temp.path()).unwrap();
        let lp = read_value(temp.path().join("lp-paging-policy.json")).unwrap();
        assert_eq!(
            lp["status"].as_str(),
            Some("safe_in_memory_only; external_paging_deferred")
        );
        let rendered = serde_json::to_string(&lp).unwrap();
        assert!(rendered.contains("option_KEY_54_not_emitted"));
        assert!(rendered.contains("no_public_Fortran_unit_or_filesystem_resource"));
        assert!(rendered.contains("PagingRequired_returned_before_FFI"));
    }
}
