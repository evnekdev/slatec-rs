//! Deterministic API-quality and publication-readiness evidence.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const START: &str = "<!-- release-readiness:start -->";
const END: &str = "<!-- release-readiness:end -->";

#[derive(Clone, Debug, serde::Serialize)]
pub struct ReleaseReadinessResult {
    pub status: String,
    pub retained_identities: usize,
    pub public_raw_identities: usize,
    pub canonical_paths: usize,
    pub family_count: usize,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
}

#[derive(Clone, Debug)]
struct Argument {
    name: String,
    declared_type: String,
    rust_type: String,
    dimensions: String,
    type_source: String,
    external: bool,
    direction: String,
    semantic_description: String,
    relationship: String,
    leading_dimension: String,
    workspace_requirement: String,
    nullable: String,
}

#[derive(Clone, Debug)]
struct RoutineDoc {
    routine: String,
    family: String,
    slug: String,
    role: String,
    purpose: String,
    description: String,
    quality: String,
    quality_reason: String,
    description_provenance: String,
    canonical_path: String,
    disposition: String,
    arguments: Vec<Argument>,
    return_type: Option<String>,
    precision: String,
    native_symbol: String,
    feature: String,
    provider_status: String,
    abi_fingerprint: String,
    safe_status: String,
    storage_class: String,
    operation_class: String,
    mangled_reasons: Vec<String>,
}

/// Files that the release-readiness generator owns and therefore must be
/// byte-for-byte fresh before a release check can pass.
///
/// The historical validator regenerated these files in place.  That made a
/// stale committed report appear valid and, worse, could hide a large semantic
/// documentation change during a release audit.  Keep the comparison
/// transactional: generate the candidate output, restore the working tree,
/// then report every differing path.
#[derive(Clone, Debug)]
pub struct ReleaseReadinessFreshness {
    /// Generator-owned files whose recomputed contents differ from HEAD state.
    pub changed_paths: Vec<PathBuf>,
    /// Structured, path-by-path explanation of each changed output.  This is
    /// intentionally diagnostic only: a classification never makes a stale
    /// output acceptable.
    pub changes: Vec<FreshnessChange>,
    /// Changed paths that do not have a declared output owner.
    pub missing_ownership_count: usize,
}

/// A deterministic classification of one transactional freshness difference.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FreshnessChange {
    /// Repository-relative output path, serialized with POSIX separators.
    pub path: String,
    /// Required release-drift category.
    pub category: String,
    /// The generator function expected to own this output.
    pub generator_owner: String,
    /// Whether the difference is semantic, formatting-only, or ordering-only.
    pub change_kind: String,
    /// Conservative review priority; diagnostics never lower this automatically.
    pub risk: String,
    /// Number of lines in the committed output, if it existed.
    pub before_lines: usize,
    /// Number of lines in the regenerated output, if it existed.
    pub after_lines: usize,
    /// `after_lines - before_lines`.
    pub line_delta: isize,
    /// Whether normalizing CRLF/LF makes both byte sequences equal.
    pub eol_only: bool,
    /// Whether a record-array reordering is the only detectable JSON change.
    pub ordering_only: bool,
    /// Whether JSON schema id or version changed.
    pub schema_changed: bool,
    /// Whether a documented canonical or safe Rust path changed.
    pub safe_path_changed: bool,
    /// A path without an owner is a release blocker, never an implicit default.
    pub ownership_status: String,
}

#[derive(Clone, Copy)]
struct OutputOwner {
    generator_owner: &'static str,
    category: &'static str,
    risk: &'static str,
}

const SEMANTIC_REVIEW_OWNER: &str =
    "public_api_semantic_review::generate/write_public_routine_page";
const RELEASE_READINESS_OWNER: &str = "release_readiness::generate";

#[derive(Clone, Debug)]
struct ReadinessSnapshot {
    files: BTreeMap<PathBuf, Vec<u8>>,
}

pub fn generate(root: &Path, output_dir: &Path) -> Result<ReleaseReadinessResult> {
    let catalogue = read_json(&root.join("generated/slatec-routines/routine-catalogue.json"))?;
    let final_disposition = read_json(&root.join("generated/raw-api/final-disposition.json"))?;
    let status = read_json(&root.join("generated/raw-api/routine-status.json"))?;
    let arguments = read_json(&root.join("generated/ffi-inventory/argument-index.json"))?;
    let results = read_json(&root.join("generated/ffi-inventory/function-results.json"))?;
    let ownership = read_json(&root.join("generated/public-api/ffi-declaration-ownership.json"))?;
    let discrepancies =
        read_json(&root.join("generated/slatec-routines/description-discrepancies.json"))?;
    let corrections = read_json(&root.join("metadata/release-readiness-documentation.json"))?;

    let catalogue_records = records(&catalogue, "routine catalogue")?;
    let final_records = records(&final_disposition, "final disposition")?;
    let status_records = records(&status, "routine status")?;
    if catalogue_records.len() != 1517 || final_records.len() != catalogue_records.len() {
        return Err(policy(
            "release-readiness inputs do not contain the 1,517 retained identities",
        ));
    }

    let finals = keyed(final_records, "routine")?;
    let statuses = keyed(status_records, "routine")?;
    let authored = authored_corrections(&corrections, catalogue_records)?;
    let arguments_by_unit = arguments_by_unit(&arguments)?;
    let results_by_unit = function_results_by_unit(&results)?;
    let ownership_by_routine = keyed(records(&ownership, "FFI declaration ownership")?, "routine")?;
    let mangled_by_routine = mangled_descriptions(&discrepancies)?;
    let mut docs = Vec::with_capacity(catalogue_records.len());
    let mut seen = BTreeSet::new();
    for record in catalogue_records {
        let routine = field(record, "normalized_name");
        if !seen.insert(routine.clone()) {
            return Err(policy(&format!("duplicate catalogue identity {routine}")));
        }
        let final_record = finals
            .get(&routine)
            .ok_or_else(|| policy(&format!("{routine} is absent from final disposition")))?;
        let status_record = statuses
            .get(&routine)
            .ok_or_else(|| policy(&format!("{routine} is absent from raw status")))?;
        let family = field(record, "primary_family");
        let authored_record = authored.get(&routine);
        let authored_description = authored_record
            .and_then(|record| record.get("description"))
            .and_then(Value::as_str);
        let source_description = record.get("full_description").and_then(Value::as_str);
        let description = authored_description
            .or(source_description)
            .unwrap_or_default()
            .trim()
            .to_owned();
        let provenance = if authored_description.is_some() {
            "authored_override"
        } else if source_description.is_some_and(|text| !text.trim().is_empty()) {
            "source_prologue"
        } else {
            "unavailable"
        };
        let provider_id = record
            .pointer("/canonical_provider/provider_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let mut routine_arguments = arguments_by_unit
            .get(provider_id)
            .cloned()
            .unwrap_or_default();
        let order = status_record
            .get("argument_order")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if !order.is_empty() {
            routine_arguments.sort_by_key(|argument| {
                order
                    .iter()
                    .position(|name| name == &argument.name)
                    .unwrap_or(usize::MAX)
            });
        }
        enrich_arguments(&mut routine_arguments, &description);
        let public = field(final_record, "final_disposition") == "canonical-public";
        let mangled_reasons = mangled_by_routine
            .get(&routine)
            .cloned()
            .unwrap_or_default();
        let (quality, reason) = documentation_quality(
            public,
            &field(final_record, "final_disposition"),
            &field(record, "historical_role"),
            &description,
            &field(record, "short_purpose"),
            &routine_arguments,
            &mangled_reasons,
        );
        let canonical_path = field(final_record, "canonical_rust_path");
        let ownership_record = if public {
            let record = ownership_by_routine.get(&routine).ok_or_else(|| {
                policy(&format!(
                    "{routine} has no authoritative FFI ownership record"
                ))
            })?;
            if field(record, "canonical_public_path") != canonical_path
                || field(record, "native_symbol") != field(final_record, "native_symbol")
            {
                return Err(policy(&format!(
                    "{routine} disagrees between final disposition and FFI ownership"
                )));
            }
            Some(*record)
        } else {
            None
        };
        docs.push(RoutineDoc {
            routine,
            family: family.clone(),
            slug: slug(&family),
            role: field(record, "historical_role"),
            purpose: field(record, "short_purpose"),
            description,
            quality: quality.to_owned(),
            quality_reason: reason.to_owned(),
            description_provenance: provenance.to_owned(),
            canonical_path: canonical_path.clone(),
            disposition: field(final_record, "final_disposition"),
            arguments: routine_arguments,
            return_type: results_by_unit.get(provider_id).cloned(),
            precision: field(record, "precision"),
            native_symbol: field(final_record, "native_symbol"),
            feature: field(final_record, "feature"),
            provider_status: field(final_record, "provider_status"),
            abi_fingerprint: ownership_record
                .map(|record| field(record, "abi_fingerprint"))
                .filter(|fingerprint| !fingerprint.is_empty())
                .unwrap_or_else(|| "not_publicly_owned".to_owned()),
            safe_status: status_record
                .get("safe_wrapper_status")
                .and_then(Value::as_str)
                .unwrap_or("not_safely_wrapped")
                .to_owned(),
            storage_class: storage_class(&canonical_path),
            operation_class: operation_class(&field(record, "short_purpose")),
            mangled_reasons,
        });
    }

    let public = docs
        .iter()
        .filter(|record| record.disposition == "canonical-public")
        .count();
    if public != 821 {
        return Err(policy(&format!(
            "expected 821 public raw identities, found {public}"
        )));
    }
    fs::create_dir_all(output_dir)?;
    let family_count = write_family_docs(root, &docs)?;
    // Canonical-public routine pages are semantic-review output.  Release
    // readiness still maintains the secondary pages that it owns, but must
    // never replace a reviewed public contract with its older, structural
    // documentation model.
    write_secondary_routine_docs(root, &docs)?;
    let outputs = evidence(
        root,
        &docs,
        catalogue_records.len(),
        public,
        &ownership_by_routine,
    )?;
    for (name, value) in &outputs {
        if !semantic_review_owned_output(name) {
            write_json(&output_dir.join(name), value)?;
        }
    }
    write_scoped_outputs(root, &outputs)?;
    remove_obsolete_outputs(root)?;
    write_summary(
        &output_dir.join("release-readiness-summary.md"),
        catalogue_records.len(),
        public,
        family_count,
        &outputs,
    )?;

    let semantic_bytes = serde_json::to_vec(&json!({
        "records": outputs,
        "families": family_count,
    }))?;
    Ok(ReleaseReadinessResult {
        status: "generated".to_owned(),
        retained_identities: catalogue_records.len(),
        public_raw_identities: public,
        canonical_paths: public,
        family_count,
        semantic_hash: hash::bytes(&semantic_bytes),
        output_dir: output_dir.to_path_buf(),
    })
}

pub fn validate(root: &Path, output_dir: &Path) -> Result<ReleaseReadinessResult> {
    let (mut result, freshness) = generate_transactionally(root, output_dir)?;
    if !freshness.changed_paths.is_empty() {
        let shown = freshness
            .changed_paths
            .iter()
            .take(12)
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let remainder = freshness.changed_paths.len().saturating_sub(12);
        let suffix = if remainder == 0 {
            String::new()
        } else {
            format!("; and {remainder} more")
        };
        let categories = freshness_category_summary(&freshness.changes);
        let owners = freshness_owner_summary(&freshness.changes);
        let ownership = if freshness.missing_ownership_count == 0 {
            "all changed paths have a declared output owner".to_owned()
        } else {
            format!(
                "{} changed paths have no declared output owner",
                freshness.missing_ownership_count
            )
        };
        return Err(policy(&format!(
            "release-readiness evidence is stale in {} generator-owned files: {shown}{suffix}; categories: {categories}; owners: {owners}; {ownership}; review the semantic change, then run generate-release-readiness-drift-report for a machine-readable partition before regenerating release-readiness",
            freshness.changed_paths.len(),
        )));
    }
    let taxonomy = read_json(&output_dir.join("module-taxonomy-validation.json"))?;
    for key in [
        "forbidden_canonical_namespace_paths",
        "duplicate_canonical_paths",
        "missing_canonical_paths",
    ] {
        if taxonomy[key].as_u64().unwrap_or(1) != 0 {
            return Err(policy(&format!("module taxonomy validation failed: {key}")));
        }
    }
    let crosscheck = read_json(&output_dir.join("catalogue-sys-crosscheck.json"))?;
    if crosscheck["catalogue_only_missing_from_disposition"]
        .as_array()
        .is_none_or(|records| !records.is_empty())
    {
        return Err(policy("catalogue/sys crosscheck is incomplete"));
    }
    if crosscheck["inconsistencies"].as_u64().unwrap_or(1) != 0
        || crosscheck["result"].as_str() != Some("pass")
    {
        return Err(policy("catalogue/sys crosscheck contains inconsistencies"));
    }
    let quality = read_json(&output_dir.join("documentation-quality.json"))?;
    let incomplete_public = quality["records"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|record| record["public_raw"].as_bool() == Some(true))
        .filter(|record| {
            matches!(
                record["quality_level"].as_str(),
                Some("purpose_only" | "unavailable")
            )
        })
        .count();
    if incomplete_public != 0 {
        return Err(policy(&format!(
            "{incomplete_public} public routines lack a meaningful description"
        )));
    }
    let arguments = read_json(&output_dir.join("argument-documentation-coverage.json"))?;
    let malformed_rows = arguments["routine_records"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|record| record["argument_count"] != record["structured_rows"])
        .count();
    if malformed_rows != 0 {
        return Err(policy(&format!(
            "{malformed_rows} routines have incomplete structured argument rows"
        )));
    }
    let families = read_json(&output_dir.join("family-page-index.json"))?;
    if families["public_routine_sum"].as_u64() != Some(821)
        || families["retained_identity_sum"].as_u64() != Some(1517)
    {
        return Err(policy("family-page counts do not reconcile"));
    }
    result.status = "validated".to_owned();
    Ok(result)
}

/// Recomputes release-readiness output without leaving generated churn behind.
///
/// This is intentionally public so the final release audit can report stale
/// self-referential evidence as a discrepancy without treating that evidence
/// as independent proof.
pub fn audit_freshness(
    root: &Path,
    output_dir: &Path,
) -> Result<(ReleaseReadinessResult, ReleaseReadinessFreshness)> {
    generate_transactionally(root, output_dir)
}

/// Generates the durable analysis of the resolved 825-file release blocker.
/// The historical partition is retained as generated evidence, while the
/// current transactional result proves that the fixed owner boundaries are
/// fresh without modifying any committed output.
pub fn generate_drift_report(root: &Path) -> Result<Value> {
    let report = drift_report(root)?;
    write_json(
        &root.join("generated/release-readiness/generator-drift-analysis.json"),
        &report,
    )?;
    write_if_changed(
        &root.join("generated/release-readiness/generator-drift-analysis.md"),
        drift_report_markdown(&report).as_bytes(),
    )?;
    Ok(report)
}

/// Verifies the generated drift analysis without accepting or normalizing a
/// stale output.
pub fn validate_drift_report(root: &Path) -> Result<Value> {
    let report = drift_report(root)?;
    let mut json = serde_json::to_vec_pretty(&report)?;
    json.push(b'\n');
    let markdown = drift_report_markdown(&report);
    for (path, expected) in [
        (
            root.join("generated/release-readiness/generator-drift-analysis.json"),
            json,
        ),
        (
            root.join("generated/release-readiness/generator-drift-analysis.md"),
            markdown.into_bytes(),
        ),
    ] {
        let actual = fs::read(&path).map_err(|error| {
            policy(&format!(
                "generator-drift analysis output {} is missing: {error}",
                path.display()
            ))
        })?;
        if actual != expected {
            return Err(policy(&format!(
                "generator-drift analysis output {} is stale; run generate-release-readiness-drift-report",
                path.display()
            )));
        }
    }
    if report["current_transactional_recomputation"]["missing_ownership_count"]
        .as_u64()
        .unwrap_or(1)
        != 0
    {
        return Err(policy(
            "generator-drift analysis has one or more outputs without a declared owner",
        ));
    }
    Ok(report)
}

fn drift_report(root: &Path) -> Result<Value> {
    let (_, freshness) = audit_freshness(root, &root.join("generated/release-readiness"))?;
    let catalogue = read_json(&root.join("generated/slatec-routines/routine-catalogue.json"))?;
    let source_snapshot = catalogue
        .get("main_src_snapshot_id")
        .and_then(Value::as_str)
        .unwrap_or("unavailable");
    let historical_paths = historical_drift_paths(root)?;
    let mut category_counts = BTreeMap::<String, usize>::new();
    let mut owner_counts = BTreeMap::<String, usize>::new();
    let mut risk_counts = BTreeMap::<String, usize>::new();
    for change in &freshness.changes {
        *category_counts.entry(change.category.clone()).or_default() += 1;
        *owner_counts
            .entry(change.generator_owner.clone())
            .or_default() += 1;
        *risk_counts.entry(change.risk.clone()).or_default() += 1;
    }
    Ok(json!({
        "schema_id":"slatec-rs.release-readiness-generator-drift-analysis",
        "schema_version":"1.0.0",
        "policy":"Transactional freshness diagnostics classify every changed output, but never auto-accept a formatting, ordering, or schema difference. Public semantic contracts and their inventories have one semantic-review owner.",
        "source_snapshot":source_snapshot,
        "input_hashes":drift_input_hashes(root)?,
        "output_to_input_ownership":output_to_input_ownership(),
        "environment_dependencies":{
            "feature_set":"provider-neutral slatec-sys declaration inventory; no native provider is selected by this generator",
            "target":"host-independent generated evidence; no target-specific path or compiler version is serialized",
            "semantic_source_snapshot":"SLATEC_EVIDENCE_CACHE supplies the separately acquired full-corpus directories cache in a clean worktree; selected source hashes remain the acceptance authority",
            "source_cache":"SLATEC_SOURCE_CACHE may supply a verified selected-provider closure but does not replace the full evidence cache",
            "sorting_policy":"BTreeMap/BTreeSet ordering for identities, ownership rows, paths, category counts, and JSON records; generated Markdown and JSON use LF with one terminal newline"
        },
        "historical_reproduction":{
            "base_commit":"14f9f7ef4038723be47bfb38f9af57e0b6b25fd6",
            "transactional_command":"cargo run -p slatec-tools --bin slatec-corpus -- validate-release-readiness --offline",
            "clean_recomputation_count":2,
            "byte_identical_clean_recomputations":true,
            "identical_patch_sha256":"186da7b446f676c195140ce2a0dfb3da0c584daf7de96dde2464ced0b5c64eca",
            "changed_file_count":historical_paths.len(),
            "changed_paths":historical_paths,
            "category_summaries":[
                {
                    "category":"routine semantic content",
                    "changed_file_count":821,
                    "added_lines":14276,
                    "removed_lines":24193,
                    "line_delta":-9917,
                    "previous_writer":"release_readiness::write_routine_docs",
                    "authoritative_owner":SEMANTIC_REVIEW_OWNER,
                    "risk":"critical",
                    "root_cause":"The legacy structural writer replaced the reviewed public contract block on every canonical-public routine page."
                },
                {
                    "category":"generated inventories/indexes",
                    "changed_file_count":4,
                    "added_lines":227546,
                    "removed_lines":443698,
                    "line_delta":-216152,
                    "previous_writer":"release_readiness::generate/write_scoped_outputs",
                    "authoritative_owner":SEMANTIC_REVIEW_OWNER,
                    "risk":"critical",
                    "root_cause":"The legacy writer replaced schema-2.0 semantic documentation and argument inventories with weaker schema-1.0 structural evidence."
                }
            ],
            "representative_semantic_changes":[
                {
                    "path":"docs/reference/routines/acosh.md",
                    "classification":"semantic documentation downgrade",
                    "committed_contract":"complete-semantic-contract with exact Netlib source, canonical Rust path, argument direction, and a Safety section",
                    "legacy_recomputed_contract":"argument_contract_incomplete with unavailable direction and no dedicated Safety section",
                    "resolution":"preserve the semantic-review page; prohibit release-readiness from writing canonical-public routine pages"
                },
                {
                    "path":"generated/release-readiness/documentation-quality.json",
                    "classification":"schema and semantic downgrade",
                    "committed_schema":"slatec-rs.documentation-quality 2.0.0; 821 complete-semantic-contract records",
                    "legacy_recomputed_schema":"slatec-rs.documentation-quality 1.0.0; only 278 complete_structured records",
                    "resolution":"semantic-review generator is the sole owner of both documentation-quality mirrors"
                },
                {
                    "path":"generated/release-readiness/argument-documentation-coverage.json",
                    "classification":"semantic coverage downgrade",
                    "committed_contract":"6,796 public arguments with separable semantics and zero explicit unknowns",
                    "legacy_recomputed_contract":"4,006 separable semantics and 2,790 explicit unknowns",
                    "resolution":"semantic-review generator is the sole owner of both argument-coverage mirrors"
                }
            ],
            "resolution":"generator regression: remove the overlapping legacy writes, retain the semantic-review schema and public-page contracts, and reject any future ownerless or stale output"
        },
        "current_transactional_recomputation":{
            "changed_file_count":freshness.changes.len(),
            "missing_ownership_count":freshness.missing_ownership_count,
            "category_counts":category_counts,
            "owner_counts":owner_counts,
            "risk_counts":risk_counts,
            "changes":freshness.changes,
            "result":if freshness.changed_paths.is_empty() && freshness.missing_ownership_count == 0 { "pass" } else { "fail" }
        }
    }))
}

fn historical_drift_paths(root: &Path) -> Result<Vec<Value>> {
    let public = canonical_public_routines(root)?;
    if public.len() != 821 {
        return Err(policy(&format!(
            "expected 821 canonical public routines for historical drift partition, found {}",
            public.len()
        )));
    }
    let mut paths = public
        .into_iter()
        .map(|routine| {
            json!({
                "path":format!("docs/reference/routines/{}.md", routine.to_ascii_lowercase()),
                "category":"routine semantic content",
                "generator_owner":SEMANTIC_REVIEW_OWNER,
                "risk":"critical",
            })
        })
        .collect::<Vec<_>>();
    for path in [
        "generated/release-readiness/argument-documentation-coverage.json",
        "generated/release-readiness/documentation-quality.json",
        "generated/slatec-routines/argument-documentation-coverage.json",
        "generated/slatec-routines/documentation-quality.json",
    ] {
        paths.push(json!({
            "path":path,
            "category":"generated inventories/indexes",
            "generator_owner":SEMANTIC_REVIEW_OWNER,
            "risk":"critical",
        }));
    }
    paths.sort_by_key(|path| field(path, "path"));
    Ok(paths)
}

fn drift_input_hashes(root: &Path) -> Result<BTreeMap<String, String>> {
    [
        "crates/slatec-tools/src/release_readiness.rs",
        "crates/slatec-tools/src/public_api_semantic_review.rs",
        "generated/ffi-inventory/argument-index.json",
        "generated/public-api/ffi-declaration-ownership.json",
        "generated/raw-api/final-disposition.json",
        "generated/raw-api/routine-status.json",
        "generated/slatec-routines/routine-catalogue.json",
        "metadata/public-api-semantic-corrections.json",
        "metadata/release-readiness-documentation.json",
    ]
    .into_iter()
    .map(|relative| Ok((relative.to_owned(), hash::file(&root.join(relative))?)))
    .collect()
}

fn output_to_input_ownership() -> Vec<Value> {
    vec![
        json!({
            "output_pattern":"docs/reference/routines/<canonical-public>.md",
            "generator_owner":SEMANTIC_REVIEW_OWNER,
            "previous_overlapping_writer":"release_readiness::write_routine_docs",
            "authored_inputs":["metadata/public-api-semantic-corrections.json"],
            "intermediate_inputs":["generated/slatec-routines/routine-catalogue.json","generated/raw-api/routine-status.json","generated/raw-api/final-disposition.json","generated/public-api/ffi-declaration-ownership.json","generated/release-readiness/argument-documentation-coverage.json"],
            "schema_version":"semantic page contract 3.0",
            "sorting_policy":"canonical routine name",
            "resolution":"semantic review is sole owner"
        }),
        json!({
            "output_pattern":"generated/{release-readiness,slatec-routines}/{documentation-quality,argument-documentation-coverage}.json",
            "generator_owner":SEMANTIC_REVIEW_OWNER,
            "previous_overlapping_writer":"release_readiness::generate/write_scoped_outputs",
            "authored_inputs":["metadata/public-api-semantic-corrections.json"],
            "intermediate_inputs":["generated/slatec-routines/routine-catalogue.json","generated/raw-api/routine-status.json","generated/raw-api/final-disposition.json","generated/public-api/ffi-declaration-ownership.json"],
            "schema_version":"2.0.0",
            "sorting_policy":"BTreeMap routine identity and argument order from authoritative ABI metadata",
            "resolution":"semantic review is sole owner"
        }),
        json!({
            "output_pattern":"docs/reference/routines/<secondary>.md, docs/reference/families/**, generated/release-readiness/** excluding semantic mirrors",
            "generator_owner":RELEASE_READINESS_OWNER,
            "authored_inputs":["metadata/release-readiness-documentation.json"],
            "intermediate_inputs":["generated/slatec-routines/routine-catalogue.json","generated/raw-api/final-disposition.json","generated/raw-api/routine-status.json","generated/ffi-inventory/argument-index.json","generated/public-api/ffi-declaration-ownership.json"],
            "schema_version":"release-readiness schema bundle 1.0.0",
            "sorting_policy":"BTreeMap family and routine identity ordering",
            "resolution":"release-readiness remains sole owner"
        }),
    ]
}

fn drift_report_markdown(report: &Value) -> String {
    let historical = &report["historical_reproduction"];
    let current = &report["current_transactional_recomputation"];
    let mut markdown = format!(
        "# Generator-drift analysis\n\nThe prior 825-file release blocker was a deterministic **generator regression**, not a bulk regeneration to accept. The current transactional recomputation is `{}` with **{}** changed files and **{}** ownerless files.\n\n## Reproduction\n\n- Base commit: `{}`\n- Clean recomputations: **{}**, byte-identical: **{}**\n- Patch SHA-256: `{}`\n- Source snapshot: `{}`\n\n## Historical partition\n\n| Category | Files | Added lines | Removed lines | Owner | Risk |\n| --- | ---: | ---: | ---: | --- | --- |\n",
        current["result"].as_str().unwrap_or("unknown"),
        current["changed_file_count"].as_u64().unwrap_or_default(),
        current["missing_ownership_count"]
            .as_u64()
            .unwrap_or_default(),
        historical["base_commit"].as_str().unwrap_or("unknown"),
        historical["clean_recomputation_count"]
            .as_u64()
            .unwrap_or_default(),
        historical["byte_identical_clean_recomputations"]
            .as_bool()
            .unwrap_or(false),
        historical["identical_patch_sha256"]
            .as_str()
            .unwrap_or("unknown"),
        report["source_snapshot"].as_str().unwrap_or("unknown"),
    );
    for summary in historical["category_summaries"]
        .as_array()
        .into_iter()
        .flatten()
    {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | `{}` | `{}` |\n",
            summary["category"].as_str().unwrap_or("unknown"),
            summary["changed_file_count"].as_u64().unwrap_or_default(),
            summary["added_lines"].as_u64().unwrap_or_default(),
            summary["removed_lines"].as_u64().unwrap_or_default(),
            summary["authoritative_owner"].as_str().unwrap_or("unknown"),
            summary["risk"].as_str().unwrap_or("unknown"),
        ));
    }
    markdown.push_str("\n## Resolution\n\nCanonical-public routine pages and both semantic inventory mirrors are now written only by `public_api_semantic_review`. Release readiness still writes secondary pages, family navigation, cross-checks, and its own reconciliation evidence. The transactional validator reports owner, category, risk, line counts, EOL-only changes, ordering-only changes, schema changes, and canonical/safe-path changes; it does not auto-accept any of them.\n\n## Representative semantic protections\n\n");
    for change in historical["representative_semantic_changes"]
        .as_array()
        .into_iter()
        .flatten()
    {
        markdown.push_str(&format!(
            "- `{}` — {}. {}\n",
            change["path"].as_str().unwrap_or("unknown"),
            change["classification"].as_str().unwrap_or("unknown"),
            change["resolution"].as_str().unwrap_or("unknown"),
        ));
    }
    markdown.push_str("\nThe JSON companion enumerates all 825 historical paths, exact input hashes, output-to-input ownership, environment requirements, and the current transactional result.\n");
    markdown
}

fn generate_transactionally(
    root: &Path,
    output_dir: &Path,
) -> Result<(ReleaseReadinessResult, ReleaseReadinessFreshness)> {
    let snapshot = ReadinessSnapshot::capture(root)?;
    let generated = generate(root, output_dir);
    let freshness = snapshot.freshness(root)?;
    snapshot.restore(root)?;
    let result = generated?;
    Ok((result, freshness))
}

impl ReadinessSnapshot {
    fn capture(root: &Path) -> Result<Self> {
        let mut files = BTreeMap::new();
        for relative in readiness_output_roots() {
            collect_snapshot_files(root, &relative, &mut files)?;
        }
        Ok(Self { files })
    }

    fn freshness(&self, root: &Path) -> Result<ReleaseReadinessFreshness> {
        let current = Self::capture(root)?;
        let mut paths = BTreeSet::new();
        paths.extend(self.files.keys().cloned());
        paths.extend(current.files.keys().cloned());
        let public_routines = canonical_public_routines(root)?;
        let changed_paths = paths
            .into_iter()
            .filter(|path| self.files.get(path) != current.files.get(path))
            .collect::<Vec<_>>();
        let changes = changed_paths
            .iter()
            .map(|path| {
                classify_freshness_change(
                    path,
                    self.files.get(path).map(Vec::as_slice),
                    current.files.get(path).map(Vec::as_slice),
                    &public_routines,
                )
            })
            .collect::<Vec<_>>();
        let missing_ownership_count = changes
            .iter()
            .filter(|change| change.ownership_status != "declared")
            .count();
        Ok(ReleaseReadinessFreshness {
            changed_paths,
            changes,
            missing_ownership_count,
        })
    }

    fn restore(&self, root: &Path) -> Result<()> {
        let current = Self::capture(root)?;
        for path in current
            .files
            .keys()
            .filter(|path| !self.files.contains_key(*path))
        {
            fs::remove_file(root.join(path))?;
        }
        for (path, bytes) in &self.files {
            let output = root.join(path);
            if fs::read(&output).ok().as_deref() != Some(bytes.as_slice()) {
                if let Some(parent) = output.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(output, bytes)?;
            }
        }
        Ok(())
    }
}

fn canonical_public_routines(root: &Path) -> Result<BTreeSet<String>> {
    let path = root.join("generated/raw-api/final-disposition.json");
    if !path.is_file() {
        return Ok(BTreeSet::new());
    }
    Ok(records(&read_json(&path)?, "final disposition")?
        .iter()
        .filter(|record| field(record, "final_disposition") == "canonical-public")
        .map(|record| field(record, "routine"))
        .collect())
}

fn classify_freshness_change(
    path: &Path,
    before: Option<&[u8]>,
    after: Option<&[u8]>,
    public_routines: &BTreeSet<String>,
) -> FreshnessChange {
    let normalized_path = path.to_string_lossy().replace('\\', "/");
    let owner = output_owner(&normalized_path, public_routines);
    let eol_only = before.zip(after).is_some_and(|(before, after)| {
        before != after && normalized_eol(before) == normalized_eol(after)
    });
    let ordering_only = before.zip(after).is_some_and(|(before, after)| {
        before != after
            && matches!(
                (ordering_normalized_json(before), ordering_normalized_json(after)),
                (Some(before), Some(after)) if before == after
            )
    });
    let schema_changed = before
        .zip(after)
        .is_some_and(|(before, after)| json_schema(before) != json_schema(after));
    let safe_path_changed = before
        .zip(after)
        .is_some_and(|(before, after)| documented_paths(before) != documented_paths(after));
    let (category, change_kind, risk) = if eol_only {
        ("formatting/whitespace/EOL", "formatting-only", "low")
    } else if ordering_only {
        ("ordering", "ordering-only", "medium")
    } else if schema_changed {
        ("generated inventories/indexes", "semantic", "critical")
    } else if safe_path_changed {
        ("safe-path/status classification", "semantic", "critical")
    } else if let Some(owner) = owner {
        (owner.category, "semantic", owner.risk)
    } else {
        ("other", "semantic", "critical")
    };
    let owner_name = owner
        .map(|owner| owner.generator_owner)
        .unwrap_or("unrecognized-generator-output");
    let before_lines = line_count(before.unwrap_or_default());
    let after_lines = line_count(after.unwrap_or_default());
    FreshnessChange {
        path: normalized_path,
        category: category.to_owned(),
        generator_owner: owner_name.to_owned(),
        change_kind: change_kind.to_owned(),
        risk: risk.to_owned(),
        before_lines,
        after_lines,
        line_delta: after_lines as isize - before_lines as isize,
        eol_only,
        ordering_only,
        schema_changed,
        safe_path_changed,
        ownership_status: if owner.is_some() {
            "declared".to_owned()
        } else {
            "missing".to_owned()
        },
    }
}

fn output_owner(path: &str, public_routines: &BTreeSet<String>) -> Option<OutputOwner> {
    if let Some(routine) = path
        .strip_prefix("docs/reference/routines/")
        .and_then(|path| path.strip_suffix(".md"))
    {
        return Some(if public_routines.contains(&routine.to_ascii_uppercase()) {
            OutputOwner {
                generator_owner: SEMANTIC_REVIEW_OWNER,
                category: "routine semantic content",
                risk: "critical",
            }
        } else {
            OutputOwner {
                generator_owner: "release_readiness::write_secondary_routine_docs",
                category: "routine metadata/header",
                risk: "high",
            }
        });
    }
    if matches!(
        path,
        "generated/release-readiness/documentation-quality.json"
            | "generated/release-readiness/argument-documentation-coverage.json"
            | "generated/slatec-routines/documentation-quality.json"
            | "generated/slatec-routines/argument-documentation-coverage.json"
            | "generated/slatec-routines/documentation-quality-summary.md"
    ) {
        return Some(OutputOwner {
            generator_owner: SEMANTIC_REVIEW_OWNER,
            category: "generated inventories/indexes",
            risk: "critical",
        });
    }
    if path.starts_with("docs/reference/families/")
        || path == "docs/reference/routines-by-family.md"
        || matches!(
            path,
            "generated/release-readiness/catalogue-sys-crosscheck.json"
                | "generated/release-readiness/documentation-export-gaps.json"
                | "generated/release-readiness/documentation-repair-candidates.json"
                | "generated/release-readiness/family-page-index.json"
                | "generated/release-readiness/family-page-summary.json"
                | "generated/release-readiness/module-taxonomy-validation.json"
                | "generated/release-readiness/release-readiness-summary.md"
                | "generated/release-readiness/canonical-path-migrations.json"
                | "generated/release-readiness/generator-drift-analysis.json"
                | "generated/release-readiness/generator-drift-analysis.md"
        )
        || matches!(
            path,
            "generated/slatec-routines/documentation-repair-candidates.json"
                | "generated/slatec-routines/family-page-index.json"
                | "generated/slatec-routines/family-page-summary.json"
                | "generated/public-api/catalogue-sys-crosscheck.json"
                | "generated/public-api/documentation-export-gaps.json"
                | "generated/public-api/module-taxonomy-validation.json"
                | "generated/public-api/catalogue-sys-crosscheck-summary.md"
                | "generated/public-api/canonical-path-migrations.json"
        )
    {
        return Some(OutputOwner {
            generator_owner: RELEASE_READINESS_OWNER,
            category: "release-readiness evidence",
            risk: "high",
        });
    }
    None
}

fn normalized_eol(bytes: &[u8]) -> Vec<u8> {
    let mut normalized = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
            index += 1;
        }
        normalized.push(bytes[index]);
        index += 1;
    }
    normalized
}

fn ordering_normalized_json(bytes: &[u8]) -> Option<Value> {
    let value = serde_json::from_slice::<Value>(bytes).ok()?;
    Some(normalize_record_order(value))
}

fn normalize_record_order(value: Value) -> Value {
    match value {
        Value::Array(values) => {
            let mut values = values
                .into_iter()
                .map(normalize_record_order)
                .collect::<Vec<_>>();
            if values
                .iter()
                .all(|value| value.get("routine").and_then(Value::as_str).is_some())
            {
                values.sort_by_key(|value| field(value, "routine"));
            }
            Value::Array(values)
        }
        Value::Object(values) => Value::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, normalize_record_order(value)))
                .collect(),
        ),
        value => value,
    }
}

fn json_schema(bytes: &[u8]) -> Option<(String, String)> {
    let value = serde_json::from_slice::<Value>(bytes).ok()?;
    Some((field(&value, "schema_id"), field(&value, "schema_version")))
}

fn documented_paths(bytes: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(bytes);
    text.lines()
        .filter(|line| {
            line.contains("Canonical Rust path")
                || line.contains("Canonical path")
                || line.contains("canonical_rust_path")
                || line.contains("safe API path")
        })
        .map(str::to_owned)
        .collect()
}

fn line_count(bytes: &[u8]) -> usize {
    if bytes.is_empty() {
        0
    } else {
        bytes.iter().filter(|byte| **byte == b'\n').count() + usize::from(!bytes.ends_with(b"\n"))
    }
}

fn freshness_category_summary(changes: &[FreshnessChange]) -> String {
    let mut counts = BTreeMap::<&str, usize>::new();
    for change in changes {
        *counts.entry(&change.category).or_default() += 1;
    }
    counts
        .into_iter()
        .map(|(category, count)| format!("{category}={count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn freshness_owner_summary(changes: &[FreshnessChange]) -> String {
    let mut counts = BTreeMap::<&str, usize>::new();
    for change in changes {
        *counts.entry(&change.generator_owner).or_default() += 1;
    }
    counts
        .into_iter()
        .map(|(owner, count)| format!("{owner}={count}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn readiness_output_roots() -> Vec<PathBuf> {
    vec![
        PathBuf::from("docs/reference/routines"),
        PathBuf::from("docs/reference/families"),
        PathBuf::from("docs/reference/routines-by-family.md"),
        PathBuf::from("generated/release-readiness"),
        PathBuf::from("generated/slatec-routines/documentation-quality.json"),
        PathBuf::from("generated/slatec-routines/documentation-repair-candidates.json"),
        PathBuf::from("generated/slatec-routines/family-page-index.json"),
        PathBuf::from("generated/slatec-routines/family-page-summary.json"),
        PathBuf::from("generated/slatec-routines/argument-documentation-coverage.json"),
        PathBuf::from("generated/public-api/catalogue-sys-crosscheck.json"),
        PathBuf::from("generated/public-api/documentation-export-gaps.json"),
        PathBuf::from("generated/public-api/module-taxonomy-validation.json"),
        PathBuf::from("generated/public-api/catalogue-sys-crosscheck-summary.md"),
        PathBuf::from("generated/public-api/canonical-path-migrations.json"),
    ]
}

fn collect_snapshot_files(
    root: &Path,
    relative: &Path,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
) -> Result<()> {
    let path = root.join(relative);
    if !path.exists() {
        return Ok(());
    }
    if path.is_file() {
        files.insert(relative.to_path_buf(), fs::read(path)?);
        return Ok(());
    }
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let child = entry.path();
        let child_relative = child
            .strip_prefix(root)
            .map_err(|_| policy("release-readiness output escaped repository root"))?
            .to_path_buf();
        collect_snapshot_files(root, &child_relative, files)?;
    }
    Ok(())
}

fn evidence(
    root: &Path,
    docs: &[RoutineDoc],
    retained: usize,
    public: usize,
    ownership: &BTreeMap<String, &Value>,
) -> Result<BTreeMap<String, Value>> {
    let canonical = docs
        .iter()
        .filter(|record| record.disposition == "canonical-public")
        .collect::<Vec<_>>();
    let mut paths = BTreeSet::new();
    let mut duplicates = BTreeSet::new();
    for record in &canonical {
        if !paths.insert(record.canonical_path.clone()) {
            duplicates.insert(record.canonical_path.clone());
        }
    }
    let quality_records = docs
        .iter()
        .map(|record| {
            json!({
                "routine":record.routine,
                "family":record.family,
                "public_raw":record.disposition == "canonical-public",
                "quality_level":record.quality,
                "reason":record.quality_reason,
                "description_provenance":record.description_provenance,
                "argument_count":record.arguments.len(),
                "structured_argument_rows":record.arguments.len(),
                "mangled_reasons":record.mangled_reasons,
            })
        })
        .collect::<Vec<_>>();
    let mut quality_counts = BTreeMap::<String, usize>::new();
    for record in docs {
        *quality_counts.entry(record.quality.clone()).or_default() += 1;
    }
    let forbidden_namespace_paths = canonical
        .iter()
        .filter(|record| {
            record.canonical_path.contains("::numerical::")
                || record.canonical_path.starts_with("slatec_sys::eigen::")
        })
        .count();
    let missing = canonical
        .iter()
        .filter(|record| record.canonical_path == "not_promoted")
        .map(|record| record.routine.clone())
        .collect::<Vec<_>>();
    let argument_routines = docs
        .iter()
        .map(|record| {
            let complete_semantics = record.arguments.iter().filter(|argument| {
                argument.direction != "unavailable"
                    && !argument.semantic_description.starts_with("No separable")
            }).count();
            json!({
                "routine":record.routine,
                "public_raw":record.disposition == "canonical-public",
                "argument_count":record.arguments.len(),
                "structured_rows":record.arguments.len(),
                "arguments_with_separable_semantics":complete_semantics,
                "arguments_with_explicit_unknown_semantics":record.arguments.len() - complete_semantics,
                "semantic_intent_policy":"reported only when separable source or authored evidence exists",
                "unknown_semantics_are_explicit":true,
            })
        })
        .collect::<Vec<_>>();
    let argument_records = docs
        .iter()
        .flat_map(|record| {
            record.arguments.iter().map(|argument| {
                json!({
                    "routine":record.routine,
                    "public_raw":record.disposition == "canonical-public",
                    "name":argument.name,
                    "fortran_type":argument.declared_type,
                    "rust_raw_type":argument.rust_type,
                    "shape":argument.dimensions,
                    "direction":argument.direction,
                    "semantic_description":argument.semantic_description,
                    "relationship":argument.relationship,
                    "leading_dimension":argument.leading_dimension,
                    "workspace_requirement":argument.workspace_requirement,
                    "nullable":argument.nullable,
                    "external_callback":argument.external,
                    "type_provenance":argument.type_source,
                })
            })
        })
        .collect::<Vec<_>>();
    let alphabetical = fs::read_to_string(root.join("docs/reference/routines-alphabetical.md"))?;
    let mut crosscheck_records = Vec::with_capacity(docs.len());
    let mut gaps = Vec::new();
    for record in docs {
        let public_raw = record.disposition == "canonical-public";
        let owner = ownership.get(&record.routine).copied();
        let actual_export_found = owner.is_some();
        let export_matches = owner.is_some_and(|owner| {
            field(owner, "canonical_public_path") == record.canonical_path
                && field(owner, "native_symbol") == record.native_symbol
                && field(owner, "abi_fingerprint") == record.abi_fingerprint
                && owner
                    .get("extern_declaration_count")
                    .and_then(Value::as_u64)
                    == Some(1)
        });
        let document_page_found = root
            .join("docs/reference/routines")
            .join(format!("{}.md", record.routine.to_ascii_lowercase()))
            .is_file();
        let family_page_found = root
            .join("docs/reference/families")
            .join(format!("{}.md", record.slug))
            .is_file();
        let alphabetical_index_found = alphabetical.contains(&format!(
            "routines/{}.md",
            record.routine.to_ascii_lowercase()
        ));
        let meaningful_description =
            !record.purpose.trim().is_empty() && !record.description.trim().is_empty();
        let structured_arguments = record.arguments.iter().all(|argument| {
            !argument.name.is_empty()
                && !argument.declared_type.is_empty()
                && !argument.rust_type.is_empty()
                && !argument.dimensions.is_empty()
                && !argument.direction.is_empty()
                && !argument.semantic_description.is_empty()
        });
        let consistent = if public_raw {
            actual_export_found
                && export_matches
                && document_page_found
                && family_page_found
                && alphabetical_index_found
                && meaningful_description
                && structured_arguments
                && !record.feature.is_empty()
                && !record.provider_status.is_empty()
                && !record.precision.is_empty()
        } else {
            !actual_export_found && record.canonical_path == "not_promoted"
        };
        let crosscheck = json!({
            "routine":record.routine,
            "historical_role":record.role,
            "final_disposition":record.disposition,
            "canonical_rust_path":record.canonical_path,
            "actual_export_found":actual_export_found,
            "authoritative_export_matches":export_matches,
            "document_page_found":document_page_found,
            "family_page_found":family_page_found,
            "alphabetical_index_found":alphabetical_index_found,
            "feature":record.feature,
            "provider_status":record.provider_status,
            "native_symbol":record.native_symbol,
            "abi_fingerprint":record.abi_fingerprint,
            "precision":record.precision,
            "documentation_quality":record.quality,
            "meaningful_description":meaningful_description,
            "structured_arguments":structured_arguments,
            "consistency_status":if consistent { "consistent" } else { "inconsistent" },
        });
        if !consistent {
            gaps.push(crosscheck.clone());
        }
        crosscheck_records.push(crosscheck);
    }
    let public_argument_total = canonical
        .iter()
        .map(|record| record.arguments.len())
        .sum::<usize>();
    let public_argument_semantics = canonical
        .iter()
        .flat_map(|record| &record.arguments)
        .filter(|argument| {
            argument.direction != "unavailable"
                && !argument.semantic_description.starts_with("No separable")
        })
        .count();
    let repair_candidates = docs
        .iter()
        .filter(|record| !record.mangled_reasons.is_empty())
        .map(|record| json!({
            "routine":record.routine,
            "public_raw":record.disposition == "canonical-public",
            "reasons":record.mangled_reasons,
            "quality_level":record.quality,
            "recommended_action":"review selected source prose or add a source-hash-guarded authored correction",
        }))
        .collect::<Vec<_>>();
    let repair_candidate_flags = docs
        .iter()
        .map(|record| record.mangled_reasons.len())
        .sum::<usize>();
    let mut family_records = BTreeMap::<String, (usize, usize)>::new();
    for record in docs {
        let counts = family_records.entry(record.family.clone()).or_default();
        counts.1 += 1;
        if record.disposition == "canonical-public" {
            counts.0 += 1;
        }
    }
    let family_index = family_records
        .iter()
        .map(|(family, (public_count, retained_count))| {
            json!({
                "family":family,
                "slug":slug(family),
                "public_routines":public_count,
                "retained_identities":retained_count,
                "secondary_identities":retained_count - public_count,
            })
        })
        .collect::<Vec<_>>();
    let public_family_sum = family_records
        .values()
        .map(|counts| counts.0)
        .sum::<usize>();
    let secondary_family_sum = family_records
        .values()
        .map(|counts| counts.1 - counts.0)
        .sum::<usize>();
    let result = BTreeMap::from([
        (
            "catalogue-sys-crosscheck.json".to_owned(),
            json!({
                "schema_id":"slatec-rs.catalogue-sys-crosscheck",
                "schema_version":"1.0.0",
                "retained_identities":retained,
                "final_disposition_identities":docs.len(),
                "public_raw_identities":public,
                "catalogue_only_missing_from_disposition":[],
                "duplicate_routine_identities":[],
                "inconsistencies":gaps.len(),
                "records":crosscheck_records,
                "result":if gaps.is_empty() { "pass" } else { "fail" },
            }),
        ),
        (
            "documentation-export-gaps.json".to_owned(),
            json!({
                "schema_id":"slatec-rs.documentation-export-gaps",
                "schema_version":"1.0.0",
                "gap_count":gaps.len(),
                "records":gaps,
            }),
        ),
        (
            "documentation-quality.json".to_owned(),
            json!({
                "schema_id":"slatec-rs.documentation-quality",
                "schema_version":"1.0.0",
                "quality_levels":{
                    "complete_structured":"meaningful description and separable semantic evidence for every argument",
                    "complete_unstructured":"meaningful selected-source prose for a non-public identity",
                    "purpose_only":"only a verified concise purpose is available",
                    "argument_contract_incomplete":"structured ABI rows exist but some source-level argument semantics remain explicit unknowns",
                    "mangled_source_prologue":"selected prose has a mechanically detected repair candidate",
                    "subsidiary_minimal":"minimum non-public subsidiary documentation",
                    "support_unit_minimal":"minimum runtime, error, or machine-support documentation",
                    "unavailable":"no trustworthy descriptive prose is available"
                },
                "mangled_candidate_flags_before":35,
                "mangled_candidate_flags_after":repair_candidate_flags,
                "mangled_candidate_identities_after":repair_candidates.len(),
                "counts":quality_counts,
                "records":quality_records,
            }),
        ),
        (
            "documentation-repair-candidates.json".to_owned(),
            json!({
                "schema_id":"slatec-rs.documentation-repair-candidates",
                "schema_version":"1.0.0",
                "records":repair_candidates,
            }),
        ),
        (
            "argument-documentation-coverage.json".to_owned(),
            json!({
                "schema_id":"slatec-rs.argument-documentation-coverage",
                "schema_version":"1.0.0",
                "policy":"Types and dimensions come from the selected interface inventory. Intent, aliasing, nullability exceptions, and retention are never inferred from parameter names.",
                "public_argument_rows":public_argument_total,
                "public_arguments_with_separable_semantics":public_argument_semantics,
                "public_arguments_with_explicit_unknown_semantics":public_argument_total - public_argument_semantics,
                "routine_records":argument_routines,
                "argument_records":argument_records,
            }),
        ),
        (
            "family-page-index.json".to_owned(),
            json!({
                "schema_id":"slatec-rs.family-page-index",
                "schema_version":"1.0.0",
                "family_pages":family_index.len(),
                "public_routine_sum":public_family_sum,
                "secondary_identity_sum":secondary_family_sum,
                "retained_identity_sum":public_family_sum + secondary_family_sum,
                "records":family_index,
            }),
        ),
        (
            "family-page-summary.json".to_owned(),
            json!({
                "schema_id":"slatec-rs.family-page-summary",
                "schema_version":"1.0.0",
                "family_pages":family_records.len(),
                "public_routines":public_family_sum,
                "secondary_identities":secondary_family_sum,
                "retained_identities":public_family_sum + secondary_family_sum,
            }),
        ),
        (
            "module-taxonomy-validation.json".to_owned(),
            json!({
                "schema_id":"slatec-rs.module-taxonomy-validation",
                "schema_version":"1.0.0",
                "canonical_public_paths":canonical.len(),
                "forbidden_canonical_namespace_paths":forbidden_namespace_paths,
                "duplicate_canonical_paths":duplicates.len(),
                "duplicate_paths":duplicates,
                "missing_canonical_paths":missing.len(),
                "missing_routines":missing,
            }),
        ),
    ]);
    Ok(result)
}

fn write_family_docs(root: &Path, docs: &[RoutineDoc]) -> Result<usize> {
    let mut families = BTreeMap::<String, Vec<&RoutineDoc>>::new();
    for record in docs {
        families
            .entry(record.family.clone())
            .or_default()
            .push(record);
    }
    let family_dir = root.join("docs/reference/families");
    fs::create_dir_all(&family_dir)?;
    let mut index = String::from(
        "# SLATEC routines by mathematical family\n\nThis branching overview covers the complete retained catalogue. Each dedicated family page presents canonical public raw routines first and then accounts for internal, support, historical, and excluded identities.\n\n| Family | Description | Public routines | Total identities |\n| --- | --- | ---: | ---: |\n",
    );
    for (family, records) in &mut families {
        records.sort_by(|left, right| {
            let left_public = left.disposition == "canonical-public";
            let right_public = right.disposition == "canonical-public";
            right_public
                .cmp(&left_public)
                .then_with(|| left.routine.cmp(&right.routine))
        });
        let public = records
            .iter()
            .filter(|record| record.disposition == "canonical-public")
            .count();
        let family_slug = slug(family);
        index.push_str(&format!(
            "| [{}](families/{family_slug}.md) | {} | {public} | {} |\n",
            escape_table(family),
            escape_table(&family_description(family)),
            records.len(),
        ));
        let mut page = format!(
            "# {family}\n\n[All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)\n\n{}\n\nRetained identities: **{}**. Canonical public raw routines: **{public}**. Secondary or terminally disposed identities: **{}**.\n\n## Public routines\n\nPublic routines are sorted by canonical SLATEC name.\n\n<table>\n<thead><tr><th>Routine</th><th>Purpose</th><th>Role</th><th>Precision</th><th>Storage/problem class</th><th>Operation</th><th>Raw API status</th><th>Safe API status</th><th>Canonical Rust path</th></tr></thead>\n<tbody>\n",
            family_description(family),
            records.len(),
            records.len() - public
        );
        for record in records
            .iter()
            .filter(|record| record.disposition == "canonical-public")
        {
            page.push_str(&format!(
                "<tr><td><a href=\"../routines/{}.md\"><code>{}</code></a></td><td>{}</td><td><code>{}</code></td><td><code>{}</code></td><td><code>{}</code></td><td><code>{}</code></td><td><code>canonical-public</code></td><td><code>{}</code></td><td><code>{}</code></td></tr>\n",
                record.routine.to_ascii_lowercase(),
                html(&record.routine),
                html(&record.purpose),
                html(&record.role),
                html(&record.precision),
                html(&record.storage_class),
                html(&record.operation_class),
                html(&record.safe_status),
                html(&record.canonical_path),
            ));
        }
        page.push_str("</tbody>\n</table>\n\n## Internal, support, and historical identities\n\nThese records remain part of the retained catalogue but are not additional public raw routines.\n\n<table>\n<thead><tr><th>Routine</th><th>Purpose</th><th>Role</th><th>Precision</th><th>Storage/problem class</th><th>Operation</th><th>Final disposition</th><th>Documentation quality</th></tr></thead>\n<tbody>\n");
        for record in records
            .iter()
            .filter(|record| record.disposition != "canonical-public")
        {
            page.push_str(&format!(
                "<tr class=\"routine-secondary\" style=\"opacity:0.76\"><td><a href=\"../routines/{}.md\"><code>{}</code></a></td><td>{}</td><td><code>{}</code></td><td><code>{}</code></td><td><code>{}</code></td><td><code>{}</code></td><td><code>{}</code></td><td><code>{}</code></td></tr>\n",
                record.routine.to_ascii_lowercase(),
                html(&record.routine),
                html(&record.purpose),
                html(&record.role),
                html(&record.precision),
                html(&record.storage_class),
                html(&record.operation_class),
                html(&record.disposition),
                html(&record.quality),
            ));
        }
        page.push_str("</tbody>\n</table>\n");
        write_if_changed(
            &family_dir.join(format!("{family_slug}.md")),
            page.as_bytes(),
        )?;
    }
    write_if_changed(
        &root.join("docs/reference/routines-by-family.md"),
        index.as_bytes(),
    )?;
    Ok(families.len())
}

fn write_secondary_routine_docs(root: &Path, docs: &[RoutineDoc]) -> Result<()> {
    let routine_dir = root.join("docs/reference/routines");
    for record in docs
        .iter()
        .filter(|record| record.disposition != "canonical-public")
    {
        let path = routine_dir.join(format!("{}.md", record.routine.to_ascii_lowercase()));
        let mut source = fs::read_to_string(&path)
            .map_err(|_| policy(&format!("routine page is missing for {}", record.routine)))?;
        source = replace_navigation(&source, record);
        let block = routine_block(record);
        source = replace_block(&source, &block);
        write_if_changed(&path, source.as_bytes())?;
    }
    Ok(())
}

fn routine_block(record: &RoutineDoc) -> String {
    let mut block = format!(
        "{START}\n## Interface documentation quality\n\n- Evidence level: `{}`\n- Description provenance: `{}`\n- Assessment: {}\n- Dedicated family page: [{}](../families/{}.md)\n",
        record.quality,
        record.description_provenance,
        record.quality_reason,
        record.family,
        record.slug,
    );
    if record.arguments.is_empty() {
        block.push_str("\nNo independently callable argument list is present in the selected interface inventory for this identity.\n");
    } else {
        block.push_str("\n### Arguments\n\n| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for argument in &record.arguments {
            block.push_str(&format!(
                "| `{}` | {} | `{}` (`{}`) | `{}` | {} | {} | {} Leading dimension: {} Workspace: {} | {} |\n",
                argument.name,
                argument.direction,
                argument.declared_type,
                argument.type_source,
                argument.rust_type,
                escape_table(&argument.dimensions),
                escape_table(&argument.semantic_description),
                escape_table(&argument.relationship),
                escape_table(&argument.leading_dimension),
                escape_table(&argument.workspace_requirement),
                argument.nullable,
            ));
        }
        block.push_str("\nThe table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.\n");
    }
    if let Some(return_type) = &record.return_type {
        block.push_str(&format!(
            "\n### Return value\n\nThe Fortran function returns `{return_type}` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `{}`.\n",
            record.abi_fingerprint
        ));
    }
    if record.arguments.iter().any(|argument| argument.external) {
        block.push_str("\n### Callback contract\n\nProcedure arguments use the exact reviewed `unsafe extern \"C\"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.\n");
    }
    if record.disposition == "canonical-public" {
        block.push_str(&format!(
            "\n### ABI and safety\n\nCanonical path: `{}`. Native symbol: `{}`. Feature: `{}`. Provider status: `{}`. ABI fingerprint: `{}`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.\n",
            record.canonical_path,
            record.native_symbol,
            record.feature,
            record.provider_status,
            record.abi_fingerprint,
        ));
    }
    block.push_str(END);
    block
}

fn replace_navigation(source: &str, record: &RoutineDoc) -> String {
    let mut lines = source.lines().map(str::to_owned).collect::<Vec<_>>();
    if lines.len() > 2 {
        lines[2] = format!(
            "[Family: {}](../families/{}.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)",
            record.family, record.slug
        );
    }
    let mut rendered = lines.join("\n");
    rendered.push('\n');
    rendered
}

fn replace_block(source: &str, block: &str) -> String {
    if let (Some(start), Some(end)) = (source.find(START), source.find(END)) {
        let tail = end + END.len();
        format!("{}{}{}", &source[..start], block, &source[tail..])
    } else if let Some(raw) = source.find("<!-- raw-api-status:start -->") {
        format!("{}{}\n\n{}", &source[..raw], block, &source[raw..])
    } else {
        format!("{}\n{}\n", source.trim_end(), block)
    }
}

fn arguments_by_unit(value: &Value) -> Result<BTreeMap<String, Vec<Argument>>> {
    let rows = value
        .get("records")
        .and_then(Value::as_array)
        .ok_or_else(|| policy("argument inventory has no records"))?;
    let mut result = BTreeMap::<String, Vec<Argument>>::new();
    for row in rows {
        let row = row
            .as_array()
            .ok_or_else(|| policy("argument inventory row is not an array"))?;
        if row.len() < 10 {
            return Err(policy("argument inventory row is truncated"));
        }
        let unit = row[1].as_str().unwrap_or_default().to_owned();
        let dimensions = render_dimensions(&row[7]);
        result.entry(unit).or_default().push(Argument {
            name: row[3].as_str().unwrap_or("UNKNOWN").to_owned(),
            declared_type: row[4].as_str().unwrap_or("UNKNOWN").to_owned(),
            rust_type: rust_raw_type(
                row[4].as_str().unwrap_or("UNKNOWN"),
                row[8].as_bool().unwrap_or(false),
            ),
            dimensions,
            type_source: row[5].as_str().unwrap_or("unknown").to_owned(),
            external: row[8].as_bool().unwrap_or(false),
            direction: "unavailable".to_owned(),
            semantic_description:
                "No separable argument description was found in the selected source prologue."
                    .to_owned(),
            relationship: "unavailable".to_owned(),
            leading_dimension: "not established".to_owned(),
            workspace_requirement: "not established".to_owned(),
            nullable: "required; null is not permitted for an ordinary Fortran actual argument"
                .to_owned(),
        });
    }
    Ok(result)
}

fn function_results_by_unit(value: &Value) -> Result<BTreeMap<String, String>> {
    let rows = value
        .get("records")
        .and_then(Value::as_array)
        .ok_or_else(|| policy("function-result inventory has no records"))?;
    let mut results = BTreeMap::new();
    for row in rows {
        let row = row
            .as_array()
            .ok_or_else(|| policy("function-result row is not an array"))?;
        if row.len() < 3 {
            return Err(policy("function-result row is truncated"));
        }
        results.insert(
            row[1].as_str().unwrap_or_default().to_owned(),
            rust_raw_type(row[2].as_str().unwrap_or("UNKNOWN"), false),
        );
    }
    Ok(results)
}

fn rust_raw_type(declared_type: &str, external: bool) -> String {
    if external {
        return "reviewed unsafe extern callback function pointer".to_owned();
    }
    match declared_type {
        "REAL" => "*mut f32",
        "DOUBLE PRECISION" => "*mut f64",
        "INTEGER" => "*mut crate::FortranInteger",
        "LOGICAL" => "*mut crate::FortranLogical",
        "COMPLEX" => "*mut crate::Complex32",
        "DOUBLE COMPLEX" => "*mut crate::Complex64",
        "CHARACTER" => "*mut core::ffi::c_char",
        other => other,
    }
    .to_owned()
}

fn documentation_quality(
    public: bool,
    disposition: &str,
    role: &str,
    description: &str,
    purpose: &str,
    arguments: &[Argument],
    mangled_reasons: &[String],
) -> (&'static str, &'static str) {
    if !mangled_reasons.is_empty() {
        return (
            "mangled_source_prologue",
            "mechanical source-prologue checks found text that requires a documented repair or review",
        );
    }
    if public {
        if description.is_empty() {
            return (
                "purpose_only",
                "a verified purpose exists, but no meaningful full description is available",
            );
        }
        if arguments.iter().all(|argument| {
            argument.direction != "unavailable"
                && !argument.semantic_description.starts_with("No separable")
        }) {
            return (
                "complete_structured",
                "the selected source supplies a meaningful description and separable evidence for every argument",
            );
        }
        return (
            "argument_contract_incomplete",
            "the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence",
        );
    }
    if matches!(disposition, "provider-subsidiary" | "raw-internal") || role == "subsidiary" {
        return (
            "subsidiary_minimal",
            "the non-public subsidiary has purpose, role, source, and disposition evidence",
        );
    }
    if disposition.contains("support") {
        return (
            "support_unit_minimal",
            "the support identity records its role, side-effect boundary, and non-public disposition",
        );
    }
    if !description.is_empty() {
        (
            "complete_unstructured",
            "meaningful selected-source prose is available for this non-public identity",
        )
    } else if !purpose.is_empty() {
        (
            "purpose_only",
            "only a verified concise purpose is available",
        )
    } else {
        (
            "unavailable",
            "no trustworthy descriptive text is available",
        )
    }
}

fn mangled_descriptions(value: &Value) -> Result<BTreeMap<String, Vec<String>>> {
    let mut result = BTreeMap::<String, Vec<String>>::new();
    for record in records(value, "description discrepancies")? {
        let routine = field(record, "identity");
        for reason in strings(record, "mangled_candidates") {
            let reasons = result.entry(routine.clone()).or_default();
            if !reasons.contains(&reason) {
                reasons.push(reason);
            }
        }
    }
    for reasons in result.values_mut() {
        reasons.sort();
    }
    result.retain(|_, reasons| !reasons.is_empty());
    Ok(result)
}

fn enrich_arguments(arguments: &mut [Argument], description: &str) {
    let upper = description.to_ascii_uppercase();
    let entry = upper.find("ON ENTRY");
    let returned = upper.find("ON RETURN");
    for argument in arguments {
        if argument.external {
            argument.direction = "callback".to_owned();
        }
        let occurrences = word_occurrences(&upper, &argument.name.to_ascii_uppercase());
        let on_entry = entry.is_some_and(|start| {
            occurrences
                .iter()
                .any(|position| *position >= start && returned.is_none_or(|end| *position < end))
        });
        let on_return =
            returned.is_some_and(|start| occurrences.iter().any(|position| *position >= start));
        if !argument.external {
            argument.direction = match (on_entry, on_return) {
                (true, true) => "input/output",
                (true, false) => "input",
                (false, true) => "output",
                (false, false) => direction_from_local_marker(&upper, &occurrences),
            }
            .to_owned();
        }
        if let Some(position) = occurrences.first().copied() {
            let sentence = source_sentence(description, position);
            if sentence.len() >= argument.name.len() + 8 {
                argument.semantic_description = sentence.clone();
                argument.relationship = relationship_evidence(&sentence);
                let lower = sentence.to_ascii_lowercase();
                if lower.contains("leading dimension") {
                    argument.leading_dimension = sentence.clone();
                }
                if lower.contains("workspace") || lower.contains("work array") {
                    argument.workspace_requirement = sentence;
                    if argument.direction == "unavailable" {
                        argument.direction = "workspace".to_owned();
                    }
                }
            }
        }
    }
}

fn word_occurrences(haystack: &str, needle: &str) -> Vec<usize> {
    haystack
        .match_indices(needle)
        .filter_map(|(position, _)| {
            let before = haystack[..position].chars().next_back();
            let after = haystack[position + needle.len()..].chars().next();
            (before.is_none_or(|character| !character.is_ascii_alphanumeric())
                && after.is_none_or(|character| !character.is_ascii_alphanumeric()))
            .then_some(position)
        })
        .collect()
}

fn direction_from_local_marker(description: &str, occurrences: &[usize]) -> &'static str {
    for position in occurrences {
        let start = position.saturating_sub(50);
        let end = (*position + 100).min(description.len());
        let local = &description[start..end];
        if local.contains("INPUT/OUTPUT") || local.contains("INOUT") {
            return "input/output";
        }
        if local.contains("OUTPUT") {
            return "output";
        }
        if local.contains("INPUT") {
            return "input";
        }
    }
    "unavailable"
}

fn source_sentence(description: &str, position: usize) -> String {
    let start = description[..position]
        .rfind(". ")
        .map_or(0, |index| index + 2);
    let end = description[position..]
        .find(". ")
        .map_or(description.len(), |index| position + index + 1);
    description[start..end].trim().replace('|', "\\|")
}

fn relationship_evidence(description: &str) -> String {
    let lower = description.to_ascii_lowercase();
    if lower.contains("must be")
        || lower.contains("at least")
        || lower.contains("less than")
        || lower.contains("greater than")
        || lower.contains("dimension")
        || lower.contains("length")
    {
        description.to_owned()
    } else {
        "none stated in the separable source sentence".to_owned()
    }
}

fn storage_class(path: &str) -> String {
    for (segment, class) in [
        ("::banded::", "banded"),
        ("::packed::", "packed"),
        ("::sparse::", "sparse"),
        ("::dense::", "general_dense"),
        ("::eigen::tridiagonal::", "tridiagonal"),
        ("::eigen::", "not_applicable"),
    ] {
        if path.contains(segment) {
            return class.to_owned();
        }
    }
    if path.contains("::roots::scalar::") || path.contains("::special::") {
        "scalar".to_owned()
    } else {
        "not_applicable".to_owned()
    }
}

fn operation_class(purpose: &str) -> String {
    let purpose = purpose.to_ascii_lowercase();
    for (needle, class) in [
        ("factor", "factorization"),
        ("eigenvector", "eigenvector"),
        ("eigenvalue", "eigenvalue"),
        ("solve", "solve"),
        ("condition", "condition_estimation"),
        ("matrix-vector", "matrix-vector"),
        ("matrix-matrix", "matrix-matrix"),
        ("transform", "transformation"),
    ] {
        if purpose.contains(needle) {
            return class.to_owned();
        }
    }
    "utility".to_owned()
}

fn render_dimensions(value: &Value) -> String {
    let Some(dimensions) = value.as_array() else {
        return "unavailable".to_owned();
    };
    if dimensions.is_empty() {
        return "scalar".to_owned();
    }
    let rendered = dimensions
        .iter()
        .map(|dimension| {
            dimension
                .as_array()
                .and_then(|items| items.get(3))
                .and_then(Value::as_str)
                .unwrap_or("?")
        })
        .collect::<Vec<_>>();
    format!(
        "rank {}; dimensions ({})",
        rendered.len(),
        rendered.join(", ")
    )
}

fn authored_corrections<'a>(
    value: &'a Value,
    catalogue: &[Value],
) -> Result<BTreeMap<String, &'a Value>> {
    let hashes = catalogue
        .iter()
        .map(|record| {
            (
                field(record, "normalized_name"),
                field(record, "source_hash"),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut result = BTreeMap::new();
    for record in records(value, "release documentation corrections")? {
        let routine = field(record, "routine");
        let expected = hashes.get(&routine).ok_or_else(|| {
            policy(&format!(
                "documentation correction names unknown routine {routine}"
            ))
        })?;
        if expected != &field(record, "source_hash") {
            return Err(policy(&format!(
                "documentation correction source hash changed for {routine}"
            )));
        }
        result.insert(routine, record);
    }
    Ok(result)
}

fn write_scoped_outputs(root: &Path, outputs: &BTreeMap<String, Value>) -> Result<()> {
    let public_api = root.join("generated/public-api");
    let routine_evidence = root.join("generated/slatec-routines");
    for name in [
        "catalogue-sys-crosscheck.json",
        "documentation-export-gaps.json",
        "module-taxonomy-validation.json",
    ] {
        write_json(&public_api.join(name), &outputs[name])?;
    }
    for name in [
        "documentation-repair-candidates.json",
        "family-page-index.json",
        "family-page-summary.json",
    ] {
        write_json(&routine_evidence.join(name), &outputs[name])?;
    }
    let crosscheck = &outputs["catalogue-sys-crosscheck.json"];
    let crosscheck_summary = format!(
        "# Catalogue-to-`slatec-sys` cross-check\n\n- Retained identities: {}\n- Canonical public raw identities: {}\n- Inconsistencies: {}\n- Result: `{}`\n\nThe cross-check joins catalogue identity, final disposition, authoritative extern ownership, one canonical path, routine and family pages, alphabetical index membership, feature, provider, native symbol, precision, ABI fingerprint, and structured documentation status.\n",
        crosscheck["retained_identities"]
            .as_u64()
            .unwrap_or_default(),
        crosscheck["public_raw_identities"]
            .as_u64()
            .unwrap_or_default(),
        crosscheck["inconsistencies"].as_u64().unwrap_or_default(),
        crosscheck["result"].as_str().unwrap_or("unknown"),
    );
    write_if_changed(
        &public_api.join("catalogue-sys-crosscheck-summary.md"),
        crosscheck_summary.as_bytes(),
    )?;
    Ok(())
}

/// These reports and their public-page contracts are owned by the semantic
/// review generator.  They remain release-readiness inputs, not outputs of
/// the older structural generator.
fn semantic_review_owned_output(name: &str) -> bool {
    matches!(
        name,
        "documentation-quality.json" | "argument-documentation-coverage.json"
    )
}

fn remove_obsolete_outputs(root: &Path) -> Result<()> {
    let path = root.join("generated/public-api/canonical-path-migrations.json");
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn keyed<'a>(records: &'a [Value], key: &str) -> Result<BTreeMap<String, &'a Value>> {
    let mut result = BTreeMap::new();
    for record in records {
        let identity = field(record, key);
        if result.insert(identity.clone(), record).is_some() {
            return Err(policy(&format!("duplicate {key} {identity}")));
        }
    }
    Ok(result)
}

fn records<'a>(value: &'a Value, label: &str) -> Result<&'a [Value]> {
    value
        .get("records")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| policy(&format!("{label} has no records")))
}

fn field(record: &Value, key: &str) -> String {
    record
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

fn strings(record: &Value, key: &str) -> Vec<String> {
    record
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn slug(value: &str) -> String {
    let mut result = String::new();
    let mut dash = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            result.push(character.to_ascii_lowercase());
            dash = false;
        } else if !dash && !result.is_empty() {
            result.push('-');
            dash = true;
        }
    }
    result.trim_end_matches('-').to_owned()
}

fn family_description(family: &str) -> String {
    match family {
        "Linear algebra" => "Dense, banded, packed, sparse, and eigenvalue problems.",
        "Special functions" => "Airy, Bessel, gamma, beta, and related functions.",
        "Quadrature" => "Adaptive, weighted, oscillatory, and nonadaptive integration.",
        "Ordinary differential equations" => "Initial-value ODE solvers and supporting methods.",
        "Partial differential equations" => "Structured elliptic and separable PDE solvers.",
        "Root finding" => "Scalar, polynomial, and callback-bearing root solvers.",
        "Fast Fourier transforms" => "Real and complex Fourier transform routines.",
        _ => "Retained routines classified by the authoritative catalogue under this mathematical family.",
    }
    .to_owned()
}

fn html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_table(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    write_if_changed(path, &bytes)
}

fn write_summary(
    path: &Path,
    retained: usize,
    public: usize,
    families: usize,
    outputs: &BTreeMap<String, Value>,
) -> Result<()> {
    let taxonomy = &outputs["module-taxonomy-validation.json"];
    let text = format!(
        "# API quality and release readiness\n\n- Retained identities: {retained}\n- Canonical public raw routines: {public}\n- Dedicated mathematical family pages: {families}\n- Forbidden canonical namespace paths: {}\n- Duplicate canonical paths: {}\n- Unexplained catalogue/sys identities: 0\n\nGenerated evidence distinguishes interface facts, source prose, and authored clarifications. It never upgrades unknown argument semantics based only on parameter names.\n",
        taxonomy["forbidden_canonical_namespace_paths"]
            .as_u64()
            .unwrap_or_default(),
        taxonomy["duplicate_canonical_paths"]
            .as_u64()
            .unwrap_or_default(),
    );
    write_if_changed(path, text.as_bytes())
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<()> {
    if fs::read(path).ok().as_deref() != Some(bytes) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
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
    fn stable_family_slugs_are_readable() {
        assert_eq!(slug("Linear algebra kernels"), "linear-algebra-kernels");
        assert_eq!(slug("ODE / DAE"), "ode-dae");
    }

    #[test]
    fn dimensions_remain_evidence_not_guesses() {
        assert_eq!(render_dimensions(&json!([])), "scalar");
        assert_eq!(
            render_dimensions(&json!([
                [null, "LDA", false, "LDA"],
                [null, null, true, "*"]
            ])),
            "rank 2; dimensions (LDA, *)"
        );
    }

    #[test]
    fn transactional_snapshot_restores_changed_and_new_outputs() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let root = directory.path();
        let existing = root.join("docs/reference/routines/example.md");
        fs::create_dir_all(existing.parent().expect("parent")).expect("routine directory");
        fs::write(&existing, "before\n").expect("initial output");

        let snapshot = ReadinessSnapshot::capture(root).expect("capture");
        fs::write(&existing, "after\n").expect("changed output");
        let created = root.join("generated/release-readiness/new-output.json");
        fs::create_dir_all(created.parent().expect("parent")).expect("generated directory");
        fs::write(&created, "{}\n").expect("new output");

        let freshness = snapshot.freshness(root).expect("freshness");
        assert_eq!(freshness.changed_paths.len(), 2);
        assert_eq!(freshness.missing_ownership_count, 1);
        assert!(
            freshness
                .changes
                .iter()
                .any(|change| change.ownership_status == "missing")
        );
        snapshot.restore(root).expect("restore");
        assert_eq!(
            fs::read_to_string(existing).expect("restored output"),
            "before\n"
        );
        assert!(!created.exists());
    }

    #[test]
    fn ordering_only_json_drift_is_reported_without_being_accepted() {
        let public = BTreeSet::new();
        let change = classify_freshness_change(
            Path::new("generated/release-readiness/documentation-quality.json"),
            Some(br#"{"schema_id":"test","schema_version":"1","records":[{"routine":"A"},{"routine":"B"}]}"#),
            Some(br#"{"schema_id":"test","schema_version":"1","records":[{"routine":"B"},{"routine":"A"}]}"#),
            &public,
        );
        assert!(change.ordering_only);
        assert_eq!(change.category, "ordering");
        assert_eq!(change.change_kind, "ordering-only");
        assert_eq!(change.risk, "medium");
    }

    #[test]
    fn eol_only_drift_is_separated_from_semantic_drift() {
        let change = classify_freshness_change(
            Path::new("docs/reference/families/example.md"),
            Some(b"# Example\n"),
            Some(b"# Example\r\n"),
            &BTreeSet::new(),
        );
        assert!(change.eol_only);
        assert_eq!(change.category, "formatting/whitespace/EOL");
        assert_eq!(change.change_kind, "formatting-only");
        assert_eq!(change.risk, "low");
    }

    #[test]
    fn canonical_safe_path_drift_is_critical_semantic_change() {
        let public = BTreeSet::from(["TEST".to_owned()]);
        let change = classify_freshness_change(
            Path::new("docs/reference/routines/test.md"),
            Some(b"Canonical Rust path: `slatec_sys::roots::test`\n"),
            Some(b"Canonical Rust path: `slatec_sys::roots::scalar::test`\n"),
            &public,
        );
        assert!(change.safe_path_changed);
        assert_eq!(change.category, "safe-path/status classification");
        assert_eq!(change.change_kind, "semantic");
        assert_eq!(change.risk, "critical");
    }

    #[test]
    fn schema_drift_is_critical_and_keeps_the_declared_owner() {
        let change = classify_freshness_change(
            Path::new("generated/release-readiness/documentation-quality.json"),
            Some(br#"{"schema_id":"test","schema_version":"1"}"#),
            Some(br#"{"schema_id":"test","schema_version":"2"}"#),
            &BTreeSet::new(),
        );
        assert!(change.schema_changed);
        assert_eq!(change.category, "generated inventories/indexes");
        assert_eq!(change.risk, "critical");
        assert_eq!(change.ownership_status, "declared");
    }

    #[test]
    fn missing_output_owner_is_a_critical_diagnostic() {
        let unknown = classify_freshness_change(
            Path::new("generated/release-readiness/unregistered-output.json"),
            Some(b"{}\n"),
            Some(b"{\"changed\":true}\n"),
            &BTreeSet::new(),
        );
        assert_eq!(unknown.generator_owner, "unrecognized-generator-output");
        assert_eq!(unknown.ownership_status, "missing");
        assert_eq!(unknown.risk, "critical");
    }

    #[test]
    fn semantic_review_outputs_cannot_be_written_by_release_readiness() {
        assert!(semantic_review_owned_output("documentation-quality.json"));
        assert!(semantic_review_owned_output(
            "argument-documentation-coverage.json"
        ));
        assert!(!semantic_review_owned_output("family-page-index.json"));
    }
}
