//! Audit the SLATEC-hosted live trees without changing the `main-src` corpus.
//!
//! The resulting inventory is comparison evidence.  It deliberately has no
//! provider-selection or compilation side effect.

use crate::error::{CorpusError, Result};
use crate::fixed_form;
use crate::hash;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA_VERSION: &str = "1.0.0";
const AUDIT_SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const COMMITTED_SIZE_LIMIT: u64 = 4_000_000;

#[derive(Deserialize)]
struct AuditConfig {
    schema_version: u32,
    policy_version: u32,
    catalogues: Catalogues,
    source_sets: Vec<SourceSet>,
    supplemental_sets: Vec<SourceSet>,
    supplemental_files: Vec<SourceSet>,
}

#[derive(Deserialize)]
struct Catalogues {
    list: String,
    toc: String,
}

#[derive(Clone, Deserialize)]
struct SourceSet {
    id: String,
    url: String,
    relationship: String,
}

#[derive(Deserialize)]
struct Records<T> {
    records: Vec<T>,
}

#[derive(Deserialize)]
struct ArtifactManifest {
    snapshot_id: String,
    sha256: String,
}

#[derive(Deserialize)]
struct ProgramManifest {
    snapshot_id: String,
    output_semantic_hash: String,
}

#[derive(Clone, Deserialize)]
struct SourceFile {
    id: String,
    snapshot_id: String,
    artifact_id: String,
    archive_member_path: String,
    member_sha256: String,
    selected_state: String,
}

#[derive(Clone, Deserialize)]
struct ProgramUnit {
    id: String,
    snapshot_id: String,
    source_file_id: String,
    name: Option<String>,
    normalized_name: Option<String>,
    kind: String,
}

#[derive(Clone)]
struct AcquiredFile {
    subset: String,
    relationship: String,
    path: String,
    url: String,
    bytes: Vec<u8>,
}

#[derive(Clone)]
struct Provider {
    id: String,
    subset: String,
    relationship: String,
    path: String,
    raw_sha256: String,
    normalized_sha256: String,
    normalized_name: String,
    kind: String,
}

#[derive(Clone)]
struct FileInventory {
    id: String,
    subset: String,
    relationship: String,
    path: String,
    raw_sha256: String,
    normalized_sha256: String,
    byte_length: usize,
    provider_ids: Vec<String>,
    scan_diagnostics: Vec<String>,
}

#[derive(Clone)]
struct CatalogueEntry {
    in_list: bool,
    in_toc: bool,
    toc_classification: String,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct FullCorpusAuditResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
    pub summary: FullCorpusSummary,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct FullCorpusSummary {
    pub files_by_source_subset: BTreeMap<String, usize>,
    pub program_units_by_source_subset: BTreeMap<String, usize>,
    pub unique_program_units_in_union: usize,
    pub duplicate_provider_groups: usize,
    pub catalogue_entries_without_source: usize,
    pub source_units_without_catalogue_entry: usize,
    pub toc_user_callable_entries: usize,
    pub toc_subsidiary_entries: usize,
    pub documented_user_callable_target: usize,
    pub documented_total_routine_floor: usize,
    pub supplemental_source_files: usize,
    pub diagnostics_by_rule: BTreeMap<String, usize>,
}

/// Retrieve (or verify cached) SLATEC-hosted catalogue and directory evidence,
/// then produce a compact union inventory.  `main-src` is read-only input.
pub fn audit(
    evidence_dir: &Path,
    corpus_manifest_dir: &Path,
    program_unit_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<FullCorpusAuditResult> {
    let config_path = Path::new("metadata/full-corpus-audit.toml");
    let config_bytes = fs::read(config_path)?;
    let config: AuditConfig =
        toml::from_str(std::str::from_utf8(&config_bytes).map_err(|_| {
            CorpusError::Policy("full-corpus audit configuration is not UTF-8".to_owned())
        })?)?;
    if config.schema_version != 1 || config.policy_version != 1 {
        return Err(CorpusError::Policy(
            "only full-corpus audit policy/schema version 1 is supported".to_owned(),
        ));
    }
    audit_with_config(
        &config,
        &config_bytes,
        evidence_dir,
        corpus_manifest_dir,
        program_unit_dir,
        output_dir,
        offline,
    )
}

fn audit_with_config(
    config: &AuditConfig,
    config_bytes: &[u8],
    evidence_dir: &Path,
    corpus_manifest_dir: &Path,
    program_unit_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<FullCorpusAuditResult> {
    let artifact: ArtifactManifest =
        read_json(&corpus_manifest_dir.join("artifact-manifest.json"))?;
    let program_manifest: ProgramManifest = read_json(&program_unit_dir.join("manifest.json"))?;
    if program_manifest.snapshot_id != artifact.snapshot_id {
        return Err(CorpusError::Verification(
            "program-unit manifest snapshot does not match immutable main-src snapshot".to_owned(),
        ));
    }
    let main_sources: Records<SourceFile> =
        read_json(&corpus_manifest_dir.join("source-files.json"))?;
    let main_units: Records<ProgramUnit> = read_json(&program_unit_dir.join("program-units.json"))?;
    let snapshot = artifact.snapshot_id.clone();
    let input_root = evidence_dir.join("full-corpus").join("audit-input");
    let catalogues = acquire_catalogues(config, &input_root, offline)?;
    let mut acquired = acquire_source_sets(&config.source_sets, &input_root, offline)?;
    acquired.extend(acquire_source_sets(
        &config.supplemental_sets,
        &input_root,
        offline,
    )?);
    acquired.extend(acquire_supplemental_files(
        &config.supplemental_files,
        &input_root,
        offline,
    )?);
    let directory_index_hashes = directory_index_hashes(
        &config
            .source_sets
            .iter()
            .chain(config.supplemental_sets.iter())
            .cloned()
            .collect::<Vec<_>>(),
        &input_root,
    )?;
    write_acquisition_record(&input_root, &catalogues, &acquired)?;
    let mut files = Vec::new();
    let mut providers = Vec::new();
    let mut diagnostics = Vec::new();
    add_main_src(
        evidence_dir,
        &snapshot,
        main_sources.records,
        main_units.records,
        &mut files,
        &mut providers,
        &mut diagnostics,
    )?;
    for file in acquired {
        let (file_inventory, mut file_providers) = inventory_acquired_file(&file);
        diagnostics.extend(file_inventory.scan_diagnostics.iter().map(|rule| {
            compact_diagnostic(
                "warning",
                rule,
                "source scanner could not establish complete program-unit coverage",
                &file.subset,
                &file.path,
            )
        }));
        providers.append(&mut file_providers);
        files.push(file_inventory);
    }
    files.sort_by(|left, right| {
        (&left.subset, &left.path, &left.id).cmp(&(&right.subset, &right.path, &right.id))
    });
    providers.sort_by(|left, right| {
        (&left.normalized_name, &left.subset, &left.path, &left.id).cmp(&(
            &right.normalized_name,
            &right.subset,
            &right.path,
            &right.id,
        ))
    });
    let catalogue = catalogue_entries(&catalogues.list, &catalogues.toc, &mut diagnostics);
    let relationships = classify_relationships(&providers, &catalogue, &mut diagnostics);
    let toc_user_callable = catalogue
        .values()
        .filter(|entry| entry.in_toc && entry.toc_classification == "user_callable")
        .count();
    if toc_user_callable != 902 {
        diagnostics.push(compact_diagnostic(
            "warning",
            "FULL-CORPUS-TOC-USER-CALLABLE-COUNT-MISMATCH",
            "conservative TOC parsing did not reproduce the documented 902 user-callable identities",
            "catalogue",
            &toc_user_callable.to_string(),
        ));
    }
    let union_count = providers
        .iter()
        .map(|provider| provider.normalized_name.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    if union_count <= 735 {
        diagnostics.push(compact_diagnostic(
            "warning",
            "FULL-CORPUS-UNION-NOT-LARGER-THAN-MAIN-SRC",
            "the discovered SLATEC-hosted union did not exceed the immutable 735-program-unit main-src subset",
            "provider-union",
            &union_count.to_string(),
        ));
    }
    let summary = make_summary(&files, &providers, &catalogue, &relationships, &diagnostics);
    let status = if diagnostics
        .iter()
        .any(|diagnostic| diagnostic[2].as_str() == Some("error"))
    {
        "failed"
    } else if diagnostics.is_empty() {
        "success"
    } else {
        "success_with_review_items"
    }
    .to_owned();
    let source_manifest_hash = hash::file(&corpus_manifest_dir.join("source-files.json"))?;
    let program_units_hash = hash::file(&program_unit_dir.join("program-units.json"))?;
    let input_hash = audit_input_hash(&catalogues, &files, &directory_index_hashes);
    let mut outputs = BTreeMap::new();
    outputs.insert(
        "artifact-index.json",
        compact(&json!({
            "schema_id": "slatec-rs/full-corpus-artifact-index",
            "schema_version": SCHEMA_VERSION,
            "main_src_snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "records": artifact_values(config, &catalogues, &files, &directory_index_hashes),
        }))?,
    );
    outputs.insert(
        "source-file-index.json",
        compact(&json!({
            "schema_id": "slatec-rs/full-corpus-source-file-index",
            "schema_version": SCHEMA_VERSION,
            "main_src_snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "record_encoding": "columnar",
            "columns": ["id", "subset", "relationship", "path", "raw_sha256", "normalized_sha256", "byte_length", "program_unit_count", "program_unit_ids", "scan_diagnostic_rules"],
            "records": files.iter().map(file_value).collect::<Vec<_>>(),
        }))?,
    );
    outputs.insert(
        "program-unit-union.json",
        compact(&json!({
            "schema_id": "slatec-rs/full-corpus-program-unit-union",
            "schema_version": SCHEMA_VERSION,
            "main_src_snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "record_encoding": "columnar",
            "columns": ["normalized_name", "provider_ids", "provider_count", "kinds", "relationship_classification"],
            "records": union_values(&providers, &relationships),
        }))?,
    );
    outputs.insert(
        "provider-relationships.json",
        compact(&json!({
            "schema_id": "slatec-rs/full-corpus-provider-relationships",
            "schema_version": SCHEMA_VERSION,
            "main_src_snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "record_encoding": "columnar",
            "columns": ["id", "normalized_name", "classification", "provider_ids", "provider_subsets", "raw_hash_count", "normalized_hash_count", "review_state"],
            "records": relationships,
        }))?,
    );
    outputs.insert(
        "catalogue-comparison.json",
        compact(&json!({
            "schema_id": "slatec-rs/full-corpus-catalogue-comparison",
            "schema_version": SCHEMA_VERSION,
            "main_src_snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "record_encoding": "columnar",
            "columns": ["normalized_name", "in_list", "in_toc", "toc_classification", "source_provider_count", "comparison_state"],
            "records": catalogue_comparison_values(&providers, &catalogue),
        }))?,
    );
    outputs.insert(
        "diagnostics.json",
        compact(&json!({
            "schema_id": "slatec-rs/full-corpus-diagnostic-set",
            "schema_version": SCHEMA_VERSION,
            "main_src_snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "stage_status": status,
            "records": diagnostics,
        }))?,
    );
    let committed_size: u64 = outputs.values().map(|bytes| bytes.len() as u64).sum();
    if committed_size > COMMITTED_SIZE_LIMIT {
        return Err(CorpusError::Verification(format!(
            "committed full-corpus audit output would be {committed_size} bytes; redesign compact inventories before committing"
        )));
    }
    let semantic_hash = semantic_hash(&outputs);
    let output_hashes: BTreeMap<_, _> = outputs
        .iter()
        .map(|(name, bytes)| ((*name).to_owned(), hash::bytes(bytes)))
        .collect();
    outputs.insert(
        "manifest.json",
        compact(&json!({
            "id": stable_id("full-corpus-audit", &[&snapshot, &input_hash]),
            "schema_id": "slatec-rs/full-corpus-audit",
            "schema_version": SCHEMA_VERSION,
            "main_src_snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "audit_semantic_version": AUDIT_SEMANTIC_VERSION,
            "main_src_artifact_hash": artifact.sha256,
            "main_src_source_manifest_hash": source_manifest_hash,
            "main_src_program_units_hash": program_units_hash,
            "program_unit_scanner_semantic_hash": program_manifest.output_semantic_hash,
            "audit_configuration_hash": hash::bytes(config_bytes),
            "audit_input_hash": input_hash,
            "directory_index_hashes": directory_index_hashes,
            "output_semantic_hash": semantic_hash,
            "output_file_hashes": output_hashes,
            "committed_output_total_bytes": committed_size,
            "stage_status": status,
            "summary": summary,
            "scope": "comparison-and-completeness-audit; no provider selection",
        }))?,
    );
    outputs.insert(
        "audit-summary.md",
        summary_markdown(&semantic_hash, &status, &summary, committed_size).into_bytes(),
    );
    promote(output_dir, &snapshot, &outputs)?;
    Ok(FullCorpusAuditResult {
        snapshot_id: snapshot,
        status,
        semantic_hash,
        output_dir: output_dir.to_owned(),
        summary,
    })
}

struct CatalogueBytes {
    list: Vec<u8>,
    toc: Vec<u8>,
}

fn acquire_catalogues(config: &AuditConfig, root: &Path, offline: bool) -> Result<CatalogueBytes> {
    let dir = root.join("catalogues");
    let list = acquire_one(&config.catalogues.list, &dir.join("list"), offline)?;
    let toc = acquire_one(&config.catalogues.toc, &dir.join("toc"), offline)?;
    Ok(CatalogueBytes { list, toc })
}

fn acquire_source_sets(
    sets: &[SourceSet],
    root: &Path,
    offline: bool,
) -> Result<Vec<AcquiredFile>> {
    let mut output = Vec::new();
    for set in sets {
        let set_root = root.join("directories").join(&set.id);
        let index = acquire_one(&set.url, &set_root.join("index.html"), offline)?;
        let names = directory_file_names(&index);
        if names.is_empty() {
            return Err(CorpusError::Verification(format!(
                "directory index {} yielded no .f source files",
                set.url
            )));
        }
        for name in names {
            let url = format!("{}{}", set.url, name);
            let bytes = acquire_one(&url, &set_root.join("files").join(&name), offline)?;
            output.push(AcquiredFile {
                subset: set.id.clone(),
                relationship: set.relationship.clone(),
                path: name,
                url,
                bytes,
            });
        }
    }
    Ok(output)
}

fn acquire_supplemental_files(
    sets: &[SourceSet],
    root: &Path,
    offline: bool,
) -> Result<Vec<AcquiredFile>> {
    let mut output = Vec::new();
    for set in sets {
        let path = root.join("supplemental").join(&set.id);
        let bytes = acquire_one(&set.url, &path, offline)?;
        output.push(AcquiredFile {
            subset: set.id.clone(),
            relationship: set.relationship.clone(),
            path: set.id.clone(),
            url: set.url.clone(),
            bytes,
        });
    }
    Ok(output)
}

fn directory_index_hashes(sets: &[SourceSet], root: &Path) -> Result<BTreeMap<String, String>> {
    let mut output = BTreeMap::new();
    for set in sets {
        let index = fs::read(root.join("directories").join(&set.id).join("index.html"))?;
        output.insert(set.id.clone(), hash::bytes(&index));
    }
    Ok(output)
}

fn acquire_one(url: &str, destination: &Path, offline: bool) -> Result<Vec<u8>> {
    if destination.is_file() {
        return fs::read(destination).map_err(CorpusError::from);
    }
    if offline {
        return Err(CorpusError::Verification(format!(
            "offline full-corpus audit input is absent: {}",
            destination.display()
        )));
    }
    let response = ureq::get(url)
        .call()
        .map_err(|error| CorpusError::Network(format!("GET {url}: {error}")))?;
    let bytes = response
        .into_body()
        .read_to_vec()
        .map_err(|error| CorpusError::Network(format!("read {url}: {error}")))?;
    if bytes.is_empty() {
        return Err(CorpusError::Verification(format!(
            "empty response from {url} was not promoted"
        )));
    }
    write_immutable(destination, &bytes)?;
    Ok(bytes)
}

fn write_immutable(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Ok(existing) = fs::read(path) {
        if existing == bytes {
            return Ok(());
        }
        return Err(CorpusError::Verification(format!(
            "previously acquired evidence differs at {}; preserve it and choose a new evidence root",
            path.display()
        )));
    }
    let parent = path.parent().ok_or_else(|| {
        CorpusError::Verification("audit evidence destination has no parent".to_owned())
    })?;
    fs::create_dir_all(parent)?;
    let temporary = path.with_extension("full-corpus-partial");
    if temporary.exists() {
        return Err(CorpusError::Verification(format!(
            "incomplete full-corpus acquisition remains at {}; investigate it explicitly",
            temporary.display()
        )));
    }
    fs::write(&temporary, bytes)?;
    fs::rename(temporary, path)?;
    Ok(())
}

fn directory_file_names(index: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(index);
    let lower = text.to_ascii_lowercase();
    let mut names = BTreeSet::new();
    let mut cursor = 0;
    while let Some(relative) = lower[cursor..].find("href") {
        let start = cursor + relative + 4;
        let tail = &text[start..];
        let Some(equals) = tail.find('=') else {
            break;
        };
        let value = tail[equals + 1..].trim_start();
        let Some(quote) = value
            .chars()
            .next()
            .filter(|character| *character == '\'' || *character == '"')
        else {
            cursor = start + equals + 1;
            continue;
        };
        let after_quote = &value[quote.len_utf8()..];
        let Some(end) = after_quote.find(quote) else {
            break;
        };
        let href = &after_quote[..end];
        if href.to_ascii_lowercase().ends_with(".f")
            && !href.contains('/')
            && !href.contains('\\')
            && href
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            names.insert(href.to_owned());
        }
        cursor = start + equals + 1 + value.len().min(end + quote.len_utf8() + 1);
    }
    names.into_iter().collect()
}

fn write_acquisition_record(
    root: &Path,
    catalogues: &CatalogueBytes,
    files: &[AcquiredFile],
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| CorpusError::Io(std::io::Error::other(error)))?
        .as_secs();
    let records: Vec<_> = files
        .iter()
        .map(|file| {
            json!({
                "subset": file.subset,
                "relationship": file.relationship,
                "path": file.path,
                "url": file.url,
                "sha256": hash::bytes(&file.bytes),
                "bytes": file.bytes.len(),
            })
        })
        .collect();
    let value = json!({
        "schema_id": "slatec-rs/full-corpus-acquisition-audit",
        "schema_version": SCHEMA_VERSION,
        "retrieval_timestamp": format!("unix-seconds:{timestamp}"),
        "catalogues": {
            "list_sha256": hash::bytes(&catalogues.list),
            "toc_sha256": hash::bytes(&catalogues.toc),
        },
        "records": records,
    });
    let mut bytes = serde_json::to_vec_pretty(&value)?;
    bytes.push(b'\n');
    let destination = root.join("acquisition.json");
    let temporary = destination.with_extension("partial");
    fs::create_dir_all(root)?;
    fs::write(&temporary, bytes)?;
    fs::rename(temporary, destination)?;
    Ok(())
}

fn inventory_acquired_file(file: &AcquiredFile) -> (FileInventory, Vec<Provider>) {
    let raw_sha256 = hash::bytes(&file.bytes);
    let normalized_sha256 = hash::bytes(&normalize_line_endings(&file.bytes));
    let source_file_id = stable_id("full-source-file", &[&file.subset, &file.path, &raw_sha256]);
    let physical = fixed_form::physical_lines(&file.bytes);
    let statements = fixed_form::logical_statements(&physical);
    let mut scan_diagnostics = Vec::new();
    let mut providers = Vec::new();
    for statement in &statements {
        if let Some(start) = fixed_form::start_declaration(&statement.normalized_statement_text) {
            if let Some(name) = start.name {
                let normalized_name = name.to_ascii_uppercase();
                let id = stable_id(
                    "full-provider",
                    &[
                        &source_file_id,
                        &normalized_name,
                        &statement.raw_spans[0].start.to_string(),
                    ],
                );
                providers.push(Provider {
                    id,
                    subset: file.subset.clone(),
                    relationship: file.relationship.clone(),
                    path: file.path.clone(),
                    raw_sha256: raw_sha256.clone(),
                    normalized_sha256: normalized_sha256.clone(),
                    normalized_name,
                    kind: start.kind,
                });
            } else {
                scan_diagnostics.push("FULL-CORPUS-UNNAMED-BLOCK-DATA".to_owned());
            }
        }
    }
    if providers.is_empty() {
        scan_diagnostics.push("FULL-CORPUS-NO-TOP-LEVEL-UNIT".to_owned());
    }
    for statement in &statements {
        for diagnostic in &statement.diagnostics {
            scan_diagnostics.push(diagnostic.rule_id.to_owned());
        }
    }
    scan_diagnostics.sort();
    scan_diagnostics.dedup();
    let inventory = FileInventory {
        id: source_file_id,
        subset: file.subset.clone(),
        relationship: file.relationship.clone(),
        path: file.path.clone(),
        raw_sha256,
        normalized_sha256,
        byte_length: file.bytes.len(),
        provider_ids: providers
            .iter()
            .map(|provider| provider.id.clone())
            .collect(),
        scan_diagnostics,
    };
    (inventory, providers)
}

fn add_main_src(
    evidence_dir: &Path,
    snapshot: &str,
    sources: Vec<SourceFile>,
    units: Vec<ProgramUnit>,
    files: &mut Vec<FileInventory>,
    providers: &mut Vec<Provider>,
    diagnostics: &mut Vec<Value>,
) -> Result<()> {
    let mut units_by_source: BTreeMap<String, Vec<ProgramUnit>> = BTreeMap::new();
    for unit in units {
        if unit.snapshot_id != snapshot {
            return Err(CorpusError::Verification(
                "main-src program-unit record snapshot mismatch".to_owned(),
            ));
        }
        units_by_source
            .entry(unit.source_file_id.clone())
            .or_default()
            .push(unit);
    }
    for source in sources {
        if source.snapshot_id != snapshot {
            return Err(CorpusError::Verification(
                "main-src source-file record snapshot mismatch".to_owned(),
            ));
        }
        let is_historical = source.archive_member_path.ends_with(".f.0");
        if source.selected_state != "selected" && !is_historical {
            continue;
        }
        let subset = if is_historical {
            "main-src-historical"
        } else {
            "main-src"
        };
        let relationship = if is_historical {
            "historical-variant"
        } else {
            "pinned-reproducible-subset"
        };
        let source_file_id = stable_id(
            "full-source-file",
            &[subset, &source.archive_member_path, &source.member_sha256],
        );
        let raw_path = evidence_dir
            .join("extracted")
            .join(snapshot)
            .join(&source.artifact_id)
            .join(&source.archive_member_path);
        let bytes = fs::read(&raw_path).map_err(|_| {
            CorpusError::Verification(format!(
                "missing extracted main-src member {}",
                raw_path.display()
            ))
        })?;
        if hash::bytes(&bytes) != source.member_sha256 {
            return Err(CorpusError::Verification(format!(
                "main-src source hash mismatch for {}",
                source.archive_member_path
            )));
        }
        let normalized_sha256 = hash::bytes(&normalize_line_endings(&bytes));
        let byte_length = bytes.len();
        let mut units_for_file = units_by_source.remove(&source.id).unwrap_or_default();
        let mut scan_diagnostics = Vec::new();
        if is_historical {
            let acquired = AcquiredFile {
                subset: subset.to_owned(),
                relationship: relationship.to_owned(),
                path: source.archive_member_path.clone(),
                url: "pinned-archive-member".to_owned(),
                bytes,
            };
            let (inventory, file_providers) = inventory_acquired_file(&acquired);
            files.push(FileInventory {
                id: source_file_id,
                subset: inventory.subset,
                relationship: inventory.relationship,
                path: inventory.path,
                raw_sha256: inventory.raw_sha256,
                normalized_sha256: inventory.normalized_sha256,
                byte_length: inventory.byte_length,
                provider_ids: inventory.provider_ids,
                scan_diagnostics: inventory.scan_diagnostics,
            });
            providers.extend(file_providers);
            continue;
        }
        units_for_file.sort_by(|left, right| left.id.cmp(&right.id));
        let mut provider_ids = Vec::new();
        for unit in units_for_file {
            let (Some(_name), Some(normalized_name)) = (unit.name, unit.normalized_name) else {
                scan_diagnostics.push("FULL-CORPUS-UNNAMED-MAIN-UNIT".to_owned());
                continue;
            };
            let id = stable_id("full-provider", &[&source_file_id, &unit.id]);
            provider_ids.push(id.clone());
            providers.push(Provider {
                id,
                subset: subset.to_owned(),
                relationship: relationship.to_owned(),
                path: source.archive_member_path.clone(),
                raw_sha256: source.member_sha256.clone(),
                normalized_sha256: normalized_sha256.clone(),
                normalized_name,
                kind: unit.kind,
            });
        }
        if provider_ids.is_empty() {
            scan_diagnostics.push("FULL-CORPUS-NO-TOP-LEVEL-UNIT".to_owned());
        }
        files.push(FileInventory {
            id: source_file_id,
            subset: subset.to_owned(),
            relationship: relationship.to_owned(),
            path: source.archive_member_path,
            raw_sha256: source.member_sha256.clone(),
            normalized_sha256,
            byte_length,
            provider_ids,
            scan_diagnostics,
        });
    }
    for source_id in units_by_source.keys() {
        diagnostics.push(compact_diagnostic(
            "error",
            "FULL-CORPUS-UNKNOWN-MAIN-SOURCE",
            "main-src program-unit record references a source not present in the source manifest",
            "main-src",
            source_id,
        ));
    }
    Ok(())
}

fn catalogue_entries(
    list: &[u8],
    toc: &[u8],
    diagnostics: &mut Vec<Value>,
) -> BTreeMap<String, CatalogueEntry> {
    let list_names = catalogue_tokens(list, false);
    let toc_names = catalogue_tokens(toc, true);
    if list_names.is_empty() {
        diagnostics.push(compact_diagnostic(
            "warning",
            "FULL-CORPUS-LIST-PARSE-EMPTY",
            "official list artifact did not match the conservative catalogue grammar",
            "catalogue",
            "list",
        ));
    }
    if toc_names.is_empty() {
        diagnostics.push(compact_diagnostic(
            "error",
            "FULL-CORPUS-TOC-PARSE-EMPTY",
            "official toc artifact did not match the conservative catalogue grammar",
            "catalogue",
            "toc",
        ));
    }
    let mut output = BTreeMap::new();
    for (name, classification) in list_names {
        output
            .entry(name.clone())
            .or_insert(CatalogueEntry {
                in_list: false,
                in_toc: false,
                toc_classification: "not_determined".to_owned(),
            })
            .in_list = true;
        let _ = classification;
    }
    for (name, classification) in toc_names {
        let entry = output.entry(name.clone()).or_insert(CatalogueEntry {
            in_list: false,
            in_toc: false,
            toc_classification: "not_determined".to_owned(),
        });
        entry.in_toc = true;
        if classification == "user_callable" || entry.toc_classification == "not_determined" {
            entry.toc_classification = classification;
        }
    }
    output
}

fn catalogue_tokens(bytes: &[u8], typed: bool) -> BTreeMap<String, String> {
    let text = String::from_utf8_lossy(bytes).to_ascii_uppercase();
    let mut output = BTreeMap::new();
    if !typed {
        // Netlib's official `list` is a numbered inventory: each record
        // begins with a decimal ordinal, then a routine name, then `has`.
        // Keep the grammar deliberately narrow so prose cannot become a
        // catalogue identity.
        for line in text.lines() {
            let Some((ordinal, remainder)) = line.trim_start().split_once('.') else {
                continue;
            };
            if ordinal.trim().parse::<usize>().is_err() {
                continue;
            }
            let Some((name, tail)) = take_catalogue_identifier(remainder.trim_start()) else {
                continue;
            };
            if is_catalogue_name(name) && tail.trim_start().starts_with("HAS ") {
                output.insert(name.to_owned(), "not_determined".to_owned());
            }
        }
    }
    if typed {
        // In the Version 4.1 TOC, Section II is an alphabetical listing with
        // the heading `Subsidiary Routines`; those entries do not carry the
        // Section I type suffix.  Parse only its explicit line grammar.
        let section_two = text
            .find("SECTION II. SUBSIDIARY ROUTINES")
            .map(|offset| &text[offset..])
            .unwrap_or("");
        let section_two = section_two
            .split("SECTION III.")
            .next()
            .unwrap_or(section_two);
        for line in section_two.lines() {
            let Some((name, tail)) = take_catalogue_identifier(line.trim_start()) else {
                continue;
            };
            if is_catalogue_name(name) && tail.trim_start().starts_with("SUBSIDIARY") {
                output.insert(name.to_owned(), "subsidiary".to_owned());
            }
        }
        // Section III is the TOC's complete alphabetic index.  Its entries
        // are columns of `NAME CATEGORY`, while a starred name has no
        // category because it is subsidiary.  The narrow category check
        // avoids promoting the explanatory prose above the table.
        let section_three = text
            .find("SECTION III. ALPHABETIC LIST OF ROUTINES")
            .map(|offset| &text[offset..])
            .unwrap_or("");
        for line in section_three.lines() {
            let tokens = line.split_whitespace().collect::<Vec<_>>();
            for (index, token) in tokens.iter().enumerate() {
                let (subsidiary, name) = token
                    .strip_prefix('*')
                    .map_or((false, *token), |name| (true, name));
                let followed_by_category = tokens
                    .get(index + 1)
                    .is_some_and(|category| is_toc_category(category));
                if is_catalogue_name(name) && (subsidiary || followed_by_category) {
                    let classification = if subsidiary {
                        "subsidiary"
                    } else {
                        "user_callable"
                    };
                    output
                        .entry(name.to_owned())
                        .and_modify(|existing| {
                            if classification == "user_callable" {
                                *existing = classification.to_owned();
                            }
                        })
                        .or_insert_with(|| classification.to_owned());
                }
            }
        }
    }
    for token in text.split_whitespace() {
        let token = token.trim_matches(|character: char| {
            matches!(
                character,
                ',' | '.' | ':' | ';' | '(' | ')' | '[' | ']' | '"' | '\''
            )
        });
        let (starred, token) = token
            .strip_prefix('*')
            .map_or((false, token), |value| (true, value));
        let Some((name, type_marker)) = token.rsplit_once('-') else {
            continue;
        };
        if !is_catalogue_name(name)
            || (typed && !matches!(type_marker, "S" | "D" | "C" | "I" | "H" | "L" | "A"))
            || (!typed && type_marker.len() > 2)
        {
            continue;
        }
        let classification = if starred {
            "subsidiary"
        } else {
            "user_callable"
        };
        output
            .entry(name.to_owned())
            .and_modify(|existing| {
                if classification == "user_callable" {
                    *existing = classification.to_owned();
                }
            })
            .or_insert_with(|| classification.to_owned());
    }
    output
}

fn take_catalogue_identifier(input: &str) -> Option<(&str, &str)> {
    let width = input
        .chars()
        .take_while(|character| {
            character.is_ascii_uppercase() || character.is_ascii_digit() || *character == '$'
        })
        .count();
    (width > 0).then_some((&input[..width], &input[width..]))
}

fn is_catalogue_name(name: &str) -> bool {
    (2..=12).contains(&name.len())
        && name
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'$')
}

fn is_toc_category(value: &str) -> bool {
    value.len() == 1 && value.bytes().all(|byte| byte.is_ascii_uppercase())
        || value.len() >= 2
            && value.as_bytes()[0].is_ascii_uppercase()
            && value.as_bytes()[1].is_ascii_digit()
            && value
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
}

fn classify_relationships(
    providers: &[Provider],
    catalogue: &BTreeMap<String, CatalogueEntry>,
    diagnostics: &mut Vec<Value>,
) -> Vec<Value> {
    let mut groups: BTreeMap<String, Vec<&Provider>> = BTreeMap::new();
    for provider in providers {
        groups
            .entry(provider.normalized_name.clone())
            .or_default()
            .push(provider);
    }
    let mut output = Vec::new();
    for (name, group) in groups {
        let classification = relationship_class(&group);
        let provider_ids: Vec<_> = group.iter().map(|provider| provider.id.clone()).collect();
        let subsets: BTreeSet<_> = group
            .iter()
            .map(|provider| provider.subset.clone())
            .collect();
        let raw_hashes: BTreeSet<_> = group
            .iter()
            .map(|provider| provider.raw_sha256.clone())
            .collect();
        let normalized_hashes: BTreeSet<_> = group
            .iter()
            .map(|provider| provider.normalized_sha256.clone())
            .collect();
        if classification == "duplicate_provider" || classification == "modified_relocated_copy" {
            diagnostics.push(compact_diagnostic(
                "warning",
                "FULL-CORPUS-MULTIPLE-PROVIDERS",
                "multiple SLATEC-hosted providers require explicit future resolution",
                "provider-union",
                &name,
            ));
        }
        output.push(json!([
            stable_id("provider-relationship", &[&name, &classification]),
            name,
            classification,
            provider_ids,
            subsets.into_iter().collect::<Vec<_>>(),
            raw_hashes.len(),
            normalized_hashes.len(),
            if catalogue.contains_key(&name) {
                "review_requested"
            } else {
                "machine_checked"
            },
        ]));
    }
    // A catalogue identity with no source is represented in the comparison
    // index. Emit an explicit review item as well so it cannot disappear in a
    // count-only report.
    let supplied: BTreeSet<_> = providers
        .iter()
        .map(|provider| provider.normalized_name.as_str())
        .collect();
    for name in catalogue
        .keys()
        .filter(|name| !supplied.contains(name.as_str()))
    {
        diagnostics.push(compact_diagnostic(
            "warning",
            "FULL-CORPUS-CATALOGUE-ONLY",
            "catalogue identity has no discovered SLATEC-hosted source provider",
            "catalogue",
            name,
        ));
    }
    output.sort_by(|left, right| left[1].as_str().cmp(&right[1].as_str()));
    output
}

fn relationship_class(group: &[&Provider]) -> String {
    if group.len() == 1 {
        return "unique_provider".to_owned();
    }
    let raw_hashes: BTreeSet<_> = group.iter().map(|provider| &provider.raw_sha256).collect();
    let normalized_hashes: BTreeSet<_> = group
        .iter()
        .map(|provider| &provider.normalized_sha256)
        .collect();
    let has_main = group.iter().any(|provider| provider.subset == "main-src");
    let has_relocated = group
        .iter()
        .any(|provider| provider.relationship == "relocated-subset");
    let has_historical = group
        .iter()
        .any(|provider| provider.relationship == "historical-variant");
    let has_supplemental = group.iter().any(|provider| {
        matches!(
            provider.relationship.as_str(),
            "legacy-error-directory"
                | "special-function-documentation-or-source-candidate"
                | "documentation-support-source-candidate"
        )
    });
    if has_historical {
        "historical_variant".to_owned()
    } else if raw_hashes.len() == 1 && has_main && has_relocated {
        "byte_identical_relocated_copy".to_owned()
    } else if normalized_hashes.len() == 1 && has_main && has_relocated {
        "normalized_identical_relocated_copy".to_owned()
    } else if raw_hashes.len() == 1 && has_main {
        // The live `slatec/src` location can be another retrieval location of
        // the same pinned content.  It is one content provider with multiple
        // locations, not an unresolved duplicate selection.
        "unique_provider".to_owned()
    } else if has_supplemental && raw_hashes.len() > 1 {
        "alternate_implementation".to_owned()
    } else if has_main && has_relocated && raw_hashes.len() > 1 {
        "modified_relocated_copy".to_owned()
    } else {
        "duplicate_provider".to_owned()
    }
}

fn make_summary(
    files: &[FileInventory],
    providers: &[Provider],
    catalogue: &BTreeMap<String, CatalogueEntry>,
    relationships: &[Value],
    diagnostics: &[Value],
) -> FullCorpusSummary {
    let mut files_by_source_subset = BTreeMap::new();
    let mut program_units_by_source_subset = BTreeMap::new();
    for file in files {
        *files_by_source_subset
            .entry(file.subset.clone())
            .or_insert(0) += 1;
    }
    for provider in providers {
        *program_units_by_source_subset
            .entry(provider.subset.clone())
            .or_insert(0) += 1;
    }
    let source_names: BTreeSet<_> = providers
        .iter()
        .map(|provider| provider.normalized_name.as_str())
        .collect();
    let catalogue_entries_without_source = catalogue
        .keys()
        .filter(|name| !source_names.contains(name.as_str()))
        .count();
    let source_units_without_catalogue_entry = source_names
        .iter()
        .filter(|name| !catalogue.contains_key(**name))
        .count();
    let toc_user_callable_entries = catalogue
        .values()
        .filter(|entry| entry.in_toc && entry.toc_classification == "user_callable")
        .count();
    let toc_subsidiary_entries = catalogue
        .values()
        .filter(|entry| entry.in_toc && entry.toc_classification == "subsidiary")
        .count();
    let duplicate_provider_groups = relationships
        .iter()
        .filter(|record| {
            matches!(
                record[2].as_str(),
                Some("duplicate_provider")
                    | Some("modified_relocated_copy")
                    | Some("alternate_implementation")
                    | Some("historical_variant")
            )
        })
        .count();
    let supplemental_source_files = files
        .iter()
        .filter(|file| {
            matches!(
                file.relationship.as_str(),
                "legacy-error-directory"
                    | "special-function-documentation-or-source-candidate"
                    | "documentation-support-source-candidate"
            )
        })
        .count();
    let mut diagnostics_by_rule = BTreeMap::new();
    for diagnostic in diagnostics {
        if let Some(rule) = diagnostic[3].as_str() {
            *diagnostics_by_rule.entry(rule.to_owned()).or_insert(0) += 1;
        }
    }
    FullCorpusSummary {
        files_by_source_subset,
        program_units_by_source_subset,
        unique_program_units_in_union: source_names.len(),
        duplicate_provider_groups,
        catalogue_entries_without_source,
        source_units_without_catalogue_entry,
        toc_user_callable_entries,
        toc_subsidiary_entries,
        documented_user_callable_target: 902,
        documented_total_routine_floor: 1400,
        supplemental_source_files,
        diagnostics_by_rule,
    }
}

fn union_values(providers: &[Provider], relationships: &[Value]) -> Vec<Value> {
    let relationship_by_name: BTreeMap<_, _> = relationships
        .iter()
        .filter_map(|record| {
            Some((
                record[1].as_str()?.to_owned(),
                record[2].as_str()?.to_owned(),
            ))
        })
        .collect();
    let mut groups: BTreeMap<String, Vec<&Provider>> = BTreeMap::new();
    for provider in providers {
        groups
            .entry(provider.normalized_name.clone())
            .or_default()
            .push(provider);
    }
    groups
        .into_iter()
        .map(|(name, group)| {
            let kinds: BTreeSet<_> = group.iter().map(|provider| provider.kind.clone()).collect();
            json!([
                name.clone(),
                group
                    .iter()
                    .map(|provider| provider.id.clone())
                    .collect::<Vec<_>>(),
                group.len(),
                kinds.into_iter().collect::<Vec<_>>(),
                relationship_by_name
                    .get(&name)
                    .cloned()
                    .unwrap_or_else(|| "unresolved".to_owned()),
            ])
        })
        .collect()
}

fn catalogue_comparison_values(
    providers: &[Provider],
    catalogue: &BTreeMap<String, CatalogueEntry>,
) -> Vec<Value> {
    let mut counts = BTreeMap::new();
    for provider in providers {
        *counts
            .entry(provider.normalized_name.clone())
            .or_insert(0_usize) += 1;
    }
    let names: BTreeSet<_> = counts.keys().chain(catalogue.keys()).cloned().collect();
    names
        .into_iter()
        .map(|name| {
            let entry = catalogue.get(&name);
            let providers = counts.get(&name).copied().unwrap_or(0);
            let state = if providers == 0 {
                "catalogue_only"
            } else if entry.is_none() {
                "source_only"
            } else {
                "both"
            };
            json!([
                name,
                entry.is_some_and(|entry| entry.in_list),
                entry.is_some_and(|entry| entry.in_toc),
                entry.map_or("not_determined", |entry| entry.toc_classification.as_str()),
                providers,
                state,
            ])
        })
        .collect()
}

fn artifact_values(
    config: &AuditConfig,
    catalogues: &CatalogueBytes,
    files: &[FileInventory],
    directory_index_hashes: &BTreeMap<String, String>,
) -> Vec<Value> {
    let mut records = vec![
        json!({
            "id": "catalogue-list",
            "kind": "official-list",
            "url": config.catalogues.list,
            "sha256": hash::bytes(&catalogues.list),
            "byte_length": catalogues.list.len(),
            "selection_state": "documentation-evidence",
        }),
        json!({
            "id": "catalogue-toc",
            "kind": "official-version-4.1-toc",
            "url": config.catalogues.toc,
            "sha256": hash::bytes(&catalogues.toc),
            "byte_length": catalogues.toc.len(),
            "selection_state": "documentation-evidence",
        }),
    ];
    let mut source_files: BTreeMap<String, usize> = BTreeMap::new();
    for file in files {
        *source_files.entry(file.subset.clone()).or_insert(0) += 1;
    }
    for set in config
        .source_sets
        .iter()
        .chain(config.supplemental_sets.iter())
        .chain(config.supplemental_files.iter())
    {
        records.push(json!({
            "id": format!("slatec-hosted-{}", set.id),
            "kind": "live-directory-or-supplemental-source",
            "url": set.url,
            "relationship": set.relationship,
            "directory_index_sha256": directory_index_hashes.get(&set.id),
            "retrieved_source_file_count": source_files.get(&set.id).copied().unwrap_or(0),
            "selection_state": "comparison-and-completeness-evidence",
        }));
    }
    records.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));
    records
}

fn file_value(file: &FileInventory) -> Value {
    json!([
        file.id,
        file.subset,
        file.relationship,
        file.path,
        file.raw_sha256,
        file.normalized_sha256,
        file.byte_length,
        file.provider_ids.len(),
        file.provider_ids,
        file.scan_diagnostics,
    ])
}

fn compact_diagnostic(
    severity: &str,
    rule: &str,
    message: &str,
    scope: &str,
    identity: &str,
) -> Value {
    json!([
        stable_id("full-corpus-diagnostic", &[severity, rule, scope, identity]),
        "full_corpus_audit",
        severity,
        rule,
        message,
        scope,
        identity,
        "review_requested",
    ])
}

fn audit_input_hash(
    catalogues: &CatalogueBytes,
    files: &[FileInventory],
    directory_index_hashes: &BTreeMap<String, String>,
) -> String {
    let mut material = Vec::new();
    material.extend_from_slice(hash::bytes(&catalogues.list).as_bytes());
    material.push(0);
    material.extend_from_slice(hash::bytes(&catalogues.toc).as_bytes());
    material.push(0);
    for (subset, sha256) in directory_index_hashes {
        material.extend_from_slice(subset.as_bytes());
        material.push(0);
        material.extend_from_slice(sha256.as_bytes());
        material.push(0);
    }
    for file in files {
        material.extend_from_slice(file.subset.as_bytes());
        material.push(0);
        material.extend_from_slice(file.path.as_bytes());
        material.push(0);
        material.extend_from_slice(file.raw_sha256.as_bytes());
        material.push(0);
    }
    hash::bytes(&material)
}

fn normalize_line_endings(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(input.len());
    let mut index = 0;
    while index < input.len() {
        if input[index] == b'\r' {
            output.push(b'\n');
            index += usize::from(input.get(index + 1) == Some(&b'\n')) + 1;
        } else {
            output.push(input[index]);
            index += 1;
        }
    }
    output
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

fn read_json<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(path)?).map_err(CorpusError::from)
}

fn promote(output_dir: &Path, snapshot: &str, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let parent = output_dir
        .parent()
        .ok_or_else(|| CorpusError::Policy("audit output directory must have parent".to_owned()))?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        "{}.staging-{snapshot}",
        output_dir.file_name().unwrap_or_default().to_string_lossy()
    ));
    if staging.exists() {
        return Err(CorpusError::Verification(format!(
            "full-corpus audit staging directory exists: {}",
            staging.display()
        )));
    }
    fs::create_dir(&staging)?;
    for (name, bytes) in files {
        fs::write(staging.join(name), bytes)?;
    }
    if output_dir.exists() {
        let identical = files.iter().all(|(name, bytes)| {
            fs::read(output_dir.join(name)).ok().as_deref() == Some(bytes.as_slice())
        });
        if identical {
            fs::remove_dir_all(staging)?;
            return Ok(());
        }
        let backup = parent.join(format!(
            "{}.previous-{snapshot}",
            output_dir.file_name().unwrap_or_default().to_string_lossy()
        ));
        if backup.exists() {
            return Err(CorpusError::Verification(format!(
                "full-corpus audit backup directory exists: {}",
                backup.display()
            )));
        }
        fs::rename(output_dir, &backup)?;
        if let Err(error) = fs::rename(&staging, output_dir) {
            let _ = fs::rename(&backup, output_dir);
            return Err(CorpusError::from(error));
        }
        fs::remove_dir_all(backup)?;
    } else {
        fs::rename(staging, output_dir)?;
    }
    Ok(())
}

fn summary_markdown(
    semantic_hash: &str,
    status: &str,
    summary: &FullCorpusSummary,
    committed_size: u64,
) -> String {
    format!(
        "# SLATEC full-corpus completeness audit\n\n- Semantic hash: `{semantic_hash}`\n- Stage status: `{status}`\n- `main-src` is retained as a 735-program-unit reproducible subset, not a complete-library claim.\n- Unique program-unit identities in the SLATEC-hosted union: {}\n- Duplicate or variant provider groups: {}\n- Catalogue entries without discovered source: {}\n- Source identities without catalogue entry: {}\n- TOC user-callable identities: {} (documented target: 902)\n- TOC subsidiary identities: {}\n- Committed generated bytes: {committed_size}\n\nDownloaded catalogues and source bytes remain in ignored local evidence. This audit does not select providers, substitute standalone packages, prove historical completeness, or authorize compilation.\n",
        summary.unique_program_units_in_union,
        summary.duplicate_provider_groups,
        summary.catalogue_entries_without_source,
        summary.source_units_without_catalogue_entry,
        summary.toc_user_callable_entries,
        summary.toc_subsidiary_entries,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provider(
        subset: &str,
        relationship: &str,
        raw: &str,
        normalized: &str,
        name: &str,
    ) -> Provider {
        Provider {
            id: format!("provider-{subset}-{name}-{raw}"),
            subset: subset.to_owned(),
            relationship: relationship.to_owned(),
            path: format!("{name}.f"),
            raw_sha256: raw.to_owned(),
            normalized_sha256: normalized.to_owned(),
            normalized_name: name.to_owned(),
            kind: "subroutine".to_owned(),
        }
    }

    #[test]
    fn directory_index_selects_only_safe_fortran_members() {
        let index = br#"<a href="alpha.f">alpha</a><a href="guide">guide</a><a href="../escape.f">bad</a><a href="Beta.F">other</a>"#;
        assert_eq!(directory_file_names(index), vec!["Beta.F", "alpha.f"]);
    }

    #[test]
    fn fixed_form_inventory_reads_multiple_program_units_without_raw_output() {
        let source = b"      SUBROUTINE ALPHA\n      END\n      REAL FUNCTION BETA(X)\n      END\n";
        let file = AcquiredFile {
            subset: "synthetic".to_owned(),
            relationship: "test".to_owned(),
            path: "fixture.f".to_owned(),
            url: "https://example.invalid/fixture.f".to_owned(),
            bytes: source.to_vec(),
        };
        let (inventory, providers) = inventory_acquired_file(&file);
        assert_eq!(inventory.provider_ids.len(), 2);
        assert_eq!(providers.len(), 2);
        assert!(
            providers
                .iter()
                .any(|provider| provider.normalized_name == "ALPHA")
        );
        assert!(
            providers
                .iter()
                .any(|provider| provider.normalized_name == "BETA")
        );
        assert!(!format!("{:?}", file.bytes).contains("SUBROUTINE"));
    }

    #[test]
    fn relationship_classification_keeps_variants_and_relocations_distinct() {
        let main = provider(
            "main-src",
            "pinned-reproducible-subset",
            "raw",
            "norm",
            "ALPHA",
        );
        let relocated = provider("lin", "relocated-subset", "raw", "norm", "ALPHA");
        assert_eq!(
            relationship_class(&[&main, &relocated]),
            "byte_identical_relocated_copy"
        );
        let normalized = provider("lin", "relocated-subset", "changed", "norm", "ALPHA");
        assert_eq!(
            relationship_class(&[&main, &normalized]),
            "normalized_identical_relocated_copy"
        );
        let modified = provider("lin", "relocated-subset", "changed", "other", "ALPHA");
        assert_eq!(
            relationship_class(&[&main, &modified]),
            "modified_relocated_copy"
        );
        let historical = provider(
            "main-src-historical",
            "historical-variant",
            "old",
            "old",
            "ALPHA",
        );
        assert_eq!(
            relationship_class(&[&main, &historical]),
            "historical_variant"
        );
    }

    #[test]
    fn catalogue_parser_tracks_user_callable_and_subsidiary_markers() {
        let toc = b"ALPHA-S *BETA-D GAMMA-A";
        let entries = catalogue_entries(b"     1.  ALPHA   has 2 records\n", toc, &mut Vec::new());
        assert_eq!(entries["ALPHA"].toc_classification, "user_callable");
        assert_eq!(entries["BETA"].toc_classification, "subsidiary");
        assert!(entries["ALPHA"].in_list);
    }

    #[test]
    fn semantic_inputs_are_deterministic_and_line_normalization_is_reversible() {
        assert_eq!(normalize_line_endings(b"A\r\nB\rC\n"), b"A\nB\nC\n");
        let mut first = BTreeMap::new();
        first.insert("b.json", b"b\n".to_vec());
        first.insert("a.json", b"a\n".to_vec());
        let mut second = BTreeMap::new();
        second.insert("a.json", b"a\n".to_vec());
        second.insert("b.json", b"b\n".to_vec());
        assert_eq!(semantic_hash(&first), semantic_hash(&second));
    }

    #[test]
    fn synthetic_audit_is_offline_deterministic_and_compact() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let config = AuditConfig {
            schema_version: 1,
            policy_version: 1,
            catalogues: Catalogues {
                list: "https://example.invalid/list".to_owned(),
                toc: "https://example.invalid/toc".to_owned(),
            },
            source_sets: vec![SourceSet {
                id: "lin".to_owned(),
                url: "https://example.invalid/lin/".to_owned(),
                relationship: "relocated-subset".to_owned(),
            }],
            supplemental_sets: Vec::new(),
            supplemental_files: Vec::new(),
        };
        write_audit_fixture(first.path());
        write_audit_fixture(second.path());
        let left = audit_with_config(
            &config,
            b"synthetic-audit-v1",
            &first.path().join("evidence"),
            &first.path().join("corpus"),
            &first.path().join("program-units"),
            &first.path().join("out"),
            true,
        )
        .unwrap();
        let right = audit_with_config(
            &config,
            b"synthetic-audit-v1",
            &second.path().join("evidence"),
            &second.path().join("corpus"),
            &second.path().join("program-units"),
            &second.path().join("out"),
            true,
        )
        .unwrap();
        assert_eq!(left.semantic_hash, right.semantic_hash);
        assert_eq!(left.summary.unique_program_units_in_union, 1);
        for file in [
            "artifact-index.json",
            "source-file-index.json",
            "program-unit-union.json",
            "provider-relationships.json",
            "catalogue-comparison.json",
            "diagnostics.json",
            "manifest.json",
            "audit-summary.md",
        ] {
            let a = fs::read(first.path().join("out").join(file)).unwrap();
            let b = fs::read(second.path().join("out").join(file)).unwrap();
            assert_eq!(a, b, "{file}");
            assert!(a.len() < 50_000, "{file} is unexpectedly large");
            assert!(!String::from_utf8_lossy(&a).contains("SUBROUTINE"));
        }
    }

    fn write_audit_fixture(root: &Path) {
        let source = b"      SUBROUTINE ALPHA\n      END\n";
        let sha = hash::bytes(source);
        let evidence = root.join("evidence/extracted/test-snapshot/slatec-source-archive/src");
        fs::create_dir_all(&evidence).unwrap();
        fs::write(evidence.join("test.f"), source).unwrap();
        let input = root.join("evidence/full-corpus/audit-input");
        fs::create_dir_all(input.join("catalogues")).unwrap();
        fs::create_dir_all(input.join("directories/lin/files")).unwrap();
        fs::write(input.join("catalogues/list"), b"ALPHA-S\n").unwrap();
        fs::write(input.join("catalogues/toc"), b"ALPHA-S\n").unwrap();
        fs::write(
            input.join("directories/lin/index.html"),
            b"<a href=\"test.f\">test</a>",
        )
        .unwrap();
        fs::write(input.join("directories/lin/files/test.f"), source).unwrap();
        let corpus = root.join("corpus");
        let units = root.join("program-units");
        fs::create_dir_all(&corpus).unwrap();
        fs::create_dir_all(&units).unwrap();
        fs::write(
            corpus.join("artifact-manifest.json"),
            compact(&json!({"snapshot_id":"test-snapshot","sha256":"a".repeat(64)})).unwrap(),
        )
        .unwrap();
        fs::write(
            corpus.join("source-files.json"),
            compact(&json!({"records":[{
                "id":"source-test", "snapshot_id":"test-snapshot", "artifact_id":"slatec-source-archive", "archive_member_path":"src/test.f", "member_sha256":sha, "selected_state":"selected"
            }]}))
            .unwrap(),
        )
        .unwrap();
        fs::write(
            units.join("manifest.json"),
            compact(&json!({"snapshot_id":"test-snapshot","output_semantic_hash":"b".repeat(64)}))
                .unwrap(),
        )
        .unwrap();
        fs::write(
            units.join("program-units.json"),
            compact(&json!({"records":[{
                "id":"program-alpha", "snapshot_id":"test-snapshot", "source_file_id":"source-test", "name":"ALPHA", "normalized_name":"ALPHA", "kind":"subroutine"
            }]}))
            .unwrap(),
        )
        .unwrap();
    }
}
