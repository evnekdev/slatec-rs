use crate::error::{CorpusError, Result};
use crate::hash;
use crate::policy::Policy;
use flate2::read::GzDecoder;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug, Serialize)]
pub struct ArchiveMember {
    pub id: String,
    pub raw_archive_path: String,
    pub normalized_path: String,
    pub entry_type: String,
    pub uncompressed_size: u64,
    pub selected: bool,
    pub exclusion_reason: Option<String>,
    pub raw_content_sha256: Option<String>,
    pub source_artifact_id: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArchiveInventory {
    pub members: Vec<ArchiveMember>,
    pub regular_member_count: u64,
    pub selected_fortran_member_count: u64,
    pub semantic_hash: String,
}

pub fn verify_artifact(path: &Path, policy: &Policy) -> Result<()> {
    let metadata = std::fs::metadata(path).map_err(|error| {
        CorpusError::Verification(format!("cannot read {}: {error}", path.display()))
    })?;
    if metadata.len() != policy.compressed_bytes {
        return Err(CorpusError::Verification(format!(
            "size mismatch for {}: expected {}, got {}",
            path.display(),
            policy.compressed_bytes,
            metadata.len()
        )));
    }
    let actual = hash::file(path)?;
    if actual != policy.sha256 {
        return Err(CorpusError::Verification(format!(
            "SHA-256 mismatch for {}: expected {}, got {actual}",
            path.display(),
            policy.sha256
        )));
    }
    Ok(())
}

pub fn inspect_archive(path: &Path, policy: &Policy) -> Result<ArchiveInventory> {
    verify_artifact(path, policy)?;
    let file = File::open(path)?;
    let decoder = GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    let mut members = Vec::new();
    let mut raw_paths = BTreeSet::new();
    let mut normalized_paths = BTreeSet::new();
    let mut casefolded_paths = BTreeSet::new();

    for entry in archive
        .entries()
        .map_err(|error| CorpusError::Archive(error.to_string()))?
    {
        let mut entry = entry.map_err(|error| CorpusError::Archive(error.to_string()))?;
        let raw_path = entry.path().map_err(|error| {
            CorpusError::UnsafeArchive(format!("unreadable member path: {error}"))
        })?;
        let raw_path = raw_path
            .to_str()
            .ok_or_else(|| CorpusError::UnsafeArchive("member path is not valid UTF-8".to_owned()))?
            .replace('\\', "/");
        let normalized_path = normalized_member_path(&raw_path)?;
        if !raw_paths.insert(raw_path.clone()) {
            return Err(CorpusError::UnsafeArchive(format!(
                "duplicate raw member path {raw_path}"
            )));
        }
        if !normalized_paths.insert(normalized_path.clone()) {
            return Err(CorpusError::UnsafeArchive(format!(
                "ambiguous normalized member path {normalized_path}"
            )));
        }
        if !casefolded_paths.insert(normalized_path.to_ascii_lowercase()) {
            return Err(CorpusError::UnsafeArchive(format!(
                "case-folding collision for {normalized_path}"
            )));
        }

        let entry_type = entry.header().entry_type();
        let type_name = if entry_type.is_file() {
            "regular"
        } else if entry_type.is_dir() {
            "directory"
        } else if entry_type.is_symlink() {
            return Err(CorpusError::UnsafeArchive(format!(
                "symbolic link member {raw_path}"
            )));
        } else if entry_type.is_hard_link() {
            return Err(CorpusError::UnsafeArchive(format!(
                "hard link member {raw_path}"
            )));
        } else {
            return Err(CorpusError::UnsafeArchive(format!(
                "unsupported archive entry type for {raw_path}"
            )));
        };

        let raw_content_sha256 = if type_name == "regular" {
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes)?;
            Some(hash::bytes(&bytes))
        } else {
            None
        };
        let (selected, exclusion_reason) = selection(&normalized_path, type_name);
        let member_id = format!(
            "archive-member-{}",
            &hash::bytes(format!("{}\u{1f}{normalized_path}", policy.source_id).as_bytes())[..16]
        );
        members.push(ArchiveMember {
            id: member_id,
            raw_archive_path: raw_path,
            normalized_path,
            entry_type: type_name.to_owned(),
            uncompressed_size: entry.size(),
            selected,
            exclusion_reason,
            raw_content_sha256,
            source_artifact_id: policy.source_id.clone(),
        });
    }
    members.sort_by(|left, right| left.raw_archive_path.cmp(&right.raw_archive_path));
    let regular_member_count = members
        .iter()
        .filter(|member| member.entry_type == "regular")
        .count() as u64;
    let selected_fortran_member_count =
        members.iter().filter(|member| member.selected).count() as u64;
    if regular_member_count != policy.regular_members {
        return Err(CorpusError::Verification(format!(
            "regular member count mismatch: expected {}, got {regular_member_count}",
            policy.regular_members
        )));
    }
    if selected_fortran_member_count != policy.selected_files {
        return Err(CorpusError::Verification(format!(
            "selected src/*.f count mismatch: expected {}, got {selected_fortran_member_count}",
            policy.selected_files
        )));
    }
    for required in ["src/sgeir.f", "src/qk15w.f", "src/dqk15w.f"] {
        if !members
            .iter()
            .any(|member| member.normalized_path == required && member.selected)
        {
            return Err(CorpusError::Verification(format!(
                "required selected member missing: {required}"
            )));
        }
    }
    if !members.iter().any(|member| {
        member.normalized_path == "src/sgeir.f.0"
            && member.entry_type == "regular"
            && !member.selected
    }) {
        return Err(CorpusError::Verification(
            "src/sgeir.f.0 must be preserved and excluded".to_owned(),
        ));
    }
    let semantic = serde_json::to_vec(&members)?;
    Ok(ArchiveInventory {
        members,
        regular_member_count,
        selected_fortran_member_count,
        semantic_hash: hash::bytes(&semantic),
    })
}

pub fn normalized_member_path(raw: &str) -> Result<String> {
    if raw.is_empty() {
        return Err(CorpusError::UnsafeArchive(
            "empty archive member path".to_owned(),
        ));
    }
    let candidate = PathBuf::from(raw);
    if candidate.is_absolute() {
        return Err(CorpusError::UnsafeArchive(format!(
            "absolute member path {raw}"
        )));
    }
    let mut parts = Vec::new();
    for component in candidate.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::CurDir => continue,
            Component::ParentDir => {
                return Err(CorpusError::UnsafeArchive(format!(
                    "path traversal member {raw}"
                )));
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(CorpusError::UnsafeArchive(format!(
                    "absolute member path {raw}"
                )));
            }
        }
    }
    if parts.is_empty() {
        return Err(CorpusError::UnsafeArchive(format!(
            "ambiguous member path {raw}"
        )));
    }
    Ok(parts.join("/"))
}

fn selection(path: &str, entry_type: &str) -> (bool, Option<String>) {
    if entry_type != "regular" {
        return (false, Some("directory-record".to_owned()));
    }
    if path.starts_with("src/") && path.ends_with(".f") {
        return (true, None);
    }
    let reason = match path {
        "src/sgeir.f.0" => "preserved-historical-provider-excluded-by-EXC-001",
        "src/changes" | "src/MD5" | "src/.depend" | "src/index" | "src/index.html" => {
            "preserved-support-record-excluded-by-EXC-002"
        }
        _ => "not-selected-by-policy-v1",
    };
    (false, Some(reason.to_owned()))
}
