//! Registry-only downstream simulation using packaged crates and a local directory source.

use crate::error::{CorpusError, Result};
use crate::hash;
use flate2::read::GzDecoder;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;

#[derive(Clone, Debug, serde::Serialize)]
pub struct RegistrySimulationResult {
    pub status: String,
    pub local_packages: usize,
    pub downstream_configurations: usize,
    pub semantic_hash: String,
    pub output_path: PathBuf,
}

pub fn validate(root: &Path, output_path: &Path) -> Result<RegistrySimulationResult> {
    let canonical_root = root.canonicalize()?;
    let root = canonical_root.as_path();
    let simulation = root.join("target/release-readiness-registry");
    let target_root = root.join("target");
    if !simulation.starts_with(&target_root) {
        return Err(policy(
            "registry simulation escaped the workspace target directory",
        ));
    }
    if simulation.exists() {
        fs::remove_dir_all(&simulation)?;
    }
    let vendor = simulation.join("vendor");
    fs::create_dir_all(&vendor)?;
    run(
        root,
        Command::new("cargo")
            .args(["vendor", "--versioned-dirs"])
            .arg(&vendor)
            .env("CARGO_NET_OFFLINE", "true"),
        "cargo vendor",
    )?;
    let source_config = format!(
        "[source.crates-io]\nreplace-with = \"local-release-vendor\"\n\n[source.local-release-vendor]\ndirectory = {:?}\n\n[net]\noffline = true\n",
        vendor.to_string_lossy()
    );
    let cargo_home = simulation.join("cargo-home");
    fs::create_dir_all(&cargo_home)?;
    fs::write(cargo_home.join("config.toml"), &source_config)?;

    let packages = [
        "slatec-sys",
        "slatec-bundled-x86_64-pc-windows-gnu",
        "slatec-src",
        "slatec-core",
        "slatec",
    ];
    for package in packages {
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
                .env("CARGO_HOME", &cargo_home),
            &format!("cargo package {package}"),
        )?;
        let archive_path = root
            .join("target/package")
            .join(format!("{package}-0.1.0.crate"));
        let bytes = fs::read(&archive_path)?;
        let archive_hash = hash::bytes(&bytes);
        Archive::new(GzDecoder::new(bytes.as_slice())).unpack(&vendor)?;
        let unpacked = vendor.join(format!("{package}-0.1.0"));
        write_directory_checksum(&unpacked, &archive_hash)?;
    }

    let cargo_config = simulation.join(".cargo/config.toml");
    fs::create_dir_all(cargo_config.parent().expect("config parent"))?;
    fs::write(&cargo_config, source_config)?;

    let configurations = [
        Consumer {
            name: "raw-no-default",
            dependency: "slatec-sys = { version = \"=0.1.0\", default-features = false }",
            source: "fn main() {}\n",
            target: None,
            run: false,
        },
        Consumer {
            name: "raw-all-declarations",
            dependency: "slatec-sys = { version = \"=0.1.0\", default-features = false, features = [\"all\"] }",
            source: "fn main() { let _ = slatec_sys::special::airy::ai as *const (); }\n",
            target: None,
            run: false,
        },
        Consumer {
            name: "external-provider-narrow",
            dependency: "slatec-src = { version = \"=0.1.0\", default-features = false, features = [\"external-backend\", \"special-gamma\"] }",
            source: "fn main() { slatec_src::ensure_linked(); }\n",
            target: None,
            run: false,
        },
        Consumer {
            name: "safe-no-default",
            dependency: "slatec = { version = \"=0.1.0\", default-features = false }",
            source: "fn main() {}\n",
            target: None,
            run: false,
        },
        Consumer {
            name: "safe-narrow-external",
            dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"external-backend\", \"special-gamma\"] }",
            source: "fn main() { let _ = slatec::special::gamma::gamma(1.0_f64); }\n",
            target: None,
            run: false,
        },
        Consumer {
            name: "safe-bundled-elementary",
            dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"bundled\", \"special-elementary\"] }",
            source: "fn main() { println!(\"{}\", slatec::special::elementary::log1p(0.5_f64).expect(\"finite input\")); }\n",
            target: Some("x86_64-pc-windows-gnu"),
            run: true,
        },
    ];
    let consumer_root = simulation.join("consumers");
    let mut records = Vec::new();
    for consumer in configurations {
        let dir = consumer_root.join(consumer.name);
        fs::create_dir_all(dir.join("src"))?;
        fs::write(
            dir.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{}\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n\n[dependencies]\n{}\n",
                consumer.name, consumer.dependency
            ),
        )?;
        fs::write(dir.join("src/main.rs"), consumer.source)?;
        let mut command = Command::new("cargo");
        command
            .arg(if consumer.run { "run" } else { "check" })
            .arg("--offline");
        if let Some(target) = consumer.target {
            command.args(["--target", target]);
        }
        run(&dir, &mut command, consumer.name)?;
        records.push(json!({
            "configuration":consumer.name,
            "dependency":consumer.dependency,
            "workspace_path_dependencies":0,
            "result":"pass",
        }));
    }
    let report = json!({
        "schema_id":"slatec-rs.registry-only-downstream-simulation",
        "schema_version":"1.0.0",
        "method":"cargo package archives installed into a Cargo directory source together with vendored registry dependencies",
        "local_packages":packages,
        "downstream_configurations":records,
        "workspace_path_dependencies":0,
        "result":"pass",
    });
    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, &bytes)?;
    Ok(RegistrySimulationResult {
        status: "validated".to_owned(),
        local_packages: packages.len(),
        downstream_configurations: records.len(),
        semantic_hash: hash::bytes(&bytes),
        output_path: output_path.to_path_buf(),
    })
}

struct Consumer {
    name: &'static str,
    dependency: &'static str,
    source: &'static str,
    target: Option<&'static str>,
    run: bool,
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
                    .map_err(|_| policy("packaged file escaped its directory"))?
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

fn run(root: &Path, command: &mut Command, label: &str) -> Result<()> {
    let output = command.current_dir(root).output()?;
    if !output.status.success() {
        return Err(policy(&format!(
            "{label} failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(())
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}
