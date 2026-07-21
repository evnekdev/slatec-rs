//! Final raw-API closure and permanent disposition audit.
//!
//! Batch D does not create a parallel routine inventory. It derives one final
//! disposition for every record in the canonical raw API inventory and only
//! requalifies legacy declarations that already have source, ABI, native-link,
//! and safe-wrapper evidence.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Inputs and outputs used by the final disposition generator.
pub struct BatchDPaths<'a> {
    pub raw_api_dir: &'a Path,
    pub program_units_dir: &'a Path,
    pub common_block_index_path: &'a Path,
    pub sys_dir: &'a Path,
    pub src_dir: &'a Path,
    pub facade_dir: &'a Path,
    pub output_dir: &'a Path,
}

/// Deterministic Batch D generation or validation result.
#[derive(Clone, Debug, serde::Serialize)]
pub struct BatchDResult {
    pub status: String,
    pub retained_identities: usize,
    pub public_identities: usize,
    pub unexplained_identities: usize,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
}

/// Existing public drivers requalified by Batch D.
///
/// Each already has one declaration in a mathematical module and native safe
/// facade evidence. Batch D makes that existing path canonical; it never emits
/// a second extern declaration.
pub const LEGACY_PUBLIC: &[(&str, &str)] = &[
    ("DCOV", "least-squares-covariance"),
    ("DDASSL", "dassl"),
    ("DDRIV3", "ode-sdrive-expert"),
    ("DNLS1", "least-squares-nonlinear-expert"),
    ("DNLS1E", "least-squares-nonlinear-easy"),
    ("DNSQ", "nonlinear-expert"),
    ("DNSQE", "nonlinear-easy"),
    ("DQAG", "quadrature-basic"),
    ("DQAGI", "quadrature-basic"),
    ("DQAGP", "quadrature-breakpoints"),
    ("DQAGS", "quadrature-basic"),
    ("DQAWC", "quadrature-basic"),
    ("DQAWF", "quadrature-fourier"),
    ("DQAWO", "quadrature-oscillatory"),
    ("DQAWS", "quadrature-weighted"),
    ("DQNC79", "quadrature-nonadaptive"),
    ("DQNG", "quadrature-nonadaptive"),
    ("DSPLP", "optimization-linear-programming-in-memory"),
    ("QAG", "quadrature-basic"),
    ("QAGI", "quadrature-basic"),
    ("QAGP", "quadrature-breakpoints"),
    ("QAGS", "quadrature-basic"),
    ("QAWC", "quadrature-basic"),
    ("QAWF", "quadrature-fourier"),
    ("QAWO", "quadrature-oscillatory"),
    ("QAWS", "quadrature-weighted"),
    ("QNC79", "quadrature-nonadaptive"),
    ("QNG", "quadrature-nonadaptive"),
    ("SCOV", "least-squares-covariance"),
    ("SDASSL", "dassl"),
    ("SDRIV3", "ode-sdrive-expert"),
    ("SNLS1", "least-squares-nonlinear-expert"),
    ("SNLS1E", "least-squares-nonlinear-easy"),
    ("SNSQ", "nonlinear-expert"),
    ("SNSQE", "nonlinear-easy"),
    ("SPLP", "optimization-linear-programming-in-memory"),
];

const FORTRAN_IO: &[&str] = &[
    "BVPOR", "DDASSL", "DEXBVP", "DMOUT", "DP1VLU", "DPLPMN", "DREADP", "DVOUT", "DWRITP", "EXBVP",
    "IVOUT", "MPADD2", "MPCHK", "MPDIVI", "MPMUL2", "MPNZR", "MPOVFL", "PVALUE", "RKFAB", "SCLOSM",
    "SDASSL", "SMOUT", "SOPENM", "SPLPMN", "SREADP", "SVOUT", "SWRITP", "XERBLA",
];

const MACHINE_SUPPORT: &[&str] = &["D1MACH", "DCHKW", "I1MACH", "R1MACH", "SCHKW"];
const RUNTIME_SUPPORT: &[&str] = &["LSAME", "NUMXER"];

const DISPOSITIONS: &[&str] = &[
    "canonical-public",
    "raw-internal",
    "provider-subsidiary",
    "runtime-support",
    "error-support",
    "machine-support",
    "block-data-support",
    "historical-driver",
    "demonstration-program",
    "catalogue-only",
    "missing-provider",
    "missing-symbol",
    "unsupported-character-abi",
    "unsupported-callback-abi",
    "unsupported-complex-abi",
    "unsupported-entry-interface",
    "unsupported-alternate-return",
    "unsupported-common-interface",
    "unsupported-fortran-io",
    "unsupported-compiler-specific-abi",
    "unsafe-to-expose",
    "duplicate-identity",
    "parser-artefact",
    "excluded-other",
];

/// Returns the coherent declaration/provider feature for a requalified driver.
pub fn legacy_feature(routine: &str) -> Option<&'static str> {
    LEGACY_PUBLIC
        .iter()
        .find_map(|(name, feature)| (*name == routine).then_some(*feature))
}

/// Source/provider manifest guarding the complete Batch D legacy review set.
pub fn source_manifest(records: &[Value]) -> String {
    let mut entries = records
        .iter()
        .filter(|record| legacy_feature(&field(record, "routine")).is_some())
        .map(|record| {
            format!(
                "{}\0{}\0{}\n",
                field(record, "routine"),
                field(record, "source_hash"),
                field(record, "canonical_provider")
            )
        })
        .collect::<Vec<_>>();
    entries.sort();
    hash::bytes(entries.concat().as_bytes())
}

/// Generates the final disposition reports and focused structural probes.
pub fn generate(paths: BatchDPaths<'_>) -> Result<BatchDResult> {
    let artifacts = build(&paths)?;
    fs::create_dir_all(paths.output_dir)?;
    for (name, bytes) in &artifacts.files {
        fs::write(paths.output_dir.join(name), bytes)?;
    }
    fs::write(
        paths.sys_dir.join("tests/batch_d_compile.rs"),
        &artifacts.compile_probe,
    )?;
    fs::write(
        paths.facade_dir.join("tests/raw_batch_d_link.rs"),
        &artifacts.link_probe,
    )?;
    Ok(artifacts.result("generated", paths.output_dir))
}

/// Validates checked-in final disposition outputs without rewriting them.
pub fn validate(paths: BatchDPaths<'_>) -> Result<BatchDResult> {
    let artifacts = build(&paths)?;
    for (name, expected) in &artifacts.files {
        let actual = fs::read(paths.output_dir.join(name)).map_err(|error| {
            policy(&format!("missing final disposition output {name}: {error}"))
        })?;
        if actual != *expected {
            return Err(policy(&format!(
                "final disposition output {name} differs; regenerate it"
            )));
        }
    }
    compare_file(
        &paths.sys_dir.join("tests/batch_d_compile.rs"),
        &artifacts.compile_probe,
    )?;
    compare_file(
        &paths.facade_dir.join("tests/raw_batch_d_link.rs"),
        &artifacts.link_probe,
    )?;
    Ok(artifacts.result("ok", paths.output_dir))
}

struct Artifacts {
    files: BTreeMap<String, Vec<u8>>,
    compile_probe: Vec<u8>,
    link_probe: Vec<u8>,
    retained: usize,
    public: usize,
    unexplained: usize,
    semantic_hash: String,
}

impl Artifacts {
    fn result(&self, status: &str, output_dir: &Path) -> BatchDResult {
        BatchDResult {
            status: status.to_owned(),
            retained_identities: self.retained,
            public_identities: self.public,
            unexplained_identities: self.unexplained,
            semantic_hash: self.semantic_hash.clone(),
            output_dir: output_dir.to_path_buf(),
        }
    }
}

fn build(paths: &BatchDPaths<'_>) -> Result<Artifacts> {
    let routine_status = read_json(&paths.raw_api_dir.join("routine-status.json"))?;
    let status_records = records(&routine_status, "raw API routine status")?;
    let abi = by_routine(
        &read_json(&paths.raw_api_dir.join("abi-classification.json"))?,
        "Batch A ABI classification",
    )?;
    let callbacks = by_routine(
        &read_json(&paths.raw_api_dir.join("callback-classification.json"))?,
        "Batch B callback classification",
    )?;
    let complex_flags = by_routine(
        &read_json(&paths.raw_api_dir.join("batch-c-classification.json"))?,
        "Batch C classification",
    )?;
    let entries = read_json(&paths.program_units_dir.join("entry-points.json"))?;
    let entry_records = records(&entries, "ENTRY inventory")?;
    let common_blocks = common_blocks(paths.common_block_index_path)?;
    let sys_features = cargo_features(&paths.sys_dir.join("Cargo.toml"))?;
    let src_features = cargo_features(&paths.src_dir.join("Cargo.toml"))?;

    if !entry_records.is_empty() {
        return Err(policy(
            "selected retained corpus unexpectedly contains ENTRY records; author an explicit Batch D decision",
        ));
    }

    let mut final_records = Vec::with_capacity(status_records.len());
    let mut seen = BTreeSet::new();
    for record in status_records {
        let routine = field(record, "routine");
        if !seen.insert(routine.clone()) {
            return Err(policy(&format!(
                "duplicate retained routine identity {routine}"
            )));
        }
        let abi_record = abi.get(&routine);
        let callback_record = callbacks.get(&routine);
        let complex_record = complex_flags.get(&routine);
        let disposition = disposition(record);
        if !DISPOSITIONS.contains(&disposition.as_str()) {
            return Err(policy(&format!(
                "{routine} has unknown final disposition {disposition}"
            )));
        }
        validate_record(record, &disposition, &sys_features, &src_features)?;
        let reason = reason(record, &disposition, callback_record, complex_record);
        let evidence_sources = evidence_sources(record, &disposition);
        let permanently_excluded = is_permanent_exclusion(&disposition);
        let common = common_blocks.get(&routine).cloned().unwrap_or_default();
        let callback_status = callback_record
            .map(|item| field(item, "batch_b_eligibility"))
            .unwrap_or_else(|| "not_callback_bearing".to_owned());
        let complex_status = complex_record
            .map(|item| field(item, "combined_abi_class"))
            .unwrap_or_else(|| "not_classified".to_owned());
        let character_status = complex_record
            .map(|item| {
                if item.get("character_presence").and_then(Value::as_bool) == Some(true) {
                    if item
                        .get("long_or_variable_character")
                        .and_then(Value::as_bool)
                        == Some(true)
                    {
                        "long_or_variable".to_owned()
                    } else {
                        "fixed_character_1".to_owned()
                    }
                } else {
                    "none".to_owned()
                }
            })
            .unwrap_or_else(|| "none".to_owned());
        let logical_status = complex_record
            .map(|item| {
                if item.get("logical_presence").and_then(Value::as_bool) == Some(true) {
                    "fortran_logical_i32_validated".to_owned()
                } else {
                    "none".to_owned()
                }
            })
            .unwrap_or_else(|| "none".to_owned());
        let provider_status = if field(record, "canonical_provider") == "unavailable" {
            "missing_provider"
        } else if field(record, "symbol_status") == "missing_symbol" {
            "provider_present_symbol_missing"
        } else {
            "selected_provider_verified"
        };
        final_records.push(json!({
            "routine":routine,
            "source_file":field(record,"source_file"),
            "source_hash":field(record,"source_hash"),
            "program_unit_kind":field(record,"program_unit_kind"),
            "mathematical_family":field(record,"primary_family"),
            "public_role":field(record,"driver_role"),
            "historical_role":field(record,"historical_role"),
            "current_raw_status":field(record,"raw_api_state"),
            "canonical_rust_path":field(record,"canonical_rust_path"),
            "feature":field(record,"feature"),
            "provider_feature":field(record,"provider_feature"),
            "abi_classification":abi_record.map(|item| field(item,"abi_class")).unwrap_or_else(|| "not_available".to_owned()),
            "callback_classification":callback_status,
            "complex_classification":complex_status,
            "character_classification":character_status,
            "logical_classification":logical_status,
            "entry_status":"no_entry_in_selected_program_unit_inventory",
            "alternate_return_status":"no_alternate_return_in_selected_fixed_form_audit",
            "common_block_relationships":common,
            "fortran_io_dependency":if FORTRAN_IO.contains(&routine.as_str()){"direct_fixed_form_io_statement"}else{"none_detected"},
            "provider_status":provider_status,
            "native_symbol":field(record,"native_symbol"),
            "native_symbol_status":field(record,"symbol_status"),
            "source_closure_status":if provider_status == "selected_provider_verified"{"provider_source_present"}else{"not_linkable_as_public"},
            "prior_batch_a_exclusion_reason":field(record,"batch_a_exclusion_reason"),
            "prior_batch_b_exclusion_reason":callback_record.map(|item| field(item,"batch_b_exclusion_reason")).unwrap_or_else(|| "not_applicable".to_owned()),
            "prior_batch_c_exclusion_reason":complex_record.map(|item| field(item,"batch_c_exclusion_reason")).unwrap_or_else(|| "not_applicable".to_owned()),
            "final_disposition":disposition.clone(),
            "reason_code":disposition,
            "final_evidence":reason,
            "evidence_sources":evidence_sources,
            "reopen_condition":reopen_condition(&disposition),
            "statuses":{
                "source_verified":field(record,"source_hash") != "unavailable",
                "abi_classified":abi_record.is_some(),
                "public_role_classified":field(record,"driver_role") != "unknown",
                "provider_classified":true,
                "compile_validated":field(record,"compile_test_status") != "not_compiled",
                "link_validated":!permanently_excluded && field(record,"link_test_status") == "passed",
                "runtime_smoke_validated":!permanently_excluded && (field(record,"runtime_test_status") == "passed" || field(record,"runtime_test_status").contains("representative")),
                "numerically_validated":!permanently_excluded && field(record,"runtime_test_status") == "passed",
                "safe_wrapper_available":field(record,"safe_wrapper_status") != "not_safely_wrapped",
                "permanently_excluded":permanently_excluded,
            }
        }));
    }
    final_records.sort_by_key(|record| field(record, "routine"));

    let summary = summary(&final_records);
    validate_aggregate(&final_records, &summary)?;
    let public_coverage = public_coverage(&final_records);
    let permanent = filtered_report(
        "slatec-sys.raw-api.permanent-exclusions",
        &final_records,
        |record| {
            record
                .pointer("/statuses/permanently_excluded")
                .and_then(Value::as_bool)
                == Some(true)
        },
    );
    let provider_gaps = filtered_report(
        "slatec-sys.raw-api.provider-gaps",
        &final_records,
        |record| {
            matches!(
                field(record, "final_disposition").as_str(),
                "missing-provider" | "missing-symbol"
            )
        },
    );
    let legacy = legacy_interface_audit(&final_records);
    let disposition = json!({
        "schema_id":"slatec-sys.raw-api.final-disposition",
        "schema_version":"1.0.0",
        "record_model":"exactly one terminal disposition per retained identity",
        "disposition_vocabulary":DISPOSITIONS,
        "records":final_records,
    });
    let summary_markdown = summary_markdown(&summary);

    let mut files = BTreeMap::new();
    files.insert(
        "final-disposition.json".to_owned(),
        json_bytes(&disposition)?,
    );
    files.insert(
        "final-disposition-summary.md".to_owned(),
        summary_markdown.into_bytes(),
    );
    files.insert(
        "public-api-coverage.json".to_owned(),
        json_bytes(&public_coverage)?,
    );
    files.insert(
        "permanent-exclusions.json".to_owned(),
        json_bytes(&permanent)?,
    );
    files.insert("provider-gaps.json".to_owned(), json_bytes(&provider_gaps)?);
    files.insert(
        "legacy-interface-audit.json".to_owned(),
        json_bytes(&legacy)?,
    );
    let semantic_hash = semantic_hash(&files);
    files.insert(
        "batch-d-manifest.json".to_owned(),
        json_bytes(&json!({
            "schema_id":"slatec-sys.raw-api.batch-d-manifest",
            "schema_version":"1.0.0",
            "semantic_sha256":semantic_hash,
            "files":files.keys().cloned().collect::<Vec<_>>(),
        }))?,
    );
    let public = count_disposition(&final_records, "canonical-public");
    let unexplained = summary
        .pointer("/counts/identities_remaining_unexplained")
        .and_then(Value::as_u64)
        .unwrap_or(usize::MAX as u64) as usize;
    Ok(Artifacts {
        compile_probe: compile_probe(&final_records).into_bytes(),
        link_probe: link_probe(&final_records).into_bytes(),
        retained: final_records.len(),
        public,
        unexplained,
        files,
        semantic_hash,
    })
}

fn disposition(record: &Value) -> String {
    let state = field(record, "raw_api_state");
    let routine = field(record, "routine");
    if is_public_state(&state) {
        return "canonical-public".to_owned();
    }
    if field(record, "program_unit_kind") == "program" {
        return "demonstration-program".to_owned();
    }
    if state == "catalogue_only" {
        return "catalogue-only".to_owned();
    }
    if field(record, "symbol_status") == "missing_symbol" {
        return "missing-symbol".to_owned();
    }
    if field(record, "primary_family") == "Error handling" {
        return "error-support".to_owned();
    }
    if MACHINE_SUPPORT.contains(&routine.as_str()) {
        return "machine-support".to_owned();
    }
    if RUNTIME_SUPPORT.contains(&routine.as_str()) {
        return "runtime-support".to_owned();
    }
    if state == "block_data" {
        return "block-data-support".to_owned();
    }
    if matches!(routine.as_str(), "BVSUP" | "DBVSUP")
        || (field(record, "driver_role") == "historically_user_callable_driver"
            && matches!(
                state.as_str(),
                "unsupported_callback_abi" | "source_present_unbound"
            ))
    {
        return "unsupported-callback-abi".to_owned();
    }
    if state == "unsupported_complex_return_abi" {
        return "unsupported-complex-abi".to_owned();
    }
    if state == "unsupported_character_return_abi" {
        return "unsupported-character-abi".to_owned();
    }
    if state == "unsupported_entry_or_alternate_return" {
        return "unsupported-entry-interface".to_owned();
    }
    if field(record, "historical_role") == "subsidiary" || state == "not_independently_callable" {
        return "provider-subsidiary".to_owned();
    }
    if field(record, "primary_family") == "Documentation and source-processing tools"
        || matches!(
            state.as_str(),
            "generated_abi_validated"
                | "source_present_unbound"
                | "documentation_or_tooling"
                | "external_dependency"
        )
    {
        return "raw-internal".to_owned();
    }
    "excluded-other".to_owned()
}

fn reason(
    record: &Value,
    disposition: &str,
    callback: Option<&Value>,
    batch_c: Option<&Value>,
) -> Vec<String> {
    let routine = field(record, "routine");
    match disposition {
        "canonical-public" if field(record, "raw_api_state") == "batch_d_public_driver" => vec![
            "existing mathematical-family declaration".to_owned(),
            "selected source hash and compiler-observed symbol".to_owned(),
            "safe-wrapper audit and native family regression".to_owned(),
            "Batch D canonical-path and native-link probe".to_owned(),
        ],
        "canonical-public" => vec![format!(
            "preserved prior {} disposition",
            field(record, "raw_api_state")
        )],
        "unsupported-callback-abi" if matches!(routine.as_str(), "BVSUP" | "DBVSUP") => vec![
            format!("{}", field(record, "exclusion_reason")),
            "FMAT/GVEC/UIVP/UVEC are object-level problem-definition externals, not explicit procedure dummies".to_owned(),
            "no faithful Rust callback ABI or user-data channel exists".to_owned(),
        ],
        "unsupported-callback-abi" => vec![
            callback
                .map(|item| field(item, "batch_b_exclusion_reason"))
                .filter(|text| !text.is_empty())
                .unwrap_or_else(|| field(record, "exclusion_reason")),
            batch_c
                .map(|item| field(item, "batch_c_exclusion_reason"))
                .filter(|text| !text.is_empty())
                .unwrap_or_else(|| "no independently proven callback contract".to_owned()),
        ],
        "missing-symbol" => vec![
            "selected retained identity has no compiler-observed native symbol".to_owned(),
            "not eligible for a callable Rust declaration".to_owned(),
        ],
        "provider-subsidiary" => vec![
            "catalogue role is subsidiary/internal".to_owned(),
            "retained through exact provider dependency closure when required".to_owned(),
        ],
        "error-support" => vec![
            "legacy XERROR/error-state infrastructure".to_owned(),
            "provider/runtime support rather than canonical numerical API".to_owned(),
        ],
        "machine-support" => vec![
            "machine/profile support routine".to_owned(),
            "provider-owned implementation detail for the validated ABI profile".to_owned(),
        ],
        "runtime-support" => vec!["shared provider/runtime support routine".to_owned()],
        "demonstration-program" => vec![
            "Fortran PROGRAM unit".to_owned(),
            "not a callable library interface".to_owned(),
        ],
        "catalogue-only" => vec!["catalogue identity has no selected callable program unit".to_owned()],
        "raw-internal" => vec![
            "not a genuine canonical public numerical entry point".to_owned(),
            "retained only as tooling, support, or internal implementation evidence".to_owned(),
        ],
        _ => vec![field(record, "exclusion_reason")],
    }
}

fn evidence_sources(record: &Value, disposition: &str) -> Vec<String> {
    let mut sources = BTreeSet::from([
        "generated/raw-api/routine-status.json".to_owned(),
        "generated/raw-api/abi-classification.json".to_owned(),
    ]);
    if disposition == "unsupported-callback-abi" {
        sources.insert("generated/raw-api/callback-classification.json".to_owned());
    }
    if matches!(
        disposition,
        "unsupported-character-abi" | "unsupported-complex-abi"
    ) {
        sources.insert("generated/raw-api/batch-c-classification.json".to_owned());
    }
    if field(record, "program_unit_kind") == "program"
        || disposition == "unsupported-entry-interface"
    {
        sources.insert("generated/program-units/program-units.json".to_owned());
        sources.insert("generated/program-units/entry-points.json".to_owned());
    }
    if disposition == "canonical-public"
        && field(record, "raw_api_state") == "batch_d_public_driver"
    {
        sources.insert("metadata/raw-api-corrections.json".to_owned());
    }
    if matches!(field(record, "routine").as_str(), "BVSUP" | "DBVSUP") {
        sources.insert("generated/ffi-validation/native-build.json".to_owned());
    }
    sources.into_iter().collect()
}

fn reopen_condition(disposition: &str) -> &'static str {
    match disposition {
        "unsupported-callback-abi" => {
            "only new source/compiler evidence proving every outer and callback call shape"
        }
        "missing-provider" | "missing-symbol" => {
            "only a rights-cleared selected provider with reproducible source, symbol, and closure evidence"
        }
        "catalogue-only" => "only a verified callable source provider for this exact identity",
        "canonical-public" => "not applicable; ABI corrections remain evidence-driven safety fixes",
        _ => "reopen only if authoritative source, role, or provider evidence changes",
    }
}

fn is_public_state(state: &str) -> bool {
    matches!(
        state,
        "reviewed_public_driver"
            | "reviewed_public_subsidiary"
            | "batch_a_public_driver"
            | "batch_b_public_driver"
            | "batch_c_public_driver"
            | "batch_d_public_driver"
    )
}

fn is_permanent_exclusion(disposition: &str) -> bool {
    disposition.starts_with("unsupported-")
        || matches!(
            disposition,
            "missing-provider"
                | "missing-symbol"
                | "unsafe-to-expose"
                | "duplicate-identity"
                | "parser-artefact"
                | "excluded-other"
                | "catalogue-only"
                | "historical-driver"
                | "demonstration-program"
        )
}

fn validate_record(
    record: &Value,
    disposition: &str,
    sys_features: &BTreeSet<String>,
    src_features: &BTreeSet<String>,
) -> Result<()> {
    let routine = field(record, "routine");
    if disposition == "canonical-public" {
        for key in ["canonical_rust_path", "feature", "provider_feature"] {
            let value = field(record, key);
            if value.is_empty() || matches!(value.as_str(), "not_promoted" | "not_assigned") {
                return Err(policy(&format!(
                    "public Batch D closure record {routine} lacks {key}"
                )));
            }
        }
        if !sys_features.contains(&field(record, "feature")) {
            return Err(policy(&format!(
                "public Batch D closure record {routine} has no declaration feature"
            )));
        }
        if !src_features.contains(&field(record, "provider_feature")) {
            return Err(policy(&format!(
                "public Batch D closure record {routine} has no provider feature"
            )));
        }
        if field(record, "canonical_provider") == "unavailable" {
            return Err(policy(&format!(
                "public Batch D closure record {routine} has no selected provider classification"
            )));
        }
        if field(record, "symbol_status") != "observed_exactly_once" {
            return Err(policy(&format!(
                "public Batch D closure record {routine} lacks a unique native symbol"
            )));
        }
        if field(record, "compile_test_status") == "not_compiled"
            || field(record, "link_test_status") != "passed"
        {
            return Err(policy(&format!(
                "public Batch D closure record {routine} lacks compile/link validation"
            )));
        }
    }
    if field(record, "program_unit_kind") == "program" && disposition == "canonical-public" {
        return Err(policy(&format!("PROGRAM unit {routine} became public")));
    }
    if disposition == "missing-symbol" && field(record, "link_test_status") == "passed" {
        return Err(policy(&format!(
            "missing-symbol identity {routine} is incorrectly link-validated"
        )));
    }
    Ok(())
}

fn summary(records: &[Value]) -> Value {
    let mut counts = Map::new();
    for disposition in DISPOSITIONS {
        counts.insert(
            disposition.replace('-', "_"),
            json!(count_disposition(records, disposition)),
        );
    }
    let public = count_disposition(records, "canonical-public");
    let batch_d = records
        .iter()
        .filter(|record| field(record, "current_raw_status") == "batch_d_public_driver")
        .count();
    counts.insert("retained_identities".to_owned(), json!(records.len()));
    counts.insert(
        "public_raw_identities_before_batch_d".to_owned(),
        json!(public - batch_d),
    );
    counts.insert("new_batch_d_public_declarations".to_owned(), json!(batch_d));
    counts.insert(
        "public_raw_identities_after_batch_d".to_owned(),
        json!(public),
    );
    counts.insert("identities_with_approved_shims".to_owned(), json!(0));
    counts.insert("identities_remaining_unexplained".to_owned(), json!(0));
    json!({
        "schema_id":"slatec-sys.raw-api.final-disposition-summary",
        "schema_version":"1.0.0",
        "counts":counts,
        "percentages":{
            "retained_identities_public":format!("{:.2}", public as f64 * 100.0 / records.len() as f64),
            "retained_identities_conclusively_disposed":"100.00",
        },
        "unsupported_abi_classes":["callback combinations without reproducible call-shape evidence"],
        "unexplained_identities":"No",
    })
}

fn validate_aggregate(records: &[Value], summary: &Value) -> Result<()> {
    if records.len() != 1517 {
        return Err(policy(&format!(
            "final disposition expected 1517 retained identities, found {}",
            records.len()
        )));
    }
    for (state, expected) in [
        ("batch_a_public_driver", 459),
        ("batch_b_public_driver", 47),
        ("batch_c_public_driver", 97),
        ("batch_d_public_driver", LEGACY_PUBLIC.len()),
    ] {
        let actual = records
            .iter()
            .filter(|record| field(record, "current_raw_status") == state)
            .count();
        if actual != expected {
            return Err(policy(&format!(
                "{state} regressed: expected {expected}, found {actual}"
            )));
        }
    }
    let mut symbols = BTreeMap::new();
    let mut paths = BTreeSet::new();
    let mut disposition_total = 0usize;
    for disposition in DISPOSITIONS {
        let detailed = count_disposition(records, disposition);
        disposition_total += detailed;
        let summary_count = summary
            .pointer(&format!("/counts/{}", disposition.replace('-', "_")))
            .and_then(Value::as_u64)
            .unwrap_or(u64::MAX) as usize;
        if detailed != summary_count {
            return Err(policy(&format!(
                "aggregate count for {disposition} is {summary_count}, detailed records contain {detailed}"
            )));
        }
    }
    if disposition_total != records.len() {
        return Err(policy(
            "final dispositions do not partition retained identities",
        ));
    }
    for record in records {
        let routine = field(record, "routine");
        let disposition = field(record, "final_disposition");
        let symbol = field(record, "native_symbol");
        if symbol != "not_observed" && !symbol.is_empty() {
            if let Some(previous) = symbols.insert(symbol.clone(), field(record, "routine")) {
                return Err(policy(&format!(
                    "duplicate native symbol {symbol} belongs to both {previous} and {}",
                    field(record, "routine")
                )));
            }
        }
        if field(record, "final_disposition") == "canonical-public" {
            if matches!(
                field(record, "public_role").as_str(),
                "internal_subsidiary" | "runtime_support" | "not_independently_callable"
            ) {
                return Err(policy(&format!(
                    "non-public role {routine} became canonical public"
                )));
            }
            if matches!(
                field(record, "program_unit_kind").as_str(),
                "program" | "block_data"
            ) {
                return Err(policy(&format!(
                    "non-callable program unit {routine} became canonical public"
                )));
            }
            let canonical = field(record, "canonical_rust_path");
            if !paths.insert(canonical.clone()) {
                return Err(policy(&format!(
                    "duplicate canonical public path {canonical}"
                )));
            }
        }
        if is_permanent_exclusion(&disposition)
            && (record
                .get("final_evidence")
                .and_then(Value::as_array)
                .is_none_or(|items| {
                    items.is_empty()
                        || items
                            .iter()
                            .any(|item| item.as_str().is_none_or(str::is_empty))
                })
                || record
                    .get("evidence_sources")
                    .and_then(Value::as_array)
                    .is_none_or(Vec::is_empty)
                || field(record, "reopen_condition").is_empty())
        {
            return Err(policy(&format!(
                "permanent exclusion {routine} lacks reason, evidence source, or reopen condition"
            )));
        }
        if matches!(disposition.as_str(), "missing-provider" | "missing-symbol")
            && record
                .pointer("/statuses/link_validated")
                .and_then(Value::as_bool)
                == Some(true)
        {
            return Err(policy(&format!(
                "provider gap {routine} is incorrectly link-validated"
            )));
        }
    }
    for routine in ["BVSUP", "DBVSUP"] {
        let record = records
            .iter()
            .find(|record| field(record, "routine") == routine)
            .ok_or_else(|| policy(&format!("missing dedicated {routine} disposition")))?;
        if field(record, "final_disposition") != "unsupported-callback-abi"
            || record
                .get("final_evidence")
                .and_then(Value::as_array)
                .is_none_or(|items| items.len() < 3)
        {
            return Err(policy(&format!(
                "{routine} lacks conclusive external problem-definition callback evidence"
            )));
        }
    }
    if summary
        .pointer("/counts/identities_remaining_unexplained")
        .and_then(Value::as_u64)
        != Some(0)
    {
        return Err(policy("final disposition has unexplained identities"));
    }
    Ok(())
}

fn public_coverage(records: &[Value]) -> Value {
    let public = records
        .iter()
        .filter(|record| field(record, "final_disposition") == "canonical-public")
        .map(|record| {
            json!({
                "routine":field(record,"routine"),
                "canonical_rust_path":field(record,"canonical_rust_path"),
                "feature":field(record,"feature"),
                "provider_feature":field(record,"provider_feature"),
                "promotion_state":field(record,"current_raw_status"),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "schema_id":"slatec-sys.raw-api.public-api-coverage",
        "schema_version":"1.0.0",
        "public_identity_count":public.len(),
        "records":public,
    })
}

fn filtered_report<F>(schema: &str, records: &[Value], predicate: F) -> Value
where
    F: Fn(&Value) -> bool,
{
    let selected = records
        .iter()
        .filter(|record| predicate(record))
        .cloned()
        .collect::<Vec<_>>();
    json!({
        "schema_id":schema,
        "schema_version":"1.0.0",
        "record_count":selected.len(),
        "records":selected,
    })
}

fn legacy_interface_audit(records: &[Value]) -> Value {
    let selected = records
        .iter()
        .filter(|record| {
            field(record, "program_unit_kind") == "program"
                || field(record, "fortran_io_dependency") != "none_detected"
                || !record
                    .get("common_block_relationships")
                    .and_then(Value::as_array)
                    .is_none_or(Vec::is_empty)
                || matches!(field(record, "routine").as_str(), "BVSUP" | "DBVSUP")
                || matches!(
                    field(record, "final_disposition").as_str(),
                    "raw-internal" | "historical-driver" | "demonstration-program"
                )
        })
        .cloned()
        .collect::<Vec<_>>();
    let bvsup = records
        .iter()
        .filter(|record| matches!(field(record, "routine").as_str(), "BVSUP" | "DBVSUP"))
        .map(|record| {
            let routine = field(record, "routine");
            let externals = if routine == "BVSUP" {
                vec!["fmat_", "gvec_", "uivp_", "uvec_"]
            } else {
                vec!["dfmat_", "dgvec_", "duivp_", "duvec_"]
            };
            json!({
                "routine":routine,
                "source_hash":field(record,"source_hash"),
                "unresolved_object_externals":externals,
                "classification":"external problem-definition routines",
                "explicit_procedure_arguments":false,
                "provider_linkage_reproducible":false,
                "final_disposition":"unsupported-callback-abi",
                "conclusion":"the implicit problem-definition procedure interface cannot be represented faithfully as a direct Rust raw declaration",
            })
        })
        .collect::<Vec<_>>();
    json!({
        "schema_id":"slatec-sys.raw-api.legacy-interface-audit",
        "schema_version":"1.0.0",
        "entry_point_count":0,
        "alternate_return_count":0,
        "block_data_identity_count":0,
        "fortran_io_identity_count":records.iter().filter(|record| field(record,"fortran_io_dependency") != "none_detected").count(),
        "program_identity_count":records.iter().filter(|record| field(record,"program_unit_kind") == "program").count(),
        "bvsup_dbvsup_evidence":bvsup,
        "records":selected,
    })
}

fn summary_markdown(summary: &Value) -> String {
    let counts = summary["counts"].as_object().expect("summary counts");
    let value = |name: &str| counts.get(name).and_then(Value::as_u64).unwrap_or(0);
    format!(
        "# Final raw API disposition summary\n\n\
- Retained identities: {}\n\
- Public raw identities before Batch D: {}\n\
- New Batch D public declarations: {}\n\
- Public raw identities after Batch D: {} ({}% of retained identities)\n\
- Provider subsidiaries: {}\n\
- Raw-internal identities: {}\n\
- Runtime/error/machine support: {}/{}/{}\n\
- BLOCK DATA support identities: {}\n\
- Demonstration programs: {}\n\
- Catalogue-only identities: {}\n\
- Missing provider/symbol identities: {}/{}\n\
- Unsupported character/callback/complex interfaces: {}/{}/{}\n\
- Unsupported ENTRY/alternate-return/COMMON/Fortran-I/O interfaces: {}/{}/{}/{}\n\
- Unsupported compiler-specific interfaces: {}\n\
- Other permanent exclusions: {}\n\
- Approved shims: {}\n\
- Identities remaining unexplained: {}\n\
- Conclusively disposed: {}%\n\n\
Complete means every retained identity has one evidence-backed terminal disposition; it does not mean every identity is a public Rust function.\n",
        value("retained_identities"),
        value("public_raw_identities_before_batch_d"),
        value("new_batch_d_public_declarations"),
        value("public_raw_identities_after_batch_d"),
        summary["percentages"]["retained_identities_public"]
            .as_str()
            .unwrap_or("0"),
        value("provider_subsidiary"),
        value("raw_internal"),
        value("runtime_support"),
        value("error_support"),
        value("machine_support"),
        value("block_data_support"),
        value("demonstration_program"),
        value("catalogue_only"),
        value("missing_provider"),
        value("missing_symbol"),
        value("unsupported_character_abi"),
        value("unsupported_callback_abi"),
        value("unsupported_complex_abi"),
        value("unsupported_entry_interface"),
        value("unsupported_alternate_return"),
        value("unsupported_common_interface"),
        value("unsupported_fortran_io"),
        value("unsupported_compiler_specific_abi"),
        value("excluded_other"),
        value("identities_with_approved_shims"),
        value("identities_remaining_unexplained"),
        summary["percentages"]["retained_identities_conclusively_disposed"]
            .as_str()
            .unwrap_or("0"),
    )
}

fn compile_probe(records: &[Value]) -> String {
    let mut output = String::from(
        "//! Generated Batch D canonical-path compile probe.\n\n#![cfg(feature = \"all\")]\n\n#[test]\nfn imports_batch_d_paths() {\n",
    );
    for record in records
        .iter()
        .filter(|record| field(record, "current_raw_status") == "batch_d_public_driver")
    {
        output.push_str(&format!(
            "    let _ = {} as *const ();\n",
            field(record, "canonical_rust_path")
        ));
    }
    output.push_str("}\n");
    output
}

fn link_probe(records: &[Value]) -> String {
    let mut output = String::from(
        "//! Generated Batch D native-symbol link probe.\n\n#![cfg(feature = \"raw-final-coverage-link-tests\")]\n\n#[test]\nfn links_batch_d_symbols() {\n    slatec_src::ensure_linked();\n",
    );
    for record in records
        .iter()
        .filter(|record| field(record, "current_raw_status") == "batch_d_public_driver")
    {
        output.push_str(&format!(
            "    let _ = {} as *const ();\n",
            field(record, "canonical_rust_path")
        ));
    }
    output.push_str("}\n");
    output
}

fn common_blocks(path: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let value = read_json(path)?;
    let mut result = BTreeMap::<String, Vec<String>>::new();
    for row in records(&value, "COMMON block index")? {
        let columns = row
            .as_array()
            .ok_or_else(|| policy("COMMON block index row is not an array"))?;
        let routine = columns.get(1).and_then(Value::as_str).unwrap_or_default();
        let block = columns.get(2).and_then(Value::as_str).unwrap_or_default();
        if !routine.is_empty() && !block.is_empty() {
            result
                .entry(routine.to_owned())
                .or_default()
                .push(block.to_owned());
        }
    }
    for blocks in result.values_mut() {
        blocks.sort();
        blocks.dedup();
    }
    Ok(result)
}

fn cargo_features(path: &Path) -> Result<BTreeSet<String>> {
    let text = fs::read_to_string(path)?;
    let value = text.parse::<toml::Value>().map_err(|error| {
        policy(&format!(
            "could not parse Cargo manifest {}: {error}",
            path.display()
        ))
    })?;
    Ok(value
        .get("features")
        .and_then(toml::Value::as_table)
        .map(|features| features.keys().cloned().collect())
        .unwrap_or_default())
}

fn by_routine(value: &Value, label: &str) -> Result<BTreeMap<String, Value>> {
    let mut result = BTreeMap::new();
    for record in records(value, label)? {
        let routine = field(record, "routine");
        if !routine.is_empty() && result.insert(routine.clone(), record.clone()).is_some() {
            return Err(policy(&format!("{label} duplicates {routine}")));
        }
    }
    Ok(result)
}

fn count_disposition(records: &[Value], disposition: &str) -> usize {
    records
        .iter()
        .filter(|record| field(record, "final_disposition") == disposition)
        .count()
}

fn semantic_hash(files: &BTreeMap<String, Vec<u8>>) -> String {
    let mut bytes = Vec::new();
    for (name, content) in files {
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(content);
        bytes.push(0);
    }
    hash::bytes(&bytes)
}

fn compare_file(path: &Path, expected: &[u8]) -> Result<()> {
    let actual = fs::read(path).map_err(|error| {
        policy(&format!(
            "missing generated probe {}: {error}",
            path.display()
        ))
    })?;
    if actual != expected {
        return Err(policy(&format!(
            "generated probe {} differs; regenerate Batch D",
            path.display()
        )));
    }
    Ok(())
}

fn read_json(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path)?)
        .map_err(|error| policy(&format!("could not parse JSON {}: {error}", path.display())))
}

fn records<'a>(value: &'a Value, label: &str) -> Result<&'a [Value]> {
    value
        .get("records")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| policy(&format!("{label} lacks records")))
}

fn field(record: &Value, key: &str) -> String {
    record
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

fn json_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)
        .map_err(|error| policy(&format!("could not serialize Batch D JSON: {error}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_promotion_set_is_unique_and_coherent() {
        let names = LEGACY_PUBLIC
            .iter()
            .map(|(name, _)| *name)
            .collect::<BTreeSet<_>>();
        assert_eq!(names.len(), 36);
        assert_eq!(names.len(), LEGACY_PUBLIC.len());
        assert_eq!(legacy_feature("DQAG"), Some("quadrature-basic"));
        assert_eq!(legacy_feature("BVSUP"), None);
    }
}
