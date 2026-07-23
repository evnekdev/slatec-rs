//! Independent, package-first evidence for the coordinated `0.1.0` release.
//!
//! This module starts from authored manifests, packaged `.crate` archives, and
//! direct hashes so a green report from another generator is not accepted as
//! proof of itself. It transactionally recomputes release-readiness only to
//! detect stale self-referential evidence; that result is never treated as
//! independent proof of semantic documentation quality.

use crate::error::{CorpusError, Result};
use crate::hash;
use crate::release_readiness;
use flate2::read::GzDecoder;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;
use tar::Archive;

const RELEASE_VERSION: &str = "0.1.0";
const CRATES_IO_COMPRESSED_LIMIT_BYTES: u64 = 10 * 1024 * 1024;
const PUBLISH_ORDER: [&str; 6] = [
    "slatec-bundled-x86_64-pc-windows-gnu",
    "slatec-bundled-x86_64-unknown-linux-gnu",
    "slatec-sys",
    "slatec-core",
    "slatec-src",
    "slatec",
];

#[derive(Clone, Debug, serde::Serialize)]
pub struct FinalReleaseAuditResult {
    /// Overall independent-audit result.
    pub status: String,
    /// Number of packaged publishable crates inspected directly.
    pub packages: usize,
    /// Number of release-candidate discrepancies found.
    pub discrepancies: usize,
    /// Number of critical blockers found by the independent checks.
    pub critical_blockers: usize,
    /// Hash of the primary generated evidence document.
    pub semantic_hash: String,
}

/// Generates the independent release-audit outputs from packaged artifacts.
pub fn generate(root: &Path, output: &Path) -> Result<FinalReleaseAuditResult> {
    let reports = reports(root)?;
    write_reports(output, &reports)?;
    result(&reports)
}

/// Recomputes the audit and rejects stale committed evidence.
pub fn validate(root: &Path, output: &Path) -> Result<FinalReleaseAuditResult> {
    let reports = reports(root)?;
    for (name, value) in &reports {
        let expected = if name.ends_with(".md") {
            value["markdown"]
                .as_str()
                .ok_or_else(|| policy("markdown output has no markdown field"))?
                .as_bytes()
                .to_vec()
        } else {
            json_bytes(value)?
        };
        let path = output.join(name);
        let actual = fs::read(&path).map_err(|error| {
            policy(&format!(
                "independent release-audit output {} is missing: {error}",
                path.display()
            ))
        })?;
        if actual != expected {
            return Err(policy(&format!(
                "independent release-audit output {} is stale; run generate-final-release-audit",
                path.display()
            )));
        }
    }
    let result = result(&reports)?;
    if result.status != "pass" {
        return Err(policy(&format!(
            "independent final-release audit is {}; resolve its discrepancies and critical blockers before treating the release check as passed",
            result.status
        )));
    }
    Ok(result)
}

fn reports(root: &Path) -> Result<BTreeMap<String, Value>> {
    let canonical_root = root.canonicalize()?;
    let root = canonical_root.as_path();
    let safe = audit_baseline(
        root,
        "public-api-freeze-baseline.json",
        "export_count",
        "records",
    )?;
    let raw = audit_baseline(
        root,
        "raw-public-abi-freeze-baseline.json",
        "export_count",
        "records",
    )?;
    let core = audit_core_baseline(root)?;
    let carrier_baseline = audit_carrier_baseline(root)?;
    let package_registry = prepare_package_registry(root)?;
    let packages = PUBLISH_ORDER
        .into_iter()
        .map(|package| audit_package(root, package, &package_registry.0, &package_registry.1))
        .collect::<Result<Vec<_>>>()?;
    let carriers = [
        "slatec-bundled-x86_64-pc-windows-gnu",
        "slatec-bundled-x86_64-unknown-linux-gnu",
    ]
    .into_iter()
    .map(|carrier| audit_carrier(root, carrier))
    .collect::<Result<Vec<_>>>()?;
    let baseline_checks = [
        safe.clone(),
        raw.clone(),
        core.clone(),
        carrier_baseline.clone(),
    ];
    let package_passed = packages
        .iter()
        .all(|record| record["result"].as_str() == Some("pass"));
    let carriers_passed = carriers
        .iter()
        .all(|record| record["result"].as_str() == Some("pass"));
    let baselines_passed = baseline_checks
        .iter()
        .all(|record| record["result"].as_str() == Some("pass"));
    let (_, readiness_freshness) =
        release_readiness::audit_freshness(root, &root.join("generated/release-readiness"))?;
    let mut discrepancies = compare_release_candidate(root, &safe, &raw, &core, &carrier_baseline)?;
    let mut critical_blockers = Vec::<Value>::new();
    if !readiness_freshness.changed_paths.is_empty() {
        let paths = readiness_freshness
            .changed_paths
            .iter()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
            .collect::<Vec<_>>();
        discrepancies.push(json!({
            "gate":"committed release-readiness evidence freshness",
            "reason":"the transactional generator recomputation differs from committed evidence",
            "changed_file_count":paths.len(),
            "sample_changed_files":paths.iter().take(20).collect::<Vec<_>>(),
        }));
        critical_blockers.push(json!({
            "blocker":"Stale release-readiness evidence",
            "severity":"critical",
            "evidence":format!("{} generator-owned files differ after a transactional recomputation", paths.len()),
            "required_action":"Review the semantic documentation changes, correct any generator or authored-input defect, then regenerate and independently review the complete output. Do not accept a bulk regeneration merely to make this audit green.",
            "owner":"maintainers",
        }));
    }
    if !baselines_passed {
        critical_blockers.push(json!({
            "blocker":"Public API or carrier baseline drift",
            "severity":"critical",
            "evidence":"an independently recomputed safe, raw, core, or carrier baseline did not match its committed freeze receipt",
            "required_action":"Review the changed public surface or carrier metadata and either restore the intended baseline or document and approve a deliberate baseline update.",
            "owner":"maintainers",
        }));
    }
    if !package_passed {
        critical_blockers.push(json!({
            "blocker":"Publishable package audit failure",
            "severity":"critical",
            "evidence":"direct inspection of at least one freshly packaged .crate failed a size, licence, dependency, or forbidden-content check",
            "required_action":"Correct the packaged manifest or contents and rerun the independent package-first audit.",
            "owner":"maintainers",
        }));
    }
    if !carriers_passed {
        critical_blockers.push(json!({
            "blocker":"Carrier archive receipt mismatch",
            "severity":"critical",
            "evidence":"direct SHA-256, size, ar, or nm inspection did not match a carrier receipt",
            "required_action":"Investigate the native archive provenance and receipt before attempting publication.",
            "owner":"maintainers",
        }));
    }
    for discrepancy in discrepancies.iter().filter(|record| {
        record["gate"].as_str() != Some("committed release-readiness evidence freshness")
    }) {
        critical_blockers.push(json!({
            "blocker":"Release-candidate discrepancy",
            "severity":"critical",
            "evidence":discrepancy,
            "required_action":"Resolve the mismatch between independently recomputed evidence and the release-candidate report before publication.",
            "owner":"maintainers",
        }));
    }
    let audit_status = if package_passed
        && carriers_passed
        && baselines_passed
        && discrepancies.is_empty()
        && critical_blockers.is_empty()
    {
        "pass"
    } else {
        "fail"
    };
    let package_size = json!({
        "schema_id":"slatec-rs.package-size-audit",
        "schema_version":"1.0.0",
        "method":"cargo package --no-verify followed by direct gzip-tar inspection",
        "crates_io_compressed_limit_bytes":CRATES_IO_COMPRESSED_LIMIT_BYTES,
        "packages":packages,
        "result":if package_passed { "pass" } else { "fail" },
    });
    let trust = json!({
        "schema_id":"slatec-rs.release-check-trust-boundaries",
        "schema_version":"1.0.0",
        "checks":[
            {"check":"safe API baseline","classification":"independently-recomputed","evidence":"current generated safe function index is hashed and counted directly against its committed baseline"},
            {"check":"raw ABI baseline","classification":"independently-recomputed","evidence":"current canonical raw API inventory is hashed and counted directly against its committed baseline"},
            {"check":"core support API baseline","classification":"independently-recomputed","evidence":"the authored core export source is hashed and its explicit exports are counted directly"},
            {"check":"carrier metadata and archive receipts","classification":"independently-recomputed","evidence":"carrier manifests, native archive sizes, and SHA-256 values are read directly; ar and nm are invoked when available"},
            {"check":"publishable package contents","classification":"independently-recomputed","evidence":"fresh cargo package archives are decompressed and inspected without using the package-content audit output"},
            {"check":"registry-only consumers","classification":"externally-observed","evidence":"target-specific CI runs the packaged consumers; this document does not promote a stored simulator report to independent proof"},
            {"check":"source-build parity","classification":"manually-reviewed","evidence":"requires a separately acquired checksum-verified cache and matching GNU Fortran toolchain; it remains an explicit release gate"},
            {"check":"external public API tooling","classification":"externally-observed","evidence":"cargo public-api snapshots and cargo semver-checks invocations are run outside this generator and recorded by the release operator"},
            {"check":"committed release-readiness reports","classification":"self-referential","evidence":"a transactional recomputation identifies stale generated evidence, but semantic documentation still requires independent review before regeneration is accepted"}
        ]
    });
    let blockers = json!({
        "schema_id":"slatec-rs.release-blockers",
        "schema_version":"1.0.0",
        "status":if critical_blockers.is_empty() { "no-critical-blocker-in-independent-local-audit" } else { "critical-blockers-present" },
        "blockers":critical_blockers,
        "manual_release_gates":[
            {"blocker":"Target CI workflow runs","severity":"release gate","evidence":"workflow definition and post-push checks","required_action":"Observe successful Windows GNU and Linux GNU registry-only workflow runs for the selected commit.","owner":"release operator"},
            {"blocker":"Source-build parity","severity":"release gate","evidence":"checksum-verified source cache and matching compiler runtime","required_action":"Run the documented source-build regression profile; do not substitute bundled evidence.","owner":"release operator"},
            {"blocker":"Registry publication propagation","severity":"release operation","evidence":"crates.io index and package availability","required_action":"Use the generated dry-run and real-publication templates in order, waiting for each prerequisite to resolve.","owner":"release operator"},
            {"blocker":"Advisory database review","severity":"release gate","evidence":"current advisory database and Cargo.lock","required_action":"Run cargo audit or the documented equivalent with current advisory data immediately before publication.","owner":"release operator"}
        ]
    });
    let audit = json!({
        "schema_id":"slatec-rs.final-independent-release-audit",
        "schema_version":"1.0.0",
        "release_version":RELEASE_VERSION,
        "publication_order":PUBLISH_ORDER,
        "status":audit_status,
        "baseline_checks":baseline_checks,
        "carrier_archive_checks":carriers,
        "package_count":PUBLISH_ORDER.len(),
        "package_size_evidence":"package-size-audit.json",
        "discrepancies":discrepancies,
        "critical_blockers":critical_blockers,
        "decision":if audit_status == "pass" {
            "The independent local audit passes only its direct evidence. Publication remains conditional on the explicit manual release gates in release-blockers.json; this report never claims an unobserved CI workflow or publication succeeded."
        } else {
            "The independent local audit found discrepancies or critical blockers. Publication is not permitted until they are resolved and the audit is regenerated; unobserved CI and publication outcomes remain manual release gates."
        }
    });
    let markdown = render_markdown(&audit, &packages, &carriers);
    let mut values = BTreeMap::new();
    values.insert("final-independent-audit.json".to_owned(), audit);
    values.insert("package-size-audit.json".to_owned(), package_size);
    values.insert("release-check-trust-boundaries.json".to_owned(), trust);
    values.insert("release-blockers.json".to_owned(), blockers);
    values.insert(
        "final-independent-audit.md".to_owned(),
        json!({"markdown":markdown}),
    );
    Ok(values)
}

fn result(reports: &BTreeMap<String, Value>) -> Result<FinalReleaseAuditResult> {
    let audit = reports
        .get("final-independent-audit.json")
        .ok_or_else(|| policy("independent audit document was not produced"))?;
    let bytes = json_bytes(audit)?;
    Ok(FinalReleaseAuditResult {
        status: audit["status"].as_str().unwrap_or("fail").to_owned(),
        packages: audit["package_count"].as_u64().unwrap_or_default() as usize,
        discrepancies: audit["discrepancies"].as_array().map_or(0, Vec::len),
        critical_blockers: audit["critical_blockers"].as_array().map_or(0, Vec::len),
        semantic_hash: hash::bytes(&bytes),
    })
}

fn audit_baseline(
    root: &Path,
    baseline_name: &str,
    count_key: &str,
    input_collection: &str,
) -> Result<Value> {
    let baseline_path = root.join("generated/release-readiness").join(baseline_name);
    let baseline: Value = serde_json::from_slice(&fs::read(&baseline_path)?)?;
    let input = baseline["input"]
        .as_str()
        .ok_or_else(|| policy(&format!("{baseline_name} has no input path")))?;
    let input_path = root.join(input);
    let input_value: Value = serde_json::from_slice(&fs::read(&input_path)?)?;
    let expected_hash = baseline["input_sha256"].as_str().ok_or_else(|| {
        policy(&format!(
            "{baseline_name} has no source hash; a freeze baseline needs one"
        ))
    })?;
    let actual_hash = hash::file(&input_path)?;
    let expected_count = if count_key == "exports" {
        baseline["exports"].as_array().map_or(0, Vec::len)
    } else {
        baseline[count_key].as_u64().unwrap_or_default() as usize
    };
    let actual_count = input_value[input_collection].as_array().map_or(0, Vec::len);
    let result = expected_hash == actual_hash && expected_count == actual_count;
    Ok(json!({
        "check":baseline["schema_id"],
        "classification":"independently-recomputed",
        "baseline":format!("generated/release-readiness/{baseline_name}"),
        "input":input,
        "expected_sha256":expected_hash,
        "actual_sha256":actual_hash,
        "expected_count":expected_count,
        "actual_count":actual_count,
        "result":if result { "pass" } else { "fail" },
    }))
}

fn audit_carrier_baseline(root: &Path) -> Result<Value> {
    let baseline: Value = serde_json::from_slice(&fs::read(
        root.join("generated/release-readiness/carrier-metadata-api-baseline.json"),
    )?)?;
    let mut records = Vec::new();
    for carrier in baseline["carriers"]
        .as_array()
        .ok_or_else(|| policy("carrier metadata baseline has no carriers"))?
    {
        let crate_name = carrier["crate"]
            .as_str()
            .ok_or_else(|| policy("carrier record has no crate name"))?;
        let manifest_dir = root.join("crates").join(crate_name);
        let cargo_hash = hash::file(&manifest_dir.join("Cargo.toml"))?;
        let bundle_hash = hash::file(&manifest_dir.join("metadata/bundle-manifest.json"))?;
        let passed = carrier["cargo_toml_sha256"].as_str() == Some(cargo_hash.as_str())
            && carrier["bundle_manifest_sha256"].as_str() == Some(bundle_hash.as_str());
        records.push(json!({
            "crate":crate_name,
            "expected_cargo_toml_sha256":carrier["cargo_toml_sha256"],
            "actual_cargo_toml_sha256":cargo_hash,
            "expected_bundle_manifest_sha256":carrier["bundle_manifest_sha256"],
            "actual_bundle_manifest_sha256":bundle_hash,
            "result":if passed { "pass" } else { "fail" },
        }));
    }
    let result = records
        .iter()
        .all(|record| record["result"].as_str() == Some("pass"));
    Ok(json!({
        "check":"slatec-rs.carrier-metadata-api-baseline",
        "classification":"independently-recomputed",
        "baseline":"generated/release-readiness/carrier-metadata-api-baseline.json",
        "expected_count":baseline["carriers"].as_array().map_or(0, Vec::len),
        "actual_count":records.len(),
        "carriers":records,
        "result":if result { "pass" } else { "fail" },
    }))
}

fn audit_core_baseline(root: &Path) -> Result<Value> {
    let baseline_path =
        root.join("generated/release-readiness/core-support-types-api-baseline.json");
    let baseline: Value = serde_json::from_slice(&fs::read(&baseline_path)?)?;
    let input = baseline["input"]
        .as_str()
        .ok_or_else(|| policy("core support baseline has no source input"))?;
    let source = root.join(input);
    let actual_hash = hash::file(&source)?;
    let text = fs::read_to_string(&source)?;
    let actual_count = text
        .lines()
        .filter(|line| {
            let line = line.trim_start();
            line.starts_with("pub fn ")
                || line.starts_with("pub struct ")
                || line.starts_with("pub enum ")
                || line.starts_with("pub type ")
        })
        .count();
    let expected_count = baseline["exports"].as_array().map_or(0, Vec::len);
    let passed = baseline["input_sha256"].as_str() == Some(actual_hash.as_str())
        && expected_count == actual_count;
    Ok(json!({
        "check":baseline["schema_id"],
        "classification":"independently-recomputed",
        "baseline":"generated/release-readiness/core-support-types-api-baseline.json",
        "input":input,
        "expected_sha256":baseline["input_sha256"],
        "actual_sha256":actual_hash,
        "expected_count":expected_count,
        "actual_count":actual_count,
        "result":if passed { "pass" } else { "fail" },
    }))
}

fn prepare_package_registry(root: &Path) -> Result<(std::path::PathBuf, std::path::PathBuf)> {
    let audit_root = root.join("target/final-release-audit");
    if audit_root.exists() {
        fs::remove_dir_all(&audit_root)?;
    }
    let vendor = audit_root.join("vendor");
    let cargo_home = audit_root.join("cargo-home");
    fs::create_dir_all(&vendor)?;
    fs::create_dir_all(&cargo_home)?;
    run(
        root,
        Command::new("cargo")
            .args(["vendor", "--versioned-dirs"])
            .arg(&vendor)
            .env("CARGO_NET_OFFLINE", "true"),
        "cargo vendor for independent package audit",
    )?;
    let config = format!(
        "[source.crates-io]\nreplace-with = \"local-release-vendor\"\n\n[source.local-release-vendor]\ndirectory = {:?}\n\n[net]\noffline = true\n",
        vendor.to_string_lossy()
    );
    fs::write(cargo_home.join("config.toml"), config)?;
    Ok((cargo_home, vendor))
}

fn audit_package(root: &Path, package: &str, cargo_home: &Path, vendor: &Path) -> Result<Value> {
    run(
        root,
        Command::new("cargo")
            .args([
                "package",
                "-p",
                package,
                "--allow-dirty",
                "--no-verify",
                "--offline",
            ])
            .env("CARGO_HOME", cargo_home),
        &format!("cargo package {package}"),
    )?;
    let archive_path = root
        .join("target/package")
        .join(format!("{package}-{RELEASE_VERSION}.crate"));
    let bytes = fs::read(&archive_path)?;
    let archive_hash = hash::bytes(&bytes);
    let unpacked = vendor.join(format!("{package}-{RELEASE_VERSION}"));
    Archive::new(GzDecoder::new(bytes.as_slice())).unpack(vendor)?;
    write_directory_checksum(&unpacked, &archive_hash)?;
    let mut archive = Archive::new(GzDecoder::new(bytes.as_slice()));
    let mut files = BTreeSet::new();
    let mut packaged_manifest = None;
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_string_lossy().replace('\\', "/");
        let relative = path
            .split_once('/')
            .map(|(_, relative)| relative.to_owned())
            .ok_or_else(|| policy(&format!("package member lacks root directory: {path}")))?;
        if relative == "Cargo.toml" {
            let mut manifest = String::new();
            use std::io::Read;
            entry.read_to_string(&mut manifest)?;
            packaged_manifest = Some(manifest);
        }
        files.insert(relative);
    }
    let manifest =
        packaged_manifest.ok_or_else(|| policy(&format!("{package} archive has no Cargo.toml")))?;
    let manifest: toml::Value = toml::from_str(&manifest)?;
    let no_workspace_or_git_dependency = !contains_path_or_git_dependency(&manifest, false);
    let required = ["Cargo.toml", "README.md", "LICENSE-APACHE", "LICENSE-MIT"];
    let missing_required = required
        .into_iter()
        .filter(|file| !files.contains(*file))
        .collect::<Vec<_>>();
    let bundled = package.starts_with("slatec-bundled-");
    let runtime_required = [
        "metadata/runtime-licenses/GPL-3.0.txt",
        "metadata/runtime-licenses/LGPL-2.1-or-later.txt",
        "metadata/runtime-licenses/GCC-RUNTIME-LIBRARY-EXCEPTION-3.1.txt",
        "metadata/runtime-licenses/SOURCES-AND-RELINKING.md",
    ];
    let missing_runtime_licenses = if bundled {
        runtime_required
            .into_iter()
            .filter(|file| !files.contains(*file))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let forbidden = files
        .iter()
        .filter(|file| forbidden_package_path(file, bundled))
        .cloned()
        .collect::<Vec<_>>();
    let compressed_size = bytes.len() as u64;
    let result = no_workspace_or_git_dependency
        && missing_required.is_empty()
        && missing_runtime_licenses.is_empty()
        && forbidden.is_empty()
        && compressed_size <= CRATES_IO_COMPRESSED_LIMIT_BYTES;
    Ok(json!({
        "crate":package,
        "archive":format!("target/package/{package}-{RELEASE_VERSION}.crate"),
        "compressed_size_limit_bytes":CRATES_IO_COMPRESSED_LIMIT_BYTES,
        "compressed_size_within_limit":compressed_size <= CRATES_IO_COMPRESSED_LIMIT_BYTES,
        "metrics_policy":"The actual archive is checked against the crates.io compressed-size limit. Raw bytes and hashes are intentionally not committed because Cargo embeds a volatile Git receipt in each package.",
        "file_count":files.len(),
        "missing_required_files":missing_required,
        "missing_runtime_license_files":missing_runtime_licenses,
        "forbidden_files":forbidden,
        "packaged_manifest_has_no_path_or_git_dependencies":no_workspace_or_git_dependency,
        "result":if result { "pass" } else { "fail" },
    }))
}

fn write_directory_checksum(directory: &Path, package_hash: &str) -> Result<()> {
    let mut files = BTreeMap::new();
    let mut pending = vec![directory.to_path_buf()];
    while let Some(current) = pending.pop() {
        for entry in fs::read_dir(current)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.file_name().and_then(|name| name.to_str())
                != Some(".cargo-checksum.json")
            {
                let relative = path
                    .strip_prefix(directory)
                    .map_err(|_| policy("packaged file escaped independent package registry"))?
                    .to_string_lossy()
                    .replace('\\', "/");
                files.insert(relative, hash::bytes(&fs::read(path)?));
            }
        }
    }
    let value = json!({"files":files,"package":package_hash});
    let mut bytes = serde_json::to_vec(&value)?;
    bytes.push(b'\n');
    fs::write(directory.join(".cargo-checksum.json"), bytes)?;
    Ok(())
}

fn forbidden_package_path(path: &str, bundled: bool) -> bool {
    let lower = path.to_ascii_lowercase();
    if lower.starts_with("target/")
        || lower.contains("source-cache")
        || lower.ends_with(".exe")
        || lower.ends_with(".map")
        || lower.ends_with(".log")
        || lower.ends_with(".env")
    {
        return true;
    }
    !bundled
        && lower.starts_with("native/")
        && (lower.ends_with(".a")
            || lower.ends_with(".lib")
            || lower.ends_with(".o")
            || lower.ends_with(".obj"))
}

fn contains_path_or_git_dependency(value: &toml::Value, inside_dependency: bool) -> bool {
    match value {
        toml::Value::Table(table) => table.iter().any(|(key, value)| {
            (inside_dependency && (key == "path" || key == "git"))
                || contains_path_or_git_dependency(
                    value,
                    inside_dependency
                        || matches!(
                            key.as_str(),
                            "dependencies" | "build-dependencies" | "dev-dependencies"
                        ),
                )
        }),
        toml::Value::Array(values) => values
            .iter()
            .any(|value| contains_path_or_git_dependency(value, inside_dependency)),
        _ => false,
    }
}

fn audit_carrier(root: &Path, carrier: &str) -> Result<Value> {
    let directory = root.join("crates").join(carrier);
    let manifest: Value =
        serde_json::from_slice(&fs::read(directory.join("metadata/bundle-manifest.json"))?)?;
    let mut paths = BTreeSet::new();
    for collection in ["archives", "runtime_archives"] {
        for record in manifest[collection]
            .as_array()
            .ok_or_else(|| policy(&format!("{carrier} lacks {collection}")))?
        {
            paths.insert((
                record["path"]
                    .as_str()
                    .ok_or_else(|| policy("carrier archive has no path"))?
                    .to_owned(),
                record["sha256"]
                    .as_str()
                    .ok_or_else(|| policy("carrier archive has no checksum"))?
                    .to_owned(),
                record["size_bytes"].as_u64(),
            ));
        }
    }
    let mut artifacts = Vec::new();
    for (relative, expected_hash, expected_size) in paths {
        let path = directory.join(&relative);
        let actual_hash = hash::file(&path)?;
        let actual_size = fs::metadata(&path)?.len();
        let command_path = command_path(&path);
        external_command("ar", ["t", command_path.as_str()])?;
        external_command("nm", ["-g", command_path.as_str()])?;
        let passed = actual_hash == expected_hash
            && expected_size.is_none_or(|expected_size| actual_size == expected_size);
        artifacts.push(json!({
            "path":relative,
            "expected_sha256":expected_hash,
            "actual_sha256":actual_hash,
            "expected_size_bytes":expected_size,
            "actual_size_bytes":actual_size,
            "archive_structure_inspection":"ar and nm completed successfully",
            "result":if passed { "pass" } else { "fail" },
        }));
    }
    let result = artifacts
        .iter()
        .all(|artifact| artifact["result"].as_str() == Some("pass"));
    Ok(json!({
        "carrier":carrier,
        "target":manifest["target"],
        "classification":"independently-recomputed",
        "artifacts":artifacts,
        "result":if result { "pass" } else { "fail" },
    }))
}

fn command_path(path: &Path) -> String {
    let rendered = path.to_string_lossy().to_string();
    rendered
        .strip_prefix("\\\\?\\")
        .unwrap_or(&rendered)
        .to_owned()
}

fn external_command<'a>(program: &str, arguments: impl IntoIterator<Item = &'a str>) -> Result<()> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| {
            policy(&format!(
                "required external archive-inspection command `{program}` did not run: {error}"
            ))
        })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(policy(&format!(
            "external archive-inspection command `{program}` failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

fn compare_release_candidate(
    root: &Path,
    safe: &Value,
    raw: &Value,
    core: &Value,
    carriers: &Value,
) -> Result<Vec<Value>> {
    let candidate: Value = serde_json::from_slice(&fs::read(
        root.join("generated/release-readiness/release-candidate-report.json"),
    )?)?;
    let actual = [
        (
            "safe API freeze",
            safe["actual_count"].as_u64().unwrap_or_default(),
        ),
        (
            "raw ABI freeze",
            raw["actual_count"].as_u64().unwrap_or_default(),
        ),
        (
            "core support API freeze",
            core["actual_count"].as_u64().unwrap_or_default(),
        ),
        (
            "target-carrier metadata freeze",
            carriers["actual_count"].as_u64().unwrap_or_default(),
        ),
    ];
    let mut discrepancies = Vec::new();
    for (gate, actual_count) in actual {
        let record = candidate["gates"].as_array().and_then(|gates| {
            gates
                .iter()
                .find(|entry| entry["gate"].as_str() == Some(gate))
        });
        let Some(record) = record else {
            discrepancies
                .push(json!({"gate":gate,"reason":"missing from release-candidate report"}));
            continue;
        };
        if record["status"].as_str() != Some("pass") {
            discrepancies.push(json!({"gate":gate,"reason":"candidate gate is not pass","candidate_status":record["status"]}));
        }
        if record["count"].as_u64() != Some(actual_count) {
            discrepancies.push(json!({"gate":gate,"reason":"count mismatch","candidate_count":record["count"],"independent_count":actual_count}));
        }
    }
    if candidate["overall_status"].as_str() != Some("release_candidate") {
        discrepancies.push(
            json!({"gate":"overall status","reason":"candidate is not marked release_candidate"}),
        );
    }
    Ok(discrepancies)
}

fn render_markdown(audit: &Value, packages: &[Value], carriers: &[Value]) -> String {
    let mut output = String::from("# Final independent release audit\n\n");
    output.push_str(&format!(
        "Independent local status: **{}**. This report is deliberately package-first and does not treat a generated release-readiness report as proof of itself.\n\n",
        audit["status"].as_str().unwrap_or("fail")
    ));
    output.push_str("## Packaged crates\n\n| Crate | Package size | Files | Result |\n| --- | --- | ---: | --- |\n");
    for package in packages {
        output.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            package["crate"].as_str().unwrap_or("unknown"),
            if package["compressed_size_within_limit"]
                .as_bool()
                .unwrap_or(false)
            {
                "within 10 MiB"
            } else {
                "exceeds 10 MiB"
            },
            package["file_count"].as_u64().unwrap_or_default(),
            package["result"].as_str().unwrap_or("fail")
        ));
    }
    output
        .push_str("\n## Carrier archives\n\n| Carrier | Target | Result |\n| --- | --- | --- |\n");
    for carrier in carriers {
        output.push_str(&format!(
            "| `{}` | `{}` | {} |\n",
            carrier["carrier"].as_str().unwrap_or("unknown"),
            carrier["target"].as_str().unwrap_or("unknown"),
            carrier["result"].as_str().unwrap_or("fail")
        ));
    }
    output.push_str("\n## Discrepancies\n\n");
    let discrepancies = audit["discrepancies"]
        .as_array()
        .map_or(&[][..], Vec::as_slice);
    if discrepancies.is_empty() {
        output.push_str("None. The independently recomputed counts and hashes match the committed release-candidate report.\n");
    } else {
        for discrepancy in discrepancies {
            output.push_str(&format!("- `{discrepancy}`\n"));
        }
    }
    output.push_str("\n## Decision boundary\n\n");
    output.push_str(
        audit["decision"]
            .as_str()
            .unwrap_or("No decision recorded."),
    );
    output.push('\n');
    output
}

fn write_reports(output: &Path, reports: &BTreeMap<String, Value>) -> Result<()> {
    fs::create_dir_all(output)?;
    for (name, value) in reports {
        let path = output.join(name);
        if name.ends_with(".md") {
            let markdown = value["markdown"]
                .as_str()
                .ok_or_else(|| policy("markdown output has no markdown field"))?;
            write_if_changed(&path, markdown.as_bytes())?;
        } else {
            write_if_changed(&path, &json_bytes(value)?)?;
        }
    }
    Ok(())
}

fn json_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<()> {
    if fs::read(path).ok().as_deref() != Some(bytes) {
        fs::write(path, bytes)?;
    }
    Ok(())
}

fn run(root: &Path, command: &mut Command, label: &str) -> Result<()> {
    let output = command.current_dir(root).output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(policy(&format!(
        "{label} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout).trim(),
        String::from_utf8_lossy(&output.stderr).trim()
    )))
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}
