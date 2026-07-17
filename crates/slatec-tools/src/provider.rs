//! Explicit provider-source acquisition and deterministic publication metadata.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const SNAPSHOT: &str = "complete-slatec-05078ebcb649b50e4435";
const PROFILE: &str = "native-profile-7e29d91c176d0c60";

#[derive(Deserialize)]
struct SourceManifest {
    sources: Vec<ProviderSource>,
    families: BTreeMap<String, Vec<String>>,
}

#[derive(Deserialize, Serialize)]
struct ProviderSource {
    id: String,
    subset: String,
    path: String,
    sha256: String,
    url: String,
}

/// Concise acquisition result.
#[derive(Debug)]
pub struct AcquisitionResult {
    /// Sources whose bytes were downloaded and verified.
    pub downloaded: usize,
    /// Sources already present with the expected hash.
    pub verified_existing: usize,
    /// Total sources required by the reviewed family closures.
    pub total: usize,
}

/// Acquires or offline-verifies all reviewed provider sources.
pub fn acquire(manifest_path: &Path, cache: &Path, offline: bool) -> Result<AcquisitionResult> {
    let manifest: SourceManifest = serde_json::from_slice(&fs::read(manifest_path)?)?;
    let mut downloaded = 0;
    let mut verified_existing = 0;
    for source in &manifest.sources {
        let path = cached_path(cache, source);
        if path.is_file() && file_sha256(&path)? == source.sha256 {
            verified_existing += 1;
            continue;
        }
        if offline {
            return Err(CorpusError::Verification(format!(
                "offline provider cache is missing or invalid for {} at {}",
                source.id,
                path.display()
            )));
        }
        let bytes = download(&source.url)?;
        let actual = hex_sha256(&bytes);
        if actual != source.sha256 {
            return Err(CorpusError::Verification(format!(
                "provider source hash mismatch for {}: expected {}, found {actual}",
                source.id, source.sha256
            )));
        }
        fs::create_dir_all(path.parent().expect("provider cache parent"))?;
        fs::write(path, bytes)?;
        downloaded += 1;
    }
    Ok(AcquisitionResult {
        downloaded,
        verified_existing,
        total: manifest.sources.len(),
    })
}

/// Generates compact source-rights and provider-publication records.
pub fn generate_metadata(root: &Path) -> Result<String> {
    let manifest_path = root.join("crates/slatec-src/metadata/family-source-closure.json");
    let manifest: SourceManifest = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    let mut subset_counts = BTreeMap::<String, usize>::new();
    for source in &manifest.sources {
        *subset_counts.entry(source.subset.clone()).or_default() += 1;
    }
    let rights_records = vec![
        json!({
            "id":"rights-main-src", "origin":"main-src", "selected_source_count":subset_counts.get("main-src").copied().unwrap_or(0),
            "upstream_url":"https://www.netlib.org/slatec/slatec_src.tgz", "original_package":"SLATEC main source archive",
            "copyright_notice":"Archive AAAAAA carries a United States Government sponsorship and warranty disclaimer; individual files have varied authorship.",
            "license_or_public_domain_statement":"No explicit archive-wide copyright licence or public-domain dedication was found.",
            "redistribution_terms":"Unresolved. Netlib hosting and the disclaimer do not grant redistribution permission.",
            "modification_requirements":"Unresolved", "attribution_requirements":"Preserve file-level provenance and notices",
            "source_redistribution":"unresolved", "compiled_redistribution":"unresolved", "prebuilt_eligible":false,
            "evidence":["docs/source-corpus/rights-register.md#right-001","https://www.netlib.org/misc/faq.html"]
        }),
        json!({
            "id":"rights-fnlib", "origin":"fnlib", "selected_source_count":subset_counts.get("fnlib").copied().unwrap_or(0),
            "upstream_url":"https://www.netlib.org/slatec/fnlib/", "original_package":"SLATEC-hosted FNLIB relocation",
            "copyright_notice":"Selected files identify varied authors and institutions in historical prologues.",
            "license_or_public_domain_statement":"No common explicit FNLIB redistribution licence was verified.",
            "redistribution_terms":"Requires file-level author or rights-holder review.",
            "modification_requirements":"Unresolved", "attribution_requirements":"Preserve authorship and revision provenance",
            "source_redistribution":"unresolved", "compiled_redistribution":"unresolved", "prebuilt_eligible":false,
            "evidence":["docs/source-corpus/rights-register.md#right-012","https://www.netlib.org/misc/faq.html"]
        }),
        json!({
            "id":"rights-lin", "origin":"lin", "selected_source_count":subset_counts.get("lin").copied().unwrap_or(0),
            "upstream_url":"https://www.netlib.org/slatec/lin/", "original_package":"Mixed SLATEC-hosted BLAS, LINPACK, EISPACK, and support sources",
            "copyright_notice":"Package provenance varies by file and upstream family.",
            "license_or_public_domain_statement":"The reference BLAS statement permits reuse with credit and modification rules, but exact matching and the non-BLAS files remain unresolved.",
            "redistribution_terms":"Do not apply the BLAS permission to the mixed directory as a whole.",
            "modification_requirements":"Reference BLAS asks that modified routines be renamed and changes documented; other files unresolved",
            "attribution_requirements":"Reference BLAS requests proper credit; other files require review",
            "source_redistribution":"partial_unresolved", "compiled_redistribution":"unresolved", "prebuilt_eligible":false,
            "evidence":["docs/source-corpus/rights-register.md#right-007","docs/source-corpus/rights-register.md#right-008","docs/source-corpus/rights-register.md#right-009","https://www.netlib.org/blas/faq.html"]
        }),
        json!({
            "id":"rights-profile-support", "origin":"project-profile-support", "selected_source_count":3,
            "upstream_url":null, "original_package":"slatec-rs GNU MinGW profile compatibility providers",
            "copyright_notice":"Project-authored compatibility code.", "license_or_public_domain_statement":"MIT OR Apache-2.0 project licence",
            "redistribution_terms":"Permitted under either project licence.", "modification_requirements":"Follow selected project licence",
            "attribution_requirements":"Follow selected project licence", "source_redistribution":"permitted", "compiled_redistribution":"permitted",
            "prebuilt_eligible":true, "evidence":["LICENSE-MIT","LICENSE-APACHE"]
        }),
        json!({
            "id":"rights-libgfortran", "origin":"GNU libgfortran", "selected_source_count":0,
            "upstream_url":"https://gcc.gnu.org/", "original_package":"GNU Compiler Collection runtime",
            "copyright_notice":"GNU runtime library notices apply to the exact compiler distribution.",
            "license_or_public_domain_statement":"GPLv3 with the GCC Runtime Library Exception where the file notice applies.",
            "redistribution_terms":"The exception permits eligible compiled target-code combinations under chosen terms; redistribution of the runtime as an independent library remains subject to its licence.",
            "modification_requirements":"Follow the exact runtime notices and eligible-compilation conditions",
            "attribution_requirements":"Retain applicable licence notices", "source_redistribution":"licensed_with_conditions",
            "compiled_redistribution":"licensed_with_conditions", "prebuilt_eligible":false,
            "evidence":["https://www.gnu.org/licenses/gcc-exception.html","https://www.gnu.org/licenses/gcc-exception-faq.html"]
        }),
        json!({
            "id":"rights-libquadmath", "origin":"GNU libquadmath", "selected_source_count":0,
            "upstream_url":"https://gcc.gnu.org/onlinedocs/gfortran/Link-Options.html", "original_package":"GNU Compiler Collection quadmath runtime",
            "copyright_notice":"GNU runtime library notices apply to the exact compiler distribution.",
            "license_or_public_domain_statement":"LGPL; GNU Fortran documentation warns that static linking introduces redistribution requirements.",
            "redistribution_terms":"Static-link distribution requires a dedicated LGPL compliance plan; slatec-rs does not redistribute this archive.",
            "modification_requirements":"Follow the exact LGPL version and notices shipped with the toolchain",
            "attribution_requirements":"Retain applicable licence notices", "source_redistribution":"licensed_with_conditions",
            "compiled_redistribution":"licensed_with_conditions", "prebuilt_eligible":false,
            "evidence":["https://gcc.gnu.org/onlinedocs/gfortran/Link-Options.html"]
        }),
    ];
    let rights = json!({
        "schema_id":"slatec-rs/source-rights", "schema_version":"1.0.0", "snapshot_id":SNAPSHOT,
        "legal_status":"risk documentation only; not legal advice", "records":rights_records,
        "decision":"prebuilt source and binary redistribution blocked until every selected historical origin and GNU runtime delivery plan is cleared"
    });
    let provider_index = json!({
        "schema_id":"slatec-rs/provider-index", "schema_version":"1.0.0", "snapshot_id":SNAPSHOT,
        "family_count":manifest.families.len(),
        "records":[
            {"mode":"prebuilt","status":"blocked","network":false,"fortran_compiler":false,"reason":"selected historical source and compiled redistribution rights unresolved","target":null,"archive_sha256":null},
            {"mode":"source-build","status":"supported","network":false,"fortran_compiler":true,"source_cache":"SLATEC_SOURCE_CACHE, populated explicitly and hash-verified","target":"x86_64-pc-windows-gnu","compiler_profile_id":PROFILE,"validated_compiler":"GNU Fortran 14.2.0 x86_64-w64-mingw32","flags":["-x","f77","-std=legacy","-ffixed-line-length-none","-c"],"runtime":["static libgfortran","static libquadmath","MinGW/UCRT system libraries"],"separate_objects":true,"whole_archive":false},
            {"mode":"system","status":"supported_escape_hatch","network":false,"fortran_compiler":false,"configuration":["SLATEC_SYSTEM_LIB_DIR","optional SLATEC_SYSTEM_LIB_NAME","SLATEC_SYSTEM_RUNTIME_LIB_DIR"]},
            {"mode":"external-backend","status":"supported_escape_hatch","network":false,"fortran_compiler":false,"link_directives":false}
        ],
        "default_backend":null,
        "policy":"A top-level application selects one backend. No backend is silently selected when no redistributable prebuilt provider exists."
    });
    let runtime_audit = json!({
        "schema_id":"slatec-rs/provider-runtime-link-audit", "schema_version":"1.0.0",
        "target":"x86_64-pc-windows-gnu", "compiler_profile_id":PROFILE,
        "compiler":"GNU Fortran (MinGW-W64 x86_64-ucrt-posix-seh) 14.2.0",
        "source_build_runtime_linkage":{"libgfortran":"static","libquadmath":"static","libgcc":"GNU/Rust MinGW link closure"},
        "consumer_test":{"name":"source-build-gamma","toolchain_directories_on_path":false,"status":"passed"},
        "observed_gnu_runtime_dll_imports":[],
        "observed_host_runtime_families":["Windows kernel","UCRT API sets","USERENV","WS2_32"],
        "licensing_note":"Static libquadmath introduces LGPL compliance obligations; libgfortran is governed by its exact GPL/GCC Runtime Library Exception notices. No runtime archive is redistributed by slatec-rs."
    });
    let licensing = root.join("generated/licensing");
    let providers = root.join("generated/providers");
    fs::create_dir_all(&licensing)?;
    fs::create_dir_all(&providers)?;
    write_json(&licensing.join("source-rights.json"), &rights)?;
    write_json(&providers.join("provider-index.json"), &provider_index)?;
    write_json(&providers.join("runtime-link-audit.json"), &runtime_audit)?;
    fs::write(
        licensing.join("provider-rights-summary.md"),
        format!(
            "# Provider rights summary\n\n- Snapshot: `{SNAPSHOT}`.\n- Reviewed provider-source origins: main-src ({}), fnlib ({}), lin ({}).\n- Netlib says most packages have no restrictions but recommends checking authors; it is not a package-specific grant.\n- No common explicit redistribution grant has been verified for all selected historical sources.\n- Prebuilt source and archive publication is therefore **blocked**.\n- Project-authored profile support may be distributed under MIT OR Apache-2.0.\n- GNU libgfortran is governed by its exact GPL/RLE notices; static libquadmath introduces LGPL compliance obligations.\n- This is risk documentation, not legal advice.\n",
            subset_counts.get("main-src").copied().unwrap_or(0),
            subset_counts.get("fnlib").copied().unwrap_or(0),
            subset_counts.get("lin").copied().unwrap_or(0)
        ),
    )?;
    fs::write(
        providers.join("provider-validation-summary.md"),
        format!(
            "# Provider validation summary\n\n- Snapshot: `{SNAPSHOT}`.\n- Validated ABI profile: `{PROFILE}` (GNU Fortran 14.2.0, x86_64-w64-mingw32).\n- Prebuilt: blocked; no archive or provider crate is published.\n- Source-build: explicit, cache-only, checksum-verified, and offline after acquisition.\n- System: explicit deterministic archive directory and name.\n- External backend: emits no native directives.\n- Native archive policy: separate source objects and no whole-archive.\n- Runtime policy: static libgfortran and libquadmath are linked by source-build; redistribution obligations remain with the produced binary and must be reviewed.\n"
        ),
    )?;
    Ok(hash::bytes(&serde_json::to_vec(
        &json!({"rights":rights,"providers":provider_index,"runtime":runtime_audit}),
    )?))
}

fn cached_path(cache: &Path, source: &ProviderSource) -> PathBuf {
    let relative = if source.subset == "main-src" {
        source.path.clone()
    } else {
        format!("{}/{}", source.subset, source.path)
    };
    cache.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR))
}

fn download(url: &str) -> Result<Vec<u8>> {
    let response = ureq::get(url)
        .call()
        .map_err(|error| CorpusError::Verification(format!("download {url}: {error}")))?;
    response
        .into_body()
        .read_to_vec()
        .map_err(|error| CorpusError::Verification(format!("read {url}: {error}")))
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

fn file_sha256(path: &Path) -> Result<String> {
    Ok(hex_sha256(&fs::read(path)?))
}

fn hex_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn offline_cache_verification_never_downloads() {
        let root = tempdir().expect("temp root");
        let cache = root.path().join("cache");
        fs::create_dir_all(cache.join("src")).expect("cache path");
        fs::write(cache.join("src/demo.f"), b"demo\n").expect("fixture source");
        let manifest = json!({
            "sources":[{"id":"demo","subset":"main-src","path":"src/demo.f","sha256":hex_sha256(b"demo\n"),"url":"https://invalid.example/demo.f"}],
            "families":{"demo":["demo"]}
        });
        let path = root.path().join("manifest.json");
        fs::write(&path, serde_json::to_vec(&manifest).expect("manifest")).expect("write");
        let result = acquire(&path, &cache, true).expect("offline verify");
        assert_eq!(result.downloaded, 0);
        assert_eq!(result.verified_existing, 1);
    }

    #[test]
    fn offline_cache_reports_missing_source() {
        let root = tempdir().expect("temp root");
        let manifest = json!({
            "sources":[{"id":"missing","subset":"fnlib","path":"missing.f","sha256":hex_sha256(b"x"),"url":"https://invalid.example/missing.f"}],
            "families":{"demo":["missing"]}
        });
        let path = root.path().join("manifest.json");
        fs::write(&path, serde_json::to_vec(&manifest).expect("manifest")).expect("write");
        let error = acquire(&path, &root.path().join("cache"), true).expect_err("missing");
        assert!(error.to_string().contains("offline provider cache"));
    }

    #[test]
    fn committed_publication_policy_blocks_unlicensed_prebuilt_artifacts() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let providers: Value = serde_json::from_slice(
            &fs::read(root.join("generated/providers/provider-index.json"))
                .expect("provider index"),
        )
        .expect("valid provider index");
        let records = providers["records"].as_array().expect("provider records");
        let prebuilt = records
            .iter()
            .find(|record| record["mode"] == "prebuilt")
            .expect("prebuilt record");
        assert_eq!(prebuilt["status"], "blocked");
        assert_eq!(providers["family_count"], 19);

        for name in ["slatec", "slatec-core", "slatec-sys", "slatec-src"] {
            let crate_root = root.join("crates").join(name);
            assert!(crate_root.join("README.md").is_file(), "{name} README");
            for license in ["LICENSE-MIT", "LICENSE-APACHE"] {
                let text = fs::read_to_string(crate_root.join(license)).expect("license file");
                assert!(text.len() > 500, "{name}/{license} is not a placeholder");
            }
        }
        let build = fs::read_to_string(root.join("crates/slatec-src/build.rs"))
            .expect("provider build script");
        assert!(!build.contains("ureq"));
        assert!(!build.contains("https://"));
        assert!(build.contains("acquire-provider-sources"));
    }
}
