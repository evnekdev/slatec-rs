//! Deterministic evidence for the compiler-free bundled-provider gate.
//!
//! This module intentionally separates facts about selected historical source
//! units from a decision to redistribute a compiled archive.  An absent
//! authored clearance record is never interpreted as a public-domain grant.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SNAPSHOT: &str = "complete-slatec-05078ebcb649b50e4435";
const TARGET: &str = "x86_64-pc-windows-gnu";
const CLASSIFICATIONS: &[&str] = &[
    "us-government-public-domain",
    "explicit-public-domain",
    "permissive-license",
    "copyleft-license",
    "third-party-license",
    "government-provenance-no-explicit-notice",
    "unresolved-provenance",
    "excluded-from-bundle",
];
const ACCEPTED_BUNDLE_CLASSIFICATIONS: &[&str] = &[
    "us-government-public-domain",
    "explicit-public-domain",
    "permissive-license",
];
const OVERLAYS: &[(&str, &str, bool)] = &[
    ("ode-sdrive-expert", "ode-sdrive-source-closure.json", false),
    ("ode-callbacks", "ode-callbacks-source-closure.json", false),
    ("dassl", "dassl-source-closure.json", true),
    (
        "optimization-linear-programming-in-memory",
        "lp-in-memory-source-closure.json",
        false,
    ),
    ("fftpack-real", "fftpack-real-source-closure.json", false),
    (
        "fftpack-complex",
        "fftpack-complex-source-closure.json",
        false,
    ),
    (
        "fishpack-cartesian-2d",
        "fishpack-cartesian-2d-source-closure.json",
        false,
    ),
    (
        "fishpack-pois3d",
        "fishpack-pois3d-source-closure.json",
        false,
    ),
    (
        "fishpack-cylindrical-polar",
        "fishpack-cylindrical-polar-source-closure.json",
        false,
    ),
    (
        "fishpack-spherical",
        "fishpack-spherical-source-closure.json",
        false,
    ),
    (
        "banded-linear-systems",
        "banded-linear-systems-source-closure.json",
        false,
    ),
    ("pchip", "pchip-source-closure.json", false),
    ("bspline", "bspline-source-closure.json", false),
    (
        "piecewise-polynomial",
        "piecewise-polynomial-source-closure.json",
        false,
    ),
    (
        "special-scalar-expanded",
        "special-scalar-expanded-source-closure.json",
        false,
    ),
];

#[derive(Deserialize)]
struct Manifest {
    sources: Vec<Source>,
    families: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Deserialize)]
struct Source {
    id: String,
    subset: String,
    path: String,
    sha256: String,
    #[serde(default)]
    url: Option<String>,
}

#[derive(Deserialize)]
struct FamilyOverlay {
    family: String,
    source_ids: Vec<String>,
    sources: Vec<Source>,
}

#[derive(Default, Deserialize)]
struct Overrides {
    #[serde(default)]
    sources: Vec<Override>,
}

#[derive(Deserialize)]
struct Override {
    id: String,
    sha256: String,
    classification: String,
    confidence: String,
    #[serde(default)]
    named_authors: Vec<String>,
    #[serde(default)]
    stated_institutions: Vec<String>,
    #[serde(default)]
    copyright_notice: Option<String>,
    #[serde(default)]
    license_statement: Option<String>,
    #[serde(default)]
    government_work_evidence: Option<String>,
    #[serde(default)]
    third_party_incorporation_notes: Option<String>,
    #[serde(default)]
    evidence: Vec<String>,
    #[serde(default)]
    unresolved_questions: Vec<String>,
}

#[derive(Deserialize)]
struct SelectedProviders {
    records: Vec<SelectedProvider>,
}

#[derive(Deserialize)]
struct SelectedProvider {
    normalized_name: String,
    source_path: String,
    source_subset: String,
    raw_sha256: String,
}

#[derive(Deserialize)]
struct PrologueIndex {
    columns: Vec<String>,
    records: Vec<Vec<Value>>,
}

/// Generates the source-level provenance, eligibility, SBOM, and carrier gate.
pub fn generate(root: &Path) -> Result<String> {
    let artifacts = artifacts(root)?;
    for (path, bytes) in &artifacts {
        write_if_changed(path, bytes)?;
    }
    Ok(hash::bytes(
        &artifacts
            .iter()
            .flat_map(|(path, bytes)| {
                let mut value = path.to_string_lossy().as_bytes().to_vec();
                value.extend_from_slice(bytes);
                value
            })
            .collect::<Vec<_>>(),
    ))
}

/// Validates that every committed bundled-provider artifact is reproducible.
pub fn validate(root: &Path) -> Result<String> {
    let artifacts = artifacts(root)?;
    for (path, expected) in &artifacts {
        let actual = fs::read(path).map_err(|error| {
            CorpusError::Verification(format!(
                "missing bundled-provider artifact {}: {error}",
                path.display()
            ))
        })?;
        if actual != *expected {
            return Err(CorpusError::Verification(format!(
                "bundled-provider artifact drifted: {}; regenerate with generate-bundled-provider-evidence",
                path.display()
            )));
        }
    }
    Ok(hash::bytes(
        &artifacts
            .iter()
            .flat_map(|(path, bytes)| {
                let mut value = path.to_string_lossy().as_bytes().to_vec();
                value.extend_from_slice(bytes);
                value
            })
            .collect::<Vec<_>>(),
    ))
}

/// Refuses archive production before the source-level redistribution gate is met.
///
/// The command deliberately performs this check before reading a source cache or
/// invoking a compiler, so a blocked bundle cannot accidentally consume local
/// compiler state or make a network-capable build path appear supported.
pub fn require_buildable(root: &Path) -> Result<()> {
    let eligibility: Value = serde_json::from_slice(&fs::read(
        root.join("generated/licensing/bundled-source-eligibility.json"),
    )?)?;
    let eligible = eligibility["summary"]["eligible_source_units"]
        .as_u64()
        .unwrap_or_default();
    let total = eligibility["summary"]["physical_source_units"]
        .as_u64()
        .unwrap_or_default();
    if eligibility["carrier"]["status"] != "ready_for_archive_production" {
        return Err(CorpusError::Policy(format!(
            "bundled archive production is blocked: {eligible} of {total} selected physical source units have an accepted redistribution classification. Add hash-guarded evidence to crates/slatec-src/metadata/bundled-provenance-overrides.json; do not compile or distribute a historical SLATEC archive before then"
        )));
    }
    Err(CorpusError::Policy(
        "bundled archive production has not yet been implemented because no eligible source set has been approved"
            .to_owned(),
    ))
}

fn artifacts(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    let manifest = materialized_manifest(root)?;
    let overrides = read_overrides(root, &manifest)?;
    let routine_names = routine_names(root)?;
    let author_fields = author_fields(root)?;
    let physical = physical_sources(&manifest);
    let mut records = Vec::with_capacity(physical.len());
    let mut classification_counts = BTreeMap::<String, usize>::new();

    for source in physical.values() {
        let override_record = source
            .ids
            .iter()
            .find_map(|id| overrides.get(id))
            .or_else(|| overrides.get(&source.primary_id));
        let classification = override_record
            .map(|record| record.classification.as_str())
            .unwrap_or("unresolved-provenance");
        let bundle_eligible = ACCEPTED_BUNDLE_CLASSIFICATIONS.contains(&classification);
        *classification_counts
            .entry(classification.to_owned())
            .or_default() += 1;
        let key = source_key(&source.subset, &source.path, &source.sha256);
        let units = routine_names.get(&key).cloned().unwrap_or_default();
        let author_field_present = author_fields.contains(&key);
        let originating_package = package_name(&source.subset);
        let default_questions = vec![
            "No hash-guarded, source-specific redistribution grant has been recorded.".to_owned(),
            "Government sponsorship, institutional affiliation, and Netlib hosting are not treated as a public-domain dedication."
                .to_owned(),
        ];
        records.push(json!({
            "selected_source_id":source.primary_id,
            "selected_source_ids":source.ids,
            "path":source.path,
            "source_subset":source.subset,
            "sha256":source.sha256,
            "canonical_upstream_url":source.url,
            "program_units":units,
            "originating_package":originating_package,
            "named_authors":override_record.map(|record| record.named_authors.clone()).unwrap_or_default(),
            "named_authors_status":if override_record.is_some() { "authored_override" } else if author_field_present { "prologue_author_field_present_but_text_not_preserved_in_committed_index" } else { "unavailable" },
            "stated_institutions":override_record.map(|record| record.stated_institutions.clone()).unwrap_or_default(),
            "copyright_notice":override_record.and_then(|record| record.copyright_notice.clone()),
            "license_statement":override_record.and_then(|record| record.license_statement.clone()),
            "government_work_evidence":override_record.and_then(|record| record.government_work_evidence.clone()),
            "third_party_incorporation_notes":override_record.and_then(|record| record.third_party_incorporation_notes.clone()).unwrap_or_else(|| default_third_party_note(&source.subset)),
            "redistribution_classification":classification,
            "confidence":override_record.map(|record| record.confidence.clone()).unwrap_or_else(|| "unreviewed".to_owned()),
            "bundle_eligible":bundle_eligible,
            "evidence":override_record.map(|record| record.evidence.clone()).unwrap_or_else(|| vec!["docs/source-corpus/rights-register.md".to_owned(), "generated/prologues/prologue-index.json".to_owned()]),
            "unresolved_questions":override_record.map(|record| record.unresolved_questions.clone()).unwrap_or(default_questions),
        }));
    }

    let source_by_id = manifest
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let status_by_source = records
        .iter()
        .map(|record| {
            (
                source_key(
                    record["source_subset"].as_str().unwrap_or_default(),
                    record["path"].as_str().unwrap_or_default(),
                    record["sha256"].as_str().unwrap_or_default(),
                ),
                record["bundle_eligible"].as_bool().unwrap_or(false),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut family_records = Vec::new();
    for (family, ids) in &manifest.families {
        let blocked = ids
            .iter()
            .filter_map(|id| source_by_id.get(id.as_str()))
            .filter(|source| {
                !status_by_source
                    .get(&source_key(&source.subset, &source.path, &source.sha256))
                    .copied()
                    .unwrap_or(false)
            })
            .map(|source| source.id.clone())
            .collect::<Vec<_>>();
        family_records.push(json!({
            "feature":family,
            "source_unit_count":ids.len(),
            "bundle_available":blocked.is_empty() && !ids.is_empty(),
            "blocked_source_ids":blocked,
            "status":if ids.is_empty() { "no_selected_source_closure" } else if blocked.is_empty() { "eligible_pending_archive" } else { "blocked_by_source_provenance" },
        }));
    }
    let eligible_source_units = records
        .iter()
        .filter(|record| record["bundle_eligible"] == true)
        .count();
    let carrier_status = if eligible_source_units == records.len() && !records.is_empty() {
        "ready_for_archive_production"
    } else {
        "blocked_by_source_provenance"
    };
    let provenance = json!({
        "schema_id":"slatec-rs/slatec-source-provenance",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "classification_vocabulary":CLASSIFICATIONS,
        "records":records,
        "summary":{"physical_source_units":physical.len(),"classification_counts":classification_counts,"legal_status":"evidence inventory, not legal advice"},
    });
    let eligibility = json!({
        "schema_id":"slatec-rs/bundled-source-eligibility",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "target":TARGET,
        "accepted_bundle_classifications":ACCEPTED_BUNDLE_CLASSIFICATIONS,
        "carrier":{"crate":"slatec-bundled-x86_64-pc-windows-gnu","archive":"native/libslatec_bundle.a","status":carrier_status,"archive_sha256":Value::Null,"reason":"No archive is produced until every source selected for a requested family has a hash-guarded accepted provenance classification."},
        "summary":{"physical_source_units":physical.len(),"eligible_source_units":eligible_source_units,"unresolved_source_units":physical.len()-eligible_source_units},
        "families":family_records,
    });
    let runtime_audit = json!({
        "schema_id":"slatec-rs/bundled-runtime-audit",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "target":TARGET,
        "status":"not_applicable_no_redistributable_archive",
        "compiler_profile":Value::Null,
        "compiler_invoked":false,
        "source_cache_read":false,
        "network_access":false,
        "runtime_components":[
            {"component":"libgfortran","static_or_dynamic":"not_assessed","actually_referenced":"not_assessed","distribution_action":"no bundled runtime is present"},
            {"component":"libquadmath","static_or_dynamic":"not_assessed","actually_referenced":"not_assessed","distribution_action":"no bundled runtime is present; do not add it speculatively"},
            {"component":"libgcc","static_or_dynamic":"not_assessed","actually_referenced":"not_assessed","distribution_action":"no bundled runtime is present"}
        ],
        "imported_dlls":[],
        "reason":"The source-level provenance gate blocks archive production before a compiler or linker can run. Source-build runtime observations remain separate evidence and are not attributed to bundled mode."
    });
    let archive_audit = json!({
        "schema_id":"slatec-rs/bundled-archive-audit",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "target":TARGET,
        "status":"not_applicable_no_redistributable_archive",
        "archive_members":[],
        "defined_symbols":[],
        "undefined_symbols":[],
        "imported_dlls":[],
        "reason":"No archive is produced while the generated eligibility report blocks every selected historical source unit."
    });
    let unresolved = provenance["records"]
        .as_array()
        .expect("provenance records")
        .iter()
        .filter(|record| !record["bundle_eligible"].as_bool().unwrap_or(false))
        .cloned()
        .collect::<Vec<_>>();
    let sbom = json!({
        "spdxVersion":"SPDX-2.3",
        "dataLicense":"CC0-1.0",
        "SPDXID":"SPDXRef-DOCUMENT",
        "name":"slatec-rs bundled provider source inventory",
        "documentNamespace":format!("https://github.com/evnekdev/slatec-rs/spdx/{SNAPSHOT}"),
        "creationInfo":{"creators":["Tool: slatec-corpus"],"created":"1970-01-01T00:00:00Z"},
        "comment":"This is a source inventory, not a claim that an archive is redistributable.",
        "files":provenance["records"].as_array().expect("provenance records").iter().map(|record| json!({
            "SPDXID":format!("SPDXRef-Source-{}",record["selected_source_id"].as_str().unwrap_or("unknown")),
            "fileName":format!("{}/{}",record["source_subset"].as_str().unwrap_or("unknown"),record["path"].as_str().unwrap_or("unknown")),
            "checksums":[{"algorithm":"SHA256","checksumValue":record["sha256"]}],
            "licenseConcluded":"NOASSERTION",
            "licenseInfoInFiles":["NOASSERTION"],
            "copyrightText":"NOASSERTION",
            "comment":format!("Redistribution classification: {}",record["redistribution_classification"].as_str().unwrap_or("unresolved-provenance")),
        })).collect::<Vec<_>>(),
    });
    let mut artifacts = BTreeMap::new();
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/slatec-source-provenance.json"),
        &provenance,
    )?;
    insert_markdown(
        &mut artifacts,
        root.join("generated/licensing/slatec-source-provenance.md"),
        provenance_markdown(&provenance),
    );
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/bundled-source-eligibility.json"),
        &eligibility,
    )?;
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/bundled-runtime-audit.json"),
        &runtime_audit,
    )?;
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/bundled-archive-audit.json"),
        &archive_audit,
    )?;
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/unresolved-provenance.json"),
        &json!({"schema_id":"slatec-rs/unresolved-provenance","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"records":unresolved}),
    )?;
    insert_markdown(
        &mut artifacts,
        root.join("generated/licensing/third-party-notices.md"),
        third_party_notices(&provenance),
    );
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/bundled-sbom.spdx.json"),
        &sbom,
    )?;
    insert_json(
        &mut artifacts,
        root.join("crates/slatec-src/metadata/bundled-source-eligibility.json"),
        &eligibility,
    )?;
    insert_json(
        &mut artifacts,
        root.join("crates/slatec-bundled-x86_64-pc-windows-gnu/metadata/bundle-manifest.json"),
        &json!({
            "schema_id":"slatec-rs/bundled-carrier-manifest","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"target":TARGET,
            "archive":"native/libslatec_bundle.a","archive_sha256":Value::Null,"status":carrier_status,
            "source_unit_count":eligible_source_units,"source_eligibility":"generated/licensing/bundled-source-eligibility.json",
            "runtime_audit":"generated/licensing/bundled-runtime-audit.json","archive_audit":"generated/licensing/bundled-archive-audit.json",
            "reason":"No historical SLATEC archive is included until the source-level provenance audit admits every selected source for a supported family."
        }),
    )?;
    Ok(artifacts)
}

fn materialized_manifest(root: &Path) -> Result<Manifest> {
    let metadata = root.join("crates/slatec-src/metadata");
    let mut manifest: Manifest =
        serde_json::from_slice(&fs::read(metadata.join("family-source-closure.json"))?)?;
    for (family, file, replace_existing) in OVERLAYS {
        if manifest.families.contains_key(*family) && !replace_existing {
            continue;
        }
        let path = metadata.join(file);
        if !path.is_file() {
            continue;
        }
        let overlay: FamilyOverlay = serde_json::from_slice(&fs::read(path)?)?;
        if overlay.family != *family {
            return Err(policy(&format!(
                "bundled-provider overlay {file} has an unexpected family"
            )));
        }
        for source in overlay.sources {
            if let Some(existing) = manifest
                .sources
                .iter()
                .find(|existing| existing.id == source.id)
            {
                if existing.subset != source.subset
                    || existing.path != source.path
                    || existing.sha256 != source.sha256
                {
                    return Err(policy(&format!(
                        "bundled-provider overlay disagrees about source {}",
                        source.id
                    )));
                }
            } else {
                manifest.sources.push(source);
            }
        }
        manifest.families.insert(overlay.family, overlay.source_ids);
    }
    Ok(manifest)
}

fn read_overrides(root: &Path, manifest: &Manifest) -> Result<BTreeMap<String, Override>> {
    let input = root.join("crates/slatec-src/metadata/bundled-provenance-overrides.json");
    let overrides: Overrides = serde_json::from_slice(&fs::read(input)?)?;
    let known = manifest
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source.sha256.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut result = BTreeMap::new();
    for record in overrides.sources {
        if !CLASSIFICATIONS.contains(&record.classification.as_str()) {
            return Err(policy(&format!(
                "unknown bundled provenance classification {}",
                record.classification
            )));
        }
        let expected = known.get(record.id.as_str()).ok_or_else(|| {
            policy(&format!(
                "bundled provenance override references unknown source {}",
                record.id
            ))
        })?;
        if *expected != record.sha256 {
            return Err(policy(&format!(
                "bundled provenance override hash changed for {}",
                record.id
            )));
        }
        if result.insert(record.id.clone(), record).is_some() {
            return Err(policy(
                "bundled provenance overrides contain a duplicate source id",
            ));
        }
    }
    Ok(result)
}

fn routine_names(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let providers: SelectedProviders = serde_json::from_slice(&fs::read(
        root.join("generated/selected-corpus/selected-providers.json"),
    )?)?;
    let mut result = BTreeMap::<String, BTreeSet<String>>::new();
    for record in providers.records {
        result
            .entry(source_key(
                &record.source_subset,
                &record.source_path,
                &record.raw_sha256,
            ))
            .or_default()
            .insert(record.normalized_name);
    }
    Ok(result
        .into_iter()
        .map(|(key, names)| (key, names.into_iter().collect()))
        .collect())
}

fn author_fields(root: &Path) -> Result<BTreeSet<String>> {
    let index: PrologueIndex = serde_json::from_slice(&fs::read(
        root.join("generated/prologues/prologue-index.json"),
    )?)?;
    let source_path = column(&index.columns, "source_member_path")?;
    let source_hash = column(&index.columns, "source_sha256")?;
    let fields = column(&index.columns, "field_canonical_names")?;
    let mut result = BTreeSet::new();
    for row in index.records {
        let Some(path) = row.get(source_path).and_then(Value::as_str) else {
            continue;
        };
        let Some(hash) = row.get(source_hash).and_then(Value::as_str) else {
            continue;
        };
        let has_author = row
            .get(fields)
            .and_then(Value::as_array)
            .is_some_and(|names| names.iter().any(|name| name == "author"));
        if has_author {
            for subset in ["main-src", "fnlib", "lin", "fishfft", "pchip"] {
                result.insert(source_key(subset, path, hash));
            }
        }
    }
    Ok(result)
}

fn column(columns: &[String], name: &str) -> Result<usize> {
    columns
        .iter()
        .position(|column| column == name)
        .ok_or_else(|| policy(&format!("prologue index lacks column {name}")))
}

struct PhysicalSource {
    primary_id: String,
    ids: Vec<String>,
    subset: String,
    path: String,
    sha256: String,
    url: Option<String>,
}

fn physical_sources(manifest: &Manifest) -> BTreeMap<String, PhysicalSource> {
    let mut result = BTreeMap::new();
    for source in &manifest.sources {
        let key = source_key(&source.subset, &source.path, &source.sha256);
        let entry = result.entry(key).or_insert_with(|| PhysicalSource {
            primary_id: source.id.clone(),
            ids: Vec::new(),
            subset: source.subset.clone(),
            path: source.path.clone(),
            sha256: source.sha256.clone(),
            url: source.url.clone(),
        });
        entry.ids.push(source.id.clone());
        if entry.url.is_none() {
            entry.url = source.url.clone();
        }
    }
    for source in result.values_mut() {
        source.ids.sort();
    }
    result
}

fn source_key(subset: &str, path: &str, sha256: &str) -> String {
    format!("{subset}\u{0}{path}\u{0}{sha256}")
}

fn package_name(subset: &str) -> &'static str {
    match subset {
        "main-src" => "SLATEC main source archive",
        "fnlib" => "SLATEC-hosted FNLIB",
        "lin" => "SLATEC-hosted mixed linear-algebra sources",
        "fishfft" => "SLATEC FISHPACK/FFTPACK source subset",
        "pchip" => "SLATEC PCHIP source subset",
        _ => "unidentified historical source subset",
    }
}

fn default_third_party_note(subset: &str) -> String {
    format!(
        "The {subset} subset is historical source provenance only. It is not treated as a grant covering incorporated third-party routines."
    )
}

fn provenance_markdown(provenance: &Value) -> String {
    let summary = &provenance["summary"];
    let mut output = format!(
        "# SLATEC source provenance\n\n- Snapshot: `{SNAPSHOT}`.\n- Physical source units audited: {}.\n- Legal status: evidence inventory, not legal advice.\n- A source is bundle-eligible only with a hash-guarded authored classification in the accepted policy vocabulary.\n\n## Classification counts\n\n| Classification | Source units | Bundle eligible |\n| --- | ---: | --- |\n",
        summary["physical_source_units"]
    );
    for classification in CLASSIFICATIONS {
        let count = summary["classification_counts"][classification]
            .as_u64()
            .unwrap_or_default();
        let eligible = ACCEPTED_BUNDLE_CLASSIFICATIONS.contains(classification);
        output.push_str(&format!("| `{classification}` | {count} | {eligible} |\n"));
    }
    output.push_str("\nThe complete machine-readable source-level records are in `slatec-source-provenance.json`. Missing author, copyright, licence, or government-work evidence is recorded as unavailable; it is not inferred from Netlib hosting, government sponsorship, or an institutional name.\n");
    output
}

fn third_party_notices(provenance: &Value) -> String {
    let unresolved = provenance["records"]
        .as_array()
        .expect("provenance records")
        .iter()
        .filter(|record| !record["bundle_eligible"].as_bool().unwrap_or(false))
        .count();
    format!(
        "# Third-party notices for a future bundled provider\n\nNo historical SLATEC source or GNU runtime archive is included by this workspace. {unresolved} selected physical source units remain outside a distributable bundle pending source-specific provenance evidence.\n\nWhen a source is cleared, its notice, licence statement, attribution, and hash-guarded evidence must be added to `crates/slatec-src/metadata/bundled-provenance-overrides.json` and regenerated before a carrier archive can be produced. GNU compiler-runtime notices are tracked separately from SLATEC source provenance.\n"
    )
}

fn insert_json(
    artifacts: &mut BTreeMap<PathBuf, Vec<u8>>,
    path: PathBuf,
    value: &Value,
) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    artifacts.insert(path, bytes);
    Ok(())
}

fn insert_markdown(artifacts: &mut BTreeMap<PathBuf, Vec<u8>>, path: PathBuf, content: String) {
    artifacts.insert(path, content.into_bytes());
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<()> {
    if fs::read(path).ok().as_deref() != Some(bytes) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)?;
    }
    Ok(())
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_audit_is_source_hash_guarded_and_currently_blocks_distribution() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let artifacts = artifacts(&root).expect("generate bundled-provider evidence");
        let provenance: Value = serde_json::from_slice(
            artifacts
                .get(&root.join("generated/licensing/slatec-source-provenance.json"))
                .expect("provenance artifact"),
        )
        .expect("valid provenance");
        assert!(
            provenance["summary"]["physical_source_units"]
                .as_u64()
                .unwrap_or_default()
                > 1_000
        );
        assert_eq!(
            provenance["summary"]["classification_counts"]["unresolved-provenance"],
            provenance["summary"]["physical_source_units"]
        );
        let eligibility: Value = serde_json::from_slice(
            artifacts
                .get(&root.join("generated/licensing/bundled-source-eligibility.json"))
                .expect("eligibility artifact"),
        )
        .expect("valid eligibility");
        assert_eq!(
            eligibility["carrier"]["status"],
            "blocked_by_source_provenance"
        );
    }

    #[test]
    fn bundled_is_the_only_canonical_pre_release_provider_name() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let facade =
            fs::read_to_string(root.join("crates/slatec/Cargo.toml")).expect("facade manifest");
        let provider = fs::read_to_string(root.join("crates/slatec-src/Cargo.toml"))
            .expect("provider manifest");
        let build = fs::read_to_string(root.join("crates/slatec-src/build.rs"))
            .expect("provider build script");
        assert!(facade.contains("default = [\"std\", \"bundled\"]"));
        assert!(facade.contains("bundled = [\"slatec-src/bundled\"]"));
        assert!(provider.contains("default = [\"bundled\"]"));
        assert!(!facade.contains("prebuilt ="));
        assert!(!provider.contains("prebuilt ="));
        assert!(build.contains("\"BUNDLED\""));
        assert!(!build.contains("\"PREBUILT\""));
    }
}
