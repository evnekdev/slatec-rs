use crate::archive::{ArchiveInventory, ArchiveMember};
use crate::diagnostics::{Diagnostic, StageStatus};
use crate::error::{CorpusError, Result};
use crate::hash;
use crate::policy::Policy;
use crate::{TOOL_NAME, TOOL_SEMANTIC_VERSION, TOOL_VERSION};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: &str = "1.0.0";
const DETERMINISTIC_CREATED_AT: &str = "1970-01-01T00:00:00Z";

pub struct ManifestResult {
    pub snapshot_id: String,
    pub output_dir: PathBuf,
    pub status: StageStatus,
    pub semantic_hash: String,
}

pub fn snapshot_id(policy: &Policy) -> String {
    let inputs = format!(
        "policy-v{}\u{1f}{}\u{1f}{}\u{1f}src/*.f\u{1f}raw-extraction-v1\u{1f}{}",
        policy.version, policy.semantic_hash, policy.sha256, TOOL_SEMANTIC_VERSION
    );
    format!("slatec-corpus-{}", &hash::bytes(inputs.as_bytes())[..20])
}

pub fn generate(
    output_dir: &Path,
    policy: &Policy,
    inventory: &ArchiveInventory,
) -> Result<ManifestResult> {
    let snapshot_id = snapshot_id(policy);
    validate_provider_set(inventory)?;
    let diagnostics = vec![Diagnostic::new(
        "CORPUS-PROVIDER-IDENTITY-PENDING",
        "warning",
        "manifest",
        vec!["program-unit uniqueness is pending a future fixed-form source scanner".to_owned()],
    )];
    let status = StageStatus::SuccessWithReviewItems;
    let artifact = json!({
        "id": "artifact-slatec-source-archive",
        "schema_id": "slatec-rs/artifact-manifest",
        "schema_version": SCHEMA_VERSION,
        "snapshot_id": snapshot_id,
        "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
        "created_at": DETERMINISTIC_CREATED_AT,
        "source_record_ids": [policy.source_id],
        "review_state": "machine_checked",
        "artifact_id": policy.source_id,
        "filename": policy.artifact_name,
        "source_url": policy.url,
        "sha256": policy.sha256,
        "compressed_size": policy.compressed_bytes,
        "expected_regular_members": policy.regular_members,
        "expected_selected_fortran_members": policy.selected_files,
        "policy_version": policy.version,
        "policy_semantic_hash": policy.semantic_hash,
        "tool_version": TOOL_VERSION,
        "tool_semantic_version": TOOL_SEMANTIC_VERSION,
        "input_artifact_hash": policy.sha256,
        "configuration_hash": policy.semantic_hash,
        "archive_member_inventory_hash": inventory.semantic_hash,
        "deterministic_ordering": "raw archive path bytewise lexical order",
        "stage_status": status,
        "warnings": diagnostics,
        "unparsed_or_unsupported_material": []
    });
    let members: Vec<Value> = inventory
        .members
        .iter()
        .map(|member| member_record(member, &snapshot_id))
        .collect();
    let source_files: Vec<Value> = inventory
        .members
        .iter()
        .filter(|member| member.entry_type == "regular")
        .map(|member| source_file_record(member, &snapshot_id))
        .collect();
    let providers: Vec<Value> = inventory.members.iter()
        .filter(|member| member.selected)
        .map(|member| json!({
            "id": format!("provider-{}", &hash::bytes(member.normalized_path.as_bytes())[..16]),
            "schema_id": "slatec-rs/selected-provider",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot_id,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": DETERMINISTIC_CREATED_AT,
            "source_record_ids": [member.id],
            "review_state": "machine_checked",
            "provider_identity": member.normalized_path,
            "provider_identity_policy": "filename-level-policy-v1",
            "source_file_id": format!("source-file-{}", &hash::bytes(member.normalized_path.as_bytes())[..16]),
            "member_sha256": member.raw_content_sha256,
            "selection_state": "selected",
            "program_unit_validation": "pending-future-source-scanner"
        }))
        .collect();
    let diagnostic_records: Vec<Value> = diagnostics
        .iter()
        .map(|diagnostic| {
            json!({
                "id": diagnostic.id,
                "schema_id": "slatec-rs/diagnostic",
                "schema_version": SCHEMA_VERSION,
                "snapshot_id": snapshot_id,
                "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
                "created_at": DETERMINISTIC_CREATED_AT,
                "source_record_ids": [],
                "review_state": "review_requested",
                "rule_id": diagnostic.rule_id,
                "severity": diagnostic.severity,
                "stage": diagnostic.stage,
                "message_template_id": diagnostic.message_template_id,
                "arguments": diagnostic.arguments,
                "review_impact": diagnostic.review_impact
            })
        })
        .collect();

    let mut files = BTreeMap::new();
    files.insert("artifact-manifest.json", canonical_json(&artifact)?);
    files.insert(
        "archive-members.json",
        canonical_json(&json!({ "records": members }))?,
    );
    files.insert(
        "source-files.json",
        canonical_json(&json!({ "records": source_files }))?,
    );
    files.insert(
        "selected-providers.json",
        canonical_json(&json!({
            "records": providers,
            "filename_uniqueness": "verified",
            "program_unit_uniqueness": "pending-future-source-scanner"
        }))?,
    );
    files.insert(
        "diagnostics.json",
        canonical_json(&json!({
            "stage_status": status,
            "records": diagnostic_records
        }))?,
    );
    let semantic_hash = semantic_hash(&files);
    let output_hashes: BTreeMap<_, _> = files
        .iter()
        .map(|(name, bytes)| ((*name).to_owned(), hash::bytes(bytes)))
        .collect();
    let manifest = json!({
        "id": format!("run-{snapshot_id}"),
        "schema_id": "slatec-rs/corpus-run",
        "schema_version": SCHEMA_VERSION,
        "snapshot_id": snapshot_id,
        "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
        "created_at": DETERMINISTIC_CREATED_AT,
        "source_record_ids": [policy.source_id],
        "review_state": "machine_checked",
        "policy_version": policy.version,
        "policy_semantic_hash": policy.semantic_hash,
        "input_artifact_hash": policy.sha256,
        "input_member_manifest_hash": inventory.semantic_hash,
        "configuration_hash": policy.semantic_hash,
        "tool_version": TOOL_VERSION,
        "tool_semantic_version": TOOL_SEMANTIC_VERSION,
        "parser_semantic_version": "not-implemented",
        "deterministic_ordering": "sorted stable identifiers and raw archive paths",
        "output_semantic_hash": semantic_hash,
        "output_file_hashes": output_hashes,
        "stage_status": status,
        "warnings": ["program-unit uniqueness is deferred; no Fortran parser was run"],
        "unparsed_or_unsupported_material": ["Fortran lexical and semantic parsing is outside this tooling layer"]
    });
    files.insert("manifest.json", canonical_json(&manifest)?);
    files.insert(
        "manifest-summary.md",
        summary(&snapshot_id, &semantic_hash, inventory).into_bytes(),
    );
    promote_files(output_dir, &snapshot_id, &files)?;
    Ok(ManifestResult {
        snapshot_id,
        output_dir: output_dir.to_owned(),
        status,
        semantic_hash,
    })
}

fn member_record(member: &ArchiveMember, snapshot_id: &str) -> Value {
    json!({
        "id": member.id,
        "schema_id": "slatec-rs/archive-member",
        "schema_version": SCHEMA_VERSION,
        "snapshot_id": snapshot_id,
        "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
        "created_at": DETERMINISTIC_CREATED_AT,
        "source_record_ids": ["artifact-slatec-source-archive"],
        "review_state": "machine_checked",
        "raw_archive_path": member.raw_archive_path,
        "normalized_path": member.normalized_path,
        "entry_type": member.entry_type,
        "uncompressed_size": member.uncompressed_size,
        "selected": member.selected,
        "exclusion_reason": member.exclusion_reason,
        "raw_content_sha256": member.raw_content_sha256,
        "source_artifact_id": member.source_artifact_id
    })
}

fn source_file_record(member: &ArchiveMember, snapshot_id: &str) -> Value {
    let id = format!(
        "source-file-{}",
        &hash::bytes(member.normalized_path.as_bytes())[..16]
    );
    json!({
        "id": id,
        "schema_id": "slatec-rs/source-file",
        "schema_version": SCHEMA_VERSION,
        "snapshot_id": snapshot_id,
        "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
        "created_at": DETERMINISTIC_CREATED_AT,
        "source_record_ids": [member.id],
        "review_state": "machine_checked",
        "artifact_id": member.source_artifact_id,
        "archive_member_path": member.raw_archive_path,
        "normalized_comparison_path": member.normalized_path,
        "member_sha256": member.raw_content_sha256,
        "byte_length": member.uncompressed_size,
        "selected_state": if member.selected { "selected" } else { "excluded" },
        "selection_reason": member.exclusion_reason,
        "program_units": { "collection_state": "unknown", "record_ids": [] },
        "extraction_scope": "raw archive inventory only; no Fortran parsing"
    })
}

fn validate_provider_set(inventory: &ArchiveInventory) -> Result<()> {
    let mut paths = BTreeSet::new();
    let mut identities = BTreeSet::new();
    for member in inventory.members.iter().filter(|member| member.selected) {
        if !paths.insert(member.normalized_path.clone()) {
            return Err(CorpusError::Verification(format!(
                "duplicate selected archive-member path {}",
                member.normalized_path
            )));
        }
        if !identities.insert(member.normalized_path.to_ascii_lowercase()) {
            return Err(CorpusError::Verification(format!(
                "duplicate selected filename-level provider identity {}",
                member.normalized_path
            )));
        }
    }
    Ok(())
}

fn canonical_json(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn semantic_hash(files: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut material = Vec::new();
    for (name, bytes) in files {
        material.extend_from_slice(name.as_bytes());
        material.push(0);
        material.extend_from_slice(bytes);
        material.push(0);
    }
    hash::bytes(&material)
}

fn promote_files(
    output_dir: &Path,
    snapshot_id: &str,
    files: &BTreeMap<&str, Vec<u8>>,
) -> Result<()> {
    let parent = output_dir
        .parent()
        .ok_or_else(|| CorpusError::Archive("output directory must have a parent".to_owned()))?;
    fs::create_dir_all(parent)?;
    let name = output_dir
        .file_name()
        .ok_or_else(|| CorpusError::Archive("output directory must have a name".to_owned()))?;
    let staging = parent.join(format!("{}.staging-{snapshot_id}", name.to_string_lossy()));
    if staging.exists() {
        return Err(CorpusError::Verification(format!(
            "manifest staging directory already exists: {}",
            staging.display()
        )));
    }
    fs::create_dir(&staging)?;
    for (name, bytes) in files {
        fs::write(staging.join(name), bytes)?;
    }
    if output_dir.exists() {
        for (name, bytes) in files {
            let existing = fs::read(output_dir.join(name)).map_err(|_| {
                CorpusError::Verification(format!(
                    "existing manifest is incomplete: {}",
                    output_dir.display()
                ))
            })?;
            if existing != *bytes {
                return Err(CorpusError::Verification(format!(
                    "existing manifest differs; preserve it and choose an explicit new output directory: {}",
                    output_dir.display()
                )));
            }
        }
        fs::remove_dir_all(staging)?;
        return Ok(());
    }
    fs::rename(staging, output_dir)?;
    Ok(())
}

fn summary(snapshot_id: &str, semantic_hash: &str, inventory: &ArchiveInventory) -> String {
    format!(
        "# Canonical SLATEC corpus manifest\n\n- Snapshot: `{snapshot_id}`\n- Semantic hash: `{semantic_hash}`\n- Regular archive members: {}\n- Selected `src/*.f` providers: {}\n- Program-unit uniqueness: pending a later source scanner; filename-level uniqueness passed.\n\nThis manifest inventories a checksum-pinned archive. It does not assert historical originality, redistribution permission, ABI safety, dependency completeness, or native-component validity.\n",
        inventory.regular_member_count, inventory.selected_fortran_member_count
    )
}
