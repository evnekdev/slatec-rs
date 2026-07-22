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
        if name == "slatec-bundled-x86_64-pc-windows-gnu" {
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
    let release_version = workspace_package
        .get("version")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| policy("workspace.package.version metadata is missing"))?;
    let package_dry_runs = publishable
        .iter()
        .map(|name| package_dry_run(root, name, release_version))
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
        "bundled_carrier":validate_bundled_carrier(root)?,
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
    let api_baseline = public_api_baseline(root)?;
    let target_support = json!({
        "schema_id":"slatec-rs.target-support","schema_version":"1.0.0",
        "targets":[
            {"target":"x86_64-pc-windows-gnu","status":"supported-bundled","evidence":["hash-verified accepted-source carrier","reduced GNU runtime closure receipt","clean packaged-consumer execution"]},
            {"target":"x86_64-unknown-linux-gnu","status":"not-shipped","blockers":["no reviewed GNU Fortran compiler/runtime profile in this release receipt","no target-specific carrier archive or packaged-consumer execution","no source-cache, runtime-closure, import, or provenance receipt for a Linux carrier"]},
            {"target":"x86_64-pc-windows-msvc","status":"not-shipped","blockers":["no reviewed Fortran provider/linker ABI strategy"]},
            {"target":"aarch64-apple-darwin","status":"not-shipped","blockers":["no reviewed Fortran provider/linker ABI strategy"]}
        ],
        "policy":"Only x86_64-pc-windows-gnu is a supported compiler-free target. Other targets must select an explicit expert provider and are not validated by this carrier report."
    });
    write_json(
        &output_dir.join("public-api-freeze-baseline.json"),
        &api_baseline,
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
    let summary = format!(
        "# Workspace package audit\n\n- Publishable crates: {}\n- Package contents audited: {}\n- Publication layers: {}\n- Native implementation-provider `links` owner: `slatec-src` (`slatec`)\n- Target carrier: `slatec-bundled-x86_64-pc-windows-gnu` (distinct metadata-only `links` namespace)\n- Result: `pass`\n\nPublication order is layer-based: independent crates in the same layer may be published in either order, followed by each dependent layer.\n",
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
        "api_baseline":api_baseline,
        "target_support":target_support,
        "scalar_disposition":scalar_disposition,
        "scalar_accuracy":scalar_accuracy,
        "docs_visibility":docs_visibility,
    }))?;
    Ok(PackageAuditResult {
        status: "validated".to_owned(),
        publishable_crates: publishable.len(),
        packages_audited: package_records.len(),
        publication_layers: layers.len(),
        semantic_hash: hash::bytes(&bytes),
    })
}

fn public_api_baseline(root: &Path) -> Result<Value> {
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
        "schema_id":"slatec-rs.public-api-freeze-baseline","schema_version":"1.0.0",
        "scope":"canonical reviewed slatec-sys raw paths; generated implementation modules and provider internals are deliberately excluded",
        "input":"generated/public-api/canonical-public-api.json",
        "input_sha256":hash::file(&source)?,
        "export_count":exports.len(),
        "exports":exports,
        "policy":"Canonical reviewed paths and routine names are frozen for 0.1.x. Feature gates are additive where possible; an ABI correction may supersede compatibility when evidence proves the existing declaration unsafe."
    }))
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
            let release_disposition = match source_classification {
                "already_safely_exposed" | "suitable_for_this_milestone" => {
                    "public-enhanced-primitive"
                }
                "internal_or_subsidiary_only" => "private-provider-subsidiary",
                "deferred_due_to_global_mutable_state" => "raw-only-with-reason",
                "deferred_due_to_branch_or_precision_ambiguity" => "raw-only-with-reason",
                "deferred_pending_contract_review" | "insufficiently_documented" => {
                    "raw-only-with-reason"
                }
                _ => "raw-only-with-reason",
            };
            *disposition_counts
                .entry(release_disposition.to_owned())
                .or_default() += 1;
            Ok(json!({
                "native_name":native_name,
                "source_classification":source_classification,
                "release_disposition":release_disposition,
                "safe_path":fields.get(6).and_then(Value::as_str).filter(|path| !path.is_empty()),
                "reason":fields.get(5).and_then(Value::as_str),
            }))
        })
        .collect::<Result<Vec<_>>>()?;
    let disposition = json!({
        "schema_id":"slatec-rs.scalar-api-disposition","schema_version":"1.0.0",
        "input":"generated/safe-api/special-existing-vs-candidate.json",
        "input_sha256":hash::file(&source)?,
        "scope":"retained scalar special-function candidates; ordinary Rust core and std operations are not wrapped merely for name parity",
        "disposition_counts":disposition_counts,
        "records":records,
        "policy":"A public scalar path must supply a reviewed numerical or domain-safety value. Subsidiaries, mutable-global-state routines, and incomplete contracts remain non-public with an explicit reason."
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

fn package_dry_run(root: &Path, name: &str, version: &str) -> Result<Value> {
    let output = Command::new("cargo")
        .args(["package", "-p", name, "--allow-dirty", "--offline"])
        .current_dir(root)
        .output()?;
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
        let artifact = root
            .join("target/package")
            .join(format!("{name}-{version}.crate"));
        return Ok(json!({
            "crate":name,
            "status":"passed_offline_prepublication",
            "package_size_bytes":fs::metadata(artifact).ok().map(|metadata| metadata.len()),
            "detail":"Cargo packaged and verified this independent publication-layer crate from its package contents."
        }));
    }
    let prerequisite = missing_crates_io_package(&stderr);
    if prerequisite.is_some() {
        return Ok(json!({
            "crate":name,
            "status":"blocked_until_publication_prerequisite_exists",
            "package_size_bytes":Value::Null,
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

fn validate_bundled_carrier(root: &Path) -> Result<Value> {
    let crate_root = root.join("crates/slatec-bundled-x86_64-pc-windows-gnu");
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
        "crate":"slatec-bundled-x86_64-pc-windows-gnu",
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
    let permitted_carrier_archive = package == "slatec-bundled-x86_64-pc-windows-gnu"
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
        let carrier = validate_bundled_carrier(&root).expect("carrier policy");
        assert_eq!(carrier["status"], "ready");
        assert!(carrier["archive_count"].as_u64().unwrap_or_default() >= 1);
    }
}
