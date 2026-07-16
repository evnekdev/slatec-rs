//! Conservative executable-interface inventory for selected SLATEC providers.
//!
//! This is intentionally not a general Fortran parser.  It reads only the
//! fixed-form declaration header and the small set of specification statements
//! needed to describe a potential raw FFI boundary.  Everything else remains
//! source evidence or a review item.

use crate::error::{CorpusError, Result};
use crate::fixed_form::{self, LogicalStatement, RawSpan, StartDeclaration};
use crate::hash;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: &str = "1.0.0";
const SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const COMMITTED_SIZE_LIMIT: u64 = 4_000_000;

#[derive(Clone, Deserialize)]
struct SelectedProvider {
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
    records: Vec<SelectedProvider>,
}

#[derive(Clone, Debug, Serialize)]
pub struct InventoryResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
    pub summary: InventorySummary,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct InventorySummary {
    pub selected_program_units: usize,
    pub processed_program_units: usize,
    pub source_files: usize,
    pub subroutines: usize,
    pub functions: usize,
    pub block_data_units: usize,
    pub programs: usize,
    pub formal_arguments: usize,
    pub explicit_types: usize,
    pub implicit_types: usize,
    pub unknown_types: usize,
    pub array_arguments: usize,
    pub character_arguments: usize,
    pub external_formal_arguments: usize,
    pub function_results: usize,
    pub conflicting_interfaces: usize,
    pub review_items: usize,
}

#[derive(Clone)]
struct SourceGroup {
    subset: String,
    path: String,
    raw_sha256: String,
    normalized_sha256: String,
    providers: Vec<SelectedProvider>,
}

#[derive(Clone)]
struct UnitSpan {
    declaration_index: usize,
    end_index: Option<usize>,
    declaration: StartDeclaration,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Clone)]
struct ParsedUnit {
    provider: SelectedProvider,
    header_span: RawSpan,
    end_span: Option<RawSpan>,
    arguments: Vec<Argument>,
    function_result: Option<FunctionResult>,
    implicit_rules: Vec<ImplicitRule>,
    parameters: Vec<Parameter>,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Clone)]
struct Argument {
    id: String,
    position: usize,
    normalized_name: String,
    declared_type: Option<String>,
    type_source: String,
    character_length: Option<String>,
    dimensions: Vec<Dimension>,
    is_external: bool,
    is_intrinsic: bool,
    locators: Vec<Value>,
    implicit_rule_id: Option<String>,
    conflict_state: String,
}

#[derive(Clone)]
struct FunctionResult {
    id: String,
    declared_type: Option<String>,
    type_source: String,
    character_length: Option<String>,
    locator: Value,
    conflict_state: String,
}

#[derive(Clone, PartialEq)]
struct Dimension {
    lower_bound: Option<String>,
    upper_bound: Option<String>,
    assumed_size: bool,
    normalized_expression: String,
}

#[derive(Clone)]
struct ImplicitRule {
    id: String,
    state: String,
    ranges: Vec<String>,
    declared_type: Option<String>,
    locator: Value,
}

#[derive(Clone)]
struct Parameter {
    id: String,
    normalized_name: String,
    raw_expression: String,
    locator: Value,
}

#[derive(Clone)]
struct EntityState {
    declared_type: Option<String>,
    character_length: Option<String>,
    dimensions: Vec<Dimension>,
    external: bool,
    intrinsic: bool,
    locators: Vec<Value>,
    conflict: bool,
    diagnostic_ids: Vec<String>,
}

#[derive(Clone)]
struct Diagnostic {
    id: String,
    severity: String,
    rule_id: String,
    program_unit_id: String,
    line: u64,
}

/// Scan exactly the providers selected by `selected-providers.json`.
pub fn scan(
    evidence_dir: &Path,
    selected_corpus_dir: &Path,
    _program_unit_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<InventoryResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "scan-ffi-inventory is evidence-only and requires --offline".to_owned(),
        ));
    }
    let providers: ProviderRecords =
        read_json(&selected_corpus_dir.join("selected-providers.json"))?;
    let selection_manifest: Value = read_json(&selected_corpus_dir.join("manifest.json"))?;
    let snapshot = providers.snapshot_id.clone();
    if selection_manifest["snapshot_id"].as_str() != Some(snapshot.as_str()) {
        return Err(CorpusError::Verification(
            "selected-provider and selected-corpus manifest snapshot mismatch".to_owned(),
        ));
    }
    let main_src_snapshot = selection_manifest["main_src_snapshot_id"]
        .as_str()
        .ok_or_else(|| {
            CorpusError::Verification(
                "selected-corpus manifest lacks the immutable main-src snapshot ID".to_owned(),
            )
        })?
        .to_owned();
    let unresolved: Value = read_json(&selected_corpus_dir.join("unresolved-providers.json"))?;
    if unresolved["records"]
        .as_array()
        .is_none_or(|records| !records.is_empty())
    {
        return Err(CorpusError::Verification(
            "selected corpus has unresolved providers; FFI inventory is blocked".to_owned(),
        ));
    }
    if providers.records.len() != 1_476 {
        return Err(CorpusError::Verification(format!(
            "complete selected-provider count must be 1476, found {}",
            providers.records.len()
        )));
    }
    let selection_hash = hash::file(&selected_corpus_dir.join("selected-providers.json"))?;
    let mut provider_ids = BTreeSet::new();
    let mut groups = BTreeMap::<(String, String, String), SourceGroup>::new();
    for provider in providers.records {
        if provider.snapshot_id != snapshot
            || !provider_ids.insert(provider.program_unit_id.clone())
        {
            return Err(CorpusError::Verification(
                "duplicate or snapshot-mismatched selected provider".to_owned(),
            ));
        }
        let key = (
            provider.source_subset.clone(),
            provider.source_path.clone(),
            provider.raw_sha256.clone(),
        );
        let group = groups.entry(key).or_insert_with(|| SourceGroup {
            subset: provider.source_subset.clone(),
            path: provider.source_path.clone(),
            raw_sha256: provider.raw_sha256.clone(),
            normalized_sha256: provider.normalized_sha256.clone(),
            providers: Vec::new(),
        });
        if group.normalized_sha256 != provider.normalized_sha256 {
            return Err(CorpusError::Verification(
                "selected providers disagree on normalized source hash".to_owned(),
            ));
        }
        group.providers.push(provider);
    }

    let mut parsed = Vec::new();
    let mut source_rows = Vec::new();
    for group in groups.values_mut() {
        group
            .providers
            .sort_by(|left, right| left.program_unit_id.cmp(&right.program_unit_id));
        let path = source_path(evidence_dir, &main_src_snapshot, &group.subset, &group.path)?;
        let bytes = fs::read(&path).map_err(|_| {
            CorpusError::Verification(format!(
                "missing selected source evidence {}",
                path.display()
            ))
        })?;
        if hash::bytes(&bytes) != group.raw_sha256 {
            return Err(CorpusError::Verification(format!(
                "selected source hash mismatch for {}:{}",
                group.subset, group.path
            )));
        }
        let physical = fixed_form::physical_lines(&bytes);
        let statements = fixed_form::logical_statements(&physical);
        let spans = top_level_units(&statements, group)?;
        let mut matched = BTreeSet::new();
        for span in spans {
            let matches = matching_providers(&span, group);
            for provider in matches {
                if !matched.insert(provider.program_unit_id.clone()) {
                    return Err(CorpusError::Verification(format!(
                        "selected provider {} has multiple declaration matches",
                        provider.program_unit_id
                    )));
                }
                parsed.push(parse_unit(provider, &span, &statements, &snapshot));
            }
        }
        if matched.len() != group.providers.len() {
            let missing = group
                .providers
                .iter()
                .filter(|provider| !matched.contains(&provider.program_unit_id))
                .map(|provider| provider.normalized_name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(CorpusError::Verification(format!(
                "selected source {}:{} lacks matching top-level declaration(s): {missing}",
                group.subset, group.path
            )));
        }
        source_rows.push(json!([
            stable_id(
                "ffi-source",
                &[&snapshot, &group.subset, &group.path, &group.raw_sha256]
            ),
            group.subset,
            group.path,
            group.raw_sha256,
            group.normalized_sha256,
            group.providers.len(),
        ]));
    }
    parsed.sort_by(|left, right| {
        left.provider
            .program_unit_id
            .cmp(&right.provider.program_unit_id)
    });
    if parsed.len() != provider_ids.len() {
        return Err(CorpusError::Verification(
            "not every selected provider was processed".to_owned(),
        ));
    }
    source_rows.sort_by(|left, right| {
        left[1]
            .as_str()
            .cmp(&right[1].as_str())
            .then(left[2].as_str().cmp(&right[2].as_str()))
    });
    write_outputs(output_dir, &snapshot, &selection_hash, &parsed, source_rows)
}

fn source_path(
    evidence_dir: &Path,
    main_src_snapshot: &str,
    subset: &str,
    path: &str,
) -> Result<PathBuf> {
    match subset {
        "main-src" => Ok(evidence_dir
            .join("extracted")
            .join(main_src_snapshot)
            .join("slatec-source-archive")
            .join(path)),
        "lin" | "fishfft" | "fnlib" | "pchip" => Ok(evidence_dir
            .join("full-corpus/audit-input/directories")
            .join(subset)
            .join("files")
            .join(path)),
        "spfun" => Ok(evidence_dir
            .join("full-corpus/audit-input/supplemental")
            .join(subset)),
        _ => Err(CorpusError::Verification(format!(
            "no selected-evidence path rule for subset {subset}"
        ))),
    }
}

fn top_level_units(statements: &[LogicalStatement], group: &SourceGroup) -> Result<Vec<UnitSpan>> {
    let mut result = Vec::new();
    let mut open: Option<UnitSpan> = None;
    for (index, statement) in statements.iter().enumerate() {
        let text = statement.normalized_statement_text.trim();
        if let Some(declaration) = fixed_form::start_declaration(text) {
            if let Some(mut previous) = open.take() {
                previous.diagnostics.push(diag(
                    group,
                    "FFI-MISSING-END",
                    "warning",
                    &statements[previous.declaration_index],
                    "program-unit declaration was followed by another declaration before END",
                ));
                result.push(previous);
            }
            open = Some(UnitSpan {
                declaration_index: index,
                end_index: None,
                declaration,
                diagnostics: Vec::new(),
            });
        } else if fixed_form::is_end(text) {
            if let Some(mut unit) = open.take() {
                unit.end_index = Some(index);
                result.push(unit);
            } else {
                return Err(CorpusError::Verification(format!(
                    "unexpected END in selected source {}:{} at line {}",
                    group.subset, group.path, statement.physical_line_start
                )));
            }
        }
    }
    if let Some(mut unit) = open {
        unit.diagnostics.push(diag(
            group,
            "FFI-MISSING-END",
            "warning",
            &statements[unit.declaration_index],
            "program unit reaches end of file without END",
        ));
        result.push(unit);
    }
    Ok(result)
}

fn matching_providers(span: &UnitSpan, group: &SourceGroup) -> Vec<SelectedProvider> {
    group
        .providers
        .iter()
        .filter(|provider| {
            provider.kind == span.declaration.kind
                && span
                    .declaration
                    .name
                    .as_ref()
                    .is_some_and(|name| name.eq_ignore_ascii_case(&provider.normalized_name))
        })
        .cloned()
        .collect()
}

fn parse_unit(
    provider: SelectedProvider,
    span: &UnitSpan,
    statements: &[LogicalStatement],
    snapshot: &str,
) -> ParsedUnit {
    let header = &statements[span.declaration_index];
    let mut diagnostics = span.diagnostics.clone();
    let formal_names = formal_arguments(
        &header.normalized_statement_text,
        &span.declaration,
        &provider,
        header,
        &mut diagnostics,
    );
    let mut entities = BTreeMap::<String, EntityState>::new();
    for name in &formal_names {
        entities.insert(
            name.clone(),
            EntityState {
                declared_type: None,
                character_length: None,
                dimensions: Vec::new(),
                external: false,
                intrinsic: false,
                locators: Vec::new(),
                conflict: false,
                diagnostic_ids: Vec::new(),
            },
        );
    }
    let mut implicit = ImplicitMap::default();
    let mut implicit_rules = Vec::new();
    let mut parameters = Vec::new();
    let last = span.end_index.unwrap_or(statements.len());
    let mut in_declaration_region = true;
    for statement in &statements[span.declaration_index.saturating_add(1)..last] {
        let text = statement.normalized_statement_text.trim();
        if text.is_empty() || text.starts_with("ENTRY ") {
            continue;
        }
        let locator = locator(snapshot, &provider, statement);
        if let Some((spec, entities_text)) = type_statement(text) {
            if !in_declaration_region {
                diagnostics.push(diag_for_provider(
                    &provider,
                    "FFI-DECLARATION-AFTER-EXECUTABLE",
                    "warning",
                    statement,
                    "type declaration appears after the declaration region ended",
                ));
                continue;
            }
            for item in split_top_level(entities_text, ',') {
                match entity_with_dimensions(&item) {
                    Some((name, dimensions)) => apply_type(
                        &mut entities,
                        &name,
                        &spec,
                        dimensions,
                        locator.clone(),
                        &provider,
                        statement,
                        &mut diagnostics,
                    ),
                    None => diagnostics.push(diag_for_provider(
                        &provider,
                        "FFI-UNSUPPORTED-DECLARATION",
                        "warning",
                        statement,
                        "could not parse a declared entity",
                    )),
                }
            }
        } else if looks_like_unsupported_type_declaration(text) {
            diagnostics.push(diag_for_provider(
                &provider,
                "FFI-UNSUPPORTED-DECLARATION",
                "warning",
                statement,
                "unsupported type declaration prevents implicit FFI typing",
            ));
            implicit.invalidate();
        } else if let Some(items) = text.strip_prefix("DIMENSION ") {
            if !in_declaration_region {
                diagnostics.push(diag_for_provider(
                    &provider,
                    "FFI-DECLARATION-AFTER-EXECUTABLE",
                    "warning",
                    statement,
                    "DIMENSION appears after the declaration region ended",
                ));
                continue;
            }
            for item in split_top_level(items, ',') {
                match entity_with_dimensions(&item) {
                    Some((name, dimensions)) if !dimensions.is_empty() => apply_dimensions(
                        &mut entities,
                        &name,
                        dimensions,
                        locator.clone(),
                        &provider,
                        statement,
                        &mut diagnostics,
                    ),
                    _ => diagnostics.push(diag_for_provider(
                        &provider,
                        "FFI-MALFORMED-DIMENSION",
                        "warning",
                        statement,
                        "DIMENSION item has no usable dimensions",
                    )),
                }
            }
        } else if let Some(items) = text.strip_prefix("EXTERNAL ") {
            apply_attribute(
                &mut entities,
                items,
                true,
                false,
                locator,
                &provider,
                statement,
                &mut diagnostics,
            );
        } else if let Some(items) = text.strip_prefix("INTRINSIC ") {
            apply_attribute(
                &mut entities,
                items,
                false,
                true,
                locator,
                &provider,
                statement,
                &mut diagnostics,
            );
        } else if text == "IMPLICIT NONE" {
            if !in_declaration_region {
                diagnostics.push(diag_for_provider(
                    &provider,
                    "FFI-DECLARATION-AFTER-EXECUTABLE",
                    "warning",
                    statement,
                    "IMPLICIT NONE appears after the declaration region ended",
                ));
            } else {
                implicit.none(&provider, statement);
                implicit_rules.push(implicit.latest_rule().expect("rule recorded").clone());
            }
        } else if let Some(specification) = text.strip_prefix("IMPLICIT ") {
            if !in_declaration_region {
                diagnostics.push(diag_for_provider(
                    &provider,
                    "FFI-DECLARATION-AFTER-EXECUTABLE",
                    "warning",
                    statement,
                    "IMPLICIT appears after the declaration region ended",
                ));
            } else if let Err(message) = implicit.apply(specification, &provider, statement) {
                diagnostics.push(diag_for_provider(
                    &provider,
                    "FFI-MALFORMED-IMPLICIT",
                    "warning",
                    statement,
                    &message,
                ));
                implicit.invalidate();
            } else if let Some(rule) = implicit.latest_rule() {
                implicit_rules.push(rule.clone());
            }
        } else if let Some(items) = text.strip_prefix("PARAMETER ") {
            if !in_declaration_region {
                diagnostics.push(diag_for_provider(
                    &provider,
                    "FFI-DECLARATION-AFTER-EXECUTABLE",
                    "warning",
                    statement,
                    "PARAMETER appears after the declaration region ended",
                ));
            } else {
                parameters.extend(parse_parameters(
                    items,
                    snapshot,
                    &provider,
                    statement,
                    &mut diagnostics,
                ));
            }
        } else if is_specification_statement(text) {
            diagnostics.push(diag_for_provider(
                &provider,
                "FFI-UNSUPPORTED-SPECIFICATION",
                "warning",
                statement,
                "specification statement is retained for manual FFI review",
            ));
        } else {
            in_declaration_region = false;
        }
    }
    let mut arguments = Vec::new();
    for (position, name) in formal_names.iter().enumerate() {
        let entity = entities.get(name).expect("formal entity inserted");
        let (declared_type, type_source, implicit_rule_id) =
            if let Some(declared) = &entity.declared_type {
                (Some(declared.clone()), "explicit".to_owned(), None)
            } else if let Some((kind, rule_id)) = implicit.type_for(name) {
                (Some(kind), "implicit_rule".to_owned(), Some(rule_id))
            } else {
                diagnostics.push(diag_for_provider(
                    &provider,
                    "FFI-UNKNOWN-ARGUMENT-TYPE",
                    "warning",
                    header,
                    "formal argument has no resolvable type",
                ));
                (None, "unknown".to_owned(), None)
            };
        if entity.external && entity.intrinsic {
            let diagnostic = diag_for_provider(
                &provider,
                "FFI-EXTERNAL-INTRINSIC-CONFLICT",
                "warning",
                header,
                "formal argument is both EXTERNAL and INTRINSIC",
            );
            diagnostics.push(diagnostic);
        }
        arguments.push(Argument {
            id: stable_id(
                "ffi-argument",
                &[
                    snapshot,
                    &provider.program_unit_id,
                    &(position + 1).to_string(),
                    name,
                ],
            ),
            position: position + 1,
            normalized_name: name.clone(),
            declared_type,
            type_source,
            character_length: entity.character_length.clone(),
            dimensions: entity.dimensions.clone(),
            is_external: entity.external,
            is_intrinsic: entity.intrinsic,
            locators: entity.locators.clone(),
            implicit_rule_id: implicit_rule_id.filter(|id| id != "fortran-default"),
            conflict_state: if entity.conflict {
                "conflicting".to_owned()
            } else {
                String::new()
            },
        });
    }
    let function_result = function_result(
        &provider,
        span,
        header,
        &entities,
        &implicit,
        snapshot,
        &mut diagnostics,
    );
    ParsedUnit {
        provider,
        header_span: header.raw_spans[0].clone(),
        end_span: span
            .end_index
            .map(|index| statements[index].raw_spans[0].clone()),
        arguments,
        function_result,
        implicit_rules,
        parameters,
        diagnostics,
    }
}

fn formal_arguments(
    text: &str,
    declaration: &StartDeclaration,
    provider: &SelectedProvider,
    statement: &LogicalStatement,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<String> {
    if !matches!(declaration.kind.as_str(), "subroutine" | "function") {
        return Vec::new();
    }
    let Some(open) = text.find('(') else {
        return Vec::new();
    };
    let Some(close) = matching_paren(text, open) else {
        diagnostics.push(diag_for_provider(
            provider,
            "FFI-MALFORMED-FORMAL-ARGUMENTS",
            "warning",
            statement,
            "program-unit declaration has an unclosed formal-argument list",
        ));
        return Vec::new();
    };
    split_top_level(&text[open + 1..close], ',')
        .into_iter()
        .filter_map(|name| {
            let normalized = normalize_identifier(&name);
            if normalized.is_empty() {
                None
            } else if !is_identifier(&normalized) {
                diagnostics.push(diag_for_provider(
                    provider,
                    "FFI-MALFORMED-FORMAL-ARGUMENTS",
                    "warning",
                    statement,
                    "formal argument is not an identifier",
                ));
                None
            } else {
                Some(normalized)
            }
        })
        .collect()
}

#[derive(Clone)]
struct TypeSpec {
    name: String,
    character_length: Option<String>,
}

fn type_statement(text: &str) -> Option<(TypeSpec, &str)> {
    for name in [
        "DOUBLE PRECISION",
        "DOUBLE COMPLEX",
        "INTEGER",
        "REAL",
        "COMPLEX",
        "LOGICAL",
    ] {
        if let Some(rest) = text
            .strip_prefix(name)
            .filter(|rest| rest.starts_with(char::is_whitespace) || rest.starts_with("::"))
        {
            return Some((
                TypeSpec {
                    name: name.to_owned(),
                    character_length: None,
                },
                rest.trim_start().trim_start_matches("::").trim_start(),
            ));
        }
    }
    let rest = text.strip_prefix("CHARACTER")?;
    let rest = rest.trim_start();
    if let Some(rest) = rest.strip_prefix('*') {
        let (length, remainder) = character_length(rest)?;
        return Some((
            TypeSpec {
                name: "CHARACTER".to_owned(),
                character_length: Some(length),
            },
            remainder,
        ));
    }
    if rest.starts_with(char::is_whitespace) || rest.starts_with("::") || rest.is_empty() {
        return Some((
            TypeSpec {
                name: "CHARACTER".to_owned(),
                character_length: None,
            },
            rest.trim_start().trim_start_matches("::").trim_start(),
        ));
    }
    None
}

fn looks_like_unsupported_type_declaration(text: &str) -> bool {
    [
        "DOUBLE PRECISION",
        "DOUBLE COMPLEX",
        "INTEGER",
        "REAL",
        "COMPLEX",
        "LOGICAL",
        "CHARACTER",
    ]
    .iter()
    .any(|prefix| text.starts_with(prefix))
}

fn character_length(input: &str) -> Option<(String, &str)> {
    let input = input.trim_start();
    if input.starts_with('(') {
        let end = matching_paren(input, 0)?;
        return Some((input[..=end].to_owned(), input[end + 1..].trim_start()));
    }
    let length: String = input
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect();
    (!length.is_empty()).then(|| {
        let rest = &input[length.len()..];
        (length, rest.trim_start())
    })
}

fn entity_with_dimensions(item: &str) -> Option<(String, Vec<Dimension>)> {
    let item = item.trim();
    let name: String = item
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '$')
        .collect();
    if !is_identifier(&name) {
        return None;
    }
    let rest = item[name.len()..].trim();
    if rest.is_empty() {
        return Some((name, Vec::new()));
    }
    if !rest.starts_with('(') {
        return None;
    }
    let end = matching_paren(rest, 0)?;
    if !rest[end + 1..].trim().is_empty() {
        return None;
    }
    let dimensions = split_top_level(&rest[1..end], ',')
        .into_iter()
        .map(parse_dimension)
        .collect();
    Some((name, dimensions))
}

fn parse_dimension(raw: String) -> Dimension {
    let normalized_expression = raw.trim().to_owned();
    if normalized_expression == "*" {
        return Dimension {
            lower_bound: None,
            upper_bound: None,
            assumed_size: true,
            normalized_expression,
        };
    }
    let parts = split_top_level(&normalized_expression, ':');
    if parts.len() == 2 {
        Dimension {
            lower_bound: Some(parts[0].trim().to_owned()).filter(|v| !v.is_empty()),
            upper_bound: Some(parts[1].trim().to_owned()).filter(|v| !v.is_empty()),
            assumed_size: parts[1].trim() == "*",
            normalized_expression,
        }
    } else {
        Dimension {
            lower_bound: None,
            upper_bound: Some(normalized_expression.clone()),
            assumed_size: false,
            normalized_expression,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_type(
    entities: &mut BTreeMap<String, EntityState>,
    name: &str,
    spec: &TypeSpec,
    dimensions: Vec<Dimension>,
    locator: Value,
    provider: &SelectedProvider,
    statement: &LogicalStatement,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let key = normalize_identifier(name);
    let entity = entities.entry(key.clone()).or_insert_with(|| EntityState {
        declared_type: None,
        character_length: None,
        dimensions: Vec::new(),
        external: false,
        intrinsic: false,
        locators: Vec::new(),
        conflict: false,
        diagnostic_ids: Vec::new(),
    });
    if entity
        .declared_type
        .as_ref()
        .is_some_and(|current| current != &spec.name)
    {
        let diagnostic = diag_for_provider(
            provider,
            "FFI-CONFLICTING-EXPLICIT-TYPE",
            "warning",
            statement,
            "incompatible explicit type declarations",
        );
        entity.conflict = true;
        entity.diagnostic_ids.push(diagnostic.id.clone());
        diagnostics.push(diagnostic);
    } else {
        entity.declared_type = Some(spec.name.clone());
        if spec.name == "CHARACTER" {
            entity.character_length = spec.character_length.clone();
        }
    }
    if !dimensions.is_empty() {
        merge_dimensions(entity, dimensions, provider, statement, diagnostics);
    }
    entity.locators.push(locator);
}

fn apply_dimensions(
    entities: &mut BTreeMap<String, EntityState>,
    name: &str,
    dimensions: Vec<Dimension>,
    locator: Value,
    provider: &SelectedProvider,
    statement: &LogicalStatement,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let key = normalize_identifier(name);
    let entity = entities.entry(key.clone()).or_insert_with(|| EntityState {
        declared_type: None,
        character_length: None,
        dimensions: Vec::new(),
        external: false,
        intrinsic: false,
        locators: Vec::new(),
        conflict: false,
        diagnostic_ids: Vec::new(),
    });
    merge_dimensions(entity, dimensions, provider, statement, diagnostics);
    entity.locators.push(locator);
}

fn merge_dimensions(
    entity: &mut EntityState,
    dimensions: Vec<Dimension>,
    provider: &SelectedProvider,
    statement: &LogicalStatement,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !entity.dimensions.is_empty() && entity.dimensions != dimensions {
        let diagnostic = diag_for_provider(
            provider,
            "FFI-CONFLICTING-DIMENSIONS",
            "warning",
            statement,
            "incompatible repeated dimensions",
        );
        entity.conflict = true;
        entity.diagnostic_ids.push(diagnostic.id.clone());
        diagnostics.push(diagnostic);
    } else if entity.dimensions.is_empty() {
        entity.dimensions = dimensions;
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_attribute(
    entities: &mut BTreeMap<String, EntityState>,
    items: &str,
    external: bool,
    intrinsic: bool,
    locator: Value,
    _provider: &SelectedProvider,
    _statement: &LogicalStatement,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    for item in split_top_level(items, ',') {
        let key = normalize_identifier(&item);
        if !is_identifier(&key) {
            continue;
        }
        let entity = entities.entry(key.clone()).or_insert_with(|| EntityState {
            declared_type: None,
            character_length: None,
            dimensions: Vec::new(),
            external: false,
            intrinsic: false,
            locators: Vec::new(),
            conflict: false,
            diagnostic_ids: Vec::new(),
        });
        entity.external |= external;
        entity.intrinsic |= intrinsic;
        entity.locators.push(locator.clone());
    }
}

#[derive(Clone)]
struct ImplicitMap {
    letters: [Option<String>; 26],
    rule_ids: [Option<String>; 26],
    rules: Vec<ImplicitRule>,
    invalid: bool,
}

impl Default for ImplicitMap {
    fn default() -> Self {
        let letters = std::array::from_fn(|index| {
            Some(
                if (8..=13).contains(&index) {
                    "INTEGER"
                } else {
                    "REAL"
                }
                .to_owned(),
            )
        });
        Self {
            letters,
            rule_ids: std::array::from_fn(|_| None),
            rules: Vec::new(),
            invalid: false,
        }
    }
}

impl ImplicitMap {
    fn none(&mut self, provider: &SelectedProvider, statement: &LogicalStatement) {
        self.letters.fill(None);
        let rule = ImplicitRule {
            id: stable_id(
                "ffi-implicit-rule",
                &[
                    &provider.program_unit_id,
                    &statement.raw_spans[0].start.to_string(),
                    "none",
                ],
            ),
            state: "implicit_none".to_owned(),
            ranges: Vec::new(),
            declared_type: None,
            locator: locator(&provider.snapshot_id, provider, statement),
        };
        self.rule_ids.fill(Some(rule.id.clone()));
        self.rules.push(rule);
    }
    fn apply(
        &mut self,
        text: &str,
        provider: &SelectedProvider,
        statement: &LogicalStatement,
    ) -> std::result::Result<(), String> {
        let mut rest = text.trim();
        let mut affected = Vec::new();
        while !rest.is_empty() {
            let Some(open) = rest.find('(') else {
                return Err("IMPLICIT clause lacks a letter range".to_owned());
            };
            let type_name = rest[..open].trim().trim_end_matches(',').trim();
            let Some((spec, _)) = type_statement(&format!("{type_name} X")) else {
                return Err("IMPLICIT clause has unsupported type".to_owned());
            };
            let Some(close) = matching_paren(rest, open) else {
                return Err("IMPLICIT letter range is unclosed".to_owned());
            };
            for range in split_top_level(&rest[open + 1..close], ',') {
                let chars: Vec<_> = range
                    .trim()
                    .chars()
                    .filter(char::is_ascii_alphabetic)
                    .collect();
                if chars.len() == 1 {
                    affected.push((chars[0], chars[0], spec.name.clone()));
                } else if chars.len() == 2 {
                    affected.push((chars[0], chars[1], spec.name.clone()));
                } else {
                    return Err("IMPLICIT letter range is malformed".to_owned());
                }
            }
            rest = rest[close + 1..].trim_start_matches(',').trim_start();
        }
        let ranges = affected
            .iter()
            .map(|(from, to, ty)| format!("{from}-{to}={ty}"))
            .collect::<Vec<_>>();
        let rule = ImplicitRule {
            id: stable_id(
                "ffi-implicit-rule",
                &[
                    &provider.program_unit_id,
                    &statement.raw_spans[0].start.to_string(),
                    &ranges.join(","),
                ],
            ),
            state: "explicit".to_owned(),
            ranges,
            declared_type: None,
            locator: locator(&provider.snapshot_id, provider, statement),
        };
        for (from, to, type_name) in affected {
            let first = from.to_ascii_uppercase() as usize - 'A' as usize;
            let last = to.to_ascii_uppercase() as usize - 'A' as usize;
            if first > 25 || last > 25 || first > last {
                return Err("IMPLICIT letter range is invalid".to_owned());
            }
            for index in first..=last {
                self.letters[index] = Some(type_name.clone());
                self.rule_ids[index] = Some(rule.id.clone());
            }
        }
        self.rules.push(rule);
        Ok(())
    }
    fn latest_rule(&self) -> Option<&ImplicitRule> {
        self.rules.last()
    }
    fn invalidate(&mut self) {
        self.invalid = true;
    }
    fn type_for(&self, name: &str) -> Option<(String, String)> {
        if self.invalid {
            return None;
        }
        let first = name.chars().next()?.to_ascii_uppercase();
        if !first.is_ascii_alphabetic() {
            return None;
        }
        let index = first as usize - 'A' as usize;
        self.letters[index].clone().map(|ty| {
            (
                ty,
                self.rule_ids[index]
                    .clone()
                    .unwrap_or_else(|| "fortran-default".to_owned()),
            )
        })
    }
}

fn parse_parameters(
    items: &str,
    snapshot: &str,
    provider: &SelectedProvider,
    statement: &LogicalStatement,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<Parameter> {
    let body = items
        .trim()
        .strip_prefix('(')
        .and_then(|v| v.strip_suffix(')'));
    let Some(body) = body else {
        diagnostics.push(diag_for_provider(
            provider,
            "FFI-MALFORMED-PARAMETER",
            "warning",
            statement,
            "PARAMETER declaration is not parenthesized",
        ));
        return Vec::new();
    };
    split_top_level(body, ',')
        .into_iter()
        .filter_map(|item| {
            let (name, expression) = item.split_once('=')?;
            let name = normalize_identifier(name);
            is_identifier(&name).then(|| Parameter {
                id: stable_id(
                    "ffi-parameter",
                    &[
                        snapshot,
                        &provider.program_unit_id,
                        &name,
                        &statement.raw_spans[0].start.to_string(),
                    ],
                ),
                normalized_name: name,
                raw_expression: expression.trim().to_owned(),
                locator: locator(snapshot, provider, statement),
            })
        })
        .collect()
}

fn function_result(
    provider: &SelectedProvider,
    span: &UnitSpan,
    header: &LogicalStatement,
    entities: &BTreeMap<String, EntityState>,
    implicit: &ImplicitMap,
    snapshot: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<FunctionResult> {
    if provider.kind != "function" {
        return None;
    }
    let prefix = span.declaration.declared_return_type.clone();
    let entity = entities.get(&provider.normalized_name);
    let explicit = entity.and_then(|entity| entity.declared_type.clone());
    let (declared_type, type_source, conflict) = match (prefix, explicit) {
        (Some(prefix), Some(explicit)) if prefix != explicit => {
            diagnostics.push(diag_for_provider(
                provider,
                "FFI-CONFLICTING-FUNCTION-RESULT",
                "warning",
                header,
                "typed function prefix and explicit function-name declaration disagree",
            ));
            (None, "conflicting".to_owned(), "conflicting".to_owned())
        }
        (Some(prefix), _) => (
            Some(prefix),
            "function_prefix".to_owned(),
            "none".to_owned(),
        ),
        (None, Some(explicit)) => (Some(explicit), "explicit".to_owned(), "none".to_owned()),
        (None, None) => match implicit.type_for(&provider.normalized_name) {
            Some((ty, _)) => (Some(ty), "implicit_rule".to_owned(), "none".to_owned()),
            None => (None, "unknown".to_owned(), "unknown".to_owned()),
        },
    };
    let character_length = entity.and_then(|entity| entity.character_length.clone());
    Some(FunctionResult {
        id: stable_id(
            "ffi-function-result",
            &[snapshot, &provider.program_unit_id],
        ),
        declared_type,
        type_source,
        character_length,
        locator: locator(snapshot, provider, header),
        conflict_state: conflict,
    })
}

fn is_specification_statement(text: &str) -> bool {
    [
        "COMMON",
        "SAVE",
        "DATA",
        "EQUIVALENCE",
        "FORMAT",
        "NAMELIST",
        "INCLUDE",
        "ENTRY",
    ]
    .iter()
    .any(|keyword| {
        text == *keyword
            || text
                .strip_prefix(keyword)
                .is_some_and(|rest| rest.starts_with(char::is_whitespace))
    })
}

fn locator(snapshot: &str, provider: &SelectedProvider, statement: &LogicalStatement) -> Value {
    let span = &statement.raw_spans[0];
    json!({
        "id": stable_id("ffi-source-locator", &[snapshot, &provider.program_unit_id, &span.start.to_string(), &span.end.to_string()]),
        "source_subset": provider.source_subset,
        "source_path": provider.source_path,
        "source_sha256": provider.raw_sha256,
        "raw_byte_start": span.start,
        "raw_byte_end": statement.raw_spans.last().map(|span| span.end).unwrap_or(span.end),
        "physical_line_start": statement.physical_line_start,
        "physical_line_end": statement.physical_line_end,
    })
}

fn diag(
    group: &SourceGroup,
    rule: &str,
    severity: &str,
    statement: &LogicalStatement,
    _message: &str,
) -> Diagnostic {
    Diagnostic {
        id: stable_id(
            "ffi-diagnostic",
            &[
                rule,
                &group.subset,
                &group.path,
                &statement.raw_spans[0].start.to_string(),
            ],
        ),
        severity: severity.to_owned(),
        rule_id: rule.to_owned(),
        program_unit_id: String::new(),
        line: statement.physical_line_start,
    }
}

fn diag_for_provider(
    provider: &SelectedProvider,
    rule: &str,
    severity: &str,
    statement: &LogicalStatement,
    _message: &str,
) -> Diagnostic {
    Diagnostic {
        id: stable_id(
            "ffi-diagnostic",
            &[
                rule,
                &provider.program_unit_id,
                &statement.raw_spans[0].start.to_string(),
            ],
        ),
        severity: severity.to_owned(),
        rule_id: rule.to_owned(),
        program_unit_id: provider.program_unit_id.clone(),
        line: statement.physical_line_start,
    }
}

fn normalize_identifier(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}
fn is_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
        && value
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
}

fn split_top_level(input: &str, separator: char) -> Vec<String> {
    let mut output = Vec::new();
    let mut start = 0;
    let mut level = 0_i32;
    let mut quote = None;
    for (index, character) in input.char_indices() {
        if let Some(current) = quote {
            if character == current {
                quote = None;
            }
            continue;
        }
        if matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if character == '(' {
            level += 1;
        } else if character == ')' {
            level -= 1;
        } else if character == separator && level == 0 {
            output.push(input[start..index].trim().to_owned());
            start = index + character.len_utf8();
        }
    }
    output.push(input[start..].trim().to_owned());
    output
        .into_iter()
        .filter(|value| !value.is_empty())
        .collect()
}

fn matching_paren(input: &str, open: usize) -> Option<usize> {
    let mut level = 0_i32;
    let mut quote = None;
    for (index, character) in input.char_indices().skip_while(|(index, _)| *index < open) {
        if let Some(current) = quote {
            if character == current {
                quote = None;
            }
            continue;
        }
        if matches!(character, '\'' | '"') {
            quote = Some(character);
        } else if character == '(' {
            level += 1;
        } else if character == ')' {
            level -= 1;
            if level == 0 {
                return Some(index);
            }
        }
    }
    None
}

fn write_outputs(
    output_dir: &Path,
    snapshot: &str,
    selection_hash: &str,
    units: &[ParsedUnit],
    source_rows: Vec<Value>,
) -> Result<InventoryResult> {
    let mut summary = InventorySummary {
        selected_program_units: units.len(),
        processed_program_units: units.len(),
        source_files: source_rows.len(),
        ..InventorySummary::default()
    };
    let mut interface_rows = Vec::new();
    let mut argument_rows = Vec::new();
    let mut result_rows = Vec::new();
    let mut external_rows = Vec::new();
    let mut implicit_rows = Vec::new();
    let mut parameter_rows = Vec::new();
    let mut diagnostics = BTreeMap::<(String, String), (Diagnostic, usize)>::new();
    let mut locators = BTreeMap::<String, Value>::new();
    for unit in units {
        match unit.provider.kind.as_str() {
            "subroutine" => summary.subroutines += 1,
            "function" => summary.functions += 1,
            "block_data" => summary.block_data_units += 1,
            "program" => summary.programs += 1,
            _ => {}
        }
        for argument in &unit.arguments {
            summary.formal_arguments += 1;
            match argument.type_source.as_str() {
                "explicit" => summary.explicit_types += 1,
                "implicit_rule" => summary.implicit_types += 1,
                _ => summary.unknown_types += 1,
            }
            if !argument.dimensions.is_empty() {
                summary.array_arguments += 1;
            }
            if argument.declared_type.as_deref() == Some("CHARACTER") {
                summary.character_arguments += 1;
            }
            if argument.is_external {
                summary.external_formal_arguments += 1;
                external_rows.push(json!([
                    argument.id,
                    unit.provider.program_unit_id,
                    argument.normalized_name,
                    "unknown"
                ]));
            }
            let locator_ids = argument
                .locators
                .iter()
                .map(|locator| insert_locator(&mut locators, locator))
                .collect::<Vec<_>>();
            argument_rows.push(json!([
                argument.id,
                unit.provider.program_unit_id,
                argument.position,
                argument.normalized_name,
                argument.declared_type,
                argument.type_source,
                argument.character_length,
                argument
                    .dimensions
                    .iter()
                    .map(dimension_value)
                    .collect::<Vec<_>>(),
                argument.is_external,
                argument.is_intrinsic,
                locator_ids,
                argument.implicit_rule_id,
                (!argument.conflict_state.is_empty()).then_some(argument.conflict_state.clone())
            ]));
        }
        if let Some(result) = &unit.function_result {
            summary.function_results += 1;
            let locator_id = insert_locator(&mut locators, &result.locator);
            result_rows.push(json!([
                result.id,
                unit.provider.program_unit_id,
                result.declared_type,
                result.type_source,
                result.character_length,
                locator_id,
                (!result.conflict_state.is_empty() && result.conflict_state != "none")
                    .then_some(result.conflict_state.clone())
            ]));
        }
        for rule in &unit.implicit_rules {
            let locator_id = insert_locator(&mut locators, &rule.locator);
            implicit_rows.push(json!([
                rule.id,
                unit.provider.program_unit_id,
                rule.state,
                rule.ranges,
                rule.declared_type,
                locator_id
            ]));
        }
        for parameter in &unit.parameters {
            let locator_id = insert_locator(&mut locators, &parameter.locator);
            parameter_rows.push(json!([
                parameter.id,
                unit.provider.program_unit_id,
                parameter.normalized_name,
                parameter.raw_expression,
                locator_id
            ]));
        }
        let unit_diagnostic_ids = unit
            .diagnostics
            .iter()
            .map(|diag| diag.id.clone())
            .collect::<Vec<_>>();
        if !unit_diagnostic_ids.is_empty() {
            summary.conflicting_interfaces += unit
                .diagnostics
                .iter()
                .filter(|diag| diag.rule_id.contains("CONFLICT"))
                .count();
        }
        let header_locator = locator_from_span(snapshot, &unit.provider, &unit.header_span);
        let header_locator_id = insert_locator(&mut locators, &header_locator);
        let end_locator_id = unit.end_span.as_ref().map(|span| {
            let locator = locator_from_span(snapshot, &unit.provider, span);
            insert_locator(&mut locators, &locator)
        });
        interface_rows.push(json!([
            unit.provider.program_unit_id,
            unit.provider.normalized_name,
            unit.provider.kind,
            source_id(snapshot, &unit.provider),
            unit.provider.selection_category,
            header_locator_id,
            end_locator_id,
            unit_diagnostic_ids
        ]));
        for diag in &unit.diagnostics {
            let key = (diag.program_unit_id.clone(), diag.rule_id.clone());
            let entry = diagnostics.entry(key).or_insert_with(|| (diag.clone(), 0));
            entry.1 += 1;
        }
    }
    let mut diagnostic_rows = diagnostics
        .into_values()
        .map(|(diag, occurrences)| {
            json!([
                diag.id,
                diag.severity,
                diag.rule_id,
                diag.program_unit_id,
                diag.line,
                occurrences
            ])
        })
        .collect::<Vec<_>>();
    summary.review_items = diagnostic_rows.len();
    for rows in [
        &mut interface_rows,
        &mut argument_rows,
        &mut result_rows,
        &mut external_rows,
        &mut implicit_rows,
        &mut parameter_rows,
        &mut diagnostic_rows,
    ] {
        rows.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    }
    let mut outputs = BTreeMap::<&str, Vec<u8>>::new();
    outputs.insert("selected-source-files.json", compact(&json!({"schema_id":"slatec-rs/ffi-selected-source", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "columns":["id","source_subset","source_path","raw_sha256","normalized_sha256","selected_program_units"], "records":source_rows}))?);
    outputs.insert("source-locator-index.json", compact(&json!({"schema_id":"slatec-rs/ffi-source-locator", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "columns":["id","source_id","raw_byte_start","raw_byte_end","physical_line_start","physical_line_end"], "records":locators.into_values().map(|locator| locator_row(snapshot, locator)).collect::<Vec<_>>() }))?);
    outputs.insert("routine-interface-index.json", compact(&json!({"schema_id":"slatec-rs/ffi-interface", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "columns":["program_unit_id","normalized_name","kind","source_id","selection_category","declaration_locator_id","end_locator_id","diagnostic_ids"], "records":interface_rows}))?);
    outputs.insert("argument-index.json", compact(&json!({"schema_id":"slatec-rs/ffi-argument", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "dimensions_encoding":["lower_bound","upper_bound","assumed_size","raw_spelling"], "columns":["id","program_unit_id","position","normalized_name","declared_type","type_source","character_length","dimensions","is_external","is_intrinsic","declaration_locator_ids","implicit_rule_id","conflict_state"], "records":argument_rows}))?);
    outputs.insert("function-results.json", compact(&json!({"schema_id":"slatec-rs/ffi-function-result", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "columns":["id","program_unit_id","declared_type","type_source","character_length","locator_id","conflict_state"], "records":result_rows}))?);
    outputs.insert("external-arguments.json", compact(&json!({"schema_id":"slatec-rs/ffi-external-argument", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "columns":["argument_id","program_unit_id","normalized_name","procedure_signature_status"], "records":external_rows}))?);
    outputs.insert("implicit-rules.json", compact(&json!({"schema_id":"slatec-rs/ffi-implicit-rule", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "columns":["id","program_unit_id","state","ranges","declared_type","locator_id"], "records":implicit_rows}))?);
    outputs.insert("parameter-index.json", compact(&json!({"schema_id":"slatec-rs/ffi-parameter", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "columns":["id","program_unit_id","normalized_name","raw_expression","locator_id"], "records":parameter_rows}))?);
    outputs.insert("source-prologue-name-order.json", compact(&json!({"schema_id":"slatec-rs/ffi-prologue-comparison", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "note":"No documented argument spelling is committed by the prologue stage; source/prologue name-order comparison is deferred to ignored raw evidence.", "columns":["program_unit_id","comparison_status"], "records":units.iter().filter(|unit| unit.provider.source_subset=="main-src").map(|unit| json!([unit.provider.program_unit_id,"not_available"])).collect::<Vec<_>>() }))?);
    outputs.insert("diagnostics.json", compact(&json!({"schema_id":"slatec-rs/ffi-diagnostic", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "columns":["id","severity","rule_id","program_unit_id","first_physical_line","occurrence_count"], "records":diagnostic_rows}))?);
    let semantic_hash = semantic_hash(&outputs);
    let status = if summary.review_items == 0 {
        "success"
    } else {
        "success_with_review_items"
    };
    outputs.insert("manifest.json", compact(&json!({"id":stable_id("ffi-inventory", &[snapshot, &semantic_hash]), "schema_id":"slatec-rs/ffi-interface-inventory", "schema_version":SCHEMA_VERSION, "snapshot_id":snapshot, "created_by":format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at":CREATED_AT, "semantic_version":SEMANTIC_VERSION, "selected_provider_manifest_sha256":selection_hash, "output_semantic_hash":semantic_hash, "status":status, "summary":summary}))?);
    outputs.insert("inventory-summary.md", format!("# Selected SLATEC FFI interface inventory\n\n- Snapshot: `{snapshot}`\n- Status: `{status}`\n- Selected and processed program units: {}\n- Source files: {}\n- Formal arguments: {} (explicit: {}; implicit: {}; unknown: {})\n- Array arguments: {}; character arguments: {}; external formals: {}\n- Function results: {}; review items: {}\n\nThis inventory is source-local and conservative. It is not a generated FFI, ABI proof, native component plan, or safe Rust API.\n", summary.processed_program_units, summary.source_files, summary.formal_arguments, summary.explicit_types, summary.implicit_types, summary.unknown_types, summary.array_arguments, summary.character_arguments, summary.external_formal_arguments, summary.function_results, summary.review_items).into_bytes());
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
            "compact FFI inventory would be {total} bytes, above the 4 MB policy limit ({sizes})"
        )));
    }
    promote(output_dir, snapshot, &outputs)?;
    Ok(InventoryResult {
        snapshot_id: snapshot.to_owned(),
        status: status.to_owned(),
        semantic_hash,
        output_dir: output_dir.to_owned(),
        summary,
    })
}

fn dimension_value(value: &Dimension) -> Value {
    json!([
        value.lower_bound,
        value.upper_bound,
        value.assumed_size,
        value.normalized_expression
    ])
}

fn insert_locator(locators: &mut BTreeMap<String, Value>, locator: &Value) -> String {
    let id = locator["id"]
        .as_str()
        .expect("internal source locator has a stable ID")
        .to_owned();
    locators
        .entry(id.clone())
        .or_insert_with(|| locator.clone());
    id
}

fn locator_row(snapshot: &str, locator: Value) -> Value {
    json!([
        locator["id"],
        stable_id(
            "ffi-source",
            &[
                snapshot,
                locator["source_subset"].as_str().expect("source subset"),
                locator["source_path"].as_str().expect("source path"),
                locator["source_sha256"].as_str().expect("source hash"),
            ]
        ),
        locator["raw_byte_start"],
        locator["raw_byte_end"],
        locator["physical_line_start"],
        locator["physical_line_end"],
    ])
}

fn source_id(snapshot: &str, provider: &SelectedProvider) -> String {
    stable_id(
        "ffi-source",
        &[
            snapshot,
            &provider.source_subset,
            &provider.source_path,
            &provider.raw_sha256,
        ],
    )
}

fn locator_from_span(snapshot: &str, provider: &SelectedProvider, span: &RawSpan) -> Value {
    json!({"id":stable_id("ffi-source-locator", &[snapshot,&provider.program_unit_id,&span.start.to_string(),&span.end.to_string()]),"source_subset":provider.source_subset,"source_path":provider.source_path,"source_sha256":provider.raw_sha256,"raw_byte_start":span.start,"raw_byte_end":span.end,"physical_line_start":span.line,"physical_line_end":span.line})
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
    let parent = output_dir
        .parent()
        .ok_or_else(|| CorpusError::Policy("FFI output directory must have a parent".to_owned()))?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        "{}.staging-{snapshot}",
        output_dir.file_name().unwrap_or_default().to_string_lossy()
    ));
    if staging.exists() {
        return Err(CorpusError::Verification(format!(
            "FFI inventory staging directory exists: {}",
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
                "FFI inventory backup directory exists: {}",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_conservative_scalar_array_character_and_external_facts() {
        let statements = fixed_form::logical_statements(&fixed_form::physical_lines(b"      SUBROUTINE DEMO(I,A,NAME,F)\n      IMPLICIT DOUBLE PRECISION (A-H,O-Z)\n      INTEGER I\n      DOUBLE PRECISION A(0:N,*)\n      CHARACTER*(*) NAME\n      EXTERNAL F\n      END\n"));
        let provider = SelectedProvider {
            snapshot_id: "s".to_owned(),
            program_unit_id: "u".to_owned(),
            normalized_name: "DEMO".to_owned(),
            kind: "subroutine".to_owned(),
            source_subset: "test".to_owned(),
            source_path: "demo.f".to_owned(),
            raw_sha256: "h".to_owned(),
            normalized_sha256: "h".to_owned(),
            selection_category: "selected_numerical_program_unit".to_owned(),
        };
        let span = UnitSpan {
            declaration_index: 0,
            end_index: Some(6),
            declaration: fixed_form::start_declaration(&statements[0].normalized_statement_text)
                .unwrap(),
            diagnostics: Vec::new(),
        };
        let parsed = parse_unit(provider, &span, &statements, "s");
        assert_eq!(parsed.arguments.len(), 4);
        assert_eq!(
            parsed.arguments[0].declared_type.as_deref(),
            Some("INTEGER")
        );
        assert_eq!(parsed.arguments[1].dimensions.len(), 2);
        assert_eq!(parsed.arguments[2].character_length.as_deref(), Some("(*)"));
        assert!(parsed.arguments[3].is_external);
    }

    #[test]
    fn uses_function_prefix_then_explicit_then_implicit_and_preserves_conflicts() {
        let statements = fixed_form::logical_statements(&fixed_form::physical_lines(
            b"      REAL FUNCTION F(X)\n      INTEGER F\n      END\n",
        ));
        let provider = SelectedProvider {
            snapshot_id: "s".to_owned(),
            program_unit_id: "u".to_owned(),
            normalized_name: "F".to_owned(),
            kind: "function".to_owned(),
            source_subset: "test".to_owned(),
            source_path: "f.f".to_owned(),
            raw_sha256: "h".to_owned(),
            normalized_sha256: "h".to_owned(),
            selection_category: "selected_numerical_program_unit".to_owned(),
        };
        let span = UnitSpan {
            declaration_index: 0,
            end_index: Some(2),
            declaration: fixed_form::start_declaration(&statements[0].normalized_statement_text)
                .unwrap(),
            diagnostics: Vec::new(),
        };
        let parsed = parse_unit(provider, &span, &statements, "s");
        assert_eq!(
            parsed.function_result.unwrap().conflict_state,
            "conflicting"
        );
    }

    #[test]
    fn split_top_level_keeps_dimension_expressions_and_repeated_declarations_deterministic() {
        assert_eq!(
            split_top_level("A(LDA,*), B(0:N), C", ','),
            vec!["A(LDA,*)", "B(0:N)", "C"]
        );
        assert_eq!(
            parse_dimension("0:N".to_owned()).lower_bound.as_deref(),
            Some("0")
        );
        assert!(parse_dimension("*".to_owned()).assumed_size);
    }

    #[test]
    fn implicit_none_leaves_untyped_arguments_for_review() {
        let statements = fixed_form::logical_statements(&fixed_form::physical_lines(
            b"      SUBROUTINE NONE(A)\n      IMPLICIT NONE\n      END\n",
        ));
        let provider = SelectedProvider {
            snapshot_id: "s".to_owned(),
            program_unit_id: "u".to_owned(),
            normalized_name: "NONE".to_owned(),
            kind: "subroutine".to_owned(),
            source_subset: "test".to_owned(),
            source_path: "none.f".to_owned(),
            raw_sha256: "h".to_owned(),
            normalized_sha256: "h".to_owned(),
            selection_category: "selected_numerical_program_unit".to_owned(),
        };
        let span = UnitSpan {
            declaration_index: 0,
            end_index: Some(2),
            declaration: fixed_form::start_declaration(&statements[0].normalized_statement_text)
                .unwrap(),
            diagnostics: Vec::new(),
        };
        let parsed = parse_unit(provider, &span, &statements, "s");
        assert_eq!(parsed.arguments[0].type_source, "unknown");
        assert!(
            parsed
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule_id == "FFI-UNKNOWN-ARGUMENT-TYPE")
        );
    }

    #[test]
    fn compact_locators_refer_back_to_a_stable_source_identity() {
        let locator = json!({
            "id":"locator-a", "source_subset":"lin", "source_path":"dasum.f",
            "source_sha256":"abc", "raw_byte_start":1, "raw_byte_end":2,
            "physical_line_start":1, "physical_line_end":1
        });
        let row = locator_row("snapshot", locator);
        assert_eq!(row[0], "locator-a");
        assert!(
            row[1]
                .as_str()
                .is_some_and(|id| id.starts_with("ffi-source-"))
        );
    }

    #[test]
    fn unsupported_type_syntax_blocks_implicit_interface_typing() {
        let statements = fixed_form::logical_statements(&fixed_form::physical_lines(
            b"      SUBROUTINE OLD(I)\n      INTEGER*2 I\n      END\n",
        ));
        let provider = SelectedProvider {
            snapshot_id: "s".to_owned(),
            program_unit_id: "u".to_owned(),
            normalized_name: "OLD".to_owned(),
            kind: "subroutine".to_owned(),
            source_subset: "test".to_owned(),
            source_path: "old.f".to_owned(),
            raw_sha256: "h".to_owned(),
            normalized_sha256: "h".to_owned(),
            selection_category: "selected_numerical_program_unit".to_owned(),
        };
        let span = UnitSpan {
            declaration_index: 0,
            end_index: Some(2),
            declaration: fixed_form::start_declaration(&statements[0].normalized_statement_text)
                .unwrap(),
            diagnostics: Vec::new(),
        };
        let parsed = parse_unit(provider, &span, &statements, "s");
        assert_eq!(parsed.arguments[0].type_source, "unknown");
    }
}
