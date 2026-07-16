use crate::error::{CorpusError, Result};
use crate::fixed_form::{self, LexDiagnostic, LogicalStatement, PhysicalLine, RawSpan};
use crate::hash;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: &str = "1.0.0";
const SCANNER_SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";

#[derive(Deserialize)]
struct Records<T> {
    records: Vec<T>,
}

#[derive(Deserialize)]
struct ArtifactManifest {
    snapshot_id: String,
    sha256: String,
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

#[derive(Deserialize)]
struct Provider {
    snapshot_id: String,
    source_file_id: String,
    member_sha256: String,
    selection_state: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScanResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
    pub summary: ScanSummary,
}

#[derive(Clone, Debug, Serialize)]
pub struct ScanSummary {
    pub files_attempted: usize,
    pub files_fully_scanned: usize,
    pub files_with_review_items: usize,
    pub physical_line_count: usize,
    pub logical_statement_count: usize,
    pub top_level_program_unit_count: usize,
    pub entry_count: usize,
    pub block_data_count: usize,
    pub duplicate_normalized_identifiers: usize,
    pub files_with_zero_program_units: usize,
    pub files_with_multiple_program_units: usize,
    pub unsupported_or_ambiguous_constructs: usize,
}

#[derive(Clone)]
struct SourceContext {
    source: SourceFile,
    bytes: Vec<u8>,
}

#[derive(Clone)]
struct Unit {
    id: String,
    source_file_id: String,
    source_artifact_id: String,
    path: String,
    member_sha256: String,
    snapshot_id: String,
    name: Option<String>,
    normalized_name: Option<String>,
    kind: String,
    declared_return_type: Option<String>,
    declaration: RawSpan,
    end: Option<RawSpan>,
    entries: Vec<Entry>,
    diagnostics: Vec<ScanDiagnostic>,
    parse_status: String,
}

#[derive(Clone)]
struct Entry {
    id: String,
    name: String,
    normalized_name: String,
    arguments: Vec<String>,
    locator: RawSpan,
}

#[derive(Clone)]
struct ScanDiagnostic {
    id: String,
    rule_id: String,
    severity: String,
    message: String,
    source_file_id: String,
    path: String,
    span: RawSpan,
}

pub fn scan(
    evidence_dir: &Path,
    manifest_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ScanResult> {
    if !offline {
        return Err(CorpusError::Policy("scan-program-units is evidence-only and requires --offline; acquire with prepare first".to_owned()));
    }
    let artifact: ArtifactManifest = read_json(&manifest_dir.join("artifact-manifest.json"))?;
    let sources: Records<SourceFile> = read_json(&manifest_dir.join("source-files.json"))?;
    let providers: Records<Provider> = read_json(&manifest_dir.join("selected-providers.json"))?;
    let source_manifest_hash = hash::file(&manifest_dir.join("source-files.json"))?;
    let provider_manifest_hash = hash::file(&manifest_dir.join("selected-providers.json"))?;
    let snapshot = artifact.snapshot_id.clone();
    let mut provider_ids = BTreeMap::new();
    for provider in providers.records {
        if provider.snapshot_id != snapshot || provider.selection_state != "selected" {
            return Err(CorpusError::Verification(
                "selected-provider manifest snapshot/state mismatch".to_owned(),
            ));
        }
        if provider_ids
            .insert(provider.source_file_id, provider.member_sha256)
            .is_some()
        {
            return Err(CorpusError::Verification(
                "duplicate source-file identity in selected-provider manifest".to_owned(),
            ));
        }
    }
    let mut selected = Vec::new();
    let mut source_ids = BTreeSet::new();
    for source in sources.records {
        if source.snapshot_id != snapshot {
            return Err(CorpusError::Verification(
                "source-file manifest snapshot mismatch".to_owned(),
            ));
        }
        if !source_ids.insert(source.id.clone()) {
            return Err(CorpusError::Verification(
                "duplicate source-file identity in source manifest".to_owned(),
            ));
        }
        if source.selected_state == "selected" {
            let provider_hash = provider_ids.get(&source.id).ok_or_else(|| {
                CorpusError::Verification(format!(
                    "selected source {} lacks selected-provider record",
                    source.id
                ))
            })?;
            if provider_hash != &source.member_sha256 {
                return Err(CorpusError::Verification(format!(
                    "selected source hash mismatch in manifests for {}",
                    source.id
                )));
            }
            selected.push(source);
        }
    }
    if selected.len() != 735 || provider_ids.len() != selected.len() {
        return Err(CorpusError::Verification(format!(
            "selected source manifest count mismatch: expected 735, got {}",
            selected.len()
        )));
    }
    selected.sort_by(|left, right| left.archive_member_path.cmp(&right.archive_member_path));
    let contexts: Vec<_> = selected
        .into_iter()
        .map(|source| {
            let path = evidence_dir
                .join("extracted")
                .join(&snapshot)
                .join(&source.artifact_id)
                .join(&source.archive_member_path);
            let bytes = fs::read(&path).map_err(|_| {
                CorpusError::Verification(format!(
                    "missing extracted selected source {}",
                    path.display()
                ))
            })?;
            if hash::bytes(&bytes) != source.member_sha256 {
                return Err(CorpusError::Verification(format!(
                    "raw extracted source hash mismatch for {}",
                    source.archive_member_path
                )));
            }
            Ok(SourceContext { source, bytes })
        })
        .collect::<Result<_>>()?;
    let config_hash = hash::bytes(format!("fixed-form-v{SCANNER_SEMANTIC_VERSION}\u{1f}{}\u{1f}{source_manifest_hash}\u{1f}{provider_manifest_hash}", artifact.sha256).as_bytes());
    let filename_collisions = filename_collision_groups(&contexts);
    let mut all_lines = Vec::new();
    let mut all_statements = Vec::new();
    let mut source_index = Vec::new();
    let mut units = Vec::new();
    let mut diagnostics = Vec::new();
    let mut files_with_review = 0;
    let mut zero_units = 0;
    let mut multi_units = 0;
    for context in contexts {
        let lines = fixed_form::physical_lines(&context.bytes);
        let statements = fixed_form::logical_statements(&lines);
        let (file_units, mut file_diagnostics) = inventory_file(&context, &statements);
        let line_diags: Vec<_> = lines
            .iter()
            .flat_map(|line| line.diagnostics.iter())
            .map(|diag| scan_diag(&context.source, &context.source.archive_member_path, diag))
            .collect();
        let statement_diags: Vec<_> = statements
            .iter()
            .flat_map(|statement| statement.diagnostics.iter())
            .map(|diag| scan_diag(&context.source, &context.source.archive_member_path, diag))
            .collect();
        if !line_diags.is_empty() || !statement_diags.is_empty() || !file_diagnostics.is_empty() {
            files_with_review += 1;
        }
        if file_units.is_empty() {
            zero_units += 1;
        }
        if file_units.len() > 1 {
            multi_units += 1;
        }
        diagnostics.extend(line_diags);
        diagnostics.extend(statement_diags);
        diagnostics.append(&mut file_diagnostics);
        let line_values: Vec<_> = lines
            .into_iter()
            .map(|line| physical_value(&context, line))
            .collect();
        let statement_values: Vec<_> = statements
            .into_iter()
            .map(|statement| statement_value(&context, statement))
            .collect();
        source_index.push(json!({
            "id": stable_id("source-scan-index", &[&context.source.id, &context.source.member_sha256]),
            "schema_id": "slatec-rs/source-scan-index",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "source_record_ids": [context.source.id],
            "review_state": if line_values.iter().any(has_diagnostics) || statement_values.iter().any(has_diagnostics) { "review_requested" } else { "machine_checked" },
            "source_file_id": context.source.id,
            "source_content_sha256": context.source.member_sha256,
            "physical_line_count": line_values.len(),
            "logical_statement_count": statement_values.len(),
            "physical_inventory_sha256": hash::bytes(&compact(&json!(line_values))?),
            "logical_inventory_sha256": hash::bytes(&compact(&json!(statement_values))?),
        }));
        all_lines.extend(line_values);
        all_statements.extend(statement_values);
        units.extend(file_units);
    }
    let duplicates = duplicate_groups(&units);
    for duplicate in &duplicates {
        diagnostics.push(ScanDiagnostic {
            id: stable_id(
                "diagnostic",
                &[&duplicate.identifier, &duplicate.collision_type],
            ),
            rule_id: "FF-DUPLICATE-PROVIDER".to_owned(),
            severity: "error".to_owned(),
            message: "unresolved duplicate provider identity".to_owned(),
            source_file_id: duplicate.records[0].source_file_id.clone(),
            path: duplicate.records[0].path.clone(),
            span: duplicate.records[0].span.clone(),
        });
    }
    diagnostics.sort_by(|left, right| {
        (
            &left.severity,
            &left.path,
            left.span.start,
            &left.rule_id,
            &left.id,
        )
            .cmp(&(
                &right.severity,
                &right.path,
                right.span.start,
                &right.rule_id,
                &right.id,
            ))
    });
    let entry_count = units.iter().map(|unit| unit.entries.len()).sum();
    let block_data_count = units
        .iter()
        .filter(|unit| unit.kind == "block_data")
        .count();
    let summary = ScanSummary {
        files_attempted: 735,
        // A review item affects confidence, not whether the file was scanned.
        files_fully_scanned: 735,
        files_with_review_items: files_with_review,
        physical_line_count: all_lines.len(),
        logical_statement_count: all_statements.len(),
        top_level_program_unit_count: units.len(),
        entry_count,
        block_data_count,
        duplicate_normalized_identifiers: duplicates.len(),
        files_with_zero_program_units: zero_units,
        files_with_multiple_program_units: multi_units,
        unsupported_or_ambiguous_constructs: diagnostics.len(),
    };
    let status = if !duplicates.is_empty() {
        "failed"
    } else if diagnostics.is_empty() {
        "success"
    } else {
        "success_with_review_items"
    }
    .to_owned();
    let unit_values: Vec<_> = units.iter().map(unit_value).collect();
    let entry_values: Vec<_> = units
        .iter()
        .flat_map(|unit| unit.entries.iter().map(|entry| entry_value(unit, entry)))
        .collect();
    let diagnostic_values: Vec<_> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic_value(&snapshot, diagnostic))
        .collect();
    let mut evidence_files = BTreeMap::new();
    evidence_files.insert(
        "physical-lines.json",
        compact(&json!({ "schema_id": "slatec-rs/physical-line-inventory", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "scanner_semantic_version": SCANNER_SEMANTIC_VERSION, "records": all_lines }))?,
    );
    evidence_files.insert(
        "logical-statements.json",
        compact(&json!({ "schema_id": "slatec-rs/logical-statement-inventory", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "scanner_semantic_version": SCANNER_SEMANTIC_VERSION, "records": all_statements }))?,
    );
    let evidence_hashes: BTreeMap<_, _> = evidence_files
        .iter()
        .map(|(name, content)| ((*name).to_owned(), hash::bytes(content)))
        .collect();
    let evidence_output_dir = evidence_dir.join("scans").join(&snapshot);
    promote(&evidence_output_dir, &snapshot, &evidence_files)?;
    let mut files = BTreeMap::new();
    files.insert(
        "source-scan-index.json",
        canonical(&json!({ "schema_id": "slatec-rs/source-scan-index-set", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": ["artifact-slatec-source-archive"], "review_state": "machine_checked", "scanner_semantic_version": SCANNER_SEMANTIC_VERSION, "records": source_index }))?,
    );
    files.insert(
        "program-units.json",
        canonical(&json!({ "schema_id": "slatec-rs/program-unit-set", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": ["artifact-slatec-source-archive"], "review_state": "machine_checked", "scanner_semantic_version": SCANNER_SEMANTIC_VERSION, "records": unit_values }))?,
    );
    files.insert(
        "entry-points.json",
        canonical(&json!({ "schema_id": "slatec-rs/entry-point-set", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": ["artifact-slatec-source-archive"], "review_state": "machine_checked", "scanner_semantic_version": SCANNER_SEMANTIC_VERSION, "records": entry_values }))?,
    );
    files.insert(
        "duplicate-providers.json",
        canonical(
            &json!({ "schema_id": "slatec-rs/duplicate-provider-set", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": ["artifact-slatec-source-archive"], "review_state": if duplicates.is_empty() { "machine_checked" } else { "review_requested" }, "scanner_semantic_version": SCANNER_SEMANTIC_VERSION, "records": duplicates.iter().map(|duplicate| duplicate_value(&snapshot, duplicate)).collect::<Vec<_>>(), "filename_collisions_without_program_unit_collision": filename_collisions.iter().map(|collision| filename_collision_value(&snapshot, collision)).collect::<Vec<_>>() }),
        )?,
    );
    files.insert(
        "diagnostics.json",
        canonical(&json!({ "schema_id": "slatec-rs/program-unit-diagnostic-set", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": ["artifact-slatec-source-archive"], "review_state": if diagnostics.is_empty() { "machine_checked" } else { "review_requested" }, "scanner_semantic_version": SCANNER_SEMANTIC_VERSION, "stage_status": status, "records": diagnostic_values }))?,
    );
    let mut semantic_inputs: BTreeMap<String, Vec<u8>> = files
        .iter()
        .map(|(name, content)| ((*name).to_owned(), content.clone()))
        .collect();
    for (name, content) in &evidence_files {
        semantic_inputs.insert(format!("evidence/{name}"), content.clone());
    }
    let semantic_hash = semantic_hash(&semantic_inputs);
    let file_hashes: BTreeMap<_, _> = files
        .iter()
        .map(|(name, content)| ((*name).to_owned(), hash::bytes(content)))
        .collect();
    files.insert("manifest.json", canonical(&json!({ "id": stable_id("program-unit-scan", &[&snapshot]), "schema_id": "slatec-rs/program-unit-scan", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": ["artifact-slatec-source-archive"], "review_state": "machine_checked", "scanner_semantic_version": SCANNER_SEMANTIC_VERSION, "source_manifest_semantic_hash": source_manifest_hash, "selected_provider_manifest_hash": provider_manifest_hash, "configuration_semantic_hash": config_hash, "output_semantic_hash": semantic_hash, "output_file_hashes": file_hashes, "evidence_output_relative_dir": format!("scans/{snapshot}"), "evidence_output_file_hashes": evidence_hashes, "coverage_status": if diagnostics.is_empty() { "complete_verified" } else { "complete_unreviewed" }, "stage_status": status, "diagnostic_count": diagnostics.len(), "unsupported_case_count": diagnostics.len(), "summary": summary }))?);
    files.insert(
        "scan-summary.md",
        summary_markdown(&snapshot, &semantic_hash, &status, &summary).into_bytes(),
    );
    promote(output_dir, &snapshot, &files)?;
    Ok(ScanResult {
        snapshot_id: snapshot,
        status,
        semantic_hash,
        output_dir: output_dir.to_owned(),
        summary,
    })
}

fn inventory_file(
    context: &SourceContext,
    statements: &[LogicalStatement],
) -> (Vec<Unit>, Vec<ScanDiagnostic>) {
    let mut units = Vec::new();
    let mut diagnostics = Vec::new();
    let mut open: Option<Unit> = None;
    for statement in statements {
        if let Some(start) = fixed_form::start_declaration(&statement.normalized_statement_text) {
            if let Some(mut prior) = open.take() {
                prior.parse_status = "unterminated".to_owned();
                let diagnostic = custom_diag(
                    context,
                    "FF-MISSING-END",
                    "warning",
                    "program unit ended implicitly by another start",
                    &statement.raw_spans[0],
                );
                prior.diagnostics.push(diagnostic.clone());
                diagnostics.push(diagnostic);
                units.push(prior);
            }
            let span = statement.raw_spans[0].clone();
            let normalized_name = start.name.as_ref().map(|name| name.to_ascii_uppercase());
            let id = stable_id(
                "program-unit",
                &[
                    &context.source.member_sha256,
                    &start.kind,
                    normalized_name.as_deref().unwrap_or("<unnamed>"),
                    &span.start.to_string(),
                ],
            );
            open = Some(Unit {
                id,
                source_file_id: context.source.id.clone(),
                source_artifact_id: context.source.artifact_id.clone(),
                path: context.source.archive_member_path.clone(),
                member_sha256: context.source.member_sha256.clone(),
                snapshot_id: context.source.snapshot_id.clone(),
                name: start.name,
                normalized_name,
                kind: start.kind,
                declared_return_type: start.declared_return_type,
                declaration: span,
                end: None,
                entries: Vec::new(),
                diagnostics: Vec::new(),
                parse_status: "complete".to_owned(),
            });
        } else if fixed_form::is_end(&statement.normalized_statement_text) {
            if let Some(mut unit) = open.take() {
                unit.end = Some(statement.raw_spans[0].clone());
                units.push(unit);
            } else {
                diagnostics.push(custom_diag(
                    context,
                    "FF-UNEXPECTED-END",
                    "warning",
                    "END appears outside a program unit",
                    &statement.raw_spans[0],
                ));
            }
        } else if let Some((name, arguments)) =
            fixed_form::entry_declaration(&statement.normalized_statement_text)
        {
            if let Some(unit) = open.as_mut() {
                let span = statement.raw_spans[0].clone();
                unit.entries.push(Entry {
                    id: stable_id(
                        "entry",
                        &[
                            &unit.id,
                            &name.to_ascii_uppercase(),
                            &span.start.to_string(),
                        ],
                    ),
                    normalized_name: name.to_ascii_uppercase(),
                    name,
                    arguments,
                    locator: span,
                });
            } else {
                diagnostics.push(custom_diag(
                    context,
                    "FF-ENTRY-OUTSIDE-UNIT",
                    "warning",
                    "ENTRY appears outside a program unit",
                    &statement.raw_spans[0],
                ));
            }
        } else if open.is_none()
            && statement.classification == "other"
            && !statement.normalized_statement_text.is_empty()
        {
            diagnostics.push(custom_diag(
                context,
                "FF-TOPLEVEL-STATEMENT",
                "warning",
                "statement outside a recognized program unit",
                &statement.raw_spans[0],
            ));
        }
    }
    if let Some(mut unit) = open {
        unit.parse_status = "unterminated".to_owned();
        let diagnostic = custom_diag(
            context,
            "FF-MISSING-END",
            "warning",
            "source ended before END",
            &unit.declaration,
        );
        unit.diagnostics.push(diagnostic.clone());
        diagnostics.push(diagnostic);
        units.push(unit);
    }
    (units, diagnostics)
}

#[derive(Clone)]
struct Duplicate {
    identifier: String,
    collision_type: String,
    case_only_spelling_variation: bool,
    records: Vec<DuplicateRecord>,
}
#[derive(Clone)]
struct DuplicateRecord {
    kind: String,
    source_file_id: String,
    path: String,
    span: RawSpan,
    spelling: String,
}
#[derive(Clone)]
struct FilenameCollision {
    normalized_filename: String,
    source_file_ids: Vec<String>,
    paths: Vec<String>,
}
fn filename_collision_groups(contexts: &[SourceContext]) -> Vec<FilenameCollision> {
    let mut groups: BTreeMap<String, Vec<&SourceContext>> = BTreeMap::new();
    for context in contexts {
        let filename = Path::new(&context.source.archive_member_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&context.source.archive_member_path)
            .to_ascii_uppercase();
        groups.entry(filename).or_default().push(context);
    }
    groups
        .into_iter()
        .filter_map(|(normalized_filename, records)| {
            (records.len() > 1).then(|| FilenameCollision {
                normalized_filename,
                source_file_ids: records
                    .iter()
                    .map(|context| context.source.id.clone())
                    .collect(),
                paths: records
                    .iter()
                    .map(|context| context.source.archive_member_path.clone())
                    .collect(),
            })
        })
        .collect()
}
fn duplicate_groups(units: &[Unit]) -> Vec<Duplicate> {
    let mut groups: BTreeMap<String, Vec<DuplicateRecord>> = BTreeMap::new();
    for unit in units {
        if let (Some(name), Some(normalized)) = (&unit.name, &unit.normalized_name) {
            groups
                .entry(normalized.clone())
                .or_default()
                .push(DuplicateRecord {
                    kind: "program_unit".to_owned(),
                    source_file_id: unit.source_file_id.clone(),
                    path: unit.path.clone(),
                    span: unit.declaration.clone(),
                    spelling: name.clone(),
                });
        }
        for entry in &unit.entries {
            groups
                .entry(entry.normalized_name.clone())
                .or_default()
                .push(DuplicateRecord {
                    kind: "entry".to_owned(),
                    source_file_id: unit.source_file_id.clone(),
                    path: unit.path.clone(),
                    span: entry.locator.clone(),
                    spelling: entry.name.clone(),
                });
        }
    }
    groups
        .into_iter()
        .filter_map(|(identifier, mut records)| {
            if records.len() < 2 {
                return None;
            }
            records.sort_by(|left, right| {
                (&left.path, left.span.start, &left.kind).cmp(&(
                    &right.path,
                    right.span.start,
                    &right.kind,
                ))
            });
            let kinds: BTreeSet<_> = records.iter().map(|record| record.kind.as_str()).collect();
            let collision_type = if kinds.len() == 1 && kinds.contains("program_unit") {
                "duplicate_top_level_program_units"
            } else if kinds.len() == 1 {
                "multiple_entry_providers"
            } else {
                "program_unit_entry_collision"
            }
            .to_owned();
            let spellings: BTreeSet<_> = records.iter().map(|record| &record.spelling).collect();
            Some(Duplicate {
                identifier,
                collision_type,
                case_only_spelling_variation: spellings.len() > 1,
                records,
            })
        })
        .collect()
}

fn physical_value(context: &SourceContext, line: PhysicalLine) -> Value {
    json!({ "id": stable_id("physical-line", &[&context.source.member_sha256, &line.span.start.to_string(), &line.span.end.to_string()]), "source_file_id": context.source.id, "span": compact_span(&line.span), "line_ending": line.line_ending, "classification": format!("{:?}", line.kind).to_ascii_lowercase(), "statement_label": line.label, "continuation": line.continuation, "statement_field_sha256": hash::bytes(&line.statement_field), "trailing_field_sha256": hash::bytes(&line.trailing_field), "diagnostic_ids": line.diagnostics.iter().map(|diag| diagnostic_id(&context.source.id, &context.source.archive_member_path, diag)).collect::<Vec<_>>() })
}
fn statement_value(context: &SourceContext, statement: LogicalStatement) -> Value {
    let span = &statement.raw_spans[0];
    json!({ "id": stable_id("logical-statement", &[&context.source.member_sha256, &span.start.to_string(), &span.end.to_string(), &statement.classification]), "source_file_id": context.source.id, "raw_spans": statement.raw_spans.iter().map(compact_span).collect::<Vec<_>>(), "physical_line_start": statement.physical_line_start, "physical_line_end": statement.physical_line_end, "statement_label": statement.statement_label, "raw_statement_text_sha256": fixed_form::statement_hash(&statement), "normalized_statement_text_sha256": hash::bytes(statement.normalized_statement_text.as_bytes()), "continuation_count": statement.continuation_count, "classification": statement.classification, "lex_status": if statement.diagnostics.is_empty() { "complete" } else { "review_required" }, "diagnostic_ids": statement.diagnostics.iter().map(|diag| diagnostic_id(&context.source.id, &context.source.archive_member_path, diag)).collect::<Vec<_>>() })
}
fn unit_value(unit: &Unit) -> Value {
    json!({ "id": unit.id, "schema_id": "slatec-rs/program-unit", "schema_version": SCHEMA_VERSION, "snapshot_id": unit.snapshot_id, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": [unit.source_file_id], "review_state": if unit.diagnostics.is_empty() && unit.parse_status == "complete" { "machine_checked" } else { "review_requested" }, "source_file_id": unit.source_file_id, "source_artifact_id": unit.source_artifact_id, "canonical_provider": true, "name": unit.name, "normalized_name": unit.normalized_name, "kind": unit.kind, "declared_return_type": unit.declared_return_type, "declaration_locator": compact_locator(unit, &unit.declaration), "source_span": compact_locator(unit, &unit.declaration), "entry_points": { "collection_state": if unit.entries.is_empty() { "known_empty" } else { "known_values" }, "record_ids": unit.entries.iter().map(|entry| entry.id.clone()).collect::<Vec<_>>() }, "end_statement_locator": unit.end.as_ref().map(|span| compact_locator(unit, span)), "parse_status": unit.parse_status, "coverage_status": if unit.parse_status == "complete" { "complete_unreviewed" } else { "partial" }, "claim_ids": [], "diagnostic_ids": unit.diagnostics.iter().map(|diag| diag.id.clone()).collect::<Vec<_>>() })
}
fn entry_value(unit: &Unit, entry: &Entry) -> Value {
    json!({ "id": entry.id, "schema_id": "slatec-rs/entry-point", "schema_version": SCHEMA_VERSION, "snapshot_id": unit.snapshot_id, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": [unit.source_file_id], "review_state": "machine_checked", "name": entry.name, "normalized_name": entry.normalized_name, "containing_program_unit_id": unit.id, "source_locator": compact_locator(unit, &entry.locator), "arguments": { "collection_state": if entry.arguments.is_empty() { "known_empty" } else { "known_values" }, "spelling": entry.arguments }, "fact_status": "verified" })
}
fn duplicate_value(snapshot: &str, duplicate: &Duplicate) -> Value {
    json!({ "id": stable_id("duplicate-provider", &[snapshot, &duplicate.identifier, &duplicate.collision_type]), "schema_id": "slatec-rs/duplicate-provider", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": duplicate.records.iter().map(|record| record.source_file_id.clone()).collect::<Vec<_>>(), "review_state": "review_requested", "normalized_identifier": duplicate.identifier, "collision_type": duplicate.collision_type, "case_only_spelling_variation": duplicate.case_only_spelling_variation, "records": duplicate.records.iter().map(|record| json!({ "kind": record.kind, "source_file_id": record.source_file_id, "path": record.path, "spelling": record.spelling, "span": { "start": record.span.start, "end": record.span.end, "line": record.span.line } })).collect::<Vec<_>>(), "resolution_state": "open" })
}
fn filename_collision_value(snapshot: &str, collision: &FilenameCollision) -> Value {
    json!({ "id": stable_id("filename-collision", &[snapshot, &collision.normalized_filename]), "schema_id": "slatec-rs/filename-collision", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": collision.source_file_ids, "review_state": "machine_checked", "normalized_filename": collision.normalized_filename, "paths": collision.paths, "collision_scope": "filename_only" })
}
fn diagnostic_value(snapshot: &str, diag: &ScanDiagnostic) -> Value {
    json!({ "id": diag.id, "schema_id": "slatec-rs/diagnostic", "schema_version": SCHEMA_VERSION, "snapshot_id": snapshot, "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at": CREATED_AT, "source_record_ids": [diag.source_file_id], "review_state": "review_requested", "rule_id": diag.rule_id, "severity": diag.severity, "stage": "program_unit_scan", "message_template_id": diag.rule_id, "arguments": [diag.message], "locator": { "member_path": diag.path, "span": { "start": diag.span.start, "end": diag.span.end, "line": diag.span.line } }, "review_impact": "review" })
}
fn compact_locator(unit: &Unit, span: &RawSpan) -> Value {
    json!({ "member_path": unit.path, "member_sha256": unit.member_sha256, "representation": "raw_bytes", "span": { "bytes": format!("{}-{}", span.start, span.end), "lines": format!("{}-{}", span.line, span.line) } })
}
fn compact_span(span: &RawSpan) -> Value {
    json!({ "start": span.start, "end": span.end, "line": span.line, "column_start": span.column_start, "column_end": span.column_end })
}
fn scan_diag(source: &SourceFile, path: &str, diag: &LexDiagnostic) -> ScanDiagnostic {
    ScanDiagnostic {
        id: diagnostic_id(&source.id, path, diag),
        rule_id: diag.rule_id.to_owned(),
        severity: diag.severity.to_owned(),
        message: diag.message.to_owned(),
        source_file_id: source.id.clone(),
        path: path.to_owned(),
        span: diag.span.clone(),
    }
}
fn custom_diag(
    context: &SourceContext,
    rule: &str,
    severity: &str,
    message: &str,
    span: &RawSpan,
) -> ScanDiagnostic {
    ScanDiagnostic {
        id: stable_id(
            "diagnostic",
            &[&context.source.id, rule, &span.start.to_string()],
        ),
        rule_id: rule.to_owned(),
        severity: severity.to_owned(),
        message: message.to_owned(),
        source_file_id: context.source.id.clone(),
        path: context.source.archive_member_path.clone(),
        span: span.clone(),
    }
}
fn diagnostic_id(source_id: &str, path: &str, diag: &LexDiagnostic) -> String {
    stable_id(
        "diagnostic",
        &[source_id, path, diag.rule_id, &diag.span.start.to_string()],
    )
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
fn canonical(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn compact(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn semantic_hash(files: &BTreeMap<String, Vec<u8>>) -> String {
    let mut material = Vec::new();
    for (name, bytes) in files {
        material.extend_from_slice(name.as_bytes());
        material.push(0);
        material.extend_from_slice(bytes);
        material.push(0);
    }
    hash::bytes(&material)
}
fn has_diagnostics(record: &Value) -> bool {
    record
        .get("diagnostic_ids")
        .and_then(Value::as_array)
        .is_some_and(|diagnostics| !diagnostics.is_empty())
}
fn promote(output_dir: &Path, snapshot: &str, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let parent = output_dir
        .parent()
        .ok_or_else(|| CorpusError::Policy("output directory must have parent".to_owned()))?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        "{}.staging-{snapshot}",
        output_dir.file_name().unwrap_or_default().to_string_lossy()
    ));
    if staging.exists() {
        return Err(CorpusError::Verification(format!(
            "scanner staging directory exists: {}",
            staging.display()
        )));
    }
    fs::create_dir(&staging)?;
    for (name, bytes) in files {
        fs::write(staging.join(name), bytes)?;
    }
    if output_dir.exists() {
        for (name, bytes) in files {
            if fs::read(output_dir.join(name)).ok().as_deref() != Some(bytes.as_slice()) {
                return Err(CorpusError::Verification(format!(
                    "existing scanner output differs: {}",
                    output_dir.display()
                )));
            }
        }
        fs::remove_dir_all(staging)?;
    } else {
        fs::rename(staging, output_dir)?;
    }
    Ok(())
}
fn summary_markdown(
    snapshot: &str,
    semantic_hash: &str,
    status: &str,
    summary: &ScanSummary,
) -> String {
    format!(
        "# Fixed-form program-unit scan\n\n- Snapshot: `{snapshot}`\n- Semantic hash: `{semantic_hash}`\n- Stage status: `{status}`\n- Files attempted: {}\n- Program units: {}\n- Entry points: {}\n- Duplicate normalized identifiers: {}\n\nThe inventory is scoped to the checksum-pinned selected corpus and scanner semantic version {SCANNER_SEMANTIC_VERSION}; it is not evidence about live or upstream provider sets.\n",
        summary.files_attempted,
        summary.top_level_program_unit_count,
        summary.entry_count,
        summary.duplicate_normalized_identifiers
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_fixed_form_program_unit_declarations() {
        for (source, kind, name) in [
            (
                "      SUBROUTINE ALPHA\n      END\n",
                "subroutine",
                Some("ALPHA"),
            ),
            (
                "      FUNCTION BETA(X)\n      END FUNCTION\n",
                "function",
                Some("BETA"),
            ),
            (
                "      DOUBLE PRECISION FUNCTION GAMMA(X)\n      END\n",
                "function",
                Some("GAMMA"),
            ),
            (
                "      CHARACTER*8 FUNCTION DELTA()\n      END\n",
                "function",
                Some("DELTA"),
            ),
            (
                "      BLOCK DATA STORE\n      END BLOCK DATA\n",
                "block_data",
                Some("STORE"),
            ),
            ("      BLOCK DATA\n      END\n", "block_data", None),
        ] {
            let lines = fixed_form::physical_lines(source.as_bytes());
            let statements = fixed_form::logical_statements(&lines);
            let declaration =
                fixed_form::start_declaration(&statements[0].normalized_statement_text).unwrap();
            assert_eq!(declaration.kind, kind);
            assert_eq!(declaration.name.as_deref(), name);
        }
    }

    #[test]
    fn lexical_layer_keeps_fixed_form_mechanics_separate() {
        let source = b"C comment\n12345 SUBROUTINE TEST\n     1 (A)\n      X = 'not ! comment'; 5H!TEXT\n      END\r\n";
        let lines = fixed_form::physical_lines(source);
        assert_eq!(lines[0].kind, fixed_form::LineKind::Comment);
        assert_eq!(lines[1].label.as_deref(), Some("12345"));
        assert!(lines[2].continuation);
        let statements = fixed_form::logical_statements(&lines);
        assert_eq!(statements.len(), 4);
        assert!(
            statements[0]
                .normalized_statement_text
                .starts_with("SUBROUTINE TEST")
        );
        assert!(statements.iter().any(|statement| {
            statement
                .raw_statement_text
                .windows(2)
                .any(|window| window == b"!T")
        }));
        assert_eq!(lines[4].line_ending, "crlf");
    }

    #[test]
    fn scanner_is_deterministic_and_detects_duplicates() {
        let temp = tempfile::tempdir().unwrap();
        write_manifest_fixture(temp.path(), false);
        let first = scan(
            &temp.path().join("evidence"),
            &temp.path().join("manifests"),
            &temp.path().join("out-a"),
            true,
        )
        .unwrap();
        let alternate_root = tempfile::tempdir().unwrap();
        write_manifest_fixture(alternate_root.path(), false);
        let second = scan(
            &alternate_root.path().join("evidence"),
            &alternate_root.path().join("manifests"),
            &alternate_root.path().join("out-b"),
            true,
        )
        .unwrap();
        assert_eq!(first.status, "success");
        assert_eq!(first.semantic_hash, second.semantic_hash);
        assert_eq!(first.summary.files_attempted, 735);
        assert_eq!(first.summary.top_level_program_unit_count, 736);
        assert_eq!(first.summary.entry_count, 1);
        for file in [
            "source-scan-index.json",
            "program-units.json",
            "entry-points.json",
            "duplicate-providers.json",
            "diagnostics.json",
            "manifest.json",
            "scan-summary.md",
        ] {
            assert_eq!(
                fs::read(temp.path().join("out-a").join(file)).unwrap(),
                fs::read(alternate_root.path().join("out-b").join(file)).unwrap()
            );
        }
        for file in ["physical-lines.json", "logical-statements.json"] {
            assert_eq!(
                fs::read(temp.path().join("evidence/scans/test-snapshot").join(file)).unwrap(),
                fs::read(
                    alternate_root
                        .path()
                        .join("evidence/scans/test-snapshot")
                        .join(file)
                )
                .unwrap()
            );
        }
        let duplicate_root = tempfile::tempdir().unwrap();
        write_manifest_fixture(duplicate_root.path(), true);
        let duplicate = scan(
            &duplicate_root.path().join("evidence"),
            &duplicate_root.path().join("manifests"),
            &duplicate_root.path().join("out"),
            true,
        )
        .unwrap();
        assert_eq!(duplicate.status, "failed");
        assert_eq!(duplicate.summary.duplicate_normalized_identifiers, 1);
    }

    #[test]
    fn scanner_rejects_manifest_or_raw_evidence_mismatches() {
        let temp = tempfile::tempdir().unwrap();
        write_manifest_fixture(temp.path(), false);
        let source = temp
            .path()
            .join("evidence/extracted/test-snapshot/slatec-source-archive/src/f010.f");
        fs::write(source, "      SUBROUTINE TAMPER\n      END\n").unwrap();
        assert!(
            scan(
                &temp.path().join("evidence"),
                &temp.path().join("manifests"),
                &temp.path().join("out"),
                true,
            )
            .is_err()
        );
    }

    fn write_manifest_fixture(root: &Path, duplicate: bool) {
        let manifest_dir = root.join("manifests");
        let raw_root = root.join("evidence/extracted/test-snapshot/slatec-source-archive/src");
        fs::create_dir_all(&manifest_dir).unwrap();
        fs::create_dir_all(&raw_root).unwrap();
        let mut sources = Vec::new();
        let mut providers = Vec::new();
        for index in 0..735 {
            let path = format!("src/f{index:03}.f");
            let bytes = if index == 0 {
                b"      SUBROUTINE FIRST(A)\n      ENTRY SECOND(B)\n      END\n      REAL FUNCTION RF(X)\n      END FUNCTION\n".to_vec()
            } else if duplicate && index == 1 {
                b"      SUBROUTINE FIRST\n      END\n".to_vec()
            } else {
                format!("      SUBROUTINE S{index:03}\n      END\n").into_bytes()
            };
            fs::write(raw_root.join(format!("f{index:03}.f")), &bytes).unwrap();
            let id = format!("source-{index:03}");
            let sha = hash::bytes(&bytes);
            sources.push(json!({ "id": id, "snapshot_id": "test-snapshot", "artifact_id": "slatec-source-archive", "archive_member_path": path, "member_sha256": sha, "selected_state": "selected" }));
            providers.push(json!({ "snapshot_id": "test-snapshot", "source_file_id": format!("source-{index:03}"), "member_sha256": hash::bytes(&bytes), "selection_state": "selected" }));
        }
        fs::write(
            manifest_dir.join("artifact-manifest.json"),
            canonical(&json!({ "snapshot_id": "test-snapshot", "sha256": "a".repeat(64) }))
                .unwrap(),
        )
        .unwrap();
        fs::write(
            manifest_dir.join("source-files.json"),
            canonical(&json!({ "records": sources })).unwrap(),
        )
        .unwrap();
        fs::write(
            manifest_dir.join("selected-providers.json"),
            canonical(&json!({ "records": providers })).unwrap(),
        )
        .unwrap();
    }
}
