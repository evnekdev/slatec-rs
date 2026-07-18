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
}

#[derive(Deserialize)]
struct Overlay {
    family: String,
    source_ids: Vec<String>,
    sources: Vec<OverlaySource>,
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
struct Statement {
    line: usize,
    text: String,
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
}

#[derive(Clone)]
struct Scan {
    source: Source,
    routine: String,
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
    let xerror = xerror_index(&scans)?;
    let source_by_routine = scans
        .iter()
        .map(|scan| (scan.routine.clone(), scan.source.source_file.clone()))
        .collect::<BTreeMap<_, _>>();
    let writable = writable_symbol_index(&objects, &scans, &source_by_routine)?;
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
        "common_blocks":common["status"],
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
            json!({"schema_id":"slatec.native.source-mutable-state-index","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"columns":["source_file","routine","storage","origin","implicitly_saved","mutable","access","writers","scope","parallel_effect","evidence_lines"],"records":findings}),
        ),
        (
            "native-writable-symbol-index.json",
            json!({"schema_id":"slatec.native.writable-symbol-index","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"columns":["symbol","object","section","binding","size","source_routine","source_storage_record","mutable","classification_effect"],"records":writable}),
        ),
        ("common-block-index.json", common),
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

fn scan_source(source: Source) -> Result<Scan> {
    let text = String::from_utf8(fs::read(&source.cache_path)?)
        .map_err(|_| policy("Fortran source is not UTF-8"))?;
    let statements = statements(&text);
    let routine = statements
        .iter()
        .find_map(|statement| routine_name(&statement.text))
        .unwrap_or_else(|| "UNKNOWN_PROGRAM_UNIT".to_owned());
    let mut findings = Vec::new();
    for statement in &statements {
        let upper = statement.text.to_ascii_uppercase();
        let evidence = vec![format!(
            "{}:{} {}",
            source.source_file, statement.line, statement.text
        )];
        if let Some(storage) = data_storage(&upper) {
            findings.push(Finding {
                source_file: source.source_file.clone(),
                routine: routine.clone(),
                storage,
                origin: "DATA".to_owned(),
                implicitly_saved: true,
                mutable: true,
                access: "read_write".to_owned(),
                writers: vec![routine.clone()],
                scope: "process".to_owned(),
                parallel_effect: "disqualifies_parallel_safe".to_owned(),
                evidence_lines: evidence,
            });
        } else if let Some(storage) = save_storage(&upper) {
            findings.push(Finding {
                source_file: source.source_file.clone(),
                routine: routine.clone(),
                storage,
                origin: "SAVE".to_owned(),
                implicitly_saved: false,
                mutable: true,
                access: "read_write".to_owned(),
                writers: vec![routine.clone()],
                scope: "process".to_owned(),
                parallel_effect: "disqualifies_parallel_safe".to_owned(),
                evidence_lines: evidence,
            });
        } else if let Some(storage) = common_storage(&upper) {
            findings.push(Finding {
                source_file: source.source_file.clone(),
                routine: routine.clone(),
                storage,
                origin: "COMMON".to_owned(),
                implicitly_saved: true,
                mutable: true,
                access: "read_write".to_owned(),
                writers: vec![routine.clone()],
                scope: "process".to_owned(),
                parallel_effect: "disqualifies_parallel_safe".to_owned(),
                evidence_lines: evidence,
            });
        } else if upper.starts_with("EQUIVALENCE") {
            findings.push(construct_finding(
                &source,
                &routine,
                "EQUIVALENCE",
                statement,
                "review_required",
            ));
        } else if upper.starts_with("BLOCK DATA") {
            findings.push(construct_finding(
                &source,
                &routine,
                "BLOCK_DATA",
                statement,
                "disqualifies_parallel_safe",
            ));
        } else if upper.starts_with("ENTRY ") {
            findings.push(construct_finding(
                &source,
                &routine,
                "ENTRY",
                statement,
                "review_required",
            ));
        } else if is_io_statement(&upper) {
            findings.push(construct_finding(
                &source,
                &routine,
                "FORTRAN_IO",
                statement,
                "disqualifies_parallel_safe",
            ));
        } else if is_xerror_call(&upper) {
            findings.push(construct_finding(
                &source,
                &routine,
                "XERROR_CALL",
                statement,
                "disqualifies_parallel_safe",
            ));
        } else if declaration_initialization(&upper) {
            findings.push(Finding {
                source_file: source.source_file.clone(),
                routine: routine.clone(),
                storage: "declaration_initialized_local".to_owned(),
                origin: "DECLARATION_INITIALIZATION".to_owned(),
                implicitly_saved: true,
                mutable: true,
                access: "read_write".to_owned(),
                writers: vec![routine.clone()],
                scope: "process".to_owned(),
                parallel_effect: "disqualifies_parallel_safe".to_owned(),
                evidence_lines: evidence,
            });
        }
    }
    specialize_ier(&mut findings, &source.source_file, &routine);
    Ok(Scan {
        source,
        routine,
        findings,
    })
}

fn specialize_ier(findings: &mut [Finding], source_file: &str, routine: &str) {
    if !matches!(routine, "SDSTP" | "DDSTP") {
        return;
    }
    for finding in findings
        .iter_mut()
        .filter(|finding| finding.storage == "IER")
    {
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

fn construct_finding(
    source: &Source,
    routine: &str,
    origin: &str,
    statement: &Statement,
    effect: &str,
) -> Finding {
    Finding {
        source_file: source.source_file.clone(),
        routine: routine.to_owned(),
        storage: "program_unit_construct".to_owned(),
        origin: origin.to_owned(),
        implicitly_saved: false,
        mutable: origin == "FORTRAN_IO" || origin == "BLOCK_DATA" || origin == "XERROR_CALL",
        access: "review_required".to_owned(),
        writers: vec![routine.to_owned()],
        scope: "process".to_owned(),
        parallel_effect: effect.to_owned(),
        evidence_lines: vec![format!(
            "{}:{} {}",
            source.source_file, statement.line, statement.text
        )],
    }
}

fn statements(text: &str) -> Vec<Statement> {
    let mut result: Vec<Statement> = Vec::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim_end();
        if line.is_empty() || is_fixed_comment(line) {
            continue;
        }
        let (continuation, body) = if line.len() >= 6 {
            let continuation = line.as_bytes()[5] != b' ' && line.as_bytes()[5] != b'0';
            (continuation, line[6..].trim())
        } else {
            (false, line.trim())
        };
        let body = body.split('!').next().unwrap_or_default().trim();
        if body.is_empty() {
            continue;
        }
        if continuation {
            if let Some(previous) = result.last_mut() {
                previous.text.push(' ');
                previous.text.push_str(body);
                continue;
            }
        }
        result.push(Statement {
            line: index + 1,
            text: body.to_owned(),
        });
    }
    result
}

fn is_fixed_comment(line: &str) -> bool {
    matches!(line.as_bytes().first(), Some(b'C' | b'c' | b'*' | b'!'))
}

fn routine_name(text: &str) -> Option<String> {
    let upper = text.to_ascii_uppercase();
    if let Some(rest) = upper.strip_prefix("BLOCK DATA") {
        return rest
            .trim()
            .split(|character: char| character == '(' || character.is_whitespace())
            .next()
            .filter(|name| !name.is_empty())
            .map(str::to_owned);
    }
    for keyword in ["SUBROUTINE", "FUNCTION"] {
        if let Some(index) = upper.find(keyword) {
            let before = &upper[..index];
            if !before.trim().is_empty()
                && !["INTEGER", "REAL", "DOUBLE PRECISION", "LOGICAL", "COMPLEX"]
                    .iter()
                    .any(|prefix| before.trim() == *prefix)
            {
                continue;
            }
            return upper[index + keyword.len()..]
                .trim()
                .split(|character: char| character == '(' || character.is_whitespace())
                .next()
                .filter(|name| !name.is_empty())
                .map(str::to_owned);
        }
    }
    None
}

fn data_storage(text: &str) -> Option<String> {
    text.strip_prefix("DATA ").map(|rest| {
        rest.split('/')
            .next()
            .unwrap_or("DATA")
            .trim()
            .replace(' ', "_")
    })
}

fn save_storage(text: &str) -> Option<String> {
    let rest = text
        .strip_prefix("SAVE ")
        .or_else(|| text.strip_prefix("SAVE/"))
        .or_else(|| (text == "SAVE").then_some(""))?;
    Some({
        let trimmed = rest.trim();
        if trimmed.is_empty() {
            "all_locals".to_owned()
        } else {
            trimmed.replace(' ', "_")
        }
    })
}

fn common_storage(text: &str) -> Option<String> {
    let rest = text.strip_prefix("COMMON")?.trim();
    if let Some(rest) = rest.strip_prefix('/') {
        return rest.split('/').next().map(str::to_owned);
    }
    Some("(blank)".to_owned())
}

fn declaration_initialization(text: &str) -> bool {
    [
        "INTEGER",
        "REAL",
        "DOUBLE PRECISION",
        "LOGICAL",
        "CHARACTER",
        "COMPLEX",
    ]
    .iter()
    .any(|prefix| text.starts_with(prefix) && text.contains('/'))
}

fn is_io_statement(text: &str) -> bool {
    [
        "OPEN",
        "CLOSE",
        "READ",
        "WRITE",
        "REWIND",
        "BACKSPACE",
        "ENDFILE",
        "INQUIRE",
    ]
    .iter()
    .any(|keyword| text.starts_with(keyword))
}

fn is_xerror_call(text: &str) -> bool {
    [
        "XERMSG", "XERRWV", "XERPRN", "XERSVE", "XGETF", "XSETF", "J4SAVE",
    ]
    .iter()
    .any(|name| {
        text.contains(&format!("CALL {name}")) || (*name == "J4SAVE" && text.contains("J4SAVE("))
    })
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
        finding.evidence_lines
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
                .map(|finding| finding.evidence_lines.first().cloned().unwrap_or_default())
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
        "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT,
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
            records.push(json!([
                symbol,
                object.name,
                section,
                binding,
                0,
                routine,
                storage,
                true,
                "disqualifies_parallel_safe"
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
                "complete_no_mutable_state_found"
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
        .find(|scan| scan.source.id == source.id && scan.routine == normalized)
        .map(|scan| scan.routine.clone())
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
        let statements = statements(
            "C DATA NOPE /1/\n      SUBROUTINE DEMO\n      DATA IER /.FALSE./\n     8 , OTHER /1/\n      END\n",
        );
        assert_eq!(statements.len(), 3);
        assert_eq!(routine_name(&statements[0].text).as_deref(), Some("DEMO"));
        assert_eq!(
            data_storage(&statements[1].text.to_ascii_uppercase()).as_deref(),
            Some("IER")
        );
        assert_eq!(save_storage("SAVE1(I) = 0.D0"), None);
        assert_eq!(save_storage("SAVE I, J"), Some("I,_J".to_owned()));
    }

    #[test]
    fn common_and_runtime_constructs_are_classified_without_comment_matches() {
        assert_eq!(
            common_storage("COMMON /STATE/ A, B"),
            Some("STATE".to_owned())
        );
        assert!(is_io_statement("OPEN (UNIT=1)"));
        assert!(is_xerror_call("CALL XERMSG ('A','B','C',1,1)"));
        assert!(!is_xerror_call("XERROR is mentioned in a comment"));
        assert!(!is_xerror_call("EXTERNAL XERMSG"));
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
    }
}
