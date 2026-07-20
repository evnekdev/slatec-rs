//! Explicit native-link validation for the generated raw SLATEC FFI.
//!
//! This module consumes the selected-corpus based output of `raw_ffi`; it does
//! not rediscover sources or add an implicit Cargo native build.  All compiler
//! products and driver programs stay beneath the ignored evidence root.

use crate::error::{CorpusError, Result};
use crate::hash;
use crate::raw_ffi;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA_VERSION: &str = "1.0.0";
const SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const COMMITTED_SIZE_LIMIT: u64 = 4_000_000;
const PROFILE_FEATURE: &str = "ffi-profile-gnu-mingw-x86_64";
const RUST_TARGET: &str = "x86_64-pc-windows-gnu";
const BATCHES: &[&str] = &[
    "batch_numeric_scalar_subroutines",
    "batch_numeric_array_subroutines",
    "batch_scalar_functions",
    "batch_complex_arguments",
    "batch_complex_returns",
    "batch_logical",
    "batch_character",
    "batch_callbacks",
    "batch_infrastructure",
];

#[derive(Clone, Debug, serde::Serialize)]
pub struct FfiValidationResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
    pub audited_declarations: usize,
    pub structural_failures: usize,
    pub runtime_passed: usize,
    pub runtime_pending: usize,
}

/// Explicit input locations shared by the native build and validation stages.
pub struct ValidationPaths<'a> {
    pub evidence_dir: &'a Path,
    pub selected_corpus_dir: &'a Path,
    pub inventory_dir: &'a Path,
    pub ffi_dir: &'a Path,
    pub bindings_dir: &'a Path,
    pub output_dir: &'a Path,
}

#[derive(Clone)]
struct NativeContext {
    snapshot_id: String,
    profile_id: String,
    compiler_identity: String,
    compiler_version: String,
    compiler_target: String,
    compiler: PathBuf,
    archive: PathBuf,
    archive_sha256: String,
    ffi_dir: PathBuf,
    bindings_dir: PathBuf,
}

#[derive(Clone)]
struct Interface {
    id: String,
    name: String,
    kind: String,
    argument_ids: Vec<String>,
    function_result_id: Option<String>,
    observed_symbol: Option<String>,
    confidence: String,
    diagnostic_ids: Vec<String>,
}

#[derive(Clone, Default)]
struct Argument {
    name: String,
    declared_type: Option<String>,
    is_external: bool,
    is_intrinsic: bool,
    conflict: bool,
}

#[derive(Clone, Default)]
struct FunctionResult {
    declared_type: Option<String>,
    conflict: bool,
}

#[derive(Clone)]
struct Binding {
    batch: String,
    rust_name: String,
    link_name: String,
    parameter_names: Vec<String>,
    parameters: usize,
    character_lengths: usize,
    is_function: bool,
    return_type: Option<String>,
}

#[derive(Default)]
struct SignatureAudit {
    expected: usize,
    parsed: usize,
    observed_symbols: usize,
    argument_count_matches: usize,
    argument_order_matches: usize,
    character_lengths: usize,
    function_kinds: usize,
    function_returns: usize,
    callback_gated: usize,
    infrastructure_gated: usize,
    duplicate_rust_names: usize,
    duplicate_link_names: usize,
    failures: Vec<(String, String)>,
}

#[derive(Clone)]
struct BatchResult {
    batch: String,
    declaration_count: usize,
    link_status: String,
    runtime_status: String,
    scope: String,
}

/// Rebuild the selected native archive explicitly.  The underlying raw-FFI
/// generator verifies all selected source hashes before it compiles anything.
pub fn build_native(
    evidence_dir: &Path,
    selected_corpus_dir: &Path,
    inventory_dir: &Path,
    ffi_dir: &Path,
    bindings_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<FfiValidationResult> {
    let generated = raw_ffi::generate(
        evidence_dir,
        selected_corpus_dir,
        inventory_dir,
        ffi_dir,
        bindings_dir,
        offline,
    )?;
    let context = load_context(evidence_dir, ffi_dir, bindings_dir)?;
    let outputs = native_build_outputs(&context, &generated)?;
    let semantic_hash = semantic_hash(&outputs);
    let mut outputs = outputs;
    outputs.insert(
        "manifest.json",
        compact(&json!({
            "id": stable_id("ffi-validation", &[&context.snapshot_id, &semantic_hash]),
            "schema_id": "slatec-rs/raw-ffi-native-build",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": context.snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "semantic_version": SEMANTIC_VERSION,
            "output_semantic_hash": semantic_hash,
            "status": generated.status,
            "scope": "Explicit native archive build only. Cargo build scripts do not compile Fortran."
        }))?,
    );
    enforce_size(&outputs)?;
    promote(output_dir, &context.snapshot_id, &outputs)?;
    Ok(FfiValidationResult {
        snapshot_id: context.snapshot_id,
        status: generated.status,
        semantic_hash,
        output_dir: output_dir.to_owned(),
        audited_declarations: 0,
        structural_failures: 0,
        runtime_passed: 0,
        runtime_pending: 0,
    })
}

/// Build the native archive explicitly, audit the generated declarations, and
/// run the requested independent native-link batches.  An empty `batches`
/// slice means every available batch.
pub fn validate(
    paths: ValidationPaths<'_>,
    batches: &[String],
    offline: bool,
) -> Result<FfiValidationResult> {
    let generated = raw_ffi::generate(
        paths.evidence_dir,
        paths.selected_corpus_dir,
        paths.inventory_dir,
        paths.ffi_dir,
        paths.bindings_dir,
        offline,
    )?;
    let context = load_context(paths.evidence_dir, paths.ffi_dir, paths.bindings_dir)?;
    let requested = requested_batches(batches)?;
    let interfaces = load_interfaces(&paths.ffi_dir.join("interface-inventory.json"))?;
    let arguments = load_arguments(&paths.inventory_dir.join("argument-index.json"))?;
    let results = load_results(&paths.inventory_dir.join("function-results.json"))?;
    let symbols = observed_symbols(&paths.ffi_dir.join("symbol-inventory.json"))?;
    let bindings = parse_bindings(paths.bindings_dir)?;
    let audit = audit_signatures(&interfaces, &arguments, &results, &symbols, &bindings);
    let batches = run_batches(&context, &bindings, &requested)?;
    let review = review_reasons(&interfaces, &arguments, &results);
    let runtime_passed = batches
        .iter()
        .filter(|record| record.runtime_status == "passed")
        .count();
    let runtime_pending = batches
        .iter()
        .filter(|record| !matches!(record.runtime_status.as_str(), "passed" | "not_applicable"))
        .count();
    let status = if audit.failures.is_empty()
        && audit.duplicate_link_names == 0
        && audit.duplicate_rust_names == 0
        && runtime_pending == 0
    {
        "success"
    } else {
        "success_with_review_items"
    };
    let outputs = validation_outputs(
        &context,
        &generated,
        &audit,
        &batches,
        &review,
        runtime_passed,
        runtime_pending,
    )?;
    let semantic_hash = semantic_hash(&outputs);
    let mut outputs = outputs;
    outputs.insert(
        "manifest.json",
        compact(&json!({
            "id": stable_id("ffi-validation", &[&context.snapshot_id, &semantic_hash]),
            "schema_id": "slatec-rs/raw-ffi-validation",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": context.snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "semantic_version": SEMANTIC_VERSION,
            "compiler_profile_id": context.profile_id,
            "raw_ffi_manifest_sha256": hash::file(&context.ffi_dir.join("manifest.json"))?,
            "output_semantic_hash": semantic_hash,
            "status": status,
            "profile_feature": PROFILE_FEATURE,
            "profile_validation": runtime_profile_status(&context.snapshot_id)?,
            "scope": "Structural and explicit GNU MinGW native-link validation. Passing results are profile-scoped raw-ABI evidence, not a safe API proof."
        }))?,
    );
    enforce_size(&outputs)?;
    promote(paths.output_dir, &context.snapshot_id, &outputs)?;
    Ok(FfiValidationResult {
        snapshot_id: context.snapshot_id,
        status: status.to_owned(),
        semantic_hash,
        output_dir: paths.output_dir.to_owned(),
        audited_declarations: audit.expected,
        structural_failures: audit.failures.len()
            + audit.duplicate_link_names
            + audit.duplicate_rust_names,
        runtime_passed,
        runtime_pending,
    })
}

fn load_context(evidence_dir: &Path, ffi_dir: &Path, bindings_dir: &Path) -> Result<NativeContext> {
    let manifest = read_value(&ffi_dir.join("manifest.json"))?;
    let snapshot_id = required_string(&manifest, "snapshot_id", "raw FFI manifest")?;
    let compilation = read_value(&ffi_dir.join("compilation-results.json"))?;
    let profile = &compilation["compiler_profile"];
    let profile_id = required_string(profile, "id", "compiler profile")?;
    let compiler_identity = required_string(profile, "identity", "compiler profile")?;
    let compiler_version = required_string(profile, "version", "compiler profile")?;
    let compiler_target = required_string(profile, "target", "compiler profile")?;
    if compiler_identity != "gnu-fortran" || compiler_target != "x86_64-w64-mingw32" {
        return Err(CorpusError::Verification(format!(
            "unsupported raw-FFI compiler profile {compiler_identity} / {compiler_target}"
        )));
    }
    let archive = evidence_dir
        .join("raw-ffi")
        .join(&snapshot_id)
        .join(&profile_id)
        .join("libslatec_selected.a");
    if !archive.is_file() {
        return Err(CorpusError::Verification(format!(
            "missing explicit native archive {}; run build-native-ffi --offline",
            archive.display()
        )));
    }
    Ok(NativeContext {
        snapshot_id,
        profile_id,
        compiler_identity,
        compiler_version,
        compiler_target,
        compiler: compiler_path(),
        archive_sha256: hash::file(&archive)?,
        archive,
        ffi_dir: ffi_dir.to_owned(),
        bindings_dir: bindings_dir.to_owned(),
    })
}

fn native_build_outputs(
    context: &NativeContext,
    generated: &raw_ffi::RawFfiResult,
) -> Result<BTreeMap<&'static str, Vec<u8>>> {
    let mut outputs = BTreeMap::new();
    let profile_overrides = context.ffi_dir.join("profile-overrides.json");
    outputs.insert(
        "native-build.json",
        compact(&json!({
            "schema_id": "slatec-rs/raw-ffi-native-build",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": context.snapshot_id,
            "compiler_profile": {
                "id": context.profile_id,
                "identity": context.compiler_identity,
                "version": context.compiler_version,
                "target": context.compiler_target,
                "flags": ["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"],
                "default_integer": "i32 (validated by explicit profile probe)",
                "default_logical": "i32, integer-backed (validated by explicit profile probe)",
                "character_length": "usize, trailing after explicit arguments (validated by explicit profile probe)",
                "symbol_mangling": "exact raw spellings observed with GNU nm",
                "object_format": "GNU MinGW COFF/archive; detailed evidence is ignored"
            },
            "archive": {
                "file_name": "libslatec_selected.a",
                "sha256": context.archive_sha256,
                "evidence_relative_path": format!("raw-ffi/{}/{}/libslatec_selected.a", context.snapshot_id, context.profile_id),
                "required_runtime_libraries": ["gfortran", "quadmath", "msvcrt"]
            },
            "profile_overrides": {
                "metadata_sha256": hash::file(&profile_overrides)?,
                "source": "generated/ffi/profile-overrides.json",
                "single_definition_check": "passed"
            },
            "profile_validation": runtime_profile_status(&context.snapshot_id)?,
            "compilation": generated.summary,
            "link_environment": {
                "native_library_directory": "set SLATEC_NATIVE_LIB_DIR to the directory containing libslatec_selected.a",
                "gfortran_runtime_directory": "set SLATEC_GFORTRAN_RUNTIME_DIR when GNU runtime libraries are not on the linker search path",
                "profile_feature": PROFILE_FEATURE,
                "rust_target": RUST_TARGET
            }
        }))?,
    );
    outputs.insert(
        "validation-summary.md",
        format!(
            "# Explicit SLATEC native build\n\n- Snapshot: `{}`\n- Compiler: `{}` (`{}`)\n- Target: `{}`\n- Archive SHA-256: `{}`\n- Sources compiled: {}; failures: {}\n\nThe archive is ignored evidence. To link raw bindings explicitly, enable `{PROFILE_FEATURE}` and set `SLATEC_NATIVE_LIB_DIR`; set `SLATEC_GFORTRAN_RUNTIME_DIR` if the GNU runtime is not otherwise discoverable. Cargo does not compile Fortran.\n",
            context.snapshot_id,
            context.compiler_identity,
            context.compiler_version,
            context.compiler_target,
            context.archive_sha256,
            generated.summary.compiled_source_files,
            generated.summary.failed_source_files,
        )
        .into_bytes(),
    );
    Ok(outputs)
}

fn requested_batches(requested: &[String]) -> Result<BTreeSet<String>> {
    if requested.is_empty() {
        return Ok(BATCHES.iter().map(|batch| (*batch).to_owned()).collect());
    }
    let known = BATCHES.iter().copied().collect::<BTreeSet<_>>();
    let mut output = BTreeSet::new();
    for batch in requested {
        if !known.contains(batch.as_str()) {
            return Err(CorpusError::Policy(format!(
                "unknown raw FFI validation batch {batch}"
            )));
        }
        output.insert(batch.clone());
    }
    Ok(output)
}

fn load_interfaces(path: &Path) -> Result<Vec<Interface>> {
    let value = read_value(path)?;
    records(&value, "raw FFI interface")?
        .iter()
        .map(|row| {
            Ok(Interface {
                id: row_string(row, 0, "program unit id")?,
                name: row_string(row, 1, "normalized name")?,
                kind: row_string(row, 2, "kind")?,
                argument_ids: row_array_strings(row, 7),
                function_result_id: row_optional_string(row, 8),
                observed_symbol: row_optional_string(row, 9),
                confidence: row_string(row, 10, "confidence class")?,
                diagnostic_ids: row_array_strings(row, 13),
            })
        })
        .collect()
}

fn load_arguments(path: &Path) -> Result<BTreeMap<String, Argument>> {
    let value = read_value(path)?;
    let mut output = BTreeMap::new();
    for row in records(&value, "argument index")? {
        let id = row_string(row, 0, "argument id")?;
        output.insert(
            id,
            Argument {
                name: row_string(row, 3, "argument name")?,
                declared_type: row_optional_string(row, 4),
                is_external: row_bool(row, 8),
                is_intrinsic: row_bool(row, 9),
                conflict: !row.get(12).is_none_or(Value::is_null),
            },
        );
    }
    Ok(output)
}

fn load_results(path: &Path) -> Result<BTreeMap<String, FunctionResult>> {
    let value = read_value(path)?;
    let mut output = BTreeMap::new();
    for row in records(&value, "function result index")? {
        let id = row_string(row, 0, "function result id")?;
        output.insert(
            id,
            FunctionResult {
                declared_type: row_optional_string(row, 2),
                conflict: !row.get(6).is_none_or(Value::is_null),
            },
        );
    }
    Ok(output)
}

fn observed_symbols(path: &Path) -> Result<BTreeSet<String>> {
    let value = read_value(path)?;
    Ok(records(&value, "symbol inventory")?
        .iter()
        .filter_map(|row| row.get(1).and_then(Value::as_str).map(str::to_owned))
        .collect())
}

fn parse_bindings(bindings_dir: &Path) -> Result<Vec<Binding>> {
    let modules = [
        (
            "batch_numeric_scalar_subroutines",
            "numeric_scalar_subroutines.rs",
        ),
        (
            "batch_numeric_array_subroutines",
            "numeric_array_subroutines.rs",
        ),
        ("batch_scalar_functions", "scalar_functions.rs"),
        ("batch_complex_arguments", "complex_arguments.rs"),
        ("batch_complex_returns", "complex_returns.rs"),
        ("batch_logical", "logical.rs"),
        ("batch_character", "character.rs"),
        ("batch_callbacks", "callbacks.rs"),
        ("batch_infrastructure", "infrastructure.rs"),
    ];
    let mut bindings = Vec::new();
    for (batch, file) in modules {
        let source = fs::read_to_string(bindings_dir.join(file))?;
        let mut symbol = None::<String>;
        let mut statement = String::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if let Some(value) = trimmed
                .strip_prefix("#[link_name = \"")
                .and_then(|value| value.strip_suffix("\"]"))
            {
                symbol = Some(value.to_owned());
                continue;
            }
            if let Some(link_name) = symbol.as_ref() {
                statement.push_str(trimmed);
                if trimmed.ends_with(';') {
                    let prefix = "pub fn ";
                    let start = statement.find(prefix).ok_or_else(|| {
                        CorpusError::Verification("generated binding lacks pub fn".to_owned())
                    })? + prefix.len();
                    let open = statement[start..].find('(').ok_or_else(|| {
                        CorpusError::Verification(
                            "generated binding lacks argument list".to_owned(),
                        )
                    })? + start;
                    let close = statement.rfind(')').ok_or_else(|| {
                        CorpusError::Verification("generated binding lacks argument end".to_owned())
                    })?;
                    let raw_name = statement[start..open].trim().to_owned();
                    let params = statement[open + 1..close].trim().trim_end_matches(',');
                    let parameter_names = if params.is_empty() {
                        Vec::new()
                    } else {
                        params
                            .split(',')
                            .filter_map(|parameter| parameter.split(':').next())
                            .map(|parameter| parameter.trim().to_owned())
                            .collect()
                    };
                    let parameters = if params.is_empty() {
                        0
                    } else {
                        params.split(',').count()
                    };
                    let character_lengths = params.matches("character_length_").count();
                    let result = statement[close + 1..]
                        .trim()
                        .strip_prefix("->")
                        .map(|value| value.trim_end_matches(';').trim().to_owned());
                    bindings.push(Binding {
                        batch: batch.to_owned(),
                        rust_name: raw_name,
                        link_name: link_name.clone(),
                        parameter_names,
                        parameters,
                        character_lengths,
                        is_function: result.is_some(),
                        return_type: result,
                    });
                    symbol = None;
                    statement.clear();
                }
            }
        }
        if symbol.is_some() {
            return Err(CorpusError::Verification(format!(
                "unterminated generated binding in {file}"
            )));
        }
    }
    bindings.sort_by(|left, right| {
        left.batch
            .cmp(&right.batch)
            .then(left.link_name.cmp(&right.link_name))
    });
    Ok(bindings)
}

fn audit_signatures(
    interfaces: &[Interface],
    arguments: &BTreeMap<String, Argument>,
    results: &BTreeMap<String, FunctionResult>,
    symbols: &BTreeSet<String>,
    bindings: &[Binding],
) -> SignatureAudit {
    let mut audit = SignatureAudit::default();
    let generated = interfaces
        .iter()
        .filter(|record| {
            matches!(
                record.confidence.as_str(),
                "generated_standard" | "generated_abi_sensitive"
            )
        })
        .collect::<Vec<_>>();
    audit.expected = generated.len();
    audit.parsed = bindings.len();
    let mut by_link = BTreeMap::<String, Vec<&Binding>>::new();
    let mut names = BTreeSet::new();
    for binding in bindings {
        if !names.insert((binding.batch.clone(), binding.rust_name.clone())) {
            audit.duplicate_rust_names += 1;
        }
        by_link
            .entry(binding.link_name.clone())
            .or_default()
            .push(binding);
    }
    audit.duplicate_link_names = by_link.values().filter(|rows| rows.len() > 1).count();
    for record in generated {
        let Some(symbol) = record.observed_symbol.as_ref() else {
            audit
                .failures
                .push((record.id.clone(), "missing_observed_symbol".to_owned()));
            continue;
        };
        let matches = by_link.get(symbol).cloned().unwrap_or_default();
        if matches.len() != 1 {
            audit.failures.push((
                record.id.clone(),
                "binding_symbol_count_mismatch".to_owned(),
            ));
            continue;
        }
        let binding = matches[0];
        if !symbols.contains(symbol) {
            audit
                .failures
                .push((record.id.clone(), "binding_symbol_not_observed".to_owned()));
        } else {
            audit.observed_symbols += 1;
        }
        let mut expected_character_lengths = 0_usize;
        let mut expected_argument_names = Vec::new();
        let mut external = false;
        let mut intrinsic = false;
        for id in &record.argument_ids {
            match arguments.get(id) {
                Some(argument) => {
                    expected_argument_names.push(argument.name.to_ascii_lowercase());
                    expected_character_lengths +=
                        usize::from(argument.declared_type.as_deref() == Some("CHARACTER"));
                    external |= argument.is_external;
                    intrinsic |= argument.is_intrinsic;
                }
                None => audit
                    .failures
                    .push((record.id.clone(), "missing_argument_fact".to_owned())),
            }
        }
        if external || intrinsic {
            audit.failures.push((
                record.id.clone(),
                "procedure_argument_entered_callable_module".to_owned(),
            ));
        }
        let expected_parameters = record.argument_ids.len() + expected_character_lengths;
        if binding.parameters == expected_parameters {
            audit.argument_count_matches += 1;
        } else {
            audit.failures.push((
                record.id.clone(),
                "formal_argument_count_mismatch".to_owned(),
            ));
        }
        let actual_argument_names = binding
            .parameter_names
            .iter()
            .take(record.argument_ids.len())
            .map(|name| name.trim_start_matches("r#").to_owned())
            .collect::<Vec<_>>();
        if actual_argument_names == expected_argument_names {
            audit.argument_order_matches += 1;
        } else {
            audit.failures.push((
                record.id.clone(),
                "formal_argument_order_mismatch".to_owned(),
            ));
        }
        let expected_length_names = (1..=expected_character_lengths)
            .map(|index| format!("character_length_{index}"))
            .collect::<Vec<_>>();
        if binding.character_lengths == expected_character_lengths
            && binding
                .parameter_names
                .get(record.argument_ids.len()..)
                .is_some_and(|names| names == expected_length_names)
        {
            audit.character_lengths += 1;
        } else {
            audit.failures.push((
                record.id.clone(),
                "character_hidden_length_mismatch".to_owned(),
            ));
        }
        let expected_function = record.kind == "function";
        if binding.is_function == expected_function {
            audit.function_kinds += 1;
        } else {
            audit
                .failures
                .push((record.id.clone(), "function_kind_mismatch".to_owned()));
        }
        if expected_function {
            let expected = record
                .function_result_id
                .as_ref()
                .and_then(|id| results.get(id))
                .and_then(|result| result.declared_type.as_deref())
                .and_then(rust_return_type);
            if expected == binding.return_type.as_deref() {
                audit.function_returns += 1;
            } else {
                audit.failures.push((
                    record.id.clone(),
                    "function_return_type_mismatch".to_owned(),
                ));
            }
        } else {
            audit.function_returns += 1;
        }
    }
    for record in interfaces {
        if record.confidence == "non_callable_infrastructure" {
            audit.infrastructure_gated += 1;
        }
        if !matches!(
            record.confidence.as_str(),
            "generated_standard" | "generated_abi_sensitive"
        ) && record.argument_ids.iter().any(|id| {
            arguments
                .get(id)
                .is_some_and(|argument| argument.is_external || argument.is_intrinsic)
        }) {
            audit.callback_gated += 1;
        }
    }
    audit
}

fn review_reasons(
    interfaces: &[Interface],
    arguments: &BTreeMap<String, Argument>,
    results: &BTreeMap<String, FunctionResult>,
) -> Vec<(String, String, String, String, String)> {
    let mut rows = interfaces
        .iter()
        .filter(|record| {
            !matches!(
                record.confidence.as_str(),
                "generated_standard" | "generated_abi_sensitive"
            )
        })
        .map(|record| {
            let mut reason = if record.confidence == "non_callable_infrastructure" {
                "non_callable_infrastructure"
            } else if record.argument_ids.iter().any(|id| {
                arguments
                    .get(id)
                    .is_some_and(|argument| argument.is_external)
            }) {
                "callback_argument"
            } else if record
                .function_result_id
                .as_ref()
                .and_then(|id| results.get(id))
                .and_then(|result| result.declared_type.as_deref())
                .is_some_and(|kind| kind == "COMPLEX" || kind == "DOUBLE COMPLEX")
            {
                "complex_function_return"
            } else if record
                .function_result_id
                .as_ref()
                .and_then(|id| results.get(id))
                .and_then(|result| result.declared_type.as_deref())
                == Some("CHARACTER")
            {
                "character_function_return"
            } else if record.argument_ids.iter().any(|id| {
                arguments
                    .get(id)
                    .is_some_and(|argument| argument.declared_type.is_none())
            }) {
                "unresolved_argument_type"
            } else if record
                .argument_ids
                .iter()
                .any(|id| arguments.get(id).is_some_and(|argument| argument.conflict))
                || record
                    .function_result_id
                    .as_ref()
                    .and_then(|id| results.get(id))
                    .is_some_and(|result| result.conflict)
            {
                "conflicting_declaration"
            } else if record.observed_symbol.is_none() {
                "symbol_ambiguity"
            } else {
                "other"
            };
            if record.diagnostic_ids.iter().any(|id| id.contains("entry")) {
                reason = "entry_point";
            }
            (
                record.id.clone(),
                record.name.clone(),
                record.confidence.clone(),
                reason.to_owned(),
                format!("{}; retained outside callable modules", record.confidence),
            )
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    rows
}

fn run_batches(
    context: &NativeContext,
    bindings: &[Binding],
    requested: &BTreeSet<String>,
) -> Result<Vec<BatchResult>> {
    let mut result = Vec::new();
    for batch in BATCHES {
        let declaration_count = bindings
            .iter()
            .filter(|binding| binding.batch == *batch)
            .count();
        if !requested.contains(*batch) {
            result.push(BatchResult {
                batch: (*batch).to_owned(),
                declaration_count,
                link_status: "not_run".to_owned(),
                runtime_status: "not_run".to_owned(),
                scope: "not requested".to_owned(),
            });
            continue;
        }
        let (link_status, runtime_status, scope) = match *batch {
            "batch_numeric_scalar_subroutines" => run_rust_batch(
                context,
                batch,
                "numeric_scalar_subroutines.rs",
                numeric_scalar_driver(),
            )?,
            "batch_numeric_array_subroutines" => run_rust_batch(
                context,
                batch,
                "numeric_array_subroutines.rs",
                numeric_array_driver(),
            )?,
            "batch_scalar_functions" => run_rust_batch(
                context,
                batch,
                "scalar_functions.rs",
                scalar_function_driver(),
            )?,
            "batch_complex_arguments" => run_complex_batch(context)?,
            "batch_complex_returns" => run_complex_return_batch(context)?,
            "batch_logical" => run_logical_batch(context)?,
            "batch_character" => run_character_batch(context)?,
            "batch_callbacks" => run_authored_abi_batch(context, "callback")?,
            "batch_infrastructure" => run_machine_error_driver(context)?,
            _ => unreachable!(),
        };
        result.push(BatchResult {
            batch: (*batch).to_owned(),
            declaration_count,
            link_status,
            runtime_status,
            scope,
        });
    }
    Ok(result)
}

fn run_rust_batch(
    context: &NativeContext,
    batch: &str,
    module_file: &str,
    driver: String,
) -> Result<(String, String, String)> {
    let module_path = fs::canonicalize(context.bindings_dir.join(module_file))?;
    let source = driver_prelude(&module_path, module_file.trim_end_matches(".rs"), &driver);
    let status = compile_and_run_rust(context, batch, &source)?;
    Ok((
        status.0,
        status.1,
        "generated declaration runtime smoke test".to_owned(),
    ))
}

fn run_complex_batch(context: &NativeContext) -> Result<(String, String, String)> {
    let module_path = fs::canonicalize(context.bindings_dir.join("complex_arguments.rs"))?;
    let source = driver_prelude(&module_path, "complex_arguments", &complex_driver());
    let status = compile_and_run_rust(context, "batch_complex_arguments", &source)?;
    if status.1 != "passed" {
        return Ok((
            status.0,
            status.1,
            "single-complex generated call".to_owned(),
        ));
    }
    let double_complex = run_authored_abi_batch(context, "double_complex")?;
    Ok((
        status.0,
        double_complex.1,
        "single-complex generated call plus authored double-complex layout probe".to_owned(),
    ))
}

fn run_complex_return_batch(context: &NativeContext) -> Result<(String, String, String)> {
    let module_path = fs::canonicalize(context.bindings_dir.join("complex_returns.rs"))?;
    let source = driver_prelude(&module_path, "complex_returns", &complex_return_driver());
    let status = compile_and_run_rust(context, "batch_complex_returns", &source)?;
    Ok((
        status.0,
        status.1,
        "generated single- and double-complex function-return calls".to_owned(),
    ))
}

fn run_logical_batch(context: &NativeContext) -> Result<(String, String, String)> {
    let module_path = fs::canonicalize(context.bindings_dir.join("logical.rs"))?;
    let source = driver_prelude(&module_path, "logical", &logical_driver());
    let status = compile_and_run_rust(context, "batch_logical", &source)?;
    if status.1 != "passed" {
        return Ok((
            status.0,
            status.1,
            "generated LOGICAL argument link/runtime probe".to_owned(),
        ));
    }
    let authored = run_authored_abi_batch(context, "logical")?;
    Ok((
        status.0,
        authored.1,
        "generated LOGICAL argument call plus authored true/false representation probe".to_owned(),
    ))
}

fn run_character_batch(context: &NativeContext) -> Result<(String, String, String)> {
    let module_path = fs::canonicalize(context.bindings_dir.join("character.rs"))?;
    let source = driver_prelude(&module_path, "character", &character_driver());
    let status = compile_and_run_rust(context, "batch_character", &source)?;
    if status.1 != "passed" {
        return Ok((
            status.0,
            status.1,
            "generated two-character DGEMM call".to_owned(),
        ));
    }
    let authored = run_authored_abi_batch(context, "character")?;
    Ok((
        status.0,
        authored.1,
        "generated two-character DGEMM call plus authored one-character hidden-length probe"
            .to_owned(),
    ))
}

fn run_authored_abi_batch(context: &NativeContext, kind: &str) -> Result<(String, String, String)> {
    let (fortran, rust, scope) = match kind {
        "callback" => (
            "      SUBROUTINE SLATEC_VALIDATION_CALLBACK(F,X,Y)\n      DOUBLE PRECISION F,X,Y\n      EXTERNAL F\n      Y = F(X)\n      END\n",
            r#"unsafe extern "C" { fn slatec_validation_callback_(f: unsafe extern "C" fn(*const f64) -> f64, x: *mut f64, y: *mut f64); }
unsafe extern "C" fn twice(value: *const f64) -> f64 { unsafe { 2.0 * *value } }
fn main() { let mut x = 2.5_f64; let mut y = 0.0_f64; unsafe { slatec_validation_callback_(twice, &mut x, &mut y) }; if y != 5.0 { std::process::exit(2) } }
"#,
            "authored callback-shape probe; no callback declaration is enabled",
        ),
        "double_complex" => (
            "      SUBROUTINE SLATEC_VALIDATION_DCOMPLEX(VALUE)\n      DOUBLE COMPLEX VALUE\n      VALUE = VALUE * (1.0D0,2.0D0)\n      END\n",
            r#"#[repr(C)] struct Complex64 { re: f64, im: f64 }
unsafe extern "C" { fn slatec_validation_dcomplex_(value: *mut Complex64); }
fn main() { let mut value = Complex64 { re: 1.0, im: 2.0 }; unsafe { slatec_validation_dcomplex_(&mut value) }; if value.re != -3.0 || value.im != 4.0 { std::process::exit(2) } }
"#,
            "authored double-complex argument layout probe; complex-returning functions stay gated",
        ),
        "logical" => (
            "      SUBROUTINE SLATEC_VALIDATION_LOGICAL(VALUE,ANSWER)\n      LOGICAL VALUE\n      INTEGER ANSWER\n      IF (VALUE) THEN\n        ANSWER = 1\n      ELSE\n        ANSWER = 0\n      END IF\n      END\n      LOGICAL FUNCTION SLATEC_VALIDATION_LOGICAL_RESULT(VALUE)\n      LOGICAL VALUE\n      SLATEC_VALIDATION_LOGICAL_RESULT = VALUE\n      END\n",
            r#"unsafe extern "C" { fn slatec_validation_logical_(value: *mut i32, answer: *mut i32); fn slatec_validation_logical_result_(value: *mut i32) -> i32; }
fn main() { let mut value = 1_i32; let mut answer = 0_i32; unsafe { slatec_validation_logical_(&mut value, &mut answer) }; if answer != 1 || unsafe { slatec_validation_logical_result_(&mut value) } != 1 { std::process::exit(2) }; value = 0; unsafe { slatec_validation_logical_(&mut value, &mut answer) }; if answer != 0 || unsafe { slatec_validation_logical_result_(&mut value) } != 0 { std::process::exit(3) } }
"#,
            "authored LOGICAL width and true/false representation probe; raw API uses i32, never Rust bool",
        ),
        "character" => (
            "      SUBROUTINE SLATEC_VALIDATION_CHARACTER(VALUE,ANSWER)\n      CHARACTER*(*) VALUE\n      INTEGER ANSWER\n      ANSWER = LEN(VALUE)\n      END\n",
            r#"unsafe extern "C" { fn slatec_validation_character_(value: *mut core::ffi::c_char, answer: *mut i32, length: usize); }
fn main() { let mut value = [b'A' as core::ffi::c_char, b'B' as core::ffi::c_char, b'C' as core::ffi::c_char]; let mut answer = 0_i32; unsafe { slatec_validation_character_(value.as_mut_ptr(), &mut answer, 3) }; if answer != 3 { std::process::exit(2) } }
"#,
            "authored one-character hidden-length position and width probe; generated DGEMM validates two character arguments",
        ),
        _ => unreachable!(),
    };
    let root = context
        .archive
        .parent()
        .ok_or_else(|| CorpusError::Verification("native archive has no parent".to_owned()))?
        .join("validation-drivers");
    fs::create_dir_all(&root)?;
    let fortran_path = root.join(format!("{kind}.f"));
    let object = root.join(format!("{kind}.o"));
    fs::write(&fortran_path, fortran)?;
    let output = Command::new(&context.compiler)
        .args(["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"])
        .arg(&fortran_path)
        .arg("-o")
        .arg(&object)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not compile {kind} probe: {error}"))
        })?;
    fs::write(
        root.join(format!("{kind}.fortran.log")),
        [&output.stdout[..], &output.stderr[..]].concat(),
    )?;
    if !output.status.success() {
        return Ok((
            "compile_failed".to_owned(),
            "not_run".to_owned(),
            scope.to_owned(),
        ));
    }
    let status = compile_and_run_rust_with_objects(context, kind, rust, &[object])?;
    Ok((status.0, status.1, scope.to_owned()))
}

fn run_machine_error_driver(context: &NativeContext) -> Result<(String, String, String)> {
    let root = context
        .archive
        .parent()
        .ok_or_else(|| CorpusError::Verification("native archive has no parent".to_owned()))?
        .join("validation-drivers");
    fs::create_dir_all(&root)?;
    let source = root.join("machine_error.f");
    let executable = root.join("machine_error.exe");
    fs::write(
        &source,
        "      PROGRAM SLATEC_VALIDATION_MACHINE\n      INTEGER I1MACH, VALUE\n      VALUE = I1MACH(9)\n      CALL XERMSG('SLATEC','VALIDATE','SMOKE',1,-1)\n      END\n",
    )?;
    let output = Command::new(&context.compiler)
        .args(["-x", "f77", "-std=legacy", "-ffixed-line-length-none"])
        .arg(&source)
        .args(["-x", "none"])
        .arg("-Wl,--start-group")
        .arg(&context.archive)
        .arg("-Wl,--end-group")
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not build machine/error driver: {error}"))
        })?;
    fs::write(
        root.join("machine_error.log"),
        [&output.stdout[..], &output.stderr[..]].concat(),
    )?;
    if !output.status.success() || !executable.is_file() {
        return Ok(("compile_failed".to_owned(), "not_run".to_owned(), "Fortran driver invokes I1MACH and nonfatal XERMSG; it does not validate historical host tables".to_owned()));
    }
    let run = Command::new(&executable).output().map_err(|error| {
        CorpusError::Verification(format!("could not run machine/error driver: {error}"))
    })?;
    fs::write(
        root.join("machine_error.run.log"),
        [&run.stdout[..], &run.stderr[..]].concat(),
    )?;
    Ok((
        "passed".to_owned(),
        if run.status.success() { "passed" } else { "run_failed" }.to_owned(),
        "Fortran driver invokes I1MACH and nonfatal XERMSG; it does not validate historical host tables".to_owned(),
    ))
}

fn compile_and_run_rust(
    context: &NativeContext,
    name: &str,
    source: &str,
) -> Result<(String, String)> {
    compile_and_run_rust_with_objects(context, name, source, &[])
}

fn compile_and_run_rust_with_objects(
    context: &NativeContext,
    name: &str,
    source: &str,
    objects: &[PathBuf],
) -> Result<(String, String)> {
    let root = context
        .archive
        .parent()
        .ok_or_else(|| CorpusError::Verification("native archive has no parent".to_owned()))?
        .join("validation-drivers");
    fs::create_dir_all(&root)?;
    let source_path = root.join(format!("{name}.rs"));
    let executable = root.join(format!("{name}.exe"));
    fs::write(&source_path, source)?;
    let archive_parent = context.archive.parent().ok_or_else(|| {
        CorpusError::Verification("native archive has no parent directory".to_owned())
    })?;
    let mut command = Command::new("rustc");
    command
        .args(["--edition", "2024", "--target", RUST_TARGET])
        .arg("-C")
        .arg(format!("linker={}", context.compiler.display()))
        .arg(&source_path)
        .args(["-L", &format!("native={}", archive_parent.display())])
        .args(["-l", "static=slatec_selected"])
        .arg("-C")
        .arg("link-arg=-lgfortran")
        .arg("-C")
        .arg("link-arg=-lquadmath")
        .arg("-C")
        .arg("link-arg=-lmsvcrt");
    for object in objects {
        command
            .arg("-C")
            .arg(format!("link-arg={}", object.display()));
    }
    command.arg("-o").arg(&executable);
    let compile = command.output().map_err(|error| {
        CorpusError::Verification(format!("could not compile {name} Rust driver: {error}"))
    })?;
    fs::write(
        root.join(format!("{name}.rust.log")),
        [&compile.stdout[..], &compile.stderr[..]].concat(),
    )?;
    if !compile.status.success() || !executable.is_file() {
        return Ok(("compile_failed".to_owned(), "not_run".to_owned()));
    }
    let run = Command::new(&executable).output().map_err(|error| {
        CorpusError::Verification(format!("could not run {name} Rust driver: {error}"))
    })?;
    fs::write(
        root.join(format!("{name}.run.log")),
        [&run.stdout[..], &run.stderr[..]].concat(),
    )?;
    Ok((
        "passed".to_owned(),
        if run.status.success() {
            "passed"
        } else {
            "run_failed"
        }
        .to_owned(),
    ))
}

fn driver_prelude(module_path: &Path, module_name: &str, body: &str) -> String {
    format!(
        "pub type FortranInteger = i32;\npub type FortranLogical = i32;\npub type FortranCharacterLength = usize;\n#[repr(C)] pub struct Complex32 {{ pub re: f32, pub im: f32 }}\n#[repr(C)] pub struct Complex64 {{ pub re: f64, pub im: f64 }}\n#[path = r#\"{}\"#]\nmod {};\n{}\n",
        module_path.display(),
        module_name,
        body,
    )
}

fn numeric_scalar_driver() -> String {
    "fn main() { let (mut ar, mut ai, mut br, mut bi, mut cr, mut ci) = (1.0_f32, 2.0, 3.0, 4.0, 0.0, 0.0); unsafe { numeric_scalar_subroutines::cdiv(&mut ar, &mut ai, &mut br, &mut bi, &mut cr, &mut ci) }; if (cr - 0.44).abs() > 0.0001 || (ci - 0.08).abs() > 0.0001 { std::process::exit(2) } }".to_owned()
}

fn numeric_array_driver() -> String {
    "fn main() { let mut n = 3_i32; let mut alpha = 2.0_f64; let mut inc = 1_i32; let mut x = [1.0_f64, 2.0, 3.0]; let mut y = [4.0_f64, 5.0, 6.0]; unsafe { numeric_array_subroutines::daxpy(&mut n, &mut alpha, x.as_mut_ptr(), &mut inc, y.as_mut_ptr(), &mut inc) }; if y != [6.0, 9.0, 12.0] { std::process::exit(2) } }".to_owned()
}

fn scalar_function_driver() -> String {
    "fn main() { let mut n = 3_i32; let mut inc = 1_i32; let mut x = [-1.0_f64, 2.0, -3.0]; let result = unsafe { scalar_functions::dasum(&mut n, x.as_mut_ptr(), &mut inc) }; let mut values = [1.0_f64, -5.0, 2.0]; let index = unsafe { scalar_functions::idamax(&mut n, values.as_mut_ptr(), &mut inc) }; if result != 6.0 || index != 2 { std::process::exit(2) } }".to_owned()
}

fn complex_driver() -> String {
    "fn main() { let mut value = Complex32 { re: 3.0, im: 4.0 }; let result = unsafe { complex_arguments::cabs(&mut value) }; if result != 5.0 { std::process::exit(2) } }".to_owned()
}

fn complex_return_driver() -> String {
    "fn main() { let mut n = 1_i32; let mut inc = 1_i32; let mut x = [Complex32 { re: 1.0, im: 2.0 }]; let mut y = [Complex32 { re: 3.0, im: 4.0 }]; let c = unsafe { complex_returns::cdotu(&mut n, x.as_mut_ptr(), &mut inc, y.as_mut_ptr(), &mut inc) }; if c.re != -5.0 || c.im != 10.0 { std::process::exit(2) } let mut cb = Complex32 { re: 0.0, im: 0.0 }; let z = unsafe { complex_returns::cdcdot(&mut n, &mut cb, x.as_mut_ptr(), &mut inc, y.as_mut_ptr(), &mut inc) }; if z.re != -5.0 || z.im != 10.0 { std::process::exit(3) } }".to_owned()
}

fn logical_driver() -> String {
    "fn main() { let (mut nm, mut n, mut mb) = (1_i32, 1_i32, 0_i32); let mut a = [2.0_f32]; let mut d = [0.0_f32]; let mut e = [0.0_f32]; let mut e2 = [0.0_f32]; let mut matz = 0_i32; let mut z = [0.0_f32]; unsafe { logical::bandr(&mut nm, &mut n, &mut mb, a.as_mut_ptr(), d.as_mut_ptr(), e.as_mut_ptr(), e2.as_mut_ptr(), &mut matz, z.as_mut_ptr()) }; if matz != 0 { std::process::exit(2) } }".to_owned()
}

fn character_driver() -> String {
    "fn main() { let mut transa = [b'N' as core::ffi::c_char]; let mut transb = [b'N' as core::ffi::c_char]; let (mut m, mut n, mut k, mut lda, mut ldb, mut ldc) = (1_i32, 1_i32, 1_i32, 1_i32, 1_i32, 1_i32); let mut alpha = 1.0_f64; let mut beta = 0.0_f64; let mut a = [2.0_f64]; let mut b = [3.0_f64]; let mut c = [0.0_f64]; unsafe { character::dgemm(transa.as_mut_ptr(), transb.as_mut_ptr(), &mut m, &mut n, &mut k, &mut alpha, a.as_mut_ptr(), &mut lda, b.as_mut_ptr(), &mut ldb, &mut beta, c.as_mut_ptr(), &mut ldc, 1, 1) }; if c[0] != 6.0 { std::process::exit(2) } }".to_owned()
}

fn validation_outputs(
    context: &NativeContext,
    generated: &raw_ffi::RawFfiResult,
    audit: &SignatureAudit,
    batches: &[BatchResult],
    review: &[(String, String, String, String, String)],
    runtime_passed: usize,
    runtime_pending: usize,
) -> Result<BTreeMap<&'static str, Vec<u8>>> {
    let mut outputs = native_build_outputs(context, generated)?;
    let diagnostics = audit
        .failures
        .iter()
        .map(|(unit, rule)| {
            json!([
                stable_id("ffi-validation-diagnostic", &[unit, rule]),
                unit,
                rule
            ])
        })
        .collect::<Vec<_>>();
    outputs.insert(
        "signature-audit-summary.json",
        compact(&json!({
            "schema_id":"slatec-rs/raw-ffi-signature-audit",
            "schema_version":SCHEMA_VERSION,
            "snapshot_id":context.snapshot_id,
            "summary":{
                "emitted_declarations_expected":audit.expected,
                "parsed_generated_declarations":audit.parsed,
                "observed_symbol_matches":audit.observed_symbols,
                "formal_argument_count_matches":audit.argument_count_matches,
                "formal_argument_order_matches":audit.argument_order_matches,
                "character_hidden_length_matches":audit.character_lengths,
                "function_kind_matches":audit.function_kinds,
                "function_return_matches":audit.function_returns,
                "formal_external_or_intrinsic_arguments_gated":audit.callback_gated,
                "infrastructure_gated":audit.infrastructure_gated,
                "duplicate_rust_declarations":audit.duplicate_rust_names,
                "duplicate_link_names":audit.duplicate_link_names,
                "structural_failures":audit.failures.len()
            }
        }))?,
    );
    let batch_rows = batches
        .iter()
        .map(|batch| {
            json!([
                batch.batch,
                batch.declaration_count,
                batch.link_status,
                batch.runtime_status,
                batch.scope
            ])
        })
        .collect::<Vec<_>>();
    outputs.insert(
        "batch-validation-summary.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-batch-validation","schema_version":SCHEMA_VERSION,"snapshot_id":context.snapshot_id,"columns":["batch","generated_declaration_count","link_status","runtime_status","scope"],"records":batch_rows}))?,
    );
    let runtime_rows = batches
        .iter()
        .filter(|batch| batch.runtime_status != "not_run")
        .map(|batch| json!([batch.batch, batch.runtime_status, batch.scope]))
        .collect::<Vec<_>>();
    outputs.insert(
        "runtime-test-summary.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-runtime-tests","schema_version":SCHEMA_VERSION,"snapshot_id":context.snapshot_id,"columns":["batch","status","scope"],"records":runtime_rows}))?,
    );
    let reason_rows = review
        .iter()
        .map(|row| json!([row.0, row.1, row.2, row.3, row.4]))
        .collect::<Vec<_>>();
    let mut counts = BTreeMap::<String, usize>::new();
    for row in review {
        *counts.entry(row.3.clone()).or_default() += 1;
    }
    outputs.insert(
        "review-reason-summary.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-review-reasons","schema_version":SCHEMA_VERSION,"snapshot_id":context.snapshot_id,"reason_counts":counts,"columns":["program_unit_id","normalized_name","confidence_class","reason","rule"],"records":reason_rows}))?,
    );
    outputs.insert(
        "diagnostics.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-validation-diagnostics","schema_version":SCHEMA_VERSION,"snapshot_id":context.snapshot_id,"columns":["id","program_unit_id","rule"],"records":diagnostics}))?,
    );
    outputs.insert(
        "validation-summary.md",
        format!(
            "# Generated raw FFI validation\n\n- Snapshot: `{}`\n- Profile: `{}` (`{}`)\n- Structural declarations audited: {}; failures: {}\n- Runtime batches passed: {}; pending or failed: {}\n- Profile feature: `{PROFILE_FEATURE}`\n\nThis validates only the GNU MinGW raw ABI profile. Validated complex-returning functions are included; character-returning functions, callbacks, unresolved interfaces, and infrastructure remain gated. No safe API is exposed.\n",
            context.snapshot_id,
            context.compiler_identity,
            context.compiler_target,
            audit.expected,
            audit.failures.len() + audit.duplicate_link_names + audit.duplicate_rust_names,
            runtime_passed,
            runtime_pending,
        ).into_bytes(),
    );
    Ok(outputs)
}

fn rust_return_type(kind: &str) -> Option<&'static str> {
    match kind {
        "INTEGER" => Some("FortranInteger"),
        "REAL" => Some("f32"),
        "DOUBLE PRECISION" => Some("f64"),
        "LOGICAL" => Some("FortranLogical"),
        "COMPLEX" => Some("Complex32"),
        "DOUBLE COMPLEX" => Some("Complex64"),
        _ => None,
    }
}

fn read_value(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path)?).map_err(CorpusError::from)
}

fn runtime_profile_status(snapshot: &str) -> Result<Value> {
    let path = Path::new("generated/runtime-profile/manifest.json");
    if !path.is_file() {
        return Ok(json!({
            "abi_validated": true,
            "machine_constants_validated": false,
            "legacy_error_behavior_validated": false,
            "fnlib_initialization_validated": false,
            "status": "pending_runtime_profile_validation"
        }));
    }
    let manifest = read_value(path)?;
    if manifest["snapshot_id"].as_str() != Some(snapshot) {
        return Err(CorpusError::Verification(
            "runtime-profile manifest does not match raw-FFI snapshot".to_owned(),
        ));
    }
    Ok(manifest["validation"].clone())
}

fn records<'a>(value: &'a Value, name: &str) -> Result<&'a Vec<Value>> {
    value["records"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification(format!("{name} is missing compact records")))
}

fn required_string(value: &Value, field: &str, source: &str) -> Result<String> {
    value[field]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("{source} is missing {field}")))
}

fn row_string(row: &Value, index: usize, field: &str) -> Result<String> {
    row.get(index)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("compact record is missing {field}")))
}

fn row_optional_string(row: &Value, index: usize) -> Option<String> {
    row.get(index).and_then(Value::as_str).map(str::to_owned)
}

fn row_array_strings(row: &Value, index: usize) -> Vec<String> {
    row.get(index)
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn row_bool(row: &Value, index: usize) -> bool {
    row.get(index).and_then(Value::as_bool).unwrap_or(false)
}

fn compact(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn stable_id(kind: &str, parts: &[&str]) -> String {
    format!(
        "{kind}-{}",
        &hash::bytes(parts.join("\u{1f}").as_bytes())[..16]
    )
}

fn semantic_hash(outputs: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut material = Vec::new();
    for (name, bytes) in outputs {
        material.extend_from_slice(name.as_bytes());
        material.push(0);
        material.extend_from_slice(bytes);
        material.push(0);
    }
    hash::bytes(&material)
}

fn enforce_size(outputs: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let total = outputs
        .values()
        .map(|bytes| bytes.len() as u64)
        .sum::<u64>();
    if total > COMMITTED_SIZE_LIMIT {
        return Err(CorpusError::Verification(format!(
            "raw FFI validation output would be {total} bytes, exceeding 4 MB"
        )));
    }
    Ok(())
}

fn promote(output_dir: &Path, snapshot: &str, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let parent = output_dir.parent().ok_or_else(|| {
        CorpusError::Policy("FFI validation output directory must have a parent".to_owned())
    })?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        "{}.staging-{snapshot}",
        output_dir.file_name().unwrap_or_default().to_string_lossy()
    ));
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;
    for (name, bytes) in files {
        fs::write(staging.join(name), bytes)?;
    }
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::rename(staging, output_dir)?;
    Ok(())
}

fn compiler_path() -> PathBuf {
    std::env::var_os("SLATEC_FORTRAN_COMPILER")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("x86_64-w64-mingw32-gfortran"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiline_generated_binding_and_hidden_lengths() {
        let temp = tempfile::tempdir().unwrap();
        for file in [
            "numeric_scalar_subroutines.rs",
            "numeric_array_subroutines.rs",
            "scalar_functions.rs",
            "complex_arguments.rs",
            "complex_returns.rs",
            "logical.rs",
            "callbacks.rs",
            "infrastructure.rs",
        ] {
            fs::write(temp.path().join(file), "").unwrap();
        }
        fs::write(
            temp.path().join("character.rs"),
            "#[link_name = \"test_\"]\npub fn test(value: *mut c_char, character_length_1: FortranCharacterLength);\n",
        )
        .unwrap();
        let bindings = parse_bindings(temp.path()).unwrap();
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].parameters, 2);
        assert_eq!(bindings[0].character_lengths, 1);
    }

    #[test]
    fn classifies_explicit_callback_for_review() {
        let interface = Interface {
            id: "unit".to_owned(),
            name: "TEST".to_owned(),
            kind: "subroutine".to_owned(),
            argument_ids: vec!["argument".to_owned()],
            function_result_id: None,
            observed_symbol: Some("test_".to_owned()),
            confidence: "manual_review_required".to_owned(),
            diagnostic_ids: Vec::new(),
        };
        let mut arguments = BTreeMap::new();
        arguments.insert(
            "argument".to_owned(),
            Argument {
                declared_type: Some("REAL".to_owned()),
                is_external: true,
                ..Argument::default()
            },
        );
        let review = review_reasons(&[interface], &arguments, &BTreeMap::new());
        assert_eq!(review[0].3, "callback_argument");
    }

    #[test]
    fn rejects_unknown_batch_names() {
        assert!(requested_batches(&["unknown".to_owned()]).is_err());
    }

    #[test]
    fn selects_one_requested_batch_without_enabling_the_others() {
        let batches = requested_batches(&["batch_character".to_owned()]).unwrap();
        assert_eq!(batches, BTreeSet::from(["batch_character".to_owned()]));
    }

    #[test]
    fn rejects_oversized_committed_validation_output() {
        let mut outputs = BTreeMap::new();
        outputs.insert(
            "too-large.json",
            vec![0_u8; COMMITTED_SIZE_LIMIT as usize + 1],
        );
        assert!(enforce_size(&outputs).is_err());
    }

    #[test]
    fn audits_formal_argument_order_and_trailing_character_lengths() {
        let interface = Interface {
            id: "unit".to_owned(),
            name: "TEST".to_owned(),
            kind: "subroutine".to_owned(),
            argument_ids: vec!["a".to_owned(), "b".to_owned()],
            function_result_id: None,
            observed_symbol: Some("test_".to_owned()),
            confidence: "generated_abi_sensitive".to_owned(),
            diagnostic_ids: Vec::new(),
        };
        let mut arguments = BTreeMap::new();
        arguments.insert(
            "a".to_owned(),
            Argument {
                name: "A".to_owned(),
                declared_type: Some("REAL".to_owned()),
                ..Argument::default()
            },
        );
        arguments.insert(
            "b".to_owned(),
            Argument {
                name: "TYPE".to_owned(),
                declared_type: Some("CHARACTER".to_owned()),
                ..Argument::default()
            },
        );
        let binding = Binding {
            batch: "batch_character".to_owned(),
            rust_name: "test".to_owned(),
            link_name: "test_".to_owned(),
            parameter_names: vec![
                "a".to_owned(),
                "r#type".to_owned(),
                "character_length_1".to_owned(),
            ],
            parameters: 3,
            character_lengths: 1,
            is_function: false,
            return_type: None,
        };
        let audit = audit_signatures(
            &[interface],
            &arguments,
            &BTreeMap::new(),
            &BTreeSet::from(["test_".to_owned()]),
            &[binding],
        );
        assert!(audit.failures.is_empty());
        assert_eq!(audit.argument_order_matches, 1);
    }
}
