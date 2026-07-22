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
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SNAPSHOT: &str = "complete-slatec-05078ebcb649b50e4435";
const TARGET: &str = "x86_64-pc-windows-gnu";
const CLASSIFICATIONS: &[&str] = &[
    "us-government-public-domain",
    "explicit-public-domain",
    "explicit-permissive",
    "explicit-copyleft-compatible",
    "third-party-notice-required",
    "unresolved-authorship",
    "unresolved-rights",
    "excluded-from-bundle",
];
const ACCEPTED_BUNDLE_CLASSIFICATIONS: &[&str] = &[
    "us-government-public-domain",
    "explicit-public-domain",
    "explicit-permissive",
    "explicit-copyleft-compatible",
    "third-party-notice-required",
];
const FIRST_BUNDLED_FAMILY: &str = "special-elementary";
const FIRST_BUNDLED_ARCHIVE: &str = "native/libslatec_bundle_special_elementary.a";
const BUNDLED_GFORTRAN_ARCHIVE: &str = "native/libslatec_bundle_gfortran.a";
const BUNDLED_QUADMATH_ARCHIVE: &str = "native/libslatec_bundle_quadmath.a";
const BUNDLED_BUILD_RECEIPT: &str = "metadata/bundle-build-receipt.json";
const PROFILE_SOURCES: &[&str] = &["i1mach.f", "r1mach.f", "d1mach.f"];
const COMPILE_FLAGS: &[&str] = &[
    "-x",
    "f77",
    "-std=legacy",
    "-ffixed-line-length-none",
    "-fno-ident",
    "-frandom-seed=slatec-bundled-special-elementary",
    "-c",
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

type ArchiveBuildAudit = (Vec<String>, Vec<String>, Vec<String>, Vec<Value>);

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

#[derive(Default, Deserialize)]
struct EvidenceRegistry {
    #[serde(default)]
    records: Vec<EvidenceRecord>,
}

#[derive(Deserialize)]
struct EvidenceRecord {
    id: String,
    authority: String,
    url: String,
    retrieved_on: String,
    excerpt: String,
    statement: String,
    scope: String,
    source_ids: Vec<String>,
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
    author_prologue: Option<String>,
    #[serde(default)]
    governing_notice: Option<String>,
    #[serde(default)]
    redistribution_conditions: Option<String>,
    #[serde(default)]
    evidence_ids: Vec<String>,
    #[serde(default)]
    manual_override_provenance: Option<String>,
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

/// Builds the first target-specific archive only after its exact family closure
/// passes the source-level redistribution gate. The source cache and compiler
/// are intentionally touched only after that decision.
pub fn require_buildable(root: &Path) -> Result<()> {
    let manifest = materialized_manifest(root)?;
    let evidence = read_evidence(root)?;
    let overrides = read_overrides(root, &manifest, &evidence)?;
    let source_by_id = manifest
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let selected = manifest
        .families
        .get(FIRST_BUNDLED_FAMILY)
        .ok_or_else(|| policy("first bundled family has no source closure"))?;
    let blocked = selected
        .iter()
        .filter(|id| {
            let Some(source) = source_by_id.get(id.as_str()) else {
                return true;
            };
            !overrides.get(source.id.as_str()).is_some_and(|record| {
                ACCEPTED_BUNDLE_CLASSIFICATIONS.contains(&record.classification.as_str())
            })
        })
        .cloned()
        .collect::<Vec<_>>();
    if !blocked.is_empty() {
        return Err(CorpusError::Policy(format!(
            "bundled archive production for {FIRST_BUNDLED_FAMILY} is blocked by unresolved source units: {}. No compiler or source cache was consulted.",
            blocked.join(", ")
        )));
    }

    let cache = env::var_os("SLATEC_SOURCE_CACHE")
        .map(PathBuf::from)
        .ok_or_else(|| {
            policy("build-bundled-provider requires SLATEC_SOURCE_CACHE after provenance clearance")
        })?;
    let compiler = env::var_os("SLATEC_GFORTRAN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("gfortran"));
    verify_bundled_compiler(&compiler)?;
    let carrier_root = root.join("crates/slatec-bundled-x86_64-pc-windows-gnu");
    let stage = root
        .join("target/bundled-provider")
        .join(FIRST_BUNDLED_FAMILY);
    if stage.exists() {
        fs::remove_dir_all(&stage)?;
    }
    fs::create_dir_all(&stage)?;
    let archive = stage.join("libslatec_bundle_special_elementary.a");
    let (members, symbols, undefined, source_records) = build_family_archive(
        root,
        &source_by_id,
        selected,
        &cache,
        &compiler,
        &stage,
        &archive,
    )?;
    let reproducibility_stage = root
        .join("target/bundled-provider")
        .join("reproducibility")
        .join(FIRST_BUNDLED_FAMILY);
    if reproducibility_stage.exists() {
        fs::remove_dir_all(&reproducibility_stage)?;
    }
    fs::create_dir_all(&reproducibility_stage)?;
    let reproducibility_archive =
        reproducibility_stage.join("libslatec_bundle_special_elementary.a");
    let _ = build_family_archive(
        root,
        &source_by_id,
        selected,
        &cache,
        &compiler,
        &reproducibility_stage,
        &reproducibility_archive,
    )?;
    let archive_sha256 = hash::file(&archive)?;
    if archive_sha256 != hash::file(&reproducibility_archive)? {
        return Err(policy(
            "same-root bundled archive reproduction produced a different SHA-256",
        ));
    }

    let runtime_required = undefined.iter().any(|symbol| symbol.contains("_gfortran_"));
    let native = carrier_root.join("native");
    fs::create_dir_all(&native)?;
    let carrier_archive = carrier_root.join(FIRST_BUNDLED_ARCHIVE);
    fs::copy(&archive, &carrier_archive)?;
    let mut runtime_archives = Vec::new();
    if runtime_required {
        let runtime_source = static_runtime_archive(&compiler, "libgfortran.a")?;
        let runtime_destination = carrier_root.join(BUNDLED_GFORTRAN_ARCHIVE);
        fs::copy(&runtime_source, &runtime_destination)?;
        runtime_archives.push(json!({
            "component":"libgfortran","path":BUNDLED_GFORTRAN_ARCHIVE,"sha256":hash::file(&runtime_destination)?,
            "link_name":"slatec_bundle_gfortran","linking_mode":"static","source":runtime_source.display().to_string(),
            "license":"GPL-3.0-with-GCC-exception","obligations":"Preserve the GCC Runtime Library Exception notice; the final consumer remains relinkable because Cargo links ordinary static archives."
        }));
    }
    let quadmath_source = static_runtime_archive(&compiler, "libquadmath.a")?;
    let quadmath_destination = carrier_root.join(BUNDLED_QUADMATH_ARCHIVE);
    fs::copy(&quadmath_source, &quadmath_destination)?;
    runtime_archives.push(json!({
        "component":"libquadmath","path":BUNDLED_QUADMATH_ARCHIVE,"sha256":hash::file(&quadmath_destination)?,
        "link_name":"slatec_bundle_quadmath","linking_mode":"static","source":quadmath_source.display().to_string(),
        "license":"LGPL-2.1-or-later","obligations":"Preserve the LGPL notice and provide the corresponding-source or relinking information required for the static runtime archive. It is included because the reviewed Rust GNU target final link requires quadmath_snprintf; the SLATEC family archive itself has no quadmath reference."
    }));
    let receipt = json!({
        "schema_id":"slatec-rs/bundled-build-receipt","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":TARGET,
        "family":FIRST_BUNDLED_FAMILY,
        "compiler":{"path":compiler.display().to_string(),"version":command_output(&compiler, &["--version"])? .lines().next().unwrap_or_default(),"target":command_output(&compiler, &["-dumpmachine"])? .trim()},
        "compile_flags":COMPILE_FLAGS,"archiver":sibling_tool(&compiler, "ar").display().to_string(),"archiver_flags":"crsD",
        "archives":[{"family":FIRST_BUNDLED_FAMILY,"path":FIRST_BUNDLED_ARCHIVE,"sha256":archive_sha256,"size_bytes":fs::metadata(&carrier_archive)?.len()}],
        "runtime_archives":runtime_archives,
        "source_units":source_records,
        "archive_audit":{"schema_id":"slatec-rs/bundled-archive-audit","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":TARGET,"status":"ready","archive_members":members,"defined_symbols":symbols,"undefined_symbols":undefined,"imported_dlls":[],"same_root_reproduction":"sha256_match"},
        "runtime_audit":{"schema_id":"slatec-rs/bundled-runtime-audit","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":TARGET,"status":"static_runtime_pending_consumer_probe","compiler_profile":"GNU Fortran 14.2.0 / x86_64-w64-mingw32","compiler_invoked":true,"source_cache_read":true,"network_access":false,"runtime_components":[
            {"component":"libgfortran","static_or_dynamic":if runtime_required { "static bundled" } else { "not required" },"actually_referenced":runtime_required,"distribution_action":if runtime_required { "include libslatec_bundle_gfortran.a with GPLv3 and Runtime Library Exception notice" } else { "not included" }},
            {"component":"libquadmath","static_or_dynamic":"static bundled","actually_referenced":"required by the reviewed Rust GNU target final link (quadmath_snprintf), not by the SLATEC archive","distribution_action":"include libslatec_bundle_quadmath.a with LGPL-2.1-or-later notice and source/relinking information"},
            {"component":"libgcc","static_or_dynamic":"toolchain-provided final-link support","actually_referenced":"not separately bundled","distribution_action":"no carrier copy; audit final consumer imports"}
        ],"imported_dlls":[],"reason":"Archive-level audit complete; final consumer import probe is recorded by the release validation command."}
    });
    write_json(&carrier_root.join(BUNDLED_BUILD_RECEIPT), &receipt)?;
    generate(root)?;
    let consumer_probe = probe_clean_consumers(root, &cache, &compiler)?;
    let mut final_receipt: Value =
        serde_json::from_slice(&fs::read(carrier_root.join(BUNDLED_BUILD_RECEIPT))?)?;
    final_receipt["runtime_audit"]["status"] = Value::String("ready".to_owned());
    final_receipt["runtime_audit"]["imported_dlls"] =
        consumer_probe["bundled_imported_dlls"].clone();
    final_receipt["runtime_audit"]["clean_consumer"] = consumer_probe.clone();
    write_json(&carrier_root.join(BUNDLED_BUILD_RECEIPT), &final_receipt)?;
    generate(root)?;
    Ok(())
}

fn verify_bundled_compiler(compiler: &Path) -> Result<()> {
    let target = command_output(compiler, &["-dumpmachine"])?;
    if target.trim() != "x86_64-w64-mingw32" {
        return Err(policy(&format!(
            "bundled archive production requires GNU Fortran target x86_64-w64-mingw32; found {}",
            target.trim()
        )));
    }
    let version = command_output(compiler, &["--version"])?;
    if !version.contains("14.2.0") {
        return Err(policy(
            "bundled archive production requires the reviewed GNU Fortran 14.2.0 toolchain",
        ));
    }
    Ok(())
}

fn build_family_archive(
    root: &Path,
    source_by_id: &BTreeMap<&str, &Source>,
    selected: &[String],
    cache: &Path,
    compiler: &Path,
    stage: &Path,
    archive: &Path,
) -> Result<ArchiveBuildAudit> {
    let objects = stage.join("objects");
    fs::create_dir_all(&objects)?;
    let mut object_paths = Vec::new();
    let mut source_records = Vec::new();
    for id in selected {
        let source = source_by_id
            .get(id.as_str())
            .ok_or_else(|| policy(&format!("bundled family references unknown source {id}")))?;
        let source_path = verified_cached_source(cache, source)?;
        let object = objects.join(format!("{}.o", source.id));
        compile_source(compiler, &source_path, &object)?;
        object_paths.push(object);
        source_records.push(json!({
            "id":source.id,"subset":source.subset,"path":source.path,"sha256":source.sha256,
            "cached_source_sha256":hash::file(&source_path)?,"object":format!("{}.o",source.id)
        }));
    }
    for profile in PROFILE_SOURCES {
        let source = root
            .join("crates/slatec-src/native/gnu-mingw-x86_64")
            .join(profile);
        let object = objects.join(format!("profile-{}.o", profile.trim_end_matches(".f")));
        let object_name = object
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_owned();
        compile_source(compiler, &source, &object)?;
        object_paths.push(object);
        source_records.push(json!({
            "id":format!("profile-{}",profile.trim_end_matches(".f")),"subset":"slatec-rs-profile","path":format!("native/gnu-mingw-x86_64/{profile}"),"sha256":hash::file(&source)?,"cached_source_sha256":hash::file(&source)?,"object":object_name
        }));
    }
    object_paths.sort();
    let ar = sibling_tool(compiler, "ar");
    if archive.exists() {
        fs::remove_file(archive)?;
    }
    let status = Command::new(&ar)
        .arg("crsD")
        .arg(archive)
        .args(&object_paths)
        .status()?;
    if !status.success() {
        return Err(policy(&format!(
            "GNU ar failed creating {}",
            archive.display()
        )));
    }
    let members = command_output(
        &ar,
        &[
            "t",
            archive
                .to_str()
                .ok_or_else(|| policy("archive path is not UTF-8"))?,
        ],
    )?
    .lines()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    let nm = sibling_tool(compiler, "nm");
    let defined = command_output(
        &nm,
        &[
            "-g",
            "--defined-only",
            archive
                .to_str()
                .ok_or_else(|| policy("archive path is not UTF-8"))?,
        ],
    )?;
    let undefined = command_output(
        &nm,
        &[
            "-u",
            archive
                .to_str()
                .ok_or_else(|| policy("archive path is not UTF-8"))?,
        ],
    )?;
    Ok((
        members,
        symbols_from_nm(&defined),
        symbols_from_nm(&undefined),
        source_records,
    ))
}

fn verified_cached_source(cache: &Path, source: &Source) -> Result<PathBuf> {
    let relative = if source.subset == "main-src" {
        source.path.clone()
    } else {
        format!("{}/{}", source.subset, source.path)
    };
    let path = cache.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    if !path.is_file() {
        return Err(policy(&format!(
            "bundled source cache is missing {} at {}",
            source.id,
            path.display()
        )));
    }
    let actual = hash::file(&path)?;
    if actual != source.sha256 {
        return Err(policy(&format!(
            "bundled source cache hash mismatch for {}: expected {}, found {actual}",
            source.id, source.sha256
        )));
    }
    Ok(path)
}

fn compile_source(compiler: &Path, source: &Path, object: &Path) -> Result<()> {
    let output = Command::new(compiler)
        .args(COMPILE_FLAGS)
        .arg(source)
        .arg("-o")
        .arg(object)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(policy(&format!(
            "GNU Fortran failed for {}: {}",
            source.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

fn sibling_tool(compiler: &Path, name: &str) -> PathBuf {
    compiler
        .parent()
        .map(|parent| parent.join(format!("{name}.exe")))
        .filter(|path| path.is_file())
        .unwrap_or_else(|| PathBuf::from(name))
}

fn command_output(program: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new(program).args(args).output()?;
    if !output.status.success() {
        return Err(policy(&format!(
            "{} {args:?} failed: {}",
            program.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn symbols_from_nm(output: &str) -> Vec<String> {
    let mut symbols = output
        .lines()
        .filter_map(|line| line.split_whitespace().last())
        .filter(|symbol| !symbol.ends_with(":") && !symbol.ends_with(".o:"))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    symbols.sort();
    symbols.dedup();
    symbols
}

fn static_runtime_archive(compiler: &Path, name: &str) -> Result<PathBuf> {
    let reported = command_output(compiler, &[&format!("-print-file-name={name}")])?;
    let candidate = PathBuf::from(reported.trim());
    if candidate.is_file() {
        return Ok(candidate);
    }
    let fallback = compiler
        .parent()
        .and_then(Path::parent)
        .map(|root| root.join("lib").join(name))
        .ok_or_else(|| policy("GNU Fortran executable has no toolchain root"))?;
    if fallback.is_file() {
        Ok(fallback)
    } else {
        Err(policy(&format!(
            "reviewed GNU toolchain does not provide static {name}: compiler reported {}",
            candidate.display()
        )))
    }
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    Ok(())
}

fn probe_clean_consumers(root: &Path, cache: &Path, compiler: &Path) -> Result<Value> {
    let probe_root = root.join("target/bundled-provider/clean-consumer");
    let source = probe_root.join("src/main.rs");
    fs::create_dir_all(source.parent().expect("consumer source parent"))?;
    let canonical_root = root.canonicalize()?;
    let slatec_path = canonical_root
        .to_string_lossy()
        .trim_start_matches(r"\\?\")
        .replace('\\', "/");
    fs::write(
        probe_root.join("Cargo.toml"),
        format!(
            "[workspace]\n\n[package]\nname = \"slatec-bundled-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\nslatec = {{ path = \"{slatec_path}/crates/slatec\", features = [\"special-elementary\"] }}\n"
        ),
    )?;
    fs::write(
        &source,
        "fn main() { let value = slatec::special::elementary::log1p(0.5).expect(\"finite input\"); println!(\"{value:.17}\"); }\n",
    )?;
    let target_dir = root.join("target/bundled-provider/clean-consumer-target");
    let bundled = run_consumer(
        root,
        &probe_root,
        &target_dir,
        &[],
        &[
            "SLATEC_SOURCE_CACHE",
            "SLATEC_SYSTEM_LIB_DIR",
            "SLATEC_SYSTEM_RUNTIME_LIB_DIR",
        ],
        Some((
            "SLATEC_GFORTRAN",
            "__bundled_provider_must_not_invoke_gfortran__",
        )),
        None,
    )?;
    let source_build_root = root.join("target/bundled-provider/source-build-consumer");
    fs::create_dir_all(source_build_root.join("src"))?;
    fs::write(
        source_build_root.join("Cargo.toml"),
        format!(
            "[workspace]\n\n[package]\nname = \"slatec-source-build-parity\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\nslatec = {{ path = \"{slatec_path}/crates/slatec\", default-features = false, features = [\"std\", \"source-build\", \"special-elementary\"] }}\n"
        ),
    )?;
    fs::write(
        source_build_root.join("src/main.rs"),
        "fn main() { let value = slatec::special::elementary::log1p(0.5).expect(\"finite input\"); println!(\"{value:.17}\"); }\n",
    )?;
    let source_build = run_consumer(
        root,
        &source_build_root,
        &root.join("target/bundled-provider/source-build-parity-target"),
        &[],
        &[],
        None,
        Some((cache, compiler)),
    )?;
    if bundled.0 != source_build.0 {
        return Err(policy(&format!(
            "bundled and source-build elementary smoke values differ: {} versus {}",
            bundled.0, source_build.0
        )));
    }
    let objdump = sibling_tool(compiler, "objdump");
    let imports = command_output(
        &objdump,
        &[
            "-p",
            bundled
                .1
                .to_str()
                .ok_or_else(|| policy("consumer executable path is not UTF-8"))?,
        ],
    )?
    .lines()
    .filter_map(|line| line.trim().strip_prefix("DLL Name: "))
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if imports.iter().any(|name| {
        name.to_ascii_lowercase().contains("gfortran")
            || name.to_ascii_lowercase().contains("quadmath")
    }) {
        return Err(policy(
            "clean bundled consumer imports a GNU Fortran or quadmath DLL",
        ));
    }
    Ok(json!({
        "status":"pass","bundled_numeric":bundled.0,"source_build_numeric":source_build.0,
        "numeric_parity":"exact_rendered_f64","bundled_imported_dlls":imports,
        "environment":{"SLATEC_GFORTRAN":"intentionally invalid","SLATEC_SOURCE_CACHE":"absent","SLATEC_SYSTEM_LIB_DIR":"absent","SLATEC_SYSTEM_RUNTIME_LIB_DIR":"absent"}
    }))
}

fn run_consumer(
    root: &Path,
    manifest_root: &Path,
    target_dir: &Path,
    feature_args: &[&str],
    remove_env: &[&str],
    invalid_env: Option<(&str, &str)>,
    source_build: Option<(&Path, &Path)>,
) -> Result<(String, PathBuf)> {
    let mut command = Command::new("cargo");
    command
        .current_dir(root)
        .args(["run", "--manifest-path"])
        .arg(manifest_root.join("Cargo.toml"))
        .args(["--target", TARGET, "--offline"])
        .args(feature_args)
        .env("CARGO_TARGET_DIR", target_dir)
        .env("CARGO_NET_OFFLINE", "true");
    for key in remove_env {
        command.env_remove(key);
    }
    if let Some((key, value)) = invalid_env {
        command.env(key, value);
    }
    if let Some((source_cache, source_compiler)) = source_build {
        command
            .env("SLATEC_SOURCE_CACHE", source_cache)
            .env("SLATEC_GFORTRAN", source_compiler);
    }
    let output = command.output()?;
    if !output.status.success() {
        return Err(policy(&format!(
            "clean bundled consumer build failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    let value = String::from_utf8_lossy(&output.stdout)
        .lines()
        .rev()
        .find(|line| line.trim().parse::<f64>().is_ok())
        .map(str::trim)
        .ok_or_else(|| policy("consumer probe did not print a numeric smoke value"))?
        .to_owned();
    Ok((
        value,
        target_dir
            .join(TARGET)
            .join("debug/slatec-bundled-consumer.exe"),
    ))
}

fn artifacts(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    let manifest = materialized_manifest(root)?;
    let evidence = read_evidence(root)?;
    let overrides = read_overrides(root, &manifest, &evidence)?;
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
            .unwrap_or_else(|| {
                if author_fields.contains(&source_key(&source.subset, &source.path, &source.sha256))
                {
                    "unresolved-rights"
                } else {
                    "unresolved-authorship"
                }
            });
        let bundle_eligible = ACCEPTED_BUNDLE_CLASSIFICATIONS.contains(&classification);
        *classification_counts
            .entry(classification.to_owned())
            .or_default() += 1;
        let key = source_key(&source.subset, &source.path, &source.sha256);
        let units = routine_names.get(&key).cloned().unwrap_or_default();
        let author_field_present = author_fields.contains(&key);
        let originating_package = package_name(&source.subset);
        let evidence_records = override_record
            .map(|record| {
                record
                    .evidence_ids
                    .iter()
                    .filter_map(|id| evidence.get(id))
                    .map(|item| {
                        json!({
                            "id":item.id,
                            "authority":item.authority,
                            "url":item.url,
                            "retrieved_on":item.retrieved_on,
                            "excerpt":item.excerpt,
                            "statement":item.statement,
                            "scope":item.scope,
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
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
            "named_authors_status":if override_record.is_some() { "reviewed_source_prologue" } else if author_field_present { "prologue_author_field_present_but_text_not_preserved_in_committed_index" } else { "unavailable" },
            "stated_institutions":override_record.map(|record| record.stated_institutions.clone()).unwrap_or_default(),
            "author_prologue":override_record.and_then(|record| record.author_prologue.clone()),
            "governing_notice":override_record.and_then(|record| record.governing_notice.clone()),
            "redistribution_conditions":override_record.and_then(|record| record.redistribution_conditions.clone()),
            "third_party_incorporation_notes":default_third_party_note(&source.subset),
            "redistribution_classification":classification,
            "confidence":override_record.map(|record| record.confidence.clone()).unwrap_or_else(|| "unreviewed".to_owned()),
            "bundle_eligible":bundle_eligible,
            "evidence":evidence_records,
            "manual_override_provenance":override_record.and_then(|record| record.manual_override_provenance.clone()),
            "unresolved_questions":if override_record.is_some() { Vec::<String>::new() } else { vec!["No hash-guarded, source-specific redistribution classification has been recorded.".to_owned(), "Government sponsorship, institutional affiliation, and Netlib hosting are not treated as a public-domain dedication.".to_owned()] },
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
    let carrier_root = root.join("crates/slatec-bundled-x86_64-pc-windows-gnu");
    let receipt = read_build_receipt(&carrier_root)?;
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
        let archive = family_archive(family);
        let archive_ready = match archive {
            Some(path) => archive_record(&carrier_root, path, &receipt)?,
            None => None,
        };
        let status = if ids.is_empty() {
            "no_selected_source_closure"
        } else if !blocked.is_empty() {
            "blocked_by_source_provenance"
        } else if archive_ready.is_some() {
            "ready"
        } else {
            "eligible_pending_archive"
        };
        family_records.push(json!({
            "feature":family,
            "source_unit_count":ids.len(),
            "bundle_available":status == "ready",
            "blocked_source_ids":blocked,
            "archive":archive,
            "archive_record":archive_ready,
            "status":status,
        }));
    }
    let eligible_source_units = records
        .iter()
        .filter(|record| record["bundle_eligible"] == true)
        .count();
    let carrier_status = if family_records
        .iter()
        .any(|record| record["status"] == "ready")
    {
        "ready"
    } else if family_records
        .iter()
        .any(|record| record["status"] == "eligible_pending_archive")
    {
        "eligible_pending_archive"
    } else {
        "blocked_by_source_provenance"
    };
    let provenance = json!({
        "schema_id":"slatec-rs/slatec-source-provenance",
        "schema_version":"2.0.0",
        "snapshot_id":SNAPSHOT,
        "classification_vocabulary":CLASSIFICATIONS,
        "records":records,
        "summary":{"physical_source_units":physical.len(),"classification_counts":classification_counts,"legal_status":"evidence inventory, not legal advice"},
    });
    let eligibility = json!({
        "schema_id":"slatec-rs/bundled-source-eligibility",
        "schema_version":"2.0.0",
        "snapshot_id":SNAPSHOT,
        "target":TARGET,
        "accepted_bundle_classifications":ACCEPTED_BUNDLE_CLASSIFICATIONS,
        "carrier":{"crate":"slatec-bundled-x86_64-pc-windows-gnu","status":carrier_status,"reason":"A family archive is available only when its entire source closure has accepted, hash-guarded provenance evidence and the generated carrier receipt verifies the archive."},
        "summary":{"physical_source_units":physical.len(),"eligible_source_units":eligible_source_units,"unresolved_source_units":physical.len()-eligible_source_units},
        "families":family_records,
    });
    let runtime_audit = receipt
        .as_ref()
        .and_then(|item| item.get("runtime_audit"))
        .cloned()
        .unwrap_or_else(|| json!({
            "schema_id":"slatec-rs/bundled-runtime-audit","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":TARGET,
            "status":"not_applicable_no_ready_archive","compiler_profile":Value::Null,"compiler_invoked":false,"source_cache_read":false,"network_access":false,
            "runtime_components":[
                {"component":"libgfortran","static_or_dynamic":"not_assessed","actually_referenced":"not_assessed","distribution_action":"no bundled runtime is present"},
                {"component":"libquadmath","static_or_dynamic":"not_assessed","actually_referenced":"not_assessed","distribution_action":"no bundled runtime is present; do not add it speculatively"},
                {"component":"libgcc","static_or_dynamic":"not_assessed","actually_referenced":"not_assessed","distribution_action":"no bundled runtime is present"}
            ],"imported_dlls":[],"reason":"No family archive has passed both provenance and deterministic build checks."
        }));
    let archive_audit = receipt
        .as_ref()
        .and_then(|item| item.get("archive_audit"))
        .cloned()
        .unwrap_or_else(|| json!({
            "schema_id":"slatec-rs/bundled-archive-audit","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":TARGET,
            "status":"not_applicable_no_ready_archive","archive_members":[],"defined_symbols":[],"undefined_symbols":[],"imported_dlls":[],
            "reason":"No family archive has passed both provenance and deterministic build checks."
        }));
    let unresolved = provenance["records"]
        .as_array()
        .expect("provenance records")
        .iter()
        .filter(|record| !record["bundle_eligible"].as_bool().unwrap_or(false))
        .cloned()
        .collect::<Vec<_>>();
    let mut sbom_files = provenance["records"]
        .as_array()
        .expect("provenance records")
        .iter()
        .map(|record| {
            json!({
                "SPDXID":format!("SPDXRef-Source-{}",record["selected_source_id"].as_str().unwrap_or("unknown")),
                "fileName":format!("{}/{}",record["source_subset"].as_str().unwrap_or("unknown"),record["path"].as_str().unwrap_or("unknown")),
                "checksums":[{"algorithm":"SHA256","checksumValue":record["sha256"]}],
                "licenseConcluded":if record["redistribution_classification"] == "explicit-public-domain" { "LicenseRef-PublicDomain" } else { "NOASSERTION" },
                "licenseInfoInFiles":if record["redistribution_classification"] == "explicit-public-domain" { json!(["LicenseRef-PublicDomain"]) } else { json!(["NOASSERTION"]) },
                "copyrightText":"NOASSERTION",
                "comment":format!("Redistribution classification: {}",record["redistribution_classification"].as_str().unwrap_or("unresolved-rights")),
            })
        })
        .collect::<Vec<_>>();
    if let Some(build_receipt) = &receipt {
        for archive in build_receipt["archives"]
            .as_array()
            .into_iter()
            .flatten()
            .chain(
                build_receipt["runtime_archives"]
                    .as_array()
                    .into_iter()
                    .flatten(),
            )
        {
            let component = archive["component"]
                .as_str()
                .or_else(|| archive["family"].as_str())
                .unwrap_or("unknown");
            let license = archive["license"]
                .as_str()
                .unwrap_or("LicenseRef-PublicDomain");
            sbom_files.push(json!({
                "SPDXID":format!("SPDXRef-Archive-{component}"),
                "fileName":archive["path"],
                "checksums":[{"algorithm":"SHA256","checksumValue":archive["sha256"]}],
                "licenseConcluded":license,
                "licenseInfoInFiles":[license],
                "copyrightText":"NOASSERTION",
                "comment":archive["obligations"].as_str().unwrap_or("Historical archive is built only from hash-guarded source units."),
            }));
        }
    }
    let sbom = json!({
        "spdxVersion":"SPDX-2.3",
        "dataLicense":"CC0-1.0",
        "SPDXID":"SPDXRef-DOCUMENT",
        "name":"slatec-rs bundled provider source inventory",
        "documentNamespace":format!("https://github.com/evnekdev/slatec-rs/spdx/{SNAPSHOT}"),
        "creationInfo":{"creators":["Tool: slatec-corpus"],"created":"1970-01-01T00:00:00Z"},
        "comment":if carrier_status == "ready" { "Includes the hash-guarded source inventory for ready bundled family archives." } else { "This is a source inventory, not a claim that an archive is redistributable." },
        "files":sbom_files,
    });
    let first_source_ids = manifest
        .families
        .get(FIRST_BUNDLED_FAMILY)
        .cloned()
        .unwrap_or_default();
    let source_unit_manifest = json!({
        "schema_id":"slatec-rs/bundled-source-unit-manifest","schema_version":"2.0.0","snapshot_id":SNAPSHOT,
        "family":FIRST_BUNDLED_FAMILY,"source_ids":first_source_ids,
        "profile_sources":PROFILE_SOURCES,"records":provenance["records"].as_array().expect("records").iter().filter(|record| {
            record["selected_source_ids"].as_array().is_some_and(|ids| ids.iter().any(|id| first_source_ids.iter().any(|source_id| id.as_str() == Some(source_id))))
        }).collect::<Vec<_>>()
    });
    let build_recipe = json!({
        "schema_id":"slatec-rs/bundled-build-recipe","schema_version":"2.0.0","target":TARGET,"family":FIRST_BUNDLED_FAMILY,
        "compiler":"GNU Fortran 14.2.0 / x86_64-w64-mingw32","compile_flags":COMPILE_FLAGS,
        "archiver":"GNU ar 2.43","archive_flags":"crsD","source_verification":"SHA-256 against family-source-closure.json before every compilation",
        "command":"cargo run -p slatec-tools --bin slatec-corpus -- build-bundled-provider --offline"
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
        third_party_notices(&provenance, carrier_status),
    );
    insert_markdown(
        &mut artifacts,
        root.join("crates/slatec-bundled-x86_64-pc-windows-gnu/metadata/THIRD-PARTY-NOTICES.md"),
        third_party_notices(&provenance, carrier_status),
    );
    insert_markdown(
        &mut artifacts,
        root.join("crates/slatec-bundled-x86_64-pc-windows-gnu/metadata/REDISTRIBUTION-NOTICE.md"),
        redistribution_notice(&provenance, carrier_status),
    );
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/bundled-sbom.spdx.json"),
        &sbom,
    )?;
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/bundled-source-unit-manifest.json"),
        &source_unit_manifest,
    )?;
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/bundled-build-recipe.json"),
        &build_recipe,
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
            "schema_id":"slatec-rs/bundled-carrier-manifest","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":TARGET,
            "status":carrier_status,"source_unit_count":eligible_source_units,"source_eligibility":"generated/licensing/bundled-source-eligibility.json",
            "source_unit_manifest":"metadata/source-unit-manifest.json","build_recipe":"metadata/build-recipe.json",
            "build_receipt":BUNDLED_BUILD_RECEIPT,"runtime_audit":"generated/licensing/bundled-runtime-audit.json","archive_audit":"generated/licensing/bundled-archive-audit.json",
            "archives":family_records.iter().filter_map(|record| record["archive_record"].as_object().map(|_| json!({"family":record["feature"],"path":record["archive"],"sha256":record["archive_record"]["sha256"],"size_bytes":record["archive_record"]["size_bytes"]}))).collect::<Vec<_>>(),
            "runtime_archives":receipt.as_ref().and_then(|item| item.get("runtime_archives")).cloned().unwrap_or_else(|| json!([])),
            "reason":"Each ready family archive has complete hash-guarded provenance, a deterministic build receipt, checksum verification, and its exact runtime closure."
        }),
    )?;
    insert_json(
        &mut artifacts,
        root.join("crates/slatec-bundled-x86_64-pc-windows-gnu/metadata/source-unit-manifest.json"),
        &source_unit_manifest,
    )?;
    insert_json(
        &mut artifacts,
        root.join("crates/slatec-bundled-x86_64-pc-windows-gnu/metadata/build-recipe.json"),
        &build_recipe,
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

fn read_evidence(root: &Path) -> Result<BTreeMap<String, EvidenceRecord>> {
    let input = root.join("crates/slatec-src/metadata/bundled-provenance-evidence.json");
    let registry: EvidenceRegistry = serde_json::from_slice(&fs::read(input)?)?;
    let mut result = BTreeMap::new();
    for record in registry.records {
        if record.id.is_empty()
            || record.authority.is_empty()
            || record.url.is_empty()
            || record.retrieved_on.is_empty()
            || record.excerpt.is_empty()
            || record.statement.is_empty()
            || record.scope.is_empty()
            || record.source_ids.is_empty()
        {
            return Err(policy(
                "bundled provenance evidence records must be complete and source-scoped",
            ));
        }
        if result.insert(record.id.clone(), record).is_some() {
            return Err(policy(
                "bundled provenance evidence contains a duplicate id",
            ));
        }
    }
    Ok(result)
}

fn read_overrides(
    root: &Path,
    manifest: &Manifest,
    evidence: &BTreeMap<String, EvidenceRecord>,
) -> Result<BTreeMap<String, Override>> {
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
        if ACCEPTED_BUNDLE_CLASSIFICATIONS.contains(&record.classification.as_str()) {
            if record.confidence.is_empty()
                || record.named_authors.is_empty()
                || record.stated_institutions.is_empty()
                || record.author_prologue.as_deref().is_none_or(str::is_empty)
                || record.governing_notice.as_deref().is_none_or(str::is_empty)
                || record
                    .redistribution_conditions
                    .as_deref()
                    .is_none_or(str::is_empty)
                || record
                    .manual_override_provenance
                    .as_deref()
                    .is_none_or(str::is_empty)
                || record.evidence_ids.is_empty()
            {
                return Err(policy(&format!(
                    "accepted bundled provenance override {} is missing required review evidence",
                    record.id
                )));
            }
            for evidence_id in &record.evidence_ids {
                let item = evidence.get(evidence_id).ok_or_else(|| {
                    policy(&format!(
                        "bundled provenance override {} references unknown evidence {evidence_id}",
                        record.id
                    ))
                })?;
                if !item.source_ids.contains(&record.id) {
                    return Err(policy(&format!(
                        "bundled provenance evidence {evidence_id} does not cover source {}",
                        record.id
                    )));
                }
            }
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

fn family_archive(family: &str) -> Option<&'static str> {
    (family == FIRST_BUNDLED_FAMILY).then_some(FIRST_BUNDLED_ARCHIVE)
}

fn read_build_receipt(carrier_root: &Path) -> Result<Option<Value>> {
    let path = carrier_root.join(BUNDLED_BUILD_RECEIPT);
    if !path.is_file() {
        return Ok(None);
    }
    Ok(Some(serde_json::from_slice(&fs::read(path)?)?))
}

fn archive_record(
    carrier_root: &Path,
    archive: &str,
    receipt: &Option<Value>,
) -> Result<Option<Value>> {
    let path = carrier_root.join(archive);
    if !path.is_file() {
        return Ok(None);
    }
    let receipt = receipt.as_ref().ok_or_else(|| {
        policy("a bundled archive is present without its deterministic build receipt")
    })?;
    let expected = receipt["archives"]
        .as_array()
        .and_then(|items| {
            items
                .iter()
                .find(|item| item["path"].as_str() == Some(archive))
        })
        .and_then(|item| item["sha256"].as_str())
        .ok_or_else(|| policy("bundled build receipt lacks an archive checksum"))?;
    let actual = hash::file(&path)?;
    if actual != expected {
        return Err(policy(&format!(
            "bundled archive checksum changed for {archive}: expected {expected}, found {actual}"
        )));
    }
    Ok(Some(
        json!({"path":archive,"sha256":actual,"size_bytes":fs::metadata(path)?.len()}),
    ))
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

fn third_party_notices(provenance: &Value, carrier_status: &str) -> String {
    let public_domain = provenance["records"]
        .as_array()
        .expect("provenance records")
        .iter()
        .filter(|record| record["redistribution_classification"] == "explicit-public-domain")
        .count();
    let unresolved = provenance["records"]
        .as_array()
        .expect("provenance records")
        .iter()
        .filter(|record| !record["bundle_eligible"].as_bool().unwrap_or(false))
        .count();
    let status = if carrier_status == "ready" {
        format!(
            "The carrier includes the `{FIRST_BUNDLED_FAMILY}` archive built from {public_domain} hash-guarded historical SLATEC units plus three project-profile units. Its historical units are classified `explicit-public-domain` from the cited NIST/NTIS statements and Version 4.1 Netlib subset scope.\n\nThe carrier also includes unmodified static `libgfortran` (GPL-3.0 with the GCC Runtime Library Exception) and `libquadmath` (LGPL-2.1-or-later) only because the audited final GNU consumer link requires them. Preserve the notices and provide the applicable corresponding-source or relinking information. The final executable is linked from ordinary static archives, so an end user can relink it with a compatible replacement runtime.\n"
        )
    } else {
        "No historical SLATEC source or GNU runtime archive is included while no family is ready.\n"
            .to_owned()
    };
    format!(
        "# Third-party notices for the bundled provider\n\n{status}\n{unresolved} selected physical source units remain outside every distributable bundle pending source-specific provenance evidence. Their status is not relaxed by the first bundle.\n\nSources: NIST ACMD SLATEC public-domain statement (<https://math.nist.gov/mcsd/Reports/95/yearly/node59.html>), NTIS SLADOC record (<https://ntrl.ntis.gov/NTRL/dashboard/searchResults/titleDetail/DE89003746.xhtml>), and the Netlib Version 4.1 index (<https://netlib.org/slatec/>). GNU runtime notices: <https://www.gnu.org/licenses/gcc-exception-3.1.html> and <https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html>.\n"
    )
}

fn redistribution_notice(provenance: &Value, carrier_status: &str) -> String {
    let source_count = provenance["summary"]["classification_counts"]["explicit-public-domain"]
        .as_u64()
        .unwrap_or_default();
    format!(
        "# Bundled-provider redistribution notice\n\nCarrier status: `{carrier_status}`. The only historical SLATEC family currently distributed by this carrier is `{FIRST_BUNDLED_FAMILY}`. Its {source_count} hash-pinned historical source units have source-specific `explicit-public-domain` records in the generated provenance inventory. This is an evidence record, not legal advice.\n\nDo not treat this notice as clearance for any other SLATEC source, family, overlay, cache, archive, or Netlib-hosted material. The static GNU runtime archives are separately identified in `bundle-build-receipt.json` and subject to their own notices and source/relinking obligations.\n"
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
    fn generated_audit_is_source_hash_guarded_and_admits_only_the_reviewed_family() {
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
            provenance["summary"]["classification_counts"]["explicit-public-domain"],
            28
        );
        let unresolved = provenance["summary"]["classification_counts"]["unresolved-rights"]
            .as_u64()
            .unwrap_or_default()
            + provenance["summary"]["classification_counts"]["unresolved-authorship"]
                .as_u64()
                .unwrap_or_default();
        assert_eq!(
            unresolved + 28,
            provenance["summary"]["physical_source_units"]
                .as_u64()
                .unwrap_or_default()
        );
        let eligibility: Value = serde_json::from_slice(
            artifacts
                .get(&root.join("generated/licensing/bundled-source-eligibility.json"))
                .expect("eligibility artifact"),
        )
        .expect("valid eligibility");
        assert_eq!(eligibility["carrier"]["status"], "ready");
        let elementary = eligibility["families"]
            .as_array()
            .and_then(|families| {
                families
                    .iter()
                    .find(|family| family["feature"] == FIRST_BUNDLED_FAMILY)
            })
            .expect("elementary family record");
        assert_eq!(elementary["status"], "ready");
        assert!(elementary["bundle_available"].as_bool().unwrap_or(false));
        let gamma = eligibility["families"]
            .as_array()
            .and_then(|families| {
                families
                    .iter()
                    .find(|family| family["feature"] == "special-gamma")
            })
            .expect("gamma family record");
        assert_eq!(gamma["status"], "blocked_by_source_provenance");
        assert!(!gamma["bundle_available"].as_bool().unwrap_or(true));
        let sbom: Value = serde_json::from_slice(
            artifacts
                .get(&root.join("generated/licensing/bundled-sbom.spdx.json"))
                .expect("SBOM artifact"),
        )
        .expect("valid SPDX JSON");
        let files = sbom["files"].as_array().expect("SBOM files");
        for name in [
            FIRST_BUNDLED_ARCHIVE,
            BUNDLED_GFORTRAN_ARCHIVE,
            BUNDLED_QUADMATH_ARCHIVE,
        ] {
            assert!(files.iter().any(|file| file["fileName"] == name));
        }
    }

    #[test]
    fn changed_override_hash_invalidates_an_approval() {
        let root = std::env::temp_dir().join(format!(
            "slatec-bundled-provider-hash-guard-{}",
            std::process::id()
        ));
        let input = root.join("crates/slatec-src/metadata");
        fs::create_dir_all(&input).expect("create test provenance input");
        fs::write(
            input.join("bundled-provenance-overrides.json"),
            r#"{"sources":[{"id":"source-a","sha256":"changed","classification":"explicit-public-domain","confidence":"reviewed","named_authors":["Author"],"stated_institutions":["Institution"],"author_prologue":"author","governing_notice":"notice","redistribution_conditions":"conditions","evidence_ids":["evidence-a"],"manual_override_provenance":"review"}]}"#,
        )
        .expect("write test override");
        let manifest = Manifest {
            sources: vec![Source {
                id: "source-a".to_owned(),
                subset: "fnlib".to_owned(),
                path: "example.f".to_owned(),
                sha256: "expected".to_owned(),
                url: None,
            }],
            families: BTreeMap::new(),
        };
        let error = match read_overrides(&root, &manifest, &BTreeMap::new()) {
            Ok(_) => panic!("a changed source hash must invalidate approval"),
            Err(error) => error,
        };
        assert!(
            error
                .to_string()
                .contains("bundled provenance override hash changed for source-a")
        );
        fs::remove_dir_all(root).expect("remove test provenance input");
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
