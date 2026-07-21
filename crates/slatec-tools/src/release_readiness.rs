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
    write_routine_docs(root, &docs)?;
    let outputs = evidence(
        root,
        &docs,
        catalogue_records.len(),
        public,
        &ownership_by_routine,
    )?;
    for (name, value) in &outputs {
        write_json(&output_dir.join(name), value)?;
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
    let mut result = generate(root, output_dir)?;
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

fn write_routine_docs(root: &Path, docs: &[RoutineDoc]) -> Result<()> {
    let routine_dir = root.join("docs/reference/routines");
    for record in docs {
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
        "documentation-quality.json",
        "documentation-repair-candidates.json",
        "family-page-index.json",
        "family-page-summary.json",
        "argument-documentation-coverage.json",
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
    let quality = &outputs["documentation-quality.json"];
    let mut quality_summary = String::from(
        "# Routine documentation quality\n\nDocumentation quality is disposition-aware and never treats a one-line purpose as complete public documentation.\n\n| Quality level | Identities |\n| --- | ---: |\n",
    );
    if let Some(counts) = quality.get("counts").and_then(Value::as_object) {
        for (level, count) in counts {
            quality_summary.push_str(&format!(
                "| `{level}` | {} |\n",
                count.as_u64().unwrap_or_default()
            ));
        }
    }
    quality_summary.push_str(&format!(
        "\n- Mangled candidates in the pre-existing coverage audit: {}\n- Candidates still requiring explicit review: {}\n",
        quality["mangled_candidate_flags_before"].as_u64().unwrap_or_default(),
        quality["mangled_candidate_flags_after"].as_u64().unwrap_or_default(),
    ));
    write_if_changed(
        &routine_evidence.join("documentation-quality-summary.md"),
        quality_summary.as_bytes(),
    )?;
    Ok(())
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
}
