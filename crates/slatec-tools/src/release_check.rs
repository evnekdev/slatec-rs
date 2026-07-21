//! One-command portable release-readiness validation.

use crate::error::{CorpusError, Result};
use serde_json::json;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};

/// A Git index lock older than this is eligible for conservative recovery on
/// Windows. Active Git processes keep the lock open, so removal fails rather
/// than racing them; non-Windows platforms do not unlink locks automatically.
const STALE_GIT_INDEX_LOCK_AGE: Duration = Duration::from_secs(30);

pub fn run(root: &Path, output_path: &Path) -> Result<()> {
    let executable = std::env::current_exe()?;
    let validators = [
        "validate-agent-guidance",
        "validate-raw-api-inventory",
        "validate-raw-batch-a",
        "validate-raw-batch-b",
        "validate-raw-batch-c",
        "validate-final-raw-api-disposition",
        "validate-all-feature-coverage",
        "validate-unique-ffi-declarations",
        "validate-public-surface-terminology",
        "validate-release-readiness",
        "validate-public-api-semantic-review",
        "validate-rendered-rustdoc-audit",
        "validate-package-contents",
        "validate-registry-simulation",
    ];
    let mut records = Vec::new();
    for validator in validators {
        execute(
            root,
            output_path,
            &mut records,
            validator,
            Command::new(&executable).args([validator, "--offline"]),
        )?;
    }
    for (label, program, arguments, rustdoc_warnings) in [
        (
            "formatting",
            "cargo",
            vec!["fmt", "--all", "--", "--check"],
            false,
        ),
        (
            "workspace-check",
            "cargo",
            vec!["check", "--workspace", "--offline"],
            false,
        ),
        (
            "strict-clippy",
            "cargo",
            vec![
                "clippy",
                "--workspace",
                "--all-targets",
                "--offline",
                "--",
                "-D",
                "warnings",
            ],
            false,
        ),
        (
            "workspace-tests",
            "cargo",
            vec!["test", "--workspace", "--offline"],
            false,
        ),
        (
            "strict-rustdoc",
            "cargo",
            vec!["doc", "--workspace", "--no-deps", "--offline"],
            true,
        ),
        (
            "workspace-doctests",
            "cargo",
            vec!["test", "--doc", "--workspace", "--offline"],
            false,
        ),
        (
            "package-assembly",
            "cargo",
            vec![
                "package",
                "--workspace",
                "--exclude",
                "slatec-tools",
                "--allow-dirty",
                "--no-verify",
                "--offline",
            ],
            false,
        ),
        ("git-diff-check", "git", vec!["diff", "--check"], false),
    ] {
        let mut command = Command::new(program);
        command.args(arguments);
        if rustdoc_warnings {
            command.env("RUSTDOCFLAGS", "-D warnings");
        }
        execute(root, output_path, &mut records, label, &mut command)?;
    }
    execute(
        root,
        output_path,
        &mut records,
        "eol-audit",
        Command::new(&executable).args(["validate-eol", "--offline"]),
    )?;
    write_report(output_path, &records, "pass")?;
    Ok(())
}

fn execute(
    root: &Path,
    output_path: &Path,
    records: &mut Vec<serde_json::Value>,
    label: &str,
    command: &mut Command,
) -> Result<()> {
    let mut retries = 0;
    let mut stale_git_index_lock_recoveries = 0;
    let output = loop {
        if recover_stale_git_index_lock(root, STALE_GIT_INDEX_LOCK_AGE)? {
            stale_git_index_lock_recoveries += 1;
        }
        let output = command.current_dir(root).output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        if output.status.success()
            || retries == 4
            || !(stderr.contains("user-mapped section open")
                || stderr.contains("os error 1224")
                || stderr.contains("index.lock"))
        {
            break output;
        }
        retries += 1;
        std::thread::sleep(Duration::from_millis(250));
    };
    records.push(json!({
        "check":label,
        "status":if output.status.success() { "pass" } else { "fail" },
        "exit_code":output.status.code(),
        "transient_file_lock_retries":retries,
        "stale_git_index_lock_recoveries":stale_git_index_lock_recoveries,
    }));
    if !output.status.success() {
        let report_note = write_report(output_path, records, "fail")
            .err()
            .map(|error| format!("\nrelease-report write failed: {error}"))
            .unwrap_or_default();
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(policy(&format!(
            "release check `{label}` failed\nstdout:\n{}\nstderr:\n{}{}",
            stdout.trim(),
            stderr.trim(),
            report_note
        )));
    }
    Ok(())
}

/// Reclaim an abandoned index lock only when it is old enough to be stale.
///
/// This intentionally does not implement a portable "remove any old lock"
/// policy: POSIX permits unlinking an open file. Git for Windows keeps an
/// active lock unavailable for deletion, which gives this repository a safe
/// way to recover the zero-byte locks left by interrupted validation commands.
fn recover_stale_git_index_lock(root: &Path, minimum_age: Duration) -> Result<bool> {
    let Some(lock_path) = git_index_lock_path(root) else {
        return Ok(false);
    };
    let metadata = match fs::metadata(&lock_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
        Err(error) => return Err(error.into()),
    };
    let age = metadata
        .modified()
        .ok()
        .and_then(|modified| SystemTime::now().duration_since(modified).ok())
        .unwrap_or(minimum_age);
    if age < minimum_age {
        return Ok(false);
    }

    #[cfg(windows)]
    {
        match fs::remove_file(&lock_path) {
            Ok(()) => Ok(true),
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::NotFound | ErrorKind::PermissionDenied
                ) =>
            {
                Ok(false)
            }
            Err(error) => Err(error.into()),
        }
    }
    #[cfg(not(windows))]
    {
        let _ = lock_path;
        Ok(false)
    }
}

fn git_index_lock_path(root: &Path) -> Option<PathBuf> {
    let dot_git = root.join(".git");
    if dot_git.is_dir() {
        return Some(dot_git.join("index.lock"));
    }
    let git_file = fs::read_to_string(dot_git).ok()?;
    let git_dir = git_file
        .lines()
        .find_map(|line| line.strip_prefix("gitdir: "))?;
    let git_dir = PathBuf::from(git_dir.trim());
    Some(
        if git_dir.is_absolute() {
            git_dir
        } else {
            root.join(git_dir)
        }
        .join("index.lock"),
    )
}

fn write_report(path: &Path, records: &[serde_json::Value], status: &str) -> Result<()> {
    let report = json!({
        "schema_id":"slatec-rs.release-check",
        "schema_version":"1.0.0",
        "status":status,
        "checks":records,
        "native_scope":"portable checks only; GNU MinGW native regressions remain a separate required profile",
    });
    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if fs::read(path).ok().as_deref() != Some(bytes.as_slice()) {
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
    fn resolves_index_lock_inside_a_worktree_gitdir_file() {
        let temporary = tempfile::tempdir().unwrap();
        let git_dir = temporary.path().join("metadata");
        fs::create_dir(&git_dir).unwrap();
        fs::write(
            temporary.path().join(".git"),
            format!("gitdir: {}\n", git_dir.display()),
        )
        .unwrap();
        assert_eq!(
            git_index_lock_path(temporary.path()),
            Some(git_dir.join("index.lock"))
        );
    }

    #[cfg(windows)]
    #[test]
    fn reclaims_an_old_unowned_windows_index_lock() {
        let temporary = tempfile::tempdir().unwrap();
        let dot_git = temporary.path().join(".git");
        fs::create_dir(&dot_git).unwrap();
        let lock = dot_git.join("index.lock");
        fs::write(&lock, b"").unwrap();

        assert!(recover_stale_git_index_lock(temporary.path(), Duration::ZERO).unwrap());
        assert!(!lock.exists());
    }

    #[cfg(windows)]
    #[test]
    fn leaves_a_fresh_windows_index_lock_in_place() {
        let temporary = tempfile::tempdir().unwrap();
        let dot_git = temporary.path().join(".git");
        fs::create_dir(&dot_git).unwrap();
        let lock = dot_git.join("index.lock");
        fs::write(&lock, b"").unwrap();

        assert!(!recover_stale_git_index_lock(temporary.path(), Duration::MAX).unwrap());
        assert!(lock.exists());
    }
}
