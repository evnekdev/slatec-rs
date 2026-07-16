//! Deterministic provider selection for the complete SLATEC-hosted corpus.
//!
//! This module consumes the full-corpus audit rather than rediscovering a
//! source tree.  It leaves the immutable `main-src` snapshot untouched and
//! records every non-selected provider as evidence.

use crate::error::{CorpusError, Result};
use crate::fixed_form;
use crate::hash;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: &str = "1.0.0";
const SELECTION_SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const COMMITTED_SIZE_LIMIT: u64 = 4_000_000;

#[derive(Deserialize)]
struct SelectionConfig {
    schema_version: u32,
    policy_version: u32,
    semantic_version: u32,
    selected_relocated_subsets: Vec<String>,
    selected_supplemental_numerical_subsets: Vec<String>,
    excluded_tool_subsets: Vec<String>,
    comparison_only_subsets: Vec<String>,
    infrastructure_program_units: Vec<String>,
}

#[derive(Deserialize)]
struct MainArtifactManifest {
    snapshot_id: String,
    sha256: String,
}

#[derive(Deserialize)]
struct FullAuditManifest {
    main_src_snapshot_id: String,
    audit_input_hash: String,
    audit_configuration_hash: String,
    directory_index_hashes: BTreeMap<String, String>,
    output_semantic_hash: String,
}

#[derive(Clone, Debug)]
struct SourceFile {
    id: String,
    subset: String,
    relationship: String,
    path: String,
    raw_sha256: String,
    normalized_sha256: String,
    provider_ids: Vec<String>,
    scan_diagnostic_rules: Vec<String>,
}

#[derive(Clone, Debug)]
struct ProviderGroup {
    normalized_name: String,
    provider_ids: Vec<String>,
    kinds: Vec<String>,
    relationship_classification: String,
    relationship_id: String,
}

#[derive(Clone, Debug)]
struct CatalogueRecord {
    normalized_name: String,
    in_current_list: bool,
    classification: String,
    classification_basis: String,
    list_lines: Vec<u64>,
    toc_lines: Vec<u64>,
}

#[derive(Clone, Debug)]
struct EntryEvidence {
    source_file_id: String,
    line: u64,
    raw_byte_start: u64,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct CompleteSelectionResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
    pub selected_program_units: usize,
    pub unresolved_provider_count: usize,
}

/// Select one exact SLATEC-hosted provider per reconciled identity.
pub fn select(
    evidence_dir: &Path,
    corpus_manifest_dir: &Path,
    full_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<CompleteSelectionResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "complete-corpus selection is evidence-only and requires --offline; run audit-full-corpus to acquire inputs"
                .to_owned(),
        ));
    }
    let config_path = Path::new("metadata/complete-corpus-selection.toml");
    let config_bytes = fs::read(config_path)?;
    let config: SelectionConfig =
        toml::from_str(std::str::from_utf8(&config_bytes).map_err(|_| {
            CorpusError::Policy("complete-corpus selection configuration is not UTF-8".to_owned())
        })?)?;
    if config.schema_version != 1 || config.policy_version != 1 || config.semantic_version != 1 {
        return Err(CorpusError::Policy(
            "only complete-corpus selection policy/schema version 1 is supported".to_owned(),
        ));
    }

    let main: MainArtifactManifest =
        read_json(&corpus_manifest_dir.join("artifact-manifest.json"))?;
    let audit: FullAuditManifest = read_json(&full_corpus_dir.join("manifest.json"))?;
    if audit.main_src_snapshot_id != main.snapshot_id {
        return Err(CorpusError::Verification(
            "full-corpus audit does not refer to the immutable main-src snapshot".to_owned(),
        ));
    }
    let sources = source_files(&full_corpus_dir.join("source-file-index.json"))?;
    let groups = provider_groups(
        &full_corpus_dir.join("program-unit-union.json"),
        &full_corpus_dir.join("provider-relationships.json"),
    )?;
    let catalogue = catalogue_records(&full_corpus_dir.join("catalogue-reconciliation.json"))?;

    let mut source_by_provider = BTreeMap::new();
    let mut source_by_id = BTreeMap::new();
    let mut entries_by_name = BTreeMap::<String, Vec<EntryEvidence>>::new();
    for source in &sources {
        for provider_id in &source.provider_ids {
            if source_by_provider
                .insert(provider_id.clone(), source.clone())
                .is_some()
            {
                return Err(CorpusError::Verification(format!(
                    "provider {provider_id} occurs in more than one full-corpus source record"
                )));
            }
        }
        let bytes = verified_source_bytes(evidence_dir, &main.snapshot_id, source)?;
        for (name, evidence) in entry_names(source, &bytes) {
            entries_by_name.entry(name).or_default().push(evidence);
        }
        source_by_id.insert(source.id.clone(), source.clone());
    }
    let _ = source_by_id;

    let infrastructure: BTreeSet<_> = config
        .infrastructure_program_units
        .iter()
        .map(|name| name.to_ascii_uppercase())
        .collect();
    let relocated: BTreeSet<_> = config.selected_relocated_subsets.iter().cloned().collect();
    let supplemental_numerical: BTreeSet<_> = config
        .selected_supplemental_numerical_subsets
        .iter()
        .cloned()
        .collect();
    let tool_subsets: BTreeSet<_> = config.excluded_tool_subsets.iter().cloned().collect();
    let comparison_only: BTreeSet<_> = config.comparison_only_subsets.iter().cloned().collect();

    let mut selected = Vec::<SelectedProvider>::new();
    let mut unresolved = Vec::<Value>::new();
    let mut diagnostics = Vec::<Value>::new();
    let mut selected_provider_ids = BTreeSet::new();
    let group_by_name: BTreeMap<_, _> = groups
        .iter()
        .map(|group| (group.normalized_name.clone(), group.clone()))
        .collect();

    for group in &groups {
        let catalogue_record = catalogue.get(&group.normalized_name);
        let source_only = catalogue_record.is_none();
        let chosen = select_provider(
            group,
            &source_by_provider,
            source_only,
            &relocated,
            &supplemental_numerical,
            &tool_subsets,
        )?;
        match chosen {
            Some((provider_id, selection_rule, review_state)) => {
                let source = source_by_provider.get(&provider_id).ok_or_else(|| {
                    CorpusError::Verification(format!(
                        "selected provider {provider_id} has no source-file record"
                    ))
                })?;
                if source_only
                    && source.subset == "spfun"
                    && !spfun_has_fnlib_prologues(evidence_dir)
                {
                    return Err(CorpusError::Verification(
                        "spfun selection requires FNLIB prologue evidence".to_owned(),
                    ));
                }
                selected_provider_ids.insert(provider_id.clone());
                selected.push(SelectedProvider {
                    program_unit_id: provider_id,
                    normalized_name: group.normalized_name.clone(),
                    kind: group.kinds.join("+"),
                    source_subset: source.subset.clone(),
                    source_relationship: source.relationship.clone(),
                    source_path: source.path.clone(),
                    raw_sha256: source.raw_sha256.clone(),
                    normalized_sha256: source.normalized_sha256.clone(),
                    selection_rule,
                    catalogue_classification: catalogue_record
                        .map(|record| record.classification.clone())
                        .unwrap_or_else(|| "source_only".to_owned()),
                    selection_category: if infrastructure.contains(&group.normalized_name) {
                        "selected_infrastructure_unit".to_owned()
                    } else {
                        "selected_numerical_program_unit".to_owned()
                    },
                    relationship_claim_ids: vec![group.relationship_id.clone()],
                    provider_relationship_classification: group.relationship_classification.clone(),
                    review_state,
                });
            }
            None => {
                let disposition = catalogue_record
                    .map(|record| record.classification.as_str())
                    .unwrap_or("source_only");
                if disposition == "catalogue_heading" || disposition == "historical_or_obsolete" {
                    continue;
                }
                if group
                    .provider_ids
                    .iter()
                    .filter_map(|id| source_by_provider.get(id))
                    .all(|source| tool_subsets.contains(&source.subset))
                {
                    continue;
                }
                unresolved.push(json!({
                    "id": stable_id("unresolved-provider", &[&group.normalized_name]),
                    "normalized_name": group.normalized_name,
                    "provider_ids": group.provider_ids,
                    "reason": "no policy-approved provider",
                    "review_state": "review_requested",
                }));
            }
        }
    }

    selected.sort_by(|left, right| left.normalized_name.cmp(&right.normalized_name));
    let mut excluded = Vec::new();
    for group in &groups {
        for provider_id in &group.provider_ids {
            if selected_provider_ids.contains(provider_id) {
                continue;
            }
            let source = source_by_provider.get(provider_id).ok_or_else(|| {
                CorpusError::Verification(format!(
                    "excluded provider {provider_id} has no source-file record"
                ))
            })?;
            let (disposition, detail) = excluded_disposition(
                &group.normalized_name,
                source,
                &selected,
                &comparison_only,
                &tool_subsets,
                evidence_dir,
                &main.snapshot_id,
            )?;
            excluded.push(json!({
                "id": stable_id("excluded-provider", &[provider_id, &disposition]),
                "program_unit_id": provider_id,
                "normalized_name": group.normalized_name,
                "source_subset": source.subset,
                "source_path": source.path,
                "raw_sha256": source.raw_sha256,
                "normalized_sha256": source.normalized_sha256,
                "disposition": disposition,
                "detail": detail,
                "relationship_claim_ids": [group.relationship_id],
                "review_state": "machine_checked",
            }));
        }
    }
    excluded.sort_by(|left, right| left["id"].as_str().cmp(&right["id"].as_str()));

    let mut catalogue_values = Vec::new();
    let mut catalogue_only_dispositions = BTreeMap::<String, usize>::new();
    for (name, record) in &catalogue {
        let group = group_by_name.get(name);
        let entries = entries_by_name.get(name).cloned().unwrap_or_default();
        let disposition = catalogue_disposition(record, group.is_some(), !entries.is_empty());
        if group.is_none() {
            *catalogue_only_dispositions
                .entry(disposition.to_owned())
                .or_insert(0) += 1;
        }
        catalogue_values.push(json!({
            "normalized_name": record.normalized_name,
            "current_list_present": record.in_current_list,
            "catalogue_sources": {
                "current_list_line_numbers": record.list_lines,
                "version_4_1_toc_line_numbers": record.toc_lines,
            },
            "classification": record.classification,
            "classification_basis": record.classification_basis,
            "source_locator_ids": entries.iter().map(entry_locator_id).collect::<Vec<_>>(),
            "provider_ids": group.map(|value| value.provider_ids.clone()).unwrap_or_default(),
            "catalogue_only_disposition": disposition,
            "review_state": if record.classification == "historical_or_obsolete" { "review_requested" } else { "machine_checked" },
            "diagnostic_ids": Vec::<String>::new(),
        }));
    }
    for group in &groups {
        if catalogue.contains_key(&group.normalized_name) {
            continue;
        }
        let source_classification = source_only_classification(
            group,
            &source_by_provider,
            &supplemental_numerical,
            &tool_subsets,
        );
        catalogue_values.push(json!({
            "normalized_name": group.normalized_name,
            "catalogue_sources": {"current_list_line_numbers": Vec::<u64>::new(), "version_4_1_toc_line_numbers": Vec::<u64>::new()},
            "classification": "source_only",
            "classification_basis": source_classification,
            "source_locator_ids": entries_by_name.get(&group.normalized_name).map(|values| values.iter().map(entry_locator_id).collect::<Vec<_>>()).unwrap_or_default(),
            "provider_ids": group.provider_ids,
            "catalogue_only_disposition": "not_applicable",
            "review_state": if source_classification == "unexpected_numerical_source" { "review_requested" } else { "machine_checked" },
            "diagnostic_ids": Vec::<String>::new(),
        }));
    }
    catalogue_values.sort_by(|left, right| {
        left["normalized_name"]
            .as_str()
            .cmp(&right["normalized_name"].as_str())
    });

    let historical_count = catalogue_values
        .iter()
        .filter(|value| value["catalogue_only_disposition"] == "historical_or_removed")
        .count();
    let heading_count = catalogue_values
        .iter()
        .filter(|value| value["catalogue_only_disposition"] == "catalogue_false_positive")
        .count();
    diagnostics.push(diagnostic(
        "info",
        "COMPLETE-SELECTION-CATALOGUE-HISTORICAL-OR-OBSOLETE",
        &historical_count.to_string(),
        "Version 4.1 user candidates absent from the current list and all audited hosted sources",
    ));
    diagnostics.push(diagnostic(
        "info",
        "COMPLETE-SELECTION-CATALOGUE-HEADING-FALSE-POSITIVE",
        &heading_count.to_string(),
        "legacy generic TOC tokenization promoted headings or category cells",
    ));
    diagnostics.push(diagnostic(
        "info",
        "COMPLETE-SELECTION-HISTORICAL-SGEIR-RESOLVED",
        "SGEIR",
        "current immutable main-src provider selected; one historical executable-line variant excluded",
    ));
    if selected
        .iter()
        .any(|provider| provider.source_subset == "spfun")
    {
        diagnostics.push(diagnostic(
            "warning",
            "COMPLETE-SELECTION-SPFUN-SOURCE-ONLY-NUMERICAL",
            "spfun",
            "FNLIB-prologue providers are selected despite catalogue absence; retain review scope",
        ));
    }
    if !unresolved.is_empty() {
        diagnostics.push(diagnostic(
            "error",
            "COMPLETE-SELECTION-UNRESOLVED-PROVIDER",
            &unresolved.len().to_string(),
            "an unresolved provider prevents authoritative complete-corpus selection",
        ));
    }

    validate_expectations(&selected, &catalogue_values)?;
    let selected_file_ids: BTreeSet<_> = selected
        .iter()
        .filter_map(|provider| source_by_provider.get(&provider.program_unit_id))
        .map(|source| source.id.clone())
        .collect();
    let selected_files: Vec<_> = sources
        .iter()
        .filter(|source| selected_file_ids.contains(&source.id))
        .collect();
    let coverage = json!({
        "selected_source_files": selected_files.len(),
        "selected_files_with_zero_detected_units": selected_files.iter().filter(|source| source.provider_ids.is_empty()).count(),
        "selected_files_with_multiple_units": selected_files.iter().filter(|source| source.provider_ids.len() > 1).count(),
        "selected_files_with_scanner_diagnostics": selected_files.iter().filter(|source| !source.scan_diagnostic_rules.is_empty()).count(),
        "entry_declarations_in_all_audited_sources": entries_by_name.values().map(Vec::len).sum::<usize>(),
    });

    let catalogue_hashes = catalogue_hashes(&full_corpus_dir.join("artifact-index.json"))?;
    let snapshot_id = complete_snapshot_id(
        &main,
        &audit,
        &config_bytes,
        &catalogue_hashes,
        &sources,
        &selected,
    );
    let stage_status = if unresolved.is_empty() {
        "success_with_review_items"
    } else {
        "failed"
    };
    let selected_values = selected
        .iter()
        .map(|provider| provider.value(&snapshot_id))
        .collect::<Vec<_>>();
    let mut outputs = BTreeMap::new();
    outputs.insert(
        "selected-providers.json",
        compact(&json!({
            "schema_id": "slatec-rs/complete-selected-provider-set",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "records": selected_values,
        }))?,
    );
    outputs.insert(
        "excluded-providers.json",
        compact(&json!({
            "schema_id": "slatec-rs/complete-excluded-provider-set",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "records": excluded,
        }))?,
    );
    outputs.insert(
        "unresolved-providers.json",
        compact(&json!({
            "schema_id": "slatec-rs/complete-unresolved-provider-set",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "records": unresolved,
        }))?,
    );
    outputs.insert(
        "catalogue-classification.json",
        compact(&json!({
            "schema_id": "slatec-rs/complete-corpus-catalogue-classification",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "records": catalogue_values,
        }))?,
    );
    outputs.insert(
        "diagnostics.json",
        compact(&json!({
            "schema_id": "slatec-rs/complete-corpus-selection-diagnostics",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "stage_status": stage_status,
            "records": diagnostics,
        }))?,
    );
    let committed_size: u64 = outputs.values().map(|bytes| bytes.len() as u64).sum();
    if committed_size > COMMITTED_SIZE_LIMIT {
        return Err(CorpusError::Verification(format!(
            "complete selected-corpus output would be {committed_size} bytes; redesign compact records"
        )));
    }
    let semantic_hash = semantic_hash(&outputs);
    let output_hashes: BTreeMap<_, _> = outputs
        .iter()
        .map(|(name, bytes)| ((*name).to_owned(), hash::bytes(bytes)))
        .collect();
    let selected_numerical = selected
        .iter()
        .filter(|provider| provider.selection_category == "selected_numerical_program_unit")
        .count();
    let selected_infrastructure = selected.len() - selected_numerical;
    outputs.insert(
        "manifest.json",
        compact(&json!({
            "id": stable_id("complete-selected-corpus", &[&snapshot_id, &semantic_hash]),
            "schema_id": "slatec-rs/complete-selected-corpus",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot_id,
            "main_src_snapshot_id": main.snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "selection_semantic_version": SELECTION_SEMANTIC_VERSION,
            "main_src_artifact_hash": main.sha256,
            "full_corpus_audit_hash": audit.output_semantic_hash,
            "full_corpus_audit_input_hash": audit.audit_input_hash,
            "full_corpus_audit_configuration_hash": audit.audit_configuration_hash,
            "directory_index_hashes": audit.directory_index_hashes,
            "catalogue_hashes": catalogue_hashes,
            "selection_configuration_hash": hash::bytes(&config_bytes),
            "output_semantic_hash": semantic_hash,
            "output_file_hashes": output_hashes,
            "committed_output_total_bytes": committed_size,
            "stage_status": stage_status,
            "coverage": coverage,
            "summary": {
                "selected_program_units": selected.len(),
                "selected_numerical_program_units": selected_numerical,
                "selected_infrastructure_program_units": selected_infrastructure,
                "unresolved_provider_count": unresolved.len(),
                "catalogue_historical_or_removed": historical_count,
                "catalogue_heading_false_positives": heading_count,
            },
        }))?,
    );
    outputs.insert(
        "coverage-summary.md",
        format!(
            "# Complete SLATEC-hosted selected corpus\n\n- Snapshot: `{snapshot_id}`\n- Semantic hash: `{semantic_hash}`\n- Stage status: `{stage_status}`\n- Selected program units: {} (numerical: {selected_numerical}; infrastructure: {selected_infrastructure})\n- Unresolved providers: {}\n- Version 4.1 historical or removed catalogue identities: {historical_count}\n- Rejected legacy TOC heading/category false positives: {heading_count}\n- Committed output bytes: {committed_size}\n\nThe immutable `main-src` snapshot remains separately identified. Raw source and catalogue evidence remain ignored; standalone upstream packages are not selected.\n",
            selected.len(),
            unresolved.len(),
        )
        .into_bytes(),
    );
    promote(output_dir, &snapshot_id, &outputs)?;
    Ok(CompleteSelectionResult {
        snapshot_id,
        status: stage_status.to_owned(),
        semantic_hash,
        output_dir: output_dir.to_owned(),
        selected_program_units: selected.len(),
        unresolved_provider_count: unresolved.len(),
    })
}

#[derive(Clone, Debug)]
struct SelectedProvider {
    program_unit_id: String,
    normalized_name: String,
    kind: String,
    source_subset: String,
    source_relationship: String,
    source_path: String,
    raw_sha256: String,
    normalized_sha256: String,
    selection_rule: String,
    catalogue_classification: String,
    selection_category: String,
    relationship_claim_ids: Vec<String>,
    provider_relationship_classification: String,
    review_state: String,
}

impl SelectedProvider {
    fn value(&self, snapshot_id: &str) -> Value {
        json!({
            "id": stable_id("complete-selected-provider", &[snapshot_id, &self.program_unit_id]),
            "schema_id": "slatec-rs/complete-selected-provider",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot_id,
            "program_unit_id": self.program_unit_id,
            "normalized_name": self.normalized_name,
            "kind": self.kind,
            "selected_provider_id": self.program_unit_id,
            "source_subset": self.source_subset,
            "source_relationship": self.source_relationship,
            "source_path": self.source_path,
            "raw_sha256": self.raw_sha256,
            "normalized_sha256": self.normalized_sha256,
            "selection_rule": self.selection_rule,
            "catalogue_classification": self.catalogue_classification,
            "selection_category": self.selection_category,
            "relationship_claim_ids": self.relationship_claim_ids,
            "provider_relationship_classification": self.provider_relationship_classification,
            "review_state": self.review_state,
        })
    }
}

fn source_files(path: &Path) -> Result<Vec<SourceFile>> {
    let rows = columnar_rows(
        path,
        &[
            "id",
            "subset",
            "relationship",
            "path",
            "raw_sha256",
            "normalized_sha256",
        ],
    )?;
    rows.into_iter()
        .map(|row| {
            Ok(SourceFile {
                id: string_at(&row, 0)?,
                subset: string_at(&row, 1)?,
                relationship: string_at(&row, 2)?,
                path: string_at(&row, 3)?,
                raw_sha256: string_at(&row, 4)?,
                normalized_sha256: string_at(&row, 5)?,
                provider_ids: string_array_at(&row, 8)?,
                scan_diagnostic_rules: string_array_at(&row, 9)?,
            })
        })
        .collect()
}

fn provider_groups(union_path: &Path, relationships_path: &Path) -> Result<Vec<ProviderGroup>> {
    let relationship_rows = columnar_rows(
        relationships_path,
        &["id", "normalized_name", "classification", "provider_ids"],
    )?;
    let relationships: BTreeMap<_, _> = relationship_rows
        .into_iter()
        .map(|row| {
            Ok((
                string_at(&row, 1)?,
                (string_at(&row, 0)?, string_at(&row, 2)?),
            ))
        })
        .collect::<Result<_>>()?;
    columnar_rows(
        union_path,
        &[
            "normalized_name",
            "provider_ids",
            "provider_count",
            "kinds",
            "relationship_classification",
        ],
    )?
    .into_iter()
    .map(|row| {
        let name = string_at(&row, 0)?;
        let (relationship_id, relationship_classification) =
            relationships.get(&name).cloned().ok_or_else(|| {
                CorpusError::Verification(format!(
                    "union provider {name} has no relationship record"
                ))
            })?;
        Ok(ProviderGroup {
            normalized_name: name,
            provider_ids: string_array_at(&row, 1)?,
            kinds: string_array_at(&row, 3)?,
            relationship_classification,
            relationship_id,
        })
    })
    .collect()
}

fn catalogue_records(path: &Path) -> Result<BTreeMap<String, CatalogueRecord>> {
    columnar_rows(
        path,
        &[
            "normalized_name",
            "in_current_list",
            "in_version_4_1_toc",
            "classification",
        ],
    )?
    .into_iter()
    .map(|row| {
        let name = string_at(&row, 0)?;
        Ok((
            name.clone(),
            CatalogueRecord {
                normalized_name: name,
                in_current_list: bool_at(&row, 1)?,
                classification: string_at(&row, 3)?,
                classification_basis: string_at(&row, 5)?,
                list_lines: number_array_at(&row, 6)?,
                toc_lines: number_array_at(&row, 7)?,
            },
        ))
    })
    .collect()
}

fn select_provider(
    group: &ProviderGroup,
    source_by_provider: &BTreeMap<String, SourceFile>,
    source_only: bool,
    relocated: &BTreeSet<String>,
    supplemental_numerical: &BTreeSet<String>,
    tool_subsets: &BTreeSet<String>,
) -> Result<Option<(String, String, String)>> {
    let mut sources = group
        .provider_ids
        .iter()
        .filter_map(|id| {
            source_by_provider
                .get(id)
                .map(|source| (id.clone(), source))
        })
        .collect::<Vec<_>>();
    sources.sort_by(|left, right| {
        (&left.1.subset, &left.1.path, &left.0).cmp(&(&right.1.subset, &right.1.path, &right.0))
    });
    if sources.is_empty() {
        return Err(CorpusError::Verification(format!(
            "provider group {} has no source files",
            group.normalized_name
        )));
    }
    if matches!(
        group.relationship_classification.as_str(),
        "modified_relocated_copy" | "duplicate_provider" | "alternate_implementation"
    ) {
        return Ok(None);
    }
    if source_only {
        if let Some((id, _)) = sources
            .iter()
            .find(|(_, source)| supplemental_numerical.contains(&source.subset))
        {
            return Ok(Some((
                id.clone(),
                "explicit-supplemental-fnlib-source-provider".to_owned(),
                "review_requested".to_owned(),
            )));
        }
        if sources
            .iter()
            .all(|(_, source)| tool_subsets.contains(&source.subset))
        {
            return Ok(None);
        }
        return Ok(None);
    }
    if let Some((id, _)) = sources
        .iter()
        .find(|(_, source)| source.subset == "main-src")
    {
        return Ok(Some((
            id.clone(),
            "immutable-main-src-precedence".to_owned(),
            "machine_checked".to_owned(),
        )));
    }
    if let Some((id, source)) = sources
        .iter()
        .find(|(_, source)| relocated.contains(&source.subset))
    {
        return Ok(Some((
            id.clone(),
            format!(
                "approved-relocated-slatec-hosted-provider:{}",
                source.subset
            ),
            "machine_checked".to_owned(),
        )));
    }
    Ok(None)
}

fn source_only_classification(
    group: &ProviderGroup,
    source_by_provider: &BTreeMap<String, SourceFile>,
    supplemental_numerical: &BTreeSet<String>,
    tool_subsets: &BTreeSet<String>,
) -> &'static str {
    if group.provider_ids.iter().any(|id| {
        source_by_provider
            .get(id)
            .is_some_and(|source| supplemental_numerical.contains(&source.subset))
    }) {
        "unexpected_numerical_source"
    } else if group.provider_ids.iter().all(|id| {
        source_by_provider
            .get(id)
            .is_some_and(|source| tool_subsets.contains(&source.subset))
    }) {
        "tool_or_test"
    } else {
        "unresolved"
    }
}

fn catalogue_disposition(
    record: &CatalogueRecord,
    has_provider: bool,
    has_entry: bool,
) -> &'static str {
    if has_provider {
        "located_after_reconciliation"
    } else if has_entry {
        "alias_or_entry"
    } else {
        match record.classification.as_str() {
            "catalogue_heading" => "catalogue_false_positive",
            "historical_or_obsolete" => "historical_or_removed",
            _ => "missing_source",
        }
    }
}

fn verified_source_bytes(
    evidence_dir: &Path,
    snapshot: &str,
    source: &SourceFile,
) -> Result<Vec<u8>> {
    let path = source_path(evidence_dir, snapshot, source)?;
    let bytes = fs::read(&path).map_err(|_| {
        CorpusError::Verification(format!(
            "missing audited source evidence {}",
            path.display()
        ))
    })?;
    if hash::bytes(&bytes) != source.raw_sha256 {
        return Err(CorpusError::Verification(format!(
            "audited source hash mismatch for {}:{}",
            source.subset, source.path
        )));
    }
    Ok(bytes)
}

fn source_path(evidence_dir: &Path, snapshot: &str, source: &SourceFile) -> Result<PathBuf> {
    match source.subset.as_str() {
        "main-src" | "main-src-historical" => Ok(evidence_dir
            .join("extracted")
            .join(snapshot)
            .join("slatec-source-archive")
            .join(&source.path)),
        "src" | "lin" | "fishfft" | "fnlib" | "pchip" | "err" => Ok(evidence_dir
            .join("full-corpus/audit-input/directories")
            .join(&source.subset)
            .join("files")
            .join(&source.path)),
        "spfun" | "subsid" | "slprep" | "sladoc" => Ok(evidence_dir
            .join("full-corpus/audit-input/supplemental")
            .join(&source.subset)),
        _ => Err(CorpusError::Verification(format!(
            "no evidence path rule for audited subset {}",
            source.subset
        ))),
    }
}

fn entry_names(source: &SourceFile, bytes: &[u8]) -> Vec<(String, EntryEvidence)> {
    let physical = fixed_form::physical_lines(bytes);
    fixed_form::logical_statements(&physical)
        .into_iter()
        .filter_map(|statement| {
            let text = statement.normalized_statement_text.trim_start();
            let tail = text.strip_prefix("ENTRY").or_else(|| {
                text.get(..5)
                    .filter(|prefix| prefix.eq_ignore_ascii_case("ENTRY"))
                    .and_then(|_| text.get(5..))
            })?;
            if !tail.starts_with(char::is_whitespace) {
                return None;
            }
            let name = tail
                .trim_start()
                .chars()
                .take_while(|character| character.is_ascii_alphanumeric() || *character == '$')
                .collect::<String>();
            (!name.is_empty()).then(|| {
                (
                    name.to_ascii_uppercase(),
                    EntryEvidence {
                        source_file_id: source.id.clone(),
                        line: statement.physical_line_start,
                        raw_byte_start: statement.raw_spans[0].start,
                    },
                )
            })
        })
        .collect()
}

fn entry_locator_id(entry: &EntryEvidence) -> String {
    stable_id(
        "entry-locator",
        &[
            &entry.source_file_id,
            &entry.line.to_string(),
            &entry.raw_byte_start.to_string(),
        ],
    )
}

fn spfun_has_fnlib_prologues(evidence_dir: &Path) -> bool {
    let path = evidence_dir.join("full-corpus/audit-input/supplemental/spfun");
    fs::read(path).is_ok_and(|bytes| has_fnlib_prologues(&bytes))
}

fn has_fnlib_prologues(bytes: &[u8]) -> bool {
    let text = String::from_utf8_lossy(bytes).to_ascii_uppercase();
    text.contains("C***LIBRARY   FNLIB") && text.contains("C***BEGIN PROLOGUE")
}

fn excluded_disposition(
    name: &str,
    source: &SourceFile,
    selected: &[SelectedProvider],
    comparison_only: &BTreeSet<String>,
    tool_subsets: &BTreeSet<String>,
    evidence_dir: &Path,
    snapshot: &str,
) -> Result<(String, Value)> {
    if source.subset == "main-src-historical" && name == "SGEIR" {
        return Ok((
            "historical_variant_excluded_by_current_main_src_precedence".to_owned(),
            sgeir_difference_summary(evidence_dir, snapshot)?,
        ));
    }
    if tool_subsets.contains(&source.subset) {
        return Ok((
            "documentation_tool_or_test_source_excluded".to_owned(),
            json!({"source_classification":"tool_or_test"}),
        ));
    }
    if comparison_only.contains(&source.subset) {
        let selected_same_name = selected
            .iter()
            .find(|provider| provider.normalized_name == name);
        return Ok((
            "redundant_comparison_evidence_excluded".to_owned(),
            json!({
                "selected_provider_id": selected_same_name.map(|provider| provider.program_unit_id.clone()),
                "raw_hash_matches_selected": selected_same_name.is_some_and(|provider| provider.raw_sha256 == source.raw_sha256),
            }),
        ));
    }
    Ok((
        "not-selected-by-complete-corpus-policy".to_owned(),
        json!({"source_classification":"unresolved"}),
    ))
}

fn sgeir_difference_summary(evidence_dir: &Path, snapshot: &str) -> Result<Value> {
    let root = evidence_dir
        .join("extracted")
        .join(snapshot)
        .join("slatec-source-archive/src");
    let current = fs::read(root.join("sgeir.f"))?;
    let historical = fs::read(root.join("sgeir.f.0"))?;
    let current_lines = normalized_lines(&current);
    let historical_lines = normalized_lines(&historical);
    let mut comment_or_blank = 0_usize;
    let mut statement_or_declaration = 0_usize;
    let mut changed_lines = Vec::new();
    for index in 0..current_lines.len().max(historical_lines.len()) {
        let before = historical_lines
            .get(index)
            .map(String::as_str)
            .unwrap_or("");
        let after = current_lines.get(index).map(String::as_str).unwrap_or("");
        if before == after {
            continue;
        }
        changed_lines.push(index + 1);
        if is_comment_or_blank(before) && is_comment_or_blank(after) {
            comment_or_blank += 1;
        } else {
            statement_or_declaration += 1;
        }
    }
    Ok(json!({
        "resolution": "current-main-src-selected-over-historical-variant",
        "historical_raw_sha256": hash::bytes(&historical),
        "current_raw_sha256": hash::bytes(&current),
        "changed_physical_line_count": changed_lines.len(),
        "comment_or_blank_change_count": comment_or_blank,
        "statement_or_declaration_change_count": statement_or_declaration,
        "changed_line_numbers": changed_lines,
        "difference_classification": if statement_or_declaration > 0 { "executable_logic" } else { "comment_or_formatting" },
    }))
}

fn normalized_lines(bytes: &[u8]) -> Vec<String> {
    std::str::from_utf8(bytes)
        .unwrap_or("")
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .split('\n')
        .map(ToOwned::to_owned)
        .collect()
}

fn is_comment_or_blank(line: &str) -> bool {
    line.is_empty()
        || line
            .as_bytes()
            .first()
            .is_some_and(|byte| matches!(byte, b'C' | b'c' | b'*' | b'!'))
}

fn catalogue_hashes(path: &Path) -> Result<BTreeMap<String, String>> {
    let value: Value = read_json(path)?;
    let mut output = BTreeMap::new();
    for record in value["records"].as_array().ok_or_else(|| {
        CorpusError::Verification("full-corpus artifact index records are not an array".to_owned())
    })? {
        let id = record["id"].as_str().unwrap_or("");
        if matches!(id, "catalogue-list" | "catalogue-toc") {
            output.insert(
                id.to_owned(),
                record["sha256"]
                    .as_str()
                    .ok_or_else(|| {
                        CorpusError::Verification("catalogue hash is absent".to_owned())
                    })?
                    .to_owned(),
            );
        }
    }
    if output.len() != 2 {
        return Err(CorpusError::Verification(
            "full-corpus artifact index does not contain both catalogue hashes".to_owned(),
        ));
    }
    Ok(output)
}

fn complete_snapshot_id(
    main: &MainArtifactManifest,
    audit: &FullAuditManifest,
    config_bytes: &[u8],
    catalogue_hashes: &BTreeMap<String, String>,
    sources: &[SourceFile],
    selected: &[SelectedProvider],
) -> String {
    let mut material = Vec::new();
    for value in [
        &main.snapshot_id,
        &main.sha256,
        &audit.audit_input_hash,
        &audit.audit_configuration_hash,
        &hash::bytes(config_bytes),
    ] {
        material.extend_from_slice(value.as_bytes());
        material.push(0);
    }
    for (name, value) in catalogue_hashes {
        material.extend_from_slice(name.as_bytes());
        material.push(0);
        material.extend_from_slice(value.as_bytes());
        material.push(0);
    }
    for source in sources {
        material.extend_from_slice(source.subset.as_bytes());
        material.push(0);
        material.extend_from_slice(source.path.as_bytes());
        material.push(0);
        material.extend_from_slice(source.raw_sha256.as_bytes());
        material.push(0);
    }
    for provider in selected {
        material.extend_from_slice(provider.program_unit_id.as_bytes());
        material.push(0);
        material.extend_from_slice(provider.selection_rule.as_bytes());
        material.push(0);
    }
    format!("complete-slatec-{}", &hash::bytes(&material)[..20])
}

fn validate_expectations(selected: &[SelectedProvider], catalogue: &[Value]) -> Result<()> {
    #[derive(Deserialize)]
    struct ExpectationFile {
        expectations: Vec<Expectation>,
    }
    #[derive(Deserialize)]
    struct Expectation {
        normalized_name: String,
        expected_selected_subset: Option<String>,
        expected_catalogue_classification: String,
    }
    let path = Path::new("crates/slatec-tools/tests/data/full-corpus-expectations.toml");
    let values: ExpectationFile = toml::from_str(&fs::read_to_string(path)?)?;
    for expected in values.expectations {
        let actual_catalogue = catalogue
            .iter()
            .find(|record| record["normalized_name"] == expected.normalized_name)
            .and_then(|record| record["classification"].as_str())
            .ok_or_else(|| {
                CorpusError::Verification(format!(
                    "reviewed expectation {} is absent from catalogue classification",
                    expected.normalized_name
                ))
            })?;
        if actual_catalogue != expected.expected_catalogue_classification {
            return Err(CorpusError::Verification(format!(
                "reviewed expectation {} catalogue classification is {}, expected {}",
                expected.normalized_name,
                actual_catalogue,
                expected.expected_catalogue_classification
            )));
        }
        if let Some(subset) = expected.expected_selected_subset {
            let actual = selected
                .iter()
                .find(|provider| provider.normalized_name == expected.normalized_name)
                .ok_or_else(|| {
                    CorpusError::Verification(format!(
                        "reviewed expectation {} has no selected provider",
                        expected.normalized_name
                    ))
                })?;
            if actual.source_subset != subset {
                return Err(CorpusError::Verification(format!(
                    "reviewed expectation {} selected {} instead of {}",
                    expected.normalized_name, actual.source_subset, subset
                )));
            }
        }
    }
    Ok(())
}

fn diagnostic(severity: &str, rule: &str, identity: &str, message: &str) -> Value {
    json!({
        "id": stable_id("complete-selection-diagnostic", &[severity, rule, identity]),
        "severity": severity,
        "rule_id": rule,
        "identity": identity,
        "message": message,
        "review_state": "review_requested",
    })
}

fn columnar_rows(path: &Path, expected_prefix: &[&str]) -> Result<Vec<Vec<Value>>> {
    let value: Value = read_json(path)?;
    let columns = value["columns"].as_array().ok_or_else(|| {
        CorpusError::Verification(format!("{} is not a columnar record set", path.display()))
    })?;
    for (index, expected) in expected_prefix.iter().enumerate() {
        if columns.get(index).and_then(Value::as_str) != Some(*expected) {
            return Err(CorpusError::Verification(format!(
                "{} column {index} is not {expected}",
                path.display()
            )));
        }
    }
    value["records"]
        .as_array()
        .ok_or_else(|| {
            CorpusError::Verification(format!("{} records are not an array", path.display()))
        })?
        .iter()
        .map(|value| {
            value.as_array().cloned().ok_or_else(|| {
                CorpusError::Verification(format!("{} contains a non-array record", path.display()))
            })
        })
        .collect()
}

fn string_at(row: &[Value], index: usize) -> Result<String> {
    row.get(index)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("column {index} is not a string")))
}

fn bool_at(row: &[Value], index: usize) -> Result<bool> {
    row.get(index)
        .and_then(Value::as_bool)
        .ok_or_else(|| CorpusError::Verification(format!("column {index} is not a boolean")))
}

fn string_array_at(row: &[Value], index: usize) -> Result<Vec<String>> {
    row.get(index)
        .and_then(Value::as_array)
        .ok_or_else(|| CorpusError::Verification(format!("column {index} is not an array")))?
        .iter()
        .map(|value| {
            value.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                CorpusError::Verification(format!("column {index} contains a non-string"))
            })
        })
        .collect()
}

fn number_array_at(row: &[Value], index: usize) -> Result<Vec<u64>> {
    row.get(index)
        .and_then(Value::as_array)
        .ok_or_else(|| CorpusError::Verification(format!("column {index} is not an array")))?
        .iter()
        .map(|value| {
            value.as_u64().ok_or_else(|| {
                CorpusError::Verification(format!("column {index} contains a non-number"))
            })
        })
        .collect()
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
        CorpusError::Policy("selected-corpus output directory must have a parent".to_owned())
    })?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        "{}.staging-{snapshot}",
        output_dir.file_name().unwrap_or_default().to_string_lossy()
    ));
    if staging.exists() {
        return Err(CorpusError::Verification(format!(
            "selected-corpus staging directory exists: {}",
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
                "selected-corpus backup directory exists: {}",
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

    fn source(id: &str, subset: &str) -> SourceFile {
        SourceFile {
            id: id.to_owned(),
            subset: subset.to_owned(),
            relationship: "fixture".to_owned(),
            path: format!("{id}.f"),
            raw_sha256: "a".repeat(64),
            normalized_sha256: "a".repeat(64),
            provider_ids: vec![id.to_owned()],
            scan_diagnostic_rules: Vec::new(),
        }
    }

    #[test]
    fn source_only_numeric_and_tool_provider_rules_are_explicit() {
        let group = ProviderGroup {
            normalized_name: "ALPHA".to_owned(),
            provider_ids: vec!["alpha".to_owned()],
            kinds: vec!["function".to_owned()],
            relationship_classification: "unique_provider".to_owned(),
            relationship_id: "relationship-alpha".to_owned(),
        };
        let mut providers = BTreeMap::new();
        providers.insert("alpha".to_owned(), source("alpha", "spfun"));
        assert_eq!(
            select_provider(
                &group,
                &providers,
                true,
                &BTreeSet::new(),
                &BTreeSet::from(["spfun".to_owned()]),
                &BTreeSet::new(),
            )
            .unwrap()
            .unwrap()
            .1,
            "explicit-supplemental-fnlib-source-provider"
        );
        providers.insert("alpha".to_owned(), source("alpha", "sladoc"));
        assert!(
            select_provider(
                &group,
                &providers,
                true,
                &BTreeSet::new(),
                &BTreeSet::new(),
                &BTreeSet::from(["sladoc".to_owned()]),
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    fn catalogue_dispositions_do_not_hide_headings_or_historical_names() {
        let heading = CatalogueRecord {
            normalized_name: "A6B".to_owned(),
            in_current_list: false,
            classification: "catalogue_heading".to_owned(),
            classification_basis: "fixture".to_owned(),
            list_lines: Vec::new(),
            toc_lines: vec![1],
        };
        let historical = CatalogueRecord {
            normalized_name: "CSBMV".to_owned(),
            classification: "historical_or_obsolete".to_owned(),
            ..heading.clone()
        };
        assert_eq!(
            catalogue_disposition(&heading, false, false),
            "catalogue_false_positive"
        );
        assert_eq!(
            catalogue_disposition(&historical, false, false),
            "historical_or_removed"
        );
        assert_eq!(
            catalogue_disposition(&historical, false, true),
            "alias_or_entry"
        );
    }

    #[test]
    fn modified_duplicate_provider_blocks_authoritative_selection() {
        let group = ProviderGroup {
            normalized_name: "ALPHA".to_owned(),
            provider_ids: vec!["main".to_owned(), "lin".to_owned()],
            kinds: vec!["subroutine".to_owned()],
            relationship_classification: "modified_relocated_copy".to_owned(),
            relationship_id: "relationship-alpha".to_owned(),
        };
        let providers = BTreeMap::from([
            ("main".to_owned(), source("main", "main-src")),
            ("lin".to_owned(), source("lin", "lin")),
        ]);
        assert!(
            select_provider(
                &group,
                &providers,
                false,
                &BTreeSet::from(["lin".to_owned()]),
                &BTreeSet::new(),
                &BTreeSet::new(),
            )
            .unwrap()
            .is_none()
        );
    }

    #[test]
    fn byte_identical_relocation_prefers_main_src_and_fnlib_marker_is_required() {
        let group = ProviderGroup {
            normalized_name: "ALPHA".to_owned(),
            provider_ids: vec!["main".to_owned(), "lin".to_owned()],
            kinds: vec!["subroutine".to_owned()],
            relationship_classification: "byte_identical_relocated_copy".to_owned(),
            relationship_id: "relationship-alpha".to_owned(),
        };
        let providers = BTreeMap::from([
            ("main".to_owned(), source("main", "main-src")),
            ("lin".to_owned(), source("lin", "lin")),
        ]);
        assert_eq!(
            select_provider(
                &group,
                &providers,
                false,
                &BTreeSet::from(["lin".to_owned()]),
                &BTreeSet::new(),
                &BTreeSet::new(),
            )
            .unwrap()
            .unwrap()
            .0,
            "main"
        );
        assert!(has_fnlib_prologues(
            b"C***BEGIN PROLOGUE\nC***LIBRARY   FNLIB\n"
        ));
        assert!(!has_fnlib_prologues(b"C***BEGIN PROLOGUE\n"));
    }

    #[test]
    fn complete_snapshot_is_independent_of_local_paths_and_order() {
        let main = MainArtifactManifest {
            snapshot_id: "main".to_owned(),
            sha256: "m".repeat(64),
        };
        let audit = FullAuditManifest {
            main_src_snapshot_id: "main".to_owned(),
            audit_input_hash: "i".repeat(64),
            audit_configuration_hash: "c".repeat(64),
            directory_index_hashes: BTreeMap::new(),
            output_semantic_hash: "o".repeat(64),
        };
        let hashes = BTreeMap::from([
            ("catalogue-list".to_owned(), "l".repeat(64)),
            ("catalogue-toc".to_owned(), "t".repeat(64)),
        ]);
        let selected = vec![SelectedProvider {
            program_unit_id: "provider".to_owned(),
            normalized_name: "ALPHA".to_owned(),
            kind: "subroutine".to_owned(),
            source_subset: "main-src".to_owned(),
            source_relationship: "fixture".to_owned(),
            source_path: "src/a.f".to_owned(),
            raw_sha256: "a".repeat(64),
            normalized_sha256: "a".repeat(64),
            selection_rule: "fixture".to_owned(),
            catalogue_classification: "user_callable".to_owned(),
            selection_category: "selected_numerical_program_unit".to_owned(),
            relationship_claim_ids: Vec::new(),
            provider_relationship_classification: "unique_provider".to_owned(),
            review_state: "machine_checked".to_owned(),
        }];
        let first = complete_snapshot_id(
            &main,
            &audit,
            b"config",
            &hashes,
            &[source("one", "main-src")],
            &selected,
        );
        let second = complete_snapshot_id(
            &main,
            &audit,
            b"config",
            &hashes,
            &[source("one", "main-src")],
            &selected,
        );
        assert_eq!(first, second);
    }
}
