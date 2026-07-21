//! Audits stable user-facing terminology and public module names.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize)]
pub struct PublicSurfaceResult {
    pub status: String,
    pub files_scanned: usize,
    pub violations: usize,
    pub semantic_hash: String,
    pub output_path: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
struct Violation {
    path: String,
    line: usize,
    surface: String,
    matched_term: String,
    text: String,
}

pub fn generate(root: &Path, output_path: &Path) -> Result<PublicSurfaceResult> {
    let mut targets = Vec::<(PathBuf, &'static str, bool)>::new();
    add_tree(
        &mut targets,
        &root.join("crates/slatec-sys/src"),
        "public-rustdoc",
        true,
    )?;
    add_tree(
        &mut targets,
        &root.join("docs/api"),
        "current-api-documentation",
        false,
    )?;
    add_tree(
        &mut targets,
        &root.join("docs/reference"),
        "reference-documentation",
        false,
    )?;
    targets.push((
        root.join("docs/architecture/slatec-sys-public-raw-api.md"),
        "public-api-architecture",
        false,
    ));
    targets.push((root.join("README.md"), "workspace-readme", false));
    for entry in fs::read_dir(root.join("crates"))? {
        let crate_dir = entry?.path();
        if crate_dir.is_dir() {
            let readme = crate_dir.join("README.md");
            if readme.is_file() {
                targets.push((readme, "crate-readme", false));
            }
            let manifest = crate_dir.join("Cargo.toml");
            if manifest.is_file() {
                targets.push((manifest, "public-feature-manifest", false));
            }
        }
    }
    targets.push((root.join("Cargo.toml"), "workspace-feature-manifest", false));
    targets.sort_by(|left, right| left.0.cmp(&right.0));
    targets.dedup_by(|left, right| left.0 == right.0);

    let mut violations = Vec::new();
    let mut files_scanned = 0usize;
    for (path, surface, rustdoc_only) in targets {
        if !path.is_file() || historical_path(root, &path) {
            continue;
        }
        files_scanned += 1;
        let text = fs::read_to_string(&path)?;
        for (index, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            if rustdoc_only
                && !(trimmed.starts_with("//!")
                    || trimmed.starts_with("///")
                    || trimmed.starts_with("#[doc")
                    || trimmed.starts_with("pub mod"))
            {
                continue;
            }
            if let Some(term) = forbidden_term(line) {
                violations.push(Violation {
                    path: slash_path(path.strip_prefix(root).unwrap_or(&path)),
                    line: index + 1,
                    surface: surface.to_owned(),
                    matched_term: term,
                    text: line.trim().to_owned(),
                });
            }
        }
    }
    let report = json!({
        "schema_id":"slatec-sys.public-api.public-surface-terminology",
        "schema_version":1,
        "policy":"Current public API surfaces use mathematical and ABI terminology; historical milestone labels are confined to a narrow path allowlist.",
        "historical_allowlist":["docs/history/raw-api-milestones/**"],
        "files_scanned":files_scanned,
        "violation_count":violations.len(),
        "violations":violations,
        "status":if violations.is_empty() {"pass"} else {"fail"},
    });
    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if fs::read(output_path).ok().as_deref() != Some(bytes.as_slice()) {
        fs::write(output_path, &bytes)?;
    }
    Ok(PublicSurfaceResult {
        status: if violations.is_empty() {
            "generated-pass"
        } else {
            "generated-fail"
        }
        .to_owned(),
        files_scanned,
        violations: violations.len(),
        semantic_hash: hash::bytes(&bytes),
        output_path: output_path.to_path_buf(),
    })
}

pub fn validate(root: &Path, output_path: &Path) -> Result<PublicSurfaceResult> {
    let mut result = generate(root, output_path)?;
    if result.violations != 0 {
        return Err(policy(&format!(
            "public-surface terminology audit found {} violations; see {}",
            result.violations,
            output_path.display()
        )));
    }
    result.status = "validated".to_owned();
    Ok(result)
}

fn add_tree(
    targets: &mut Vec<(PathBuf, &'static str, bool)>,
    root: &Path,
    surface: &'static str,
    rustdoc_only: bool,
) -> Result<()> {
    if !root.exists() {
        return Ok(());
    }
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| {
                if rustdoc_only {
                    extension == "rs"
                } else {
                    extension == "md" || extension == "rs"
                }
            }) {
                targets.push((path, surface, rustdoc_only));
            }
        }
    }
    Ok(())
}

fn forbidden_term(line: &str) -> Option<String> {
    let lower = line.to_ascii_lowercase();
    for letter in ['a', 'b', 'c', 'd'] {
        for separator in [' ', '_', '-'] {
            let term = format!("batch{separator}{letter}");
            if contains_term(&lower, &term) {
                return Some(term);
            }
        }
        let term = format!("raw-batch-{letter}");
        if contains_term(&lower, &term) {
            return Some(term);
        }
    }
    for term in [
        "#[deprecated",
        "compatibility re-export",
        "compatibility alias",
        "compatibility-only",
        "deprecated path",
        "deprecated compatibility",
        "previous canonical path",
        "migration from old namespace",
        "slatec_sys::generated",
        "slatec_sys::families",
        "special_scalar_expanded",
        "nonlinear::complex",
        "::numerical::",
        "slatec_sys::eigen::",
    ] {
        if lower.contains(term) {
            return Some(term.to_owned());
        }
    }
    None
}

fn contains_term(line: &str, term: &str) -> bool {
    line.match_indices(term).any(|(start, _)| {
        let before = line[..start].chars().next_back();
        let after = line[start + term.len()..].chars().next();
        before.is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_')
            && after.is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_')
    })
}

fn historical_path(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root)
        .ok()
        .is_some_and(|relative| slash_path(relative).starts_with("docs/history/"))
}

fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}
