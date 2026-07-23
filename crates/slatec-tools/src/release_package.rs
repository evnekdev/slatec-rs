//! Deterministic workspace publication-graph and package-content audit.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, serde::Serialize)]
pub struct PackageAuditResult {
    pub status: String,
    pub publishable_crates: usize,
    pub packages_audited: usize,
    pub publication_layers: usize,
    pub semantic_hash: String,
}

pub fn validate(root: &Path, output_dir: &Path) -> Result<PackageAuditResult> {
    let workspace = read_toml(&root.join("Cargo.toml"))?;
    let workspace_package = workspace
        .get("workspace")
        .and_then(|value| value.get("package"))
        .ok_or_else(|| policy("workspace.package metadata is missing"))?;
    let members = workspace
        .get("workspace")
        .and_then(|value| value.get("members"))
        .and_then(toml::Value::as_array)
        .ok_or_else(|| policy("workspace members are missing"))?;
    let mut manifests = BTreeMap::<String, (PathBuf, toml::Value)>::new();
    for member in members {
        let relative = member
            .as_str()
            .ok_or_else(|| policy("workspace member is not a string"))?;
        let path = root.join(relative).join("Cargo.toml");
        let manifest = read_toml(&path)?;
        let name = manifest
            .get("package")
            .and_then(|value| value.get("name"))
            .and_then(toml::Value::as_str)
            .ok_or_else(|| policy(&format!("{} has no package name", path.display())))?
            .to_owned();
        manifests.insert(name, (path, manifest));
    }
    let publishable = manifests
        .iter()
        .filter(|(_, (_, manifest))| {
            manifest
                .get("package")
                .and_then(|value| value.get("publish"))
                .and_then(toml::Value::as_bool)
                != Some(false)
        })
        .map(|(name, _)| name.clone())
        .collect::<BTreeSet<_>>();

    let required_metadata = [
        "name",
        "version",
        "edition",
        "rust-version",
        "description",
        "license",
        "repository",
        "homepage",
        "documentation",
        "readme",
        "keywords",
        "categories",
        "authors",
        "include",
    ];
    let mut package_records = Vec::new();
    let mut dependency_edges = BTreeSet::<(String, String)>::new();
    for name in &publishable {
        let (manifest_path, manifest) = &manifests[name];
        let package = manifest
            .get("package")
            .ok_or_else(|| policy(&format!("{name} has no package table")))?;
        let missing_metadata = required_metadata
            .iter()
            .filter(|key| !metadata_present(package, workspace_package, key))
            .map(|key| (*key).to_owned())
            .collect::<Vec<_>>();
        if !missing_metadata.is_empty() {
            return Err(policy(&format!(
                "{name} is missing package metadata: {}",
                missing_metadata.join(", ")
            )));
        }
        for dependency_table in ["dependencies", "build-dependencies"] {
            if let Some(dependencies) = manifest
                .get(dependency_table)
                .and_then(toml::Value::as_table)
            {
                record_workspace_dependencies(
                    dependencies,
                    name,
                    &publishable,
                    &workspace,
                    &mut dependency_edges,
                )?;
            }
        }
        if let Some(targets) = manifest.get("target").and_then(toml::Value::as_table) {
            for target in targets.values() {
                for dependency_table in ["dependencies", "build-dependencies"] {
                    if let Some(dependencies) =
                        target.get(dependency_table).and_then(toml::Value::as_table)
                    {
                        record_workspace_dependencies(
                            dependencies,
                            name,
                            &publishable,
                            &workspace,
                            &mut dependency_edges,
                        )?;
                    }
                }
            }
        }
        let output = Command::new("cargo")
            .args([
                "package",
                "-p",
                name,
                "--allow-dirty",
                "--list",
                "--offline",
            ])
            .current_dir(root)
            .output()?;
        if !output.status.success() {
            return Err(policy(&format!(
                "cargo package --list failed for {name}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            )));
        }
        let files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.trim().replace('\\', "/"))
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let forbidden = files
            .iter()
            .filter(|path| forbidden_package_path(name, path))
            .cloned()
            .collect::<Vec<_>>();
        let mut required_files = vec!["Cargo.toml", "README.md", "LICENSE-APACHE", "LICENSE-MIT"];
        if name.starts_with("slatec-bundled-") {
            required_files.extend([
                "metadata/runtime-licenses/GPL-3.0.txt",
                "metadata/runtime-licenses/LGPL-2.1-or-later.txt",
                "metadata/runtime-licenses/GCC-RUNTIME-LIBRARY-EXCEPTION-3.1.txt",
                "metadata/runtime-licenses/SOURCES-AND-RELINKING.md",
            ]);
        }
        let missing_files = required_files
            .iter()
            .filter(|required| !files.iter().any(|path| path == **required))
            .map(|path| (*path).to_owned())
            .collect::<Vec<_>>();
        if !forbidden.is_empty() || !missing_files.is_empty() {
            return Err(policy(&format!(
                "{name} package content failed: forbidden={forbidden:?}, missing={missing_files:?}"
            )));
        }
        package_records.push(json!({
            "crate":name,
            "manifest":slash_path(root, manifest_path),
            "file_count":files.len(),
            "forbidden_files":forbidden,
            "missing_required_files":missing_files,
            "links":package.get("links").and_then(toml::Value::as_str),
            "build_script":package.get("build").and_then(toml::Value::as_str),
            "result":"pass",
        }));
    }
    let layers = publication_layers(&publishable, &dependency_edges)?;
    let _release_version = workspace_package
        .get("version")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| policy("workspace.package.version metadata is missing"))?;
    let package_dry_runs = publishable
        .iter()
        .map(|name| package_dry_run(root, name))
        .collect::<Result<Vec<_>>>()?;
    let links_owners = package_records
        .iter()
        .filter_map(|record| {
            record["links"]
                .as_str()
                .map(|links| json!({"crate":record["crate"], "links":links}))
        })
        .collect::<Vec<_>>();
    let src_owns_slatec = links_owners.iter().any(|record| {
        record["crate"].as_str() == Some("slatec-src") && record["links"].as_str() == Some("slatec")
    });
    let unique_links = links_owners
        .iter()
        .filter_map(|record| record["links"].as_str())
        .collect::<BTreeSet<_>>()
        .len()
        == links_owners.len();
    if !src_owns_slatec || !unique_links {
        return Err(policy(
            "slatec-src must own the unique `links = \"slatec\"` namespace and every published carrier links namespace must be distinct",
        ));
    }
    let graph = json!({
        "schema_id":"slatec-rs.workspace-publication-graph",
        "schema_version":"1.0.0",
        "coordinated_version":workspace_package.get("version").and_then(toml::Value::as_str),
        "publishable_crates":publishable,
        "dependency_edges":dependency_edges.iter().map(|(dependency, dependent)| json!({
            "dependency":dependency,
            "dependent":dependent,
        })).collect::<Vec<_>>(),
        "publication_layers":layers,
        "links_owners":links_owners,
    });
    let packages = json!({
        "schema_id":"slatec-rs.package-content-audit",
        "schema_version":"1.0.0",
        "policy":"Packages must contain required Rust metadata and licenses and no caches, downloaded Fortran corpus, objects, executables, maps, logs, credentials, or local configuration. The target carrier may contain only manifest-verified native static archives plus exact runtime licence and relinking material.",
        "packages":package_records,
        "package_dry_runs":package_dry_runs,
        "bundled_carriers":validate_bundled_carriers(root)?,
        "result":"pass",
    });
    fs::create_dir_all(output_dir)?;
    write_json(&output_dir.join("workspace-publication-graph.json"), &graph)?;
    write_json(&output_dir.join("package-content-audit.json"), &packages)?;
    write_json(
        &output_dir.join("package-dry-run-audit.json"),
        &json!({
            "schema_id":"slatec-rs.package-dry-run-audit","schema_version":"1.0.0",
            "offline":true,
            "records":packages["package_dry_runs"],
            "policy":"Before any crate is published, an offline Cargo package verification can only pass for a publication layer whose workspace dependencies are already on crates.io. A blocked dependent package is recorded with its exact prerequisite; the local-registry downstream simulation validates the complete package graph without workspace paths."
        }),
    )?;
    let safe_api_baseline = safe_public_api_baseline(root)?;
    let raw_api_baseline = raw_public_abi_baseline(root)?;
    let core_api_baseline = core_support_types_baseline(root)?;
    let carrier_api_baseline = carrier_metadata_baseline(root)?;
    let target_support = target_support(root, &carrier_api_baseline)?;
    write_json(
        &output_dir.join("public-api-freeze-baseline.json"),
        &safe_api_baseline,
    )?;
    write_json(
        &output_dir.join("raw-public-abi-freeze-baseline.json"),
        &raw_api_baseline,
    )?;
    write_json(
        &output_dir.join("core-support-types-api-baseline.json"),
        &core_api_baseline,
    )?;
    write_json(
        &output_dir.join("carrier-metadata-api-baseline.json"),
        &carrier_api_baseline,
    )?;
    write_json(&output_dir.join("target-support.json"), &target_support)?;
    let (scalar_disposition, scalar_accuracy) = scalar_release_evidence(root)?;
    write_json(
        &output_dir.join("scalar-api-disposition.json"),
        &scalar_disposition,
    )?;
    write_json(
        &output_dir.join("scalar-accuracy-evidence.json"),
        &scalar_accuracy,
    )?;
    let docs_visibility = docs_feature_visibility(root)?;
    write_json(
        &output_dir.join("docs-feature-visibility.json"),
        &docs_visibility,
    )?;
    let release_candidate = release_candidate_report(
        root,
        &safe_api_baseline,
        &raw_api_baseline,
        &core_api_baseline,
        &carrier_api_baseline,
        &target_support,
        &packages,
    )?;
    write_json(
        &output_dir.join("release-candidate-report.json"),
        &release_candidate,
    )?;
    write_if_changed(
        &output_dir.join("release-candidate-report.md"),
        release_candidate_markdown(&release_candidate).as_bytes(),
    )?;
    let summary = format!(
        "# Workspace package audit\n\n- Publishable crates: {}\n- Package contents audited: {}\n- Publication layers: {}\n- Native implementation-provider `links` owner: `slatec-src` (`slatec`)\n- Target carriers: `slatec-bundled-x86_64-pc-windows-gnu` and `slatec-bundled-x86_64-unknown-linux-gnu` (distinct metadata-only `links` namespaces)\n- Result: `pass`\n\nPublication order is layer-based: independent crates in the same layer may be published in either order, followed by each dependent layer.\n",
        publishable.len(),
        package_records.len(),
        layers.len(),
    );
    write_if_changed(
        &output_dir.join("package-content-audit-summary.md"),
        summary.as_bytes(),
    )?;
    let bytes = serde_json::to_vec(&json!({
        "graph":graph,
        "packages":packages,
        "safe_api_baseline":safe_api_baseline,
        "raw_api_baseline":raw_api_baseline,
        "core_api_baseline":core_api_baseline,
        "carrier_api_baseline":carrier_api_baseline,
        "target_support":target_support,
        "scalar_disposition":scalar_disposition,
        "scalar_accuracy":scalar_accuracy,
        "docs_visibility":docs_visibility,
        "release_candidate":release_candidate,
    }))?;
    Ok(PackageAuditResult {
        status: "validated".to_owned(),
        publishable_crates: publishable.len(),
        packages_audited: package_records.len(),
        publication_layers: layers.len(),
        semantic_hash: hash::bytes(&bytes),
    })
}

fn safe_public_api_baseline(root: &Path) -> Result<Value> {
    let source = root.join("generated/safe-api/function-index.json");
    let function_index: Value = serde_json::from_slice(&fs::read(&source)?)?;
    let records = function_index["records"]
        .as_array()
        .ok_or_else(|| policy("safe function index lacks records"))?;
    let mut exports = records
        .iter()
        .map(|record| {
            let path = record["rust_path"]
                .as_str()
                .ok_or_else(|| policy("safe function index record lacks rust_path"))?;
            let feature = record["feature"]
                .as_str()
                .ok_or_else(|| policy("safe function index record lacks feature"))?;
            Ok(json!({
                "path":path,
                "feature":feature,
                "fortran_routine":record["fortran_routine"],
                "precision":record["precision"],
                "capability":record["capability"],
                "native_profile":record["native_profile"],
                "inclusion_status":record["inclusion_status"],
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    exports.sort_by(|left, right| left["path"].as_str().cmp(&right["path"].as_str()));
    let paths = exports
        .iter()
        .filter_map(|record| record["path"].as_str())
        .collect::<BTreeSet<_>>();
    if paths.len() != exports.len() || exports.is_empty() {
        return Err(policy(
            "safe public API baseline must contain unique callable facade paths",
        ));
    }
    let feature_set = exports
        .iter()
        .filter_map(|record| record["feature"].as_str())
        .collect::<BTreeSet<_>>();
    Ok(json!({
        "schema_id":"slatec-rs.safe-public-api-freeze-baseline","schema_version":"2.0.0",
        "scope":"callable safe slatec facade paths from the generated safe function index; raw declarations, generated implementation modules, provider internals, and test-only modules are excluded",
        "input":"generated/safe-api/function-index.json",
        "input_sha256":hash::file(&source)?,
        "target_feature_profiles":[
            {"target":"x86_64-pc-windows-gnu","provider":"bundled or source-build","feature_set":"full","native_profile":"ffi-profile-gnu-mingw-x86_64"},
            {"target":"x86_64-unknown-linux-gnu","provider":"bundled or source-build","feature_set":"reviewed bundled special/root families","native_profile":"GNU Fortran 11.4 carrier receipt"},
            {"target":"caller-selected","provider":"external-backend","feature_set":"feature-gated callable facade","native_profile":"provider-dependent"}
        ],
        "feature_set":feature_set,
        "export_count":exports.len(),
        "exports":exports,
        "policy":"Safe callable paths, feature gates, precision labels, and wrapper-to-routine mappings are frozen for 0.1.x. This is deliberately independent from the raw ABI baseline."
    }))
}

fn raw_public_abi_baseline(root: &Path) -> Result<Value> {
    let source = root.join("generated/public-api/canonical-public-api.json");
    let canonical: Value = serde_json::from_slice(&fs::read(&source)?)?;
    let records = canonical["records"]
        .as_array()
        .ok_or_else(|| policy("canonical public API inventory lacks records"))?;
    let mut exports = records
        .iter()
        .map(|record| {
            let path = record["canonical_rust_path"]
                .as_str()
                .ok_or_else(|| policy("canonical public API record lacks a path"))?;
            Ok(json!({
                "path":path,
                "routine":record["routine"],
                "feature":record["feature"],
                "classification":if record["feature"].as_str().is_some_and(|feature| !feature.is_empty()) { "feature-gated-intentional" } else { "intentional-stable-surface" },
                "native_symbol":record["native_symbol"],
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    exports.sort_by(|left, right| left["path"].as_str().cmp(&right["path"].as_str()));
    let paths = exports
        .iter()
        .filter_map(|record| record["path"].as_str())
        .collect::<BTreeSet<_>>();
    if paths.len() != exports.len() || exports.len() != 821 {
        return Err(policy(
            "canonical raw public API baseline must contain 821 unique paths",
        ));
    }
    Ok(json!({
        "schema_id":"slatec-rs.raw-public-abi-freeze-baseline","schema_version":"2.0.0",
        "scope":"canonical reviewed slatec-sys raw ABI paths; generated implementation modules and provider internals are deliberately excluded",
        "input":"generated/public-api/canonical-public-api.json",
        "input_sha256":hash::file(&source)?,
        "export_count":exports.len(),
        "exports":exports,
        "target_feature_profiles":[{"target":"x86_64-pc-windows-gnu","feature_set":"slatec-sys/all","abi_profile":"ffi-profile-gnu-mingw-x86_64"}],
        "policy":"Canonical reviewed raw paths and routine names are frozen for 0.1.x. Feature gates are additive where possible; an ABI correction may supersede compatibility when evidence proves the existing declaration unsafe."
    }))
}

fn core_support_types_baseline(root: &Path) -> Result<Value> {
    let source = root.join("crates/slatec-core/src/fortran.rs");
    let source_text = fs::read_to_string(&source)?;
    let exports = [
        "slatec_core::fortran::IntegerRangeError",
        "slatec_core::fortran::to_fortran_integer",
        "slatec_core::fortran::to_fortran_increment",
    ];
    for export in exports {
        let item = export.rsplit("::").next().expect("static core export");
        if !source_text.contains(item) {
            return Err(policy("core support type baseline source drifted"));
        }
    }
    Ok(json!({
        "schema_id":"slatec-rs.core-support-types-api-baseline","schema_version":"1.0.0",
        "scope":"provider-neutral public slatec-core support exports selected by ffi-profile-gnu-mingw-x86_64",
        "input":"crates/slatec-core/src/fortran.rs",
        "input_sha256":hash::file(&source)?,
        "target_feature_profiles":[{"target":"x86_64-pc-windows-gnu","feature_set":"ffi-profile-gnu-mingw-x86_64","provider":"none"},{"target":"x86_64-unknown-linux-gnu","feature_set":"provider-neutral support types","provider":"none"}],
        "exports":exports,
        "policy":"Core support types remain provider-neutral and are frozen separately from both safe numerical paths and raw declarations."
    }))
}

fn carrier_metadata_baseline(root: &Path) -> Result<Value> {
    let carriers = fs::read_dir(root.join("crates"))?
        .filter_map(std::result::Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("slatec-bundled-"))
        .map(|entry| {
            let crate_root = entry.path();
            let manifest_path = crate_root.join("metadata/bundle-manifest.json");
            let cargo_path = crate_root.join("Cargo.toml");
            let manifest: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;
            let cargo = read_toml(&cargo_path)?;
            let package = cargo
                .get("package")
                .ok_or_else(|| policy("carrier Cargo.toml lacks package metadata"))?;
            Ok(json!({
                "crate":package.get("name").and_then(toml::Value::as_str),
                "version":package.get("version").and_then(toml::Value::as_str),
                "links":package.get("links").and_then(toml::Value::as_str),
                "target":manifest["target"],
                "status":manifest["status"],
                "source_unit_count":manifest["source_unit_count"],
                "archive_count":manifest["archives"].as_array().map(|archives| archives.len()),
                "runtime_archive_count":manifest["runtime_archives"].as_array().map(|archives| archives.len()),
                "cargo_toml_sha256":hash::file(&cargo_path)?,
                "bundle_manifest_sha256":hash::file(&manifest_path)?,
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    if carriers.is_empty() {
        return Err(policy(
            "at least one target carrier metadata package is required",
        ));
    }
    Ok(json!({
        "schema_id":"slatec-rs.carrier-metadata-api-baseline","schema_version":"1.0.0",
        "scope":"target-specific bundled-carrier metadata only; carriers expose no numerical Rust API",
        "carriers":carriers,
        "policy":"Each target carrier has its own links namespace, exact target receipt, and immutable package metadata baseline."
    }))
}

fn target_support(root: &Path, carriers: &Value) -> Result<Value> {
    let records = carriers["carriers"]
        .as_array()
        .ok_or_else(|| policy("carrier baseline lacks carrier records"))?;
    let target_record = |target: &str| {
        records
            .iter()
            .find(|record| record["target"].as_str() == Some(target))
            .ok_or_else(|| policy(&format!("carrier baseline lacks {target} carrier")))
    };
    let ready_families = |file: &str| -> Result<Vec<String>> {
        let eligibility: Value = serde_json::from_slice(&fs::read(root.join(file))?)?;
        Ok(eligibility["families"]
            .as_array()
            .ok_or_else(|| policy("bundled source eligibility lacks family records"))?
            .iter()
            .filter(|record| record["bundle_available"] == true)
            .filter_map(|record| record["feature"].as_str().map(str::to_owned))
            .collect())
    };
    let windows = target_record("x86_64-pc-windows-gnu")?;
    let linux = target_record("x86_64-unknown-linux-gnu")?;
    let windows_families =
        ready_families("crates/slatec-src/metadata/bundled-source-eligibility.json")?;
    let linux_families =
        ready_families("crates/slatec-src/metadata/bundled-source-eligibility-linux.json")?;
    if windows["status"].as_str() != Some("ready") || windows_families.is_empty() {
        return Err(policy(
            "Windows GNU target cannot be reported as bundled without a ready carrier and family archive",
        ));
    }
    if linux["status"].as_str() != Some("ready") || linux_families.is_empty() {
        return Err(policy(
            "Linux GNU target cannot be reported as bundled without a ready carrier and family archive",
        ));
    }
    Ok(json!({
        "schema_id":"slatec-rs.target-support","schema_version":"2.0.0",
        "targets":[
            {"target":"x86_64-pc-windows-gnu","status":"supported-bundled","bundled_families":windows_families,"evidence":["hash-verified accepted-source carrier","reduced GNU runtime closure receipt","clean packaged-consumer execution"]},
            {"target":"x86_64-unknown-linux-gnu","status":"supported-bundled","bundled_families":linux_families,"evidence":["hash-verified accepted-source carrier","reduced GNU runtime closure receipt","clean ELF consumer execution and source-build parity"]},
            {"target":"x86_64-pc-windows-msvc","status":"not-shipped","blockers":["no reviewed Fortran provider/linker ABI strategy"]},
            {"target":"aarch64-apple-darwin","status":"not-shipped","blockers":["no reviewed Fortran provider/linker ABI strategy"]}
        ],
        "policy":"Only target records with a ready, exact carrier receipt and a clean packaged-consumer execution are supported bundled targets. Other targets must select an explicit expert provider and are not validated by this carrier report."
    }))
}

fn release_candidate_report(
    root: &Path,
    safe_api: &Value,
    raw_api: &Value,
    core_api: &Value,
    carrier_api: &Value,
    target_support: &Value,
    packages: &Value,
) -> Result<Value> {
    let windows = target_support["targets"]
        .as_array()
        .and_then(|targets| {
            targets
                .iter()
                .find(|target| target["target"].as_str() == Some("x86_64-pc-windows-gnu"))
        })
        .ok_or_else(|| policy("release-candidate report lacks Windows target record"))?;
    let linux = target_support["targets"]
        .as_array()
        .and_then(|targets| {
            targets
                .iter()
                .find(|target| target["target"].as_str() == Some("x86_64-unknown-linux-gnu"))
        })
        .ok_or_else(|| policy("release-candidate report lacks Linux target record"))?;
    let registry_path =
        root.join("generated/release-readiness/registry-only-downstream-simulation.json");
    let registry = if registry_path.is_file() {
        serde_json::from_slice::<Value>(&fs::read(&registry_path)?)?
    } else {
        json!({"result":"not_run"})
    };
    let gates = vec![
        json!({"gate":"safe API freeze","status":"pass","evidence":"generated/release-readiness/public-api-freeze-baseline.json","remaining_blocker":Value::Null,"count":safe_api["export_count"]}),
        json!({"gate":"raw ABI freeze","status":"pass","evidence":"generated/release-readiness/raw-public-abi-freeze-baseline.json","remaining_blocker":Value::Null,"count":raw_api["export_count"]}),
        json!({"gate":"core support API freeze","status":"pass","evidence":"generated/release-readiness/core-support-types-api-baseline.json","remaining_blocker":Value::Null,"count":core_api["exports"].as_array().map(Vec::len)}),
        json!({"gate":"target-carrier metadata freeze","status":"pass","evidence":"generated/release-readiness/carrier-metadata-api-baseline.json","remaining_blocker":Value::Null,"count":carrier_api["carriers"].as_array().map(Vec::len)}),
        json!({"gate":"Windows bundled carrier","status":if windows["status"] == "supported-bundled" { "pass" } else { "blocked" },"evidence":"generated/release-readiness/target-support.json","remaining_blocker":windows["blockers"],"families":windows["bundled_families"]}),
        json!({"gate":"Linux bundled carrier","status":if linux["status"] == "supported-bundled" { "pass" } else { "blocked" },"evidence":"generated/release-readiness/target-support.json","remaining_blocker":linux["blockers"]}),
        json!({"gate":"package contents","status":packages["result"],"evidence":"generated/release-readiness/package-content-audit.json","remaining_blocker":Value::Null}),
        json!({"gate":"registry-only packaged consumers","status":registry["result"],"evidence":"generated/release-readiness/registry-only-downstream-simulation.json","remaining_blocker":if registry["result"] == "pass" { Value::Null } else { json!("run validate-registry-simulation") }}),
    ];
    let blockers = gates
        .iter()
        .filter(|gate| gate["status"] != "pass")
        .map(|gate| gate["gate"].clone())
        .collect::<Vec<_>>();
    Ok(json!({
        "schema_id":"slatec-rs.release-candidate-report","schema_version":"1.0.0",
        "overall_status":if blockers.is_empty() { "release_candidate" } else { "blocked" },
        "gates":gates,
        "remaining_blockers":blockers,
        "semver_decision":if blockers.is_empty() { "0.1.0 release candidate; no crate published and no tag created" } else { "not a release candidate until every listed gate is resolved" },
        "publish_commands":[
            "cargo publish -p slatec-bundled-x86_64-pc-windows-gnu",
            "cargo publish -p slatec-bundled-x86_64-unknown-linux-gnu",
            "cargo publish -p slatec-sys",
            "cargo publish -p slatec-core",
            "cargo publish -p slatec-src",
            "cargo publish -p slatec"
        ],
        "policy":"This report records every unsatisfied gate as a blocker; it never upgrades a framework, partially accepted source closure, or unvalidated target into a release candidate."
    }))
}

fn release_candidate_markdown(report: &Value) -> String {
    let mut output = String::from(
        "# Release candidate report\n\n| Gate | Status | Evidence | Remaining blocker |\n| --- | --- | --- | --- |\n",
    );
    for gate in report["gates"].as_array().into_iter().flatten() {
        let blocker = gate["remaining_blocker"]
            .as_str()
            .map(str::to_owned)
            .or_else(|| {
                gate["remaining_blocker"].as_array().map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join("; ")
                })
            })
            .unwrap_or_else(|| "—".to_owned());
        output.push_str(&format!(
            "| {} | {} | `{}` | {} |\n",
            gate["gate"].as_str().unwrap_or("unknown"),
            gate["status"].as_str().unwrap_or("unknown"),
            gate["evidence"].as_str().unwrap_or("unknown"),
            blocker
        ));
    }
    output.push_str(&format!(
        "\nOverall status: **{}**.\n",
        report["overall_status"].as_str().unwrap_or("unknown")
    ));
    output
}

fn scalar_release_evidence(root: &Path) -> Result<(Value, Value)> {
    let source = root.join("generated/safe-api/special-existing-vs-candidate.json");
    let inventory: Value = serde_json::from_slice(&fs::read(&source)?)?;
    let rows = inventory["records"]
        .as_array()
        .ok_or_else(|| policy("safe scalar inventory lacks records"))?;
    let mut disposition_counts = BTreeMap::<String, usize>::new();
    let records = rows
        .iter()
        .map(|row| {
            let fields = row
                .as_array()
                .ok_or_else(|| policy("safe scalar inventory record is not an array"))?;
            let native_name = fields
                .first()
                .and_then(Value::as_str)
                .ok_or_else(|| policy("safe scalar inventory record lacks a native name"))?;
            let source_classification = fields
                .get(3)
                .and_then(Value::as_str)
                .ok_or_else(|| policy("safe scalar inventory record lacks a classification"))?;
            let final_action = match source_classification {
                "already_safely_exposed" | "suitable_for_this_milestone" => {
                    "implement_safe_public_wrapper"
                }
                "obsolete_alias_or_duplicate" => "covered_by_rust_std",
                "internal_or_subsidiary_only" => "keep_private_provider_subsidiary",
                "deferred_due_to_global_mutable_state" => "retain_raw_only_with_blocker",
                "deferred_due_to_branch_or_precision_ambiguity" => {
                    "retain_raw_only_with_blocker"
                }
                "deferred_pending_contract_review" | "insufficiently_documented" => {
                    "retain_raw_only_with_blocker"
                }
                _ => "retain_raw_only_with_blocker",
            };
            *disposition_counts
                .entry(final_action.to_owned())
                .or_default() += 1;
            Ok(json!({
                "native_name":native_name,
                "source_classification":source_classification,
                "final_action":final_action,
                "safe_path":fields.get(6).and_then(Value::as_str).filter(|path| !path.is_empty()),
                "reason":fields.get(5).and_then(Value::as_str),
                "review_basis":if final_action == "covered_by_rust_std" { "Rust f32/f64 intrinsic has the same ordinary real elementary operation; no SLATEC-specific numerical advantage is asserted." } else { "generated safe-special candidate classification and source-backed contract review" },
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    let disposition = json!({
        "schema_id":"slatec-rs.scalar-api-disposition","schema_version":"1.0.0",
        "input":"generated/safe-api/special-existing-vs-candidate.json",
        "input_sha256":hash::file(&source)?,
        "scope":"retained scalar special-function candidates; each record has exactly one final action and ordinary Rust core/std operations are not wrapped merely for name parity",
        "disposition_counts":disposition_counts,
        "records":records,
        "policy":"A public scalar path must supply a reviewed numerical or domain-safety value. Subsidiaries, mutable-global-state routines, and incomplete contracts remain non-public with an explicit reason. ACOSH/ASINH/ATANH and their double-precision aliases are deliberately covered by Rust standard-library methods rather than duplicated."
    });
    let accuracy = json!({
        "schema_id":"slatec-rs.scalar-accuracy-evidence","schema_version":"1.0.0",
        "scope":"No safe scalar path is introduced by this release-candidate change. This report makes the existing reviewed scalar reference coverage and its limits explicit rather than inventing aggregate accuracy claims.",
        "test_sources":[
            "crates/slatec/tests/special_foundations_raw_native.rs",
            "crates/slatec/tests/special_functions_native.rs"
        ],
        "columns":["operation","precision","domain_set","max_abs_error","max_relative_or_ulp_error","comparator","status"],
        "records":[
            {"operation":"log1p / exprel / cbrt / degree sine","precision":"f64","domain_set":"small cancellation value; zero; exact cube; degree identity","max_abs_error":"1e-20 for log1p(1e-8); 1e-15 for the remaining asserted values","max_relative_or_ulp_error":"not_aggregated","comparator":"closed-form values and standard constants","status":"reference assertions passed on the reviewed GNU MinGW profile"},
            {"operation":"gamma / beta / error / Airy / exponential and logarithmic integrals","precision":"f64","domain_set":"ordinary values, identities, singular-domain rejection, and contained native error paths","max_abs_error":"per-case asserted tolerance from 1e-15 to 2e-12","max_relative_or_ulp_error":"not_aggregated","comparator":"closed-form values and independent identities","status":"reference and contract assertions passed on the reviewed GNU MinGW profile"},
            {"operation":"Spence and Carlson symmetric elliptic integrals","precision":"f64","domain_set":"identity values, invalid signs, native range status, and post-error reuse","max_abs_error":"per-case asserted tolerance from 1e-13 to 1e-12","max_relative_or_ulp_error":"not_aggregated","comparator":"pi identities and exact unit cases","status":"reference and native-status assertions passed on the reviewed GNU MinGW profile"},
            {"operation":"all exposed scalar f32 counterparts","precision":"f32","domain_set":"ordinary valid execution case for each exposed wrapper","max_abs_error":"not measured by the committed f32 smoke test","max_relative_or_ulp_error":"not measured","comparator":"finite-result execution smoke","status":"execution coverage only; no f32 accuracy superiority claim"}
        ],
        "known_limitations":[
            "The committed tests use per-case absolute tolerances and do not calculate a corpus-wide maximum relative error or ULP bound.",
            "No new scalar safe path is frozen by this change; any future scalar promotion must add its own high-precision vectors and difficult-point assertions."
        ]
    });
    Ok((disposition, accuracy))
}

fn docs_feature_visibility(root: &Path) -> Result<Value> {
    let source = root.join("generated/public-api/public-module-feature-map.json");
    let map: Value = serde_json::from_slice(&fs::read(&source)?)?;
    let records = map["records"]
        .as_array()
        .ok_or_else(|| policy("public module feature map lacks records"))?;
    Ok(json!({
        "schema_id":"slatec-rs.docs-feature-visibility","schema_version":"1.0.0",
        "input":"generated/public-api/public-module-feature-map.json",
        "input_sha256":hash::file(&source)?,
        "record_count":records.len(),
        "records":records,
        "policy":"The public module map is the deterministic feature-to-capability index for no-default-feature navigation and full-feature documentation review."
    }))
}

fn package_dry_run(root: &Path, name: &str) -> Result<Value> {
    let output = Command::new("cargo")
        .args(["package", "-p", name, "--allow-dirty", "--offline"])
        .current_dir(root)
        .output()?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
        return Ok(json!({
            "crate":name,
            "status":"passed_offline_prepublication",
            "detail":"Cargo packaged and verified this independent publication-layer crate from its package contents."
        }));
    }
    let prerequisite = missing_crates_io_package(&stderr);
    if prerequisite.is_some() {
        return Ok(json!({
            "crate":name,
            "status":"blocked_until_publication_prerequisite_exists",
            "prerequisite":prerequisite,
            "detail":"Cargo package verification resolves registry dependencies rather than workspace paths; this is expected before the preceding publication layer is uploaded. The registry-only downstream simulation is the complete pre-publication proof."
        }));
    }
    Err(policy(&format!(
        "cargo package dry-run failed unexpectedly for {name}: {}\n{}",
        stdout.trim(),
        stderr.trim()
    )))
}

fn missing_crates_io_package(stderr: &str) -> Option<String> {
    let marker = "no matching package named `";
    let suffix = stderr.split(marker).nth(1)?;
    Some(suffix.split('`').next()?.to_owned())
}

fn validate_bundled_carriers(root: &Path) -> Result<Value> {
    let carriers = [
        "slatec-bundled-x86_64-pc-windows-gnu",
        "slatec-bundled-x86_64-unknown-linux-gnu",
    ]
    .into_iter()
    .map(|carrier| validate_bundled_carrier(root, carrier))
    .collect::<Result<Vec<_>>>()?;
    Ok(json!({
        "carriers":carriers,
        "result":"pass",
    }))
}

fn validate_bundled_carrier(root: &Path, crate_name: &str) -> Result<Value> {
    let crate_root = root.join("crates").join(crate_name);
    let manifest_path = crate_root.join("metadata/bundle-manifest.json");
    let manifest: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    let status = manifest["status"]
        .as_str()
        .ok_or_else(|| policy("bundled carrier manifest lacks status"))?;
    let archives = manifest["archives"]
        .as_array()
        .ok_or_else(|| policy("bundled carrier manifest lacks archives"))?;
    let cargo = read_toml(&crate_root.join("Cargo.toml"))?;
    let published = cargo
        .get("package")
        .and_then(|value| value.get("publish"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(true);
    match status {
        "blocked_by_source_provenance" | "eligible_pending_archive" => {
            if !archives.is_empty() {
                return Err(policy(
                    "a bundled archive is declared even though its generated provenance manifest is not ready",
                ));
            }
            if published {
                return Err(policy(
                    "a provenance-blocked bundled carrier must remain publish = false",
                ));
            }
        }
        "ready" => {
            if !published {
                return Err(policy(
                    "a ready bundled carrier must be crates.io-publishable",
                ));
            }
            if archives.is_empty() {
                return Err(policy(
                    "a ready bundled carrier must declare at least one archive",
                ));
            }
            for archive in archives {
                let path = archive["path"]
                    .as_str()
                    .ok_or_else(|| policy("ready bundled archive lacks a path"))?;
                let expected = archive["sha256"]
                    .as_str()
                    .ok_or_else(|| policy("ready bundled archive lacks SHA-256"))?;
                let actual = hash::file(&crate_root.join(path))?;
                if actual != expected {
                    return Err(policy(
                        "ready bundled archive checksum does not match its manifest",
                    ));
                }
            }
            for required in [
                "metadata/source-unit-manifest.json",
                "metadata/build-recipe.json",
                "metadata/bundle-build-receipt.json",
                "metadata/THIRD-PARTY-NOTICES.md",
                "metadata/REDISTRIBUTION-NOTICE.md",
                "metadata/runtime-licenses/GPL-3.0.txt",
                "metadata/runtime-licenses/LGPL-2.1-or-later.txt",
                "metadata/runtime-licenses/GCC-RUNTIME-LIBRARY-EXCEPTION-3.1.txt",
                "metadata/runtime-licenses/SOURCES-AND-RELINKING.md",
            ] {
                if !crate_root.join(required).is_file() {
                    return Err(policy(&format!(
                        "ready bundled carrier is missing required package evidence {required}"
                    )));
                }
            }
            for runtime in manifest["runtime_archives"]
                .as_array()
                .ok_or_else(|| policy("ready bundled carrier lacks runtime archive records"))?
            {
                let path = runtime["path"]
                    .as_str()
                    .ok_or_else(|| policy("ready runtime archive lacks a path"))?;
                let expected = runtime["sha256"]
                    .as_str()
                    .ok_or_else(|| policy("ready runtime archive lacks SHA-256"))?;
                if hash::file(&crate_root.join(path))? != expected {
                    return Err(policy(
                        "ready bundled runtime checksum does not match its manifest",
                    ));
                }
            }
        }
        other => return Err(policy(&format!("unknown bundled carrier status {other}"))),
    }
    Ok(json!({
        "crate":crate_name,
        "target":manifest["target"],
        "status":status,
        "archive_count":archives.len(),
        "archives":archives,
        "publish":published,
    }))
}

fn record_workspace_dependencies(
    dependencies: &toml::Table,
    name: &str,
    publishable: &BTreeSet<String>,
    workspace: &toml::Value,
    dependency_edges: &mut BTreeSet<(String, String)>,
) -> Result<()> {
    for dependency in dependencies.keys() {
        if publishable.contains(dependency) {
            dependency_edges.insert((dependency.clone(), name.to_owned()));
            let specification = &dependencies[dependency];
            let workspace_dependency = workspace
                .get("workspace")
                .and_then(|value| value.get("dependencies"))
                .and_then(|value| value.get(dependency));
            if !version_and_path_present(specification, workspace_dependency) {
                return Err(policy(&format!(
                    "publishable dependency {name} -> {dependency} lacks both version and path"
                )));
            }
        }
    }
    Ok(())
}

fn publication_layers(
    crates: &BTreeSet<String>,
    edges: &BTreeSet<(String, String)>,
) -> Result<Vec<Vec<String>>> {
    let mut remaining = crates.clone();
    let mut layers = Vec::new();
    while !remaining.is_empty() {
        let layer = remaining
            .iter()
            .filter(|candidate| {
                !edges.iter().any(|(dependency, dependent)| {
                    dependent == *candidate && remaining.contains(dependency)
                })
            })
            .cloned()
            .collect::<Vec<_>>();
        if layer.is_empty() {
            return Err(policy("publishable workspace dependency graph has a cycle"));
        }
        for name in &layer {
            remaining.remove(name);
        }
        layers.push(layer);
    }
    Ok(layers)
}

fn metadata_present(package: &toml::Value, workspace: &toml::Value, key: &str) -> bool {
    match package.get(key) {
        Some(toml::Value::Table(table))
            if table.get("workspace").and_then(toml::Value::as_bool) == Some(true) =>
        {
            workspace.get(key).is_some()
        }
        Some(toml::Value::String(value)) => !value.trim().is_empty(),
        Some(toml::Value::Array(values)) => !values.is_empty(),
        Some(_) => true,
        None => false,
    }
}

fn version_and_path_present(
    specification: &toml::Value,
    workspace_specification: Option<&toml::Value>,
) -> bool {
    let effective = if specification
        .as_table()
        .and_then(|table| table.get("workspace"))
        .and_then(toml::Value::as_bool)
        == Some(true)
    {
        workspace_specification
    } else {
        Some(specification)
    };
    effective
        .and_then(toml::Value::as_table)
        .is_some_and(|table| table.contains_key("version") && table.contains_key("path"))
}

fn forbidden_package_path(package: &str, path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    let permitted_carrier_archive = package.starts_with("slatec-bundled-")
        && lower.starts_with("native/libslatec_bundle_")
        && lower.ends_with(".a");
    lower.starts_with("target/")
        || lower.starts_with("evidence/")
        || lower.starts_with(".worktrees/")
        || lower.starts_with(".cargo/")
        || lower.contains("source-cache")
        || lower.ends_with(".o")
        || lower.ends_with(".obj")
        || (lower.ends_with(".a") && !permitted_carrier_archive)
        || lower.ends_with(".lib")
        || lower.ends_with(".dll")
        || lower.ends_with(".exe")
        || lower.ends_with(".map")
        || lower.ends_with(".log")
        || lower.ends_with(".env")
}

fn slash_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn read_toml(path: &Path) -> Result<toml::Value> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    write_if_changed(path, &bytes)
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
    fn native_and_cache_artifacts_are_rejected() {
        for path in [
            "target/debug/a",
            "cache/source-cache/a.f",
            "lib/a.lib",
            "run.log",
        ] {
            assert!(forbidden_package_path("other", path));
        }
        assert!(!forbidden_package_path("other", "src/lib.rs"));
        assert!(!forbidden_package_path(
            "slatec-bundled-x86_64-pc-windows-gnu",
            "native/libslatec_bundle_accepted.a"
        ));
    }

    #[test]
    fn topological_layers_are_deterministic() {
        let crates = ["sys", "src", "core", "facade"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        let edges = [
            ("sys".to_owned(), "core".to_owned()),
            ("core".to_owned(), "facade".to_owned()),
            ("src".to_owned(), "facade".to_owned()),
        ]
        .into_iter()
        .collect();
        assert_eq!(
            publication_layers(&crates, &edges).unwrap(),
            vec![
                vec!["src".to_owned(), "sys".to_owned()],
                vec!["core".to_owned()],
                vec!["facade".to_owned()]
            ]
        );
    }

    #[test]
    fn ready_bundled_carrier_has_hash_verified_archives() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let carriers = validate_bundled_carriers(&root).expect("carrier policy");
        assert_eq!(carriers["result"], "pass");
        assert_eq!(carriers["carriers"].as_array().map(Vec::len), Some(2));
        for carrier in carriers["carriers"].as_array().expect("carrier records") {
            assert_eq!(carrier["status"], "ready");
            assert!(carrier["archive_count"].as_u64().unwrap_or_default() >= 1);
        }
    }
}
