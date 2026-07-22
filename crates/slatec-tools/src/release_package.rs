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
                for dependency in dependencies.keys() {
                    if publishable.contains(dependency) {
                        dependency_edges.insert((dependency.clone(), name.clone()));
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
            .filter(|path| forbidden_package_path(path))
            .cloned()
            .collect::<Vec<_>>();
        let required_files = ["Cargo.toml", "README.md", "LICENSE-APACHE", "LICENSE-MIT"];
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
    let links_owners = package_records
        .iter()
        .filter_map(|record| {
            record["links"]
                .as_str()
                .map(|links| json!({"crate":record["crate"], "links":links}))
        })
        .collect::<Vec<_>>();
    if links_owners.len() != 1
        || links_owners[0]["crate"].as_str() != Some("slatec-src")
        || links_owners[0]["links"].as_str() != Some("slatec")
    {
        return Err(policy(
            "the native `links = \"slatec\"` namespace must be owned only by slatec-src",
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
        "policy":"Packages must contain required Rust metadata and licenses and no caches, downloaded Fortran corpus, native objects, archives, executables, maps, logs, credentials, or local configuration.",
        "packages":package_records,
        "bundled_carrier":validate_bundled_carrier(root)?,
        "result":"pass",
    });
    fs::create_dir_all(output_dir)?;
    write_json(&output_dir.join("workspace-publication-graph.json"), &graph)?;
    write_json(&output_dir.join("package-content-audit.json"), &packages)?;
    let summary = format!(
        "# Workspace package audit\n\n- Publishable crates: {}\n- Package contents audited: {}\n- Publication layers: {}\n- Native `links` owner: `slatec-src` (`slatec`)\n- Result: `pass`\n\nPublication order is layer-based: independent crates in the same layer may be published in either order, followed by each dependent layer.\n",
        publishable.len(),
        package_records.len(),
        layers.len(),
    );
    write_if_changed(
        &output_dir.join("package-content-audit-summary.md"),
        summary.as_bytes(),
    )?;
    let bytes = serde_json::to_vec(&json!({"graph":graph,"packages":packages}))?;
    Ok(PackageAuditResult {
        status: "validated".to_owned(),
        publishable_crates: publishable.len(),
        packages_audited: package_records.len(),
        publication_layers: layers.len(),
        semantic_hash: hash::bytes(&bytes),
    })
}

fn validate_bundled_carrier(root: &Path) -> Result<Value> {
    let crate_root = root.join("crates/slatec-bundled-x86_64-pc-windows-gnu");
    let manifest_path = crate_root.join("metadata/bundle-manifest.json");
    let manifest: Value = serde_json::from_slice(&fs::read(&manifest_path)?)?;
    let status = manifest["status"]
        .as_str()
        .ok_or_else(|| policy("bundled carrier manifest lacks status"))?;
    let archive = manifest["archive"]
        .as_str()
        .ok_or_else(|| policy("bundled carrier manifest lacks archive path"))?;
    let archive_path = crate_root.join(archive);
    let cargo = read_toml(&crate_root.join("Cargo.toml"))?;
    let published = cargo
        .get("package")
        .and_then(|value| value.get("publish"))
        .and_then(toml::Value::as_bool)
        .unwrap_or(true);
    match status {
        "blocked_by_source_provenance" => {
            if archive_path.exists() {
                return Err(policy(
                    "a bundled archive is present even though its generated provenance manifest blocks distribution",
                ));
            }
            if published {
                return Err(policy(
                    "a provenance-blocked bundled carrier must remain publish = false",
                ));
            }
        }
        "ready_for_archive_production" => {
            if !archive_path.is_file() || manifest["archive_sha256"].as_str().is_none() {
                return Err(policy(
                    "a ready bundled carrier must contain its archive and SHA-256 manifest entry",
                ));
            }
        }
        other => return Err(policy(&format!("unknown bundled carrier status {other}"))),
    }
    Ok(json!({
        "crate":"slatec-bundled-x86_64-pc-windows-gnu",
        "target":manifest["target"],
        "status":status,
        "archive":archive,
        "archive_present":archive_path.is_file(),
        "publish":published,
    }))
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

fn forbidden_package_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.starts_with("target/")
        || lower.starts_with("evidence/")
        || lower.starts_with(".worktrees/")
        || lower.starts_with(".cargo/")
        || lower.contains("source-cache")
        || lower.ends_with(".o")
        || lower.ends_with(".obj")
        || lower.ends_with(".a")
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
            assert!(forbidden_package_path(path));
        }
        assert!(!forbidden_package_path("src/lib.rs"));
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
    fn blocked_bundled_carrier_contains_no_archive() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let carrier = validate_bundled_carrier(&root).expect("carrier policy");
        assert_eq!(carrier["status"], "blocked_by_source_provenance");
        assert_eq!(carrier["archive_present"], false);
    }
}
