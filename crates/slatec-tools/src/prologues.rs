use crate::error::{CorpusError, Result};
use crate::fixed_form::{self, LineKind, RawSpan};
use crate::hash;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: &str = "1.0.0";
const PROLOGUE_SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const COMMITTED_SIZE_LIMIT: u64 = 4_000_000;

const COLLECTION_FIELDS: &[&str] = &[
    "keywords",
    "arguments",
    "argument_definitions",
    "parameters",
    "routines_called",
    "required_routines",
    "references",
    "see_also",
    "error_messages",
    "revision_history",
];

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
    declaration_locator: Locator,
    end_statement_locator: Option<Locator>,
}

#[derive(Clone, Deserialize)]
struct Locator {
    member_path: String,
    member_sha256: String,
    span: SpanText,
}

#[derive(Clone, Deserialize)]
struct SpanText {
    lines: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PrologueScanResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
    pub summary: PrologueSummary,
}

#[derive(Clone, Debug, Serialize)]
pub struct PrologueSummary {
    pub program_units_processed: usize,
    pub recognized_prologues: usize,
    pub absent_prologues: usize,
    pub total_recognized_fields: usize,
    pub unknown_fields: usize,
    pub ambiguous_associations: usize,
    pub review_item_count: usize,
    pub files_with_multiple_candidate_prologues: usize,
    pub dialect_counts: BTreeMap<String, usize>,
    pub diagnostics_by_rule: BTreeMap<String, usize>,
}

#[derive(Clone)]
struct SourceContext {
    source: SourceFile,
    bytes: Vec<u8>,
    lines: Vec<RawLine>,
}

#[derive(Clone)]
struct RawLine {
    span: RawSpan,
    content: Vec<u8>,
    kind: LineKind,
}

#[derive(Clone)]
struct Candidate {
    start_index: usize,
    end_index: usize,
    method: String,
    confidence: String,
}

#[derive(Clone)]
struct PrologueRecord {
    id: String,
    program_unit_id: String,
    source_file_id: String,
    path: String,
    source_sha256: String,
    dialect: String,
    association_status: String,
    association_method: String,
    association_confidence: String,
    line_start: Option<u64>,
    line_end: Option<u64>,
    raw_hash: Option<String>,
    fields: Vec<FieldRecord>,
    diagnostics: Vec<PrologueDiagnostic>,
    alternate_candidate_count: usize,
    collection_states: BTreeMap<String, String>,
}

#[derive(Clone)]
struct FieldRecord {
    id: String,
    program_unit_id: String,
    canonical_name: String,
    original_heading: String,
    line_start: u64,
    line_end: u64,
    content_hash: String,
    collection_state: String,
    diagnostic_ids: Vec<String>,
}

#[derive(Clone)]
struct PrologueDiagnostic {
    id: String,
    rule_id: String,
    severity: String,
    message: String,
    source_file_id: String,
    path: String,
    line: u64,
    byte_start: u64,
    byte_end: u64,
    program_unit_id: Option<String>,
}

pub fn scan(
    evidence_dir: &Path,
    corpus_manifest_dir: &Path,
    program_unit_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<PrologueScanResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "scan-prologues is evidence-only and requires --offline".to_owned(),
        ));
    }
    let artifact: ArtifactManifest =
        read_json(&corpus_manifest_dir.join("artifact-manifest.json"))?;
    let source_set: Records<SourceFile> =
        read_json(&corpus_manifest_dir.join("source-files.json"))?;
    let program_manifest: ProgramManifest = read_json(&program_unit_dir.join("manifest.json"))?;
    let program_units: Records<ProgramUnit> =
        read_json(&program_unit_dir.join("program-units.json"))?;
    if program_manifest.snapshot_id != artifact.snapshot_id {
        return Err(CorpusError::Verification(
            "program-unit manifest snapshot does not match corpus snapshot".to_owned(),
        ));
    }
    let snapshot = artifact.snapshot_id.clone();
    let source_manifest_hash = hash::file(&corpus_manifest_dir.join("source-files.json"))?;
    let program_manifest_hash = hash::file(&program_unit_dir.join("program-units.json"))?;
    let mut sources = BTreeMap::new();
    for source in source_set.records {
        if source.snapshot_id != snapshot {
            return Err(CorpusError::Verification(
                "source-file manifest snapshot mismatch".to_owned(),
            ));
        }
        if source.selected_state == "selected" {
            sources.insert(source.id.clone(), source);
        }
    }
    let mut source_contexts = BTreeMap::new();
    for source in sources.values() {
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
        source_contexts.insert(
            source.id.clone(),
            SourceContext {
                source: source.clone(),
                lines: raw_lines(&bytes),
                bytes,
            },
        );
    }
    let mut units = program_units.records;
    units.sort_by(|left, right| {
        (
            &left.declaration_locator.member_path,
            first_line(&left.declaration_locator.span.lines),
            &left.id,
        )
            .cmp(&(
                &right.declaration_locator.member_path,
                first_line(&right.declaration_locator.span.lines),
                &right.id,
            ))
    });
    let mut records = Vec::new();
    let mut diagnostics = Vec::new();
    let mut files_with_multiple_candidates = BTreeSet::new();
    for unit in &units {
        validate_unit_locator(unit, &snapshot)?;
        let context = source_contexts.get(&unit.source_file_id).ok_or_else(|| {
            CorpusError::Verification(format!(
                "program unit {} references missing selected source {}",
                unit.id, unit.source_file_id
            ))
        })?;
        if unit.declaration_locator.member_sha256 != context.source.member_sha256
            || unit.declaration_locator.member_path != context.source.archive_member_path
        {
            return Err(CorpusError::Verification(format!(
                "program unit locator mismatch for {}",
                unit.id
            )));
        }
        let record = extract_for_unit(unit, context);
        if record.alternate_candidate_count > 0 {
            files_with_multiple_candidates.insert(context.source.id.clone());
        }
        diagnostics.extend(record.diagnostics.clone());
        records.push(record);
    }
    let mut seen_record_ids = BTreeSet::new();
    let mut seen_field_ids = BTreeSet::new();
    for record in &records {
        if !seen_record_ids.insert(record.id.clone()) {
            return Err(CorpusError::Verification(format!(
                "duplicate generated prologue id {}",
                record.id
            )));
        }
        for field in &record.fields {
            if !seen_field_ids.insert(field.id.clone()) {
                return Err(CorpusError::Verification(format!(
                    "duplicate generated field id {}",
                    field.id
                )));
            }
        }
    }
    diagnostics.sort_by(|left, right| {
        (
            &left.severity,
            &left.path,
            left.line,
            &left.rule_id,
            &left.id,
        )
            .cmp(&(
                &right.severity,
                &right.path,
                right.line,
                &right.rule_id,
                &right.id,
            ))
    });
    let mut dialect_counts = BTreeMap::new();
    let mut diagnostics_by_rule = BTreeMap::new();
    for record in &records {
        *dialect_counts.entry(record.dialect.clone()).or_insert(0) += 1;
    }
    for diagnostic in &diagnostics {
        *diagnostics_by_rule
            .entry(diagnostic.rule_id.clone())
            .or_insert(0) += 1;
    }
    let total_recognized_fields = records
        .iter()
        .flat_map(|record| &record.fields)
        .filter(|field| field.canonical_name != "unrecognized")
        .count();
    let unknown_fields = records
        .iter()
        .flat_map(|record| &record.fields)
        .filter(|field| field.canonical_name == "unrecognized")
        .count();
    let ambiguous_associations = records
        .iter()
        .filter(|record| record.association_status == "ambiguous")
        .count();
    let summary = PrologueSummary {
        program_units_processed: records.len(),
        recognized_prologues: records
            .iter()
            .filter(|record| {
                !matches!(
                    record.dialect.as_str(),
                    "absent" | "unrecognized" | "ambiguous"
                )
            })
            .count(),
        absent_prologues: records
            .iter()
            .filter(|record| record.dialect == "absent")
            .count(),
        total_recognized_fields,
        unknown_fields,
        ambiguous_associations,
        review_item_count: diagnostics.len(),
        files_with_multiple_candidate_prologues: files_with_multiple_candidates.len(),
        dialect_counts,
        diagnostics_by_rule,
    };
    let status = if diagnostics.iter().any(|diag| diag.severity == "error") {
        "failed"
    } else if diagnostics.is_empty() {
        "success"
    } else {
        "success_with_review_items"
    }
    .to_owned();
    let detail_records: Vec<_> = records
        .iter()
        .map(|record| detail_value(record, &source_contexts))
        .collect::<Result<_>>()?;
    let detail_bytes = compact(&json!({
        "schema_id": "slatec-rs/prologue-raw-evidence",
        "schema_version": SCHEMA_VERSION,
        "snapshot_id": snapshot,
        "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
        "created_at": CREATED_AT,
        "prologue_parser_semantic_version": PROLOGUE_SEMANTIC_VERSION,
        "records": detail_records
    }))?;
    let mut evidence_files = BTreeMap::new();
    evidence_files.insert("raw-prologues.json", detail_bytes);
    let evidence_hashes: BTreeMap<_, _> = evidence_files
        .iter()
        .map(|(name, content)| ((*name).to_owned(), hash::bytes(content)))
        .collect();
    let evidence_output_dir = evidence_dir.join("prologues").join(&snapshot);
    promote(&evidence_output_dir, &snapshot, &evidence_files)?;
    let prologue_values: Vec<_> = records.iter().map(prologue_index_value).collect();
    let field_values: Vec<_> = records
        .iter()
        .flat_map(|record| record.fields.iter().map(field_index_value))
        .collect();
    let diagnostic_values: Vec<_> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic_value(&snapshot, diagnostic))
        .collect();
    let mut files = BTreeMap::new();
    files.insert(
        "prologue-index.json",
        compact(&json!({
            "schema_id": "slatec-rs/prologue-index-set",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "source_record_ids": ["artifact-slatec-source-archive"],
            "review_state": if diagnostics.is_empty() { "machine_checked" } else { "review_requested" },
            "prologue_parser_semantic_version": PROLOGUE_SEMANTIC_VERSION,
            "record_encoding": "columnar",
            "columns": [
                "id",
                "program_unit_id",
                "source_file_id",
                "source_member_path",
                "source_sha256",
                "dialect",
                "association_status",
                "association_method",
                "association_confidence",
                "prologue_line_start",
                "prologue_line_end",
                "field_canonical_names",
                "field_count",
                "unknown_field_count",
                "diagnostic_count",
                "diagnostic_ids",
                "raw_evidence_hash",
                "collection_states",
                "alternate_candidate_count"
            ],
            "records": prologue_values
        }))?,
    );
    files.insert(
        "field-index.json",
        compact(&json!({
            "schema_id": "slatec-rs/prologue-field-index-set",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "source_record_ids": ["artifact-slatec-source-archive"],
            "review_state": if diagnostics.is_empty() { "machine_checked" } else { "review_requested" },
            "statement_kind": "reported_external_claim",
            "extraction_status": "extracted_reported_external_claim",
            "record_encoding": "columnar",
            "columns": [
                "field_id",
                "program_unit_id",
                "canonical_name",
                "original_heading",
                "line_start",
                "line_end",
                "content_hash",
                "collection_state",
                "diagnostic_ids"
            ],
            "records": field_values
        }))?,
    );
    files.insert(
        "dialect-summary.json",
        compact(&json!({
            "schema_id": "slatec-rs/prologue-dialect-summary",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "prologue_parser_semantic_version": PROLOGUE_SEMANTIC_VERSION,
            "dialect_counts": summary.dialect_counts
        }))?,
    );
    files.insert(
        "diagnostics.json",
        compact(&json!({
            "schema_id": "slatec-rs/prologue-diagnostic-set",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "source_record_ids": ["artifact-slatec-source-archive"],
            "review_state": if diagnostics.is_empty() { "machine_checked" } else { "review_requested" },
            "stage_status": status,
            "records": diagnostic_values
        }))?,
    );
    let committed_size: u64 = files.values().map(|bytes| bytes.len() as u64).sum();
    if committed_size > COMMITTED_SIZE_LIMIT {
        return Err(CorpusError::Verification(format!(
            "committed prologue output would be {committed_size} bytes; redesign compact indexes before committing"
        )));
    }
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
    files.insert(
        "manifest.json",
        compact(&json!({
            "id": stable_id("prologue-scan", &[&snapshot]),
            "schema_id": "slatec-rs/prologue-scan",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "source_record_ids": ["artifact-slatec-source-archive"],
            "review_state": if diagnostics.is_empty() { "machine_checked" } else { "review_requested" },
            "prologue_parser_semantic_version": PROLOGUE_SEMANTIC_VERSION,
            "source_manifest_semantic_hash": source_manifest_hash,
            "program_unit_manifest_semantic_hash": program_manifest.output_semantic_hash,
            "program_unit_records_hash": program_manifest_hash,
            "input_artifact_hash": artifact.sha256,
            "configuration_semantic_hash": hash::bytes(format!("prologue-v{PROLOGUE_SEMANTIC_VERSION}\u{1f}{source_manifest_hash}\u{1f}{program_manifest_hash}").as_bytes()),
            "output_semantic_hash": semantic_hash,
            "output_file_hashes": file_hashes,
            "evidence_output_relative_dir": format!("prologues/{snapshot}"),
            "evidence_output_file_hashes": evidence_hashes,
            "committed_output_total_bytes": committed_size,
            "coverage_status": if diagnostics.is_empty() { "complete_verified" } else { "complete_unreviewed" },
            "stage_status": status,
            "diagnostic_count": diagnostics.len(),
            "summary": summary
        }))?,
    );
    files.insert(
        "extraction-summary.md",
        summary_markdown(&semantic_hash, &status, &summary, committed_size).into_bytes(),
    );
    promote(output_dir, &snapshot, &files)?;
    Ok(PrologueScanResult {
        snapshot_id: snapshot,
        status,
        semantic_hash,
        output_dir: output_dir.to_owned(),
        summary,
    })
}

fn validate_unit_locator(unit: &ProgramUnit, snapshot: &str) -> Result<()> {
    if unit.snapshot_id != snapshot {
        return Err(CorpusError::Verification(format!(
            "program unit {} snapshot mismatch",
            unit.id
        )));
    }
    if let Some(end) = &unit.end_statement_locator {
        if end.member_path != unit.declaration_locator.member_path
            || end.member_sha256 != unit.declaration_locator.member_sha256
        {
            return Err(CorpusError::Verification(format!(
                "program unit {} has mismatched declaration/end locators",
                unit.id
            )));
        }
    }
    Ok(())
}

fn extract_for_unit(unit: &ProgramUnit, context: &SourceContext) -> PrologueRecord {
    let declaration_line = first_line(&unit.declaration_locator.span.lines);
    let declaration_index = declaration_line.saturating_sub(1) as usize;
    let candidates = candidates_for_unit(context, declaration_index);
    let mut diagnostics = Vec::new();
    let primary = select_primary_candidate(&candidates);
    let alternate_candidate_count = candidates.len().saturating_sub(1);
    let (dialect, fields, line_start, line_end, raw_hash) = if let Some(candidate) = primary {
        let raw_hash = candidate_hash(context, candidate);
        let dialect = classify_dialect(context, candidate);
        let mut fields = extract_fields(context, unit, candidate, &dialect, &mut diagnostics);
        if dialect == "unrecognized"
            && !context.lines[candidate.start_index..=candidate.end_index]
                .iter()
                .all(|line| line.kind == LineKind::Blank)
        {
            diagnostics.push(custom_diag(
                context,
                unit,
                "PR-UNRECOGNIZED-DIALECT",
                "warning",
                "comment block associated but dialect was not recognized",
                &context.lines[candidate.start_index].span,
            ));
        }
        (
            dialect,
            {
                fields.sort_by(|left, right| {
                    (left.line_start, &left.canonical_name, &left.id).cmp(&(
                        right.line_start,
                        &right.canonical_name,
                        &right.id,
                    ))
                });
                fields
            },
            Some(context.lines[candidate.start_index].span.line),
            Some(context.lines[candidate.end_index].span.line),
            Some(raw_hash),
        )
    } else {
        diagnostics.push(custom_diag(
            context,
            unit,
            "PR-NO-PROLOGUE",
            "warning",
            "no associated comment prologue found",
            &context.lines[declaration_index.min(context.lines.len().saturating_sub(1))].span,
        ));
        ("absent".to_owned(), Vec::new(), None, None, None)
    };
    if candidates.len() > 1 {
        diagnostics.push(custom_diag(
            context,
            unit,
            "PR-AMBIGUOUS-ASSOCIATION",
            "warning",
            "multiple nearby candidate prologue blocks were found",
            &context.lines[candidates[0].start_index].span,
        ));
    }
    let association_status = if primary.is_none() {
        "absent"
    } else if candidates.len() > 1 {
        "ambiguous"
    } else {
        "associated"
    }
    .to_owned();
    let (association_method, association_confidence) = primary
        .map(|candidate| (candidate.method.clone(), candidate.confidence.clone()))
        .unwrap_or_else(|| ("none".to_owned(), "none".to_owned()));
    let mut collection_states = BTreeMap::new();
    for collection in COLLECTION_FIELDS {
        let state = fields
            .iter()
            .find(|field| field.canonical_name == *collection)
            .map(|field| field.collection_state.clone())
            .unwrap_or_else(|| "unknown".to_owned());
        collection_states.insert((*collection).to_owned(), state);
    }
    let id = stable_id(
        "prologue",
        &[
            &unit.id,
            line_start
                .map(|line| line.to_string())
                .unwrap_or_else(|| "absent".to_owned())
                .as_str(),
            raw_hash.as_deref().unwrap_or("absent"),
        ],
    );
    PrologueRecord {
        id,
        program_unit_id: unit.id.clone(),
        source_file_id: unit.source_file_id.clone(),
        path: context.source.archive_member_path.clone(),
        source_sha256: context.source.member_sha256.clone(),
        dialect,
        association_status,
        association_method,
        association_confidence,
        line_start,
        line_end,
        raw_hash,
        fields,
        diagnostics,
        alternate_candidate_count,
        collection_states,
    }
}

fn raw_lines(bytes: &[u8]) -> Vec<RawLine> {
    let physical = fixed_form::physical_lines(bytes);
    physical
        .into_iter()
        .map(|line| RawLine {
            content: bytes[line.span.start as usize..line.span.end as usize]
                .iter()
                .copied()
                .take_while(|byte| *byte != b'\r' && *byte != b'\n')
                .collect(),
            span: line.span,
            kind: line.kind,
        })
        .collect()
}

fn candidates_for_unit(context: &SourceContext, declaration_index: usize) -> Vec<Candidate> {
    let mut candidates = Vec::new();
    if declaration_index + 1 < context.lines.len() {
        if let Some(candidate) = forward_candidate(context, declaration_index + 1) {
            candidates.push(candidate);
        }
    }
    if declaration_index > 0 {
        if let Some(candidate) = backward_candidate(context, declaration_index - 1) {
            candidates.push(candidate);
        }
    }
    candidates.sort_by(|left, right| {
        let left_score = candidate_score(context, left);
        let right_score = candidate_score(context, right);
        right_score
            .cmp(&left_score)
            .then(left.start_index.cmp(&right.start_index))
    });
    candidates
}

fn forward_candidate(context: &SourceContext, mut index: usize) -> Option<Candidate> {
    while index < context.lines.len() && context.lines[index].kind == LineKind::Blank {
        index += 1;
    }
    if index >= context.lines.len() || context.lines[index].kind != LineKind::Comment {
        return None;
    }
    let start = index;
    let mut end = index;
    let mut saw_end_marker = false;
    while index < context.lines.len() {
        match context.lines[index].kind {
            LineKind::Comment | LineKind::Blank => {
                end = index;
                let text = comment_text(&context.lines[index]);
                if normalize_heading_text(&text).contains("END PROLOGUE") {
                    saw_end_marker = true;
                    break;
                }
                index += 1;
            }
            LineKind::Statement => break,
        }
    }
    Some(Candidate {
        start_index: start,
        end_index: end,
        method: if saw_end_marker {
            "post_declaration_sentinel"
        } else {
            "post_declaration_comment_block"
        }
        .to_owned(),
        confidence: if saw_end_marker { "high" } else { "medium" }.to_owned(),
    })
}

fn backward_candidate(context: &SourceContext, mut index: usize) -> Option<Candidate> {
    while index > 0 && context.lines[index].kind == LineKind::Blank {
        index -= 1;
    }
    if context.lines[index].kind != LineKind::Comment {
        return None;
    }
    let end = index;
    while index > 0
        && matches!(
            context.lines[index - 1].kind,
            LineKind::Comment | LineKind::Blank
        )
    {
        index -= 1;
    }
    Some(Candidate {
        start_index: index,
        end_index: end,
        method: "pre_declaration_comment_block".to_owned(),
        confidence: "medium".to_owned(),
    })
}

fn select_primary_candidate(candidates: &[Candidate]) -> Option<&Candidate> {
    candidates.first()
}

fn candidate_score(context: &SourceContext, candidate: &Candidate) -> u8 {
    let dialect = classify_dialect(context, candidate);
    match dialect.as_str() {
        "slatec-final" => 5,
        "quadpack" | "slatec-legacy" | "package-legacy" => 4,
        "unrecognized" => 1,
        _ => 0,
    }
}

fn classify_dialect(context: &SourceContext, candidate: &Candidate) -> String {
    let mut joined = String::new();
    for line in &context.lines[candidate.start_index..=candidate.end_index] {
        joined.push_str(&normalize_heading_text(&comment_text(line)));
        joined.push('\n');
    }
    let final_begin = joined.contains("BEGIN PROLOGUE");
    let final_end = joined.contains("END PROLOGUE");
    let legacy = joined.contains("DATE WRITTEN")
        || joined.contains("REVISION DATE")
        || joined.contains("CATEGORY NO");
    let quadpack = joined.contains("QUADPACK") || joined.contains("INTEGRATION");
    let package = joined.contains("BLAS")
        || joined.contains("LINPACK")
        || joined.contains("EISPACK")
        || joined.contains("FFTPACK")
        || joined.contains("FISHPACK")
        || joined.contains("PCHIP");
    let mut hits = Vec::new();
    if final_begin || final_end {
        hits.push("slatec-final");
    }
    if legacy && quadpack {
        hits.push("quadpack");
    } else if legacy {
        hits.push("slatec-legacy");
    }
    if package {
        hits.push("package-legacy");
    }
    hits.sort_unstable();
    hits.dedup();
    if hits.len() > 1 && !hits.contains(&"slatec-final") {
        "ambiguous".to_owned()
    } else {
        hits.first().copied().unwrap_or("unrecognized").to_owned()
    }
}

fn extract_fields(
    context: &SourceContext,
    unit: &ProgramUnit,
    candidate: &Candidate,
    _dialect: &str,
    diagnostics: &mut Vec<PrologueDiagnostic>,
) -> Vec<FieldRecord> {
    let mut fields = Vec::new();
    let mut current: Option<OpenField> = None;
    let mut seen = BTreeSet::new();
    for line in &context.lines[candidate.start_index..=candidate.end_index] {
        if line.kind == LineKind::Blank {
            if let Some(open) = current.as_mut() {
                open.lines.push(line.clone());
            }
            continue;
        }
        let comment = comment_text(line);
        if let Some(heading) = detect_heading(&comment) {
            if let Some(open) = current.take() {
                fields.push(close_field(context, unit, open));
            }
            if heading.canonical == "sentinel" {
                continue;
            }
            let mut diagnostic_ids = Vec::new();
            if heading.canonical == "unrecognized" {
                let diagnostic = custom_diag(
                    context,
                    unit,
                    "PR-UNRECOGNIZED-FIELD",
                    "warning",
                    "unrecognized prologue field heading preserved",
                    &line.span,
                );
                diagnostic_ids.push(diagnostic.id.clone());
                diagnostics.push(diagnostic);
            } else if !seen.insert(heading.canonical.clone()) {
                let diagnostic = custom_diag(
                    context,
                    unit,
                    "PR-DUPLICATE-FIELD",
                    "warning",
                    "duplicate prologue field heading preserved",
                    &line.span,
                );
                diagnostic_ids.push(diagnostic.id.clone());
                diagnostics.push(diagnostic);
            }
            current = Some(OpenField {
                canonical_name: heading.canonical,
                original_heading: heading.original,
                lines: vec![line.clone()],
                diagnostic_ids,
            });
        } else if let Some(open) = current.as_mut() {
            open.lines.push(line.clone());
        } else if !comment.trim().is_empty()
            && looks_like_heading(&normalize_heading_text(&comment))
            && !normalize_heading_text(&comment).contains("BEGIN PROLOGUE")
            && !normalize_heading_text(&comment).contains("END PROLOGUE")
        {
            let diagnostic = custom_diag(
                context,
                unit,
                "PR-MALFORMED-HEADING",
                "warning",
                "possible malformed prologue heading preserved as unrecognized material",
                &line.span,
            );
            diagnostics.push(diagnostic.clone());
            current = Some(OpenField {
                canonical_name: "unrecognized".to_owned(),
                original_heading: comment.trim().to_owned(),
                lines: vec![line.clone()],
                diagnostic_ids: vec![diagnostic.id],
            });
        } else if fields.is_empty() {
            current = Some(OpenField {
                canonical_name: "unrecognized".to_owned(),
                original_heading: "(preamble)".to_owned(),
                lines: vec![line.clone()],
                diagnostic_ids: Vec::new(),
            });
        }
    }
    if let Some(open) = current {
        fields.push(close_field(context, unit, open));
    }
    fields
}

struct OpenField {
    canonical_name: String,
    original_heading: String,
    lines: Vec<RawLine>,
    diagnostic_ids: Vec<String>,
}

struct Heading {
    canonical: String,
    original: String,
}

fn detect_heading(comment: &str) -> Option<Heading> {
    let mut text = comment.trim();
    if text.starts_with("***") {
        text = text.trim_start_matches('*').trim_start();
    }
    let normalized = normalize_heading_text(text);
    if normalized.contains("BEGIN PROLOGUE")
        || normalized.contains("END PROLOGUE")
        || normalized.contains("FIRST EXECUTABLE STATEMENT")
    {
        return Some(Heading {
            canonical: "sentinel".to_owned(),
            original: text.to_owned(),
        });
    }
    if let Some(canonical) = alias_for(&normalized) {
        return Some(Heading {
            canonical: canonical.to_owned(),
            original: heading_prefix(text),
        });
    }
    if looks_like_heading(&normalized) && has_heading_marker(text) {
        return Some(Heading {
            canonical: "unrecognized".to_owned(),
            original: heading_prefix(text),
        });
    }
    None
}

fn alias_for(normalized: &str) -> Option<&'static str> {
    for (canonical, aliases) in [
        ("purpose", &["PURPOSE"][..]),
        ("library", &["LIBRARY"]),
        ("category", &["CATEGORY", "CATEGORY NO", "CATEGORY NO."]),
        ("type", &["TYPE"]),
        ("keywords", &["KEYWORDS", "KEY WORDS"]),
        ("author", &["AUTHOR", "AUTHORS"]),
        ("description", &["DESCRIPTION", "METHOD"]),
        ("usage", &["USAGE", "CALLING SEQUENCE"]),
        ("arguments", &["ARGUMENTS", "ARGUMENT"]),
        (
            "argument_definitions",
            &[
                "ARGUMENT DEFINITIONS",
                "ARGUMENT DEFINITION",
                "PARAMETER DESCRIPTION",
            ],
        ),
        ("parameters", &["PARAMETERS", "PARAMETER"]),
        (
            "routines_called",
            &["ROUTINES CALLED", "SUBROUTINES CALLED"],
        ),
        (
            "revision_history",
            &["REVISION HISTORY", "REVISION DATE", "DATE WRITTEN"],
        ),
        ("references", &["REFERENCES", "REFERENCE"]),
        ("see_also", &["SEE ALSO", "SEE-ALSO"]),
        ("error_messages", &["ERROR MESSAGES", "ERROR MESSAGE"]),
        ("precision", &["PRECISION"]),
        (
            "required_routines",
            &["REQUIRED ROUTINES", "AUXILIARY ROUTINES"],
        ),
        ("subsidiary", &["SUBSIDIARY"]),
    ] {
        if aliases
            .iter()
            .any(|alias| heading_matches(normalized, alias))
        {
            return Some(canonical);
        }
    }
    None
}

fn heading_matches(normalized: &str, alias: &str) -> bool {
    normalized == alias
        || normalized.strip_prefix(alias).is_some_and(|rest| {
            rest.starts_with(' ') || rest.starts_with(':') || rest.starts_with('-')
        })
}

fn heading_prefix(text: &str) -> String {
    let text = text.trim();
    for delimiter in ["  ", ":", "-", "."] {
        if let Some((head, _)) = text.split_once(delimiter) {
            let head = head.trim();
            if !head.is_empty() {
                return head.to_owned();
            }
        }
    }
    text.split_whitespace()
        .take(4)
        .collect::<Vec<_>>()
        .join(" ")
}

fn looks_like_heading(normalized: &str) -> bool {
    if normalized.is_empty() || normalized.len() > 80 {
        return false;
    }
    let words = normalized.split_whitespace().count();
    words <= 4
        && normalized.chars().all(|ch| {
            ch.is_ascii_uppercase()
                || ch.is_ascii_digit()
                || matches!(ch, ' ' | '.' | ':' | '-' | '/' | '*')
        })
}

fn has_heading_marker(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.starts_with("***")
        || trimmed.ends_with(':')
        || trimmed.chars().any(|ch| ch.is_ascii_uppercase())
            && !trimmed.chars().any(|ch| ch.is_ascii_lowercase())
}

fn close_field(_context: &SourceContext, unit: &ProgramUnit, open: OpenField) -> FieldRecord {
    let first = open
        .lines
        .first()
        .expect("open field has at least one line");
    let last = open.lines.last().expect("open field has at least one line");
    let raw = raw_join(&open.lines);
    let content_hash = hash::bytes(&raw);
    let collection_state = if COLLECTION_FIELDS.contains(&open.canonical_name.as_str()) {
        if explicit_none(&raw) {
            "known_empty"
        } else if open.canonical_name == "unrecognized" {
            "partial"
        } else {
            "known_values"
        }
    } else if open.canonical_name == "unrecognized" {
        "partial"
    } else {
        "known_values"
    }
    .to_owned();
    FieldRecord {
        id: stable_id(
            "pf",
            &[
                &unit.id,
                &open.canonical_name,
                &first.span.start.to_string(),
                &content_hash,
            ],
        ),
        program_unit_id: unit.id.clone(),
        canonical_name: open.canonical_name,
        original_heading: open.original_heading,
        line_start: first.span.line,
        line_end: last.span.line,
        content_hash,
        collection_state,
        diagnostic_ids: open.diagnostic_ids,
    }
}

fn explicit_none(bytes: &[u8]) -> bool {
    let normalized = normalize_heading_text(&String::from_utf8_lossy(bytes));
    normalized.contains(" NONE") || normalized.ends_with("NONE") || normalized.contains("(NONE)")
}

fn comment_text(line: &RawLine) -> String {
    if line.kind != LineKind::Comment || line.content.is_empty() {
        return String::new();
    }
    String::from_utf8_lossy(&line.content[1..]).to_string()
}

fn normalize_heading_text(text: &str) -> String {
    text.replace(['\t', ','], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim_matches('*')
        .trim()
        .to_ascii_uppercase()
}

fn candidate_hash(context: &SourceContext, candidate: &Candidate) -> String {
    let start = context.lines[candidate.start_index].span.start as usize;
    let end = context.lines[candidate.end_index].span.end as usize;
    hash::bytes(&context.bytes[start..end])
}

fn raw_join(lines: &[RawLine]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for line in lines {
        bytes.extend_from_slice(&line.content);
        bytes.push(b'\n');
    }
    bytes
}

fn first_line(lines: &str) -> u64 {
    lines
        .split_once('-')
        .map(|(first, _)| first)
        .unwrap_or(lines)
        .parse()
        .unwrap_or(1)
}

fn detail_value(
    record: &PrologueRecord,
    contexts: &BTreeMap<String, SourceContext>,
) -> Result<Value> {
    let Some(context) = contexts.get(&record.source_file_id) else {
        return Err(CorpusError::Verification(
            "raw evidence detail context missing".to_owned(),
        ));
    };
    let raw_text = if let (Some(line_start), Some(line_end)) = (record.line_start, record.line_end)
    {
        let start = line_start.saturating_sub(1) as usize;
        let end = line_end.saturating_sub(1) as usize;
        let mut text = String::new();
        for line in &context.lines[start..=end] {
            text.push_str(&String::from_utf8_lossy(&line.content));
            text.push('\n');
        }
        Some(text)
    } else {
        None
    };
    Ok(json!({
        "id": record.id,
        "program_unit_id": record.program_unit_id,
        "source_file_id": record.source_file_id,
        "member_path": record.path,
        "dialect": record.dialect,
        "line_start": record.line_start,
        "line_end": record.line_end,
        "raw_evidence_hash": record.raw_hash,
        "raw_text": raw_text,
        "fields": record.fields.iter().map(|field| json!({
            "id": field.id,
            "canonical_name": field.canonical_name,
            "original_heading": field.original_heading,
            "line_start": field.line_start,
            "line_end": field.line_end,
            "content_hash": field.content_hash
        })).collect::<Vec<_>>()
    }))
}

fn prologue_index_value(record: &PrologueRecord) -> Value {
    json!([
        record.id,
        record.program_unit_id,
        record.source_file_id,
        record.path,
        record.source_sha256,
        record.dialect,
        record.association_status,
        record.association_method,
        record.association_confidence,
        record.line_start,
        record.line_end,
        record
            .fields
            .iter()
            .map(|field| field.canonical_name.clone())
            .collect::<Vec<_>>(),
        record.fields.len(),
        record
            .fields
            .iter()
            .filter(|field| field.canonical_name == "unrecognized")
            .count(),
        record.diagnostics.len(),
        record
            .diagnostics
            .iter()
            .map(|diag| diag.id.clone())
            .collect::<Vec<_>>(),
        record.raw_hash,
        record.collection_states,
        record.alternate_candidate_count
    ])
}

fn field_index_value(field: &FieldRecord) -> Value {
    json!([
        field.id,
        field.program_unit_id,
        field.canonical_name,
        field.original_heading,
        field.line_start,
        field.line_end,
        field.content_hash,
        field.collection_state,
        field.diagnostic_ids
    ])
}

fn diagnostic_value(snapshot: &str, diag: &PrologueDiagnostic) -> Value {
    json!({
        "id": diag.id,
        "schema_id": "slatec-rs/diagnostic",
        "schema_version": SCHEMA_VERSION,
        "snapshot_id": snapshot,
        "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
        "created_at": CREATED_AT,
        "source_record_ids": [diag.source_file_id],
        "review_state": "review_requested",
        "rule_id": diag.rule_id,
        "severity": diag.severity,
        "stage": "prologue_scan",
        "message_template_id": diag.rule_id,
        "arguments": [diag.message],
        "program_unit_id": diag.program_unit_id,
        "locator": {
            "member_path": diag.path,
            "representation": "raw_bytes",
            "span": {
                "bytes": format!("{}-{}", diag.byte_start, diag.byte_end),
                "lines": format!("{}-{}", diag.line, diag.line)
            }
        },
        "review_impact": "review"
    })
}

fn custom_diag(
    context: &SourceContext,
    unit: &ProgramUnit,
    rule: &str,
    severity: &str,
    message: &str,
    span: &RawSpan,
) -> PrologueDiagnostic {
    PrologueDiagnostic {
        id: stable_id(
            "prologue-diagnostic",
            &[&unit.id, rule, &span.start.to_string()],
        ),
        rule_id: rule.to_owned(),
        severity: severity.to_owned(),
        message: message.to_owned(),
        source_file_id: context.source.id.clone(),
        path: context.source.archive_member_path.clone(),
        line: span.line,
        byte_start: span.start,
        byte_end: span.end,
        program_unit_id: Some(unit.id.clone()),
    }
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
            "prologue staging directory exists: {}",
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
                    "existing prologue output differs: {}",
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
    semantic_hash: &str,
    status: &str,
    summary: &PrologueSummary,
    committed_size: u64,
) -> String {
    format!(
        "# SLATEC prologue extraction\n\n- Semantic hash: `{semantic_hash}`\n- Stage status: `{status}`\n- Program units processed: {}\n- Recognized prologues: {}\n- Absent prologues: {}\n- Recognized fields: {}\n- Unknown fields: {}\n- Ambiguous associations: {}\n- Review items: {}\n- Committed generated bytes: {committed_size}\n\nRaw prologue text is stored only in ignored local evidence. Extracted fields are documentary `reported_external_claim` evidence and do not verify executable declarations, dependencies, ABI properties, or safety.\n",
        summary.program_units_processed,
        summary.recognized_prologues,
        summary.absent_prologues,
        summary.total_recognized_fields,
        summary.unknown_fields,
        summary.ambiguous_associations,
        summary.review_item_count
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_final_slatec_fields_and_explicit_empty_collection() {
        let temp = tempfile::tempdir().unwrap();
        write_fixture(
            temp.path(),
            "C***BEGIN PROLOGUE  ALPHA\nC***PURPOSE  Demo purpose.\nC***ROUTINES CALLED  (NONE)\nC***UNKNOWN THING\nC   preserved\nC***END PROLOGUE  ALPHA\n      SUBROUTINE ALPHA\n      END\n",
        );
        let result = scan(
            &temp.path().join("evidence"),
            &temp.path().join("corpus"),
            &temp.path().join("program-units"),
            &temp.path().join("out"),
            true,
        )
        .unwrap();
        assert_eq!(result.summary.program_units_processed, 1);
        assert_eq!(result.summary.recognized_prologues, 1);
        assert_eq!(result.summary.unknown_fields, 1);
        let index = fs::read_to_string(temp.path().join("out/prologue-index.json")).unwrap();
        assert!(index.contains("\"slatec-final\""));
        assert!(index.contains("\"routines_called\":\"known_empty\""));
        assert!(!index.contains("Demo purpose"));
    }

    #[test]
    fn distinguishes_absent_from_empty_and_preserves_raw_evidence_ignored_shape() {
        let temp = tempfile::tempdir().unwrap();
        write_fixture(temp.path(), "      SUBROUTINE BETA\n      END\n");
        let result = scan(
            &temp.path().join("evidence"),
            &temp.path().join("corpus"),
            &temp.path().join("program-units"),
            &temp.path().join("out"),
            true,
        )
        .unwrap();
        assert_eq!(result.summary.absent_prologues, 1);
        assert_eq!(result.summary.review_item_count, 1);
        let index = fs::read_to_string(temp.path().join("out/prologue-index.json")).unwrap();
        assert!(index.contains("\"routines_called\":\"unknown\""));
        assert!(
            temp.path()
                .join("evidence/prologues/test-snapshot/raw-prologues.json")
                .exists()
        );
    }

    #[test]
    fn recognizes_legacy_and_ambiguous_candidate_blocks() {
        let temp = tempfile::tempdir().unwrap();
        write_fixture(
            temp.path(),
            "C DATE WRITTEN  1980\nC REVISION DATE  1981\nC PURPOSE  before\n      SUBROUTINE GAMMA\nC***BEGIN PROLOGUE  GAMMA\nC***PURPOSE  after\nC***END PROLOGUE  GAMMA\n      END\n",
        );
        let result = scan(
            &temp.path().join("evidence"),
            &temp.path().join("corpus"),
            &temp.path().join("program-units"),
            &temp.path().join("out"),
            true,
        )
        .unwrap();
        assert_eq!(result.summary.ambiguous_associations, 1);
        assert_eq!(
            result.summary.dialect_counts.get("slatec-final").copied(),
            Some(1)
        );
    }

    #[test]
    fn prologue_outputs_are_deterministic_across_roots_and_compact() {
        let a = tempfile::tempdir().unwrap();
        let b = tempfile::tempdir().unwrap();
        let source = "      SUBROUTINE DELTA\nC***BEGIN PROLOGUE  DELTA\nC***LIBRARY  SLATEC\nC***REFERENCES  NONE\nC***END PROLOGUE  DELTA\n      END\n";
        write_fixture(a.path(), source);
        write_fixture(b.path(), source);
        let first = scan(
            &a.path().join("evidence"),
            &a.path().join("corpus"),
            &a.path().join("program-units"),
            &a.path().join("out"),
            true,
        )
        .unwrap();
        let second = scan(
            &b.path().join("evidence"),
            &b.path().join("corpus"),
            &b.path().join("program-units"),
            &b.path().join("out"),
            true,
        )
        .unwrap();
        assert_eq!(first.semantic_hash, second.semantic_hash);
        for file in [
            "manifest.json",
            "prologue-index.json",
            "field-index.json",
            "dialect-summary.json",
            "diagnostics.json",
            "extraction-summary.md",
        ] {
            let left = fs::read(a.path().join("out").join(file)).unwrap();
            let right = fs::read(b.path().join("out").join(file)).unwrap();
            assert_eq!(left, right, "{file}");
            assert!(left.len() < 50_000, "{file} unexpectedly large");
        }
    }

    #[test]
    fn rejects_raw_source_hash_mismatch() {
        let temp = tempfile::tempdir().unwrap();
        write_fixture(temp.path(), "      SUBROUTINE EPS\n      END\n");
        fs::write(
            temp.path()
                .join("evidence/extracted/test-snapshot/slatec-source-archive/src/test.f"),
            "      SUBROUTINE TAMPER\n      END\n",
        )
        .unwrap();
        assert!(
            scan(
                &temp.path().join("evidence"),
                &temp.path().join("corpus"),
                &temp.path().join("program-units"),
                &temp.path().join("out"),
                true,
            )
            .is_err()
        );
    }

    fn write_fixture(root: &Path, source: &str) {
        let corpus = root.join("corpus");
        let units = root.join("program-units");
        let raw_root = root.join("evidence/extracted/test-snapshot/slatec-source-archive/src");
        fs::create_dir_all(&corpus).unwrap();
        fs::create_dir_all(&units).unwrap();
        fs::create_dir_all(&raw_root).unwrap();
        fs::write(raw_root.join("test.f"), source).unwrap();
        let sha = hash::bytes(source.as_bytes());
        let lines = fixed_form::physical_lines(source.as_bytes());
        let statements = fixed_form::logical_statements(&lines);
        let declaration = statements
            .iter()
            .find(|statement| statement.classification == "program_unit_start")
            .unwrap();
        let end = statements
            .iter()
            .find(|statement| statement.classification == "program_unit_end")
            .unwrap();
        let source_id = "source-test";
        fs::write(
            corpus.join("artifact-manifest.json"),
            compact(&json!({"snapshot_id":"test-snapshot","sha256":"a".repeat(64)})).unwrap(),
        )
        .unwrap();
        fs::write(
            corpus.join("source-files.json"),
            compact(&json!({"records":[{
                "id": source_id,
                "snapshot_id": "test-snapshot",
                "artifact_id": "slatec-source-archive",
                "archive_member_path": "src/test.f",
                "member_sha256": sha,
                "selected_state": "selected"
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
                "id": "program-unit-test",
                "snapshot_id": "test-snapshot",
                "source_file_id": source_id,
                "source_artifact_id": "slatec-source-archive",
                "name": "TEST",
                "normalized_name": "TEST",
                "declaration_locator": {
                    "member_path": "src/test.f",
                    "member_sha256": sha,
                    "span": {"lines": format!("{}-{}", declaration.physical_line_start, declaration.physical_line_end)}
                },
                "end_statement_locator": {
                    "member_path": "src/test.f",
                    "member_sha256": sha,
                    "span": {"lines": format!("{}-{}", end.physical_line_start, end.physical_line_end)}
                }
            }]}))
            .unwrap(),
        )
        .unwrap();
    }
}
