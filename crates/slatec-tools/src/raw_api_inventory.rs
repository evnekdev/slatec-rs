//! Canonical, evidence-backed status inventory for the public `slatec-sys` API.
//!
//! This module deliberately joins facts rather than promoting declarations.  In
//! particular, an ABI-shaped generated declaration is never considered reviewed
//! merely because it compiled or appeared in `slatec_sys::generated`.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Input and output locations used by the raw API inventory generator.
pub struct RawApiPaths<'a> {
    pub catalogue_dir: &'a Path,
    pub ffi_dir: &'a Path,
    pub ffi_inventory_dir: &'a Path,
    pub ffi_validation_dir: &'a Path,
    pub safe_api_dir: &'a Path,
    pub corrections_path: &'a Path,
    pub sys_dir: &'a Path,
    pub src_dir: &'a Path,
    pub facade_dir: &'a Path,
    pub docs_dir: &'a Path,
    pub output_dir: &'a Path,
}

/// Result of a deterministic inventory generation or validation pass.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RawApiInventoryResult {
    pub status: String,
    pub identity_count: usize,
    pub reviewed_count: usize,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
}

#[derive(Clone, Debug)]
struct Interface {
    program_unit_id: String,
    name: String,
    kind: String,
    source_hash: String,
    native_symbol: String,
    confidence: String,
    batch: String,
    review_state: String,
    argument_ids: Vec<String>,
}

#[derive(Clone, Debug)]
struct Correction {
    routine: String,
    source_hash: String,
    status: String,
    canonical_path: String,
    compatibility_paths: Vec<String>,
    feature: String,
    provider_feature: String,
    role: String,
    source_file: String,
    arguments: Vec<String>,
    documentation: Value,
    link_test_status: String,
    runtime_test_status: String,
    safe_wrapper_path: String,
}

/// Generates all R1 raw API reports and rejects an inconsistent reviewed entry.
pub fn generate(paths: RawApiPaths<'_>) -> Result<RawApiInventoryResult> {
    let catalogue_value = read_json(&paths.catalogue_dir.join("routine-catalogue.json"))?;
    let catalogue = records(&catalogue_value, "catalogue")?;
    let interfaces = interfaces(&paths.ffi_dir.join("interface-inventory.json"))?;
    let arguments = argument_names(&paths.ffi_inventory_dir.join("argument-index.json"))?;
    let safe_api_index = read_json(&paths.safe_api_dir.join("function-index.json"))?;
    let _safe_api_records = records(&safe_api_index, "safe API function index")?;
    let corrections = corrections(paths.corrections_path)?;
    let validation = validation_batches(
        &paths
            .ffi_validation_dir
            .join("batch-validation-summary.json"),
    )?;
    let sys_features = cargo_features(&paths.sys_dir.join("Cargo.toml"))?;
    let src_features = cargo_features(&paths.src_dir.join("Cargo.toml"))?;
    let facade_features = cargo_features(&paths.facade_dir.join("Cargo.toml"))?;
    let legacy_declarations = legacy_declarations(paths.sys_dir)?;

    let mut by_provider = BTreeMap::new();
    let mut by_name = BTreeMap::<String, Vec<Interface>>::new();
    for interface in interfaces {
        by_provider.insert(interface.program_unit_id.clone(), interface.clone());
        by_name
            .entry(interface.name.clone())
            .or_default()
            .push(interface);
    }

    let mut correction_by_name = BTreeMap::new();
    for correction in corrections {
        if correction_by_name
            .insert(correction.routine.clone(), correction)
            .is_some()
        {
            return Err(policy(
                "raw-api corrections must contain one record per routine",
            ));
        }
    }

    let mut output_records = Vec::with_capacity(catalogue.len());
    for catalogue_record in catalogue {
        let name = string(catalogue_record, "normalized_name")?;
        let source_hash = nonempty_or(field(catalogue_record, "source_hash"), "unavailable");
        let provider = catalogue_record
            .pointer("/canonical_provider/provider_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let interface = by_provider.get(provider).or_else(|| {
            by_name.get(&name).and_then(|candidates| {
                candidates
                    .iter()
                    .find(|candidate| {
                        source_hash != "unavailable" && candidate.source_hash == source_hash
                    })
                    .or_else(|| candidates.first())
            })
        });
        let correction = correction_by_name.get(&name);
        let legacy_paths = legacy_declarations.get(&name).cloned().unwrap_or_default();
        if let Some(correction) = correction {
            if correction.source_hash != source_hash {
                return Err(policy(&format!(
                    "raw-api correction for {name} is guarded by {}, but catalogue selected {source_hash}",
                    correction.source_hash
                )));
            }
            if correction.source_file
                != catalogue_record
                    .get("source_file")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
            {
                return Err(policy(&format!(
                    "raw-api correction for {name} names a different selected source file"
                )));
            }
        }

        let argument_order = interface
            .map(|item| {
                item.argument_ids
                    .iter()
                    .filter_map(|id| arguments.get(id).cloned())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if let Some(correction) = correction {
            if correction.arguments != argument_order {
                return Err(policy(&format!(
                    "raw-api correction argument order for {name} differs from executable declaration"
                )));
            }
        }

        let generated_status = generated_status(interface);
        let reviewed_status = correction
            .map(|item| item.status.clone())
            .or_else(|| {
                (!legacy_paths.is_empty())
                    .then(|| "preexisting_family_declaration_requires_r1_review".to_owned())
            })
            .unwrap_or_else(|| "not_reviewed_by_raw_api_registry".to_owned());
        let raw_state = correction
            .map(|item| item.status.clone())
            .or_else(|| (!legacy_paths.is_empty()).then(|| "documentation_or_tooling".to_owned()))
            .unwrap_or_else(|| raw_state(catalogue_record, interface, &generated_status));
        let feature = correction
            .map(|item| item.feature.clone())
            .unwrap_or_else(|| generated_feature(interface));
        let provider_feature = correction
            .map(|item| item.provider_feature.clone())
            .unwrap_or_else(|| "not_assigned".to_owned());
        let intended_module = correction
            .map(|item| module_from_path(&item.canonical_path))
            .unwrap_or_else(|| taxonomy(catalogue_record, &name));
        let canonical_path = correction
            .map(|item| item.canonical_path.clone())
            .unwrap_or_else(|| "not_promoted".to_owned());
        let is_reviewed = correction.is_some();
        let documentation_status = if is_reviewed {
            "complete_authored"
        } else if !legacy_paths.is_empty() {
            "legacy_partial_rustdoc"
        } else {
            "not_documented"
        };
        let argument_documentation_status = if is_reviewed {
            "complete_authored"
        } else if !legacy_paths.is_empty() {
            "requires_r1_argument_review"
        } else {
            "not_documented"
        };
        let signature_review_status = if is_reviewed {
            "reviewed_against_source_hash"
        } else if !legacy_paths.is_empty() {
            "preexisting_review_not_requalified_for_r1"
        } else if generated_status == "generated_abi_validated" {
            "profile_validated_not_semantically_reviewed"
        } else {
            "not_reviewed"
        };
        let compile_test_status = if interface.is_some() {
            "compiler_observed"
        } else {
            "not_compiled"
        };
        let link_test_status = correction
            .map(|item| item.link_test_status.clone())
            .unwrap_or_else(|| batch_status(interface, &validation, "link_status"));
        let runtime_test_status = correction
            .map(|item| item.runtime_test_status.clone())
            .unwrap_or_else(|| batch_status(interface, &validation, "runtime_status"));
        let safe_wrapper_status = correction
            .map(|item| item.safe_wrapper_path.clone())
            .or_else(|| {
                catalogue_record
                    .get("safe_api_paths")
                    .and_then(Value::as_array)
                    .and_then(|items| items.first())
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            })
            .unwrap_or_else(|| "not_safely_wrapped".to_owned());
        let feasibility = if is_reviewed {
            "public_raw_reviewed"
        } else if generated_status == "generated_abi_validated" {
            "promotable_after_semantic_review"
        } else if raw_state == "catalogue_only"
            || raw_state.starts_with("unsupported_")
            || raw_state == "runtime_or_machine_support"
            || raw_state == "not_independently_callable"
        {
            "explicitly_excluded"
        } else {
            "unreviewed"
        };
        let exclusion_reason = if is_reviewed || generated_status == "generated_abi_validated" {
            "none".to_owned()
        } else {
            exclusion_reason(&raw_state)
        };
        let role = correction
            .map(|item| item.role.clone())
            .unwrap_or_else(|| driver_role(catalogue_record));
        let native_symbol = interface
            .map(|item| item.native_symbol.clone())
            .unwrap_or_else(|| "not_observed".to_owned());
        let symbol_status = if interface
            .map(|item| !item.native_symbol.is_empty())
            .unwrap_or(false)
        {
            "observed_exactly_once"
        } else {
            "missing_symbol"
        };

        output_records.push(json!({
            "routine": name,
            "canonical_provider": provider_or_unavailable(catalogue_record),
            "source_hash": source_hash,
            "program_unit_kind": interface.map(|item| item.kind.clone()).unwrap_or_else(|| field(catalogue_record, "kind")),
            "historical_role": field(catalogue_record, "historical_role"),
            "driver_role": role,
            "primary_family": field(catalogue_record, "primary_family"),
            "precision": field(catalogue_record, "precision"),
            "native_symbol": native_symbol,
            "symbol_status": symbol_status,
            "generated_declaration_status": generated_status,
            "reviewed_declaration_status": reviewed_status,
            "raw_api_state": raw_state,
            "canonical_rust_path": canonical_path,
            "intended_canonical_module": intended_module,
            "compatibility_paths": correction.map(|item| item.compatibility_paths.clone()).unwrap_or_default(),
            "legacy_family_declaration_status":if legacy_paths.is_empty() { "not_present" } else { "preexisting_family_declaration_requires_r1_review" },
            "legacy_rust_paths":legacy_paths,
            "feature": feature,
            "provider_feature": provider_feature,
            "safe_facade_feature": correction
                .map(|_| safe_feature(&name, &facade_features))
                .unwrap_or_else(|| "not_assigned".to_owned()),
            "documentation_status": documentation_status,
            "argument_documentation_status": argument_documentation_status,
            "documentation_evidence": correction.map(|item| item.documentation.clone()).unwrap_or_else(|| json!({"purpose":"unavailable","arguments":"unavailable"})),
            "signature_review_status": signature_review_status,
            "argument_order": argument_order,
            "compile_test_status": compile_test_status,
            "link_test_status": link_test_status,
            "runtime_test_status": runtime_test_status,
            "example_status": example_status(catalogue_record),
            "safe_wrapper_status": safe_wrapper_status,
            "public_raw_feasibility": feasibility,
            "exclusion_reason": exclusion_reason,
        }));
    }
    output_records.sort_by_key(|record| field(record, "routine"));
    validate_reviewed(&output_records, &sys_features, &src_features, paths.sys_dir)?;

    let coverage = coverage_summary(&output_records);
    let taxonomy_output = taxonomy_report(&output_records);
    let canonical_paths = canonical_paths(&output_records);
    let documentation_audit = documentation_audit(&output_records);
    let feature_map = feature_map(
        &output_records,
        &sys_features,
        &src_features,
        &facade_features,
    );
    let exclusions = exclusion_report(&output_records);
    let priority = promotion_priority(&output_records);
    let roots = roots_report(&output_records);
    let routine_status = json!({
        "schema_id":"slatec-sys.raw-api.routine-status",
        "schema_version":"1.0.0",
        "record_model":"exactly one record per retained catalogue identity",
        "state_model":["reviewed_public_driver","reviewed_public_subsidiary","generated_candidate","generated_abi_validated","source_present_unbound","unsupported_callback_abi","unsupported_complex_return_abi","unsupported_character_return_abi","unsupported_entry_or_alternate_return","conflicting_interface","ambiguous_symbol","missing_symbol","not_independently_callable","runtime_or_machine_support","block_data","documentation_or_tooling","catalogue_only","external_dependency"],
        "records": output_records,
    });
    let validation_summary =
        validation_markdown(&coverage, &documentation_audit, &feature_map, &roots);
    let mut files = BTreeMap::new();
    files.insert("routine-status.json", json_bytes(&routine_status)?);
    files.insert("coverage-summary.json", json_bytes(&coverage)?);
    files.insert("canonical-paths.json", json_bytes(&canonical_paths)?);
    files.insert("module-taxonomy.json", json_bytes(&taxonomy_output)?);
    files.insert(
        "documentation-audit.json",
        json_bytes(&documentation_audit)?,
    );
    files.insert("feature-provider-map.json", json_bytes(&feature_map)?);
    files.insert("exclusion-report.json", json_bytes(&exclusions)?);
    files.insert("promotion-priority.json", json_bytes(&priority)?);
    files.insert("roots-family-report.json", json_bytes(&roots)?);
    files.insert("validation-summary.md", validation_summary.into_bytes());
    let semantic_hash = outputs_hash(&files);
    files.insert(
        "manifest.json",
        json_bytes(&json!({
            "schema_id":"slatec-sys.raw-api.manifest",
            "schema_version":"1.0.0",
            "semantic_hash": semantic_hash,
            "inputs":{
                "catalogue":"generated/slatec-routines/routine-catalogue.json",
                "interface_inventory":"generated/ffi/interface-inventory.json",
                "compiler_validation":"generated/ffi-validation/batch-validation-summary.json",
                "corrections":"metadata/raw-api-corrections.json"
            },
            "outputs":files.keys().collect::<Vec<_>>(),
        }))?,
    );
    write_outputs(paths.output_dir, &files)?;
    write_routine_page_statuses(paths.docs_dir, &output_records)?;
    write_raw_coverage_page(paths.docs_dir, &coverage)?;
    Ok(RawApiInventoryResult {
        status: "success".to_owned(),
        identity_count: output_records.len(),
        reviewed_count: output_records
            .iter()
            .filter(|record| field(record, "reviewed_declaration_status").starts_with("reviewed_"))
            .count(),
        semantic_hash,
        output_dir: paths.output_dir.to_path_buf(),
    })
}

/// Regenerates and independently checks the committed R1 status model.
pub fn validate(paths: RawApiPaths<'_>) -> Result<RawApiInventoryResult> {
    let result = generate(paths)?;
    let status = read_json(&result.output_dir.join("routine-status.json"))?;
    let records = records(&status, "raw-api status")?;
    if records.len() != result.identity_count {
        return Err(policy(
            "raw-api status record count changed during validation",
        ));
    }
    for required in [
        "routine",
        "canonical_provider",
        "source_hash",
        "program_unit_kind",
        "historical_role",
        "primary_family",
        "precision",
        "native_symbol",
        "symbol_status",
        "generated_declaration_status",
        "reviewed_declaration_status",
        "raw_api_state",
        "driver_role",
        "canonical_rust_path",
        "feature",
        "provider_feature",
        "documentation_status",
        "argument_documentation_status",
        "signature_review_status",
        "compile_test_status",
        "link_test_status",
        "runtime_test_status",
        "example_status",
        "safe_wrapper_status",
        "public_raw_feasibility",
        "exclusion_reason",
        "legacy_family_declaration_status",
        "legacy_rust_paths",
    ] {
        if records.iter().any(|record| record.get(required).is_none()) {
            return Err(policy(&format!(
                "raw-api record missing required field {required}"
            )));
        }
    }
    for record in records {
        let reviewed = field(record, "reviewed_declaration_status").starts_with("reviewed_");
        let state = field(record, "raw_api_state");
        if reviewed != state.starts_with("reviewed_") {
            return Err(policy(&format!(
                "raw-api status disagrees about reviewed state for {}",
                field(record, "routine")
            )));
        }
        if field(record, "public_raw_feasibility") == "explicitly_excluded"
            && field(record, "exclusion_reason") == "none"
        {
            return Err(policy(&format!(
                "explicitly excluded routine {} lacks a reason",
                field(record, "routine")
            )));
        }
    }
    Ok(result)
}

fn interfaces(path: &Path) -> Result<Vec<Interface>> {
    let rows = table(&read_json(path)?, "raw FFI interface inventory")?;
    rows.into_iter()
        .map(|row| {
            Ok(Interface {
                program_unit_id: required(&row, "program_unit_id", path)?,
                name: required(&row, "normalized_name", path)?,
                kind: required(&row, "kind", path)?,
                source_hash: required(&row, "raw_sha256", path)?,
                native_symbol: optional(&row, "observed_raw_symbol"),
                confidence: required(&row, "confidence_class", path)?,
                batch: optional(&row, "binding_batch"),
                review_state: optional(&row, "review_state"),
                argument_ids: strings(row.get("argument_ids")),
            })
        })
        .collect::<Result<Vec<_>>>()
}

fn argument_names(path: &Path) -> Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    for row in table(&read_json(path)?, "raw FFI argument inventory")? {
        result.insert(
            required(&row, "id", path)?,
            required(&row, "normalized_name", path)?,
        );
    }
    Ok(result)
}

fn corrections(path: &Path) -> Result<Vec<Correction>> {
    let value = read_json(path)?;
    let records = records(&value, "raw-api corrections")?;
    records
        .iter()
        .map(|record| {
            Ok(Correction {
                routine: string(record, "routine")?,
                source_hash: string(record, "source_hash")?,
                status: string(record, "review_status")?,
                canonical_path: string(record, "canonical_rust_path")?,
                compatibility_paths: strings(record.get("compatibility_paths")),
                feature: string(record, "feature")?,
                provider_feature: string(record, "provider_feature")?,
                role: string(record, "driver_role")?,
                source_file: string(record, "source_file")?,
                arguments: strings(record.get("arguments")),
                documentation: record
                    .get("documentation")
                    .cloned()
                    .ok_or_else(|| policy("raw-api correction lacks documentation evidence"))?,
                link_test_status: string(record, "link_test_status")?,
                runtime_test_status: string(record, "runtime_test_status")?,
                safe_wrapper_path: string(record, "safe_wrapper_path")?,
            })
        })
        .collect()
}

fn validation_batches(path: &Path) -> Result<BTreeMap<String, BTreeMap<String, String>>> {
    let mut result = BTreeMap::new();
    for row in table(&read_json(path)?, "raw FFI validation batches")? {
        let batch = required(&row, "batch", path)?;
        result.insert(
            batch,
            ["link_status", "runtime_status"]
                .iter()
                .map(|key| ((*key).to_owned(), optional(&row, key)))
                .collect(),
        );
    }
    Ok(result)
}

fn cargo_features(path: &Path) -> Result<BTreeSet<String>> {
    let text = fs::read_to_string(path)?;
    let value: toml::Value = toml::from_str(&text)?;
    Ok(value
        .get("features")
        .and_then(toml::Value::as_table)
        .map(|table| table.keys().cloned().collect())
        .unwrap_or_default())
}

fn legacy_declarations(sys_dir: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let source_dir = sys_dir.join("src");
    let mut result = BTreeMap::<String, Vec<String>>::new();
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if stem == "lib" {
            continue;
        }
        let source = fs::read_to_string(&path)?;
        for line in source.lines() {
            let Some(name) = line
                .trim_start()
                .strip_prefix("pub fn ")
                .and_then(|value| value.split_once('(').map(|(name, _)| name))
            else {
                continue;
            };
            if name.is_empty()
                || !name
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
            {
                continue;
            }
            result
                .entry(name.to_ascii_uppercase())
                .or_default()
                .push(format!("slatec_sys::{stem}::{name}"));
        }
    }
    for paths in result.values_mut() {
        paths.sort();
        paths.dedup();
    }
    Ok(result)
}

fn generated_status(interface: Option<&Interface>) -> String {
    match interface.map(|item| item.confidence.as_str()) {
        Some("generated_standard") | Some("generated_abi_sensitive") => {
            "generated_abi_validated".to_owned()
        }
        Some(_) => "not_generated".to_owned(),
        None => "not_generated".to_owned(),
    }
}

fn raw_state(record: &Value, interface: Option<&Interface>, generated: &str) -> String {
    if record
        .get("identity_status")
        .and_then(Value::as_str)
        .map(|value| value.contains("catalogue_only"))
        .unwrap_or(false)
    {
        return "catalogue_only".to_owned();
    }
    match interface {
        Some(item)
            if item.confidence == "manual_review_required" && item.batch == "batch_callbacks" =>
        {
            "unsupported_callback_abi".to_owned()
        }
        Some(item) if item.confidence == "non_callable_infrastructure" => {
            "runtime_or_machine_support".to_owned()
        }
        Some(item) if item.review_state.contains("conflict") => "conflicting_interface".to_owned(),
        Some(item) if item.native_symbol.is_empty() => "missing_symbol".to_owned(),
        Some(_) if generated == "generated_abi_validated" => "generated_abi_validated".to_owned(),
        Some(_) => "source_present_unbound".to_owned(),
        None if record.get("source_hash").and_then(Value::as_str).is_some() => {
            "source_present_unbound".to_owned()
        }
        None => "catalogue_only".to_owned(),
    }
}

fn generated_feature(interface: Option<&Interface>) -> String {
    match interface.map(|item| item.batch.as_str()) {
        Some("batch_numeric_scalar_subroutines") => "raw-ffi-numeric-scalar-subroutines".to_owned(),
        Some("batch_numeric_array_subroutines") => "raw-ffi-numeric-array-subroutines".to_owned(),
        Some("batch_scalar_functions") => "raw-ffi-scalar-functions".to_owned(),
        Some("batch_complex_arguments") => "raw-ffi-complex-arguments".to_owned(),
        Some("batch_logical") => "raw-ffi-logical".to_owned(),
        Some("batch_character") => "raw-ffi-character".to_owned(),
        Some("batch_callbacks") => "raw-ffi-callbacks".to_owned(),
        _ => "not_assigned".to_owned(),
    }
}

fn batch_status(
    interface: Option<&Interface>,
    validations: &BTreeMap<String, BTreeMap<String, String>>,
    key: &str,
) -> String {
    if generated_status(interface) != "generated_abi_validated" {
        return "not_tested".to_owned();
    }
    interface
        .and_then(|item| validations.get(&item.batch))
        .and_then(|record| record.get(key))
        .cloned()
        .unwrap_or_else(|| "not_tested".to_owned())
}

fn taxonomy(record: &Value, routine: &str) -> String {
    if matches!(routine, "FZERO" | "DFZERO") {
        return "roots::scalar".to_owned();
    }
    if matches!(routine, "CPZERO" | "RPZERO") {
        return "roots::polynomial".to_owned();
    }
    match field(record, "primary_family").as_str() {
        "Approximation" => "approximation".to_owned(),
        "Arithmetic and extended-range arithmetic"
        | "Documentation and source-processing tools"
        | "Error handling"
        | "Runtime and machine support"
        | "Shared numerical utilities" => "runtime".to_owned(),
        "Dense linear algebra" => "linear_algebra::dense".to_owned(),
        "Eigenvalue problems" => "eigen".to_owned(),
        "Elementary and transcendental functions" | "Special functions" => "special".to_owned(),
        "FFTPACK transforms" => "fftpack".to_owned(),
        "FISHPACK elliptic PDE solvers" => "pde::fishpack".to_owned(),
        "Interpolation" | "PCHIP" => "interpolation".to_owned(),
        "Linear algebra kernels" => "blas".to_owned(),
        "Nonlinear equations" => "nonlinear".to_owned(),
        "Numerical quadrature" => "quadrature".to_owned(),
        "ODE solvers" if routine.starts_with("DDA") || routine.starts_with("SDA") => {
            "dae".to_owned()
        }
        "ODE solvers" => "ode".to_owned(),
        "Optimization and least squares"
            if routine.contains("LSE")
                || routine.contains("NNLS")
                || routine.contains("BOLS")
                || routine.contains("LSQ") =>
        {
            "least_squares".to_owned()
        }
        "Optimization and least squares" => "optimization".to_owned(),
        "Probability and statistics" => "statistics".to_owned(),
        _ => "unclassified".to_owned(),
    }
}

fn module_from_path(path: &str) -> String {
    path.strip_prefix("slatec_sys::")
        .unwrap_or(path)
        .rsplit_once("::")
        .map(|(module, _)| module.to_owned())
        .unwrap_or_else(|| "runtime".to_owned())
}

fn driver_role(record: &Value) -> String {
    match field(record, "historical_role").as_str() {
        "user_callable" => "historically_user_callable_driver".to_owned(),
        "subsidiary" => "internal_subsidiary".to_owned(),
        "shared_utility" => "shared_utility".to_owned(),
        "runtime_support" => "runtime_support".to_owned(),
        _ => "not_independently_callable".to_owned(),
    }
}

fn exclusion_reason(state: &str) -> String {
    match state {
        "unsupported_callback_abi" => {
            "callback ABI has compiler-shape evidence but no routine-specific callback contract"
                .to_owned()
        }
        "runtime_or_machine_support" => {
            "runtime or machine-support unit is not independently callable".to_owned()
        }
        "catalogue_only" => "catalogue identity has no selected executable provider".to_owned(),
        "missing_symbol" => "selected provider produced no unique native symbol".to_owned(),
        "conflicting_interface" => "source interface evidence conflicts".to_owned(),
        "source_present_unbound" => {
            "source exists but no reviewed or ABI-validated public declaration is recorded"
                .to_owned()
        }
        "documentation_or_tooling" => "pre-existing declaration remains deferred until the R1 source-hash, argument-documentation, and ABI review gate passes".to_owned(),
        _ => "not explicitly excluded".to_owned(),
    }
}

fn provider_or_unavailable(record: &Value) -> String {
    record
        .pointer("/canonical_provider/provider_id")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| "unavailable".to_owned())
}

fn safe_feature(routine: &str, facade_features: &BTreeSet<String>) -> String {
    let candidate = if routine == "POIS3D" {
        "fishpack-pois3d"
    } else if routine == "HWSCRT" {
        "fishpack-cartesian-2d"
    } else if matches!(routine, "FZERO" | "DFZERO") {
        "roots-scalar"
    } else {
        "not_assigned"
    };
    if candidate != "not_assigned" && facade_features.contains(candidate) {
        candidate.to_owned()
    } else {
        "not_assigned".to_owned()
    }
}

fn example_status(record: &Value) -> String {
    if record
        .get("safe_api_paths")
        .and_then(Value::as_array)
        .map(|items| !items.is_empty())
        .unwrap_or(false)
    {
        "safe_facade_example_or_test".to_owned()
    } else {
        "not_recorded".to_owned()
    }
}

fn validate_reviewed(
    records: &[Value],
    sys_features: &BTreeSet<String>,
    src_features: &BTreeSet<String>,
    sys_dir: &Path,
) -> Result<()> {
    let mut paths = BTreeSet::new();
    for record in records
        .iter()
        .filter(|record| field(record, "reviewed_declaration_status").starts_with("reviewed_"))
    {
        for required in [
            "canonical_provider",
            "source_hash",
            "native_symbol",
            "canonical_rust_path",
            "feature",
            "provider_feature",
        ] {
            if field(record, required).is_empty() || field(record, required) == "not_assigned" {
                return Err(policy(&format!(
                    "reviewed {} lacks {required}",
                    field(record, "routine")
                )));
            }
        }
        if !paths.insert(field(record, "canonical_rust_path")) {
            return Err(policy(
                "reviewed declarations must have unique canonical paths",
            ));
        }
        if field(record, "symbol_status") != "observed_exactly_once" {
            return Err(policy(&format!(
                "reviewed {} lacks a unique observed native symbol",
                field(record, "routine")
            )));
        }
        if field(record, "signature_review_status") != "reviewed_against_source_hash" {
            return Err(policy(&format!(
                "reviewed {} lacks a source-hash signature review",
                field(record, "routine")
            )));
        }
        if !sys_features.contains(&field(record, "feature")) {
            return Err(policy(&format!(
                "reviewed {} has no slatec-sys feature",
                field(record, "routine")
            )));
        }
        if !src_features.contains(&field(record, "provider_feature")) {
            return Err(policy(&format!(
                "reviewed {} has no slatec-src provider feature",
                field(record, "routine")
            )));
        }
        if field(record, "documentation_status") != "complete_authored"
            || field(record, "argument_documentation_status") != "complete_authored"
            || !record
                .get("documentation_evidence")
                .and_then(|value| value.get("safety"))
                .and_then(Value::as_str)
                .map(|value| !value.is_empty())
                .unwrap_or(false)
        {
            return Err(policy(&format!(
                "reviewed {} lacks complete documentation",
                field(record, "routine")
            )));
        }
        if field(record, "link_test_status") != "passed"
            || field(record, "runtime_test_status") != "passed"
        {
            return Err(policy(&format!(
                "reviewed {} lacks required link/runtime validation",
                field(record, "routine")
            )));
        }
        let source = reviewed_source(sys_dir, &field(record, "routine"))?;
        if !source.contains("# Safety") {
            return Err(policy(&format!(
                "reviewed {} has no Rustdoc Safety section",
                field(record, "routine")
            )));
        }
        if !source.contains("https://") {
            return Err(policy(&format!(
                "reviewed {} has no Rustdoc source link",
                field(record, "routine")
            )));
        }
        for argument in strings(record.get("argument_order")) {
            if !source.contains(&format!("`{argument}`")) {
                return Err(policy(&format!(
                    "reviewed {} lacks Rustdoc for argument {argument}",
                    field(record, "routine")
                )));
            }
        }
    }
    Ok(())
}

fn reviewed_source(sys_dir: &Path, routine: &str) -> Result<String> {
    let file = match routine {
        "FZERO" | "DFZERO" => "src/roots.rs",
        "HWSCRT" => "src/fishpack_cartesian_2d.rs",
        "POIS3D" => "src/fishpack_pois3d.rs",
        _ => {
            return Err(policy(
                "reviewed raw declaration lacks a registered source file",
            ));
        }
    };
    Ok(fs::read_to_string(sys_dir.join(file))?)
}

fn coverage_summary(records: &[Value]) -> Value {
    let count = |predicate: &dyn Fn(&Value) -> bool| {
        records.iter().filter(|record| predicate(record)).count()
    };
    json!({
        "schema_id":"slatec-sys.raw-api.coverage-summary",
        "schema_version":"1.0.0",
        "metric_definitions":{
            "generated_raw_declaration_candidates":"records with a compiler-emitted declaration candidate",
            "abi_validated_generated_declarations":"generated candidates in an explicitly validated ABI batch",
            "reviewed_family_raw_declarations":"only entries in the hash-guarded correction registry",
            "preexisting_family_declarations_pending_r1_review":"legacy family extern declarations that have not passed the R1 documentation and source-hash review gate",
            "provider_backed_callable_raw_routines":"records with a selected provider and an observed native symbol",
            "fully_documented_raw_routines":"reviewed records with complete routine, argument, and Safety documentation"
        },
        "counts":{
            "retained_identities":records.len(),
            "historically_user_callable_routines":count(&|r| field(r,"historical_role")=="user_callable"),
            "generated_raw_declaration_candidates":count(&|r| field(r,"generated_declaration_status")=="generated_abi_validated"),
            "abi_validated_generated_declarations":count(&|r| field(r,"generated_declaration_status")=="generated_abi_validated"),
            "reviewed_family_raw_declarations":count(&|r| field(r,"reviewed_declaration_status").starts_with("reviewed_")),
            "reviewed_user_callable_raw_drivers":count(&|r| field(r,"reviewed_declaration_status")=="reviewed_public_driver"),
            "reviewed_public_subsidiaries":count(&|r| field(r,"reviewed_declaration_status")=="reviewed_public_subsidiary"),
            "provider_backed_callable_raw_routines":count(&|r| field(r,"symbol_status")=="observed_exactly_once" && field(r,"driver_role")!="runtime_support"),
            "link_tested_raw_routines":count(&|r| field(r,"link_test_status")=="passed"),
        "runtime_tested_raw_routines":count(&|r| field(r,"runtime_test_status")=="passed"),
        "fully_documented_raw_routines":count(&|r| field(r,"documentation_status")=="complete_authored" && field(r,"argument_documentation_status")=="complete_authored"),
        "preexisting_family_declarations_pending_r1_review":count(&|r| field(r,"legacy_family_declaration_status")!="not_present" && !field(r,"reviewed_declaration_status").starts_with("reviewed_")),
            "safely_wrapped_routines":count(&|r| field(r,"safe_wrapper_status")!="not_safely_wrapped"),
            "explicitly_excluded_routines":count(&|r| field(r,"public_raw_feasibility")=="explicitly_excluded"),
            "unclassified_routines":count(&|r| field(r,"intended_canonical_module")=="unclassified"),
        }
    })
}

fn taxonomy_report(records: &[Value]) -> Value {
    let modules = [
        "blas",
        "linear_algebra::dense",
        "linear_algebra::banded",
        "linear_algebra::packed",
        "linear_algebra::sparse",
        "eigen",
        "roots::scalar",
        "roots::polynomial",
        "nonlinear",
        "least_squares",
        "optimization",
        "quadrature",
        "ode",
        "dae",
        "pde::fishpack",
        "fftpack",
        "interpolation",
        "approximation",
        "special",
        "statistics",
        "integral_equations",
        "runtime",
    ];
    json!({
        "schema_id":"slatec-sys.raw-api.module-taxonomy",
        "schema_version":"1.0.0",
        "long_term_modules":modules,
        "mapping":records.iter().map(|record| json!({
            "routine":field(record,"routine"),
            "primary_family":field(record,"primary_family"),
            "intended_canonical_module":field(record,"intended_canonical_module"),
            "promotion_state":field(record,"raw_api_state"),
        })).collect::<Vec<_>>()
    })
}

fn canonical_paths(records: &[Value]) -> Value {
    json!({
        "schema_id":"slatec-sys.raw-api.canonical-paths",
        "schema_version":"1.0.0",
        "policy":"One callable routine receives one canonical path when promoted. Existing paths remain compatibility re-exports; generated ABI-shaped module paths are transitional and unstable.",
        "records":records.iter().map(|record| json!({
            "routine":field(record,"routine"),
            "intended_module":field(record,"intended_canonical_module"),
            "canonical_rust_path":field(record,"canonical_rust_path"),
            "intended_canonical_rust_path":planned_canonical_path(record),
            "compatibility_paths":record.get("compatibility_paths").cloned().unwrap_or_else(|| json!([])),
            "legacy_rust_paths":record.get("legacy_rust_paths").cloned().unwrap_or_else(|| json!([])),
            "status":if field(record,"canonical_rust_path")=="not_promoted" { "planned" } else { "implemented" },
        })).collect::<Vec<_>>()
    })
}

fn documentation_audit(records: &[Value]) -> Value {
    let reviewed = records
        .iter()
        .filter(|record| field(record, "reviewed_declaration_status").starts_with("reviewed_"))
        .collect::<Vec<_>>();
    let missing = reviewed
        .iter()
        .filter_map(|record| {
            let mut fields = Vec::new();
            if field(record, "documentation_status") != "complete_authored" {
                fields.push("routine_docs");
            }
            if field(record, "argument_documentation_status") != "complete_authored" {
                fields.push("argument_docs");
            }
            if !record
                .pointer("/documentation_evidence/safety")
                .and_then(Value::as_str)
                .map(|value| !value.is_empty())
                .unwrap_or(false)
            {
                fields.push("Safety");
            }
            if record
                .pointer("/documentation_evidence/source_link")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .is_empty()
            {
                fields.push("source_link");
            }
            if record
                .pointer("/documentation_evidence/abi_profile")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .is_empty()
            {
                fields.push("abi_profile");
            }
            (!fields.is_empty())
                .then(|| json!({"routine":field(record,"routine"),"missing":fields}))
        })
        .collect::<Vec<_>>();
    let review_queue = records
        .iter()
        .filter(|record| field(record, "documentation_status") != "complete_authored")
        .map(|record| {
            let missing = if field(record, "legacy_family_declaration_status") != "not_present" {
                vec![
                    "source_hash_requalification",
                    "argument_semantics",
                    "Safety",
                    "source_link",
                    "abi_profile_statement",
                ]
            } else {
                vec![
                    "routine_docs",
                    "argument_docs",
                    "Safety",
                    "source_link",
                    "abi_profile_statement",
                ]
            };
            json!({
                "routine":field(record,"routine"),
                "raw_api_state":field(record,"raw_api_state"),
                "declaration_status":field(record,"reviewed_declaration_status"),
                "missing":missing,
            })
        })
        .collect::<Vec<_>>();
    json!({
        "schema_id":"slatec-sys.raw-api.documentation-audit",
        "schema_version":"1.0.0",
        "public_extern_declarations":reviewed.len(),
        "routine_docs_present":reviewed.iter().filter(|record| field(record,"documentation_status")=="complete_authored").count(),
        "argument_docs_complete":reviewed.iter().filter(|record| field(record,"argument_documentation_status")=="complete_authored").count(),
        "safety_sections_present":reviewed.iter().filter(|record| record.pointer("/documentation_evidence/safety").and_then(Value::as_str).map(|value| !value.is_empty()).unwrap_or(false)).count(),
        "source_links_present":reviewed.iter().filter(|record| record.pointer("/documentation_evidence/source_link").and_then(Value::as_str).map(|value| !value.is_empty()).unwrap_or(false)).count(),
        "abi_profile_statements_present":reviewed.iter().filter(|record| record.pointer("/documentation_evidence/abi_profile").and_then(Value::as_str).map(|value| !value.is_empty()).unwrap_or(false)).count(),
        "missing":missing,
        "review_queue":review_queue,
        "review_queue_policy":"Non-reviewed declarations remain queued rather than receiving inferred pointer semantics."
    })
}

fn feature_map(
    records: &[Value],
    sys: &BTreeSet<String>,
    src: &BTreeSet<String>,
    facade: &BTreeSet<String>,
) -> Value {
    let reviewed = records
        .iter()
        .filter(|record| field(record, "reviewed_declaration_status").starts_with("reviewed_"))
        .collect::<Vec<_>>();
    let findings = reviewed
        .iter()
        .filter_map(|record| {
            let mut problems = Vec::new();
            if !sys.contains(&field(record, "feature")) {
                problems.push("missing_slatec_sys_feature");
            }
            if !src.contains(&field(record, "provider_feature")) {
                problems.push("missing_slatec_src_feature");
            }
            let facade_feature = field(record, "safe_facade_feature");
            if facade_feature != "not_assigned" && !facade.contains(&facade_feature) {
                problems.push("missing_slatec_feature");
            }
            (!problems.is_empty())
                .then(|| json!({"routine":field(record,"routine"),"problems":problems}))
        })
        .collect::<Vec<_>>();
    json!({
        "schema_id":"slatec-sys.raw-api.feature-provider-map",
        "schema_version":"1.0.0",
        "records":records.iter().map(|record| json!({
            "routine":field(record,"routine"),
            "raw_declaration_feature":field(record,"feature"),
            "native_provider_feature":field(record,"provider_feature"),
            "safe_facade_feature":field(record,"safe_facade_feature"),
            "aggregate_feature_membership":"recorded by the three feature names; no raw declaration enables provider compilation itself",
        })).collect::<Vec<_>>(),
        "reviewed_findings":findings,
        "status":if findings.is_empty() { "passed" } else { "failed" },
    })
}

fn exclusion_report(records: &[Value]) -> Value {
    let mut by_reason = BTreeMap::<String, usize>::new();
    for record in records
        .iter()
        .filter(|record| field(record, "public_raw_feasibility") == "explicitly_excluded")
    {
        *by_reason
            .entry(field(record, "exclusion_reason"))
            .or_default() += 1;
    }
    json!({
        "schema_id":"slatec-sys.raw-api.exclusion-report",
        "schema_version":"1.0.0",
        "by_reason":by_reason,
        "records":records.iter().filter(|record| field(record,"public_raw_feasibility")=="explicitly_excluded").map(|record| json!({"routine":field(record,"routine"),"state":field(record,"raw_api_state"),"reason":field(record,"exclusion_reason")})).collect::<Vec<_>>()
    })
}

fn promotion_priority(records: &[Value]) -> Value {
    let mut groups = BTreeMap::<String, Vec<&Value>>::new();
    for record in records {
        groups
            .entry(field(record, "intended_canonical_module"))
            .or_default()
            .push(record);
    }
    let mut priorities = groups.into_iter().map(|(module, records)| {
        let user = records.iter().filter(|record| field(record,"historical_role")=="user_callable").count();
        let generated = records.iter().filter(|record| field(record,"generated_declaration_status")=="generated_abi_validated").count();
        let callbacks = records.iter().filter(|record| field(record,"raw_api_state")=="unsupported_callback_abi").count();
        let documented = records.iter().filter(|record| field(record,"documentation_status")=="complete_authored").count();
        let wrapped = records.iter().filter(|record| field(record,"safe_wrapper_status")!="not_safely_wrapped").count();
        let score = (user * 3 + generated * 2 + documented * 8 + wrapped * 4).saturating_sub(callbacks * 5);
        json!({"module":module,"historically_user_callable":user,"generated_abi_validated":generated,"documentation_complete":documented,"safe_wrapper_audits":wrapped,"unresolved_callbacks":callbacks,"promotion_score":score})
    }).collect::<Vec<_>>();
    priorities.sort_by(|left, right| {
        right["promotion_score"]
            .as_u64()
            .cmp(&left["promotion_score"].as_u64())
            .then_with(|| left["module"].as_str().cmp(&right["module"].as_str()))
    });
    json!({"schema_id":"slatec-sys.raw-api.promotion-priority","schema_version":"1.0.0","ranking_inputs":["historical user-callable count","generated declaration availability","documentation completeness","safe-wrapper audits","unresolved callbacks"],"families":priorities})
}

fn roots_report(records: &[Value]) -> Value {
    let roots = records.iter().filter(|record| is_root(record)).map(|record| {
        json!({
            "routine":field(record,"routine"),
            "root_category":root_category(record),
            "historical_role":field(record,"historical_role"),
            "provider":field(record,"canonical_provider"),
            "symbol":field(record,"native_symbol"),
            "generated_declaration_status":field(record,"generated_declaration_status"),
            "reviewed_declaration_status":field(record,"reviewed_declaration_status"),
            "current_rust_path":current_rust_path(record),
            "intended_canonical_rust_path":planned_canonical_path(record),
            "provider_feature_status":field(record,"provider_feature"),
            "documentation_status":field(record,"documentation_status"),
            "test_status":{"compile":field(record,"compile_test_status"),"link":field(record,"link_test_status"),"runtime":field(record,"runtime_test_status")},
            "safe_wrapper_status":field(record,"safe_wrapper_status"),
            "exclusion_or_deferment_reason":field(record,"exclusion_reason"),
        })
    }).collect::<Vec<_>>();
    let categories = roots
        .iter()
        .fold(BTreeMap::<String, usize>::new(), |mut counts, record| {
            *counts.entry(field(record, "root_category")).or_default() += 1;
            counts
        });
    json!({"schema_id":"slatec-sys.raw-api.roots-family-report","schema_version":"1.0.0","scope":"FZERO/DFZERO, polynomial candidates, callback-bearing drivers, subsidiaries, and inherited root evidence","category_counts":categories,"records":roots})
}

fn planned_canonical_path(record: &Value) -> String {
    let path = field(record, "canonical_rust_path");
    if path != "not_promoted" {
        return path;
    }
    format!(
        "slatec_sys::{}::{}",
        field(record, "intended_canonical_module"),
        field(record, "routine").to_ascii_lowercase(),
    )
}

fn current_rust_path(record: &Value) -> String {
    if let Some(path) = record
        .get("legacy_rust_paths")
        .and_then(Value::as_array)
        .and_then(|paths| paths.first())
        .and_then(Value::as_str)
    {
        return path.to_owned();
    }
    if let Some(path) = record
        .get("compatibility_paths")
        .and_then(Value::as_array)
        .and_then(|paths| paths.first())
        .and_then(Value::as_str)
    {
        return path.to_owned();
    }
    let generated_module = match field(record, "feature").as_str() {
        "raw-ffi-numeric-scalar-subroutines" => "numeric_scalar_subroutines",
        "raw-ffi-numeric-array-subroutines" => "numeric_array_subroutines",
        "raw-ffi-scalar-functions" => "scalar_functions",
        "raw-ffi-complex-arguments" => "complex_arguments",
        "raw-ffi-logical" => "logical",
        "raw-ffi-character" => "character",
        _ => return field(record, "canonical_rust_path"),
    };
    format!(
        "slatec_sys::generated::{generated_module}::{}",
        field(record, "routine").to_ascii_lowercase(),
    )
}

fn is_root(record: &Value) -> bool {
    let name = field(record, "routine");
    matches!(name.as_str(), "FZERO" | "DFZERO" | "CPZERO" | "RPZERO")
        || field(record, "primary_family")
            .to_ascii_lowercase()
            .contains("root")
        || field(record, "intended_canonical_module").starts_with("roots")
}

fn root_category(record: &Value) -> &'static str {
    let name = field(record, "routine");
    if matches!(name.as_str(), "FZERO" | "DFZERO") {
        "FZERO / DFZERO"
    } else if matches!(name.as_str(), "CPZERO" | "RPZERO") {
        "polynomial-root routine"
    } else if field(record, "driver_role") == "internal_subsidiary" {
        "subsidiary"
    } else if field(record, "raw_api_state") == "unsupported_callback_abi" {
        "callback-bearing driver"
    } else {
        "inherited-family evidence"
    }
}

fn validation_markdown(coverage: &Value, audit: &Value, features: &Value, roots: &Value) -> String {
    let count = |key: &str| {
        coverage
            .pointer(&format!("/counts/{key}"))
            .and_then(Value::as_u64)
            .unwrap_or_default()
    };
    format!(
        "# SLATEC-SYS raw API validation\n\n\
         This report is generated from exactly one status record per retained catalogue identity. A generated declaration is not a reviewed declaration.\n\n\
         | Metric | Count |\n| --- | ---: |\n\
         | Retained identities | {} |\n\
         | Generated ABI-validated declarations | {} |\n\
         | Reviewed public raw declarations | {} |\n\
         | Provider-backed callable raw routines | {} |\n\
         | Link-tested raw routines | {} |\n\
         | Runtime-tested raw routines | {} |\n\
         | Fully documented raw routines | {} |\n\
         | Explicit exclusions | {} |\n\n\
         Documentation audit: {} reviewed declarations, {} reviewed records missing fields, {} queued declarations. Feature/provider reconciliation: `{}`. Roots report entries: {}.\n\n\
         The `slatec_sys::generated` namespace remains transitional, ABI-shaped generated access and is not a stable public namespace.\n",
        count("retained_identities"),
        count("abi_validated_generated_declarations"),
        count("reviewed_family_raw_declarations"),
        count("provider_backed_callable_raw_routines"),
        count("link_tested_raw_routines"),
        count("runtime_tested_raw_routines"),
        count("fully_documented_raw_routines"),
        count("explicitly_excluded_routines"),
        audit["public_extern_declarations"]
            .as_u64()
            .unwrap_or_default(),
        audit["missing"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        audit["review_queue"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        features["status"].as_str().unwrap_or("failed"),
        roots["records"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
    )
}

fn table(value: &Value, source: &str) -> Result<Vec<BTreeMap<String, Value>>> {
    let columns = value
        .get("columns")
        .and_then(Value::as_array)
        .ok_or_else(|| policy(&format!("{source} has no columns")))?;
    let names = columns
        .iter()
        .map(|column| {
            column
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| policy(&format!("{source} has non-string column")))
        })
        .collect::<Result<Vec<_>>>()?;
    let rows = records(value, source)?;
    rows.iter()
        .map(|row| {
            let cells = row
                .as_array()
                .or_else(|| row.get("value").and_then(Value::as_array))
                .ok_or_else(|| policy(&format!("{source} has malformed row")))?;
            if cells.len() != names.len() {
                return Err(policy(&format!("{source} has row with wrong width")));
            }
            Ok(names.iter().cloned().zip(cells.iter().cloned()).collect())
        })
        .collect()
}

fn records<'a>(value: &'a Value, source: &str) -> Result<&'a Vec<Value>> {
    value
        .get("records")
        .and_then(Value::as_array)
        .ok_or_else(|| policy(&format!("{source} has no records")))
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn string(value: &Value, key: &str) -> Result<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| policy(&format!("missing string {key}")))
}
fn field(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}
fn nonempty_or(value: String, fallback: &str) -> String {
    if value.is_empty() {
        fallback.to_owned()
    } else {
        value
    }
}
fn strings(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}
fn optional(row: &BTreeMap<String, Value>, key: &str) -> String {
    row.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}
fn required(row: &BTreeMap<String, Value>, key: &str, path: &Path) -> Result<String> {
    row.get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| policy(&format!("{} lacks {key}", path.display())))
}
fn json_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn outputs_hash(files: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut bytes = Vec::new();
    for (name, contents) in files {
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(contents);
    }
    hash::bytes(&bytes)
}
fn write_outputs(root: &Path, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    fs::create_dir_all(root)?;
    for (name, contents) in files {
        let path = root.join(name);
        if fs::read(&path).ok().as_deref() != Some(contents.as_slice()) {
            fs::write(path, contents)?;
        }
    }
    Ok(())
}

const ROUTINE_STATUS_START: &str = "<!-- raw-api-status:start -->";
const ROUTINE_STATUS_END: &str = "<!-- raw-api-status:end -->";
const RAW_COVERAGE_START: &str = "<!-- raw-api-coverage:start -->";
const RAW_COVERAGE_END: &str = "<!-- raw-api-coverage:end -->";

fn write_raw_coverage_page(docs_dir: &Path, coverage: &Value) -> Result<()> {
    let path = docs_dir.join("reference").join("routine-coverage.md");
    let original = fs::read_to_string(&path)?;
    let count = |key: &str| {
        coverage
            .pointer(&format!("/counts/{key}"))
            .and_then(Value::as_u64)
            .unwrap_or_default()
    };
    let block = format!(
        "{RAW_COVERAGE_START}\n\
         ## Canonical raw API inventory\n\n\
         The former aggregate is retired because it conflated generated declarations, reviewed declarations, and validation evidence. The [authoritative status records](../../generated/raw-api/routine-status.json) make every metric reproducible.\n\n\
         | Metric | Count |\n\
         | --- | ---: |\n\
         | Retained identities | {} |\n\
         | Historically user-callable routines | {} |\n\
         | Generated raw declaration candidates | {} |\n\
         | ABI-validated generated declarations | {} |\n\
         | Reviewed family raw declarations | {} |\n\
         | Reviewed user-callable raw drivers | {} |\n\
         | Reviewed public subsidiaries | {} |\n\
         | Provider-backed callable raw routines | {} |\n\
         | Link-tested raw routines | {} |\n\
         | Runtime-tested raw routines | {} |\n\
         | Fully documented raw routines | {} |\n\
         | Pre-existing family declarations pending R1 review | {} |\n\
         | Safely wrapped routines | {} |\n\
         | Explicitly excluded routines | {} |\n\
         | Unclassified routines | {} |\n\n\
         The definitions and exclusions are generated in [coverage-summary.json](../../generated/raw-api/coverage-summary.json) and [exclusion-report.json](../../generated/raw-api/exclusion-report.json).\n\
         {RAW_COVERAGE_END}",
        count("retained_identities"),
        count("historically_user_callable_routines"),
        count("generated_raw_declaration_candidates"),
        count("abi_validated_generated_declarations"),
        count("reviewed_family_raw_declarations"),
        count("reviewed_user_callable_raw_drivers"),
        count("reviewed_public_subsidiaries"),
        count("provider_backed_callable_raw_routines"),
        count("link_tested_raw_routines"),
        count("runtime_tested_raw_routines"),
        count("fully_documented_raw_routines"),
        count("preexisting_family_declarations_pending_r1_review"),
        count("safely_wrapped_routines"),
        count("explicitly_excluded_routines"),
        count("unclassified_routines"),
    );
    let updated = if let Some(start) = original.find(RAW_COVERAGE_START) {
        let end = original[start..]
            .find(RAW_COVERAGE_END)
            .map(|offset| start + offset + RAW_COVERAGE_END.len())
            .ok_or_else(|| policy("routine coverage page has unclosed raw API inventory"))?;
        format!("{}{}{}", &original[..start], block, &original[end..])
    } else {
        let separator = if original.ends_with('\n') {
            "\n"
        } else {
            "\n\n"
        };
        format!("{original}{separator}{block}\n")
    };
    if updated != original {
        fs::write(path, updated)?;
    }
    Ok(())
}

fn write_routine_page_statuses(docs_dir: &Path, records: &[Value]) -> Result<()> {
    let pages = docs_dir.join("reference").join("routines");
    for record in records {
        let routine = field(record, "routine");
        let path = pages.join(format!("{}.md", routine.to_ascii_lowercase()));
        if !path.is_file() {
            return Err(policy(&format!(
                "raw-api status has no generated routine page for {routine}"
            )));
        }
        let original = fs::read_to_string(&path)?;
        let block = routine_page_status(record);
        let updated = if let Some(start) = original.find(ROUTINE_STATUS_START) {
            let end = original[start..]
                .find(ROUTINE_STATUS_END)
                .map(|offset| start + offset + ROUTINE_STATUS_END.len())
                .ok_or_else(|| {
                    policy(&format!("routine page {routine} has unclosed raw status"))
                })?;
            format!("{}{}{}", &original[..start], block, &original[end..])
        } else {
            let separator = if original.ends_with('\n') {
                "\n"
            } else {
                "\n\n"
            };
            format!("{original}{separator}{block}\n")
        };
        if updated != original {
            fs::write(path, updated)?;
        }
    }
    Ok(())
}

fn routine_page_status(record: &Value) -> String {
    let provider_backed = if field(record, "symbol_status") == "observed_exactly_once" {
        "yes"
    } else {
        "no"
    };
    format!(
        "{ROUTINE_STATUS_START}\n\
         ## Raw Rust API status\n\n\
         This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).\n\n\
         - Generated raw declaration: `{}`\n\
         - Reviewed family declaration: `{}`\n\
         - Canonical Rust path: `{}`\n\
         - Current legacy Rust paths: `{}`\n\
         - Provider-backed callable symbol: `{provider_backed}` (`{}`)\n\
         - Documentation status: `{}`\n\
         - Link-test status: `{}`\n\
         - Runtime-test status: `{}`\n\
         - Safe-wrapper status: `{}`\n\
         - Exclusion or deferment reason: `{}`\n\
         {ROUTINE_STATUS_END}",
        field(record, "generated_declaration_status"),
        field(record, "reviewed_declaration_status"),
        field(record, "canonical_rust_path"),
        record
            .get("legacy_rust_paths")
            .and_then(Value::as_array)
            .map(|paths| paths
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(", "))
            .filter(|paths| !paths.is_empty())
            .unwrap_or_else(|| "none".to_owned()),
        field(record, "symbol_status"),
        field(record, "documentation_status"),
        field(record, "link_test_status"),
        field(record, "runtime_test_status"),
        field(record, "safe_wrapper_status"),
        field(record, "exclusion_reason"),
    )
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taxonomy_keeps_the_roots_pilot_out_of_nonlinear() {
        assert_eq!(
            taxonomy(&json!({"primary_family":"Nonlinear equations"}), "FZERO"),
            "roots::scalar"
        );
    }

    #[test]
    fn generated_batches_never_imply_review() {
        let interface = Interface {
            program_unit_id: "id".to_owned(),
            name: "X".to_owned(),
            kind: "subroutine".to_owned(),
            source_hash: "hash".to_owned(),
            native_symbol: "x_".to_owned(),
            confidence: "generated_standard".to_owned(),
            batch: "batch_numeric_scalar_subroutines".to_owned(),
            review_state: "machine_checked".to_owned(),
            argument_ids: Vec::new(),
        };
        assert_eq!(
            generated_status(Some(&interface)),
            "generated_abi_validated"
        );
        assert_ne!(generated_status(Some(&interface)), "reviewed_public_driver");
    }
}
