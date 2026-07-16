use crate::archive::{ArchiveInventory, normalized_member_path};
use crate::error::{CorpusError, Result};
use crate::hash;
use crate::policy::Policy;
use flate2::read::GzDecoder;
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn raw_root(evidence_dir: &Path, snapshot_id: &str, policy: &Policy) -> PathBuf {
    evidence_dir
        .join("extracted")
        .join(snapshot_id)
        .join(&policy.source_id)
}

pub fn extract_raw(
    artifact_path: &Path,
    evidence_dir: &Path,
    snapshot_id: &str,
    policy: &Policy,
    inventory: &ArchiveInventory,
) -> Result<PathBuf> {
    let final_root = raw_root(evidence_dir, snapshot_id, policy);
    if final_root.exists() {
        verify_existing_tree(&final_root, inventory)?;
        return Ok(final_root);
    }
    let staging_root = evidence_dir
        .join("work")
        .join(format!("extract-{snapshot_id}"))
        .join(&policy.source_id);
    if staging_root.exists() {
        return Err(CorpusError::Verification(format!(
            "incomplete extraction staging directory exists: {}; remove it explicitly after investigation",
            staging_root.display()
        )));
    }
    fs::create_dir_all(&staging_root)?;
    let expected: BTreeMap<_, _> = inventory
        .members
        .iter()
        .filter(|member| member.entry_type == "regular")
        .map(|member| (member.normalized_path.as_str(), member))
        .collect();
    let file = File::open(artifact_path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    for entry in archive
        .entries()
        .map_err(|error| CorpusError::Archive(error.to_string()))?
    {
        let mut entry = entry.map_err(|error| CorpusError::Archive(error.to_string()))?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        let raw = entry
            .path()
            .map_err(|error| CorpusError::UnsafeArchive(error.to_string()))?;
        let raw = raw
            .to_str()
            .ok_or_else(|| CorpusError::UnsafeArchive("member path is not UTF-8".to_owned()))?;
        let path = normalized_member_path(&raw.replace('\\', "/"))?;
        let manifest_member = expected.get(path.as_str()).ok_or_else(|| {
            CorpusError::Archive(format!(
                "archive changed after inspection; unknown member {path}"
            ))
        })?;
        let destination = staging_root.join(&path);
        ensure_under_root(&staging_root, &destination)?;
        let parent = destination
            .parent()
            .ok_or_else(|| CorpusError::Archive("member has no parent".to_owned()))?;
        fs::create_dir_all(parent)?;
        let temporary = destination.with_extension("slatec-corpus-partial");
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        output.write_all(&bytes)?;
        output.sync_all()?;
        drop(output);
        let written_hash = hash::file(&temporary)?;
        if manifest_member.raw_content_sha256.as_deref() != Some(written_hash.as_str()) {
            return Err(CorpusError::Verification(format!(
                "extracted member hash mismatch: {path}"
            )));
        }
        fs::rename(&temporary, &destination)?;
    }
    verify_existing_tree(&staging_root, inventory)?;
    let final_parent = final_root
        .parent()
        .ok_or_else(|| CorpusError::Archive("invalid final extraction root".to_owned()))?;
    fs::create_dir_all(final_parent)?;
    fs::rename(&staging_root, &final_root)?;
    Ok(final_root)
}

fn verify_existing_tree(root: &Path, inventory: &ArchiveInventory) -> Result<()> {
    for member in inventory
        .members
        .iter()
        .filter(|member| member.entry_type == "regular")
    {
        let path = root.join(&member.normalized_path);
        if !path.is_file() {
            return Err(CorpusError::Verification(format!(
                "missing extracted member {}",
                member.normalized_path
            )));
        }
        let actual = hash::file(&path)?;
        if member.raw_content_sha256.as_deref() != Some(actual.as_str()) {
            return Err(CorpusError::Verification(format!(
                "extracted member hash mismatch {}",
                member.normalized_path
            )));
        }
    }
    Ok(())
}

fn ensure_under_root(root: &Path, destination: &Path) -> Result<()> {
    if !destination.starts_with(root) {
        return Err(CorpusError::UnsafeArchive(format!(
            "member escapes evidence root: {}",
            destination.display()
        )));
    }
    Ok(())
}
