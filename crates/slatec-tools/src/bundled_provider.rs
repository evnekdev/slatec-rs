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
const WINDOWS_TARGET: &str = "x86_64-pc-windows-gnu";
const LINUX_TARGET: &str = "x86_64-unknown-linux-gnu";

struct BundleTarget {
    rust_target: &'static str,
    compiler_target: &'static str,
    compiler_version: &'static str,
    carrier_directory: &'static str,
    carrier_crate: &'static str,
    profile_directory: &'static str,
    executable_name: &'static str,
    dependency_format: DependencyFormat,
}

struct ArchiveBuildEnvironment<'a> {
    root: &'a Path,
    cache: &'a Path,
    compiler: &'a Path,
    target: &'a BundleTarget,
}

struct ConsumerEnvironment<'a> {
    remove_env: &'a [&'a str],
    invalid_env: Option<(&'a str, &'a str)>,
    source_build: Option<(&'a Path, &'a Path)>,
}

#[derive(Clone, Copy)]
enum DependencyFormat {
    Pe,
    Elf,
}

const WINDOWS_BUNDLE_TARGET: BundleTarget = BundleTarget {
    rust_target: WINDOWS_TARGET,
    compiler_target: "x86_64-w64-mingw32",
    compiler_version: "14.2.0",
    carrier_directory: "crates/slatec-bundled-x86_64-pc-windows-gnu",
    carrier_crate: "slatec-bundled-x86_64-pc-windows-gnu",
    profile_directory: "gnu-mingw-x86_64",
    executable_name: "slatec-bundled-consumer.exe",
    dependency_format: DependencyFormat::Pe,
};

const LINUX_BUNDLE_TARGET: BundleTarget = BundleTarget {
    rust_target: LINUX_TARGET,
    compiler_target: "x86_64-linux-gnu",
    compiler_version: "11.4.0",
    carrier_directory: "crates/slatec-bundled-x86_64-unknown-linux-gnu",
    carrier_crate: "slatec-bundled-x86_64-unknown-linux-gnu",
    profile_directory: "gnu-linux-x86_64",
    executable_name: "slatec-bundled-consumer",
    dependency_format: DependencyFormat::Elf,
};

fn selected_bundle_target() -> Result<&'static BundleTarget> {
    match env::var("SLATEC_BUNDLED_TARGET").as_deref() {
        Err(_) | Ok(WINDOWS_TARGET) => Ok(&WINDOWS_BUNDLE_TARGET),
        Ok(LINUX_TARGET) => Ok(&LINUX_BUNDLE_TARGET),
        Ok(other) => Err(policy(&format!(
            "SLATEC_BUNDLED_TARGET must be {WINDOWS_TARGET} or {LINUX_TARGET}; found {other}"
        ))),
    }
}
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
const BUNDLED_ARCHIVE_FAMILY: &str = "accepted-source-closure";
const BUNDLED_ARCHIVE: &str = "native/libslatec_bundle_accepted.a";
const BUNDLED_GFORTRAN_ARCHIVE: &str = "native/libslatec_bundle_gfortran.a";
const BUNDLED_QUADMATH_ARCHIVE: &str = "native/libslatec_bundle_quadmath.a";
const BUNDLED_BUILD_RECEIPT: &str = "metadata/bundle-build-receipt.json";
const PROFILE_SOURCES: &[&str] = &["i1mach.f", "r1mach.f", "d1mach.f"];
const COMPILE_FLAGS: &[&str] = &[
    "-x",
    "f77",
    "-std=f95",
    "-ffixed-line-length-none",
    "-fno-ident",
    "-frandom-seed=slatec-bundled-accepted-source-closure",
    "-c",
];
const PROFILE_COMPILE_FLAGS: &[&str] = &[
    "-x",
    "f77",
    "-std=f2008",
    "-ffixed-line-length-none",
    "-fno-ident",
    "-frandom-seed=slatec-bundled-accepted-source-closure",
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
    #[serde(default)]
    scopes: Vec<ScopeApproval>,
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
    #[serde(default)]
    source_scope: Option<SourceScope>,
    source_ids: Vec<String>,
}

#[derive(Clone, Deserialize)]
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
    #[serde(default)]
    scope_approval_id: Option<String>,
}

#[derive(Deserialize)]
struct ScopeApproval {
    id: String,
    families: Vec<String>,
    source_set_sha256: String,
    source_count: usize,
    classification: String,
    confidence: String,
    #[serde(default)]
    stated_institutions: Vec<String>,
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
struct SourceScope {
    families: Vec<String>,
    source_set_sha256: String,
    source_count: usize,
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

/// Generates deterministic Linux carrier metadata without compiling native
/// code. The archive itself is produced only by the target-selected builder.
pub fn generate_linux(root: &Path) -> Result<String> {
    let artifacts = linux_artifacts(root, &LINUX_BUNDLE_TARGET)?;
    for (path, bytes) in &artifacts {
        write_if_changed(path, bytes)?;
    }
    Ok(semantic_hash(&artifacts))
}

/// Validates that every generated Linux carrier metadata artifact is current.
pub fn validate_linux(root: &Path) -> Result<String> {
    let artifacts = linux_artifacts(root, &LINUX_BUNDLE_TARGET)?;
    for (path, expected) in &artifacts {
        let actual = fs::read(path).map_err(|error| {
            CorpusError::Verification(format!(
                "missing Linux bundled-provider artifact {}: {error}",
                path.display()
            ))
        })?;
        if actual != *expected {
            return Err(CorpusError::Verification(format!(
                "Linux bundled-provider artifact drifted: {}; regenerate with generate-linux-bundled-provider-evidence",
                path.display()
            )));
        }
    }
    Ok(semantic_hash(&artifacts))
}

/// Runs the Linux carrier's clean-consumer and source-build parity probes
/// against an already receipt-verified deterministic archive.
pub fn validate_linux_carrier(root: &Path) -> Result<()> {
    let target = &LINUX_BUNDLE_TARGET;
    let cache = env::var_os("SLATEC_SOURCE_CACHE")
        .map(PathBuf::from)
        .ok_or_else(|| policy("Linux carrier validation requires SLATEC_SOURCE_CACHE"))?;
    let compiler = env::var_os("SLATEC_GFORTRAN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("gfortran"));
    verify_bundled_compiler(&compiler, target)?;
    let carrier_root = root.join(target.carrier_directory);
    let mut receipt = read_build_receipt(&carrier_root)?.ok_or_else(|| {
        policy("Linux carrier validation requires a deterministic bundle-build-receipt.json")
    })?;
    let archive = carrier_root.join(BUNDLED_ARCHIVE);
    let expected = receipt["archives"]
        .as_array()
        .and_then(|archives| archives.first())
        .and_then(|archive| archive["sha256"].as_str())
        .ok_or_else(|| policy("Linux carrier receipt lacks its archive checksum"))?;
    let actual = hash::file(&archive)?;
    if actual != expected {
        return Err(policy(&format!(
            "Linux carrier archive checksum mismatch: expected {expected}, found {actual}"
        )));
    }
    let consumer_probe = probe_clean_consumers(root, &cache, &compiler, target)?;
    receipt["runtime_audit"]["status"] = Value::String("ready".to_owned());
    receipt["runtime_audit"]["consumer_dependencies"] =
        consumer_probe["bundled_native_dependencies"].clone();
    receipt["runtime_audit"]["clean_consumer"] = consumer_probe;
    write_json(&carrier_root.join(BUNDLED_BUILD_RECEIPT), &receipt)?;
    generate_target_evidence(root, target)
}

fn semantic_hash(artifacts: &BTreeMap<PathBuf, Vec<u8>>) -> String {
    hash::bytes(
        &artifacts
            .iter()
            .flat_map(|(path, bytes)| {
                let mut value = path.to_string_lossy().as_bytes().to_vec();
                value.extend_from_slice(bytes);
                value
            })
            .collect::<Vec<_>>(),
    )
}

/// Builds one target-specific accepted-source archive only after every selected
/// family closure passes the source-level redistribution gate. The source cache
/// and compiler are intentionally touched only after that decision.
pub fn require_buildable(root: &Path) -> Result<()> {
    let target = selected_bundle_target()?;
    let manifest = materialized_manifest(root)?;
    let evidence = read_evidence(root)?;
    let overrides = read_overrides(root, &manifest, &evidence)?;
    let source_by_id = manifest
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let included_families = manifest
        .families
        .iter()
        .filter(|(_, ids)| !ids.is_empty())
        .filter(|(_, ids)| {
            ids.iter().all(|id| {
                source_by_id.get(id.as_str()).is_some_and(|source| {
                    overrides.get(source.id.as_str()).is_some_and(|record| {
                        ACCEPTED_BUNDLE_CLASSIFICATIONS.contains(&record.classification.as_str())
                    })
                })
            })
        })
        .map(|(family, _)| family.clone())
        .collect::<Vec<_>>();
    if included_families.is_empty() {
        return Err(policy(
            "no bundled family has a complete accepted source closure; no compiler or source cache was consulted",
        ));
    }
    let selected = included_families
        .iter()
        .flat_map(|family| manifest.families[family].iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    let cache = env::var_os("SLATEC_SOURCE_CACHE")
        .map(PathBuf::from)
        .ok_or_else(|| {
            policy("build-bundled-provider requires SLATEC_SOURCE_CACHE after provenance clearance")
        })?;
    let compiler = env::var_os("SLATEC_GFORTRAN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("gfortran"));
    verify_bundled_compiler(&compiler, target)?;
    let carrier_root = root.join(target.carrier_directory);
    let stage = root
        .join("target/bundled-provider")
        .join(BUNDLED_ARCHIVE_FAMILY);
    if stage.exists() {
        fs::remove_dir_all(&stage)?;
    }
    fs::create_dir_all(&stage)?;
    let archive = stage.join("libslatec_bundle_accepted.a");
    let archive_environment = ArchiveBuildEnvironment {
        root,
        cache: &cache,
        compiler: &compiler,
        target,
    };
    let (members, symbols, undefined, source_records) = build_family_archive(
        &source_by_id,
        &selected,
        &archive_environment,
        &stage,
        &archive,
    )?;
    let reproducibility_stage = root
        .join("target/bundled-provider")
        .join("reproducibility")
        .join(BUNDLED_ARCHIVE_FAMILY);
    if reproducibility_stage.exists() {
        fs::remove_dir_all(&reproducibility_stage)?;
    }
    fs::create_dir_all(&reproducibility_stage)?;
    let reproducibility_archive = reproducibility_stage.join("libslatec_bundle_accepted.a");
    let _ = build_family_archive(
        &source_by_id,
        &selected,
        &archive_environment,
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
    let carrier_archive = carrier_root.join(BUNDLED_ARCHIVE);
    fs::copy(&archive, &carrier_archive)?;
    let obsolete_family_archive = carrier_root.join("native/libslatec_bundle_special_elementary.a");
    if obsolete_family_archive.is_file() {
        fs::remove_file(obsolete_family_archive)?;
    }
    let mut runtime_archives = Vec::new();
    let mut runtime_member_audits = Vec::new();
    let mut quadmath_required = false;
    if runtime_required {
        let runtime_source = static_runtime_archive(&compiler, "libgfortran.a")?;
        let runtime_destination = carrier_root.join(BUNDLED_GFORTRAN_ARCHIVE);
        let gfortran_seed = undefined
            .iter()
            .filter(|symbol| symbol.starts_with("_gfortran_"))
            .cloned()
            .collect::<Vec<_>>();
        let reduction = reduce_static_runtime_archive(
            &compiler,
            &runtime_source,
            &runtime_destination,
            &gfortran_seed,
            &stage.join("gfortran"),
        )?;
        runtime_archives.push(json!({
            "component":"libgfortran","path":BUNDLED_GFORTRAN_ARCHIVE,"sha256":hash::file(&runtime_destination)?,
            "link_name":"slatec_bundle_gfortran","linking_mode":"static-reduced-closure","source":runtime_source.display().to_string(),
            "upstream_sha256":hash::file(&runtime_source)?,"upstream_size_bytes":fs::metadata(&runtime_source)?.len(),"retained_member_count":reduction.selected_members.len(),"retained_members":reduction.selected_members,
            "externally_resolved_symbols":reduction.externally_resolved_symbols,"ambiguous_symbol_providers":reduction.ambiguous_symbols,"undefined_symbols":reduction.undefined_symbols,
            "license":"GPL-3.0-with-GCC-exception","obligations":"Preserve the GCC Runtime Library Exception notice; the final consumer remains relinkable because Cargo links ordinary static archives."
        }));
        let quadmath_source = static_runtime_archive(&compiler, "libquadmath.a")?;
        let quadmath_destination = carrier_root.join(BUNDLED_QUADMATH_ARCHIVE);
        let quadmath_reduction = reduce_static_runtime_archive(
            &compiler,
            &quadmath_source,
            &quadmath_destination,
            &runtime_archives
                .last()
                .and_then(|record| record.get("externally_resolved_symbols"))
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>(),
            &stage.join("quadmath"),
        )?;
        quadmath_required = !quadmath_reduction.selected_members.is_empty();
        runtime_archives.push(json!({
            "component":"libquadmath","path":BUNDLED_QUADMATH_ARCHIVE,"sha256":hash::file(&quadmath_destination)?,
            "link_name":"slatec_bundle_quadmath","linking_mode":"static-reduced-closure","source":quadmath_source.display().to_string(),
            "upstream_sha256":hash::file(&quadmath_source)?,"upstream_size_bytes":fs::metadata(&quadmath_source)?.len(),"retained_member_count":quadmath_reduction.selected_members.len(),"retained_members":quadmath_reduction.selected_members,
            "externally_resolved_symbols":quadmath_reduction.externally_resolved_symbols,"ambiguous_symbol_providers":quadmath_reduction.ambiguous_symbols,"undefined_symbols":quadmath_reduction.undefined_symbols,
            "license":"LGPL-2.1-or-later","obligations":"Preserve the LGPL notice and provide the corresponding-source or relinking information required for the static runtime archive. The retained quadmath closure is selected because libgfortran formatted-write members reference quadmath_snprintf; the SLATEC archive has no direct quadmath reference."
        }));
        runtime_member_audits.push(json!({"component":"libgfortran","seed_symbols":gfortran_seed}));
        runtime_member_audits.push(json!({"component":"libquadmath","causal_symbol":"quadmath_snprintf","causal_chain":"SLATEC XERROR diagnostic formatting -> libgfortran write.o/transfer128.o -> quadmath_snprintf -> libquadmath quadmath-printf.o"}));
    }
    let receipt = json!({
        "schema_id":"slatec-rs/bundled-build-receipt","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":target.rust_target,
        "family":BUNDLED_ARCHIVE_FAMILY,
        "compiler":{"path":compiler.display().to_string(),"version":command_output(&compiler, &["--version"])? .lines().next().unwrap_or_default(),"target":command_output(&compiler, &["-dumpmachine"])? .trim()},
        "historical_source_compile_flags":COMPILE_FLAGS,"profile_source_compile_flags":PROFILE_COMPILE_FLAGS,"archiver":sibling_tool(&compiler, "ar").display().to_string(),"archiver_flags":"crsD",
        "archives":[{"family":BUNDLED_ARCHIVE_FAMILY,"families":included_families,"path":BUNDLED_ARCHIVE,"sha256":archive_sha256,"size_bytes":fs::metadata(&carrier_archive)?.len()}],
        "runtime_archives":runtime_archives,
        "source_units":source_records,
        "archive_audit":{"schema_id":"slatec-rs/bundled-archive-audit","schema_version":"3.0.0","snapshot_id":SNAPSHOT,"target":target.rust_target,"status":"ready","archive_model":"one accepted-source archive; family eligibility remains exact and the static linker extracts only referenced members","archive_members":members,"defined_symbols":symbols,"undefined_symbols":undefined,"included_families":included_families,"included_source_ids":selected,"imported_dlls":[],"same_root_reproduction":"sha256_match"},
        "runtime_audit":{"schema_id":"slatec-rs/bundled-runtime-audit","schema_version":"3.0.0","snapshot_id":SNAPSHOT,"target":target.rust_target,"status":"static_runtime_pending_consumer_probe","compiler_profile":format!("GNU Fortran {} / {}", target.compiler_version, target.compiler_target),"compiler_invoked":true,"source_cache_read":true,"network_access":false,"runtime_member_audits":runtime_member_audits,"runtime_components":[
            {"component":"libgfortran","static_or_dynamic":if runtime_required { "static reduced closure" } else { "not required" },"actually_referenced":runtime_required,"distribution_action":if runtime_required { "include reduced libslatec_bundle_gfortran.a with GPLv3 and Runtime Library Exception notice" } else { "not included" }},
            {"component":"libquadmath","static_or_dynamic":if quadmath_required { "static reduced closure" } else { "not required" },"actually_referenced":if quadmath_required { "required by libgfortran formatted-write members through quadmath_snprintf, not by the SLATEC archive" } else { "not required by the reduced runtime closure" },"distribution_action":if quadmath_required { "include reduced libslatec_bundle_quadmath.a with LGPL-2.1-or-later notice and source/relinking information" } else { "not included" }},
            {"component":"libgcc","static_or_dynamic":"toolchain-provided final-link support","actually_referenced":"not separately bundled","distribution_action":"no carrier copy; audit final consumer imports"}
        ],"imported_dlls":[],"reason":"Archive-level audit complete; final consumer import probe is recorded by the release validation command."}
    });
    write_json(&carrier_root.join(BUNDLED_BUILD_RECEIPT), &receipt)?;
    generate_target_evidence(root, target)?;
    let consumer_probe = probe_clean_consumers(root, &cache, &compiler, target)?;
    let mut final_receipt: Value =
        serde_json::from_slice(&fs::read(carrier_root.join(BUNDLED_BUILD_RECEIPT))?)?;
    final_receipt["runtime_audit"]["status"] = Value::String("ready".to_owned());
    final_receipt["runtime_audit"]["consumer_dependencies"] =
        consumer_probe["bundled_native_dependencies"].clone();
    final_receipt["runtime_audit"]["clean_consumer"] = consumer_probe.clone();
    write_json(&carrier_root.join(BUNDLED_BUILD_RECEIPT), &final_receipt)?;
    generate_target_evidence(root, target)?;
    Ok(())
}

fn verify_bundled_compiler(compiler: &Path, target: &BundleTarget) -> Result<()> {
    let compiler_target = command_output(compiler, &["-dumpmachine"])?;
    if compiler_target.trim() != target.compiler_target {
        return Err(policy(&format!(
            "bundled archive production for {} requires GNU Fortran target {}; found {}",
            target.rust_target,
            target.compiler_target,
            compiler_target.trim(),
        )));
    }
    let version = command_output(compiler, &["--version"])?;
    if !version.contains(target.compiler_version) {
        return Err(policy(&format!(
            "bundled archive production for {} requires the reviewed GNU Fortran {} toolchain",
            target.rust_target, target.compiler_version
        )));
    }
    Ok(())
}

fn build_family_archive(
    source_by_id: &BTreeMap<&str, &Source>,
    selected: &[String],
    environment: &ArchiveBuildEnvironment<'_>,
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
        let source_path = verified_cached_source(environment.cache, source)?;
        let object = objects.join(format!("{}.o", source.id));
        compile_source(environment.compiler, COMPILE_FLAGS, &source_path, &object)?;
        object_paths.push(object);
        source_records.push(json!({
            "id":source.id,"subset":source.subset,"path":source.path,"sha256":source.sha256,
            "cached_source_sha256":hash::file(&source_path)?,"object":format!("{}.o",source.id)
        }));
    }
    for profile in PROFILE_SOURCES {
        let source = environment
            .root
            .join("crates/slatec-src/native")
            .join(environment.target.profile_directory)
            .join(profile);
        let object = objects.join(format!("profile-{}.o", profile.trim_end_matches(".f")));
        let object_name = object
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_owned();
        compile_source(
            environment.compiler,
            PROFILE_COMPILE_FLAGS,
            &source,
            &object,
        )?;
        object_paths.push(object);
        source_records.push(json!({
            "id":format!("profile-{}",profile.trim_end_matches(".f")),"subset":"slatec-rs-profile","path":format!("native/{}/{profile}", environment.target.profile_directory),"sha256":hash::file(&source)?,"cached_source_sha256":hash::file(&source)?,"object":object_name
        }));
    }
    object_paths.sort();
    let ar = sibling_tool(environment.compiler, "ar");
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
    let nm = sibling_tool(environment.compiler, "nm");
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

fn compile_source(compiler: &Path, flags: &[&str], source: &Path, object: &Path) -> Result<()> {
    let output = Command::new(compiler)
        .args(flags)
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

struct ReducedRuntimeArchive {
    selected_members: Vec<String>,
    externally_resolved_symbols: Vec<String>,
    ambiguous_symbols: Vec<String>,
    undefined_symbols: Vec<String>,
}

/// Materialize the deterministic member closure of a static runtime archive.
///
/// This deliberately follows only global undefined symbols from members that
/// satisfy the selected closure. Symbols without a definition in this archive
/// are recorded as final-link dependencies rather than guessed or copied from
/// another runtime.
fn reduce_static_runtime_archive(
    compiler: &Path,
    source: &Path,
    destination: &Path,
    seed_symbols: &[String],
    stage: &Path,
) -> Result<ReducedRuntimeArchive> {
    let ar = sibling_tool(compiler, "ar");
    let nm = sibling_tool(compiler, "nm");
    let members = command_output(
        &ar,
        &[
            "t",
            source
                .to_str()
                .ok_or_else(|| policy("runtime archive path is not UTF-8"))?,
        ],
    )?
    .lines()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if members.is_empty() {
        return Err(policy("runtime archive has no members"));
    }
    let mut defined = BTreeMap::<String, BTreeSet<String>>::new();
    let mut undefined = BTreeMap::<String, BTreeSet<String>>::new();
    for member in &members {
        undefined.insert(member.clone(), BTreeSet::new());
    }
    let source_text = source
        .to_str()
        .ok_or_else(|| policy("runtime archive path is not UTF-8"))?;
    let defined_output = command_output(&nm, &["-A", "-g", "--defined-only", source_text])?;
    for line in defined_output.lines() {
        let Some((member, symbol)) = archive_nm_member_symbol(line, source) else {
            continue;
        };
        defined.entry(symbol).or_default().insert(member);
    }
    let undefined_output = command_output(&nm, &["-A", "-u", source_text])?;
    for line in undefined_output.lines() {
        let Some((member, symbol)) = archive_nm_member_symbol(line, source) else {
            continue;
        };
        undefined.entry(member).or_default().insert(symbol);
    }

    let mut pending = seed_symbols.iter().cloned().collect::<BTreeSet<_>>();
    let mut selected = BTreeSet::<String>::new();
    let mut external = BTreeSet::<String>::new();
    let mut ambiguous = BTreeSet::<String>::new();
    while let Some(symbol) = pending.pop_first() {
        let Some(candidates) = defined.get(&symbol) else {
            external.insert(symbol);
            continue;
        };
        let Some(member) = candidates.iter().next() else {
            external.insert(symbol);
            continue;
        };
        if candidates.len() > 1 {
            ambiguous.insert(symbol.clone());
        }
        if selected.insert(member.clone()) {
            pending.extend(undefined.get(member).into_iter().flatten().cloned());
        }
    }
    if selected.is_empty() {
        return Err(policy("runtime reduction selected no archive members"));
    }

    let objects = stage.join("runtime-objects");
    if objects.exists() {
        fs::remove_dir_all(&objects)?;
    }
    fs::create_dir_all(&objects)?;
    if selected.iter().any(|member| {
        Path::new(member).file_name().and_then(|name| name.to_str()) != Some(member.as_str())
    }) {
        return Err(policy(
            "runtime archive member path escaped its archive root",
        ));
    }
    let extraction = Command::new(&ar)
        .current_dir(&objects)
        .arg("x")
        .arg(source)
        .args(&selected)
        .output()?;
    if !extraction.status.success() {
        return Err(policy(&format!(
            "GNU ar could not extract reduced runtime members: {}",
            String::from_utf8_lossy(&extraction.stderr).trim()
        )));
    }
    let extracted = selected
        .iter()
        .map(|member| objects.join(member))
        .collect::<Vec<_>>();
    if extracted.iter().any(|object| !object.is_file()) {
        return Err(policy(
            "GNU ar extraction did not produce every selected runtime member",
        ));
    }
    if destination.exists() {
        fs::remove_file(destination)?;
    }
    let status = Command::new(&ar)
        .arg("crsD")
        .arg(destination)
        .args(&extracted)
        .status()?;
    if !status.success() {
        return Err(policy(&format!(
            "GNU ar failed creating reduced runtime archive {}",
            destination.display()
        )));
    }
    let selected_members = selected.into_iter().collect::<Vec<_>>();
    let undefined_symbols = selected_members
        .iter()
        .flat_map(|member| undefined.get(member).into_iter().flatten().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    Ok(ReducedRuntimeArchive {
        selected_members,
        externally_resolved_symbols: external.into_iter().collect(),
        ambiguous_symbols: ambiguous.into_iter().collect(),
        undefined_symbols,
    })
}

fn archive_nm_member_symbol(line: &str, archive: &Path) -> Option<(String, String)> {
    let archive_prefix = format!("{}:", archive.display());
    let suffix = line.strip_prefix(&archive_prefix)?;
    let (member, details) = suffix.split_once(':')?;
    let symbol = details.split_whitespace().last()?;
    if symbol.ends_with(':') || symbol.ends_with(".o:") {
        return None;
    }
    Some((member.to_owned(), symbol.to_owned()))
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

fn probe_clean_consumers(
    root: &Path,
    cache: &Path,
    compiler: &Path,
    target: &BundleTarget,
) -> Result<Value> {
    let probe_workspace = match target.dependency_format {
        DependencyFormat::Pe => root.join("target/bundled-provider"),
        DependencyFormat::Elf => env::temp_dir().join("slatec-bundled-provider-linux"),
    };
    let probe_root = probe_workspace.join("clean-consumer");
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
            "[workspace]\n\n[package]\nname = \"slatec-bundled-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\nslatec = {{ path = \"{slatec_path}/crates/slatec\", features = [\"special-elementary\", \"special-gamma\", \"special-beta\", \"special-error\", \"special-integrals\", \"special-polynomials\", \"special-airy\", \"special-bessel\", \"special-scalar-expanded\", \"roots-scalar\"] }}\n"
        ),
    )?;
    fs::write(
        &source,
        "fn main() { use slatec::special::{airy::airy_ai, bessel::bessel_j0, elementary::log1p, error_functions::erf, gamma::{beta, gamma}, integrals::exponential_integral_e1, scalar_expanded::{carlson_rf, spence_integral}}; use slatec::polynomials::chebyshev::chebyshev_series; use slatec::roots::{find_root, RootBracket, RootOptions}; let root = find_root(|x| x * x - 2.0, RootBracket { lower: 0.0, upper: 2.0 }, RootOptions::default()).expect(\"bracketed root\").estimate; let value = log1p(0.5).unwrap() + gamma(0.5).unwrap() + beta(1.0, 1.0).unwrap() + erf(0.5).unwrap() + exponential_integral_e1(1.0).unwrap() + chebyshev_series(0.0, &[1.0, 0.5]).unwrap() + airy_ai(0.0).unwrap() + bessel_j0(1.0).unwrap() + spence_integral(0.5).unwrap() + carlson_rf(1.0, 2.0, 3.0).unwrap() + root; println!(\"{value:.17}\"); }\n",
    )?;
    let target_dir = probe_workspace.join("clean-consumer-target");
    let bundled = run_consumer(
        root,
        &probe_root,
        &target_dir,
        &[],
        ConsumerEnvironment {
            remove_env: &[
                "SLATEC_SOURCE_CACHE",
                "SLATEC_SYSTEM_LIB_DIR",
                "SLATEC_SYSTEM_RUNTIME_LIB_DIR",
            ],
            invalid_env: Some((
                "SLATEC_GFORTRAN",
                "__bundled_provider_must_not_invoke_gfortran__",
            )),
            source_build: None,
        },
        target,
    )?;
    let source_build_root = probe_workspace.join("source-build-consumer");
    fs::create_dir_all(source_build_root.join("src"))?;
    fs::write(
        source_build_root.join("Cargo.toml"),
        format!(
            "[workspace]\n\n[package]\nname = \"slatec-source-build-parity\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\nslatec = {{ path = \"{slatec_path}/crates/slatec\", default-features = false, features = [\"std\", \"source-build\", \"special-elementary\", \"special-gamma\", \"special-beta\", \"special-error\", \"special-integrals\", \"special-polynomials\", \"special-airy\", \"special-bessel\", \"special-scalar-expanded\", \"roots-scalar\"] }}\n"
        ),
    )?;
    fs::write(
        source_build_root.join("src/main.rs"),
        "fn main() { use slatec::special::{airy::airy_ai, bessel::bessel_j0, elementary::log1p, error_functions::erf, gamma::{beta, gamma}, integrals::exponential_integral_e1, scalar_expanded::{carlson_rf, spence_integral}}; use slatec::polynomials::chebyshev::chebyshev_series; use slatec::roots::{find_root, RootBracket, RootOptions}; let root = find_root(|x| x * x - 2.0, RootBracket { lower: 0.0, upper: 2.0 }, RootOptions::default()).expect(\"bracketed root\").estimate; let value = log1p(0.5).unwrap() + gamma(0.5).unwrap() + beta(1.0, 1.0).unwrap() + erf(0.5).unwrap() + exponential_integral_e1(1.0).unwrap() + chebyshev_series(0.0, &[1.0, 0.5]).unwrap() + airy_ai(0.0).unwrap() + bessel_j0(1.0).unwrap() + spence_integral(0.5).unwrap() + carlson_rf(1.0, 2.0, 3.0).unwrap() + root; println!(\"{value:.17}\"); }\n",
    )?;
    let source_build = run_consumer(
        root,
        &source_build_root,
        &probe_workspace.join("source-build-parity-target"),
        &[],
        ConsumerEnvironment {
            remove_env: &[],
            invalid_env: None,
            source_build: Some((cache, compiler)),
        },
        target,
    )?;
    if bundled.0 != source_build.0 {
        return Err(policy(&format!(
            "bundled and source-build scalar release smoke values differ: {} versus {}",
            bundled.0, source_build.0
        )));
    }
    let executable = bundled
        .1
        .to_str()
        .ok_or_else(|| policy("consumer executable path is not UTF-8"))?;
    let imports = match target.dependency_format {
        DependencyFormat::Pe => {
            command_output(&sibling_tool(compiler, "objdump"), &["-p", executable])?
                .lines()
                .filter_map(|line| line.trim().strip_prefix("DLL Name: "))
                .map(str::to_owned)
                .collect::<Vec<_>>()
        }
        DependencyFormat::Elf => {
            command_output(&sibling_tool(compiler, "readelf"), &["-d", executable])?
                .lines()
                .filter_map(|line| line.split_once("Shared library: ["))
                .filter_map(|(_, value)| value.strip_suffix(']'))
                .map(str::to_owned)
                .collect::<Vec<_>>()
        }
    };
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
        "numeric_parity":"exact_rendered_f64","bundled_native_dependencies":imports,
        "environment":{"SLATEC_GFORTRAN":"intentionally invalid","SLATEC_SOURCE_CACHE":"absent","SLATEC_SYSTEM_LIB_DIR":"absent","SLATEC_SYSTEM_RUNTIME_LIB_DIR":"absent"}
    }))
}

fn run_consumer(
    root: &Path,
    manifest_root: &Path,
    target_dir: &Path,
    feature_args: &[&str],
    environment: ConsumerEnvironment<'_>,
    target: &BundleTarget,
) -> Result<(String, PathBuf)> {
    let mut command = Command::new("cargo");
    command
        .current_dir(root)
        .args(["run", "--manifest-path"])
        .arg(manifest_root.join("Cargo.toml"))
        .args(["--target", target.rust_target, "--offline"])
        .args(feature_args)
        .env("CARGO_TARGET_DIR", target_dir)
        .env("CARGO_NET_OFFLINE", "true");
    for key in environment.remove_env {
        command.env_remove(key);
    }
    if let Some((key, value)) = environment.invalid_env {
        command.env(key, value);
    }
    if let Some((source_cache, source_compiler)) = environment.source_build {
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
            .join(target.rust_target)
            .join("debug")
            .join(target.executable_name),
    ))
}

fn generate_target_evidence(root: &Path, target: &BundleTarget) -> Result<()> {
    if target.rust_target == WINDOWS_TARGET {
        generate(root)?;
        return Ok(());
    }
    for (path, bytes) in linux_artifacts(root, target)? {
        write_if_changed(&path, &bytes)?;
    }
    Ok(())
}

fn linux_artifacts(root: &Path, target: &BundleTarget) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    let manifest = materialized_manifest(root)?;
    let evidence = read_evidence(root)?;
    let overrides = read_overrides(root, &manifest, &evidence)?;
    let provenance: Value = serde_json::from_slice(&fs::read(
        root.join("generated/licensing/slatec-source-provenance.json"),
    )?)?;
    let source_by_id = manifest
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let carrier_root = root.join(target.carrier_directory);
    let receipt = read_build_receipt(&carrier_root)?;
    let mut families = Vec::new();
    for (feature, source_ids) in &manifest.families {
        let accepted = !source_ids.is_empty()
            && source_ids.iter().all(|id| {
                source_by_id.get(id.as_str()).is_some_and(|source| {
                    overrides.get(source.id.as_str()).is_some_and(|record| {
                        ACCEPTED_BUNDLE_CLASSIFICATIONS.contains(&record.classification.as_str())
                    })
                })
            });
        let archive_record = if accepted {
            archive_record_for_family(&carrier_root, source_ids, &receipt)?
        } else {
            None
        };
        let status = if source_ids.is_empty() {
            "no_selected_source_closure"
        } else if !accepted {
            "blocked_by_source_provenance"
        } else if archive_record.is_some() {
            "ready"
        } else {
            "eligible_pending_archive"
        };
        families.push(json!({
            "feature":feature,
            "source_unit_count":source_ids.len(),
            "bundle_available":status == "ready",
            "status":status,
            "archive":archive_record.as_ref().map(|_| BUNDLED_ARCHIVE),
            "archive_record":archive_record,
        }));
    }
    let carrier_status = if families.iter().any(|family| family["status"] == "ready") {
        "ready"
    } else if families
        .iter()
        .any(|family| family["status"] == "eligible_pending_archive")
    {
        "eligible_pending_archive"
    } else {
        "blocked_by_source_provenance"
    };
    let eligibility = json!({
        "schema_id":"slatec-rs/bundled-source-eligibility",
        "schema_version":"2.0.0",
        "snapshot_id":SNAPSHOT,
        "target":target.rust_target,
        "carrier":{"crate":target.carrier_crate,"status":carrier_status,"reason":"A family archive is available only when its complete hash-guarded source closure is present in this target's deterministic carrier receipt."},
        "families":families,
    });
    let receipt_value = receipt.clone().unwrap_or_else(|| json!({}));
    let source_unit_manifest = json!({
        "schema_id":"slatec-rs/bundled-source-unit-manifest",
        "schema_version":"2.0.0",
        "snapshot_id":SNAPSHOT,
        "target":target.rust_target,
        "source_units":receipt_value["source_units"],
    });
    let build_recipe = json!({
        "schema_id":"slatec-rs/bundled-build-recipe",
        "schema_version":"3.0.0",
        "target":target.rust_target,
        "compiler":format!("GNU Fortran {} / {}", target.compiler_version, target.compiler_target),
        "historical_source_compile_flags":COMPILE_FLAGS,
        "profile_source_compile_flags":PROFILE_COMPILE_FLAGS,
        "command":format!("SLATEC_BUNDLED_TARGET={} cargo run -p slatec-tools --bin slatec-corpus -- build-bundled-provider --offline", target.rust_target),
    });
    let carrier_manifest = json!({
        "schema_id":"slatec-rs/bundled-carrier-manifest",
        "schema_version":"2.0.0",
        "snapshot_id":SNAPSHOT,
        "target":target.rust_target,
        "status":carrier_status,
        "source_unit_count":receipt_value["source_units"].as_array().map_or(0, Vec::len),
        "source_eligibility":"metadata/bundled-source-eligibility.json",
        "source_unit_manifest":"metadata/source-unit-manifest.json",
        "build_recipe":"metadata/build-recipe.json",
        "build_receipt":BUNDLED_BUILD_RECEIPT,
        "runtime_audit":"metadata/bundle-build-receipt.json",
        "archive_audit":"metadata/bundle-build-receipt.json",
        "archives":families.iter().filter_map(|family| family["archive_record"].as_object().map(|_| json!({"family":family["feature"],"path":family["archive"],"sha256":family["archive_record"]["sha256"],"size_bytes":family["archive_record"]["size_bytes"]}))).collect::<Vec<_>>(),
        "runtime_archives":receipt_value["runtime_archives"].as_array().cloned().unwrap_or_default(),
        "reason":"This target-specific archive is built from exact accepted source closures and a deterministic receipt."
    });
    let mut artifacts = BTreeMap::new();
    insert_json(
        &mut artifacts,
        root.join("crates/slatec-src/metadata/bundled-source-eligibility-linux.json"),
        &eligibility,
    )?;
    insert_json(
        &mut artifacts,
        carrier_root.join("metadata/bundled-source-eligibility.json"),
        &eligibility,
    )?;
    insert_json(
        &mut artifacts,
        carrier_root.join("metadata/source-unit-manifest.json"),
        &source_unit_manifest,
    )?;
    insert_json(
        &mut artifacts,
        carrier_root.join("metadata/build-recipe.json"),
        &build_recipe,
    )?;
    insert_json(
        &mut artifacts,
        carrier_root.join("metadata/bundle-manifest.json"),
        &carrier_manifest,
    )?;
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/linux/bundled-source-eligibility.json"),
        &eligibility,
    )?;
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/linux/bundled-archive-audit.json"),
        &receipt_value["archive_audit"],
    )?;
    insert_json(
        &mut artifacts,
        root.join("generated/licensing/linux/bundled-runtime-audit.json"),
        &receipt_value["runtime_audit"],
    )?;
    insert_markdown(
        &mut artifacts,
        carrier_root.join("metadata/THIRD-PARTY-NOTICES.md"),
        third_party_notices(&provenance, carrier_status),
    );
    insert_markdown(
        &mut artifacts,
        carrier_root.join("metadata/REDISTRIBUTION-NOTICE.md"),
        redistribution_notice(&provenance, carrier_status),
    );
    for name in [
        "GPL-3.0.txt",
        "LGPL-2.1-or-later.txt",
        "GCC-RUNTIME-LIBRARY-EXCEPTION-3.1.txt",
    ] {
        artifacts.insert(
            carrier_root.join("metadata/runtime-licenses").join(name),
            read_lf_text(
                root.join("crates/slatec-bundled-x86_64-pc-windows-gnu/metadata/runtime-licenses")
                    .join(name),
            )?,
        );
    }
    artifacts.insert(
        carrier_root.join("metadata/runtime-licenses/SOURCES-AND-RELINKING.md"),
        format!(
            "# Runtime sources and relinking\n\nThis carrier was built with GNU Fortran {} for `{}`. Its receipt records the exact static `libgfortran` and `libquadmath` source archives, retained members, checksums, and external symbols. To relink, replace the ordinary static archives under `native/` with compatible rebuilt archives and rebuild the consuming Rust application. Preserve the GNU runtime notices shipped beside this file.\n",
            target.compiler_version, target.rust_target
        )
        .into_bytes(),
    );
    for name in ["LICENSE-APACHE", "LICENSE-MIT"] {
        artifacts.insert(carrier_root.join(name), read_lf_text(root.join(name))?);
    }
    Ok(artifacts)
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
            "named_authors_status":if override_record.is_some_and(|record| record.scope_approval_id.is_some()) { "source-prologue-not-extracted-in-exact-scope-approval" } else if override_record.is_some() { "reviewed_source_prologue" } else if author_field_present { "prologue_author_field_present_but_text_not_preserved_in_committed_index" } else { "unavailable" },
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
            "scope_approval_id":override_record.and_then(|record| record.scope_approval_id.clone()),
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
        let archive_ready = archive_record_for_family(&carrier_root, ids, &receipt)?;
        let archive = archive_ready.as_ref().map(|_| BUNDLED_ARCHIVE);
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
    let family_matrix = manifest
        .families
        .iter()
        .map(|(family, ids)| {
            let mut already_accepted = 0usize;
            let mut newly_accepted = 0usize;
            let mut unresolved = 0usize;
            for id in ids {
                match overrides.get(id) {
                    Some(record)
                        if ACCEPTED_BUNDLE_CLASSIFICATIONS
                            .contains(&record.classification.as_str()) =>
                    {
                        if record.scope_approval_id.is_some() {
                            newly_accepted += 1;
                        } else {
                            already_accepted += 1;
                        }
                    }
                    _ => unresolved += 1,
                }
            }
            let family_record = family_records
                .iter()
                .find(|record| record["feature"].as_str() == Some(family))
                .expect("constructed family record");
            json!({
                "family":family,
                "closure_size":ids.len(),
                "already_accepted":already_accepted,
                "newly_accepted":newly_accepted,
                "still_unresolved":unresolved,
                "runtime_closure":if family_record["bundle_available"].as_bool() == Some(true) { "reduced_static_gfortran_and_quadmath_as_recorded" } else { "not_bundled" },
                "bundle_action":family_record["status"],
                "archive":family_record["archive"],
            })
        })
        .collect::<Vec<_>>();
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
        "target":WINDOWS_TARGET,
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
            "schema_id":"slatec-rs/bundled-runtime-audit","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":WINDOWS_TARGET,
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
            "schema_id":"slatec-rs/bundled-archive-audit","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":WINDOWS_TARGET,
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
    let bundled_source_ids = receipt
        .as_ref()
        .and_then(|item| item["archive_audit"]["included_source_ids"].as_array())
        .map(|ids| {
            ids.iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let source_unit_manifest = json!({
        "schema_id":"slatec-rs/bundled-source-unit-manifest","schema_version":"2.0.0","snapshot_id":SNAPSHOT,
        "archive_model":"one accepted-source archive","family":BUNDLED_ARCHIVE_FAMILY,"source_ids":bundled_source_ids,
        "profile_sources":PROFILE_SOURCES,"records":provenance["records"].as_array().expect("records").iter().filter(|record| {
            record["selected_source_ids"].as_array().is_some_and(|ids| ids.iter().any(|id| bundled_source_ids.iter().any(|source_id| id.as_str() == Some(source_id))))
        }).collect::<Vec<_>>()
    });
    let build_recipe = json!({
        "schema_id":"slatec-rs/bundled-build-recipe","schema_version":"3.0.0","target":WINDOWS_TARGET,"family":BUNDLED_ARCHIVE_FAMILY,
        "archive_model":"one accepted-source archive; family records share one archive only when their full exact source closure is present",
        "compiler":"GNU Fortran 14.2.0 / x86_64-w64-mingw32","historical_source_compile_flags":COMPILE_FLAGS,"profile_source_compile_flags":PROFILE_COMPILE_FLAGS,
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
        root.join("generated/licensing/bundle-family-matrix.json"),
        &json!({
            "schema_id":"slatec-rs/bundle-family-matrix","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"target":WINDOWS_TARGET,
            "columns":["family","closure_size","already_accepted","newly_accepted","still_unresolved","runtime_closure","bundle_action","archive"],
            "records":family_matrix,
            "policy":"A family is bundled only when its complete exact source closure is accepted and present in the carrier receipt. Source-cache absence is a build blocker, not an implicit provenance approval."
        }),
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
            "schema_id":"slatec-rs/bundled-carrier-manifest","schema_version":"2.0.0","snapshot_id":SNAPSHOT,"target":WINDOWS_TARGET,
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
            || (record.source_ids.is_empty() && record.source_scope.is_none())
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
    let Overrides { sources, scopes } = overrides;
    let known = manifest
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source.sha256.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut result = BTreeMap::new();
    for record in sources {
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
    for scope in scopes {
        validate_scope_approval(&scope, manifest, evidence)?;
        let source_ids = source_ids_for_families(manifest, &scope.families)?;
        for id in source_ids {
            let source = manifest
                .sources
                .iter()
                .find(|source| source.id == id)
                .ok_or_else(|| policy("scope source disappeared from materialized manifest"))?;
            if result.contains_key(&source.id) {
                continue;
            }
            result.insert(
                source.id.clone(),
                Override {
                    id: source.id.clone(),
                    sha256: source.sha256.clone(),
                    classification: scope.classification.clone(),
                    confidence: scope.confidence.clone(),
                    named_authors: Vec::new(),
                    stated_institutions: scope.stated_institutions.clone(),
                    author_prologue: None,
                    governing_notice: scope.governing_notice.clone(),
                    redistribution_conditions: scope.redistribution_conditions.clone(),
                    evidence_ids: scope.evidence_ids.clone(),
                    manual_override_provenance: scope.manual_override_provenance.clone(),
                    scope_approval_id: Some(scope.id.clone()),
                },
            );
        }
    }
    Ok(result)
}

fn validate_scope_approval(
    scope: &ScopeApproval,
    manifest: &Manifest,
    evidence: &BTreeMap<String, EvidenceRecord>,
) -> Result<()> {
    if !ACCEPTED_BUNDLE_CLASSIFICATIONS.contains(&scope.classification.as_str())
        || scope.id.is_empty()
        || scope.families.is_empty()
        || scope.source_set_sha256.is_empty()
        || scope.source_count == 0
        || scope.confidence.is_empty()
        || scope.stated_institutions.is_empty()
        || scope.governing_notice.as_deref().is_none_or(str::is_empty)
        || scope
            .redistribution_conditions
            .as_deref()
            .is_none_or(str::is_empty)
        || scope
            .manual_override_provenance
            .as_deref()
            .is_none_or(str::is_empty)
        || scope.evidence_ids.is_empty()
    {
        return Err(policy(
            "accepted bundled provenance scope is missing required review evidence",
        ));
    }
    let ids = source_ids_for_families(manifest, &scope.families)?;
    if ids.len() != scope.source_count
        || source_set_hash(manifest, &ids)? != scope.source_set_sha256
    {
        return Err(policy(
            "bundled provenance scope source set changed; refresh the hash-guarded authored approval",
        ));
    }
    for evidence_id in &scope.evidence_ids {
        let item = evidence.get(evidence_id).ok_or_else(|| {
            policy(&format!(
                "bundled provenance scope {} references unknown evidence {evidence_id}",
                scope.id
            ))
        })?;
        let Some(evidence_scope) = &item.source_scope else {
            return Err(policy(&format!(
                "bundled provenance scope {} requires exact source-scope evidence",
                scope.id
            )));
        };
        let evidence_ids = source_ids_for_families(manifest, &evidence_scope.families)?;
        if evidence_scope.source_count != ids.len()
            || evidence_scope.source_set_sha256 != scope.source_set_sha256
            || evidence_ids != ids
        {
            return Err(policy(&format!(
                "bundled provenance evidence {evidence_id} does not cover the exact approved source set"
            )));
        }
    }
    Ok(())
}

fn source_ids_for_families(manifest: &Manifest, families: &[String]) -> Result<Vec<String>> {
    let mut ids = BTreeSet::new();
    for family in families {
        let closure = manifest.families.get(family).ok_or_else(|| {
            policy(&format!(
                "bundled provenance scope references unknown family {family}"
            ))
        })?;
        ids.extend(closure.iter().cloned());
    }
    if ids.is_empty() {
        return Err(policy("bundled provenance scope selects no source units"));
    }
    Ok(ids.into_iter().collect())
}

fn source_set_hash(manifest: &Manifest, ids: &[String]) -> Result<String> {
    let known = manifest
        .sources
        .iter()
        .map(|source| (source.id.as_str(), source.sha256.as_str()))
        .collect::<BTreeMap<_, _>>();
    let mut bytes = Vec::new();
    for id in ids {
        let sha256 = known.get(id.as_str()).ok_or_else(|| {
            policy(&format!(
                "approved source set references unknown source {id}"
            ))
        })?;
        bytes.extend_from_slice(id.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(sha256.as_bytes());
        bytes.push(b'\n');
    }
    Ok(hash::bytes(&bytes))
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

fn read_build_receipt(carrier_root: &Path) -> Result<Option<Value>> {
    let path = carrier_root.join(BUNDLED_BUILD_RECEIPT);
    if !path.is_file() {
        return Ok(None);
    }
    Ok(Some(serde_json::from_slice(&fs::read(path)?)?))
}

fn archive_record_for_family(
    carrier_root: &Path,
    family_source_ids: &[String],
    receipt: &Option<Value>,
) -> Result<Option<Value>> {
    // An empty selected closure is not evidence that this feature is bundled:
    // `Iterator::all` would otherwise accept it vacuously.  A feature becomes
    // available only after its non-empty exact closure is both approved and
    // present in the receipt.
    if family_source_ids.is_empty() {
        return Ok(None);
    }
    let path = carrier_root.join(BUNDLED_ARCHIVE);
    if !path.is_file() {
        return Ok(None);
    }
    let receipt = receipt.as_ref().ok_or_else(|| {
        policy("a bundled archive is present without its deterministic build receipt")
    })?;
    let archive = receipt["archives"]
        .as_array()
        .and_then(|items| {
            items
                .iter()
                .find(|item| item["path"].as_str() == Some(BUNDLED_ARCHIVE))
        })
        .ok_or_else(|| policy("bundled build receipt lacks an archive checksum"))?;
    let expected = archive["sha256"]
        .as_str()
        .ok_or_else(|| policy("bundled build receipt archive checksum is not a string"))?;
    let included = receipt["archive_audit"]["included_source_ids"]
        .as_array()
        .ok_or_else(|| policy("bundled build receipt lacks included source identities"))?;
    if !family_source_ids.iter().all(|id| {
        included
            .iter()
            .any(|included_id| included_id.as_str() == Some(id))
    }) {
        return Ok(None);
    }
    let actual = hash::file(&path)?;
    if actual != expected {
        return Err(policy(&format!(
            "bundled archive checksum changed for {BUNDLED_ARCHIVE}: expected {expected}, found {actual}"
        )));
    }
    Ok(Some(
        json!({"path":BUNDLED_ARCHIVE,"sha256":actual,"size_bytes":fs::metadata(path)?.len()}),
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
            "The carrier includes one `{BUNDLED_ARCHIVE_FAMILY}` archive built from {public_domain} hash-guarded historical SLATEC units plus three project-profile units. Each family remains available only when its exact source closure is contained in that archive; static linking extracts only the members reached by the final program. Its historical units are classified `explicit-public-domain` from the cited NIST/NTIS statements and Version 4.1 Netlib subset scope.\n\nThe carrier also includes static `libgfortran` (GPL-3.0 with the GCC Runtime Library Exception) and `libquadmath` (LGPL-2.1-or-later) only because the audited final GNU consumer link requires their retained members. Preserve the notices and provide the applicable corresponding-source or relinking information. The final executable is linked from ordinary static archives, so an end user can relink it with a compatible replacement runtime.\n"
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
        "# Bundled-provider redistribution notice\n\nCarrier status: `{carrier_status}`. The carrier distributes one `{BUNDLED_ARCHIVE_FAMILY}` archive. A family is available only when its entire exact source closure is recorded in that archive; its {source_count} hash-pinned historical source units have source-specific `explicit-public-domain` records in the generated provenance inventory. This is an evidence record, not legal advice.\n\nDo not treat this notice as clearance for any other SLATEC source, family, overlay, cache, archive, or Netlib-hosted material. The static GNU runtime archives are separately identified in `bundle-build-receipt.json` and subject to their own notices and source/relinking obligations.\n"
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

fn read_lf_text(path: PathBuf) -> Result<Vec<u8>> {
    let content = String::from_utf8(fs::read(&path)?)
        .map_err(|_| policy(&format!("expected UTF-8 text at {}", path.display())))?;
    Ok(content
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .into_bytes())
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
    fn generated_audit_is_source_hash_guarded_and_admits_only_exactly_cleared_families() {
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
            129
        );
        let unresolved = provenance["summary"]["classification_counts"]["unresolved-rights"]
            .as_u64()
            .unwrap_or_default()
            + provenance["summary"]["classification_counts"]["unresolved-authorship"]
                .as_u64()
                .unwrap_or_default();
        assert_eq!(
            unresolved + 129,
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
                    .find(|family| family["feature"] == "special-elementary")
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
        assert_eq!(gamma["status"], "ready");
        assert!(gamma["bundle_available"].as_bool().unwrap_or(false));
        let bessel = eligibility["families"]
            .as_array()
            .and_then(|families| {
                families
                    .iter()
                    .find(|family| family["feature"] == "special-bessel")
            })
            .expect("bessel family record");
        assert_eq!(bessel["status"], "ready");
        assert!(bessel["bundle_available"].as_bool().unwrap_or(false));
        let scalar_expanded = eligibility["families"]
            .as_array()
            .and_then(|families| {
                families
                    .iter()
                    .find(|family| family["feature"] == "special-scalar-expanded")
            })
            .expect("scalar-expanded family record");
        assert_eq!(scalar_expanded["status"], "ready");
        assert!(
            scalar_expanded["bundle_available"]
                .as_bool()
                .unwrap_or(false)
        );
        let sbom: Value = serde_json::from_slice(
            artifacts
                .get(&root.join("generated/licensing/bundled-sbom.spdx.json"))
                .expect("SBOM artifact"),
        )
        .expect("valid SPDX JSON");
        let files = sbom["files"].as_array().expect("SBOM files");
        for name in [
            BUNDLED_ARCHIVE,
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
