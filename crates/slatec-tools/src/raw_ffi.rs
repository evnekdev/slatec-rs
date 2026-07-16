//! Corpus-wide native compilation, symbol observation, and raw FFI generation.
//!
//! The module intentionally consumes the selected-provider manifest and the
//! conservative interface inventory.  It does not discover a corpus, parse
//! executable statements, or make a Cargo build implicitly compile Fortran.
//! Compilation products and verbose compiler evidence remain under the ignored
//! evidence root; committed records contain only compact structural facts.

use crate::error::{CorpusError, Result};
use crate::hash;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA_VERSION: &str = "1.0.0";
const SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const COMMITTED_SIZE_LIMIT: u64 = 4_000_000;
const FORTRAN_FLAGS: &[&str] = &["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"];
const PROFILE_TARGET: &str = "x86_64-w64-mingw32";
const PROFILE_OVERRIDE_NAMES: &[&str] = &["D1MACH", "I1MACH", "R1MACH", "XERHLT"];

#[derive(Clone, Deserialize)]
struct Provider {
    snapshot_id: String,
    program_unit_id: String,
    normalized_name: String,
    kind: String,
    source_subset: String,
    source_path: String,
    raw_sha256: String,
    normalized_sha256: String,
    selection_category: String,
}

#[derive(Deserialize)]
struct ProviderRecords {
    snapshot_id: String,
    records: Vec<Provider>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct RawFfiResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
    pub bindings_dir: PathBuf,
    pub summary: RawFfiSummary,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct RawFfiSummary {
    pub selected_program_units: usize,
    pub selected_source_files: usize,
    pub compiled_source_files: usize,
    pub failed_source_files: usize,
    pub observed_defined_symbols: usize,
    pub generated_standard: usize,
    pub generated_abi_sensitive: usize,
    pub manual_review_required: usize,
    pub unsupported: usize,
    pub non_callable_infrastructure: usize,
    pub generated_bindings: usize,
}

#[derive(Clone)]
struct SourceGroup {
    subset: String,
    path: String,
    raw_sha256: String,
    normalized_sha256: String,
    providers: Vec<Provider>,
}

#[derive(Clone, Default)]
struct UnitFacts {
    kind: String,
    declaration_locator_id: Option<String>,
    diagnostic_ids: Vec<String>,
    arguments: Vec<ArgumentFact>,
    function_result: Option<ResultFact>,
}

#[derive(Clone, Default)]
struct ArgumentFact {
    id: String,
    position: u64,
    name: String,
    declared_type: Option<String>,
    type_source: String,
    character_length: Option<String>,
    dimensions: Vec<Value>,
    is_external: bool,
    is_intrinsic: bool,
    locator_ids: Vec<String>,
    conflict_state: Option<String>,
}

#[derive(Clone, Default)]
struct ResultFact {
    id: String,
    declared_type: Option<String>,
    type_source: String,
    character_length: Option<String>,
    locator_id: Option<String>,
    conflict_state: Option<String>,
}

#[derive(Clone)]
struct CompileRecord {
    source_id: String,
    group: SourceGroup,
    status: String,
    exit_code: Option<i32>,
    failure_rule: Option<String>,
    object_path: Option<PathBuf>,
    object_id: Option<String>,
    diagnostic_log_id: String,
    symbols: Vec<Symbol>,
}

#[derive(Clone)]
struct ProfileOverride {
    normalized_name: String,
    original_provider_id: String,
    original_source_id: String,
    original_raw_sha256: String,
    relative_path: String,
    raw_sha256: String,
    object_path: PathBuf,
    object_id: String,
    symbols: Vec<Symbol>,
}

#[derive(Clone)]
struct Symbol {
    source_id: String,
    raw: String,
    kind: String,
    normalized: String,
}

#[derive(Clone)]
struct InterfaceRecord {
    provider: Provider,
    facts: UnitFacts,
    observed_symbol: Option<String>,
    confidence: String,
    batch: Option<String>,
    review_state: String,
    diagnostic_ids: Vec<String>,
}

#[derive(Clone)]
struct CompilerProfile {
    identity: String,
    version: String,
    target: String,
    profile_id: String,
}

#[derive(Clone, Default)]
struct AbiValidation {
    status: String,
    integer_real_double: String,
    logical: String,
    complex: String,
    character: String,
    callback: String,
    selected_numeric_rust: String,
    selected_complex_rust: String,
    selected_machine_error: String,
}

/// Compile the exact selected corpus, observe its symbols, classify every
/// selected program unit, and generate only raw declarations with a validated
/// ABI category.  This operation is intentionally offline-only.
pub fn generate(
    evidence_dir: &Path,
    selected_corpus_dir: &Path,
    inventory_dir: &Path,
    output_dir: &Path,
    bindings_dir: &Path,
    offline: bool,
) -> Result<RawFfiResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-raw-ffi only uses verified local evidence and requires --offline".to_owned(),
        ));
    }

    let providers: ProviderRecords =
        read_json(&selected_corpus_dir.join("selected-providers.json"))?;
    let snapshot = providers.snapshot_id.clone();
    if providers.records.len() != 1_476 {
        return Err(CorpusError::Verification(format!(
            "raw FFI must use the exact 1476-provider selected manifest, found {}",
            providers.records.len()
        )));
    }
    let selection_manifest: Value = read_json(&selected_corpus_dir.join("manifest.json"))?;
    if selection_manifest["snapshot_id"].as_str() != Some(snapshot.as_str()) {
        return Err(CorpusError::Verification(
            "selected-provider and selected-corpus manifest snapshot mismatch".to_owned(),
        ));
    }
    let main_src_snapshot = selection_manifest["main_src_snapshot_id"]
        .as_str()
        .ok_or_else(|| {
            CorpusError::Verification("selected-corpus manifest lacks main-src snapshot".to_owned())
        })?
        .to_owned();
    let unresolved: Value = read_json(&selected_corpus_dir.join("unresolved-providers.json"))?;
    if unresolved["records"]
        .as_array()
        .is_none_or(|rows| !rows.is_empty())
    {
        return Err(CorpusError::Verification(
            "selected corpus has unresolved providers; raw FFI generation is blocked".to_owned(),
        ));
    }
    let selection_hash = hash::file(&selected_corpus_dir.join("selected-providers.json"))?;
    let facts = load_inventory(inventory_dir, &snapshot, &selection_hash)?;
    if facts.len() != providers.records.len() {
        return Err(CorpusError::Verification(format!(
            "interface inventory has {} records but selected manifest has {}",
            facts.len(),
            providers.records.len()
        )));
    }

    let groups = source_groups(providers.records, &snapshot)?;
    let compiler = compiler_profile()?;
    let workspace = evidence_dir
        .join("raw-ffi")
        .join(&snapshot)
        .join(&compiler.profile_id);
    fs::create_dir_all(workspace.join("objects"))?;
    fs::create_dir_all(workspace.join("logs"))?;

    let mut compilation = Vec::new();
    for group in groups.values() {
        compilation.push(compile_group(
            evidence_dir,
            &snapshot,
            &main_src_snapshot,
            group,
            &compiler,
            &workspace,
        )?);
    }
    compilation.sort_by(|left, right| left.source_id.cmp(&right.source_id));
    let overrides = compile_profile_overrides(&compiler, &workspace, &compilation)?;
    let archive = archive_objects(&workspace, &compilation, &overrides)?;
    let abi = run_abi_validation(&compiler, &workspace, archive.as_deref())?;

    let symbols = compilation
        .iter()
        .flat_map(|record| record.symbols.iter().cloned())
        .collect::<Vec<_>>();
    let mut observed = BTreeMap::<String, Vec<String>>::new();
    for symbol in &symbols {
        observed
            .entry(symbol.normalized.clone())
            .or_default()
            .push(symbol.raw.clone());
    }
    for raw in observed.values_mut() {
        raw.sort();
        raw.dedup();
    }
    let compiled_by_provider = compilation_by_provider(&compilation);
    let mut interfaces =
        providers_from_compilation(groups, &facts, &observed, &compiled_by_provider, &abi)?;
    interfaces.sort_by(|left, right| {
        left.provider
            .program_unit_id
            .cmp(&right.provider.program_unit_id)
    });

    let summary = summarize(&interfaces, &compilation, symbols.len());
    let outputs = outputs(
        &snapshot,
        &compiler,
        &compilation,
        &symbols,
        &interfaces,
        &abi,
        &summary,
        &overrides,
    )?;
    let semantic_hash = semantic_hash(&outputs);
    let status = if summary.failed_source_files == 0
        && summary.unsupported == 0
        && summary.manual_review_required == 0
    {
        "success"
    } else {
        "success_with_review_items"
    };
    let mut outputs = outputs;
    outputs.insert(
        "manifest.json",
        compact(&json!({
            "id": stable_id("raw-ffi", &[&snapshot, &semantic_hash]),
            "schema_id": "slatec-rs/raw-ffi-generation",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "semantic_version": SEMANTIC_VERSION,
            "selected_provider_manifest_sha256": selection_hash,
            "interface_inventory_semantic_hash": inventory_semantic_hash(inventory_dir)?,
            "compiler_profile_id": compiler.profile_id,
            "output_semantic_hash": semantic_hash,
            "status": status,
            "summary": summary,
            "scope": "Raw declarations are emitted only when a selected provider compiled, its symbol was observed, and the required ABI class passed the explicit GNU Fortran profile validation. This is not a safe API or a general ABI proof."
        }))?,
    );
    enforce_size(&outputs)?;
    promote(output_dir, &snapshot, &outputs)?;
    generate_bindings(bindings_dir, &snapshot, &compiler, &interfaces)?;

    Ok(RawFfiResult {
        snapshot_id: snapshot,
        status: status.to_owned(),
        semantic_hash,
        output_dir: output_dir.to_owned(),
        bindings_dir: bindings_dir.to_owned(),
        summary,
    })
}

fn load_inventory(
    inventory_dir: &Path,
    snapshot: &str,
    expected_selection_hash: &str,
) -> Result<BTreeMap<String, UnitFacts>> {
    let manifest: Value = read_json(&inventory_dir.join("manifest.json"))?;
    if manifest["snapshot_id"].as_str() != Some(snapshot)
        || manifest["selected_provider_manifest_sha256"].as_str() != Some(expected_selection_hash)
    {
        return Err(CorpusError::Verification(
            "FFI interface inventory does not match the selected-provider manifest".to_owned(),
        ));
    }
    let units = compact_records(
        &inventory_dir.join("routine-interface-index.json"),
        snapshot,
    )?;
    let arguments = compact_records(&inventory_dir.join("argument-index.json"), snapshot)?;
    let results = compact_records(&inventory_dir.join("function-results.json"), snapshot)?;
    let mut facts = BTreeMap::<String, UnitFacts>::new();
    for row in units {
        let id = string_at(&row, 0, "interface program unit")?;
        let kind = string_at(&row, 2, "interface kind")?;
        let declaration_locator_id = optional_string_at(&row, 5);
        let diagnostic_ids = value_at(&row, 7)
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default();
        if facts
            .insert(
                id.clone(),
                UnitFacts {
                    kind,
                    declaration_locator_id,
                    diagnostic_ids,
                    ..UnitFacts::default()
                },
            )
            .is_some()
        {
            return Err(CorpusError::Verification(
                "duplicate program-unit ID in FFI interface inventory".to_owned(),
            ));
        }
    }
    for row in arguments {
        let program_unit_id = string_at(&row, 1, "argument program unit")?;
        let fact = ArgumentFact {
            id: string_at(&row, 0, "argument ID")?,
            position: value_at(&row, 2).and_then(Value::as_u64).ok_or_else(|| {
                CorpusError::Verification("argument position is missing".to_owned())
            })?,
            name: string_at(&row, 3, "argument name")?,
            declared_type: optional_string_at(&row, 4),
            type_source: optional_string_at(&row, 5).unwrap_or_else(|| "unknown".to_owned()),
            character_length: optional_string_at(&row, 6),
            dimensions: value_at(&row, 7)
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            is_external: value_at(&row, 8).and_then(Value::as_bool).unwrap_or(false),
            is_intrinsic: value_at(&row, 9).and_then(Value::as_bool).unwrap_or(false),
            locator_ids: value_at(&row, 10)
                .and_then(Value::as_array)
                .map(|values| {
                    values
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                })
                .unwrap_or_default(),
            conflict_state: optional_string_at(&row, 12),
        };
        facts
            .get_mut(&program_unit_id)
            .ok_or_else(|| {
                CorpusError::Verification(
                    "argument refers to a program unit absent from interface inventory".to_owned(),
                )
            })?
            .arguments
            .push(fact);
    }
    for row in results {
        let program_unit_id = string_at(&row, 1, "function-result program unit")?;
        let fact = ResultFact {
            id: string_at(&row, 0, "function-result ID")?,
            declared_type: optional_string_at(&row, 2),
            type_source: optional_string_at(&row, 3).unwrap_or_else(|| "unknown".to_owned()),
            character_length: optional_string_at(&row, 4),
            locator_id: optional_string_at(&row, 5),
            conflict_state: optional_string_at(&row, 6),
        };
        facts
            .get_mut(&program_unit_id)
            .ok_or_else(|| {
                CorpusError::Verification(
                    "function result refers to a program unit absent from interface inventory"
                        .to_owned(),
                )
            })?
            .function_result = Some(fact);
    }
    for fact in facts.values_mut() {
        fact.arguments.sort_by_key(|argument| argument.position);
    }
    Ok(facts)
}

fn compact_records(path: &Path, snapshot: &str) -> Result<Vec<Vec<Value>>> {
    let value: Value = read_json(path)?;
    if value["snapshot_id"].as_str() != Some(snapshot) {
        return Err(CorpusError::Verification(format!(
            "{} has a mismatched snapshot ID",
            path.display()
        )));
    }
    value["records"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification(format!("{} lacks records", path.display())))?
        .iter()
        .map(|value| {
            value.as_array().cloned().ok_or_else(|| {
                CorpusError::Verification(format!(
                    "{} has a non-array compact record",
                    path.display()
                ))
            })
        })
        .collect()
}

fn value_at(row: &[Value], index: usize) -> Option<&Value> {
    row.get(index)
}

fn string_at(row: &[Value], index: usize, field: &str) -> Result<String> {
    optional_string_at(row, index).ok_or_else(|| {
        CorpusError::Verification(format!("{field} is missing from compact interface record"))
    })
}

fn optional_string_at(row: &[Value], index: usize) -> Option<String> {
    value_at(row, index)
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn source_groups(
    providers: Vec<Provider>,
    snapshot: &str,
) -> Result<BTreeMap<String, SourceGroup>> {
    let mut ids = BTreeSet::new();
    let mut groups = BTreeMap::<String, SourceGroup>::new();
    for provider in providers {
        if provider.snapshot_id != snapshot || !ids.insert(provider.program_unit_id.clone()) {
            return Err(CorpusError::Verification(
                "duplicate or snapshot-mismatched selected provider".to_owned(),
            ));
        }
        let source_id = source_id(snapshot, &provider);
        let group = groups.entry(source_id).or_insert_with(|| SourceGroup {
            subset: provider.source_subset.clone(),
            path: provider.source_path.clone(),
            raw_sha256: provider.raw_sha256.clone(),
            normalized_sha256: provider.normalized_sha256.clone(),
            providers: Vec::new(),
        });
        if group.subset != provider.source_subset
            || group.path != provider.source_path
            || group.raw_sha256 != provider.raw_sha256
            || group.normalized_sha256 != provider.normalized_sha256
        {
            return Err(CorpusError::Verification(
                "selected-provider source identity has inconsistent hashes".to_owned(),
            ));
        }
        group.providers.push(provider);
    }
    for group in groups.values_mut() {
        group
            .providers
            .sort_by(|left, right| left.program_unit_id.cmp(&right.program_unit_id));
    }
    Ok(groups)
}

fn compiler_profile() -> Result<CompilerProfile> {
    let compiler = compiler_path();
    let version = command_output(&compiler, &["--version"])?;
    let target = command_output(&compiler, &["-dumpmachine"])?;
    if !version.contains("GNU Fortran") {
        return Err(CorpusError::Verification(format!(
            "only the explicitly supported GNU Fortran profile is supported, found {version}"
        )));
    }
    if target != PROFILE_TARGET {
        return Err(CorpusError::Verification(format!(
            "runtime profile requires GNU Fortran target {PROFILE_TARGET}, found {target}"
        )));
    }
    let identity = "gnu-fortran".to_owned();
    let profile_id = stable_id(
        "native-profile",
        &[&identity, &version, &target, &FORTRAN_FLAGS.join(" ")],
    );
    Ok(CompilerProfile {
        identity,
        version,
        target,
        profile_id,
    })
}

fn profile_source_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("native/profile/gnu-mingw-x86_64")
        .join(format!("{}.f", name.to_ascii_lowercase()))
}

fn compile_profile_overrides(
    compiler: &CompilerProfile,
    workspace: &Path,
    compilation: &[CompileRecord],
) -> Result<Vec<ProfileOverride>> {
    let compiler_path = compiler_path();
    let mut overrides = Vec::new();
    for name in PROFILE_OVERRIDE_NAMES {
        let owners = compilation
            .iter()
            .filter(|record| {
                record
                    .group
                    .providers
                    .iter()
                    .any(|provider| provider.normalized_name == *name)
            })
            .collect::<Vec<_>>();
        if owners.len() != 1 {
            return Err(CorpusError::Verification(format!(
                "profile override {name} requires exactly one selected source owner, found {}",
                owners.len()
            )));
        }
        let owner = owners[0];
        let provider = owner
            .group
            .providers
            .iter()
            .find(|provider| provider.normalized_name == *name)
            .expect("owner was selected by provider name");
        let source = profile_source_path(name);
        let bytes = fs::read(&source).map_err(|error| {
            CorpusError::Verification(format!(
                "missing profile compatibility provider {}: {error}",
                source.display()
            ))
        })?;
        let raw_sha256 = hash::bytes(&bytes);
        let relative_path = format!(
            "native/profile/gnu-mingw-x86_64/{}.f",
            name.to_ascii_lowercase()
        );
        let object_id = stable_id(
            "native-profile-object",
            &[&compiler.profile_id, name, &raw_sha256],
        );
        let object = workspace.join("objects").join(format!("{object_id}.o"));
        let log = workspace
            .join("logs")
            .join(format!("{object_id}.compile.log"));
        let result = Command::new(&compiler_path)
            .args(FORTRAN_FLAGS)
            .arg(&source)
            .arg("-o")
            .arg(&object)
            .output()
            .map_err(|error| {
                CorpusError::Verification(format!(
                    "could not compile profile override {name}: {error}"
                ))
            })?;
        fs::write(&log, [&result.stdout[..], &result.stderr[..]].concat())?;
        if !result.status.success() || !object.is_file() {
            return Err(CorpusError::Verification(format!(
                "profile override {name} did not compile; see {}",
                log.display()
            )));
        }
        let symbols = inspect_symbols(
            &compiler_path,
            &object,
            &owner.source_id,
            workspace,
            &object_id,
        )?;
        let owned_symbols = symbols
            .iter()
            .filter(|symbol| symbol.normalized == *name)
            .count();
        if owned_symbols != 1 {
            return Err(CorpusError::Verification(format!(
                "profile override {name} defines {owned_symbols} matching symbols, expected one"
            )));
        }
        overrides.push(ProfileOverride {
            normalized_name: (*name).to_owned(),
            original_provider_id: provider.program_unit_id.clone(),
            original_source_id: owner.source_id.clone(),
            original_raw_sha256: owner.group.raw_sha256.clone(),
            relative_path,
            raw_sha256,
            object_path: object,
            object_id,
            symbols,
        });
    }
    overrides.sort_by(|left, right| left.normalized_name.cmp(&right.normalized_name));
    Ok(overrides)
}

fn compiler_path() -> PathBuf {
    std::env::var_os("SLATEC_FORTRAN_COMPILER")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("x86_64-w64-mingw32-gfortran"))
}

fn compile_group(
    evidence_dir: &Path,
    snapshot: &str,
    main_src_snapshot: &str,
    group: &SourceGroup,
    compiler: &CompilerProfile,
    workspace: &Path,
) -> Result<CompileRecord> {
    let source_id = source_group_id(snapshot, group);
    let source = source_path(evidence_dir, main_src_snapshot, group)?;
    let bytes = fs::read(&source).map_err(|_| {
        CorpusError::Verification(format!(
            "missing selected source evidence {}",
            source.display()
        ))
    })?;
    if hash::bytes(&bytes) != group.raw_sha256 {
        return Err(CorpusError::Verification(format!(
            "raw SHA-256 mismatch for selected source {}:{}",
            group.subset, group.path
        )));
    }
    let object_id = stable_id(
        "native-object",
        &[&compiler.profile_id, &source_id, &group.raw_sha256],
    );
    let object = workspace.join("objects").join(format!("{object_id}.o"));
    let log = workspace
        .join("logs")
        .join(format!("{object_id}.compile.log"));
    let compiler_path = compiler_path();
    let diagnostic_log_id = stable_id("native-compiler-log", &[&compiler.profile_id, &source_id]);
    if object.is_file() {
        let symbol_log = workspace.join("logs").join(format!("{object_id}.nm.log"));
        let symbols = if symbol_log.is_file() {
            symbols_from_nm(&fs::read(&symbol_log)?, &source_id)
        } else {
            inspect_symbols(&compiler_path, &object, &source_id, workspace, &object_id)?
        };
        return Ok(CompileRecord {
            source_id: source_id.clone(),
            group: group.clone(),
            status: "compiled".to_owned(),
            exit_code: Some(0),
            failure_rule: None,
            object_path: Some(object.clone()),
            object_id: Some(object_id.clone()),
            diagnostic_log_id,
            symbols,
        });
    }
    let result = Command::new(&compiler_path)
        .args(FORTRAN_FLAGS)
        .arg(&source)
        .arg("-o")
        .arg(&object)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not start GNU Fortran compiler: {error}"))
        })?;
    fs::write(&log, [&result.stdout[..], &result.stderr[..]].concat())?;
    if !result.status.success() || !object.is_file() {
        return Ok(CompileRecord {
            source_id,
            group: group.clone(),
            status: "failed".to_owned(),
            exit_code: result.status.code(),
            failure_rule: Some("compiler_exit_nonzero".to_owned()),
            object_path: None,
            object_id: None,
            diagnostic_log_id,
            symbols: Vec::new(),
        });
    }
    let symbols = inspect_symbols(&compiler_path, &object, &source_id, workspace, &object_id)?;
    Ok(CompileRecord {
        source_id,
        group: group.clone(),
        status: "compiled".to_owned(),
        exit_code: result.status.code(),
        failure_rule: None,
        object_path: Some(object),
        object_id: Some(object_id),
        diagnostic_log_id,
        symbols,
    })
}

fn source_path(
    evidence_dir: &Path,
    main_src_snapshot: &str,
    group: &SourceGroup,
) -> Result<PathBuf> {
    match group.subset.as_str() {
        "main-src" => Ok(evidence_dir
            .join("extracted")
            .join(main_src_snapshot)
            .join("slatec-source-archive")
            .join(&group.path)),
        "lin" | "fishfft" | "fnlib" | "pchip" => Ok(evidence_dir
            .join("full-corpus/audit-input/directories")
            .join(&group.subset)
            .join("files")
            .join(&group.path)),
        "spfun" => Ok(evidence_dir
            .join("full-corpus/audit-input/supplemental")
            .join("spfun")),
        _ => Err(CorpusError::Verification(format!(
            "no evidence-path rule for selected subset {}",
            group.subset
        ))),
    }
}

fn source_group_id(snapshot: &str, group: &SourceGroup) -> String {
    stable_id(
        "selected-source",
        &[snapshot, &group.subset, &group.path, &group.raw_sha256],
    )
}

fn source_id(snapshot: &str, provider: &Provider) -> String {
    stable_id(
        "selected-source",
        &[
            snapshot,
            &provider.source_subset,
            &provider.source_path,
            &provider.raw_sha256,
        ],
    )
}

fn inspect_symbols(
    compiler: &Path,
    object: &Path,
    source_id: &str,
    workspace: &Path,
    object_id: &str,
) -> Result<Vec<Symbol>> {
    let nm = compiler_tool(compiler, "nm")?;
    let output = Command::new(nm)
        .args(["-g", "--defined-only"])
        .arg(object)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not inspect native symbols: {error}"))
        })?;
    fs::write(
        workspace.join("logs").join(format!("{object_id}.nm.log")),
        [&output.stdout[..], &output.stderr[..]].concat(),
    )?;
    if !output.status.success() {
        return Err(CorpusError::Verification(format!(
            "symbol inspection failed for native object {object_id}"
        )));
    }
    Ok(symbols_from_nm(&output.stdout, source_id))
}

fn symbols_from_nm(bytes: &[u8], source_id: &str) -> Vec<Symbol> {
    String::from_utf8_lossy(bytes)
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let _address = fields.next()?;
            let kind = fields.next()?.to_owned();
            let raw = fields.next()?.to_owned();
            Some(Symbol {
                source_id: source_id.to_owned(),
                normalized: normalize_symbol(&raw),
                raw,
                kind,
            })
        })
        .collect()
}

fn compiler_tool(compiler: &Path, tool: &str) -> Result<PathBuf> {
    let output = Command::new(compiler)
        .arg(format!("-print-prog-name={tool}"))
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not locate compiler tool {tool}: {error}"))
        })?;
    let path = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if !output.status.success() || path.is_empty() {
        return Err(CorpusError::Verification(format!(
            "compiler could not locate required tool {tool}"
        )));
    }
    Ok(PathBuf::from(path))
}

fn command_output(program: &Path, arguments: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not run {}: {error}", program.display()))
        })?;
    if !output.status.success() {
        return Err(CorpusError::Verification(format!(
            "{} {:?} failed",
            program.display(),
            arguments
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_owned())
}

fn normalize_symbol(raw: &str) -> String {
    raw.trim_start_matches('_')
        .trim_end_matches('_')
        .to_ascii_uppercase()
}

fn archive_objects(
    workspace: &Path,
    compilation: &[CompileRecord],
    overrides: &[ProfileOverride],
) -> Result<Option<PathBuf>> {
    let original_objects = compilation
        .iter()
        .filter_map(|record| record.object_path.as_ref())
        .collect::<Vec<_>>();
    if original_objects.is_empty() {
        return Ok(None);
    }
    write_archive(
        workspace,
        "libslatec_selected_original.a",
        &original_objects,
    )?;
    let replaced = overrides
        .iter()
        .map(|record| record.original_source_id.as_str())
        .collect::<BTreeSet<_>>();
    let mut profile_objects = compilation
        .iter()
        .filter(|record| !replaced.contains(record.source_id.as_str()))
        .filter_map(|record| record.object_path.as_ref())
        .collect::<Vec<_>>();
    profile_objects.extend(overrides.iter().map(|record| &record.object_path));
    let archive = write_archive(workspace, "libslatec_selected.a", &profile_objects)?;
    validate_override_symbol_ownership(&archive, overrides)?;
    Ok(Some(archive))
}

fn write_archive(workspace: &Path, name: &str, objects: &[&PathBuf]) -> Result<PathBuf> {
    let compiler_path = compiler_path();
    let ar = compiler_tool(&compiler_path, "ar")?;
    let archive = workspace.join(name);
    if archive.exists() {
        fs::remove_file(&archive)?;
    }
    for (index, chunk) in objects.chunks(96).enumerate() {
        let mode = if index == 0 { "rcsD" } else { "rD" };
        let output = Command::new(&ar)
            .arg(mode)
            .arg(&archive)
            .args(chunk)
            .output()
            .map_err(|error| {
                CorpusError::Verification(format!("could not start archiver: {error}"))
            })?;
        fs::write(
            workspace
                .join("logs")
                .join(format!("{}-{index}.log", name.trim_end_matches(".a"))),
            [&output.stdout[..], &output.stderr[..]].concat(),
        )?;
        if !output.status.success() {
            return Err(CorpusError::Verification(format!(
                "archiver failed while creating {name}"
            )));
        }
    }
    if !archive.is_file() {
        return Err(CorpusError::Verification(format!(
            "archiver did not create {name}"
        )));
    }
    Ok(archive)
}

fn validate_override_symbol_ownership(archive: &Path, overrides: &[ProfileOverride]) -> Result<()> {
    let nm = compiler_tool(&compiler_path(), "nm")?;
    let output = Command::new(nm)
        .args(["-g", "--defined-only"])
        .arg(archive)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!(
                "could not inspect profile archive symbols: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(CorpusError::Verification(
            "profile archive symbol inspection failed".to_owned(),
        ));
    }
    let mut counts = BTreeMap::<String, usize>::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Some(raw) = line.split_whitespace().last() {
            *counts.entry(normalize_symbol(raw)).or_default() += 1;
        }
    }
    ensure_single_override_counts(
        &counts,
        &overrides
            .iter()
            .map(|record| record.normalized_name.as_str())
            .collect::<Vec<_>>(),
    )
}

fn ensure_single_override_counts(counts: &BTreeMap<String, usize>, names: &[&str]) -> Result<()> {
    for name in names {
        let count = counts.get(*name).copied().unwrap_or(0);
        if count != 1 {
            return Err(CorpusError::Verification(format!(
                "profile archive contains {count} definitions of {name}, expected exactly one"
            )));
        }
    }
    Ok(())
}

fn compilation_by_provider(records: &[CompileRecord]) -> BTreeMap<String, bool> {
    let mut result = BTreeMap::new();
    for record in records {
        for provider in &record.group.providers {
            result.insert(
                provider.program_unit_id.clone(),
                record.status == "compiled",
            );
        }
    }
    result
}

fn providers_from_compilation(
    groups: BTreeMap<String, SourceGroup>,
    facts: &BTreeMap<String, UnitFacts>,
    observed: &BTreeMap<String, Vec<String>>,
    compiled: &BTreeMap<String, bool>,
    abi: &AbiValidation,
) -> Result<Vec<InterfaceRecord>> {
    let mut records = Vec::new();
    for group in groups.into_values() {
        for provider in group.providers {
            let facts = facts
                .get(&provider.program_unit_id)
                .cloned()
                .ok_or_else(|| {
                    CorpusError::Verification(format!(
                        "selected provider {} has no interface inventory record",
                        provider.program_unit_id
                    ))
                })?;
            if provider.kind != facts.kind {
                return Err(CorpusError::Verification(format!(
                    "selected provider {} kind disagrees with interface inventory",
                    provider.program_unit_id
                )));
            }
            let mut diagnostic_ids = facts.diagnostic_ids.clone();
            let candidate_symbols = observed
                .get(&provider.normalized_name)
                .cloned()
                .unwrap_or_default();
            let observed_symbol =
                (candidate_symbols.len() == 1).then(|| candidate_symbols[0].clone());
            let (confidence, batch, mut rule_ids) = classify(
                &provider,
                &facts,
                compiled
                    .get(&provider.program_unit_id)
                    .copied()
                    .unwrap_or(false),
                observed_symbol.as_deref(),
                candidate_symbols.len(),
                abi,
            );
            for rule in rule_ids.drain(..) {
                diagnostic_ids.push(stable_id(
                    "raw-ffi-diagnostic",
                    &[&provider.program_unit_id, &rule],
                ));
            }
            diagnostic_ids.sort();
            diagnostic_ids.dedup();
            let review_state = if confidence == "generated_standard" {
                "machine_checked"
            } else if confidence == "generated_abi_sensitive" {
                "abi_profile_checked"
            } else {
                "needs_review"
            }
            .to_owned();
            records.push(InterfaceRecord {
                provider,
                facts,
                observed_symbol,
                confidence,
                batch,
                review_state,
                diagnostic_ids,
            });
        }
    }
    Ok(records)
}

fn classify(
    provider: &Provider,
    facts: &UnitFacts,
    source_compiled: bool,
    observed_symbol: Option<&str>,
    observed_symbol_count: usize,
    abi: &AbiValidation,
) -> (String, Option<String>, Vec<String>) {
    if provider.selection_category == "selected_infrastructure_unit" {
        return (
            "non_callable_infrastructure".to_owned(),
            Some("batch_infrastructure".to_owned()),
            Vec::new(),
        );
    }
    if !source_compiled {
        return (
            "unsupported".to_owned(),
            None,
            vec!["RAW-FFI-COMPILE-FAILED".to_owned()],
        );
    }
    if observed_symbol_count == 0 || observed_symbol.is_none() {
        return (
            "manual_review_required".to_owned(),
            None,
            vec![if observed_symbol_count == 0 {
                "RAW-FFI-SYMBOL-MISSING".to_owned()
            } else {
                "RAW-FFI-SYMBOL-AMBIGUOUS".to_owned()
            }],
        );
    }
    if !basic_facts_known(facts) {
        return (
            "manual_review_required".to_owned(),
            None,
            vec!["RAW-FFI-UNRESOLVED-INTERFACE".to_owned()],
        );
    }
    if facts
        .arguments
        .iter()
        .any(|argument| argument.is_external || argument.is_intrinsic)
    {
        return (
            "manual_review_required".to_owned(),
            Some("batch_callbacks".to_owned()),
            vec!["RAW-FFI-PROCEDURE-ARGUMENT".to_owned()],
        );
    }
    let result_type = facts
        .function_result
        .as_ref()
        .and_then(|result| result.declared_type.as_deref());
    if matches!(
        result_type,
        Some("COMPLEX") | Some("DOUBLE COMPLEX") | Some("CHARACTER")
    ) {
        return (
            "manual_review_required".to_owned(),
            None,
            vec!["RAW-FFI-RETURN-ABI-UNVALIDATED".to_owned()],
        );
    }
    if abi.status != "passed" {
        return (
            "manual_review_required".to_owned(),
            None,
            vec!["RAW-FFI-PROFILE-UNVALIDATED".to_owned()],
        );
    }
    if facts
        .arguments
        .iter()
        .any(|argument| matches!(argument.declared_type.as_deref(), Some("CHARACTER")))
    {
        if abi.character != "passed" {
            return manual_profile_review("RAW-FFI-CHARACTER-ABI-UNVALIDATED");
        }
        return (
            "generated_abi_sensitive".to_owned(),
            Some("batch_character".to_owned()),
            Vec::new(),
        );
    }
    if facts.arguments.iter().any(|argument| {
        matches!(
            argument.declared_type.as_deref(),
            Some("COMPLEX") | Some("DOUBLE COMPLEX")
        )
    }) {
        if abi.complex != "passed" || abi.selected_complex_rust != "passed" {
            return manual_profile_review("RAW-FFI-COMPLEX-BATCH-UNVALIDATED");
        }
        return (
            "generated_abi_sensitive".to_owned(),
            Some("batch_complex_arguments".to_owned()),
            Vec::new(),
        );
    }
    if facts
        .arguments
        .iter()
        .any(|argument| matches!(argument.declared_type.as_deref(), Some("LOGICAL")))
        || result_type == Some("LOGICAL")
    {
        if abi.logical != "passed" {
            return manual_profile_review("RAW-FFI-LOGICAL-ABI-UNVALIDATED");
        }
        return (
            "generated_abi_sensitive".to_owned(),
            Some("batch_logical".to_owned()),
            Vec::new(),
        );
    }
    if provider.kind == "function" {
        if abi.integer_real_double != "passed" || abi.selected_numeric_rust != "passed" {
            return manual_profile_review("RAW-FFI-SCALAR-FUNCTION-BATCH-UNVALIDATED");
        }
        return (
            "generated_abi_sensitive".to_owned(),
            Some("batch_scalar_functions".to_owned()),
            Vec::new(),
        );
    }
    if abi.selected_numeric_rust != "passed" {
        return manual_profile_review("RAW-FFI-NUMERIC-BATCH-UNVALIDATED");
    }
    let batch = if facts
        .arguments
        .iter()
        .any(|argument| !argument.dimensions.is_empty())
    {
        "batch_numeric_array_subroutines"
    } else {
        "batch_numeric_scalar_subroutines"
    };
    (
        "generated_standard".to_owned(),
        Some(batch.to_owned()),
        Vec::new(),
    )
}

fn manual_profile_review(rule: &str) -> (String, Option<String>, Vec<String>) {
    (
        "manual_review_required".to_owned(),
        None,
        vec![rule.to_owned()],
    )
}

fn basic_facts_known(facts: &UnitFacts) -> bool {
    facts.arguments.iter().all(|argument| {
        known_type(argument.declared_type.as_deref())
            && argument.type_source != "unknown"
            && argument.type_source != "conflicting"
            && !has_conflict(argument.conflict_state.as_deref())
            && (argument.declared_type.as_deref() != Some("CHARACTER")
                || argument.character_length.is_some())
            && (!argument.locator_ids.is_empty() || facts.declaration_locator_id.is_some())
    }) && match facts.kind.as_str() {
        "subroutine" => true,
        "function" => facts.function_result.as_ref().is_some_and(|result| {
            known_type(result.declared_type.as_deref())
                && result.type_source != "unknown"
                && result.type_source != "conflicting"
                && !has_conflict(result.conflict_state.as_deref())
                && (result.declared_type.as_deref() != Some("CHARACTER")
                    || result.character_length.is_some())
                && (result.locator_id.is_some() || facts.declaration_locator_id.is_some())
        }),
        _ => false,
    }
}

fn has_conflict(value: Option<&str>) -> bool {
    value.is_some_and(|value| !value.is_empty() && value != "none")
}

fn known_type(value: Option<&str>) -> bool {
    matches!(
        value,
        Some("INTEGER")
            | Some("REAL")
            | Some("DOUBLE PRECISION")
            | Some("COMPLEX")
            | Some("DOUBLE COMPLEX")
            | Some("LOGICAL")
            | Some("CHARACTER")
    )
}

fn summarize(
    interfaces: &[InterfaceRecord],
    compilation: &[CompileRecord],
    observed_symbols: usize,
) -> RawFfiSummary {
    let mut summary = RawFfiSummary {
        selected_program_units: interfaces.len(),
        selected_source_files: compilation.len(),
        compiled_source_files: compilation
            .iter()
            .filter(|record| record.status == "compiled")
            .count(),
        failed_source_files: compilation
            .iter()
            .filter(|record| record.status != "compiled")
            .count(),
        observed_defined_symbols: observed_symbols,
        ..RawFfiSummary::default()
    };
    for record in interfaces {
        match record.confidence.as_str() {
            "generated_standard" => summary.generated_standard += 1,
            "generated_abi_sensitive" => summary.generated_abi_sensitive += 1,
            "manual_review_required" => summary.manual_review_required += 1,
            "unsupported" => summary.unsupported += 1,
            "non_callable_infrastructure" => summary.non_callable_infrastructure += 1,
            _ => {}
        }
    }
    summary.generated_bindings = summary.generated_standard + summary.generated_abi_sensitive;
    summary
}

#[allow(clippy::too_many_arguments)]
fn outputs(
    snapshot: &str,
    compiler: &CompilerProfile,
    compilation: &[CompileRecord],
    symbols: &[Symbol],
    interfaces: &[InterfaceRecord],
    abi: &AbiValidation,
    summary: &RawFfiSummary,
    overrides: &[ProfileOverride],
) -> Result<BTreeMap<&'static str, Vec<u8>>> {
    let mut outputs = BTreeMap::new();
    let mut source_rows = compilation
        .iter()
        .map(|record| {
            json!([
                record.source_id,
                record.group.subset,
                record.group.path,
                record.group.raw_sha256,
                record.group.normalized_sha256,
                record
                    .group
                    .providers
                    .iter()
                    .map(|provider| provider.program_unit_id.clone())
                    .collect::<Vec<_>>(),
            ])
        })
        .collect::<Vec<_>>();
    source_rows.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    outputs.insert(
        "selected-source-files.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-selected-source","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"columns":["source_id","source_subset","source_path","raw_sha256","normalized_sha256","program_unit_ids"],"records":source_rows}))?,
    );
    let mut compilation_rows = compilation
        .iter()
        .map(|record| {
            json!([
                record.source_id,
                record.group.subset,
                record.group.path,
                record.status,
                record.exit_code,
                record.failure_rule,
                record.object_id,
                record.diagnostic_log_id,
                record.symbols.len(),
            ])
        })
        .collect::<Vec<_>>();
    compilation_rows.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    outputs.insert(
        "compilation-results.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-compilation","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"compiler_profile":{"id":compiler.profile_id,"identity":compiler.identity,"version":compiler.version,"target":compiler.target,"flags":FORTRAN_FLAGS,"default_integer_and_logical":"validated by ignored ABI probe when abi_validation.status is passed","symbol_mangling":"raw spellings are observed by nm; generated declarations use those exact spellings","object_format":"compiler-target dependent; preserved in ignored build evidence"},"columns":["source_id","source_subset","source_path","status","exit_code","failure_rule","object_id","diagnostic_log_id","defined_symbol_count"],"records":compilation_rows}))?,
    );
    let override_rows = overrides
        .iter()
        .map(|record| {
            json!([
                record.normalized_name,
                record.original_provider_id,
                record.original_source_id,
                record.original_raw_sha256,
                record.relative_path,
                record.raw_sha256,
                record.object_id,
                record
                    .symbols
                    .iter()
                    .map(|symbol| symbol.raw.clone())
                    .collect::<Vec<_>>(),
                "profile_contract_from_fortran_intrinsics_or_site_hook",
                "validated_single_symbol_owner",
            ])
        })
        .collect::<Vec<_>>();
    outputs.insert(
        "profile-overrides.json",
        compact(&json!({
            "schema_id":"slatec-rs/raw-ffi-profile-overrides",
            "schema_version":SCHEMA_VERSION,
            "snapshot_id":snapshot,
            "compiler_profile_id":compiler.profile_id,
            "columns":["normalized_name","original_provider_id","original_source_id","original_raw_sha256","override_path","override_raw_sha256","object_id","defined_symbols","selection_rule","validation_result"],
            "records":override_rows
        }))?,
    );
    let mut symbol_rows = symbols
        .iter()
        .map(|symbol| json!([symbol.source_id, symbol.raw, symbol.kind, symbol.normalized]))
        .collect::<Vec<_>>();
    symbol_rows.sort_by(|left, right| {
        left[0]
            .as_str()
            .cmp(&right[0].as_str())
            .then(left[1].as_str().cmp(&right[1].as_str()))
    });
    outputs.insert(
        "symbol-inventory.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-symbol","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"normalization_rule":"remove leading and trailing underscore characters only for provider matching; raw spelling is authoritative","columns":["source_id","raw_symbol","nm_kind","normalized_symbol"],"records":symbol_rows}))?,
    );
    let mut interface_rows = interfaces
        .iter()
        .map(|record| {
            json!([
                record.provider.program_unit_id,
                record.provider.normalized_name,
                record.provider.kind,
                record.provider.source_subset,
                record.provider.source_path,
                record.provider.raw_sha256,
                record.facts.declaration_locator_id,
                record
                    .facts
                    .arguments
                    .iter()
                    .map(|argument| argument.id.clone())
                    .collect::<Vec<_>>(),
                record
                    .facts
                    .function_result
                    .as_ref()
                    .map(|result| result.id.clone()),
                record.observed_symbol,
                record.confidence,
                record.batch,
                record.review_state,
                record.diagnostic_ids,
            ])
        })
        .collect::<Vec<_>>();
    interface_rows.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    outputs.insert(
        "interface-inventory.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-interface","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"source_interface_inventory":"generated/ffi-inventory","columns":["program_unit_id","normalized_name","kind","source_subset","source_path","raw_sha256","declaration_locator_id","argument_ids","function_result_id","observed_raw_symbol","confidence_class","binding_batch","review_state","diagnostic_ids"],"records":interface_rows}))?,
    );
    let mut review_rows = interfaces
        .iter()
        .filter(|record| {
            !matches!(
                record.confidence.as_str(),
                "generated_standard" | "generated_abi_sensitive"
            )
        })
        .map(|record| {
            json!([
                record.provider.program_unit_id,
                record.provider.normalized_name,
                record.confidence,
                record.batch,
                record.diagnostic_ids,
                record.facts.declaration_locator_id,
            ])
        })
        .collect::<Vec<_>>();
    review_rows.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    outputs.insert(
        "review-queue.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-review-queue","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"columns":["program_unit_id","normalized_name","confidence_class","batch","diagnostic_ids","declaration_locator_id"],"records":review_rows}))?,
    );
    let mut diagnostic_rows = BTreeSet::new();
    for record in interfaces {
        for id in &record.diagnostic_ids {
            diagnostic_rows.insert((id.clone(), record.provider.program_unit_id.clone()));
        }
    }
    outputs.insert(
        "diagnostics.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-diagnostic","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"columns":["id","program_unit_id"],"records":diagnostic_rows.into_iter().map(|(id,unit)|json!([id,unit])).collect::<Vec<_>>() }))?,
    );
    outputs.insert(
        "confidence-summary.json",
        compact(&json!({"schema_id":"slatec-rs/raw-ffi-confidence-summary","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"summary":summary,"profile_validation":{"abi_validated":abi.status,"machine_constants_validated":"validated_by_generated_runtime_profile_outputs","legacy_error_behavior_validated":"validated_by_generated_runtime_profile_outputs","fnlib_initialization_validated":"validated_by_generated_runtime_profile_outputs"},"abi_validation":{"status":abi.status,"integer_real_double":abi.integer_real_double,"logical":abi.logical,"complex":abi.complex,"character":abi.character,"callback":abi.callback,"selected_numeric_rust":abi.selected_numeric_rust,"selected_complex_rust":abi.selected_complex_rust,"selected_machine_error":abi.selected_machine_error,"selected_machine_error_scope":"ABI smoke only; numerical machine constants and legacy error levels are validated separately under generated/runtime-profile"},"confidence_classes":{"generated_standard":"numeric subroutine with observed symbol and validated basic GNU Fortran ABI","generated_abi_sensitive":"observed symbol and validated profile; module identifies the ABI-sensitive category","manual_review_required":"callback, unsupported return ABI, unresolved interface, or symbol ambiguity","unsupported":"selected source did not compile under this explicit profile","non_callable_infrastructure":"selected infrastructure unit, retained as corpus evidence but not emitted as a callable raw binding"}}))?,
    );
    outputs.insert(
        "generation-summary.md",
        format!("# Complete selected-corpus raw FFI generation\n\n- Snapshot: `{snapshot}`\n- GNU Fortran profile: `{}` (`{}`)\n- Selected program units: {} from {} physical sources\n- Sources compiled: {}; failures: {}\n- Observed defined symbols: {}\n- Generated standard bindings: {}; ABI-sensitive bindings: {}\n- Manual review: {}; unsupported: {}; non-callable infrastructure: {}\n- ABI profile validation: `{}`\n\nRaw declarations are generated only for an observed compiled symbol with a supported, explicitly validated ABI class. Compiler logs, objects, archives, and source bytes remain ignored evidence. This is not a safe API and does not establish ABI correctness for gated interfaces.\n", compiler.identity, compiler.target, summary.selected_program_units, summary.selected_source_files, summary.compiled_source_files, summary.failed_source_files, summary.observed_defined_symbols, summary.generated_standard, summary.generated_abi_sensitive, summary.manual_review_required, summary.unsupported, summary.non_callable_infrastructure, abi.status).into_bytes(),
    );
    Ok(outputs)
}

fn inventory_semantic_hash(inventory_dir: &Path) -> Result<String> {
    let manifest: Value = read_json(&inventory_dir.join("manifest.json"))?;
    manifest["output_semantic_hash"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| {
            CorpusError::Verification("FFI inventory manifest lacks semantic hash".to_owned())
        })
}

fn enforce_size(outputs: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let total = outputs
        .values()
        .map(|bytes| bytes.len() as u64)
        .sum::<u64>();
    if total > COMMITTED_SIZE_LIMIT {
        let sizes = outputs
            .iter()
            .map(|(name, bytes)| format!("{name}={}", bytes.len()))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(CorpusError::Verification(format!(
            "raw FFI committed output would be {total} bytes, exceeding 4 MB ({sizes})"
        )));
    }
    Ok(())
}

fn generate_bindings(
    bindings_dir: &Path,
    snapshot: &str,
    compiler: &CompilerProfile,
    interfaces: &[InterfaceRecord],
) -> Result<()> {
    let parent = bindings_dir.parent().ok_or_else(|| {
        CorpusError::Policy("generated Rust bindings directory must have a parent".to_owned())
    })?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        "{}.staging-{snapshot}",
        bindings_dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    ));
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;
    let batches = [
        "batch_numeric_scalar_subroutines",
        "batch_numeric_array_subroutines",
        "batch_scalar_functions",
        "batch_complex_arguments",
        "batch_logical",
        "batch_character",
        "batch_callbacks",
        "batch_infrastructure",
    ];
    let mut modules = BTreeMap::<&str, Vec<&InterfaceRecord>>::new();
    for batch in batches {
        modules.insert(batch, Vec::new());
    }
    for record in interfaces {
        if matches!(
            record.confidence.as_str(),
            "generated_standard" | "generated_abi_sensitive"
        ) {
            if let Some(batch) = record.batch.as_deref() {
                modules.entry(batch).or_default().push(record);
            }
        }
    }
    let mut module_index = String::from(
        "//! Generated raw GNU Fortran declarations for the selected SLATEC corpus.\n//!\n//! Do not edit. Regenerate with `slatec-corpus generate-raw-ffi --offline`.\n\n",
    );
    for batch in batches {
        let file_stem = batch.trim_start_matches("batch_");
        let feature = file_stem.replace('_', "-");
        module_index.push_str(&format!(
            "#[cfg(feature = \"raw-ffi-{feature}\")]\npub mod {file_stem};\n"
        ));
        let mut records = modules.remove(batch).unwrap_or_default();
        records.sort_by(|left, right| {
            left.provider
                .normalized_name
                .cmp(&right.provider.normalized_name)
                .then(
                    left.provider
                        .program_unit_id
                        .cmp(&right.provider.program_unit_id),
                )
        });
        let content = binding_module(batch, snapshot, compiler, &records)?;
        fs::write(staging.join(format!("{file_stem}.rs")), content)?;
    }
    fs::write(staging.join("mod.rs"), module_index)?;
    format_generated_bindings(&staging)?;
    if bindings_dir.exists() {
        fs::remove_dir_all(bindings_dir)?;
    }
    fs::rename(staging, bindings_dir)?;
    Ok(())
}

fn format_generated_bindings(directory: &Path) -> Result<()> {
    let formatter = std::env::var_os("RUSTFMT").unwrap_or_else(|| "rustfmt".into());
    let mut files = fs::read_dir(directory)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .collect::<Vec<_>>();
    files.sort();
    let output = Command::new(formatter)
        .args(["--edition", "2024"])
        .args(&files)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!(
                "could not run rustfmt for generated bindings: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(CorpusError::Verification(
            "rustfmt rejected generated raw FFI bindings".to_owned(),
        ));
    }
    Ok(())
}

fn binding_module(
    batch: &str,
    snapshot: &str,
    compiler: &CompilerProfile,
    records: &[&InterfaceRecord],
) -> Result<String> {
    if records.is_empty() {
        return Ok(format!(
            "//! Generated raw declarations for `{batch}`.\n//! Snapshot: `{snapshot}`. GNU Fortran target: `{}`.\n//!\n//! No callable declarations are enabled for this batch.\n",
            compiler.target
        ));
    }
    let mut output = format!(
        "//! Generated raw declarations for `{batch}`.\n//! Snapshot: `{snapshot}`. GNU Fortran target: `{}`.\n//!\n//! Every declaration is unsafe: it exposes the compiler-observed raw ABI only.\n#![allow(clippy::missing_safety_doc, unused_imports)]\n\nuse crate::{{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical}};\nuse core::ffi::c_char;\n\nunsafe extern \"C\" {{\n",
        compiler.target
    );
    for record in records {
        let symbol = record.observed_symbol.as_ref().ok_or_else(|| {
            CorpusError::Verification(
                "attempted to generate a binding without observed symbol".to_owned(),
            )
        })?;
        let rust_name = rust_identifier(&record.provider.normalized_name);
        let mut parameters = Vec::new();
        let mut character_arguments = 0_usize;
        for argument in &record.facts.arguments {
            let raw_type = rust_argument_type(argument)?;
            parameters.push(format!(
                "{}: *mut {raw_type}",
                rust_identifier(&argument.name)
            ));
            if argument.declared_type.as_deref() == Some("CHARACTER") {
                character_arguments += 1;
            }
        }
        for index in 0..character_arguments {
            parameters.push(format!(
                "character_length_{}: FortranCharacterLength",
                index + 1
            ));
        }
        let result = if record.provider.kind == "function" {
            format!(
                " -> {}",
                rust_return_type(record.facts.function_result.as_ref())?
            )
        } else {
            String::new()
        };
        output.push_str(&format!(
            "    #[link_name = \"{symbol}\"]\n    pub fn {rust_name}({}){result};\n",
            parameters.join(", ")
        ));
    }
    output.push_str("}\n");
    Ok(output)
}

fn rust_argument_type(argument: &ArgumentFact) -> Result<&'static str> {
    match argument.declared_type.as_deref() {
        Some("INTEGER") => Ok("FortranInteger"),
        Some("REAL") => Ok("f32"),
        Some("DOUBLE PRECISION") => Ok("f64"),
        Some("COMPLEX") => Ok("Complex32"),
        Some("DOUBLE COMPLEX") => Ok("Complex64"),
        Some("LOGICAL") => Ok("FortranLogical"),
        Some("CHARACTER") => Ok("c_char"),
        _ => Err(CorpusError::Verification(
            "attempted to generate a binding for an unsupported argument type".to_owned(),
        )),
    }
}

fn rust_return_type(result: Option<&ResultFact>) -> Result<&'static str> {
    match result.and_then(|result| result.declared_type.as_deref()) {
        Some("INTEGER") => Ok("FortranInteger"),
        Some("REAL") => Ok("f32"),
        Some("DOUBLE PRECISION") => Ok("f64"),
        Some("LOGICAL") => Ok("FortranLogical"),
        _ => Err(CorpusError::Verification(
            "attempted to generate a binding for an unsupported function result".to_owned(),
        )),
    }
}

fn rust_identifier(name: &str) -> String {
    let lowered = name.to_ascii_lowercase();
    match lowered.as_str() {
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move"
        | "mut" | "pub" | "ref" | "return" | "self" | "static" | "struct" | "super" | "trait"
        | "true" | "type" | "unsafe" | "use" | "where" | "while" | "async" | "await" | "dyn"
        | "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv"
        | "typeof" | "unsized" | "virtual" | "yield" | "try" => format!("r#{lowered}"),
        _ => lowered,
    }
}

fn run_abi_validation(
    compiler: &CompilerProfile,
    workspace: &Path,
    archive: Option<&Path>,
) -> Result<AbiValidation> {
    let compiler_path = compiler_path();
    let fortran_source = workspace.join("abi_profile_probe.f");
    let object = workspace.join("abi_profile_probe.o");
    let compile_log = workspace.join("logs/abi_profile_probe.compile.log");
    fs::write(&fortran_source, abi_probe_fortran())?;
    let compile = Command::new(&compiler_path)
        .args(FORTRAN_FLAGS)
        .arg(&fortran_source)
        .arg("-o")
        .arg(&object)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not compile ABI probe: {error}"))
        })?;
    fs::write(
        &compile_log,
        [&compile.stdout[..], &compile.stderr[..]].concat(),
    )?;
    if !compile.status.success() || !object.is_file() {
        return Ok(AbiValidation {
            status: "compile_failed".to_owned(),
            ..AbiValidation::default()
        });
    }
    let basic = run_rust_probe(
        &compiler_path,
        workspace,
        &object,
        None,
        "abi_profile",
        abi_probe_rust(),
    )?;
    let mut validation = AbiValidation {
        status: basic.clone(),
        integer_real_double: basic.clone(),
        logical: basic.clone(),
        complex: basic.clone(),
        character: basic.clone(),
        callback: basic,
        selected_numeric_rust: "not_run".to_owned(),
        selected_complex_rust: "not_run".to_owned(),
        selected_machine_error: "not_run".to_owned(),
    };
    if let Some(archive) = archive {
        validation.selected_numeric_rust = run_rust_probe(
            &compiler_path,
            workspace,
            &object,
            Some(archive),
            "selected_numeric",
            selected_numeric_rust(),
        )?;
        validation.selected_complex_rust = run_rust_probe(
            &compiler_path,
            workspace,
            &object,
            Some(archive),
            "selected_complex",
            selected_complex_rust(),
        )?;
        validation.selected_machine_error =
            run_fortran_runtime_probe(&compiler_path, workspace, archive)?;
    }
    let _ = compiler;
    Ok(validation)
}

fn run_rust_probe(
    compiler: &Path,
    workspace: &Path,
    abi_object: &Path,
    archive: Option<&Path>,
    name: &str,
    source: String,
) -> Result<String> {
    let rust_source = workspace.join(format!("{name}.rs"));
    let executable = workspace.join(format!("{name}.exe"));
    let compile_log = workspace.join("logs").join(format!("{name}.compile.log"));
    let run_log = workspace.join("logs").join(format!("{name}.run.log"));
    fs::write(&rust_source, source)?;
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let mut command = Command::new(rustc);
    command
        .arg("--edition=2024")
        .arg("--target")
        .arg("x86_64-pc-windows-gnu")
        .arg(&rust_source)
        .arg("-C")
        .arg(format!("linker={}", compiler.display()))
        .arg("-C")
        .arg(format!("link-arg={}", abi_object.display()));
    if let Some(archive) = archive {
        command
            .arg("-C")
            .arg("link-arg=-Wl,--start-group")
            .arg("-C")
            .arg(format!("link-arg={}", archive.display()))
            .arg("-C")
            .arg("link-arg=-Wl,--end-group")
            .arg("-C")
            .arg("link-arg=-lgfortran")
            .arg("-C")
            .arg("link-arg=-lm")
            .arg("-C")
            .arg("link-arg=-lmsvcrt");
    }
    let compile = command
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not compile Rust ABI probe {name}: {error}"))
        })?;
    fs::write(
        &compile_log,
        [&compile.stdout[..], &compile.stderr[..]].concat(),
    )?;
    if !compile.status.success() || !executable.is_file() {
        return Ok("compile_failed".to_owned());
    }
    let run = Command::new(&executable).output().map_err(|error| {
        CorpusError::Verification(format!("could not run Rust ABI probe {name}: {error}"))
    })?;
    fs::write(&run_log, [&run.stdout[..], &run.stderr[..]].concat())?;
    Ok(if run.status.success() {
        "passed"
    } else {
        "run_failed"
    }
    .to_owned())
}

fn run_fortran_runtime_probe(compiler: &Path, workspace: &Path, archive: &Path) -> Result<String> {
    let source = workspace.join("selected_machine_error.f");
    let executable = workspace.join("selected_machine_error.exe");
    let log = workspace.join("logs/selected_machine_error.log");
    fs::write(&source, selected_machine_error_fortran())?;
    let output = Command::new(compiler)
        .args(["-x", "f77", "-std=legacy", "-ffixed-line-length-none"])
        .arg(&source)
        .args(["-x", "none"])
        .arg("-Wl,--start-group")
        .arg(archive)
        .arg("-Wl,--end-group")
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not compile selected runtime probe: {error}"))
        })?;
    fs::write(&log, [&output.stdout[..], &output.stderr[..]].concat())?;
    if !output.status.success() || !executable.is_file() {
        return Ok("compile_failed".to_owned());
    }
    let run = Command::new(&executable).output().map_err(|error| {
        CorpusError::Verification(format!("could not run selected runtime probe: {error}"))
    })?;
    fs::write(&log, [&run.stdout[..], &run.stderr[..]].concat())?;
    Ok(if run.status.success() {
        "passed"
    } else {
        "run_failed"
    }
    .to_owned())
}

fn abi_probe_fortran() -> &'static str {
    "      SUBROUTINE SLATEC_PROBE_INTEGER(VALUE)\n      INTEGER VALUE\n      VALUE = 31415\n      END\n      SUBROUTINE SLATEC_PROBE_REAL(VALUE)\n      REAL VALUE\n      VALUE = VALUE * 2.0\n      END\n      DOUBLE PRECISION FUNCTION SLATEC_PROBE_DOUBLE(VALUE)\n      DOUBLE PRECISION VALUE\n      SLATEC_PROBE_DOUBLE = VALUE * 3.0D0\n      END\n      INTEGER FUNCTION SLATEC_PROBE_IFUNCTION(VALUE)\n      INTEGER VALUE\n      SLATEC_PROBE_IFUNCTION = VALUE + 7\n      END\n      REAL FUNCTION SLATEC_PROBE_RFUNCTION(VALUE)\n      REAL VALUE\n      SLATEC_PROBE_RFUNCTION = VALUE + 0.5\n      END\n      SUBROUTINE SLATEC_PROBE_LOGICAL(VALUE, RESULT)\n      LOGICAL VALUE\n      INTEGER RESULT\n      IF (VALUE) THEN\n        RESULT = 1\n      ELSE\n        RESULT = 0\n      END IF\n      END\n      SUBROUTINE SLATEC_PROBE_COMPLEX(VALUE)\n      COMPLEX VALUE\n      VALUE = VALUE * (1.0,2.0)\n      END\n      SUBROUTINE SLATEC_PROBE_CHARACTER(VALUE, RESULT)\n      CHARACTER*(*) VALUE\n      INTEGER RESULT\n      RESULT = LEN(VALUE)\n      END\n      SUBROUTINE SLATEC_PROBE_ARRAY(N, VALUES)\n      INTEGER N\n      DOUBLE PRECISION VALUES(N)\n      VALUES(1) = VALUES(1) + 1.0D0\n      END\n      SUBROUTINE SLATEC_PROBE_CALLBACK(F, X, RESULT)\n      DOUBLE PRECISION F, X, RESULT\n      EXTERNAL F\n      RESULT = F(X)\n      END\n"
}

fn abi_probe_rust() -> String {
    r#"#[repr(C)]
struct Complex32 { re: f32, im: f32 }
#[link(name = "gfortran")]
unsafe extern "C" {
    fn slatec_probe_integer_(value: *mut i32);
    fn slatec_probe_real_(value: *mut f32);
    fn slatec_probe_double_(value: *mut f64) -> f64;
    fn slatec_probe_ifunction_(value: *mut i32) -> i32;
    fn slatec_probe_rfunction_(value: *mut f32) -> f32;
    fn slatec_probe_logical_(value: *mut i32, result: *mut i32);
    fn slatec_probe_complex_(value: *mut Complex32);
    fn slatec_probe_character_(value: *mut core::ffi::c_char, result: *mut i32, length: usize);
    fn slatec_probe_array_(n: *mut i32, values: *mut f64);
    fn slatec_probe_callback_(callback: unsafe extern "C" fn(*const f64) -> f64, value: *mut f64, result: *mut f64);
}
unsafe extern "C" fn plus_one(value: *const f64) -> f64 { unsafe { *value + 1.0 } }
fn main() {
    let mut integer = 0_i32; unsafe { slatec_probe_integer_(&mut integer) }; if integer != 31415 { std::process::exit(2) }
    let mut real = 1.5_f32; unsafe { slatec_probe_real_(&mut real) }; if real != 3.0 { std::process::exit(3) }
    let mut double = 2.0_f64; if unsafe { slatec_probe_double_(&mut double) } != 6.0 { std::process::exit(4) }
    let mut i = 5_i32; if unsafe { slatec_probe_ifunction_(&mut i) } != 12 { std::process::exit(5) }
    let mut r = 1.0_f32; if unsafe { slatec_probe_rfunction_(&mut r) } != 1.5 { std::process::exit(6) }
    let mut logical = 1_i32; let mut logical_result = 0_i32; unsafe { slatec_probe_logical_(&mut logical, &mut logical_result) }; if logical_result != 1 { std::process::exit(7) }
    let mut complex = Complex32 { re: 1.0, im: 2.0 }; unsafe { slatec_probe_complex_(&mut complex) }; if complex.re != -3.0 || complex.im != 4.0 { std::process::exit(8) }
    let mut text = [b'a' as core::ffi::c_char, b'b' as core::ffi::c_char, b'c' as core::ffi::c_char]; let mut length = 0_i32; unsafe { slatec_probe_character_(text.as_mut_ptr(), &mut length, 3) }; if length != 3 { std::process::exit(9) }
    let mut n = 2_i32; let mut values = [1.0_f64, 2.0]; unsafe { slatec_probe_array_(&mut n, values.as_mut_ptr()) }; if values[0] != 2.0 { std::process::exit(10) }
    let mut input = 2.0_f64; let mut result = 0.0_f64; unsafe { slatec_probe_callback_(plus_one, &mut input, &mut result) }; if result != 3.0 { std::process::exit(11) }
}
"#.to_owned()
}

fn selected_numeric_rust() -> String {
    r#"#[link(name = "gfortran")]
unsafe extern "C" {
    fn daxpy_(n: *mut i32, alpha: *mut f64, x: *mut f64, incx: *mut i32, y: *mut f64, incy: *mut i32);
    fn dasum_(n: *mut i32, x: *mut f64, incx: *mut i32) -> f64;
}
fn main() {
    let mut n = 3_i32; let mut alpha = 2.0_f64; let mut inc = 1_i32; let mut x = [1.0_f64, 2.0, 3.0]; let mut y = [4.0_f64, 5.0, 6.0];
    unsafe { daxpy_(&mut n, &mut alpha, x.as_mut_ptr(), &mut inc, y.as_mut_ptr(), &mut inc) };
    if y != [6.0, 9.0, 12.0] { std::process::exit(2) }
    if unsafe { dasum_(&mut n, x.as_mut_ptr(), &mut inc) } != 6.0 { std::process::exit(3) }
}
"#.to_owned()
}

fn selected_complex_rust() -> String {
    r#"#[repr(C)]
struct Complex32 { re: f32, im: f32 }
#[link(name = "gfortran")]
unsafe extern "C" { fn cabs_(value: *mut Complex32) -> f32; }
fn main() { let mut value = Complex32 { re: 3.0, im: 4.0 }; let result = unsafe { cabs_(&mut value) }; if result != 5.0 { std::process::exit(2) } }
"#.to_owned()
}

fn selected_machine_error_fortran() -> &'static str {
    "      PROGRAM SLATEC_RAW_FFI_RUNTIME\n      INTEGER I1MACH, MACHINE_VALUE\n      MACHINE_VALUE = I1MACH(9)\n      CALL XERMSG('SLATEC','RAWFFI','SMOKE',1,-1)\n      END\n"
}

fn read_json<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(path)?).map_err(CorpusError::from)
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

fn promote(output_dir: &Path, snapshot: &str, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let parent = output_dir.parent().ok_or_else(|| {
        CorpusError::Policy("raw FFI output directory must have a parent".to_owned())
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

#[cfg(test)]
mod tests {
    use super::*;

    fn provider(kind: &str, category: &str) -> Provider {
        Provider {
            snapshot_id: "snapshot".to_owned(),
            program_unit_id: "unit".to_owned(),
            normalized_name: "TEST".to_owned(),
            kind: kind.to_owned(),
            source_subset: "main-src".to_owned(),
            source_path: "src/test.f".to_owned(),
            raw_sha256: "a".repeat(64),
            normalized_sha256: "a".repeat(64),
            selection_category: category.to_owned(),
        }
    }

    fn facts(kind: &str, argument_type: Option<&str>) -> UnitFacts {
        UnitFacts {
            kind: kind.to_owned(),
            declaration_locator_id: Some("locator".to_owned()),
            arguments: argument_type
                .map(|declared_type| {
                    vec![ArgumentFact {
                        id: "arg".to_owned(),
                        position: 1,
                        name: "A".to_owned(),
                        declared_type: Some(declared_type.to_owned()),
                        type_source: "explicit".to_owned(),
                        character_length: (declared_type == "CHARACTER").then(|| "1".to_owned()),
                        ..ArgumentFact::default()
                    }]
                })
                .unwrap_or_default(),
            function_result: (kind == "function").then(|| ResultFact {
                id: "result".to_owned(),
                declared_type: Some("REAL".to_owned()),
                type_source: "function_prefix".to_owned(),
                ..ResultFact::default()
            }),
            ..UnitFacts::default()
        }
    }

    fn validated() -> AbiValidation {
        AbiValidation {
            status: "passed".to_owned(),
            integer_real_double: "passed".to_owned(),
            logical: "passed".to_owned(),
            complex: "passed".to_owned(),
            character: "passed".to_owned(),
            callback: "passed".to_owned(),
            selected_numeric_rust: "passed".to_owned(),
            selected_complex_rust: "passed".to_owned(),
            selected_machine_error: "passed".to_owned(),
        }
    }

    #[test]
    fn classifies_standard_and_sensitive_batches_conservatively() {
        let standard = classify(
            &provider("subroutine", "selected_numerical_program_unit"),
            &facts("subroutine", Some("DOUBLE PRECISION")),
            true,
            Some("test_"),
            1,
            &validated(),
        );
        assert_eq!(standard.0, "generated_standard");
        assert_eq!(
            standard.1.as_deref(),
            Some("batch_numeric_scalar_subroutines")
        );
        let character = classify(
            &provider("subroutine", "selected_numerical_program_unit"),
            &facts("subroutine", Some("CHARACTER")),
            true,
            Some("test_"),
            1,
            &validated(),
        );
        assert_eq!(character.0, "generated_abi_sensitive");
        assert_eq!(character.1.as_deref(), Some("batch_character"));
        let function = classify(
            &provider("function", "selected_numerical_program_unit"),
            &facts("function", Some("REAL")),
            true,
            Some("test_"),
            1,
            &validated(),
        );
        assert_eq!(function.1.as_deref(), Some("batch_scalar_functions"));
    }

    #[test]
    fn keeps_callbacks_and_missing_symbols_out_of_generated_bindings() {
        let mut callback_facts = facts("subroutine", Some("REAL"));
        callback_facts.arguments[0].is_external = true;
        let callback = classify(
            &provider("subroutine", "selected_numerical_program_unit"),
            &callback_facts,
            true,
            Some("test_"),
            1,
            &validated(),
        );
        assert_eq!(callback.0, "manual_review_required");
        assert_eq!(callback.1.as_deref(), Some("batch_callbacks"));
        let missing = classify(
            &provider("subroutine", "selected_numerical_program_unit"),
            &facts("subroutine", Some("REAL")),
            true,
            None,
            0,
            &validated(),
        );
        assert_eq!(missing.0, "manual_review_required");
    }

    #[test]
    fn selected_infrastructure_is_non_callable_even_when_its_source_compiles() {
        let infrastructure = classify(
            &provider("function", "selected_infrastructure_unit"),
            &facts("function", Some("REAL")),
            true,
            Some("test_"),
            1,
            &validated(),
        );
        assert_eq!(infrastructure.0, "non_callable_infrastructure");
        assert_eq!(infrastructure.1.as_deref(), Some("batch_infrastructure"));
    }

    #[test]
    fn generated_binding_uses_exact_observed_symbol_and_character_lengths() {
        let mut record = InterfaceRecord {
            provider: provider("subroutine", "selected_numerical_program_unit"),
            facts: facts("subroutine", Some("CHARACTER")),
            observed_symbol: Some("test_".to_owned()),
            confidence: "generated_abi_sensitive".to_owned(),
            batch: Some("batch_character".to_owned()),
            review_state: "abi_profile_checked".to_owned(),
            diagnostic_ids: Vec::new(),
        };
        record.facts.arguments[0].name = "TYPE".to_owned();
        let text = binding_module(
            "batch_character",
            "snapshot",
            &CompilerProfile {
                identity: "gnu-fortran".to_owned(),
                version: "GNU Fortran".to_owned(),
                target: "x86_64".to_owned(),
                profile_id: "profile".to_owned(),
            },
            &[&record],
        )
        .unwrap();
        assert!(text.contains("#[link_name = \"test_\"]"));
        assert!(text.contains("r#type: *mut c_char"));
        assert!(text.contains("character_length_1: FortranCharacterLength"));
    }

    #[test]
    fn profile_override_set_is_explicit_and_unique() {
        let names = PROFILE_OVERRIDE_NAMES
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        assert_eq!(names.len(), PROFILE_OVERRIDE_NAMES.len());
        for name in PROFILE_OVERRIDE_NAMES {
            assert!(profile_source_path(name).is_file(), "{name}");
        }
    }

    #[test]
    fn duplicate_override_symbol_owners_are_rejected() {
        let valid = BTreeMap::from([("I1MACH".to_owned(), 1)]);
        assert!(ensure_single_override_counts(&valid, &["I1MACH"]).is_ok());
        let duplicate = BTreeMap::from([("I1MACH".to_owned(), 2)]);
        assert!(ensure_single_override_counts(&duplicate, &["I1MACH"]).is_err());
    }
}
