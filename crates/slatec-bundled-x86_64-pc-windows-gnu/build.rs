use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct Manifest {
    archives: Vec<Archive>,
    runtime_archives: Vec<RuntimeArchive>,
    status: String,
    target: String,
}

#[derive(Deserialize)]
struct Archive {
    family: String,
    path: String,
    sha256: String,
}

#[derive(Deserialize)]
struct RuntimeArchive {
    component: String,
    link_name: String,
    path: String,
    sha256: String,
}

fn main() {
    println!("cargo:rerun-if-changed=metadata/bundle-manifest.json");
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let manifest: Manifest = serde_json::from_slice(
        &fs::read(root.join("metadata/bundle-manifest.json"))
            .expect("read generated bundled carrier manifest"),
    )
    .expect("parse generated bundled carrier manifest");
    println!("cargo:target={}", manifest.target);
    println!("cargo:status={}", manifest.status);

    for archive in &manifest.archives {
        let path = verified_archive(&root, &archive.path, &archive.sha256);
        let key = cargo_key(&archive.family);
        println!("cargo:archive-{key}={}", path.display());
        println!("cargo:available-{key}=true");
    }
    for runtime in &manifest.runtime_archives {
        let path = verified_archive(&root, &runtime.path, &runtime.sha256);
        let key = cargo_key(&runtime.component);
        println!("cargo:runtime-{key}={}", path.display());
        println!("cargo:runtime-link-name-{key}={}", runtime.link_name);
        println!("cargo:runtime-available-{key}=true");
    }
}

fn verified_archive(root: &Path, relative: &str, expected: &str) -> PathBuf {
    let path = root.join(relative);
    let actual = format!(
        "{:x}",
        Sha256::digest(fs::read(&path).expect("read verified bundled carrier archive"))
    );
    assert_eq!(
        actual, expected,
        "bundled carrier archive checksum does not match its generated manifest"
    );
    path
}

fn cargo_key(value: &str) -> String {
    value.replace('-', "_")
}
