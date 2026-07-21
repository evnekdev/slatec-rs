//! One-command portable release-readiness validation.

use crate::error::{CorpusError, Result};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

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
    let output = loop {
        let output = command.current_dir(root).output()?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        if output.status.success()
            || retries == 4
            || !(stderr.contains("user-mapped section open") || stderr.contains("os error 1224"))
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
