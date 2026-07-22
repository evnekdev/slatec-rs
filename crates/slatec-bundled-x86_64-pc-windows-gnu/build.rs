use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Manifest {
    archive: String,
    archive_sha256: Option<String>,
    reason: String,
    status: String,
    target: String,
}

fn main() {
    println!("cargo:rerun-if-changed=metadata/bundle-manifest.json");
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let manifest: Manifest = serde_json::from_slice(
        &fs::read(root.join("metadata/bundle-manifest.json"))
            .expect("read generated bundled carrier manifest"),
    )
    .expect("parse generated bundled carrier manifest");
    let archive = root.join(&manifest.archive);
    println!("cargo:target={}", manifest.target);
    println!("cargo:archive={}", archive.display());
    println!("cargo:status={}", manifest.status);
    println!("cargo:reason={}", manifest.reason.replace('\n', " "));
    let ready = manifest.status == "ready_for_archive_production"
        && archive.is_file()
        && manifest.archive_sha256.as_deref().is_some();
    if ready {
        let actual = format!(
            "{:x}",
            Sha256::digest(fs::read(&archive).expect("read archive"))
        );
        if Some(actual.as_str()) != manifest.archive_sha256.as_deref() {
            panic!("bundled carrier archive checksum does not match its generated manifest");
        }
    }
    println!("cargo:available={ready}");
}
