//! Audits committed and changed textual files for deterministic LF-only bytes.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::json;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, serde::Serialize)]
pub struct EolAuditResult {
    pub status: String,
    pub tracked_text_files: usize,
    pub changed_text_files: usize,
    pub violations: usize,
    pub semantic_hash: String,
    pub output_path: PathBuf,
}

pub fn validate(root: &Path, output_path: &Path) -> Result<EolAuditResult> {
    let eol = git(root, &["ls-files", "--eol", "-z"])?;
    let mut tracked_text_files = 0usize;
    let mut violations = Vec::new();
    for raw in eol.split(|byte| *byte == 0).filter(|raw| !raw.is_empty()) {
        let line = String::from_utf8_lossy(raw);
        let Some((metadata, path)) = line.split_once('\t') else {
            continue;
        };
        let index = metadata.split_whitespace().next().unwrap_or("i/none");
        if index != "i/-text" && index != "i/none" {
            tracked_text_files += 1;
            if index != "i/lf" {
                violations.push(json!({
                    "path":path.replace('\\', "/"),
                    "surface":"git-index",
                    "observed":index,
                    "required":"i/lf",
                }));
            }
            let working_tree = metadata.split_whitespace().nth(1).unwrap_or("w/none");
            if working_tree != "w/lf" {
                violations.push(json!({
                    "path":path.replace('\\', "/"),
                    "surface":"git-working-tree",
                    "observed":working_tree,
                    "required":"w/lf",
                    "remediation":"git add --renormalize -- .; then restore unchanged paths with core.autocrlf=false if this worktree predates .gitattributes",
                }));
            }
        }
    }

    let mut changed = BTreeSet::new();
    for arguments in [
        ["diff", "--name-only", "-z"].as_slice(),
        ["diff", "--cached", "--name-only", "-z"].as_slice(),
        ["ls-files", "--others", "--exclude-standard", "-z"].as_slice(),
    ] {
        for raw in git(root, arguments)?
            .split(|byte| *byte == 0)
            .filter(|raw| !raw.is_empty())
        {
            changed.insert(String::from_utf8_lossy(raw).replace('\\', "/"));
        }
    }
    let mut changed_text_files = 0usize;
    for relative in changed {
        let path = root.join(&relative);
        let Ok(bytes) = fs::read(path) else {
            continue;
        };
        if bytes.contains(&0) {
            continue;
        }
        changed_text_files += 1;
        let crlf = bytes.windows(2).filter(|pair| *pair == b"\r\n").count();
        let carriage_returns = bytes.iter().filter(|byte| **byte == b'\r').count();
        if carriage_returns != 0 {
            violations.push(json!({
                "path":relative,
                "surface":"changed-working-file",
                "crlf_sequences":crlf,
                "lone_carriage_returns":carriage_returns - crlf,
            }));
        }
    }
    let report = json!({
        "schema_id":"slatec-rs.eol-audit",
        "schema_version":"1.1.0",
        "policy":"Every text blob in the Git index and every changed or untracked textual working file uses LF-only bytes. .gitattributes enforces eol=lf and final staging uses git add --renormalize.",
        "tracked_text_files":tracked_text_files,
        "changed_working_files_checked":true,
        "violations":violations,
        "violation_count":violations.len(),
        "result":if violations.is_empty() { "pass" } else { "fail" },
    });
    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if fs::read(output_path).ok().as_deref() != Some(bytes.as_slice()) {
        fs::write(output_path, &bytes)?;
    }
    if !violations.is_empty() {
        return Err(policy(&format!(
            "EOL audit found {} index or changed-file violations",
            violations.len()
        )));
    }
    Ok(EolAuditResult {
        status: "validated".to_owned(),
        tracked_text_files,
        changed_text_files,
        violations: 0,
        semantic_hash: hash::bytes(&bytes),
        output_path: output_path.to_path_buf(),
    })
}

fn git(root: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Err(policy(&format!("git {} failed", arguments.join(" "))));
    }
    Ok(output.stdout)
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}
