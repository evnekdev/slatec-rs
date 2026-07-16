use crate::archive::verify_artifact;
use crate::error::{CorpusError, Result};
use crate::hash;
use crate::policy::Policy;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::ResponseExt;

#[derive(Serialize)]
struct AcquisitionRecord<'a> {
    artifact_id: &'a str,
    requested_url: &'a str,
    final_url: &'a str,
    redirects: Vec<String>,
    retrieval_timestamp: String,
    response_size: u64,
    media_type: Option<String>,
    raw_sha256: String,
    tool_version: String,
    policy_version: u32,
    acquisition_status: String,
}

pub fn acquire(
    artifact_path: &Path,
    evidence_dir: &Path,
    policy: &Policy,
    offline: bool,
) -> Result<()> {
    if artifact_path.exists() {
        verify_artifact(artifact_path, policy)?;
        return write_record(
            evidence_dir,
            policy,
            artifact_path,
            policy.url.clone(),
            Vec::new(),
            None,
            "already_present_verified".to_owned(),
        );
    }
    if offline {
        return Err(CorpusError::Network(format!(
            "offline mode: verified artifact is absent at {}",
            artifact_path.display()
        )));
    }
    let response = ureq::get(&policy.url)
        .config()
        .save_redirect_history(true)
        .build()
        .call()
        .map_err(|error| CorpusError::Network(error.to_string()))?;
    let final_url = response.get_uri().to_string();
    let redirects = response
        .get_redirect_history()
        .map(|history| history.iter().map(ToString::to_string).collect())
        .unwrap_or_default();
    let media_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let mut body = response.into_body();
    let bytes = body
        .read_to_vec()
        .map_err(|error| CorpusError::Network(error.to_string()))?;
    if bytes.len() as u64 != policy.compressed_bytes || hash::bytes(&bytes) != policy.sha256 {
        return Err(CorpusError::Verification("download did not match policy-pinned size and SHA-256; temporary bytes were not promoted".to_owned()));
    }
    let parent = artifact_path
        .parent()
        .ok_or_else(|| CorpusError::Verification("artifact path has no parent".to_owned()))?;
    fs::create_dir_all(parent)?;
    let temporary = artifact_path.with_extension("slatec-corpus-download-partial");
    if temporary.exists() {
        return Err(CorpusError::Verification(format!(
            "download temporary path exists: {}; investigate or remove it explicitly",
            temporary.display()
        )));
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    output.write_all(&bytes)?;
    output.sync_all()?;
    drop(output);
    verify_artifact(&temporary, policy)?;
    fs::rename(&temporary, artifact_path)?;
    write_record(
        evidence_dir,
        policy,
        artifact_path,
        final_url,
        redirects,
        media_type,
        "downloaded_and_verified".to_owned(),
    )
}

fn write_record(
    evidence_dir: &Path,
    policy: &Policy,
    artifact_path: &Path,
    final_url: String,
    redirects: Vec<String>,
    media_type: Option<String>,
    acquisition_status: String,
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| CorpusError::Network(error.to_string()))?
        .as_secs();
    let record = AcquisitionRecord {
        artifact_id: &policy.source_id,
        requested_url: &policy.url,
        final_url: &final_url,
        redirects,
        retrieval_timestamp: format!("unix-seconds:{timestamp}"),
        response_size: fs::metadata(artifact_path)?.len(),
        media_type,
        raw_sha256: hash::file(artifact_path)?,
        tool_version: format!("{TOOL_NAME} {TOOL_VERSION}"),
        policy_version: policy.version,
        acquisition_status,
    };
    let dir = evidence_dir.join("acquisition");
    fs::create_dir_all(&dir)?;
    let temporary = dir.join("slatec-source-archive.json.partial");
    let destination = dir.join("slatec-source-archive.json");
    let mut bytes = serde_json::to_vec_pretty(&record)?;
    bytes.push(b'\n');
    fs::write(&temporary, bytes)?;
    fs::rename(&temporary, &destination)?;
    Ok(())
}
