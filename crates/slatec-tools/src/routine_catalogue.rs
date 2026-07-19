//! Complete-corpus routine catalogue and documentation renderer.
//!
//! This consumes the full-corpus comparison evidence instead of the selected
//! build closure.  It is deliberately offline and deterministic.

use crate::error::{CorpusError, Result};
use crate::hash;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const VERSION: &str = "1.1.0";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const NO_DESCRIPTION: &str = "Description unavailable from current source evidence.";

#[derive(Debug, serde::Serialize)]
pub struct CatalogueResult {
    pub identity_count: usize,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
}

#[derive(Clone)]
struct Provider {
    id: String,
    subset: String,
    relationship: String,
    path: String,
    raw_hash: String,
    normalized_hash: String,
}

#[derive(Clone)]
struct Identity {
    name: String,
    provider_ids: Vec<String>,
    kinds: Vec<String>,
    provider_group: String,
    in_list: bool,
    in_toc: bool,
    toc_role: String,
    list_lines: Vec<usize>,
    toc_lines: Vec<usize>,
    gams: Option<String>,
}

#[derive(Clone)]
struct Pilot {
    purpose: String,
    domain: Option<String>,
    package: Option<String>,
}

#[derive(Clone)]
struct TocPurpose {
    purpose: String,
    gams: Option<String>,
    role: String,
    group: Option<String>,
}

#[derive(Clone, Default)]
struct SourceDescription {
    purpose: Option<String>,
    full_description: Option<String>,
    field: Option<String>,
    dialect: String,
    was_normalized: bool,
}

#[derive(Clone, Default)]
struct DirectoryEntry {
    purpose: Option<String>,
    gams: Option<String>,
    source_url: Option<String>,
    fullsource_url: Option<String>,
    directory_url: Option<String>,
}

pub fn generate(
    full_corpus_dir: &Path,
    _program_unit_dir: &Path,
    ffi_dir: &Path,
    safe_api_dir: &Path,
    output_dir: &Path,
    docs_dir: &Path,
) -> Result<CatalogueResult> {
    let manifest = read_json(&full_corpus_dir.join("manifest.json"))?;
    let providers = read_providers(full_corpus_dir)?;
    let identities = read_identities(full_corpus_dir)?;
    let toc = toc_purposes()?;
    let directories = directory_entries()?;
    let snapshot = manifest
        .get("main_src_snapshot_id")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let source = source_descriptions(&providers, snapshot)?;
    let raw = raw_statuses(ffi_dir)?;
    let safe = safe_paths(safe_api_dir)?;
    let pilot = pilot_metadata()?;
    let mut records = identities
        .values()
        .map(|identity| {
            let mut identity = identity.clone();
            if let Some(toc) = toc.get(&identity.name) {
                identity.gams = toc.gams.clone();
            }
            build_record(
                &identity,
                &providers,
                toc.get(&identity.name),
                &directories,
                &source,
                raw.get(&identity.name),
                safe.get(&identity.name),
                pilot.get(&identity.name),
            )
        })
        .collect::<Result<Vec<_>>>()?;
    records.sort_by(|a, b| name(a).cmp(name(b)));
    validate_records(&records, &providers)?;

    let coverage = coverage(&records, &providers, &manifest);
    let description_coverage = description_coverage(&records);
    let discrepancies = description_discrepancies(&records);
    let external_references = external_reference_map(&records);
    let excluded = excluded_candidates(&records);
    let provider_map = provider_map(&records);
    let family_index = family_index(&records);
    let alphabetical_index = alphabetical_index(&records);
    let diagnostics = diagnostics(&records, &manifest);
    let catalogue = json!({
        "schema_id":"slatec-rs/routine-catalogue", "schema_version":VERSION,
        "created_by":format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at":CREATED_AT,
        "main_src_snapshot_id":manifest.get("main_src_snapshot_id"),
        "identity_model":"source_artifact -> source_file -> program_unit -> logical_identity -> provider",
        "canonical_provider_policy":"Prefer immutable main-src, then live slatec/src, then stable subset/path order. Retain every alternate or conflicting provider.",
        "description_policy":"Prefer complete verified source-prologue PURPOSE, then legacy source fields, Version 4.1 TOC, cached Netlib directory metadata, reviewed pilot metadata, then explicit unavailability. Full DESCRIPTION/ABSTRACT prose is retained separately from concise purposes.",
        "records":records,
    });
    let mut files = BTreeMap::new();
    files.insert("routine-catalogue.json", json_bytes(&catalogue)?);
    files.insert("provider-map.json", json_bytes(&provider_map)?);
    files.insert("family-index.json", json_bytes(&family_index)?);
    files.insert("alphabetical-index.json", json_bytes(&alphabetical_index)?);
    files.insert("coverage-summary.json", json_bytes(&coverage)?);
    files.insert(
        "description-coverage.json",
        json_bytes(&description_coverage)?,
    );
    files.insert(
        "description-discrepancies.json",
        json_bytes(&discrepancies)?,
    );
    files.insert(
        "external-reference-map.json",
        json_bytes(&external_references)?,
    );
    files.insert("excluded-candidates.json", json_bytes(&excluded)?);
    files.insert("reconciliation-diagnostics.json", json_bytes(&diagnostics)?);
    let semantic_hash = output_hash(&files);
    let file_hashes: BTreeMap<_, _> = files
        .iter()
        .map(|(name, bytes)| ((*name).to_owned(), hash::bytes(bytes)))
        .collect();
    files.insert("manifest.json", json_bytes(&json!({
        "schema_id":"slatec-rs/routine-catalogue-manifest", "schema_version":VERSION,
        "created_by":format!("{TOOL_NAME} {TOOL_VERSION}"), "created_at":CREATED_AT,
        "main_src_snapshot_id":manifest.get("main_src_snapshot_id"), "identity_count":records.len(),
        "offline_regeneration":true, "output_semantic_hash":semantic_hash,
        "output_file_hashes":file_hashes,
        "evidence_sources":["generated/full-corpus","generated/program-units","generated/ffi","generated/safe-api","metadata/routines-pilot.toml","cached Netlib Version 4.1 TOC"],
    }))?);
    write_jsons(output_dir, &files)?;
    write_docs(docs_dir, &records, &coverage, &diagnostics)?;
    validate_docs(docs_dir, &records)?;
    Ok(CatalogueResult {
        identity_count: records.len(),
        semantic_hash,
        output_dir: output_dir.to_owned(),
    })
}

fn read_providers(root: &Path) -> Result<BTreeMap<String, Provider>> {
    let value = read_json(&root.join("source-file-index.json"))?;
    let columns = columns(&value)?;
    let mut output = BTreeMap::new();
    for row in rows(&value)? {
        let subset = cell_string(row, &columns, "subset")?;
        let relationship = cell_string(row, &columns, "relationship")?;
        let path = cell_string(row, &columns, "path")?;
        let raw_hash = cell_string(row, &columns, "raw_sha256")?;
        let normalized_hash = cell_string(row, &columns, "normalized_sha256")?;
        for id in cell_strings(row, &columns, "program_unit_ids")? {
            output.insert(
                id.clone(),
                Provider {
                    id,
                    subset: subset.clone(),
                    relationship: relationship.clone(),
                    path: path.clone(),
                    raw_hash: raw_hash.clone(),
                    normalized_hash: normalized_hash.clone(),
                },
            );
        }
    }
    if output.is_empty() {
        return Err(policy("source-file index contains no provider references"));
    }
    Ok(output)
}

fn read_identities(root: &Path) -> Result<BTreeMap<String, Identity>> {
    let union = read_json(&root.join("program-unit-union.json"))?;
    let union_columns = columns(&union)?;
    let mut output = BTreeMap::new();
    for row in rows(&union)? {
        let name = cell_string(row, &union_columns, "normalized_name")?;
        output.insert(
            name.clone(),
            Identity {
                name,
                provider_ids: cell_strings(row, &union_columns, "provider_ids")?,
                kinds: cell_strings(row, &union_columns, "kinds")?,
                provider_group: cell_string(row, &union_columns, "relationship_classification")?,
                in_list: false,
                in_toc: false,
                toc_role: "unknown".to_owned(),
                list_lines: Vec::new(),
                toc_lines: Vec::new(),
                gams: None,
            },
        );
    }
    let reconciliation = read_json(&root.join("catalogue-reconciliation.json"))?;
    let columns = columns(&reconciliation)?;
    for row in rows(&reconciliation)? {
        let routine = cell_string(row, &columns, "normalized_name")?;
        let entry = output.entry(routine.clone()).or_insert_with(|| Identity {
            name: routine,
            provider_ids: Vec::new(),
            kinds: vec!["unknown".to_owned()],
            provider_group: "unresolved".to_owned(),
            in_list: false,
            in_toc: false,
            toc_role: "unknown".to_owned(),
            list_lines: Vec::new(),
            toc_lines: Vec::new(),
            gams: None,
        });
        entry.in_list = cell_bool(row, &columns, "in_current_list")?;
        entry.in_toc = cell_bool(row, &columns, "in_version_4_1_toc")?;
        entry.toc_role = cell_string(row, &columns, "classification")?;
        entry.list_lines = cell_usizes(row, &columns, "list_line_numbers")?;
        entry.toc_lines = cell_usizes(row, &columns, "toc_line_numbers")?;
    }
    if output.is_empty() {
        return Err(policy("full corpus contains no identities"));
    }
    Ok(output)
}

#[allow(clippy::too_many_arguments)]
fn build_record(
    identity: &Identity,
    provider_map: &BTreeMap<String, Provider>,
    toc: Option<&TocPurpose>,
    directories: &BTreeMap<(String, String), DirectoryEntry>,
    source: &BTreeMap<(String, String), SourceDescription>,
    raw: Option<&String>,
    safe: Option<&Vec<String>>,
    pilot: Option<&Pilot>,
) -> Result<Value> {
    let mut providers = identity
        .provider_ids
        .iter()
        .map(|id| {
            provider_map
                .get(id)
                .cloned()
                .ok_or_else(|| policy("provider reference does not resolve"))
        })
        .collect::<Result<Vec<_>>>()?;
    providers.sort_by(provider_order);
    let canonical = providers.first();
    let kind = identity
        .kinds
        .iter()
        .find(|kind| kind.as_str() != "unknown")
        .cloned()
        .unwrap_or_else(|| "unknown".to_owned());
    let catalogue_role = role(identity, &kind);
    let canonical_source = providers
        .iter()
        .find_map(|provider| source.get(&(provider.id.clone(), identity.name.clone())));
    let directory = providers
        .iter()
        .find_map(|provider| directories.get(&(provider.subset.clone(), provider.path.clone())));
    let toc_has_description = toc.is_some_and(|item| !item.purpose.is_empty());
    let (description, description_type, confidence, source_field, dialect, normalized) =
        if let Some(item) = canonical_source.and_then(|item| item.purpose.as_ref()) {
            (
                item.clone(),
                "canonical_source_prologue",
                "high",
                canonical_source
                    .and_then(|item| item.field.as_deref())
                    .unwrap_or("PURPOSE"),
                canonical_source
                    .map(|item| item.dialect.as_str())
                    .unwrap_or("unknown"),
                canonical_source.is_some_and(|item| item.was_normalized),
            )
        } else if toc_has_description {
            let item = toc.expect("checked above");
            (
                item.purpose.clone(),
                "netlib_slatec_toc",
                "high",
                "TOC Section I/II purpose",
                "netlib_toc",
                true,
            )
        } else if let Some(item) = directory.and_then(|item| item.purpose.as_ref()) {
            (
                item.clone(),
                "netlib_directory_index",
                "medium",
                "directory for",
                "netlib_directory",
                true,
            )
        } else if let Some(pilot) = pilot {
            (
                pilot.purpose.clone(),
                "pilot_metadata",
                "reviewed",
                "purpose",
                "reviewed_pilot",
                false,
            )
        } else {
            (
                NO_DESCRIPTION.to_owned(),
                "unavailable",
                "unavailable",
                "unavailable",
                "unavailable",
                false,
            )
        };
    let full_description = canonical_source
        .and_then(|item| item.full_description.as_ref())
        .cloned();
    let description_url = providers
        .iter()
        .find_map(|provider| directory_url(directories, provider));
    let source_variants = description_variants(canonical_source, toc, directory);
    let reference = official_references(&providers, directories, toc);
    let (family, domain, package) = classify(identity, &providers, pilot);
    let raw_binding_status = raw.cloned().unwrap_or_else(|| "not_bound".to_owned());
    let safe_api_paths = safe.cloned().unwrap_or_default();
    let safe_api_status = if safe_api_paths.is_empty() {
        "none"
    } else {
        "safe_public"
    };
    let audit_status = if !safe_api_paths.is_empty() {
        "deeply_audited"
    } else if raw_binding_status == "bound" {
        "family_inventory_only"
    } else {
        "identity_only"
    };
    let source_status = source_status(identity, &providers);
    let profile = if raw_binding_status == "bound" {
        "selected_by_profile"
    } else if providers
        .iter()
        .any(|provider| provider.subset == "main-src")
    {
        "available_but_unselected"
    } else if providers.is_empty() {
        "unknown"
    } else {
        "outside_current_immutable_snapshot"
    };
    let alternatives = providers
        .iter()
        .skip(1)
        .map(provider_value)
        .collect::<Vec<_>>();
    let relationships = providers.iter().enumerate().map(|(index, provider)| json!({"provider_id":provider.id,"relationship":if index == 0 { "canonical" } else { alternate_relationship(&identity.provider_group) }})).collect::<Vec<_>>();
    Ok(json!({
        "canonical_name":identity.name, "normalized_name":identity.name, "display_name":identity.name,
        "kind":kind, "role": if kind == "block_data" { "initialization_unit" } else { catalogue_role }, "catalogue_role":catalogue_role,
        "observed_dependency_role":if identity.toc_role == "subsidiary" { "subsidiary" } else { "unknown" },
        "short_description":description, "short_purpose":description, "full_description":full_description, "description_summary":description, "description_source":description_type, "description_source_url":description_url, "description_source_field":source_field, "description_confidence":confidence, "description_dialect":dialect, "description_was_normalized":normalized,
        "description_evidence":{"selected_source":description_type,"variants":source_variants,"toc_shared_purpose_group":toc.and_then(|item| item.group.as_ref()),"unavailable_justification":if confidence == "unavailable" { unavailable_justification(identity, &providers, toc, directory) } else { Value::Null }},
        "official_documentation":reference,
        "user_callable_status":user_status(identity, &kind), "primary_family":family, "secondary_families":[], "mathematical_domain":domain, "gams_classification":gams(identity), "package_provenance":package,
        "precision":precision(&identity.name), "scalar_kind":scalar_kind(&identity.name), "source_status":source_status,
        "canonical_provider":canonical.map(provider_value), "alternate_providers":alternatives, "provider_relationships":relationships,
        "source_file":canonical.map(|provider| format!("{}/{}", provider.subset, provider.path)), "source_hash":canonical.map(|provider| provider.raw_hash.clone()),
        "catalogue_membership":{"current_list":identity.in_list,"version_4_1_toc":identity.in_toc,"list_lines":identity.list_lines,"toc_lines":identity.toc_lines}, "toc_membership":identity.in_toc, "entry_parent":Value::Null,
        "raw_binding_status":raw_binding_status, "source_profile_status":profile, "audit_status":audit_status, "safe_api_status":safe_api_status, "safe_api_paths":safe_api_paths,
        "implementation_status":if safe_api_paths.is_empty() { "not_exposed_as_safe_api" } else { "safe_api_available" }, "deferral_or_exclusion_reason":if safe_api_paths.is_empty() { "Catalogue inclusion does not imply a Rust binding or safe API." } else { "" },
        "notes":notes(identity, pilot),
    }))
}

fn pilot_metadata() -> Result<BTreeMap<String, Pilot>> {
    let path = Path::new("metadata/routines-pilot.toml");
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }
    let parsed: toml::Value = toml::from_str(&fs::read_to_string(path)?)?;
    let mut output = BTreeMap::new();
    for value in parsed
        .get("routines")
        .and_then(toml::Value::as_array)
        .unwrap_or(&Vec::new())
    {
        let Some(table) = value.as_table() else {
            continue;
        };
        let (Some(name), Some(purpose)) = (
            table.get("name").and_then(toml::Value::as_str),
            table.get("purpose").and_then(toml::Value::as_str),
        ) else {
            continue;
        };
        output.insert(
            name.to_ascii_uppercase(),
            Pilot {
                purpose: purpose.to_owned(),
                domain: table
                    .get("candidate_domain")
                    .and_then(toml::Value::as_str)
                    .map(ToOwned::to_owned),
                package: table
                    .get("origin_package")
                    .and_then(toml::Value::as_str)
                    .map(ToOwned::to_owned),
            },
        );
    }
    Ok(output)
}

fn raw_statuses(root: &Path) -> Result<BTreeMap<String, String>> {
    let path = root.join("interface-inventory.json");
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }
    let value = read_json(&path)?;
    let columns = columns(&value)?;
    let mut output = BTreeMap::new();
    for row in rows(&value)? {
        let routine = cell_string(row, &columns, "normalized_name")?;
        let confidence = cell_string(row, &columns, "confidence_class")?;
        output.entry(routine).or_insert_with(|| {
            if confidence.starts_with("generated_") {
                "bound".to_owned()
            } else {
                "not_bound".to_owned()
            }
        });
    }
    Ok(output)
}

fn safe_paths(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let path = root.join("function-index.json");
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }
    let value = read_json(&path)?;
    let mut output = BTreeMap::new();
    if let Some(records) = value.get("records").and_then(Value::as_array) {
        for record in records {
            let (Some(routine), Some(path)) = (
                record.get("fortran_routine").and_then(Value::as_str),
                record.get("rust_path").and_then(Value::as_str),
            ) else {
                continue;
            };
            output
                .entry(routine.to_ascii_uppercase())
                .or_insert_with(Vec::new)
                .push(path.to_owned());
        }
    }
    for paths in output.values_mut() {
        paths.sort();
        paths.dedup();
    }
    Ok(output)
}

fn toc_purposes() -> Result<BTreeMap<String, TocPurpose>> {
    let path = Path::new("evidence/full-corpus/audit-input/catalogues/toc");
    if !path.is_file() {
        return Ok(BTreeMap::new());
    }
    let mut output = BTreeMap::new();
    let text = fs::read_to_string(path)?;
    let lines = text.lines().collect::<Vec<_>>();
    let section_one = lines
        .iter()
        .position(|line| line.contains("SECTION I. User-callable Routines"))
        .unwrap_or(0);
    let section_two = lines
        .iter()
        .position(|line| line.contains("SECTION II. Subsidiary Routines"))
        .unwrap_or(lines.len());
    let section_three = lines
        .iter()
        .position(|line| line.contains("SECTION III. Alphabetic List of Routines"))
        .unwrap_or(lines.len());
    let mut gams = None;
    let mut names = Vec::new();
    let mut fragments = Vec::new();
    let flush = |output: &mut BTreeMap<String, TocPurpose>,
                 names: &mut Vec<String>,
                 fragments: &mut Vec<String>,
                 gams: &Option<String>| {
        let purpose = normalize_description_lines(fragments);
        if !purpose.is_empty() {
            let group = format!(
                "toc-section-i-{}",
                names
                    .first()
                    .map(String::as_str)
                    .unwrap_or("unknown")
                    .to_ascii_lowercase()
            );
            for name in names.iter() {
                output.insert(
                    name.clone(),
                    TocPurpose {
                        purpose: purpose.clone(),
                        gams: gams.clone(),
                        role: "user_callable".to_owned(),
                        group: Some(group.clone()),
                    },
                );
            }
        }
        names.clear();
        fragments.clear();
    };
    for line in &lines[section_one..section_two] {
        let trimmed = line.trim();
        if let Some(code) = toc_gams_heading(trimmed) {
            gams = Some(code);
            continue;
        }
        let Some((routine, rest)) = toc_typed_name(trimmed) else {
            if !names.is_empty() && !trimmed.is_empty() && !toc_noise(trimmed) {
                fragments.push(trimmed.to_owned());
            }
            continue;
        };
        let continues = !rest.is_empty()
            && !fragments.is_empty()
            && rest
                .chars()
                .next()
                .is_some_and(|character| character.is_ascii_lowercase());
        if !names.is_empty() && !continues && !rest.is_empty() {
            flush(&mut output, &mut names, &mut fragments, &gams);
        }
        names.push(routine);
        if !rest.is_empty() {
            fragments.push(rest);
        }
    }
    flush(&mut output, &mut names, &mut fragments, &gams);
    let mut subsidiary_name: Option<String> = None;
    let mut subsidiary_fragments = Vec::new();
    let flush_subsidiary = |output: &mut BTreeMap<String, TocPurpose>,
                            name: &mut Option<String>,
                            fragments: &mut Vec<String>| {
        if let Some(name) = name.take() {
            let purpose = normalize_description_lines(fragments);
            if !purpose.is_empty() {
                output.insert(
                    name.clone(),
                    TocPurpose {
                        purpose,
                        gams: None,
                        role: "subsidiary".to_owned(),
                        group: Some(format!("toc-section-ii-{}", name.to_ascii_lowercase())),
                    },
                );
            }
        }
        fragments.clear();
    };
    for line in &lines[section_two..section_three] {
        let trimmed = line.trim();
        if trimmed.is_empty() || toc_noise(trimmed) {
            continue;
        }
        let mut words = trimmed.split_whitespace();
        let candidate = words.next().unwrap_or("");
        let rest = words.collect::<Vec<_>>().join(" ");
        if identifier(candidate) && !rest.is_empty() && candidate == candidate.to_ascii_uppercase()
        {
            flush_subsidiary(&mut output, &mut subsidiary_name, &mut subsidiary_fragments);
            subsidiary_name = Some(candidate.to_owned());
            subsidiary_fragments.push(rest);
        } else if subsidiary_name.is_some() {
            subsidiary_fragments.push(trimmed.to_owned());
        }
    }
    flush_subsidiary(&mut output, &mut subsidiary_name, &mut subsidiary_fragments);
    if section_three < lines.len() {
        for line in &lines[section_three..] {
            for column in [0_usize, 39] {
                let routine = line
                    .get(column..line.len().min(column + 12))
                    .unwrap_or("")
                    .trim();
                let routine = routine.strip_prefix('*').unwrap_or(routine).trim();
                let classification = line
                    .get((column + 12)..line.len().min(column + 31))
                    .unwrap_or("")
                    .split_whitespace()
                    .next()
                    .unwrap_or("");
                if identifier(routine)
                    && classification.chars().next().is_some_and(|major| {
                        matches!(
                            major,
                            'A' | 'C'
                                | 'D'
                                | 'E'
                                | 'F'
                                | 'G'
                                | 'H'
                                | 'I'
                                | 'J'
                                | 'K'
                                | 'L'
                                | 'N'
                                | 'R'
                                | 'Z'
                        )
                    })
                {
                    output
                        .entry(routine.to_ascii_uppercase())
                        .or_insert_with(|| TocPurpose {
                            purpose: String::new(),
                            gams: None,
                            role: "unknown".to_owned(),
                            group: None,
                        })
                        .gams = Some(classification.trim_end_matches(',').to_owned());
                }
            }
        }
    }
    Ok(output)
}

fn toc_gams_heading(line: &str) -> Option<String> {
    let (code, _) = line.split_once('.')?;
    (code.len() <= 8
        && code.chars().next().is_some_and(|c| {
            matches!(
                c,
                'A' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'N' | 'R' | 'Z'
            )
        })
        && code.chars().all(|c| c.is_ascii_alphanumeric()))
    .then(|| code.to_owned())
}

fn toc_typed_name(line: &str) -> Option<(String, String)> {
    let (token, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
    let (name, marker) = token.rsplit_once('-')?;
    (identifier(name) && matches!(marker, "S" | "D" | "C" | "I" | "H" | "L" | "A"))
        .then(|| (name.to_ascii_uppercase(), rest.trim().to_owned()))
}

fn toc_noise(line: &str) -> bool {
    line.contains("Table of Contents")
        || line.starts_with("SLATEC Common")
        || line.starts_with("Version ")
        || line.starts_with("Page ")
}

fn directory_entries() -> Result<BTreeMap<(String, String), DirectoryEntry>> {
    let mut output = BTreeMap::new();
    for subset in ["src", "lin", "fishfft", "fnlib", "pchip", "err"] {
        let path = Path::new("evidence/full-corpus/audit-input/directories")
            .join(subset)
            .join("index.html");
        if !path.is_file() {
            continue;
        }
        let directory_url = format!("https://www.netlib.org/slatec/{subset}/");
        let mut current: Option<(String, DirectoryEntry)> = None;
        let flush = |output: &mut BTreeMap<(String, String), DirectoryEntry>,
                     current: &mut Option<(String, DirectoryEntry)>| {
            if let Some((name, entry)) = current.take() {
                output.insert((subset.to_owned(), name), entry);
            }
        };
        for raw in fs::read_to_string(&path)?.lines() {
            let text = html_text(raw).trim().to_owned();
            if text.starts_with("file")
                && text
                    .get(4..)
                    .is_some_and(|tail| tail.starts_with(char::is_whitespace))
            {
                flush(&mut output, &mut current);
                let links = html_hrefs(raw);
                let filename = links
                    .first()
                    .map(|(_, label)| label.clone())
                    .or_else(|| text.split_whitespace().nth(1).map(ToOwned::to_owned));
                if let Some(filename) = filename {
                    let fullsource_url = links.get(1).map(|(href, _)| absolute_netlib_url(href));
                    current = Some((
                        filename.clone(),
                        DirectoryEntry {
                            source_url: Some(format!(
                                "https://www.netlib.org/slatec/{subset}/{filename}"
                            )),
                            fullsource_url,
                            directory_url: Some(directory_url.clone()),
                            ..DirectoryEntry::default()
                        },
                    ));
                }
            } else if let Some((_, entry)) = current.as_mut() {
                if let Some(value) = text
                    .strip_prefix("for")
                    .filter(|tail| tail.starts_with(char::is_whitespace))
                {
                    entry.purpose = Some(value.trim().to_owned());
                } else if let Some(value) = text
                    .strip_prefix("gams")
                    .filter(|tail| tail.starts_with(char::is_whitespace))
                {
                    entry.gams = Some(value.trim().to_owned());
                } else if raw.starts_with(char::is_whitespace)
                    && !text.is_empty()
                    && !text.starts_with("by")
                {
                    if let Some(purpose) = entry.purpose.as_mut() {
                        purpose.push(' ');
                        purpose.push_str(&text);
                    }
                }
            }
        }
        flush(&mut output, &mut current);
    }
    for entry in output.values_mut() {
        entry.purpose = entry
            .purpose
            .take()
            .map(|value| normalize_description_lines(&[value]));
    }
    Ok(output)
}

fn html_text(line: &str) -> String {
    let mut output = String::new();
    let mut inside = false;
    for character in line.chars() {
        match character {
            '<' => inside = true,
            '>' => inside = false,
            _ if !inside => output.push(character),
            _ => {}
        }
    }
    output.replace("&amp;", "&")
}

fn html_hrefs(line: &str) -> Vec<(String, String)> {
    let mut output = Vec::new();
    let mut remaining = line;
    while let Some(start) = remaining.find("href=\"") {
        let after = &remaining[(start + 6)..];
        let Some(end) = after.find('"') else {
            break;
        };
        let href = after[..end].to_owned();
        let after_href = &after[(end + 1)..];
        let Some(close) = after_href.find("</a>") else {
            break;
        };
        output.push((href, html_text(&after_href[..close]).trim().to_owned()));
        remaining = &after_href[(close + 4)..];
    }
    output
}

fn absolute_netlib_url(href: &str) -> String {
    if href.starts_with("http") {
        href.to_owned()
    } else {
        format!("https://www.netlib.org{href}")
    }
}

fn source_descriptions(
    providers: &BTreeMap<String, Provider>,
    snapshot: &str,
) -> Result<BTreeMap<(String, String), SourceDescription>> {
    let mut output = BTreeMap::new();
    for provider in providers.values() {
        let path = provider_source_path(provider, snapshot);
        if !path.is_file() {
            continue;
        }
        let text = fs::read_to_string(path)?;
        let parsed = parse_source_descriptions(&text);
        for (name, description) in parsed {
            output.insert((provider.id.clone(), name), description);
        }
    }
    Ok(output)
}

fn provider_source_path(provider: &Provider, snapshot: &str) -> PathBuf {
    if provider.subset == "main-src" {
        Path::new("evidence/extracted")
            .join(snapshot)
            .join("slatec-source-archive")
            .join(&provider.path)
    } else if ["src", "lin", "fishfft", "fnlib", "pchip", "err"].contains(&provider.subset.as_str())
    {
        Path::new("evidence/full-corpus/audit-input/directories")
            .join(&provider.subset)
            .join("files")
            .join(&provider.path)
    } else {
        Path::new("evidence/full-corpus/audit-input/supplemental")
            .join(&provider.subset)
            .join(&provider.path)
    }
}

fn parse_source_descriptions(text: &str) -> BTreeMap<String, SourceDescription> {
    let mut output = BTreeMap::new();
    let mut starts = text
        .match_indices("*DECK ")
        .map(|(position, _)| position)
        .collect::<Vec<_>>();
    if starts.is_empty() {
        starts.push(0);
    }
    starts.push(text.len());
    for window in starts.windows(2) {
        let unit = &text[window[0]..window[1]];
        let name = unit
            .lines()
            .find_map(|line| {
                line.trim_start()
                    .strip_prefix("*DECK ")
                    .map(|value| value.trim().to_ascii_uppercase())
            })
            .or_else(|| program_name(unit));
        let Some(name) = name.filter(|name| identifier(name)) else {
            continue;
        };
        if let Some(description) = parse_prologue(unit) {
            output.insert(name, description);
        }
    }
    output
}

fn program_name(text: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let upper = line.trim_start().to_ascii_uppercase();
        ["SUBROUTINE ", "FUNCTION ", "PROGRAM ", "BLOCK DATA "]
            .iter()
            .find_map(|prefix| upper.strip_prefix(prefix))
            .and_then(|rest| rest.split(|c: char| c == '(' || c.is_whitespace()).next())
            .filter(|name| identifier(name))
            .map(ToOwned::to_owned)
    })
}

fn parse_prologue(text: &str) -> Option<SourceDescription> {
    let lines = text.lines().collect::<Vec<_>>();
    let begin = lines
        .iter()
        .position(|line| line.to_ascii_uppercase().contains("BEGIN PROLOGUE"));
    let end = lines
        .iter()
        .position(|line| line.to_ascii_uppercase().contains("END PROLOGUE"))
        .unwrap_or(lines.len());
    let range = if let Some(begin) = begin {
        &lines[begin..end]
    } else {
        &lines[..end]
    };
    let mut purpose = Vec::new();
    let mut description = Vec::new();
    let mut active: Option<&str> = None;
    for raw in range {
        let (marker, content) = prologue_line(raw);
        if let Some(marker) = marker {
            let marker = marker.to_ascii_uppercase();
            if ["PURPOSE", "DESCRIPTION", "ABSTRACT", "FUNCTION"].contains(&marker.as_str()) {
                active = Some(if marker == "PURPOSE" {
                    "purpose"
                } else {
                    "description"
                });
                if !content.is_empty() {
                    if active == Some("purpose") {
                        purpose.push(content);
                    } else {
                        description.push(content);
                    }
                }
                continue;
            }
            active = None;
            continue;
        }
        let stop = content.to_ascii_uppercase();
        let decorative_heading = stop
            .chars()
            .filter(|character| character.is_ascii_alphabetic())
            .collect::<String>();
        if active == Some("description")
            && [
                "DESCRIPTION OF PARAMETERS",
                "CALLING SEQUENCE",
                "PARAMETERS:",
                "ARGUMENTS:",
                "INPUT--",
                "OUTPUT--",
                "***** I N P U T",
                "***** O U T P U T",
                "***SEE ALSO",
            ]
            .iter()
            .any(|boundary| stop.starts_with(boundary))
            || (active == Some("description")
                && matches!(decorative_heading.as_str(), "INPUT" | "OUTPUT" | "METHOD"))
        {
            active = None;
            continue;
        }
        if let Some(active) = active {
            if active == "purpose" {
                purpose.push(content);
            } else {
                description.push(content);
            }
        }
    }
    let purpose = normalize_description_lines(&purpose);
    let full_description = normalize_description_lines(&description);
    let has_purpose = !purpose.is_empty();
    (!purpose.is_empty() || !full_description.is_empty()).then(|| SourceDescription {
        purpose: has_purpose.then_some(purpose),
        full_description: (!full_description.is_empty()).then_some(full_description),
        field: Some(
            if !has_purpose {
                "DESCRIPTION"
            } else {
                "PURPOSE"
            }
            .to_owned(),
        ),
        dialect: if begin.is_some() {
            "slatec_standard".to_owned()
        } else {
            "legacy_comment".to_owned()
        },
        was_normalized: true,
    })
}

fn prologue_line(raw: &str) -> (Option<String>, String) {
    let trimmed = raw.trim_start();
    let comment = trimmed
        .strip_prefix(['C', 'c', '*', '!'])
        .unwrap_or("")
        .trim_start();
    let explicit_marker = comment.starts_with("***");
    let starred = comment.strip_prefix("***").unwrap_or(comment).trim_start();
    for label in [
        "PURPOSE",
        "DESCRIPTION",
        "ABSTRACT",
        "FUNCTION",
        "LIBRARY",
        "CATEGORY",
        "TYPE",
        "KEYWORDS",
        "AUTHOR",
        "REFERENCES",
        "ROUTINES",
        "REVISION",
        "END",
        "BEGIN",
    ] {
        if let Some(rest) = starred.strip_prefix(label) {
            let valid_legacy_boundary = !matches!(label, "DESCRIPTION" | "ABSTRACT" | "FUNCTION")
                || rest.trim().is_empty()
                || rest.trim_start().starts_with(':');
            if (explicit_marker || valid_legacy_boundary)
                && (rest.is_empty()
                    || rest.starts_with(char::is_whitespace)
                    || rest.starts_with(':'))
            {
                return (
                    Some(label.to_owned()),
                    rest.trim_start_matches(':').trim().to_owned(),
                );
            }
        }
    }
    if let Some((label, rest)) = starred.split_once(char::is_whitespace) {
        let label_upper = label.trim_end_matches(':').to_ascii_uppercase();
        if [
            "PURPOSE",
            "DESCRIPTION",
            "ABSTRACT",
            "FUNCTION",
            "LIBRARY",
            "CATEGORY",
            "TYPE",
            "KEYWORDS",
            "AUTHOR",
            "REFERENCES",
            "ROUTINES",
            "REVISION",
            "END",
            "BEGIN",
        ]
        .contains(&label_upper.as_str())
            && (!matches!(
                label_upper.as_str(),
                "DESCRIPTION" | "ABSTRACT" | "FUNCTION"
            ) || explicit_marker
                || rest.trim().is_empty()
                || rest.trim_start().starts_with(':'))
        {
            return (Some(label_upper), rest.trim().to_owned());
        }
    }
    (None, comment.to_owned())
}

fn directory_url(
    directories: &BTreeMap<(String, String), DirectoryEntry>,
    provider: &Provider,
) -> Option<String> {
    directories
        .get(&(provider.subset.clone(), provider.path.clone()))
        .and_then(|entry| entry.source_url.clone())
}

fn description_variants(
    source: Option<&SourceDescription>,
    toc: Option<&TocPurpose>,
    directory: Option<&DirectoryEntry>,
) -> Vec<Value> {
    let mut variants = Vec::new();
    if let Some(item) = source {
        if let Some(purpose) = &item.purpose {
            variants.push(json!({"source":"source_prologue","text":purpose,"field":"PURPOSE","dialect":item.dialect}));
        }
    }
    if let Some(item) = toc.filter(|item| !item.purpose.is_empty()) {
        variants.push(json!({"source":"netlib_slatec_toc","text":item.purpose,"field":"TOC purpose","role":item.role}));
    }
    if let Some(item) = directory.and_then(|item| item.purpose.as_ref()) {
        variants.push(json!({"source":"netlib_directory_index","text":item,"field":"for"}));
    }
    variants
}

fn official_references(
    providers: &[Provider],
    directories: &BTreeMap<(String, String), DirectoryEntry>,
    toc: Option<&TocPurpose>,
) -> Value {
    let directory = providers
        .iter()
        .find_map(|provider| directories.get(&(provider.subset.clone(), provider.path.clone())));
    json!({
        "netlib_source_url":directory.and_then(|entry| entry.source_url.as_ref()).map(|url| json!({"url":url,"status":"verified_cached"})),
        "netlib_fullsource_url":directory.and_then(|entry| entry.fullsource_url.as_ref()).map(|url| json!({"url":url,"status":"verified_cached"})),
        "netlib_directory_entry_url":directory.and_then(|entry| entry.directory_url.as_ref()).map(|url| json!({"url":url,"status":"verified_cached"})),
        "netlib_toc_reference":toc.map(|item| json!({"url":"https://www.netlib.org/slatec/toc","status":"verified_cached","role":item.role,"shared_purpose_group":item.group})),
        "nist_gams_module_url":Value::Null,
        "nist_gams_status":"not_matched_during_offline_generation",
        "secondary_html_reference_url":Value::Null,
        "secondary_html_status":"not_verified"
    })
}

fn unavailable_justification(
    identity: &Identity,
    providers: &[Provider],
    toc: Option<&TocPurpose>,
    directory: Option<&DirectoryEntry>,
) -> Value {
    json!({
        "identity":identity.name,
        "toc_checked":identity.in_toc || toc.is_some(),
        "canonical_provider_checked":!providers.is_empty(),
        "directory_checked":directory.is_some() || !providers.is_empty(),
        "nist_gams_checked":"no deterministic cached module match",
        "reason":"No complete purpose was present in the available cached source prologue, Version 4.1 TOC, or matching Netlib directory entry."
    })
}

fn comparison(left: &str, right: &str, right_source: &str) -> &'static str {
    let normalized = |value: &str| {
        value
            .to_ascii_lowercase()
            .chars()
            .filter(|character| {
                character.is_ascii_alphanumeric() || character.is_ascii_whitespace()
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };
    let left_normalized = normalized(left);
    let right_normalized = normalized(right);
    if left == right {
        "exact_match"
    } else if left_normalized == right_normalized {
        "punctuation_or_case_only"
    } else if left_normalized.contains(&right_normalized) {
        if right_source == "netlib_slatec_toc" {
            "toc_is_summary"
        } else {
            "directory_is_summary"
        }
    } else if right_normalized.contains(&left_normalized) {
        "source_is_expansion"
    } else {
        "semantic_disagreement"
    }
}

fn mangled_reasons(value: &str) -> Vec<&'static str> {
    let mut output = Vec::new();
    let lower = value.to_ascii_lowercase();
    if [" of", " to", " and", " with", " for", " the"]
        .iter()
        .any(|suffix| lower.ends_with(suffix))
    {
        output.push("ends_with_incomplete_connector");
    }
    if value.contains("C***") || value.contains("***PURPOSE") {
        output.push("contains_fixed_form_marker");
    }
    if value.contains("...") {
        output.push("contains_unexplained_ellipsis");
    }
    if value.matches('(').count() != value.matches(')').count() {
        output.push("unbalanced_parentheses");
    }
    if lower.trim() == "subsidiary to" {
        output.push("incomplete_subsidiary_reference");
    }
    output
}

fn description_coverage(records: &[Value]) -> Value {
    let confidence_count = |source: &str| {
        records
            .iter()
            .filter(|record| field(record, "description_source") == source)
            .count()
    };
    let agreements = records
        .iter()
        .filter(|record| {
            record
                .pointer("/description_evidence/variants")
                .and_then(Value::as_array)
                .is_some_and(|variants| {
                    variants.len() > 1
                        && variants.windows(2).all(|pair| {
                            comparison(
                                pair[0]
                                    .get("text")
                                    .and_then(Value::as_str)
                                    .unwrap_or_default(),
                                pair[1]
                                    .get("text")
                                    .and_then(Value::as_str)
                                    .unwrap_or_default(),
                                pair[1]
                                    .get("source")
                                    .and_then(Value::as_str)
                                    .unwrap_or_default(),
                            ) != "semantic_disagreement"
                        })
                })
        })
        .count();
    let mangled = records
        .iter()
        .filter(|record| !mangled_reasons(field(record, "short_purpose")).is_empty())
        .count();
    json!({"schema_id":"slatec-rs/description-coverage","schema_version":VERSION,
        "total_retained_identities":records.len(), "user_callable_identities":records.iter().filter(|record| field(record,"user_callable_status") == "historically_user_callable").count(), "subsidiary_identities":records.iter().filter(|record| field(record,"user_callable_status") == "historically_subsidiary").count(),
        "complete_short_purposes":records.iter().filter(|record| field(record,"description_confidence") != "unavailable").count(), "complete_full_descriptions":records.iter().filter(|record| record.get("full_description").and_then(Value::as_str).is_some_and(|value| !value.is_empty())).count(), "toc_only_descriptions":confidence_count("netlib_slatec_toc"), "directory_index_only_descriptions":confidence_count("netlib_directory_index"), "gams_only_descriptions":0, "pilot_only_descriptions":confidence_count("pilot_metadata"), "unavailable_descriptions":confidence_count("unavailable"), "mangled_description_candidates":mangled, "cross_source_description_agreements":agreements, "cross_source_description_disagreements":records.iter().filter(|record| record.pointer("/description_evidence/variants").and_then(Value::as_array).is_some_and(|variants| variants.windows(2).any(|pair| comparison(pair[0].get("text").and_then(Value::as_str).unwrap_or_default(), pair[1].get("text").and_then(Value::as_str).unwrap_or_default(), pair[1].get("source").and_then(Value::as_str).unwrap_or_default()) == "semantic_disagreement"))).count(), "verified_netlib_links":records.iter().filter(|record| record.pointer("/official_documentation/netlib_source_url").is_some_and(|value| !value.is_null())).count(), "verified_gams_matches":0, "verified_secondary_html_matches":0,
        "unavailable_user_callable":records.iter().filter(|record| field(record,"user_callable_status") == "historically_user_callable" && field(record,"description_confidence") == "unavailable").map(|record| json!({"identity":name(record),"justification":record.pointer("/description_evidence/unavailable_justification")})).collect::<Vec<_>>()
    })
}

fn description_discrepancies(records: &[Value]) -> Value {
    let mut entries = Vec::new();
    for record in records {
        let variants = record
            .pointer("/description_evidence/variants")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for pair in variants.windows(2) {
            let left = pair[0]
                .get("text")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let right = pair[1]
                .get("text")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let classification = comparison(
                left,
                right,
                pair[1]
                    .get("source")
                    .and_then(Value::as_str)
                    .unwrap_or_default(),
            );
            if classification != "exact_match" {
                entries.push(json!({"identity":name(record),"classification":classification,"left":pair[0],"right":pair[1],"mangled_candidates":mangled_reasons(field(record,"short_purpose"))}));
            }
        }
    }
    json!({"schema_id":"slatec-rs/description-discrepancies","schema_version":VERSION,"records":entries})
}

fn external_reference_map(records: &[Value]) -> Value {
    json!({"schema_id":"slatec-rs/external-reference-map","schema_version":VERSION,"verification_scope":{"netlib":"cached official directory and TOC evidence","nist_gams":"institutional site reviewed; no per-module cached matches","secondary_html":"no routine rendering verified in this refresh"},"records":records.iter().map(|record| json!({"identity":name(record),"references":record.get("official_documentation")})).collect::<Vec<_>>()})
}

fn excluded_candidates(records: &[Value]) -> Value {
    json!({"schema_id":"slatec-rs/excluded-candidates","schema_version":VERSION,"records":records.iter().filter(|record| field(record,"source_status") == "catalogue_only").map(|record| json!({"identity":name(record),"reason":"No reconciled source provider; retained as a catalogue-only identity for audit rather than inventing a provider or description."})).collect::<Vec<_>>()})
}

fn normalize_description_lines(lines: &[String]) -> String {
    let mut paragraphs = Vec::new();
    let mut paragraph = String::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            if !paragraph.is_empty() {
                paragraphs.push(std::mem::take(&mut paragraph));
            }
            continue;
        }
        if line.chars().all(|c| c == '-' || c == '=') {
            continue;
        }
        if paragraph.ends_with('-') {
            paragraph.pop();
            paragraph.push_str(line);
        } else {
            if !paragraph.is_empty() {
                paragraph.push(' ');
            }
            paragraph.push_str(line);
        }
    }
    if !paragraph.is_empty() {
        paragraphs.push(paragraph);
    }
    paragraphs
        .join("\n\n")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn role(identity: &Identity, kind: &str) -> &'static str {
    if kind == "block_data" {
        return "initialization_unit";
    }
    if kind == "entry" {
        return "entry_point";
    }
    if matches!(identity.name.as_str(), "D1MACH" | "I1MACH" | "R1MACH") {
        return "machine_constant";
    }
    if identity.name.starts_with("XER")
        || matches!(
            identity.name.as_str(),
            "FDUMP" | "J4SAVE" | "NUMXER" | "XSETF" | "XSETUN"
        )
    {
        return "runtime_helper";
    }
    match identity.toc_role.as_str() {
        "user_callable" | "user_candidate" => "user_callable",
        "subsidiary" => "subsidiary",
        _ if identity.provider_ids.is_empty() => "catalogue_only_unresolved",
        _ => "source_only_unclassified",
    }
}

fn user_status(identity: &Identity, kind: &str) -> &'static str {
    if kind == "block_data" {
        "not_applicable"
    } else if matches!(
        identity.toc_role.as_str(),
        "user_callable" | "user_candidate"
    ) {
        "historically_user_callable"
    } else if identity.toc_role == "subsidiary" {
        "historically_subsidiary"
    } else {
        "unresolved"
    }
}

fn classify(
    identity: &Identity,
    providers: &[Provider],
    pilot: Option<&Pilot>,
) -> (String, String, String) {
    if let Some(pilot) = pilot {
        return (
            family_label(pilot.domain.as_deref().unwrap_or("unclassified")),
            pilot
                .domain
                .clone()
                .unwrap_or_else(|| "uncategorized".to_owned()),
            pilot
                .package
                .clone()
                .unwrap_or_else(|| "unknown".to_owned()),
        );
    }
    let routine = identity.name.as_str();
    if matches!(routine, "D1MACH" | "I1MACH" | "R1MACH") {
        return (
            "Machine constants".to_owned(),
            "runtime-support".to_owned(),
            "slatec-machine-constants".to_owned(),
        );
    }
    if routine.starts_with("XER")
        || matches!(routine, "FDUMP" | "J4SAVE" | "NUMXER" | "XSETF" | "XSETUN")
        || providers.iter().any(|p| p.subset == "err")
    {
        return (
            "Error handling and runtime".to_owned(),
            "runtime-support".to_owned(),
            "slatec-error".to_owned(),
        );
    }
    if providers.iter().any(|p| p.subset == "pchip") {
        return (
            "PCHIP".to_owned(),
            "interpolation".to_owned(),
            "pchip".to_owned(),
        );
    }
    if providers.iter().any(|p| p.subset == "fishfft") {
        return if gams_major(identity) == Some('J') {
            (
                "FFTPACK and transforms".to_owned(),
                "transforms".to_owned(),
                "fftpack".to_owned(),
            )
        } else {
            (
                "FISHPACK and PDE methods".to_owned(),
                "pde-integral-equations".to_owned(),
                "fishpack".to_owned(),
            )
        };
    }
    if providers.iter().any(|p| p.subset == "fnlib") {
        return (
            "Special functions".to_owned(),
            "special-functions".to_owned(),
            "fnlib".to_owned(),
        );
    }
    if bspline(routine) {
        return (
            "B-splines".to_owned(),
            "interpolation".to_owned(),
            "unknown".to_owned(),
        );
    }
    if routine.starts_with("PP") || routine.starts_with("DPP") {
        return (
            "Piecewise polynomials".to_owned(),
            "interpolation".to_owned(),
            "unknown".to_owned(),
        );
    }
    let (family, domain) = match gams_major(identity) {
        Some('A') => ("Arithmetic and numerical utilities", "numeric-support"),
        Some('C') => ("Special functions", "special-functions"),
        Some('D') => ("Linear algebra", "dense-linear-algebra"),
        Some('E') => ("Interpolation", "interpolation"),
        Some('F') => ("Nonlinear equations", "nonlinear-equations"),
        Some('G') => ("Optimization and least squares", "optimization"),
        Some('H') => ("Quadrature and cubature", "quadrature"),
        Some('I') => ("Differential and integral equations", "ode-dae"),
        Some('J') => ("Integral transforms", "transforms"),
        Some('K') => ("Approximation", "approximation"),
        Some('L') => ("Probability and statistics", "probability-statistics"),
        Some('N') | Some('R') => ("Utilities", "data-utilities"),
        _ => ("Unclassified or unresolved", "uncategorized"),
    };
    (family.to_owned(), domain.to_owned(), "unknown".to_owned())
}

fn family_label(domain: &str) -> String {
    match domain {
        "runtime-support" => "Error handling and runtime",
        "linear-algebra-kernels" => "BLAS",
        "dense-linear-algebra" | "sparse-linear-algebra" => "Linear algebra",
        "special-functions" => "Special functions",
        "interpolation" => "Interpolation",
        "quadrature" => "Quadrature and cubature",
        "ode-dae" => "Differential and integral equations",
        "optimization" => "Optimization and least squares",
        "transforms" => "FFTPACK and transforms",
        "nonlinear-equations" => "Nonlinear equations",
        "probability-statistics" => "Probability and statistics",
        "approximation" => "Approximation",
        _ => return title(domain),
    }
    .to_owned()
}

fn source_status(identity: &Identity, providers: &[Provider]) -> &'static str {
    if providers.is_empty() {
        "catalogue_only"
    } else if matches!(
        identity.provider_group.as_str(),
        "duplicate_provider"
            | "modified_relocated_copy"
            | "alternate_implementation"
            | "historical_variant"
    ) {
        "conflicting"
    } else if providers
        .iter()
        .any(|provider| provider.subset == "main-src")
    {
        "canonical_verified"
    } else {
        "provider_present"
    }
}
fn alternate_relationship(group: &str) -> &'static str {
    match group {
        "byte_identical_relocated_copy" => "byte_identical_duplicate",
        "normalized_identical_relocated_copy" => "normalized_equivalent",
        "modified_relocated_copy" | "historical_variant" | "alternate_implementation" => {
            "modified_variant"
        }
        "duplicate_provider" => "conflicting_provider",
        _ => "relocated_copy",
    }
}
fn provider_order(a: &Provider, b: &Provider) -> std::cmp::Ordering {
    let rank = |p: &Provider| match p.subset.as_str() {
        "main-src" => 0,
        "src" => 1,
        _ => 2,
    };
    (rank(a), &a.subset, &a.path, &a.id).cmp(&(rank(b), &b.subset, &b.path, &b.id))
}
fn provider_value(provider: &Provider) -> Value {
    json!({"provider_id":provider.id,"subset":provider.subset,"relationship":provider.relationship,"source_file":provider.path,"source_hash":provider.raw_hash,"normalized_source_hash":provider.normalized_hash})
}
fn precision(name: &str) -> &'static str {
    match name.as_bytes().first() {
        Some(b'D') => "f64",
        Some(b'S') | Some(b'R') => "f32",
        Some(b'C') => "complex_f32",
        Some(b'Z') => "complex_f64",
        Some(b'I') => "integer_or_index",
        _ => "unknown",
    }
}
fn scalar_kind(name: &str) -> &'static str {
    match precision(name) {
        "f64" | "f32" => "real",
        "complex_f32" | "complex_f64" => "complex",
        "integer_or_index" => "integer",
        _ => "unknown",
    }
}
fn gams_major(identity: &Identity) -> Option<char> {
    identity
        .gams
        .as_ref()
        .and_then(|classification| classification.chars().next())
}
fn gams(identity: &Identity) -> Value {
    identity
        .gams
        .as_ref()
        .map(|classification| json!({"code":classification,"major":classification.chars().next().map(|major| major.to_string()),"status":"toc_evidence"}))
        .unwrap_or(Value::Null)
}
fn bspline(name: &str) -> bool {
    matches!(
        name,
        "BINT4"
            | "BINTK"
            | "BSPDOC"
            | "BSPDR"
            | "BSPEV"
            | "BSPLVD"
            | "BSPLVN"
            | "BSPPP"
            | "BSPVD"
            | "BSPVN"
            | "BSQAD"
            | "BVALU"
            | "DBVALU"
    ) || name.starts_with("DBINT")
        || name.starts_with("DBSP")
        || name.starts_with("DBSQ")
}
fn title(id: &str) -> String {
    id.split('-')
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .map(|initial| initial.to_ascii_uppercase().to_string() + chars.as_str())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 12
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'$' || byte == b'_')
}
fn notes(identity: &Identity, pilot: Option<&Pilot>) -> Vec<String> {
    let mut notes = Vec::new();
    if identity.provider_ids.is_empty() {
        notes.push(
            "Catalogue identity has no reconciled source provider and remains visible for review."
                .to_owned(),
        );
    }
    if matches!(
        identity.provider_group.as_str(),
        "duplicate_provider"
            | "modified_relocated_copy"
            | "alternate_implementation"
            | "historical_variant"
    ) {
        notes.push(
            "Multiple or variant providers remain separate pending manual reconciliation."
                .to_owned(),
        );
    }
    if pilot.is_some() {
        notes.push(
            "Enriched from the 20-routine pilot; this catalogue is the canonical corpus view."
                .to_owned(),
        );
    }
    notes
}

fn coverage(records: &[Value], providers: &BTreeMap<String, Provider>, manifest: &Value) -> Value {
    let count = |key: &str, expected: &str| {
        records
            .iter()
            .filter(|record| field(record, key) == expected)
            .count()
    };
    let source_files = providers
        .values()
        .map(|provider| (&provider.subset, &provider.path))
        .collect::<BTreeSet<_>>()
        .len();
    let source_only = records
        .iter()
        .filter(|record| {
            field(record, "source_status") != "catalogue_only"
                && !record
                    .pointer("/catalogue_membership/current_list")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
                && !record
                    .pointer("/catalogue_membership/version_4_1_toc")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
        })
        .count();
    let equivalent_provider_groups = records
        .iter()
        .filter(|record| {
            field(record, "source_status") != "conflicting"
                && record
                    .get("alternate_providers")
                    .and_then(Value::as_array)
                    .is_some_and(|providers| !providers.is_empty())
        })
        .count();
    json!({"schema_id":"slatec-rs/routine-catalogue-coverage-summary","schema_version":VERSION,
        "total_logical_identities":records.len(), "user_callable_identities":count("user_callable_status","historically_user_callable"), "subsidiary_helper_identities":count("user_callable_status","historically_subsidiary"), "entry_identities":count("kind","entry"), "block_data_identities":count("kind","block_data"),
        "source_only_identities":source_only, "catalogue_only_identities":count("source_status","catalogue_only"), "unresolved_role_identities":count("user_callable_status","unresolved"), "provider_count":providers.len(), "source_file_count":source_files,
        "duplicate_provider_groups":count("source_status","conflicting"), "duplicate_equivalent_provider_groups":equivalent_provider_groups, "conflicting_provider_groups":count("source_status","conflicting"), "described_identities":records.iter().filter(|record| field(record,"description_confidence") != "unavailable").count(), "identities_without_descriptions":count("description_confidence","unavailable"), "raw_bound_identities":count("raw_binding_status","bound"), "safely_wrapped_identities":count("safe_api_status","safe_public"), "deeply_audited_identities":count("audit_status","deeply_audited"),
        "documented_comparison":{"user_callable_target":902,"total_routine_floor":1400,"audit_reported_union":manifest.pointer("/summary/unique_program_units_in_union")}})
}

fn provider_map(records: &[Value]) -> Value {
    json!({"schema_id":"slatec-rs/routine-provider-map","schema_version":VERSION,"records":records.iter().map(|record| json!({"normalized_name":name(record),"canonical_provider":record.get("canonical_provider"),"alternate_providers":record.get("alternate_providers"),"provider_relationships":record.get("provider_relationships")})).collect::<Vec<_>>()})
}
fn family_index(records: &[Value]) -> Value {
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for record in records {
        grouped
            .entry(field(record, "primary_family").to_owned())
            .or_default()
            .push(name(record).to_owned());
    }
    json!({"schema_id":"slatec-rs/routine-family-index","schema_version":VERSION,"records":grouped.into_iter().map(|(family, mut identities)| { identities.sort(); json!({"family":family,"identities":identities}) }).collect::<Vec<_>>()})
}
fn alphabetical_index(records: &[Value]) -> Value {
    json!({"schema_id":"slatec-rs/routine-alphabetical-index","schema_version":VERSION,"records":records.iter().map(|record| json!({"normalized_name":name(record),"anchor":anchor(name(record)),"primary_family":record.get("primary_family"),"role":record.get("role")})).collect::<Vec<_>>()})
}
fn diagnostics(records: &[Value], manifest: &Value) -> Value {
    let mut output = Vec::new();
    for record in records {
        if field(record, "source_status") == "catalogue_only" {
            output.push(json!({"rule":"catalogue_only","identity":name(record),"message":"Catalogue evidence has no reconciled provider."}));
        }
        if field(record, "source_status") == "conflicting" {
            output.push(json!({"rule":"conflicting_provider","identity":name(record),"message":"Variant providers were retained for manual reconciliation."}));
        }
        if field(record, "user_callable_status") == "unresolved" {
            output.push(json!({"rule":"unresolved_role","identity":name(record),"message":"User/subsidiary status remains unresolved."}));
        }
    }
    json!({"schema_id":"slatec-rs/routine-reconciliation-diagnostics","schema_version":VERSION,"documented_targets":{"user_callable":902,"total_floor":1400},"audit_union":manifest.pointer("/summary/unique_program_units_in_union"),"records":output})
}

fn write_docs(root: &Path, records: &[Value], summary: &Value, diagnostics: &Value) -> Result<()> {
    let output = root.join("reference");
    fs::create_dir_all(&output)?;
    let total = summary
        .get("total_logical_identities")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let user = summary
        .get("user_callable_identities")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let subsidiary = summary
        .get("subsidiary_helper_identities")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    fs::write(
        output.join("slatec-routine-index.md"),
        format!(
            "# Complete SLATEC Routine Index\n\nThis deterministic catalogue is both a historical inventory and a `slatec-rs` coverage map. It contains **{total}** logical identities: **{user}** historically user-callable and **{subsidiary}** explicitly subsidiary identities. Source files, program units, logical identities, and providers are deliberately distinct.\n\n- [Browse by Function Family](routines-by-family.md)\n- [Browse Alphabetically](routines-alphabetical.md)\n- [Coverage and Reconciliation](routine-coverage.md)\n\n## Evidence and status\n\nEvidence reconciles immutable `main-src`, Netlib `slatec/list`, the Version 4.1 TOC, live `src`, relocated subsets, supplements, pilot metadata, raw-interface inventories, and safe-API indexes. A **provider** is a source location; **raw binding** means generated ABI coverage under a reviewed profile; **safe API** means a public Rust wrapper; **audited** is reserved for reviewed safe wrappers; **deferred** means it is not exposed as a safe API.\n\nInclusion does not imply a modern recommendation, ABI safety, compilation, a canonical provider, a raw binding, or a stable public interface for a subsidiary routine.\n"
        ),
    )?;
    writeln!(
        fs::OpenOptions::new()
            .append(true)
            .open(output.join("slatec-routine-index.md"))?,
        "\n## Description evidence\n\nDescriptions are assembled from canonical Netlib source prologues, the official Version 4.1 TOC, cached Netlib directory metadata, NIST GAMS where a verified module match exists, and reviewed secondary sources. Source revisions can differ; the canonical source prologue takes precedence. Compact indexes show a complete short purpose; [routine details and evidence](routine-details.md) retain full source descriptions and external cross-references. External references are cross-checks, not replacements for local evidence."
    )?;
    fs::write(
        output.join("routines-by-family.md"),
        family_markdown(records),
    )?;
    fs::write(
        output.join("routines-alphabetical.md"),
        alphabetical_markdown(records),
    )?;
    fs::write(output.join("routine-details.md"), details_markdown(records))?;
    let diagnostics_count = diagnostics
        .get("records")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    fs::write(
        output.join("routine-coverage.md"),
        format!(
            "# SLATEC Routine Coverage and Reconciliation\n\n[Complete index](slatec-routine-index.md) · [Browse by family](routines-by-family.md) · [Alphabetical lookup](routines-alphabetical.md)\n\n| Measure | Count |\n| --- | ---: |\n| Logical identities | {total} |\n| Historically user-callable | {user} |\n| Subsidiary/helper | {subsidiary} |\n| Source files | {} |\n| Providers | {} |\n| Catalogue-only | {} |\n| Raw-bound | {} |\n| Safely wrapped | {} |\n| Deeply audited | {} |\n\nThe documented reference values are approximately 902 user-callable and at least 1,400 total routines. Computed values are reported as found; unresolved entries and provider variants remain visible rather than being invented or collapsed.\n\n## Outstanding reconciliation\n\n**{diagnostics_count}** diagnostics cover catalogue-only identities, conflicting providers, and unresolved roles. Description text uses a concise TOC or pilot purpose only where current evidence provides it; all other records state explicit unavailability.\n",
            summary
                .get("source_file_count")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            summary
                .get("provider_count")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            summary
                .get("catalogue_only_identities")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            summary
                .get("raw_bound_identities")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            summary
                .get("safely_wrapped_identities")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            summary
                .get("deeply_audited_identities")
                .and_then(Value::as_u64)
                .unwrap_or(0)
        ),
    )?;
    Ok(())
}

fn family_markdown(records: &[Value]) -> String {
    let mut groups: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for record in records {
        groups
            .entry(field(record, "primary_family").to_owned())
            .or_default()
            .push(record);
    }
    let mut output = "# SLATEC Routines by Function Family\n\n[Complete index](slatec-routine-index.md) · [Alphabetical lookup](routines-alphabetical.md) · [Coverage](routine-coverage.md)\n\n`source`, `raw`, and `safe` are independent coverage dimensions.\n".to_owned();
    for (family, mut rows) in groups {
        rows.sort_by(|a, b| name(a).cmp(name(b)));
        output.push_str(&format!("\n## {family}\n\n| Routine | Role | Purpose | Precision | Source | Raw | Safe |\n| --- | --- | --- | --- | --- | --- | --- |\n"));
        for row in rows {
            output.push_str(&format!(
                "| [{}](routine-details.md#{}) | {} | {} | {} | {} | {} | {} |\n",
                name(row),
                anchor(name(row)),
                field(row, "role"),
                compact(field(row, "short_description")),
                field(row, "precision"),
                field(row, "source_status"),
                field(row, "raw_binding_status"),
                field(row, "safe_api_status")
            ));
        }
    }
    output
}
fn alphabetical_markdown(records: &[Value]) -> String {
    let mut groups: BTreeMap<char, Vec<&Value>> = BTreeMap::new();
    for record in records {
        groups
            .entry(
                name(record)
                    .chars()
                    .next()
                    .filter(char::is_ascii_alphabetic)
                    .unwrap_or('#'),
            )
            .or_default()
            .push(record);
    }
    let mut output = "# SLATEC Routines: Alphabetical Index\n\n[Complete index](slatec-routine-index.md) · [Browse by family](routines-by-family.md) · [Coverage](routine-coverage.md)\n\n[A](#alpha-a) [B](#alpha-b) [C](#alpha-c) [D](#alpha-d) [E](#alpha-e) [F](#alpha-f) [G](#alpha-g) [H](#alpha-h) [I](#alpha-i) [J](#alpha-j) [K](#alpha-k) [L](#alpha-l) [M](#alpha-m) [N](#alpha-n) [O](#alpha-o) [P](#alpha-p) [Q](#alpha-q) [R](#alpha-r) [S](#alpha-s) [T](#alpha-t) [U](#alpha-u) [V](#alpha-v) [W](#alpha-w) [X](#alpha-x) [Y](#alpha-y) [Z](#alpha-z)\n".to_owned();
    for (letter, mut rows) in groups {
        rows.sort_by(|a, b| name(a).cmp(name(b)));
        output.push_str(&format!("\n## <a id=\"alpha-{}\"></a>{}\n\n| Routine | Family | Role | Purpose | Coverage |\n| --- | --- | --- | --- | --- |\n", letter.to_ascii_lowercase(), if letter == '#' { "Other".to_owned() } else { letter.to_string() }));
        for row in rows {
            output.push_str(&format!(
                "| <a id=\"{}\"></a>[{}](routine-details.md#{}) | {} | {} | {} | source: {}; raw: {}; safe: {} |\n",
                anchor(name(row)),
                name(row),
                anchor(name(row)),
                field(row, "primary_family"),
                field(row, "role"),
                compact(field(row, "short_description")),
                field(row, "source_status"),
                field(row, "raw_binding_status"),
                field(row, "safe_api_status")
            ));
        }
    }
    output
}

fn details_markdown(records: &[Value]) -> String {
    let mut output = "# SLATEC Routine Details\n\n[Complete index](slatec-routine-index.md) | [Browse by family](routines-by-family.md) | [Alphabetical lookup](routines-alphabetical.md)\n\nFull descriptions retain the complete logical source `DESCRIPTION` or `ABSTRACT` field where cached evidence provides one.\n".to_owned();
    for record in records {
        output.push_str(&format!(
            "\n## <a id=\"{}\"></a>{}\n\n",
            anchor(name(record)),
            name(record)
        ));
        output.push_str(&format!("- Purpose: {}\n- Role: {}\n- Kind: {}\n- Precision: {}\n- Family: {}\n- Package provenance: {}\n- Canonical provider: {}\n- Raw binding: {}\n- Safe API: {}\n", compact(field(record, "short_purpose")), field(record, "role"), field(record, "kind"), field(record, "precision"), field(record, "primary_family"), field(record, "package_provenance"), record.pointer("/canonical_provider/source_file").and_then(Value::as_str).unwrap_or("unavailable"), field(record, "raw_binding_status"), field(record, "safe_api_status")));
        if let Some(code) = record
            .pointer("/gams_classification/code")
            .and_then(Value::as_str)
        {
            output.push_str(&format!("- GAMS classification: {code}\n"));
        }
        output.push_str("\n### Full description\n\n");
        output.push_str(
            record
                .get("full_description")
                .and_then(Value::as_str)
                .unwrap_or("No full source description is available in the cached evidence."),
        );
        output.push_str("\n\n### Official documentation\n\n");
        for (key, label) in [
            ("netlib_source_url", "Source"),
            ("netlib_fullsource_url", "Full source"),
            ("netlib_toc_reference", "TOC"),
            ("nist_gams_module_url", "GAMS"),
            ("secondary_html_reference_url", "External docs"),
        ] {
            if let Some(url) = record
                .pointer(&format!("/official_documentation/{key}/url"))
                .and_then(Value::as_str)
            {
                output.push_str(&format!("[{}]({}) ", label, url));
            }
        }
        output.push_str("\n\n### Evidence notes\n\n");
        output.push_str(&format!(
            "Selected from `{}` using `{}`; confidence: `{}`.\n",
            field(record, "description_source"),
            field(record, "description_source_field"),
            field(record, "description_confidence")
        ));
    }
    output
}

fn validate_records(records: &[Value], providers: &BTreeMap<String, Provider>) -> Result<()> {
    let mut names = BTreeSet::new();
    for record in records {
        if !names.insert(name(record).to_owned()) {
            return Err(policy("duplicate canonical key"));
        }
        if field(record, "short_description").is_empty() {
            return Err(policy("record lacks description or unavailable marker"));
        }
        for relationship in record
            .get("provider_relationships")
            .and_then(Value::as_array)
            .unwrap_or(&Vec::new())
        {
            let id = relationship
                .get("provider_id")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if !providers.contains_key(id) {
                return Err(policy("provider reference does not resolve"));
            }
        }
    }
    Ok(())
}
fn validate_docs(root: &Path, records: &[Value]) -> Result<()> {
    if !fs::read_to_string(root.join("index.md"))?.contains("Complete SLATEC Routine Index") {
        return Err(policy(
            "documentation navigation lacks Complete SLATEC Routine Index",
        ));
    }
    let alphabetical = fs::read_to_string(root.join("reference/routines-alphabetical.md"))?;
    let details = fs::read_to_string(root.join("reference/routine-details.md"))?;
    for record in records {
        if !alphabetical.contains(&format!("id=\"{}\"", anchor(name(record)))) {
            return Err(policy("alphabetical anchor missing"));
        }
        if !details.contains(&format!("id=\"{}\"", anchor(name(record)))) {
            return Err(policy("routine detail anchor missing"));
        }
        if record
            .pointer("/official_documentation/netlib_source_url/url")
            .is_some_and(|url| {
                !url.as_str()
                    .unwrap_or_default()
                    .starts_with("https://www.netlib.org/")
            })
        {
            return Err(policy("unverified Netlib source URL"));
        }
    }
    Ok(())
}
fn write_jsons(root: &Path, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    fs::create_dir_all(root)?;
    for (name, bytes) in files {
        fs::write(root.join(name), bytes)?;
    }
    Ok(())
}
fn output_hash(files: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut material = Vec::new();
    for (name, bytes) in files {
        material.extend_from_slice(name.as_bytes());
        material.push(0);
        material.extend_from_slice(bytes);
        material.push(0);
    }
    hash::bytes(&material)
}
fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn json_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}
fn rows(value: &Value) -> Result<&Vec<Value>> {
    value
        .get("records")
        .and_then(Value::as_array)
        .ok_or_else(|| policy("evidence records missing"))
}
fn columns(value: &Value) -> Result<BTreeMap<String, usize>> {
    let mut output = BTreeMap::new();
    for (index, value) in value
        .get("columns")
        .and_then(Value::as_array)
        .ok_or_else(|| policy("evidence columns missing"))?
        .iter()
        .enumerate()
    {
        output.insert(
            value
                .as_str()
                .ok_or_else(|| policy("non-string column"))?
                .to_owned(),
            index,
        );
    }
    Ok(output)
}
fn cell<'a>(row: &'a Value, columns: &BTreeMap<String, usize>, key: &str) -> Result<&'a Value> {
    row.as_array()
        .and_then(|row| columns.get(key).and_then(|index| row.get(*index)))
        .ok_or_else(|| policy("incomplete evidence row"))
}
fn cell_string(row: &Value, columns: &BTreeMap<String, usize>, key: &str) -> Result<String> {
    cell(row, columns, key)?
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| policy("invalid evidence string"))
}
fn cell_strings(row: &Value, columns: &BTreeMap<String, usize>, key: &str) -> Result<Vec<String>> {
    cell(row, columns, key)?
        .as_array()
        .ok_or_else(|| policy("invalid evidence string array"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| policy("invalid evidence array member"))
        })
        .collect()
}
fn cell_usizes(row: &Value, columns: &BTreeMap<String, usize>, key: &str) -> Result<Vec<usize>> {
    cell(row, columns, key)?
        .as_array()
        .ok_or_else(|| policy("invalid evidence number array"))?
        .iter()
        .map(|value| {
            value
                .as_u64()
                .map(|value| value as usize)
                .ok_or_else(|| policy("invalid evidence number"))
        })
        .collect()
}
fn cell_bool(row: &Value, columns: &BTreeMap<String, usize>, key: &str) -> Result<bool> {
    cell(row, columns, key)?
        .as_bool()
        .ok_or_else(|| policy("invalid evidence bool"))
}
fn name(record: &Value) -> &str {
    record
        .get("normalized_name")
        .and_then(Value::as_str)
        .unwrap_or("UNKNOWN")
}
fn field<'a>(record: &'a Value, key: &str) -> &'a str {
    record.get(key).and_then(Value::as_str).unwrap_or("unknown")
}
fn anchor(name: &str) -> String {
    format!(
        "routine-{}",
        name.to_ascii_lowercase()
            .chars()
            .map(|character| if character.is_ascii_alphanumeric() {
                character
            } else {
                '-'
            })
            .collect::<String>()
    )
}
fn compact(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(format!("routine catalogue: {message}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source(text: &str) -> SourceDescription {
        parse_prologue(text).expect("fixture contains a prologue")
    }

    #[test]
    fn anchors_are_stable() {
        assert_eq!(anchor("DAXPY"), "routine-daxpy");
        assert_eq!(anchor("A$1"), "routine-a-1");
    }
    #[test]
    fn main_src_is_preferred_as_canonical_provider() {
        let main = Provider {
            id: "a".to_owned(),
            subset: "main-src".to_owned(),
            relationship: "x".to_owned(),
            path: "a.f".to_owned(),
            raw_hash: "a".to_owned(),
            normalized_hash: "a".to_owned(),
        };
        let live = Provider {
            id: "b".to_owned(),
            subset: "src".to_owned(),
            relationship: "x".to_owned(),
            path: "a.f".to_owned(),
            raw_hash: "a".to_owned(),
            normalized_hash: "a".to_owned(),
        };
        assert_eq!(provider_order(&main, &live), std::cmp::Ordering::Less);
    }

    #[test]
    fn extracts_complete_multiline_purpose_and_description() {
        let description = source(
            "C***BEGIN PROLOGUE TEST\nC***PURPOSE Compute a value from a\nC            two-line field.\nC***DESCRIPTION\nC\nC First paragraph continues\nC across lines.\nC\nC Second paragraph.\nC***REFERENCES x\nC***END PROLOGUE TEST\n      SUBROUTINE TEST\n",
        );
        assert_eq!(
            description.purpose.as_deref(),
            Some("Compute a value from a two-line field.")
        );
        assert_eq!(
            description.full_description.as_deref(),
            Some("First paragraph continues across lines. Second paragraph.")
        );
    }

    #[test]
    fn normalizes_five_line_fixed_form_purpose_and_hyphenation() {
        let description = source(
            "C***PURPOSE A five-line mono-\nC            tonic purpose with\nC            mathematical X(I)\nC            and punctuation\nC            intact.\nC***LIBRARY X\n",
        );
        assert_eq!(
            description.purpose.as_deref(),
            Some("A five-line monotonic purpose with mathematical X(I) and punctuation intact.")
        );
    }

    #[test]
    fn stops_before_parameters_and_executable_code() {
        let description = source(
            "C***DESCRIPTION\nC Useful prose.\nC Description of Parameters\nC X input array.\nC***END PROLOGUE X\n      X = 1\n",
        );
        assert_eq!(
            description.full_description.as_deref(),
            Some("Useful prose.")
        );
    }

    #[test]
    fn stops_before_decorative_legacy_input_heading() {
        let description = source(
            "C***DESCRIPTION\nC Useful legacy prose.\nC *****  I N P U T  ******\nC X is an argument.\nC***END PROLOGUE X\n",
        );
        assert_eq!(
            description.full_description.as_deref(),
            Some("Useful legacy prose.")
        );
    }

    #[test]
    fn supports_legacy_comment_headers() {
        let description = source(
            "C PURPOSE Legacy purpose.\nC DESCRIPTION\nC Legacy explanation.\nC REFERENCES none\n",
        );
        assert_eq!(description.purpose.as_deref(), Some("Legacy purpose."));
        assert_eq!(
            description.full_description.as_deref(),
            Some("Legacy explanation.")
        );
        assert_eq!(description.dialect, "legacy_comment");
    }

    #[test]
    fn parses_multiple_decks_without_cross_routine_leakage() {
        let parsed = parse_source_descriptions(
            "*DECK ONE\nC***PURPOSE First.\nC***END PROLOGUE ONE\n*DECK TWO\nC***PURPOSE Second.\nC***END PROLOGUE TWO\n",
        );
        assert_eq!(
            parsed.get("ONE").and_then(|value| value.purpose.as_deref()),
            Some("First.")
        );
        assert_eq!(
            parsed.get("TWO").and_then(|value| value.purpose.as_deref()),
            Some("Second.")
        );
    }

    #[test]
    fn preserves_subsidiary_and_math_punctuation() {
        let description =
            source("C***PURPOSE Subsidiary to A, B, and C (X(I) .GE. 0).\nC***END PROLOGUE X\n");
        assert_eq!(
            description.purpose.as_deref(),
            Some("Subsidiary to A, B, and C (X(I) .GE. 0).")
        );
    }

    #[test]
    fn recognizes_grouped_toc_variant_names() {
        assert_eq!(
            toc_typed_name("DAXPY-D   with a vector."),
            Some(("DAXPY".to_owned(), "with a vector.".to_owned()))
        );
        assert_eq!(
            toc_typed_name("DAXPY-D"),
            Some(("DAXPY".to_owned(), String::new()))
        );
        assert_eq!(toc_typed_name("not a routine"), None);
    }

    #[test]
    fn normalizes_wrapped_directory_and_toc_text() {
        assert_eq!(
            normalize_description_lines(&[
                "Integrate a function at arbitrarily spaced".to_owned(),
                "abscissas.".to_owned()
            ]),
            "Integrate a function at arbitrarily spaced abscissas."
        );
    }

    #[test]
    fn diagnostics_identify_known_truncation_shapes() {
        assert!(
            mangled_reasons("Compute the value of").contains(&"ends_with_incomplete_connector")
        );
        assert!(mangled_reasons("C***PURPOSE broken").contains(&"contains_fixed_form_marker"));
        assert!(mangled_reasons("f(x").contains(&"unbalanced_parentheses"));
    }
}
