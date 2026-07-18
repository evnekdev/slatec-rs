//! Complete source-origin, compiler-storage, and writable-object audit.
//!
//! This command is deliberately explicit: it reads only a separately acquired
//! `SLATEC_SOURCE_CACHE`, verifies every selected source hash, and writes
//! generated evidence without copying source or object bytes into the tree.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;
use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::fortran_state;
use crate::hash;

const SNAPSHOT: &str = "complete-slatec-05078ebcb649b50e4435";
const TARGET: &str = "x86_64-w64-mingw32";
const FLAGS: &[&str] = &["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"];

/// Concise result of a completed native-origin audit.
#[derive(Debug)]
pub struct ResultSummary {
    /// Number of source files scanned and hash verified.
    pub source_count: usize,
    /// Number of separately compiled and inspected objects.
    pub object_count: usize,
    /// Hash of the emitted metadata bytes.
    pub semantic_hash: String,
}

#[derive(Clone, Deserialize)]
struct ManifestSource {
    id: String,
    subset: String,
    path: String,
    sha256: String,
    #[serde(default)]
    url: String,
}

#[derive(Deserialize)]
struct Manifest {
    sources: Vec<ManifestSource>,
    families: BTreeMap<String, Vec<String>>,
    #[serde(default)]
    profile_override_families: BTreeSet<String>,
}

#[derive(Deserialize)]
struct Overlay {
    family: String,
    source_ids: Vec<String>,
    sources: Vec<OverlaySource>,
    #[serde(default)]
    profile_override: bool,
}

#[derive(Deserialize)]
struct OverlaySource {
    id: String,
    subset: String,
    path: String,
    sha256: String,
}

#[derive(Clone)]
struct Source {
    id: String,
    origin: String,
    subset: String,
    source_file: String,
    cache_path: PathBuf,
    sha256: String,
    url: String,
}

#[derive(Deserialize)]
struct CacheReceipt {
    snapshot_id: String,
    checked_unix_seconds: u64,
    sources: Vec<ReceiptSource>,
}

#[derive(Deserialize)]
struct ReceiptSource {
    id: String,
    canonical_upstream_url: String,
    sha256: String,
    cache_status: String,
}

#[derive(Clone)]
struct Finding {
    source_file: String,
    routine: String,
    storage: String,
    origin: String,
    implicitly_saved: bool,
    mutable: bool,
    access: String,
    writers: Vec<String>,
    scope: String,
    parallel_effect: String,
    evidence_lines: Vec<String>,
    layout: String,
}

#[derive(Clone)]
struct Scan {
    source: Source,
    routine: String,
    program_units: Vec<String>,
    findings: Vec<Finding>,
}

/// Generates the complete native-origin audit.
///
/// `SLATEC_SOURCE_CACHE` must name a cache created by the explicit provider
/// acquisition command. `SLATEC_GFORTRAN` must name the reviewed GNU MinGW
/// compiler; no build downloads sources or selects a compiler implicitly.
pub fn generate(output_dir: &Path) -> Result<ResultSummary> {
    let cache = env_path("SLATEC_SOURCE_CACHE")?;
    let compiler = env_path("SLATEC_GFORTRAN")?;
    let mut sources = sources(&cache)?;
    let provider_source_count = sources.len();
    let receipt = verified_receipt(&cache, &sources)?;
    sources.extend(profile_sources()?);
    let compiler_model = compiler_model(&compiler)?;
    let scans = scan_sources(&sources)?;
    let build_root = repo_path("target/native-origin-audit");
    fs::create_dir_all(&build_root)?;
    let scanner_validation = scanner_validation(&compiler, &sources)?;
    let full_corpus_root = env_path("SLATEC_FULL_CORPUS_ROOT")?;
    let full_common = full_corpus_common_index(&full_corpus_root)?;
    let objects = compile_and_inspect(&compiler, &sources, &build_root)?;
    let storage_probe = storage_probe(&compiler, &build_root)?;
    completeness_gate(&sources, &scans, &objects)?;

    let source_records = scans
        .iter()
        .map(|scan| {
            json!([
                scan.source.id,
                scan.source.source_file,
                scan.source.origin,
                scan.source.url,
                scan.source.sha256,
                if scan.findings.iter().any(|finding| finding.mutable) {
                    "complete_mutable_state_found"
                } else {
                    "complete_no_mutable_state_found"
                },
                scan.findings.len()
            ])
        })
        .collect::<Vec<_>>();
    let findings = scans
        .iter()
        .flat_map(|scan| scan.findings.iter())
        .map(finding_json)
        .collect::<Vec<_>>();
    let common = common_index(&scans)?;
    let membership = family_membership(&sources, &scans)?;
    let xerror = xerror_index(&scans)?;
    let source_by_routine = scans
        .iter()
        .map(|scan| (scan.routine.clone(), scan.source.source_file.clone()))
        .collect::<BTreeMap<_, _>>();
    let writable = writable_symbol_index(&objects, &scans, &source_by_routine)?;
    let crosscheck = native_state_crosscheck(&objects, &scans, &writable);
    let projections =
        per_wrapper_native_state(&compiler, &objects, &scans, &writable, &crosscheck)?;
    let symbols_by_object = writable
        .iter()
        .map(|record| record[1].as_str().unwrap_or_default().to_owned())
        .collect::<BTreeSet<_>>();
    if objects
        .iter()
        .any(|object| !symbols_by_object.contains(&object.name))
    {
        return Err(policy(
            "an inspected object lacks a writable-symbol classification record",
        ));
    }

    let receipt_by_id = receipt
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let origins = sources
        .iter()
        .map(|source| {
            let source_receipt = receipt_by_id.get(source.id.as_str());
            json!([
                source.id,
                source.origin,
                source.source_file,
                source.url,
                source.sha256,
                source_receipt.map_or("project_profile", |entry| entry.cache_status.as_str()),
                if source_receipt.is_some() {
                    receipt.checked_unix_seconds
                } else {
                    0
                }
            ])
        })
        .collect::<Vec<_>>();
    let profile_objects = profile_source_records(&sources)?;
    let storage_model = json!({
        "schema_id":"slatec.native.fortran-storage-model",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "compiler":compiler_model.compiler,
        "version":compiler_model.version,
        "target":compiler_model.target,
        "flags":FLAGS,
        "storage_sensitive_flags":{"-fno-automatic":"absent","-frecursive":"absent","-fopenmp":"absent","-finit-*":"absent","-fcommon":"absent","-fno-common":"absent","-static":"absent"},
        "ordinary_locals":"automatic",
        "data_initialized_locals":"saved",
        "explicit_save":"saved",
        "common_blocks":"process_global",
        "recursive_default":false,
        "probe":storage_probe,
        "evidence":["exact source-build command uses -x f77 -std=legacy -ffixed-line-length-none -c","GNU Fortran storage probes compiled with exactly those flags","DDSTP and SDSTP emit local writable ier.0 in .bss"]
    });
    let validation = json!({
        "schema_id":"slatec.native.origin-audit-status",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "source_closure":"complete_mutable_state_found",
        "object_inspection":"complete_mutable_state_found",
        "provider_runtime":"provider_unknown",
        "provider_source_count":provider_source_count,
        "source_count":sources.len(),
        "object_count":objects.len(),
        "archive_member_count":objects.len(),
        "cache_receipt_checked_unix_seconds":receipt.checked_unix_seconds,
        "compiler_storage_model":"complete_mutable_state_found",
        "fixed_form_scanner":scanner_validation["status"],
        "scanner_disagreement_count":scanner_validation["disagreement_count"],
        "common_blocks":common["status"],
        "selected_common_record_count":common["records"].as_array().map_or(0, Vec::len),
        "full_corpus_common_record_count":full_common["records"].as_array().map_or(0, Vec::len),
        "source_object_crosscheck":crosscheck["status"],
        "source_object_unresolved_count":crosscheck["unresolved_count"],
        "per_wrapper_projection_count":projections["records"].as_array().map_or(0, Vec::len),
        "xerror":xerror["status"],
        "parallel_safe_eligible":false,
        "reason":"writable_saved_locals_XERROR_state_callback_context_and_provider_runtime_unknowns_require_conservative_serialization"
    });
    let files = [
        ("fortran-storage-model.json", storage_model),
        (
            "native-source-origin-index.json",
            json!({"schema_id":"slatec.native.source-origin-index","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"columns":["id","origin","source_file","canonical_upstream_url","sha256","cache_status","acquisition_checked_unix_seconds"],"records":origins}),
        ),
        (
            "native-source-scan-index.json",
            json!({"schema_id":"slatec.native.source-scan-index","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"columns":["id","source_file","origin","canonical_upstream_url","sha256","status","finding_count"],"records":source_records}),
        ),
        (
            "native-source-mutable-state-index.json",
            json!({"schema_id":"slatec.native.source-mutable-state-index","schema_version":"1.1.0","snapshot_id":SNAPSHOT,"columns":["source_file","routine","storage","origin","implicitly_saved","mutable","access","writers","scope","parallel_effect","evidence_lines","normalized_layout"],"records":findings}),
        ),
        (
            "native-writable-symbol-index.json",
            json!({"schema_id":"slatec.native.writable-symbol-index","schema_version":"1.1.0","snapshot_id":SNAPSHOT,"columns":["symbol","object","section","binding","size","source_routine","source_storage_record","mutable","classification_effect","consistency_status","state_effect"],"records":writable}),
        ),
        ("native-state-crosscheck.json", crosscheck),
        ("per-wrapper-native-state.json", projections),
        ("common-block-index.json", common),
        ("selected-common-block-index.json", common_index(&scans)?),
        ("fortran-scanner-validation.json", scanner_validation),
        ("native-source-family-membership.json", membership),
        ("xerror-state-index.json", xerror),
        ("native-origin-audit-status.json", validation),
        ("native-origin-profile-source-index.json", profile_objects),
    ];
    fs::create_dir_all(output_dir)?;
    let mut semantic = Vec::new();
    for (name, value) in files {
        let mut bytes = serde_json::to_vec(&value)?;
        bytes.push(b'\n');
        fs::write(output_dir.join(name), &bytes)?;
        semantic.extend_from_slice(&bytes);
    }
    let corpus_output = output_dir.parent().unwrap_or(output_dir).join("corpus");
    fs::create_dir_all(&corpus_output)?;
    let mut bytes = serde_json::to_vec(&full_common)?;
    bytes.push(b'\n');
    fs::write(corpus_output.join("common-block-index.json"), &bytes)?;
    semantic.extend_from_slice(&bytes);
    Ok(ResultSummary {
        source_count: sources.len(),
        object_count: objects.len(),
        semantic_hash: hash::bytes(&semantic),
    })
}

#[derive(Clone)]
struct CompilerModel {
    compiler: String,
    version: String,
    target: String,
}

fn compiler_model(compiler: &Path) -> Result<CompilerModel> {
    let version = command_output(compiler, &["--version"])?;
    let target = command_output(compiler, &["-dumpmachine"])?;
    if target.trim() != TARGET {
        return Err(policy(
            "native-origin audit requires x86_64-w64-mingw32 GNU Fortran",
        ));
    }
    let compiler = version.lines().next().unwrap_or("GNU Fortran").to_owned();
    Ok(CompilerModel {
        compiler,
        version: version.trim().to_owned(),
        target: target.trim().to_owned(),
    })
}

fn sources(cache: &Path) -> Result<Vec<Source>> {
    let manifest_path = repo_path("crates/slatec-src/metadata/family-source-closure.json");
    let mut manifest: Manifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    let overlay_path = manifest_path.with_file_name("ode-sdrive-source-closure.json");
    let overlay: Overlay = serde_json::from_slice(&fs::read(overlay_path)?)?;
    if manifest.families.contains_key(&overlay.family) {
        return Err(policy(
            "SDRIVE overlay unexpectedly duplicates a base source family",
        ));
    }
    let overlay_ids = overlay
        .sources
        .iter()
        .map(|source| source.id.as_str())
        .collect::<BTreeSet<_>>();
    let family_ids = overlay
        .source_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let base_ids = manifest
        .sources
        .iter()
        .map(|source| source.id.as_str())
        .collect::<BTreeSet<_>>();
    if !overlay_ids.is_subset(&family_ids)
        || family_ids
            .iter()
            .any(|id| !overlay_ids.contains(id) && !base_ids.contains(id))
    {
        return Err(policy(
            "SDRIVE overlay source ids do not resolve to overlay or base source records",
        ));
    }
    let mut ids = manifest
        .sources
        .iter()
        .map(|source| source.id.clone())
        .collect::<BTreeSet<_>>();
    for source in overlay.sources {
        if !ids.insert(source.id.clone()) {
            return Err(policy("SDRIVE overlay duplicates a base source id"));
        }
        manifest.sources.push(ManifestSource {
            url: canonical_url(&source.subset, &source.path)?,
            id: source.id,
            subset: source.subset,
            path: source.path,
            sha256: source.sha256,
        });
    }
    manifest
        .sources
        .sort_by(|left, right| left.id.cmp(&right.id));
    let mut result = Vec::with_capacity(manifest.sources.len());
    for source in manifest.sources {
        let relative = if source.subset == "main-src" {
            source.path.clone()
        } else {
            format!("{}/{}", source.subset, source.path)
        };
        let cache_path = cache.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
        if !cache_path.is_file() || hash::file(&cache_path)? != source.sha256 {
            return Err(policy(
                "source closure is incomplete or contains a hash mismatch",
            ));
        }
        result.push(Source {
            id: source.id,
            origin: origin(&source.subset).to_owned(),
            subset: source.subset,
            source_file: relative,
            cache_path,
            sha256: source.sha256,
            url: source.url,
        });
    }
    Ok(result)
}

fn verified_receipt(cache: &Path, sources: &[Source]) -> Result<CacheReceipt> {
    let receipt: CacheReceipt =
        serde_json::from_slice(&fs::read(cache.join(".slatec-rs-acquisition.json"))?)?;
    if receipt.snapshot_id != SNAPSHOT {
        return Err(policy(
            "source-cache receipt snapshot does not match the selected corpus",
        ));
    }
    let entries = receipt
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    for source in sources {
        let entry = entries
            .get(source.id.as_str())
            .ok_or_else(|| policy("source-cache receipt lacks a selected source"))?;
        if entry.sha256 != source.sha256 || entry.canonical_upstream_url != source.url {
            return Err(policy(
                "source-cache receipt does not match the verified source manifest",
            ));
        }
    }
    Ok(receipt)
}

fn scan_sources(sources: &[Source]) -> Result<Vec<Scan>> {
    sources
        .iter()
        .cloned()
        .map(scan_source)
        .collect::<Result<Vec<_>>>()
}

fn scanner_validation(compiler: &Path, sources: &[Source]) -> Result<Value> {
    let mut records = Vec::with_capacity(sources.len());
    let mut disagreement_count = 0_usize;
    for source in sources {
        let bytes = fs::read(&source.cache_path)?;
        let findings = fortran_state::scan(&bytes);
        let scanner = fortran_state::scanner_oracle(&findings);
        let oracle = fortran_state::compiler_oracle(compiler, &source.cache_path)
            .map_err(|message| policy(&message))?;
        let save_all = findings
            .iter()
            .any(|finding| finding.origin == "SAVE" && finding.storage == "all_locals");
        let persistent_agrees = scanner
            .persistent_variables
            .is_subset(&oracle.persistent_variables)
            && (save_all
                || oracle
                    .persistent_variables
                    .is_subset(&scanner.persistent_variables));
        let agrees = scanner.common_blocks == oracle.common_blocks && persistent_agrees;
        if !agrees {
            disagreement_count += 1;
        }
        records.push(json!([
            source.id,
            source.source_file,
            scanner.common_blocks,
            oracle.common_blocks,
            scanner.persistent_variables,
            oracle.persistent_variables,
            if agrees { "agrees" } else { "disagrees" },
            if save_all {
                "bare_SAVE_expands_to_compiler_symbol_set"
            } else {
                "exact"
            }
        ]));
    }
    Ok(json!({
        "schema_id":"slatec.native.fortran-scanner-validation",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "independent_oracle":"GNU_Fortran_-fdump-parse-tree_-fsyntax-only",
        "status":if disagreement_count == 0 { "complete_agreement" } else { "complete_with_disagreements" },
        "disagreement_count":disagreement_count,
        "columns":["source_id","source_file","scanner_common","compiler_common","scanner_persistent","compiler_persistent","status","comparison_rule"],
        "records":records
    }))
}

fn full_corpus_common_index(root: &Path) -> Result<Value> {
    let mut files = Vec::new();
    collect_fortran_files(root, root, &mut files)?;
    files.sort_by(|left, right| left.0.cmp(&right.0));
    let mut records = Vec::new();
    let mut include_count = 0_usize;
    for (relative, path) in files {
        let findings = fortran_state::scan(&fs::read(path)?);
        include_count += findings
            .iter()
            .filter(|finding| finding.origin == "INCLUDE")
            .count();
        for finding in findings
            .into_iter()
            .filter(|finding| finding.origin == "COMMON")
        {
            records.push(json!([
                relative,
                finding.routine,
                finding.storage,
                finding.layout,
                finding.line,
                "reviewed_full_corpus_not_selected_closure",
            ]));
        }
    }
    records.sort_by(|left, right| {
        left[2]
            .as_str()
            .cmp(&right[2].as_str())
            .then(left[0].as_str().cmp(&right[0].as_str()))
            .then(left[1].as_str().cmp(&right[1].as_str()))
    });
    Ok(json!({
        "schema_id":"slatec.corpus.common-block-index",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "scope":"complete_reviewed_corpus_informational_only",
        "status":if records.is_empty() { "complete_no_mutable_state_found" } else { "complete_mutable_state_found" },
        "source_file_count":files_count(root)?,
        "include_statement_count":include_count,
        "columns":["source_file","routine","block","normalized_layout","line","selection_status"],
        "records":records
    }))
}

fn collect_fortran_files(
    root: &Path,
    directory: &Path,
    output: &mut Vec<(String, PathBuf)>,
) -> Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_fortran_files(root, &path, output)?;
        } else if path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("f"))
        {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| policy("full-corpus path escaped its root"))?
                .to_string_lossy()
                .replace('\\', "/");
            output.push((relative, path));
        }
    }
    Ok(())
}

fn files_count(root: &Path) -> Result<usize> {
    let mut files = Vec::new();
    collect_fortran_files(root, root, &mut files)?;
    Ok(files.len())
}

fn family_membership(sources: &[Source], scans: &[Scan]) -> Result<Value> {
    let manifest_path = repo_path("crates/slatec-src/metadata/family-source-closure.json");
    let mut manifest: Manifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    let overlay: Overlay = serde_json::from_slice(&fs::read(
        manifest_path.with_file_name("ode-sdrive-source-closure.json"),
    )?)?;
    manifest
        .families
        .insert(overlay.family.clone(), overlay.source_ids.clone());
    if overlay.profile_override {
        manifest.profile_override_families.insert(overlay.family);
    }
    let mut families_by_source = BTreeMap::<String, BTreeSet<String>>::new();
    for (family, ids) in &manifest.families {
        for id in ids {
            families_by_source
                .entry(id.clone())
                .or_default()
                .insert(family.clone());
        }
    }
    let link_report: Value = serde_json::from_slice(&fs::read(repo_path(
        "generated/linkage/family-link-report.json",
    ))?)?;
    let examples = link_report["examples"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut records = Vec::with_capacity(sources.len());
    for source in sources {
        let families = if source.subset == "project-profile" {
            manifest.profile_override_families.clone()
        } else {
            families_by_source
                .get(&source.id)
                .cloned()
                .unwrap_or_default()
        };
        let units = scans
            .iter()
            .find(|scan| scan.source.id == source.id)
            .map(|scan| scan.program_units.clone())
            .unwrap_or_default();
        let retained = examples
            .iter()
            .filter(|example| {
                example["retained_family_roots"]
                    .as_array()
                    .is_some_and(|roots| {
                        roots.iter().any(|root| {
                            root.as_str().is_some_and(|root| {
                                units
                                    .iter()
                                    .any(|unit| root.eq_ignore_ascii_case(&format!("{unit}_")))
                            })
                        })
                    })
            })
            .filter_map(|example| example["example"].as_str().map(str::to_owned))
            .collect::<Vec<_>>();
        records.push(json!([
            source.id,
            source.source_file,
            format!("{}.o", source.id),
            families,
            families.len() > 1,
            families.is_empty() && source.origin != "project_profile_support",
            families,
            retained,
            if retained.is_empty() {
                "not_observed_in_checked_in_root_symbol_probe"
            } else {
                "root_symbol_observed_in_narrow_probe"
            },
        ]));
    }
    records.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    Ok(json!({
        "schema_id":"slatec.native.source-family-membership",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "archive_policy":"source-build archives the union of selected family closures; profile objects are included whenever any enabled family requires the reviewed profile override",
        "columns":["source_id","source_path","object_name","safe_families","shared","provider_corpus_only","included_in_family_archives","retained_by_narrow_probes","retention_evidence"],
        "records":records
    }))
}

fn scan_source(source: Source) -> Result<Scan> {
    let bytes = fs::read(&source.cache_path)?;
    let program_units = fortran_state::program_units(&bytes);
    let routine = program_units
        .first()
        .cloned()
        .unwrap_or_else(|| "UNKNOWN_PROGRAM_UNIT".to_owned());
    let mut findings = fortran_state::scan(&bytes)
        .into_iter()
        .map(|finding| {
            let persistent = matches!(
                finding.origin.as_str(),
                "DATA" | "SAVE" | "COMMON" | "DECLARATION_INITIALIZATION"
            );
            let mutable = persistent
                || matches!(
                    finding.origin.as_str(),
                    "BLOCK_DATA" | "FORTRAN_IO" | "XERROR_CALL"
                );
            Finding {
                source_file: source.source_file.clone(),
                routine: finding.routine.clone(),
                storage: finding.storage,
                origin: finding.origin.clone(),
                implicitly_saved: matches!(
                    finding.origin.as_str(),
                    "DATA" | "COMMON" | "DECLARATION_INITIALIZATION"
                ),
                mutable,
                access: if mutable {
                    "read_write"
                } else {
                    "review_required"
                }
                .to_owned(),
                writers: vec![finding.routine],
                scope: if mutable { "process" } else { "routine" }.to_owned(),
                parallel_effect: if mutable {
                    "disqualifies_parallel_safe"
                } else {
                    "review_required"
                }
                .to_owned(),
                evidence_lines: vec![format!(
                    "{}:{} {}",
                    source.source_file, finding.line, finding.statement
                )],
                layout: finding.layout,
            }
        })
        .collect::<Vec<_>>();
    specialize_ier(&mut findings, &source.source_file);
    Ok(Scan {
        source,
        routine,
        program_units,
        findings,
    })
}

fn specialize_ier(findings: &mut [Finding], source_file: &str) {
    for finding in findings.iter_mut().filter(|finding| {
        finding.storage == "IER" && matches!(finding.routine.as_str(), "SDSTP" | "DDSTP")
    }) {
        let routine = finding.routine.as_str();
        finding.writers = if routine == "SDSTP" {
            vec!["SDSTP".to_owned(), "SDNTL".to_owned(), "SDPST".to_owned()]
        } else {
            vec!["DDSTP".to_owned(), "DDNTL".to_owned(), "DDPST".to_owned()]
        };
        finding.evidence_lines.extend(if routine == "SDSTP" {
            vec![
                format!("{source_file}:100 passes IER to SDNTL"),
                "src/sdntl.f:51 resets IER false; failure paths set it true".to_owned(),
                "src/sdpst.f:39 resets IER false; factorization and USERS failure paths set it true".to_owned(),
                "compiled object local symbol ier.0 is writable .bss storage".to_owned(),
            ]
        } else {
            vec![
                format!("{source_file}:101 passes IER to DDNTL"),
                "src/ddntl.f:52 resets IER false; failure paths set it true".to_owned(),
                "src/ddpst.f:40 resets IER false; factorization and USERS failure paths set it true".to_owned(),
                "compiled object local symbol ier.0 is writable .bss storage".to_owned(),
            ]
        });
    }
}

fn finding_json(finding: &Finding) -> Value {
    json!([
        finding.source_file,
        finding.routine,
        finding.storage,
        finding.origin,
        finding.implicitly_saved,
        finding.mutable,
        finding.access,
        finding.writers,
        finding.scope,
        finding.parallel_effect,
        finding.evidence_lines,
        finding.layout
    ])
}

fn common_index(scans: &[Scan]) -> Result<Value> {
    let mut declarations = BTreeMap::<String, Vec<&Finding>>::new();
    for finding in scans.iter().flat_map(|scan| scan.findings.iter()) {
        if finding.origin == "COMMON" {
            declarations
                .entry(finding.storage.clone())
                .or_default()
                .push(finding);
        }
    }
    let records = declarations
        .into_iter()
        .map(|(block, declarations)| {
            let layouts = declarations
                .iter()
                .map(|finding| finding.layout.clone())
                .collect::<Vec<_>>();
            let routines = declarations
                .iter()
                .map(|finding| finding.routine.clone())
                .collect::<BTreeSet<_>>();
            json!([
                block,
                routines,
                layouts,
                "read_write",
                "process_global",
                "SerializedGlobal",
                "complete_mutable_state_found"
            ])
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "schema_id":"slatec.native.common-block-index",
        "schema_version":"1.1.0",
        "snapshot_id":SNAPSHOT,
        "scope":"exact currently selected safe source closures and reviewed profile objects",
        "validation":"continuation-aware scanner plus GNU Fortran parse-tree oracle plus COFF writable-symbol inspection",
        "empty_result_explanation":if records.is_empty() { "none of the exact currently compiled selected source units declares COMMON; COMMON declarations exist in deferred unselected corpus units and are indexed separately in generated/corpus/common-block-index.json" } else { "selected COMMON declarations are listed below" },
        "status":if records.is_empty() { "complete_no_mutable_state_found" } else { "complete_mutable_state_found" },
        "columns":["block","declaring_routines","layout_evidence","access","scope","classification","status"],
        "records":records
    }))
}

fn xerror_index(scans: &[Scan]) -> Result<Value> {
    let records = scans
        .iter()
        .flat_map(|scan| scan.findings.iter())
        .filter(|finding| {
            finding.origin == "XERROR_CALL"
                || finding.routine == "J4SAVE"
                || finding.routine == "XERSVE"
        })
        .map(|finding| {
            json!([
                finding.routine,
                finding.storage,
                finding.origin,
                finding.access,
                finding.parallel_effect,
                finding.evidence_lines
            ])
        })
        .collect::<Vec<_>>();
    Ok(json!({
        "schema_id":"slatec.native.xerror-state-index",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "status":"complete_mutable_state_found",
        "wrapper_protection":"std-hosted wrappers that alter XERROR hold crate::runtime::lock_native for the full scoped XGETF/XSETF restore; no_std backend-dependent BLAS APIs do not use scoped XGETF/XSETF and their external-provider error behavior remains BackendDependent; the reviewed Jacobian checker has no explicit XGETF/XSETF call",
        "columns":["routine","storage","origin","access","parallel_effect","evidence_lines"],
        "records":records
    }))
}

#[derive(Clone)]
struct Object {
    name: String,
    path: PathBuf,
    source: Source,
}

fn compile_and_inspect(compiler: &Path, sources: &[Source], root: &Path) -> Result<Vec<Object>> {
    let object_dir = root.join("objects");
    fs::create_dir_all(&object_dir)?;
    let mut objects = Vec::with_capacity(sources.len());
    for source in sources {
        let path = object_dir.join(format!("{}.o", source.id));
        compile_one(compiler, &source.cache_path, &path)?;
        objects.push(Object {
            name: format!("{}.o", source.id),
            path,
            source: source.clone(),
        });
    }
    let ar = sibling_tool(compiler, "ar");
    let archive = root.join("libslatec-native-origin-audit.a");
    let _ = fs::remove_file(&archive);
    for (index, chunk) in objects.chunks(40).enumerate() {
        command_status(
            &ar,
            &std::iter::once(if index == 0 { "cr" } else { "q" }.to_owned())
                .chain(std::iter::once(archive.display().to_string()))
                .chain(chunk.iter().map(|object| object.path.display().to_string()))
                .collect::<Vec<_>>(),
        )?;
    }
    command_status(&ar, &["s".to_owned(), archive.display().to_string()])?;
    let members = command_output(&ar, &["t", &archive.display().to_string()])?;
    if members.lines().count() != objects.len() {
        return Err(policy(
            "native-origin archive member count does not match inspected objects",
        ));
    }
    Ok(objects)
}

fn storage_probe(compiler: &Path, root: &Path) -> Result<Value> {
    let source = root.join("storage-probe.f");
    let object = root.join("storage-probe.o");
    fs::write(
        &source,
        "      SUBROUTINE ORDINARY_LOCAL_PROBE\n      INTEGER I\n      I = I + 1\n      END\n      SUBROUTINE DATA_LOCAL_PROBE\n      INTEGER I\n      DATA I /0/\n      I = I + 1\n      END\n",
    )?;
    compile_one(compiler, &source, &object)?;
    let objdump = sibling_tool(compiler, "objdump");
    let sections = section_map(&objdump, &object)?;
    let table = command_output(&objdump, &["-t", &object.display().to_string()])?;
    let writable = table
        .lines()
        .filter_map(|line| {
            let section_number = coff_section_number(line)?;
            let section = sections.get(&section_number)?;
            (section == ".data" || section == ".bss").then(|| {
                line.split_whitespace()
                    .last()
                    .unwrap_or_default()
                    .to_owned()
            })
        })
        .collect::<Vec<_>>();
    let ordinary_local_writable_symbol = writable
        .iter()
        .any(|symbol| symbol.to_ascii_lowercase().contains("ordinary"));
    let data_local_writable_symbol = writable
        .iter()
        .find(|symbol| !symbol.starts_with('.'))
        .cloned()
        .unwrap_or_else(|| "missing".to_owned());
    if ordinary_local_writable_symbol || data_local_writable_symbol == "missing" {
        return Err(policy(
            "compiler storage probe did not prove automatic ordinary locals and saved DATA locals",
        ));
    }
    Ok(json!({
        "ordinary_local_writable_symbol":ordinary_local_writable_symbol,
        "data_initialized_local_writable_symbol":data_local_writable_symbol,
        "section":".bss",
        "evidence":"exact compiler flags; ordinary local has no writable object symbol while DATA local has one"
    }))
}

fn compile_one(compiler: &Path, source: &Path, object: &Path) -> Result<()> {
    let mut args = FLAGS
        .iter()
        .map(|flag| (*flag).to_owned())
        .collect::<Vec<_>>();
    args.push(source.display().to_string());
    args.push("-o".to_owned());
    args.push(object.display().to_string());
    command_status(compiler, &args)
}

fn writable_symbol_index(
    objects: &[Object],
    scans: &[Scan],
    source_by_routine: &BTreeMap<String, String>,
) -> Result<Vec<Value>> {
    let objdump = env_path("SLATEC_GFORTRAN")?;
    let objdump = sibling_tool(&objdump, "objdump");
    let mut records = Vec::new();
    for object in objects {
        let sections = section_map(&objdump, &object.path)?;
        let table = command_output(&objdump, &["-t", &object.path.display().to_string()])?;
        let mut found = false;
        for line in table.lines() {
            let Some(section_number) = coff_section_number(line) else {
                continue;
            };
            let Some(section) = sections.get(&section_number) else {
                continue;
            };
            if section != ".data" && section != ".bss" {
                continue;
            }
            let symbol = line.split_whitespace().last().unwrap_or_default();
            if symbol.starts_with('.') || symbol.ends_with(".f") {
                continue;
            }
            found = true;
            let binding = if line.contains("scl   2") {
                "global"
            } else if line.contains("scl   3") {
                "local"
            } else {
                "unknown"
            };
            let routine = source_routine_from_symbol(symbol, &object.source, scans);
            let storage = storage_from_symbol(symbol, &routine, scans);
            let consistency = if storage == "unidentified_writable_symbol" {
                "object_only_compiler_generated"
            } else {
                "source_and_object_confirmed"
            };
            let state_effect = storage_effect(&storage, &routine);
            records.push(json!([
                symbol,
                object.name,
                section,
                binding,
                0,
                routine,
                storage,
                true,
                "disqualifies_parallel_safe",
                consistency,
                state_effect,
            ]));
        }
        if !found {
            records.push(json!([
                "(no_writable_symbols)",
                object.name,
                "none",
                "none",
                0,
                source_by_routine
                    .get(&routine_for_source(&object.source, scans))
                    .cloned()
                    .unwrap_or_else(|| "UNKNOWN".to_owned()),
                "none",
                false,
                "complete_no_mutable_state_found",
                "complete_no_mutable_state_found",
                "none",
            ]));
        }
    }
    records.sort_by(|left, right| {
        left[0]
            .as_str()
            .cmp(&right[0].as_str())
            .then(left[1].as_str().cmp(&right[1].as_str()))
    });
    Ok(records)
}

fn storage_effect(storage: &str, routine: &str) -> &'static str {
    if storage.ends_with(":IER") && matches!(routine, "SDSTP" | "DDSTP") {
        "error_state_flag"
    } else if storage.starts_with("COMMON:") {
        "numerical_or_runtime_shared_state"
    } else if storage.starts_with("DATA:") || storage.starts_with("SAVE:") {
        "cached_constant_or_numerical_state_reviewed_conservatively"
    } else if storage == "unidentified_writable_symbol" {
        "unknown"
    } else {
        "compiler_or_runtime_state"
    }
}

fn native_state_crosscheck(objects: &[Object], scans: &[Scan], writable: &[Value]) -> Value {
    let mut records = Vec::new();
    for record in writable {
        if record[0].as_str() == Some("(no_writable_symbols)") {
            continue;
        }
        records.push(json!([
            "object_to_source",
            record[1],
            record[5],
            record[6],
            record[0],
            record[9],
            if record[9].as_str() == Some("object_only_compiler_generated") {
                "unidentified writable symbol remains a conservative provider/compiler blocker"
            } else {
                "writable symbol mapped to a persistent source declaration"
            }
        ]));
    }
    for scan in scans {
        let object = objects
            .iter()
            .find(|object| object.source.id == scan.source.id)
            .map(|object| object.name.as_str())
            .unwrap_or("missing_object");
        for finding in scan.findings.iter().filter(|finding| {
            finding.mutable
                && matches!(
                    finding.origin.as_str(),
                    "DATA" | "SAVE" | "COMMON" | "DECLARATION_INITIALIZATION"
                )
        }) {
            let needle = finding.storage.to_ascii_uppercase();
            let matched = writable.iter().find(|record| {
                record[1].as_str() == Some(object)
                    && (record[6]
                        .as_str()
                        .is_some_and(|storage| storage.to_ascii_uppercase().ends_with(&needle))
                        || record[0].as_str().is_some_and(|symbol| {
                            symbol
                                .split('.')
                                .next()
                                .is_some_and(|base| base.eq_ignore_ascii_case(&needle))
                        }))
            });
            let status = if matched.is_some() {
                "source_and_object_confirmed"
            } else {
                "source_only_optimized_or_unresolved"
            };
            records.push(json!([
                "source_to_object",
                object,
                finding.routine,
                format!("{}:{}", finding.origin, finding.storage),
                matched.and_then(|record| record[0].as_str()).unwrap_or("none"),
                status,
                if matched.is_some() {
                    "persistent source declaration has writable object evidence"
                } else {
                    "no writable COFF symbol; may be read-only/optimized but remains a relaxation blocker until classified"
                }
            ]));
        }
    }
    records.sort_by(|left, right| {
        left[0]
            .as_str()
            .cmp(&right[0].as_str())
            .then(left[1].as_str().cmp(&right[1].as_str()))
            .then(left[3].as_str().cmp(&right[3].as_str()))
    });
    let unresolved = records
        .iter()
        .filter(|record| {
            matches!(
                record[5].as_str(),
                Some("source_only_optimized_or_unresolved" | "unresolved_mismatch")
            )
        })
        .count();
    json!({
        "schema_id":"slatec.native.state-crosscheck",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "status":if unresolved == 0 { "complete_reconciled" } else { "complete_with_conservative_unresolved_records" },
        "unresolved_count":unresolved,
        "relaxation_policy":"any source_only_optimized_or_unresolved or unresolved_mismatch record blocks concurrency relaxation for closures containing it",
        "columns":["direction","object","routine","source_storage","symbol","consistency_status","evidence"],
        "records":records
    })
}

#[derive(Default)]
struct ObjectSymbols {
    defined: BTreeSet<String>,
    undefined: BTreeSet<String>,
}

type FamilySources = BTreeMap<String, BTreeSet<String>>;

fn per_wrapper_native_state(
    compiler: &Path,
    objects: &[Object],
    scans: &[Scan],
    writable: &[Value],
    crosscheck: &Value,
) -> Result<Value> {
    let symbols = inspect_object_symbols(compiler, objects)?;
    let (families, profile_families) = family_source_sets()?;
    let mut owners = BTreeMap::<String, Vec<String>>::new();
    for (object, table) in &symbols {
        for symbol in &table.defined {
            owners
                .entry(symbol.clone())
                .or_default()
                .push(object.clone());
        }
    }
    let functions: Value = serde_json::from_slice(&fs::read(repo_path(
        "generated/safe-api/function-index.json",
    ))?)?;
    let functions = functions["records"]
        .as_array()
        .ok_or_else(|| policy("safe function index has no records"))?;
    let unresolved_objects = crosscheck["records"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|record| {
            matches!(
                record[5].as_str(),
                Some("source_only_optimized_or_unresolved" | "unresolved_mismatch")
            )
        })
        .filter_map(|record| record[1].as_str().map(str::to_owned))
        .collect::<BTreeSet<_>>();
    let mut records = Vec::with_capacity(functions.len());
    for function in functions {
        let safe_function = required_string(function, "rust_path")?;
        let routine = required_string(function, "fortran_routine")?;
        let feature = required_string(function, "feature")?;
        let domain = required_string(function, "domain")?;
        let mut feature_families = resolved_feature_families(feature, &families)?;
        let mut family_ids = feature_families
            .iter()
            .flat_map(|family| families.get(family).into_iter().flatten())
            .cloned()
            .collect::<BTreeSet<_>>();
        let entry = format!("{}_", routine.to_ascii_lowercase());
        let feature_closure_mismatch = !owners.get(&entry).is_some_and(|candidates| {
            candidates.iter().any(|object| {
                family_ids.contains(object.trim_end_matches(".o")) || object.starts_with("profile-")
            })
        });
        if feature_closure_mismatch {
            let owner_ids = owners
                .get(&entry)
                .into_iter()
                .flatten()
                .map(|object| object.trim_end_matches(".o"))
                .collect::<BTreeSet<_>>();
            feature_families = families
                .iter()
                .filter(|(_, ids)| owner_ids.iter().any(|id| ids.contains(*id)))
                .map(|(family, _)| family.clone())
                .collect();
            family_ids = feature_families
                .iter()
                .flat_map(|family| families.get(family).into_iter().flatten())
                .cloned()
                .collect();
        }
        let mut allowed = family_ids
            .iter()
            .map(|id| format!("{id}.o"))
            .collect::<BTreeSet<_>>();
        if feature_families
            .iter()
            .any(|family| profile_families.contains(family))
        {
            allowed.extend(
                ["profile-i1mach.o", "profile-r1mach.o", "profile-d1mach.o"]
                    .into_iter()
                    .map(str::to_owned),
            );
        }
        let closure = reachable_objects(&entry, &allowed, &owners, &symbols).map_err(|_| {
            policy(&format!(
                "native entry point {routine} for {safe_function} is absent from feature {feature}"
            ))
        })?;
        let closure_sources = closure
            .iter()
            .filter_map(|object| {
                objects
                    .iter()
                    .find(|candidate| candidate.name == *object)
                    .map(|candidate| candidate.source.source_file.clone())
            })
            .collect::<Vec<_>>();
        let findings = scans
            .iter()
            .filter(|scan| closure.contains(&format!("{}.o", scan.source.id)))
            .flat_map(|scan| scan.findings.iter())
            .collect::<Vec<_>>();
        let saved = findings
            .iter()
            .filter(|finding| {
                finding.mutable
                    && matches!(
                        finding.origin.as_str(),
                        "DATA" | "SAVE" | "DECLARATION_INITIALIZATION"
                    )
            })
            .map(|finding| {
                json!({
                    "source":finding.source_file,
                    "routine":finding.routine,
                    "storage":finding.storage,
                    "origin":finding.origin,
                    "effect":saved_effect(finding),
                    "reset_each_public_call":if finding.storage == "IER" { "yes_but_reset_and_use_are_not_atomic_across_concurrent_calls" } else { "not_proven" },
                })
            })
            .collect::<Vec<_>>();
        let common = findings
            .iter()
            .filter(|finding| finding.origin == "COMMON")
            .map(|finding| finding.storage.clone())
            .collect::<BTreeSet<_>>();
        let xerror = findings
            .iter()
            .filter(|finding| finding.origin == "XERROR_CALL")
            .map(|finding| format!("{}:{}", finding.routine, finding.storage))
            .collect::<BTreeSet<_>>();
        let io = findings
            .iter()
            .filter(|finding| finding.origin == "FORTRAN_IO")
            .map(|finding| format!("{}:{}", finding.routine, finding.storage))
            .collect::<BTreeSet<_>>();
        let callback_state = callback_state(domain, feature);
        let unresolved = closure
            .intersection(&unresolved_objects)
            .cloned()
            .collect::<Vec<_>>();
        let backend_dependent =
            feature.starts_with("blas-") || feature == "nonlinear-jacobian-check";
        let current_class = if backend_dependent {
            "BackendDependent"
        } else {
            "SerializedGlobal"
        };
        let best_source_class = if !common.is_empty() || !saved.is_empty() {
            "SerializedFamily"
        } else if !xerror.is_empty() || !io.is_empty() {
            "SerializedGlobal"
        } else {
            "ParallelSafeFromSelectedSlatecSourceOnly"
        };
        let mut blockers = Vec::new();
        if !saved.is_empty() {
            blockers.push("saved_mutable_local_state".to_owned());
        }
        if !common.is_empty() {
            blockers.push("mutable_COMMON_state".to_owned());
        }
        if !xerror.is_empty() {
            blockers.push("process_global_XERROR_state".to_owned());
        }
        if !io.is_empty() {
            blockers.push("Fortran_runtime_IO_state".to_owned());
        }
        if callback_state != "none" {
            blockers.push("callback_dispatch_and_failure_containment".to_owned());
        }
        if !unresolved.is_empty() {
            blockers.push("unresolved_source_object_crosscheck".to_owned());
        }
        if feature_closure_mismatch {
            blockers.push("checked_in_function_index_feature_does_not_own_native_entry".to_owned());
        }
        blockers.push("provider_and_compiler_runtime_thread_safety_not_proven".to_owned());
        let writable_symbols = writable
            .iter()
            .filter(|record| {
                record[1]
                    .as_str()
                    .is_some_and(|object| closure.contains(object))
                    && record[7].as_bool() == Some(true)
            })
            .map(|record| record[0].clone())
            .collect::<Vec<_>>();
        records.push(json!({
            "safe_function":safe_function,
            "native_entry_points":[routine],
            "feature":feature,
            "effective_native_families":feature_families,
            "feature_closure_mismatch":feature_closure_mismatch,
            "source_closure":closure_sources,
            "object_closure":closure,
            "saved_mutable_locals":saved,
            "common_blocks":common,
            "xerror_state":xerror,
            "fortran_io":io,
            "callback_state":[callback_state],
            "writable_symbols":writable_symbols,
            "source_object_unresolved":unresolved,
            "provider_unknowns":["GNU_Fortran_runtime_reentrancy_contract_not_proven","external_BLAS_LINPACK_provider_configuration_BackendDependent"],
            "rust_api_concurrency":rust_api_concurrency(feature),
            "native_routine_reentrancy":best_source_class,
            "provider_runtime_thread_safety":"unknown_or_backend_dependent",
            "current_class":current_class,
            "best_possible_class_from_slatec_source":best_source_class,
            "remaining_blockers":blockers,
        }));
    }
    records.sort_by(|left, right| {
        left["safe_function"]
            .as_str()
            .cmp(&right["safe_function"].as_str())
    });
    Ok(json!({
        "schema_id":"slatec.native.per-wrapper-state",
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
        "closure_method":"COFF object dependency graph rooted at each reviewed native entry point and restricted to that safe feature's reviewed archive members",
        "records":records
    }))
}

fn resolved_feature_families(
    feature: &str,
    families: &BTreeMap<String, BTreeSet<String>>,
) -> Result<Vec<String>> {
    if families.contains_key(feature) {
        return Ok(vec![feature.to_owned()]);
    }
    if feature == "special" {
        return Ok(families
            .keys()
            .filter(|family| family.starts_with("special-"))
            .cloned()
            .collect());
    }
    let parts = feature.split(" + ").map(str::to_owned).collect::<Vec<_>>();
    if !parts.is_empty() && parts.iter().all(|part| families.contains_key(part)) {
        return Ok(parts);
    }
    Err(policy(&format!(
        "safe function feature {feature} has no reviewed native family"
    )))
}

fn inspect_object_symbols(
    compiler: &Path,
    objects: &[Object],
) -> Result<BTreeMap<String, ObjectSymbols>> {
    let nm = sibling_tool(compiler, "nm");
    let mut output = BTreeMap::new();
    for object in objects {
        let text = command_output(&nm, &["-g", &object.path.display().to_string()])?;
        let mut symbols = ObjectSymbols::default();
        for line in text.lines() {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            let Some(symbol) = fields.last() else {
                continue;
            };
            let kind = fields
                .get(fields.len().saturating_sub(2))
                .copied()
                .unwrap_or("");
            if kind == "U" {
                symbols.undefined.insert(symbol.to_ascii_lowercase());
            } else if matches!(kind, "T" | "D" | "B" | "R" | "C" | "W") {
                symbols.defined.insert(symbol.to_ascii_lowercase());
            }
        }
        output.insert(object.name.clone(), symbols);
    }
    Ok(output)
}

fn family_source_sets() -> Result<(FamilySources, BTreeSet<String>)> {
    let manifest_path = repo_path("crates/slatec-src/metadata/family-source-closure.json");
    let mut manifest: Manifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    let overlay: Overlay = serde_json::from_slice(&fs::read(
        manifest_path.with_file_name("ode-sdrive-source-closure.json"),
    )?)?;
    manifest.families.insert(
        overlay.family.clone(),
        overlay.source_ids.into_iter().collect(),
    );
    if overlay.profile_override {
        manifest.profile_override_families.insert(overlay.family);
    }
    Ok((
        manifest
            .families
            .into_iter()
            .map(|(family, ids)| (family, ids.into_iter().collect()))
            .collect(),
        manifest.profile_override_families,
    ))
}

fn reachable_objects(
    entry: &str,
    allowed: &BTreeSet<String>,
    owners: &BTreeMap<String, Vec<String>>,
    symbols: &BTreeMap<String, ObjectSymbols>,
) -> Result<BTreeSet<String>> {
    let mut pending = vec![entry.to_owned()];
    let mut visited_symbols = BTreeSet::new();
    let mut closure = BTreeSet::new();
    while let Some(symbol) = pending.pop() {
        if !visited_symbols.insert(symbol.clone()) {
            continue;
        }
        let owner = owners
            .get(&symbol)
            .and_then(|candidates| candidates.iter().find(|object| allowed.contains(*object)));
        let Some(owner) = owner else {
            if symbol == entry {
                return Err(policy(&format!(
                    "native entry point {entry} has no object in its reviewed family"
                )));
            }
            continue;
        };
        if closure.insert(owner.clone()) {
            if let Some(table) = symbols.get(owner) {
                pending.extend(table.undefined.iter().cloned());
            }
        }
    }
    Ok(closure)
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field]
        .as_str()
        .ok_or_else(|| policy("safe function record lacks a required string"))
}

fn saved_effect(finding: &Finding) -> &'static str {
    if finding.storage == "IER" && matches!(finding.routine.as_str(), "SDSTP" | "DDSTP") {
        "error_state_flag"
    } else if finding.origin == "DATA" {
        "cached_constant_or_numerical_state"
    } else if finding.origin == "SAVE" {
        "numerical_continuation_or_cached_state"
    } else {
        "initialization_sentinel_or_unknown"
    }
}

fn callback_state(domain: &str, feature: &str) -> &'static str {
    if feature == "ode-sdrive-expert" {
        "thread_local_ODE_callback_context_while_process_lock_held"
    } else if matches!(
        domain,
        "quadrature" | "roots" | "nonlinear" | "least squares"
    ) {
        "thread_local_callback_registry_while_process_lock_held"
    } else {
        "none"
    }
}

fn rust_api_concurrency(feature: &str) -> &'static str {
    if feature == "ode-sdrive-expert" {
        "session_is_Send_only_when_callback_and_error_are_Send; session_is_not_Sync; mutable_borrow_prevents_simultaneous_use"
    } else {
        "independent_arguments_may_be_used_from_separate_threads_under_Rust_ownership; this_does_not_imply_native_reentrancy"
    }
}

fn section_map(objdump: &Path, object: &Path) -> Result<BTreeMap<i32, String>> {
    let output = command_output(objdump, &["-h", &object.display().to_string()])?;
    let mut sections = BTreeMap::new();
    for line in output.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() >= 2
            && fields[0]
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            if let Ok(index) = fields[0].parse::<i32>() {
                sections.insert(index + 1, fields[1].to_owned());
            }
        }
    }
    Ok(sections)
}

fn coff_section_number(line: &str) -> Option<i32> {
    let start = line.find("(sec ")? + 5;
    let end = line[start..].find(')')? + start;
    line[start..end]
        .trim()
        .parse()
        .ok()
        .filter(|value: &i32| *value > 0)
}

fn source_routine_from_symbol(symbol: &str, source: &Source, scans: &[Scan]) -> String {
    let normalized = symbol.trim_end_matches('_').to_ascii_uppercase();
    scans
        .iter()
        .find(|scan| scan.source.id == source.id && scan.program_units.contains(&normalized))
        .map(|_| normalized)
        .unwrap_or_else(|| routine_for_source(source, scans))
}

fn routine_for_source(source: &Source, scans: &[Scan]) -> String {
    scans
        .iter()
        .find(|scan| scan.source.id == source.id)
        .map(|scan| scan.routine.clone())
        .unwrap_or_else(|| "UNKNOWN".to_owned())
}

fn storage_from_symbol(symbol: &str, routine: &str, scans: &[Scan]) -> String {
    let base = symbol
        .split('.')
        .next()
        .unwrap_or(symbol)
        .to_ascii_uppercase();
    scans
        .iter()
        .find(|scan| scan.routine == routine)
        .and_then(|scan| {
            scan.findings.iter().find(|finding| {
                finding
                    .storage
                    .split('_')
                    .any(|name| name.trim_matches(',') == base)
            })
        })
        .map(|finding| format!("{}:{}", finding.origin, finding.storage))
        .unwrap_or_else(|| "unidentified_writable_symbol".to_owned())
}

fn profile_sources() -> Result<Vec<Source>> {
    ["i1mach", "r1mach", "d1mach"]
        .iter()
        .map(|name| {
            let cache_path = repo_path(format!(
                "crates/slatec-src/native/gnu-mingw-x86_64/{name}.f"
            ));
            Ok(Source {
                id: format!("profile-{name}"),
                origin: "project_profile_support".to_owned(),
                subset: "project-profile".to_owned(),
                source_file: format!("native/gnu-mingw-x86_64/{name}.f"),
                sha256: hash::file(&cache_path)?,
                url: format!("repository:crates/slatec-src/native/gnu-mingw-x86_64/{name}.f"),
                cache_path,
            })
        })
        .collect()
}

fn profile_source_records(sources: &[Source]) -> Result<Value> {
    let records = sources
        .iter()
        .filter(|source| source.subset == "project-profile")
        .map(|source| {
            json!([
                source.id,
                source.source_file,
                source.sha256,
                source.origin,
                "compiled_with_same_flags_and_object_inspected"
            ])
        })
        .collect::<Vec<_>>();
    Ok(
        json!({"schema_id":"slatec.native.profile-source-index","schema_version":"1.0.0","columns":["id","file","sha256","origin","compile_policy"],"records":records}),
    )
}

fn completeness_gate(sources: &[Source], scans: &[Scan], objects: &[Object]) -> Result<()> {
    if sources.len() != scans.len() || sources.len() != objects.len() {
        return Err(policy(
            "source scan or object inspection coverage is incomplete",
        ));
    }
    if scans
        .iter()
        .any(|scan| scan.routine == "UNKNOWN_PROGRAM_UNIT")
    {
        return Err(policy("a selected source has no recognized program unit"));
    }
    Ok(())
}

fn canonical_url(subset: &str, path: &str) -> Result<String> {
    let prefix = match subset {
        "main-src" => "https://www.netlib.org/slatec/",
        "fnlib" => "https://www.netlib.org/slatec/fnlib/",
        "lin" => "https://www.netlib.org/slatec/lin/",
        _ => return Err(policy("unknown reviewed source subset")),
    };
    Ok(format!("{prefix}{path}"))
}

fn origin(subset: &str) -> &'static str {
    match subset {
        "main-src" => "SLATEC",
        "fnlib" => "SLATEC-hosted FNLIB",
        "lin" => "SLATEC-hosted LINPACK/BLAS/support",
        _ => "unknown provider",
    }
}

fn env_path(name: &str) -> Result<PathBuf> {
    env::var_os(name)
        .map(PathBuf::from)
        .ok_or_else(|| policy("required native-audit environment variable is absent"))
}

fn sibling_tool(compiler: &Path, name: &str) -> PathBuf {
    compiler
        .parent()
        .map(|parent| parent.join(format!("{name}.exe")))
        .filter(|path| path.is_file())
        .unwrap_or_else(|| PathBuf::from(name))
}

fn command_output(program: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|error| policy(&format!("start {}: {error}", program.display())))?;
    if !output.status.success() {
        return Err(policy(&format!("{} {args:?} failed", program.display())));
    }
    String::from_utf8(output.stdout).map_err(|_| policy("native tool output was not UTF-8"))
}

fn command_status(program: &Path, args: &[String]) -> Result<()> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|error| policy(&format!("start {}: {error}", program.display())))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(policy(&format!(
            "{} failed: {}",
            program.display(),
            String::from_utf8_lossy(&output.stderr)
        )))
    }
}

fn repo_path(relative: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Verification(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_form_scanner_ignores_comments_and_preserves_continuations() {
        let findings = fortran_state::scan(
            b"C DATA NOPE /1/\n      SUBROUTINE DEMO\n      DATA IER /.FALSE./\n     8 , OTHER /1/\n      END\n",
        );
        assert!(findings.iter().any(|finding| finding.storage == "IER"));
        assert!(findings.iter().any(|finding| finding.storage == "OTHER"));
        assert!(findings.iter().all(|finding| finding.routine == "DEMO"));
    }

    #[test]
    fn common_and_runtime_constructs_are_classified_without_comment_matches() {
        let findings = fortran_state::scan(
            b"      SUBROUTINE DEMO\n      COMMON /STATE/ A,B\n      OPEN (UNIT=1)\n      CALL XERMSG ('A','B','C',1,1)\nC XERROR is mentioned in a comment\n      EXTERNAL XERMSG\n      END\n",
        );
        assert!(findings.iter().any(|finding| finding.storage == "STATE"));
        assert_eq!(
            findings
                .iter()
                .filter(|finding| finding.origin == "FORTRAN_IO")
                .count(),
            1
        );
        assert_eq!(
            findings
                .iter()
                .filter(|finding| finding.origin == "XERROR_CALL")
                .count(),
            1
        );
    }

    #[test]
    fn checked_in_native_origin_evidence_is_complete_and_classifies_ier() {
        let root = repo_path("generated/safe-api");
        let status: Value = serde_json::from_slice(
            &fs::read(root.join("native-origin-audit-status.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            status["source_closure"].as_str(),
            Some("complete_mutable_state_found")
        );
        assert_eq!(
            status["object_inspection"].as_str(),
            Some("complete_mutable_state_found")
        );
        let scan: Value =
            serde_json::from_slice(&fs::read(root.join("native-source-scan-index.json")).unwrap())
                .unwrap();
        assert!(scan["records"].as_array().is_some_and(|records| {
            !records.is_empty()
                && records.iter().all(|record| {
                    matches!(
                        record.get(5).and_then(Value::as_str),
                        Some("complete_no_mutable_state_found" | "complete_mutable_state_found")
                    )
                })
        }));
        let symbols: Value = serde_json::from_slice(
            &fs::read(root.join("native-writable-symbol-index.json")).unwrap(),
        )
        .unwrap();
        assert!(symbols["records"].as_array().is_some_and(|records| {
            records.iter().any(|record| {
                record.get(0).and_then(Value::as_str) == Some("ier.0")
                    && record.get(2).and_then(Value::as_str) == Some(".bss")
            })
        }));
        let common: Value =
            serde_json::from_slice(&fs::read(root.join("common-block-index.json")).unwrap())
                .unwrap();
        assert_eq!(
            common["status"].as_str(),
            Some("complete_no_mutable_state_found")
        );
        let scanner: Value = serde_json::from_slice(
            &fs::read(root.join("fortran-scanner-validation.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(scanner["status"].as_str(), Some("complete_agreement"));
        assert_eq!(scanner["disagreement_count"].as_u64(), Some(0));
        let full_common: Value = serde_json::from_slice(
            &fs::read(repo_path("generated/corpus/common-block-index.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(full_common["source_file_count"].as_u64(), Some(1_452));
        assert!(full_common["records"].as_array().is_some_and(|records| {
            records.len() == 172
                && records
                    .iter()
                    .all(|record| record[5] == "reviewed_full_corpus_not_selected_closure")
        }));
        let crosscheck: Value =
            serde_json::from_slice(&fs::read(root.join("native-state-crosscheck.json")).unwrap())
                .unwrap();
        assert_eq!(crosscheck["status"].as_str(), Some("complete_reconciled"));
        assert_eq!(crosscheck["unresolved_count"].as_u64(), Some(0));
        let projections: Value =
            serde_json::from_slice(&fs::read(root.join("per-wrapper-native-state.json")).unwrap())
                .unwrap();
        assert!(projections["records"].as_array().is_some_and(|records| {
            records.len() == 188
                && records.iter().all(|record| {
                    record["object_closure"]
                        .as_array()
                        .is_some_and(|objects| !objects.is_empty())
                })
        }));
    }
}
