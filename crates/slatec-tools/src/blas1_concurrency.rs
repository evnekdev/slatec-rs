//! Backend-qualified BLAS Level 1 concurrency and ownership metadata.
//!
//! The scope is read from the checked-in concurrency-relaxation candidate
//! index.  This module deliberately does not infer that an arbitrary external
//! BLAS is thread-safe from the BLAS specification alone.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const SAFE_API: &str = "generated/safe-api";
const SNAPSHOT: &str = "complete-slatec-05078ebcb649b50e4435";
const CANDIDATE_OUTCOME: &str = "candidate_backend_dependent_parallel";
const QUALIFIED_ROUTINES: &[&str] = &[
    "DASUM", "DAXPY", "DCOPY", "DDOT", "DSCAL", "DSWAP", "IDAMAX", "ISAMAX", "SASUM", "SAXPY",
    "SCOPY", "SDOT", "SSCAL", "SSWAP",
];

/// Summary of deterministic ownership and BLAS concurrency evidence.
#[derive(Debug)]
pub struct ResultSummary {
    /// Number of safe wrappers in the candidate scope.
    pub candidate_count: usize,
    /// Number of backend-specific wrapper records.
    pub backend_record_count: usize,
    /// Stable hash over every emitted byte.
    pub semantic_hash: String,
}

#[derive(Clone)]
struct Candidate {
    safe_function: String,
    routine: String,
    feature: String,
    precision: &'static str,
    variant: &'static str,
    source_file: String,
    source_hash: String,
    entry_object: String,
    object_closure: Vec<Value>,
    undefined_symbols: Vec<Value>,
}

/// Generates backend-specific BLAS Level 1 concurrency evidence.
pub fn generate(output_dir: &Path) -> Result<ResultSummary> {
    fs::create_dir_all(output_dir)?;
    let mut projections = keyed_records(
        repo_path(SAFE_API).join("per-wrapper-native-state.json"),
        "safe_function",
    )?;
    // Keep the ownership gate complete when a family supplies focused native
    // evidence before the next broad native-origin archive audit.
    for projection in crate::safe_pchip::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    for projection in crate::safe_banded::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    for projection in crate::safe_bspline::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    for projection in crate::safe_piecewise_polynomial::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    for projection in crate::safe_fftpack_complex::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    for projection in crate::safe_fishpack::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    for projection in crate::safe_pois3d::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    for projection in crate::safe_dassl::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    for projection in crate::safe_special::native_state_projections()? {
        let safe_function = string(&projection, "safe_function")?.to_owned();
        projections.entry(safe_function).or_insert(projection);
    }
    let functions = records(repo_path(SAFE_API).join("function-index.json"))?;
    let source_index = source_index()?;
    let membership = membership_index()?;
    let candidates = candidate_scope(&projections, &source_index)?;
    let ownership = ownership_records(&functions, &projections, &membership)?;
    let compiler = read_value(repo_path(SAFE_API).join("fortran-storage-model.json"))?;

    let evidence = candidates
        .iter()
        .map(candidate_evidence)
        .collect::<Vec<_>>();
    let backends = candidates
        .iter()
        .flat_map(backend_records)
        .collect::<Vec<_>>();
    let providers = provider_qualification(&compiler);
    let values = [
        (
            "function-family-ownership.json",
            json!({
                "schema_id":"slatec.safe-api.function-family-ownership",
                "schema_version":"1.0.0",
                "snapshot_id":SNAPSHOT,
                "policy":"the declared safe feature must own the native entry object; transitive objects shared by reviewed family closures are listed explicitly",
                "unexplained_mismatch_count":0,
                "columns":["safe_function","native_entry_point","declared_family","entry_object","effective_native_families","shared_dependency_objects","status"],
                "records":ownership,
            }),
        ),
        (
            "blas1-concurrency-evidence.json",
            json!({
                "schema_id":"slatec.blas1.concurrency-evidence",
                "schema_version":"1.0.0",
                "snapshot_id":SNAPSHOT,
                "candidate_source":"generated/safe-api/concurrency-relaxation-candidates.json records with outcome candidate_backend_dependent_parallel",
                "candidate_count":candidates.len(),
                "claim":"independent calls with non-overlapping mutable storage may execute concurrently",
                "columns":["safe_function","native_routine","precision","variant","source_file","source_hash","entry_object","object_closure","external_undefined_symbols","object_writable_sections","assembly_calls","native_state","COMMON","SAVE_DATA_initialized_local","XERROR","Fortran_IO","callback","ordinary_locals","buffers_read","buffers_written","aliasing","partial_overlap","increments","zero_length","shared_read_only_inputs","independent_storage_required","stress_test"],
                "records":evidence,
            }),
        ),
        (
            "backend-concurrency-index.json",
            json!({
                "schema_id":"slatec.backend.concurrency-index",
                "schema_version":"1.0.0",
                "snapshot_id":SNAPSHOT,
                "candidate_count":candidates.len(),
                "backend_profile_count":4,
                "columns":["safe_function","native_routine","backend_profile","source_hash","compiler","compiler_flags","native_state","XERROR","Fortran_IO","callback","provider_contract","stress_test","classification","independent_storage_required","runtime_behavior"],
                "records":backends,
            }),
        ),
        (
            "blas1-provider-qualification.json",
            json!({
                "schema_id":"slatec.blas1.provider-qualification",
                "schema_version":"1.0.0",
                "snapshot_id":SNAPSHOT,
                "selection_policy":"only the source-build Cargo profile identifies exact implementation bytes; system and external profiles do not expose provider identity or version",
                "oversubscription_policy":"native-call concurrency is distinct from provider-internal threading; slatec-rs does not change global provider thread counts",
                "columns":["backend_profile","selection","identity_observable","implementation","provider_contract","internal_threading","global_controls","classification","qualified_parallel_path","primary_evidence"],
                "records":providers,
            }),
        ),
    ];
    let mut semantic = Vec::new();
    for (name, value) in values {
        let bytes = serde_json::to_vec(&value)?;
        fs::write(output_dir.join(name), &bytes)?;
        semantic.extend_from_slice(&bytes);
    }
    let summary = summary_markdown(candidates.len());
    fs::write(
        output_dir.join("blas1-concurrency-audit-summary.md"),
        &summary,
    )?;
    semantic.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        candidate_count: candidates.len(),
        backend_record_count: backends.len(),
        semantic_hash: hash::bytes(&semantic),
    })
}

fn candidate_scope(
    projections: &BTreeMap<String, Value>,
    sources: &BTreeMap<String, (String, String)>,
) -> Result<Vec<Candidate>> {
    let relaxation = records(repo_path(SAFE_API).join("concurrency-relaxation-candidates.json"))?;
    let mut output = Vec::new();
    for row in relaxation
        .iter()
        .filter(|row| row[4].as_str() == Some(CANDIDATE_OUTCOME))
    {
        let safe = row[0]
            .as_str()
            .ok_or_else(|| policy("candidate safe path is not a string"))?;
        let routine = row[1][0]
            .as_str()
            .ok_or_else(|| policy("candidate native routine is absent"))?;
        if !QUALIFIED_ROUTINES.contains(&routine) {
            return Err(policy(
                "candidate scope contains an unreviewed BLAS routine",
            ));
        }
        let projection = projections
            .get(safe)
            .ok_or_else(|| policy("candidate lacks native-state projection"))?;
        for field in [
            "saved_mutable_locals",
            "common_blocks",
            "xerror_state",
            "fortran_io",
            "source_object_unresolved",
            "writable_symbols",
        ] {
            if projection[field]
                .as_array()
                .is_some_and(|records| !records.is_empty())
            {
                return Err(policy(
                    "candidate contains mutable or unresolved native evidence",
                ));
            }
        }
        let source_file = projection["source_closure"]
            .as_array()
            .and_then(|records| records.first())
            .and_then(Value::as_str)
            .ok_or_else(|| policy("candidate source closure is empty"))?;
        let (_, source_hash) = sources
            .get(source_file)
            .ok_or_else(|| policy("candidate source is absent from verified source index"))?;
        let object_closure = projection["object_closure"]
            .as_array()
            .cloned()
            .ok_or_else(|| policy("candidate object closure is absent"))?;
        if object_closure.len() != 1 {
            return Err(policy(
                "candidate object closure is not the reviewed single-object closure",
            ));
        }
        let undefined_symbols = projection["external_undefined_symbols"]
            .as_array()
            .cloned()
            .ok_or_else(|| policy("candidate undefined-symbol evidence is absent"))?;
        if !undefined_symbols.is_empty() {
            return Err(policy(
                "candidate object has unresolved imports or transitive dependencies",
            ));
        }
        output.push(Candidate {
            safe_function: safe.to_owned(),
            routine: routine.to_owned(),
            feature: row[2].as_str().unwrap_or_default().to_owned(),
            precision: if routine.starts_with('D') || routine == "IDAMAX" {
                "f64"
            } else {
                "f32"
            },
            variant: if safe.ends_with("_strided") {
                "strided"
            } else {
                "contiguous"
            },
            source_file: source_file.to_owned(),
            source_hash: source_hash.clone(),
            entry_object: string(projection, "entry_object")?.to_owned(),
            object_closure,
            undefined_symbols,
        });
    }
    output.sort_by(|left, right| left.safe_function.cmp(&right.safe_function));
    if output.len() != 28 {
        return Err(policy(
            "authoritative BLAS Level 1 candidate count is not 28",
        ));
    }
    if output
        .iter()
        .any(|candidate| candidate.feature != "blas-level1")
    {
        return Err(policy(
            "candidate scope contains a non-BLAS-Level-1 feature",
        ));
    }
    Ok(output)
}

fn candidate_evidence(candidate: &Candidate) -> Value {
    let contract = argument_contract(&candidate.routine);
    json!([
        candidate.safe_function,
        candidate.routine,
        candidate.precision,
        candidate.variant,
        candidate.source_file,
        candidate.source_hash,
        candidate.entry_object,
        candidate.object_closure,
        candidate.undefined_symbols,
        "zero_sized_.data_and_.bss_sections_only; no_nonzero_writable_symbol",
        "no_call_instructions_in_objdump_disassembly",
        "none_found_in_exact_source_and_object_closure",
        false,
        false,
        false,
        false,
        false,
        "automatic_under_-std=legacy_without_-fno-automatic_or_initialization",
        contract.0,
        contract.1,
        contract.2,
        "safe_slice_borrows_prohibit_partially_overlapping_mutable_arguments; unsafe_external_aliases_are_outside_the_safe_API_contract",
        "positive_and_negative_nonzero_increments_supported; zero_increment_rejected_before_FFI",
        "returns_before_FFI_with_identity_result_or_no_op",
        contract.3,
        true,
        "passed_high_contention_source_backend_test_with_actual_simultaneous_native_entry"
    ])
}

fn argument_contract(routine: &str) -> (&'static str, &'static str, &'static str, bool) {
    match routine.trim_start_matches('D').trim_start_matches('S') {
        "ASUM" | "DOT" => (
            if routine.ends_with("DOT") { "x,y" } else { "x" },
            "none",
            "read_only_inputs_may_alias_each_other_and_may_be_shared_between_calls",
            true,
        ),
        "COPY" => (
            "x",
            "y",
            "x_to_y_aliasing_and_partial_overlap_are_not_expressible_by_the_safe_borrowed_API",
            true,
        ),
        "AXPY" => (
            "x,y",
            "y",
            "x_and_y_overlap_is_not_expressible_by_the_safe_borrowed_API",
            true,
        ),
        "SCAL" => ("x", "x", "one_unique_mutable_slice", false),
        "SWAP" => (
            "x,y",
            "x,y",
            "distinct_unique_mutable_slices_required",
            false,
        ),
        "ISAMAX" | "IDAMAX" => (
            "x",
            "none",
            "read_only_input_may_be_shared_between_calls",
            true,
        ),
        _ => ("unresolved", "unresolved", "unresolved", false),
    }
}

fn backend_records(candidate: &Candidate) -> Vec<Value> {
    [
        (
            "source-build-gnu-mingw-reviewed",
            "GNU Fortran 14.2.0 x86_64-w64-mingw32",
            json!(["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"]),
            "hash_verified_Netlib_SLATEC_source_object_with_no_external_imports",
            "passed_high_contention_actual_simultaneous_native_entry",
            "ParallelSafe",
            "unchanged_direct_dispatch_no_runtime_lock",
        ),
        (
            "system-static-archive-unidentified",
            "unknown",
            json!([]),
            "user_selected_archive_identity_and_version_not_observable",
            "not_portable_across_arbitrary_user_archive",
            "BackendDependent",
            "unchanged_direct_dispatch_no_qualified_parallel_promise",
        ),
        (
            "external-user-linked-unidentified",
            "unknown",
            json!([]),
            "crate_emits_no_link_directives_and_cannot_observe_provider_identity_or_version",
            "not_run_against_arbitrary_user_provider",
            "BackendDependent",
            "unchanged_direct_dispatch_no_qualified_parallel_promise",
        ),
        (
            "unknown-provider",
            "unknown",
            json!([]),
            "no_provider_contract",
            "not_applicable",
            "BackendDependent",
            "conservative_no_qualified_parallel_path",
        ),
    ]
    .into_iter()
    .map(
        |(backend, compiler, flags, contract, stress, class, behavior)| {
            json!([
                candidate.safe_function,
                candidate.routine,
                backend,
                if backend.starts_with("source-build") {
                    candidate.source_hash.as_str()
                } else {
                    "unknown"
                },
                compiler,
                flags,
                if backend.starts_with("source-build") {
                    "none_found"
                } else {
                    "provider_unknown"
                },
                false,
                false,
                false,
                contract,
                stress,
                class,
                true,
                behavior,
            ])
        },
    )
    .collect()
}

fn provider_qualification(compiler: &Value) -> Vec<Value> {
    let reviewed = format!(
        "{} {}",
        compiler["compiler"].as_str().unwrap_or("GNU Fortran"),
        compiler["version"].as_str().unwrap_or("unknown")
    );
    vec![
        json!([
            "source-build-gnu-mingw-reviewed",
            "Cargo source-build feature plus hash-verified cache",
            true,
            reviewed,
            "exact selected sources and production flags audited",
            "none; scalar loops are single-threaded",
            "none",
            "ParallelSafe",
            true,
            "generated/safe-api/native-source-scan-index.json; generated/safe-api/native-writable-symbol-index.json; crates/slatec/tests/blas_level1_concurrency.rs"
        ]),
        json!([
            "system-static-archive-unidentified",
            "Cargo system feature and user paths",
            false,
            "user static archive",
            "no identity/version handshake",
            "unknown",
            "unknown",
            "BackendDependent",
            false,
            "crates/slatec-src/build.rs: system provider accepts user paths and archive name"
        ]),
        json!([
            "external-user-linked-unidentified",
            "Cargo external-backend feature",
            false,
            "linkage supplied by final application",
            "no identity/version handshake",
            "unknown",
            "unknown",
            "BackendDependent",
            false,
            "crates/slatec-src/build.rs: external backend emits no link directives"
        ]),
        json!([
            "OpenBLAS-unqualified",
            "not an explicit slatec-rs backend profile",
            false,
            "possible external provider",
            "provider documentation must be tied to an exact build and version",
            "may use internal worker threads depending on build/routine",
            "global and environment thread controls exist; slatec-rs does not change them",
            "BackendDependent",
            false,
            "https://github.com/OpenMathLib/OpenBLAS#setting-the-number-of-threads-using-environment-variables"
        ]),
        json!([
            "Intel-MKL-unqualified",
            "not an explicit slatec-rs backend profile",
            false,
            "possible external provider",
            "provider documentation must be tied to an exact version",
            "may use internal worker threads",
            "global/local/environment thread controls exist; slatec-rs does not change them",
            "BackendDependent",
            false,
            "https://www.intel.com/content/www/us/en/docs/onemkl/developer-reference-c/2024-2/threading-control.html"
        ]),
        json!([
            "BLIS-unqualified",
            "not an explicit slatec-rs backend profile",
            false,
            "possible external provider",
            "provider documentation must be tied to an exact build and version",
            "implementation-dependent",
            "runtime/environment controls may exist; slatec-rs does not change them",
            "BackendDependent",
            false,
            "https://github.com/flame/blis/blob/master/docs/Multithreading.md"
        ]),
        json!([
            "Accelerate-unqualified",
            "not supported by the reviewed GNU MinGW source profile",
            false,
            "possible external provider on Apple targets",
            "provider contract is not selected or verified by Cargo features",
            "provider-managed",
            "provider-managed; slatec-rs does not change it",
            "BackendDependent",
            false,
            "https://developer.apple.com/documentation/accelerate/blas_threading"
        ]),
        json!([
            "Netlib-external-unqualified",
            "not the hash-verified slatec-src profile",
            false,
            "possible separately built Netlib BLAS",
            "compiler flags and runtime linkage unknown",
            "normally single-threaded but not proven for user binary",
            "unknown",
            "BackendDependent",
            false,
            "https://www.netlib.org/blas/"
        ]),
        json!([
            "unknown-provider",
            "fallback",
            false,
            "unknown",
            "none",
            "unknown",
            "unknown",
            "BackendDependent",
            false,
            "none"
        ]),
    ]
}

fn ownership_records(
    functions: &[Value],
    projections: &BTreeMap<String, Value>,
    membership: &BTreeMap<String, Value>,
) -> Result<Vec<Value>> {
    let mut output = Vec::with_capacity(functions.len());
    for function in functions {
        let safe = string(function, "rust_path")?;
        let projection = projections
            .get(safe)
            .ok_or_else(|| policy("safe function lacks native ownership projection"))?;
        if projection["feature_closure_mismatch"] != false {
            return Err(policy("unexplained function-family ownership mismatch"));
        }
        let feature = string(function, "feature")?;
        let entry_object = string(projection, "entry_object")?;
        let shared = projection["object_closure"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|object| {
                object
                    .as_str()
                    .filter(|object| *object != entry_object)
                    .and_then(|object| membership.get(object))
                    .is_some_and(|record| record[4].as_bool() == Some(true))
            })
            .cloned()
            .collect::<Vec<_>>();
        output.push(json!([
            safe,
            string(function, "fortran_routine")?,
            feature,
            entry_object,
            projection["effective_native_families"],
            shared,
            "declared_family_owns_entry; shared_transitive_dependencies_explicit"
        ]));
    }
    output.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    Ok(output)
}

fn source_index() -> Result<BTreeMap<String, (String, String)>> {
    let value = read_value(repo_path(SAFE_API).join("native-source-scan-index.json"))?;
    value["records"]
        .as_array()
        .ok_or_else(|| policy("source index lacks records"))?
        .iter()
        .map(|row| {
            Ok((
                row[1]
                    .as_str()
                    .ok_or_else(|| policy("source path"))?
                    .to_owned(),
                (
                    row[0]
                        .as_str()
                        .ok_or_else(|| policy("source id"))?
                        .to_owned(),
                    row[4]
                        .as_str()
                        .ok_or_else(|| policy("source hash"))?
                        .to_owned(),
                ),
            ))
        })
        .collect::<Result<_>>()
}

fn membership_index() -> Result<BTreeMap<String, Value>> {
    let value = read_value(repo_path(SAFE_API).join("native-source-family-membership.json"))?;
    value["records"]
        .as_array()
        .ok_or_else(|| policy("membership index lacks records"))?
        .iter()
        .map(|row| {
            Ok((
                row[2]
                    .as_str()
                    .ok_or_else(|| policy("membership object"))?
                    .to_owned(),
                row.clone(),
            ))
        })
        .collect::<Result<_>>()
}

fn summary_markdown(candidate_count: usize) -> String {
    format!(
        "# BLAS Level 1 backend concurrency audit\n\n- Candidate safe wrappers: {candidate_count}; native routines: 14.\n- The exact hash-verified `source-build` GNU MinGW objects contain no reachable mutable static state, COMMON, XERROR, Fortran I/O, callbacks, or external runtime imports. Independent calls with non-overlapping mutable storage are classified `ParallelSafe` for that backend only.\n- `system`, `external-backend`, named vendor libraries without an explicit project profile, and unknown providers remain `BackendDependent`. Provider identity and version are not observable through those Cargo profiles.\n- Safe slices prevent overlapping mutable arguments within one call. Cross-call safety still requires non-overlapping mutable storage; read-only buffers may be shared. Positive and negative nonzero increments are supported, zero increments are rejected, and zero-length calls return before FFI.\n- The selected Netlib/SLATEC loops are single-threaded. External providers may use worker threads or global controls, so native-call concurrency may oversubscribe a process and is not necessarily faster. `slatec-rs` does not change provider thread counts.\n- Runtime behavior is unchanged: BLAS Level 1 remains a direct `core`-capable FFI surface, while callback, XERROR, ODE, and solver families retain the process-global exclusive runtime lock.\n- Storage layout and LP paging policies are unchanged.\n"
    )
}

fn keyed_records(path: PathBuf, key: &str) -> Result<BTreeMap<String, Value>> {
    records(path)?
        .into_iter()
        .map(|record| {
            let value = string(&record, key)?.to_owned();
            Ok((value, record))
        })
        .collect::<Result<_>>()
}

fn records(path: PathBuf) -> Result<Vec<Value>> {
    let value = read_value(path)?;
    value["records"]
        .as_array()
        .cloned()
        .ok_or_else(|| policy("metadata file lacks records"))
}

fn read_value(path: PathBuf) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field]
        .as_str()
        .ok_or_else(|| policy("expected metadata string"))
}

fn repo_path(relative: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Verification(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic_and_scope_is_exact() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let one = generate(first.path()).unwrap();
        let two = generate(second.path()).unwrap();
        assert_eq!(one.candidate_count, 28);
        assert_eq!(one.backend_record_count, 112);
        assert_eq!(one.semantic_hash, two.semantic_hash);
        for name in [
            "function-family-ownership.json",
            "blas1-concurrency-evidence.json",
            "backend-concurrency-index.json",
            "blas1-provider-qualification.json",
            "blas1-concurrency-audit-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }

    #[test]
    fn reviewed_source_is_parallel_safe_but_unknown_backends_are_not() {
        let output = tempfile::tempdir().unwrap();
        generate(output.path()).unwrap();
        let records = records(output.path().join("backend-concurrency-index.json")).unwrap();
        assert_eq!(
            records
                .iter()
                .filter(
                    |row| row[2] == "source-build-gnu-mingw-reviewed" && row[12] == "ParallelSafe"
                )
                .count(),
            28
        );
        assert!(
            records
                .iter()
                .filter(|row| row[2] != "source-build-gnu-mingw-reviewed")
                .all(|row| row[12] == "BackendDependent")
        );
    }

    #[test]
    fn excluded_saved_state_routines_never_reenter_scope() {
        let output = tempfile::tempdir().unwrap();
        generate(output.path()).unwrap();
        let evidence = records(output.path().join("blas1-concurrency-evidence.json")).unwrap();
        assert!(
            evidence
                .iter()
                .all(|row| !matches!(row[1].as_str(), Some("SNRM2" | "DNRM2" | "SROT" | "DROT")))
        );
    }
}
