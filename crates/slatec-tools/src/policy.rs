use crate::error::{CorpusError, Result};
use crate::hash;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct PolicyFile {
    schema_version: u32,
    policy_version: u32,
    #[serde(rename = "baseline")]
    baseline: Baseline,
    #[serde(rename = "selected_source")]
    selected: SelectedSource,
}

#[derive(Debug, Deserialize)]
struct Baseline {
    source_id: String,
    artifact_name: String,
    url: String,
    sha256: String,
    compressed_bytes: u64,
    regular_members: u64,
    fortran_members: u64,
}

#[derive(Debug, Deserialize)]
struct SelectedSource {
    profile: String,
    include_globs: Vec<String>,
    expected_selected_files: u64,
}

#[derive(Clone, Debug)]
pub struct Policy {
    pub schema_version: u32,
    pub version: u32,
    pub source_id: String,
    pub artifact_name: String,
    pub url: String,
    pub sha256: String,
    pub compressed_bytes: u64,
    pub regular_members: u64,
    pub selected_files: u64,
    pub profile: String,
    pub semantic_hash: String,
    pub path: PathBuf,
}

impl Policy {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read(path)?;
        let parsed: PolicyFile =
            toml::from_str(std::str::from_utf8(&raw).map_err(|_| {
                CorpusError::Policy("canonical-corpus.toml is not UTF-8".to_owned())
            })?)?;
        if parsed.schema_version != 1 || parsed.policy_version != 1 {
            return Err(CorpusError::Policy(
                "only canonical corpus policy/schema version 1 is supported".to_owned(),
            ));
        }
        if parsed.baseline.source_id != "slatec-source-archive"
            || parsed.baseline.artifact_name != "slatec_src.tgz"
            || parsed.baseline.sha256
                != "4c8c02fee905325ee4906bf8f7ece5593d895da3e5f208322f8aacea6d0eb9dc"
            || parsed.baseline.compressed_bytes != 1_768_291
            || parsed.baseline.regular_members != 741
            || parsed.baseline.fortran_members != 735
            || parsed.selected.include_globs != ["src/*.f"]
            || parsed.selected.expected_selected_files != 735
            || parsed.selected.profile != "default"
        {
            return Err(CorpusError::Policy(
                "metadata disagrees with the policy-v1 canonical baseline".to_owned(),
            ));
        }
        Ok(Self {
            schema_version: parsed.schema_version,
            version: parsed.policy_version,
            source_id: parsed.baseline.source_id,
            artifact_name: parsed.baseline.artifact_name,
            url: parsed.baseline.url,
            sha256: parsed.baseline.sha256,
            compressed_bytes: parsed.baseline.compressed_bytes,
            regular_members: parsed.baseline.regular_members,
            selected_files: parsed.selected.expected_selected_files,
            profile: parsed.selected.profile,
            semantic_hash: hash::bytes(&raw),
            path: path.to_owned(),
        })
    }
}
