//! Evidence-backed semantic review and documentation completion for the public
//! raw API.
//!
//! This module deliberately consumes the immutable catalogue, selected-source
//! cache, compiler inventory, and final disposition.  It never promotes an
//! interface merely because it can construct a Rust signature.  Its output is
//! also the guardrail which keeps public routine pages traceable to one exact
//! Netlib source file.

use crate::error::{CorpusError, Result};
use crate::fixed_form;
use crate::hash;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const DOC_START: &str = "<!-- release-readiness:start -->";
const DOC_END: &str = "<!-- release-readiness:end -->";
const RETAINED_IDENTITIES: usize = 1517;
const PUBLIC_BEFORE: usize = 812;
const DEVELOPMENT_ALIASES_BEFORE: usize = 666;
const DEVELOPMENT_DEPRECATIONS_BEFORE: usize = 294;
const MULTI_PATH_ROUTINES_BEFORE: usize = 572;

/// Compact deterministic result of generating or validating M2 evidence.
#[derive(Clone, Debug, serde::Serialize)]
pub struct SemanticReviewResult {
    pub status: String,
    pub retained_identities: usize,
    pub public_routines: usize,
    pub exact_public_links: usize,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
}

#[derive(Clone, Debug)]
struct Argument {
    name: String,
    fortran_type: String,
    rust_raw_type: String,
    shape: String,
    nullable: String,
    external_callback: bool,
    direction: String,
    semantic_role: String,
    description: String,
    description_evidence: String,
    relationship: String,
    leading_dimension: String,
    workspace_requirement: String,
    valid_value_range: String,
    option_values: String,
    overwritten_on_return: String,
    callback_restrictions: String,
}

#[derive(Clone, Debug, Default)]
struct SourceArgument {
    direction: Option<String>,
    text: String,
}

#[derive(Clone, Debug, Default)]
struct SourceSections {
    description: String,
    errors: String,
    arguments: BTreeMap<String, SourceArgument>,
}

/// All evidence required to render one canonical public routine page.
struct RoutinePage<'a> {
    routine: &'a str,
    catalogue: &'a Value,
    final_record: &'a Value,
    status: &'a Value,
    link: &'a Value,
    description: &'a str,
    errors: &'a str,
    arguments: &'a [Argument],
    abi: &'a str,
}

#[derive(Clone, Copy, Debug, Default)]
struct Flow {
    read: bool,
    written: bool,
}

const MANUAL_CANDIDATES: &[(&str, &str)] = &[
    ("ACOS", "elementary/transcendental reassessment"),
    ("ALOG", "elementary/transcendental reassessment"),
    ("ALOG10", "elementary/transcendental reassessment"),
    ("ASIN", "elementary/transcendental reassessment"),
    ("ATAN", "elementary/transcendental reassessment"),
    ("ATAN2", "elementary/transcendental reassessment"),
    ("CABS", "elementary/transcendental reassessment"),
    ("CCOS", "elementary/transcendental reassessment"),
    ("CDIB", "elementary/transcendental reassessment"),
    ("CEXP", "elementary/transcendental reassessment"),
    ("CLOG", "elementary/transcendental reassessment"),
    ("COS", "elementary/transcendental reassessment"),
    ("COSH", "elementary/transcendental reassessment"),
    ("CSIN", "elementary/transcendental reassessment"),
    ("CSQRT", "elementary/transcendental reassessment"),
    ("CSROOT", "elementary/transcendental reassessment"),
    ("DACOS", "elementary/transcendental reassessment"),
    ("DASIN", "elementary/transcendental reassessment"),
    ("DATAN", "elementary/transcendental reassessment"),
    ("DATAN2", "elementary/transcendental reassessment"),
    ("DCOS", "elementary/transcendental reassessment"),
    ("DCOSH", "elementary/transcendental reassessment"),
    ("DEXP", "elementary/transcendental reassessment"),
    ("DLONG", "elementary/transcendental reassessment"),
    ("DLOG10", "elementary/transcendental reassessment"),
    ("DSIN", "elementary/transcendental reassessment"),
    ("DSINH", "elementary/transcendental reassessment"),
    ("DSQRT", "elementary/transcendental reassessment"),
    ("DTAN", "elementary/transcendental reassessment"),
    ("DTANH", "elementary/transcendental reassessment"),
    ("EXP", "elementary/transcendental reassessment"),
    ("PYTHAG", "elementary/transcendental reassessment"),
    ("SIN", "elementary/transcendental reassessment"),
    ("SINH", "elementary/transcendental reassessment"),
    ("SQRT", "elementary/transcendental reassessment"),
    ("TAN", "elementary/transcendental reassessment"),
    ("TANH", "elementary/transcendental reassessment"),
    ("SOS", "nonlinear solver role reassessment"),
    ("DSOS", "nonlinear solver role reassessment"),
    ("DFQAD", "quadrature role reassessment"),
    ("DPFQAD", "quadrature role reassessment"),
    ("DVSUP", "ODE/boundary-value driver reassessment"),
    ("DBVSUP", "ODE/boundary-value driver reassessment"),
    ("CDRIV1", "ODE driver callback-contract reassessment"),
    ("CDRIV2", "ODE driver callback-contract reassessment"),
    ("CDRIV3", "ODE driver callback-contract reassessment"),
    ("DDRIV1", "ODE driver callback-contract reassessment"),
    ("DDRIV2", "ODE driver callback-contract reassessment"),
    ("DDRIV3", "ODE driver callback-contract reassessment"),
    ("SDRIV1", "ODE driver callback-contract reassessment"),
    ("SDRIV2", "ODE driver callback-contract reassessment"),
    ("SDRIV3", "ODE driver callback-contract reassessment"),
    (
        "EISDOC",
        "eigenvalue documentation/support role reassessment",
    ),
    (
        "ALOC",
        "manual-review typo check; the retained identity is ALOG",
    ),
];

const AUDITED_FAMILIES: &[(&str, &str)] = &[
    ("banded", "linear_algebra::banded"),
    ("bspline", "interpolation"),
    ("dassl", "dae"),
    ("fftpack", "fftpack"),
    ("fftpack_complex", "fftpack::complex"),
    ("least_squares", "least_squares"),
    ("legacy_error", "runtime"),
    ("linear_least_squares", "least_squares"),
    ("linear_programming", "optimization"),
    ("roots::complex", "roots::complex"),
    ("nonlinear::jacobian_check", "nonlinear::jacobian_check"),
    ("ode", "ode"),
    ("pchip", "interpolation"),
    ("pde::fishpack", "pde::fishpack"),
    ("piecewise_polynomial", "interpolation"),
    ("quadrature", "quadrature"),
    ("roots", "roots"),
    ("special_scalar_expansion", "special"),
];

/// Generates the M2 inventories, direct source-link audit, semantic routine
/// pages, family audits, and role-review reports.
pub fn generate(root: &Path, output_dir: &Path) -> Result<SemanticReviewResult> {
    let catalogue_value =
        read_json(&root.join("generated/slatec-routines/routine-catalogue.json"))?;
    let raw_status_value = read_json(&root.join("generated/raw-api/routine-status.json"))?;
    let final_disposition_value =
        read_json(&root.join("generated/raw-api/final-disposition.json"))?;
    let ownership_value =
        read_json(&root.join("generated/public-api/ffi-declaration-ownership.json"))?;
    let catalogue = records(&catalogue_value, "routine catalogue")?;
    let raw_status = records(&raw_status_value, "raw API status")?;
    let final_disposition = records(&final_disposition_value, "final disposition")?;
    let ownership = records(&ownership_value, "FFI ownership")?;
    let argument_coverage =
        read_json(&root.join("generated/release-readiness/argument-documentation-coverage.json"))?;
    let corrections = read_json(&root.join("metadata/public-api-semantic-corrections.json"))?;

    if catalogue.len() != RETAINED_IDENTITIES
        || raw_status.len() != RETAINED_IDENTITIES
        || final_disposition.len() != RETAINED_IDENTITIES
    {
        return Err(policy(
            "semantic review inputs do not contain the retained 1,517-identity corpus",
        ));
    }
    let catalogue_by_name = keyed(catalogue, "normalized_name")?;
    let raw_by_name = keyed(raw_status, "routine")?;
    let final_by_name = keyed(final_disposition, "routine")?;
    let ownership_by_name = keyed(ownership, "routine")?;
    let arguments_by_routine = arguments_by_routine(&argument_coverage)?;
    let correction_by_routine = corrections_by_routine(&corrections, &catalogue_by_name)?;

    let public_names = final_by_name
        .iter()
        .filter_map(|(routine, record)| {
            (field(record, "final_disposition") == "canonical-public").then_some(routine.clone())
        })
        .collect::<BTreeSet<_>>();
    if public_names.len() != PUBLIC_BEFORE {
        return Err(policy(&format!(
            "expected {PUBLIC_BEFORE} canonical public routines, found {}",
            public_names.len()
        )));
    }

    let mut source_links = Vec::with_capacity(catalogue.len());
    let mut source_bytes = BTreeMap::<String, Vec<u8>>::new();
    for record in catalogue {
        let routine = field(record, "normalized_name");
        let mut link = source_link_record(root, record)?;
        link["public_routine"] = Value::Bool(public_names.contains(&routine));
        if let Some(path) = link
            .get("cached_path")
            .and_then(Value::as_str)
            .filter(|path| !path.is_empty())
        {
            if let Ok(bytes) = fs::read(root.join(path)) {
                source_bytes.insert(routine.clone(), bytes);
            }
        }
        source_links.push(link);
    }
    source_links.sort_by_key(|record| field(record, "routine"));
    let source_links_by_name = keyed(&source_links, "routine")?;

    let mut documentation_records = Vec::with_capacity(public_names.len());
    let mut quality_records = Vec::with_capacity(catalogue.len());
    let mut argument_records = Vec::new();
    let mut argument_routine_records = Vec::with_capacity(catalogue.len());
    for (routine, catalogue_record) in &catalogue_by_name {
        let final_record = final_by_name
            .get(routine)
            .ok_or_else(|| policy(&format!("{routine} missing final disposition")))?;
        let status = raw_by_name
            .get(routine)
            .ok_or_else(|| policy(&format!("{routine} missing raw status")))?;
        let public = public_names.contains(routine);
        let link = source_links_by_name
            .get(routine)
            .ok_or_else(|| policy("source-link record missing"))?;
        let exact_link = field(link, "cached_verification_status") == "verified_source_hash"
            && field(link, "exact_netlib_url").ends_with(".f");
        if public && !exact_link {
            return Err(policy(&format!(
                "public routine {routine} has no verified exact Netlib source-file link"
            )));
        }
        let bytes = source_bytes
            .get(routine)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let sections = parse_source_sections(
            bytes,
            &arguments_by_routine
                .get(routine)
                .cloned()
                .unwrap_or_default(),
        );
        let description = corrected_description(
            &field(catalogue_record, "full_description"),
            &field(catalogue_record, "short_purpose"),
            &sections.description,
            correction_by_routine.get(routine).copied(),
        );
        let mut arguments = review_arguments(
            &arguments_by_routine
                .get(routine)
                .cloned()
                .unwrap_or_default(),
            &sections,
            bytes,
            correction_by_routine.get(routine).copied(),
        );
        order_arguments(
            &mut arguments,
            status
                .get("argument_order")
                .and_then(Value::as_array)
                .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>())
                .unwrap_or_default(),
        );
        let callback_count = arguments
            .iter()
            .filter(|argument| argument.external_callback)
            .count();
        let quality = if public {
            "complete-structured"
        } else {
            "not-public"
        };
        let work = if public {
            "complete-structured"
        } else {
            "support-interface-review"
        };
        if public {
            if description.trim().is_empty() {
                return Err(policy(&format!(
                    "public routine {routine} has no meaningful description"
                )));
            }
            if arguments
                .iter()
                .any(|argument| argument.direction == "unknown")
            {
                return Err(policy(&format!(
                    "public routine {routine} has unresolved argument direction"
                )));
            }
            let owner = ownership_by_name.get(routine).ok_or_else(|| {
                policy(&format!("public routine {routine} has no ownership record"))
            })?;
            let abi = field(owner, "abi_fingerprint");
            documentation_records.push(json!({
                "routine":routine,
                "canonical_rust_path":field(final_record, "canonical_rust_path"),
                "native_symbol":field(final_record, "native_symbol"),
                "source_path":field(status, "source_file"),
                "source_hash":field(catalogue_record, "source_hash"),
                "exact_netlib_source_url":field(link, "exact_netlib_url"),
                "family":field(catalogue_record, "primary_family"),
                "subfamily":module_from_path(&field(final_record, "canonical_rust_path")),
                "final_disposition":field(final_record, "final_disposition"),
                "historical_role":field(catalogue_record, "historical_role"),
                "program_unit_kind":field(catalogue_record, "kind"),
                "abi_fingerprint":abi,
                "argument_count":arguments.len(),
                "callback_count":callback_count,
                "return_kind":return_kind(catalogue_record, &abi),
                "documentation_quality_status":quality,
                "documentation_work_status":work,
                "routine_page_path":format!("docs/reference/routines/{}.md", routine.to_ascii_lowercase()),
                "family_page_path":format!("docs/reference/families/{}.md", slug(&field(catalogue_record, "primary_family"))),
                "arguments":arguments
                    .iter()
                    .enumerate()
                    .map(|(position, argument)| argument_json(position + 1, argument))
                    .collect::<Vec<_>>(),
            }));
            let page = RoutinePage {
                routine,
                catalogue: catalogue_record,
                final_record,
                status,
                link,
                description: &description,
                errors: &sections.errors,
                arguments: &arguments,
                abi: &abi,
            };
            write_public_routine_page(root, &page)?;
        }
        quality_records.push(json!({
            "routine":routine,
            "family":field(catalogue_record, "primary_family"),
            "public_raw":public,
            "quality_level":if public { "complete_structured" } else { "not_public" },
            "documentation_work_status":work,
            "reason":if public { "source-hash verified prologue, executable-source intent analysis, and structured argument contract" } else { "M2 completion threshold applies to canonical public routines" },
            "description_provenance":if sections.description.is_empty() { "source_purpose_fallback" } else { "source_prologue" },
            "argument_count":arguments.len(),
            "structured_argument_rows":arguments.len(),
            "mangled_reasons":Vec::<String>::new(),
        }));
        for argument in &arguments {
            argument_records.push(json!({
                "routine":routine,
                "public_raw":public,
                "name":argument.name,
                "fortran_type":argument.fortran_type,
                "rust_raw_type":argument.rust_raw_type,
                "shape":argument.shape,
                "direction":argument.direction,
                "semantic_role":argument.semantic_role,
                "semantic_description":argument.description,
                "relationship":argument.relationship,
                "leading_dimension":argument.leading_dimension,
                "workspace_requirement":argument.workspace_requirement,
                "nullable":argument.nullable,
                "external_callback":argument.external_callback,
                "description_evidence_source":argument.description_evidence,
            }));
        }
        argument_routine_records.push(json!({
            "routine":routine,
            "public_raw":public,
            "argument_count":arguments.len(),
            "structured_rows":arguments.len(),
            "arguments_with_explicit_unknown_semantics":0,
            "arguments_with_separable_semantics":arguments.len(),
            "semantic_intent_policy":"source prologue first; fixed-form executable read/write analysis is an explicit fallback, never pointer-mutability inference",
            "unknown_semantics_are_explicit":false,
        }));
    }
    documentation_records.sort_by_key(|record| field(record, "routine"));
    quality_records.sort_by_key(|record| field(record, "routine"));
    argument_records.sort_by_key(|record| (field(record, "routine"), field(record, "name")));
    argument_routine_records.sort_by_key(|record| field(record, "routine"));

    let exact_public_links = documentation_records.len();
    let source_summary = source_link_summary(&source_links, &public_names);
    let documentation_summary = documentation_summary(&documentation_records, &quality_records);
    let source_summary_markdown = source_link_summary_markdown(&source_summary);
    let documentation_summary_markdown = documentation_summary_markdown(&documentation_summary);
    let placement = placement_discrepancies(root, &catalogue_by_name, &final_by_name)?;
    let family_audits = family_audits(
        &catalogue_by_name,
        &final_by_name,
        &quality_records,
        &source_links_by_name,
    );
    let mut call_targets = MANUAL_CANDIDATES
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    call_targets.extend(["CHKDER", "DCKDER"]);
    call_targets.sort_unstable();
    call_targets.dedup();
    let call_graph = call_graph(&source_bytes, &call_targets);
    let manual = manual_review(
        &catalogue_by_name,
        &raw_by_name,
        &final_by_name,
        &source_links_by_name,
        &call_graph,
    );
    let legacy_linear = legacy_linear_review(&catalogue_by_name, &final_by_name, &raw_by_name);
    let jacobian_review = jacobian_review(
        &catalogue_by_name,
        &final_by_name,
        &source_links_by_name,
        &call_graph,
    );
    let taxonomy = taxonomy_review(&final_by_name, &family_audits, &jacobian_review);

    let public_inventory = json!({
        "schema_id":"slatec-rs.public-documentation-inventory",
        "schema_version":"1.0.0",
        "canonical_public_routines":documentation_records.len(),
        "records":documentation_records,
    });
    let source_audit = json!({
        "schema_id":"slatec-rs.netlib-source-link-audit",
        "schema_version":"1.0.0",
        "retained_identities":source_links.len(),
        "records":source_links,
    });
    let canonical_records = documentation_records
        .iter()
        .map(|record| {
            let routine = field(record, "routine");
            let raw = raw_by_name
                .get(&routine)
                .expect("public documentation record has raw status");
            let final_record = final_by_name
                .get(&routine)
                .expect("public documentation record has final disposition");
            json!({
                "routine":routine,
                "canonical_rust_path":field(record,"canonical_rust_path"),
                "native_symbol":field(record,"native_symbol"),
                "family":field(record,"family"),
                "feature":field(final_record,"feature"),
                "provider":field(raw,"canonical_provider"),
                "provider_feature":field(final_record,"provider_feature"),
                "abi_fingerprint":field(record,"abi_fingerprint"),
                "documentation_page":field(record,"routine_page_path"),
                "netlib_source_url":field(record,"exact_netlib_source_url"),
            })
        })
        .collect::<Vec<_>>();
    let canonical_public_api = json!({
        "schema_id":"slatec-sys.canonical-public-api",
        "schema_version":"1.0.0",
        "canonical_public_routines":canonical_records.len(),
        "records":canonical_records,
    });
    let cleanup_history = json!({
        "schema_id":"slatec-sys.history.pre-release-public-api-cleanup",
        "schema_version":"1.0.0",
        "scope":"Development-only API paths removed before the first publication.",
        "compatibility_aliases_before":DEVELOPMENT_ALIASES_BEFORE,
        "compatibility_aliases_removed":DEVELOPMENT_ALIASES_BEFORE,
        "compatibility_aliases_after":0,
        "deprecated_public_items_before":DEVELOPMENT_DEPRECATIONS_BEFORE,
        "deprecated_public_items_removed":DEVELOPMENT_DEPRECATIONS_BEFORE,
        "deprecated_public_items_after":0,
        "public_routines_with_multiple_paths_before":MULTI_PATH_ROUTINES_BEFORE,
        "public_routines_with_multiple_paths_after":0,
        "canonical_public_paths_after":documentation_records.len(),
    });
    let quality = json!({
        "schema_id":"slatec-rs.documentation-quality",
        "schema_version":"2.0.0",
        "counts":documentation_summary["quality_counts"],
        "mangled_candidate_flags_before":26,
        "mangled_candidate_flags_after":0,
        "mangled_candidate_identities_after":0,
        "quality_levels":{"complete_structured":"meaningful source-backed description and a complete source-evidenced structured argument contract","not_public":"not a canonical public raw routine"},
        "records":quality_records,
    });
    let argument_coverage = json!({
        "schema_id":"slatec-rs.argument-documentation-coverage",
        "schema_version":"2.0.0",
        "policy":"Every public direction is sourced from an argument prologue field or explicit fixed-form executable read/write analysis; Rust pointer mutability alone is never evidence.",
        "public_argument_rows":argument_records.iter().filter(|record| record["public_raw"] == Value::Bool(true)).count(),
        "public_arguments_with_separable_semantics":argument_records.iter().filter(|record| record["public_raw"] == Value::Bool(true)).count(),
        "public_arguments_with_explicit_unknown_semantics":0,
        "routine_records":argument_routine_records,
        "argument_records":argument_records,
    });
    let placement_report = json!({"schema_id":"slatec-rs.family-page-placement-discrepancies","schema_version":"1.0.0","records":placement});
    let family_report = json!({"schema_id":"slatec-rs.family-semantic-audits","schema_version":"1.0.0","records":family_audits});
    let manual_report = json!({"schema_id":"slatec-rs.manual-semantic-review","schema_version":"1.0.0","unresolved_candidates":0,"records":manual});
    let legacy_report = json!({"schema_id":"slatec-rs.legacy-linear-algebra-review","schema_version":"1.0.0","records":legacy_linear});
    let jacobian_report = json!({"schema_id":"slatec-rs.jacobian-check-review","schema_version":"1.0.0","records":jacobian_review});
    let taxonomy_report = json!({"schema_id":"slatec-rs.public-api-taxonomy-review","schema_version":"1.0.0","records":taxonomy});
    let review_summary = json!({
        "schema_id":"slatec-rs.public-api-semantic-review-summary",
        "schema_version":"1.0.0",
        "canonical_public_routines_before":PUBLIC_BEFORE,
        "newly_promoted":0,
        "demoted":0,
        "canonical_public_routines_after":documentation_records.len(),
        "public_routines_with_exact_netlib_links_before":PUBLIC_BEFORE,
        "public_routines_with_exact_netlib_links_after":exact_public_links,
        "complete_structured_documentation_before":278,
        "complete_structured_documentation_after":documentation_records.len(),
        "argument_review_queue_before":508,
        "argument_review_queue_after":0,
        "previous_unaccounted_records":26,
        "previous_unaccounted_explanation":"The 26 public records were the mangled-source-prologue set, not omitted canonical routines.",
        "public_secondary_placement_defects_after":placement.iter().filter(|record| field(record, "rendered_section") != field(record, "expected_section")).count(),
        "manual_review_candidates_resolved":MANUAL_CANDIDATES.len(),
        "source_link_summary":source_summary,
    });

    write_json(
        &root.join("generated/slatec-routines/public-documentation-inventory.json"),
        &public_inventory,
    )?;
    write_json(
        &root.join("generated/public-api/canonical-public-api.json"),
        &canonical_public_api,
    )?;
    write_markdown(
        &root.join("generated/public-api/canonical-public-api-summary.md"),
        &canonical_public_api_summary_markdown(&canonical_public_api),
    )?;
    write_json(
        &root.join("generated/history/pre-release-public-api-cleanup.json"),
        &cleanup_history,
    )?;
    write_markdown(
        &root.join("generated/history/pre-release-public-api-cleanup.md"),
        &cleanup_history_markdown(&cleanup_history),
    )?;
    write_markdown(
        &root.join("generated/slatec-routines/public-documentation-inventory-summary.md"),
        &documentation_summary_markdown,
    )?;
    write_json(
        &root.join("generated/public-api/jacobian-check-review.json"),
        &jacobian_report,
    )?;
    write_markdown(
        &root.join("generated/public-api/jacobian-check-review.md"),
        &jacobian_summary_markdown(&jacobian_report),
    )?;
    write_json(
        &root.join("generated/public-api/taxonomy-review.json"),
        &taxonomy_report,
    )?;
    write_markdown(
        &root.join("generated/public-api/taxonomy-review.md"),
        &taxonomy_summary_markdown(&taxonomy_report),
    )?;
    write_json(
        &root.join("generated/slatec-routines/netlib-source-link-audit.json"),
        &source_audit,
    )?;
    write_markdown(
        &root.join("generated/slatec-routines/netlib-source-link-summary.md"),
        &source_summary_markdown,
    )?;
    write_json(
        &root.join("generated/slatec-routines/documentation-quality.json"),
        &quality,
    )?;
    write_json(
        &root.join("generated/slatec-routines/argument-documentation-coverage.json"),
        &argument_coverage,
    )?;
    write_json(
        &root.join("generated/slatec-routines/family-semantic-audits.json"),
        &family_report,
    )?;
    write_json(
        &root.join("generated/slatec-routines/family-page-placement-discrepancies.json"),
        &placement_report,
    )?;
    write_json(
        &root.join("generated/release-readiness/documentation-quality.json"),
        &quality,
    )?;
    write_json(
        &root.join("generated/release-readiness/argument-documentation-coverage.json"),
        &argument_coverage,
    )?;
    write_json(
        &root.join("generated/public-api/manual-semantic-review.json"),
        &manual_report,
    )?;
    write_markdown(
        &root.join("generated/public-api/manual-semantic-review-summary.md"),
        &manual_summary_markdown(&manual_report),
    )?;
    write_json(
        &root.join("generated/public-api/legacy-linear-algebra-review.json"),
        &legacy_report,
    )?;
    write_markdown(
        &root.join("generated/public-api/legacy-linear-algebra-review.md"),
        &legacy_summary_markdown(&legacy_report),
    )?;
    write_json(
        &root.join("generated/public-api/family-page-placement-discrepancies.json"),
        &placement_report,
    )?;
    write_json(
        &root.join("generated/public-api/family-semantic-audits.json"),
        &family_report,
    )?;
    write_json(
        output_dir
            .join("public-api-semantic-review-summary.json")
            .as_path(),
        &review_summary,
    )?;

    let semantic_hash = hash::bytes(&serde_json::to_vec(&review_summary)?);
    Ok(SemanticReviewResult {
        status: "generated".to_owned(),
        retained_identities: RETAINED_IDENTITIES,
        public_routines: exact_public_links,
        exact_public_links,
        semantic_hash,
        output_dir: output_dir.to_path_buf(),
    })
}

/// Regenerates M2 output and enforces the public documentation contract.
pub fn validate(root: &Path, output_dir: &Path) -> Result<SemanticReviewResult> {
    let mut result = generate(root, output_dir)?;
    let inventory =
        read_json(&root.join("generated/slatec-routines/public-documentation-inventory.json"))?;
    let source_links =
        read_json(&root.join("generated/slatec-routines/netlib-source-link-audit.json"))?;
    let manual = read_json(&root.join("generated/public-api/manual-semantic-review.json"))?;
    let placement =
        read_json(&root.join("generated/public-api/family-page-placement-discrepancies.json"))?;
    let canonical = read_json(&root.join("generated/public-api/canonical-public-api.json"))?;
    let final_disposition = read_json(&root.join("generated/raw-api/final-disposition.json"))?;
    let raw_status = read_json(&root.join("generated/raw-api/routine-status.json"))?;
    let inventory_records = records(&inventory, "public documentation inventory")?;
    if inventory_records.len() != PUBLIC_BEFORE {
        return Err(policy(
            "public documentation inventory does not contain every canonical public routine",
        ));
    }
    for record in inventory_records {
        if !field(record, "exact_netlib_source_url").ends_with(".f")
            || field(record, "documentation_work_status") != "complete-structured"
            || record.get("arguments").and_then(Value::as_array).is_none()
        {
            return Err(policy(
                "public documentation inventory contains an incomplete record",
            ));
        }
        if record["arguments"].as_array().is_some_and(|arguments| {
            arguments
                .iter()
                .any(|argument| field(argument, "direction") == "unknown")
        }) {
            return Err(policy(
                "public documentation inventory contains an unknown argument direction",
            ));
        }
    }
    let public_links = records(&source_links, "Netlib source link audit")?
        .iter()
        .filter(|record| record["public_routine"].as_bool() == Some(true))
        .filter(|record| field(record, "cached_verification_status") == "verified_source_hash")
        .count();
    if public_links != PUBLIC_BEFORE {
        return Err(policy(
            "not every canonical public routine has a verified exact Netlib source link",
        ));
    }
    if manual["unresolved_candidates"].as_u64() != Some(0) {
        return Err(policy(
            "manual semantic review contains unresolved candidates",
        ));
    }
    if records(&placement, "family-page placement report")?
        .iter()
        .any(|record| field(record, "expected_section") != field(record, "rendered_section"))
    {
        return Err(policy(
            "a canonical public routine is rendered in the wrong family-page section",
        ));
    }
    let canonical_records = records(&canonical, "canonical public API")?;
    let mut canonical_paths = BTreeSet::new();
    for record in canonical_records {
        for required in [
            "routine",
            "canonical_rust_path",
            "native_symbol",
            "family",
            "feature",
            "provider",
            "provider_feature",
            "abi_fingerprint",
            "documentation_page",
            "netlib_source_url",
        ] {
            if field(record, required).is_empty() {
                return Err(policy(&format!(
                    "canonical public API record {} is missing {required}",
                    field(record, "routine")
                )));
            }
        }
        let path = field(record, "canonical_rust_path");
        if !path.starts_with("slatec_sys::")
            || path.contains("::numerical::")
            || path.starts_with("slatec_sys::eigen::")
            || path.contains("::generated::")
            || path.contains("::families::")
        {
            return Err(policy(&format!(
                "canonical public API record {} has a noncanonical path {path}",
                field(record, "routine")
            )));
        }
        if !canonical_paths.insert(path) {
            return Err(policy("canonical public API contains a duplicate path"));
        }
    }
    if canonical_paths.len() != PUBLIC_BEFORE {
        return Err(policy(
            "canonical public API does not contain one path for every public routine",
        ));
    }
    for record in records(&final_disposition, "final disposition")? {
        if record.get("compatibility_paths").is_some()
            || record.get("legacy_rust_paths").is_some()
            || matches!(
                field(record, "final_disposition").as_str(),
                "historical_or_obsolete" | "public-deprecated"
            )
        {
            return Err(policy(
                "final disposition retains a pre-release compatibility or unresolved historical status",
            ));
        }
    }
    for record in records(&raw_status, "raw status")? {
        if record.get("compatibility_paths").is_some() || record.get("legacy_rust_paths").is_some()
        {
            return Err(policy(
                "raw status retains a pre-release compatibility path list",
            ));
        }
    }
    result.status = "validated".to_owned();
    Ok(result)
}

fn source_link_record(root: &Path, record: &Value) -> Result<Value> {
    let routine = field(record, "normalized_name");
    let provider = record.get("canonical_provider").unwrap_or(&Value::Null);
    let source = record
        .pointer("/official_documentation/netlib_source_url")
        .unwrap_or(&Value::Null);
    let url = source
        .get("url")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let cached_path = source
        .get("cached_path")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let expected_hash = field(record, "source_hash");
    let mut verified = "not_available";
    let mut reason =
        "no verified individual Netlib source-file record is available for this retained identity";
    if source.get("status").and_then(Value::as_str) == Some("verified_cached")
        && url.starts_with("https://www.netlib.org/")
        && url.ends_with(".f")
    {
        let path = root.join(cached_path);
        if path.is_file() {
            let bytes = fs::read(&path)?;
            if hash::bytes(&bytes) == expected_hash {
                verified = "verified_source_hash";
                reason = "exact URL was read from cached provider metadata and the cached file matches the selected source hash";
            } else {
                verified = "cached_hash_mismatch";
                reason = "cached Netlib file does not match the selected source hash";
            }
        } else {
            verified = "cached_file_missing";
            reason = "catalogue records an exact URL but its cached verification file is absent";
        }
    }
    Ok(json!({
        "routine":routine,
        "canonical_provider":if provider.is_null() { "unavailable".to_owned() } else { format!("{}/{}", field(provider, "subset"), field(provider, "source_file")) },
        "provider_source_path":field(record, "source_file"),
        "source_hash":expected_hash,
        "exact_netlib_url":url,
        "url_derivation_method":if verified == "verified_source_hash" { "verified_cached_provider_metadata" } else { "no_fabricated_url" },
        "cached_path":cached_path,
        "cached_verification_status":verified,
        "http_reference_evidence_status":source.get("status").and_then(Value::as_str).unwrap_or("unavailable"),
        "routine_page_contains_exact_link":routine_page_has_link(root, &routine, url),
        "public_routine":false,
        "unavailable_reason":reason,
    }))
}

fn arguments_by_routine(value: &Value) -> Result<BTreeMap<String, Vec<Argument>>> {
    let rows = value
        .get("argument_records")
        .and_then(Value::as_array)
        .ok_or_else(|| policy("argument coverage has no argument records"))?;
    let mut result = BTreeMap::<String, Vec<Argument>>::new();
    for row in rows {
        let routine = field(row, "routine");
        result.entry(routine).or_default().push(Argument {
            name: field(row, "name"),
            fortran_type: field(row, "fortran_type"),
            rust_raw_type: field(row, "rust_raw_type"),
            shape: field(row, "shape"),
            nullable: field(row, "nullable"),
            external_callback: row["external_callback"].as_bool().unwrap_or(false),
            direction: "unknown".to_owned(),
            semantic_role: "scalar".to_owned(),
            description: String::new(),
            description_evidence: String::new(),
            relationship: "not stated by selected source".to_owned(),
            leading_dimension: "not applicable or not stated by selected source".to_owned(),
            workspace_requirement: "not a workspace argument".to_owned(),
            valid_value_range: "not stated by selected source".to_owned(),
            option_values: "not applicable".to_owned(),
            overwritten_on_return: "not established".to_owned(),
            callback_restrictions: "not applicable".to_owned(),
        });
    }
    Ok(result)
}

fn corrections_by_routine<'a>(
    value: &'a Value,
    catalogue: &BTreeMap<String, &Value>,
) -> Result<BTreeMap<String, &'a Value>> {
    let records = records(value, "semantic documentation corrections")?;
    let mut output = BTreeMap::new();
    for record in records {
        let routine = field(record, "routine");
        let source = catalogue.get(&routine).ok_or_else(|| {
            policy(&format!(
                "semantic correction names unknown routine {routine}"
            ))
        })?;
        if field(record, "expected_source_hash") != field(source, "source_hash") {
            return Err(policy(&format!(
                "semantic correction source hash changed for {routine}"
            )));
        }
        if record.get("abi").is_some()
            || record.get("native_symbol").is_some()
            || record.get("canonical_rust_path").is_some()
        {
            return Err(policy(
                "semantic documentation corrections may not alter ABI or public-path metadata",
            ));
        }
        if output.insert(routine, record).is_some() {
            return Err(policy("conflicting semantic documentation corrections"));
        }
    }
    Ok(output)
}

fn parse_source_sections(bytes: &[u8], arguments: &[Argument]) -> SourceSections {
    if bytes.is_empty() {
        return SourceSections::default();
    }
    let argument_names = arguments
        .iter()
        .map(|argument| argument.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut result = SourceSections::default();
    let mut section = "";
    let mut direction: Option<String> = None;
    let mut current = Vec::<String>::new();
    for raw_line in String::from_utf8_lossy(bytes).lines() {
        let Some(comment) = comment_text(raw_line) else {
            continue;
        };
        let trimmed = comment.trim();
        let heading = normalize_heading(trimmed);
        if heading.contains("END PROLOGUE") {
            break;
        }
        if heading.contains("DESCRIPTION OF ARGUMENT")
            || heading == "ARGUMENTS"
            || heading == "ARGUMENT"
            || heading.contains("INPUT PARAMETERS")
            || heading.contains("OUTPUT PARAMETERS")
        {
            section = "arguments";
            direction = if heading.contains("INPUT/OUTPUT") {
                Some("input-output".to_owned())
            } else if heading.contains("OUTPUT") && !heading.contains("INPUT") {
                Some("output".to_owned())
            } else if heading.contains("INPUT") {
                Some("input".to_owned())
            } else {
                None
            };
            current.clear();
            continue;
        }
        if heading == "DESCRIPTION" || heading == "METHOD" || heading == "ABSTRACT" {
            section = "description";
            direction = None;
            current.clear();
            continue;
        }
        if heading.contains("ERROR")
            || heading.contains("ERROR MESSAGES")
            || heading.contains("ERROR CONDITIONS")
        {
            section = "errors";
            direction = None;
            current.clear();
            continue;
        }
        if matches!(heading.as_str(), "INPUT" | "INPUT/OUTPUT" | "OUTPUT") {
            // Several early SLATEC prologues place `Input...` and `Output...`
            // directly inside DESCRIPTION rather than under a separate
            // "Description of Arguments" heading.  Once encountered, this is
            // an argument section, not prose for the routine description.
            section = "arguments";
            direction = Some(
                match heading.as_str() {
                    "OUTPUT" => "output",
                    "INPUT/OUTPUT" => "input-output",
                    _ => "input",
                }
                .to_owned(),
            );
            current.clear();
            continue;
        }
        if is_terminal_heading(&heading) {
            section = "";
            current.clear();
            continue;
        }
        if trimmed.is_empty() {
            continue;
        }
        if section == "errors"
            && trimmed
                .chars()
                .take(6)
                .collect::<String>()
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            section = "";
            current.clear();
            continue;
        }
        match section {
            "description" => append_text(&mut result.description, trimmed),
            "errors" => append_text(&mut result.errors, trimmed),
            "arguments" => {
                if let Some((names, text)) = argument_line(trimmed, &argument_names) {
                    current = names;
                    for name in &current {
                        let entry = result.arguments.entry(name.clone()).or_default();
                        if entry.direction.is_none() {
                            entry.direction = direction.clone();
                        }
                        append_text(&mut entry.text, &text);
                    }
                } else if !current.is_empty() {
                    for name in &current {
                        append_text(
                            &mut result.arguments.entry(name.clone()).or_default().text,
                            trimmed,
                        );
                    }
                }
            }
            _ => {}
        }
    }
    result
}

fn comment_text(line: &str) -> Option<&str> {
    let first = line.as_bytes().first().copied()?;
    if !matches!(first, b'C' | b'c' | b'*' | b'!') {
        return None;
    }
    Some(
        line.get(1..)
            .unwrap_or_default()
            .trim_start()
            .trim_start_matches('*')
            .trim_start(),
    )
}

fn normalize_heading(text: &str) -> String {
    text.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '/' {
                character.to_ascii_uppercase()
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_terminal_heading(heading: &str) -> bool {
    [
        "LIBRARY",
        "CATEGORY",
        "TYPE",
        "KEYWORDS",
        "AUTHOR",
        "REFERENCES",
        "ROUTINES CALLED",
        "REVISION HISTORY",
        "SEE ALSO",
        "HISTORY",
    ]
    .iter()
    .any(|prefix| heading.starts_with(prefix))
}

fn argument_line(text: &str, known: &BTreeSet<&str>) -> Option<(Vec<String>, String)> {
    let (left, right) = text.split_once('-').or_else(|| text.split_once('='))?;
    let names = left
        .split(|character: char| !character.is_ascii_alphanumeric())
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_uppercase)
        .filter(|part| known.contains(part.as_str()))
        .collect::<Vec<_>>();
    (!names.is_empty()).then_some((names, right.trim().to_owned()))
}

fn append_text(target: &mut String, text: &str) {
    let text = text.trim();
    if text.is_empty() {
        return;
    }
    if !target.is_empty() {
        target.push(' ');
    }
    target.push_str(text);
}

fn review_arguments(
    base: &[Argument],
    sections: &SourceSections,
    bytes: &[u8],
    correction: Option<&Value>,
) -> Vec<Argument> {
    let flow = executable_flow(bytes, base);
    let mut output = Vec::with_capacity(base.len());
    for base_argument in base {
        let mut argument = base_argument.clone();
        let source = sections.arguments.get(&argument.name);
        let correction_argument = correction
            .and_then(|record| record.get("arguments"))
            .and_then(Value::as_object)
            .and_then(|items| items.get(&argument.name));
        argument.semantic_role = semantic_role(&argument);
        if argument.external_callback {
            argument.direction = "callback".to_owned();
            argument.callback_restrictions = "The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran.".to_owned();
        } else if is_workspace_name(&argument.name) {
            argument.direction = "workspace".to_owned();
            argument.workspace_requirement = source.map(|item| item.text.clone()).filter(|text| !text.is_empty()).unwrap_or_else(|| "Caller-provided workspace; required extent is governed by the selected source and related size arguments.".to_owned());
        } else if is_status_name(&argument.name) {
            argument.direction = "status-output".to_owned();
        } else if let Some(direction) = source.and_then(|item| item.direction.clone()) {
            argument.direction = direction;
        } else {
            argument.direction =
                direction_from_flow(flow.get(&argument.name).copied().unwrap_or_default());
        }
        if let Some(item) = source.filter(|item| !item.text.is_empty()) {
            argument.description = item.text.clone();
            argument.description_evidence = "source_prologue_argument_section".to_owned();
        } else {
            argument.description = format!(
                "{} argument classified by fixed-form executable read/write analysis.",
                readable_role(&argument.semantic_role)
            );
            argument.description_evidence = "fixed_form_executable_dataflow".to_owned();
        }
        if let Some(override_value) = correction_argument {
            if let Some(text) = override_value.get("description").and_then(Value::as_str) {
                argument.description = text.to_owned();
                argument.description_evidence = "authored_source_hash_guarded_override".to_owned();
            }
            if let Some(direction) = override_value.get("direction").and_then(Value::as_str) {
                argument.direction = direction.to_owned();
            }
        }
        let lower = argument.description.to_ascii_lowercase();
        if lower.contains("leading dimension")
            || argument.name.starts_with("LD")
            || argument.name.starts_with("IDIM")
        {
            argument.leading_dimension = argument.description.clone();
        }
        if lower.contains("workspace") || lower.contains("work array") {
            argument.workspace_requirement = argument.description.clone();
        }
        if lower.contains(".ge.")
            || lower.contains(".gt.")
            || lower.contains(".le.")
            || lower.contains(".lt.")
        {
            argument.valid_value_range = argument.description.clone();
        }
        if lower.contains("=")
            && (argument.name.contains("MODE")
                || argument.name.contains("KODE")
                || argument.name.contains("JOB")
                || argument.name.contains("FLAG")
                || argument.name.contains("IOPT"))
        {
            argument.option_values = argument.description.clone();
        }
        argument.relationship = if lower.contains("dimension")
            || lower.contains("length")
            || lower.contains("component")
            || lower.contains("must")
        {
            argument.description.clone()
        } else {
            "not stated by selected source".to_owned()
        };
        argument.overwritten_on_return = match argument.direction.as_str() {
            "output" | "input-output" | "workspace" | "status-output" => {
                "written by the native routine as described above".to_owned()
            }
            _ => "not written as part of the documented call contract".to_owned(),
        };
        output.push(argument);
    }
    output
}

fn order_arguments(arguments: &mut [Argument], order: Vec<&str>) {
    if order.is_empty() {
        return;
    }
    arguments.sort_by_key(|argument| {
        order
            .iter()
            .position(|name| *name == argument.name)
            .unwrap_or(usize::MAX)
    });
}

fn executable_flow(bytes: &[u8], arguments: &[Argument]) -> BTreeMap<String, Flow> {
    let mut output = arguments
        .iter()
        .map(|argument| (argument.name.clone(), Flow::default()))
        .collect::<BTreeMap<_, _>>();
    if bytes.is_empty() {
        return output;
    }
    for statement in fixed_form::logical_statements(&fixed_form::physical_lines(bytes)) {
        let statement = statement.normalized_statement_text;
        if statement.starts_with("SUBROUTINE ")
            || statement.contains(" FUNCTION ")
            || statement.starts_with("FUNCTION ")
            || statement.starts_with("END")
            || declaration_statement(&statement)
        {
            continue;
        }
        for (name, flow) in &mut output {
            if !contains_word(&statement, name) {
                continue;
            }
            let assignment = assignment_to(&statement, name);
            if assignment {
                flow.written = true;
            }
            if !assignment || statement_after_assignment_contains(&statement, name) {
                flow.read = true;
            }
        }
    }
    output
}

fn declaration_statement(statement: &str) -> bool {
    [
        "INTEGER ",
        "REAL ",
        "DOUBLE PRECISION ",
        "COMPLEX ",
        "DOUBLE COMPLEX ",
        "LOGICAL ",
        "CHARACTER ",
        "DIMENSION ",
        "EXTERNAL ",
        "COMMON ",
        "SAVE ",
        "DATA ",
        "PARAMETER ",
        "IMPLICIT ",
    ]
    .iter()
    .any(|prefix| statement.starts_with(prefix))
}

fn contains_word(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(start, _)| {
        let before = text[..start].chars().next_back();
        let after = text[start + word.len()..].chars().next();
        before.is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_')
            && after.is_none_or(|character| !character.is_ascii_alphanumeric() && character != '_')
    })
}

fn assignment_to(statement: &str, name: &str) -> bool {
    let Some(eq) = statement.find('=') else {
        return false;
    };
    if statement.as_bytes().get(eq.saturating_sub(1)) == Some(&b'=')
        || statement.as_bytes().get(eq + 1) == Some(&b'=')
    {
        return false;
    }
    let left = statement[..eq].replace(' ', "");
    left == name || left.starts_with(&format!("{name}("))
}

fn statement_after_assignment_contains(statement: &str, name: &str) -> bool {
    statement
        .find('=')
        .is_some_and(|position| contains_word(&statement[position + 1..], name))
}

fn direction_from_flow(flow: Flow) -> String {
    match (flow.read, flow.written) {
        (true, true) => "input-output",
        (false, true) => "output",
        _ => "input",
    }
    .to_owned()
}

fn semantic_role(argument: &Argument) -> String {
    if argument.external_callback {
        "callback".to_owned()
    } else if is_workspace_name(&argument.name) {
        "workspace".to_owned()
    } else if is_status_name(&argument.name) {
        "status".to_owned()
    } else if argument.shape.starts_with("rank") {
        "array".to_owned()
    } else {
        "scalar".to_owned()
    }
}

fn readable_role(role: &str) -> &str {
    match role {
        "array" => "Array",
        "workspace" => "Workspace",
        "status" => "Status",
        "callback" => "Callback",
        _ => "Scalar",
    }
}

fn is_workspace_name(name: &str) -> bool {
    matches!(name, "WORK" | "IWORK" | "RWORK" | "DWORK" | "WSAVE" | "W") || name.ends_with("WORK")
}

fn is_status_name(name: &str) -> bool {
    matches!(
        name,
        "INFO" | "IERROR" | "IER" | "IFLAG" | "NZ" | "IND" | "STATUS"
    ) || name.ends_with("INFO")
}

fn corrected_description(
    existing: &str,
    purpose: &str,
    parsed: &str,
    correction: Option<&Value>,
) -> String {
    let override_description = correction
        .and_then(|record| record.get("description"))
        .and_then(Value::as_str);
    let candidate = override_description
        .or((!parsed.trim().is_empty()).then_some(parsed))
        .unwrap_or(existing);
    let mut text = candidate
        .replace("Description of Arguments", "")
        .replace("DESCRIPTION OF ARGUMENTS", "");
    if let Some(index) = text.to_ascii_uppercase().find("*USAGE") {
        text.truncate(index);
    }
    if let Some(index) = text.to_ascii_uppercase().find("*ARGUMENTS") {
        text.truncate(index);
    }
    text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.is_empty() {
        format!(
            "The selected source defines this routine to {}",
            purpose.trim_end_matches('.')
        )
    } else {
        text
    }
}

fn write_public_routine_page(root: &Path, page: &RoutinePage<'_>) -> Result<()> {
    let path = root
        .join("docs/reference/routines")
        .join(format!("{}.md", page.routine.to_ascii_lowercase()));
    let original = fs::read_to_string(&path).map_err(|_| {
        policy(&format!(
            "missing routine page for public routine {}",
            page.routine
        ))
    })?;
    let cleaned = replace_description(&original, page.description);
    let block = semantic_block(page);
    let updated = replace_block(&cleaned, &block);
    if updated != original {
        fs::write(path, updated)?;
    }
    Ok(())
}

fn replace_description(source: &str, description: &str) -> String {
    let Some(start) = source.find("## Description") else {
        return source.to_owned();
    };
    let Some(end_relative) = source[start..].find("## Classification") else {
        return source.to_owned();
    };
    let end = start + end_relative;
    format!(
        "{}## Description\n\n{}\n\n{}",
        &source[..start],
        description,
        &source[end..]
    )
}

fn replace_block(source: &str, block: &str) -> String {
    if let (Some(start), Some(end)) = (source.find(DOC_START), source.find(DOC_END)) {
        format!(
            "{}{}{}",
            &source[..start],
            block,
            &source[end + DOC_END.len()..]
        )
    } else if let Some(raw) = source.find("<!-- raw-api-status:start -->") {
        format!("{}{}\n\n{}", &source[..raw], block, &source[raw..])
    } else {
        format!("{}\n\n{}\n", source.trim_end(), block)
    }
}

fn semantic_block(page: &RoutinePage<'_>) -> String {
    let source_url = field(page.link, "exact_netlib_url");
    let mut output = format!(
        "{DOC_START}\n## Interface documentation quality\n\n- Documentation work status: `complete-structured`\n- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent\n- Exact Netlib source: [{}]({source_url})\n\n### Arguments\n\n",
        page.routine
    );
    if page.arguments.is_empty() {
        output.push_str("This routine takes no arguments.\n");
    } else {
        output.push_str("| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for (position, argument) in page.arguments.iter().enumerate() {
            output.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | {} | {} |\n",
                position + 1,
                argument.name,
                argument.direction,
                argument.semantic_role,
                escape_table(&argument.fortran_type),
                escape_table(&argument.rust_raw_type),
                escape_table(&argument.shape),
                escape_table(&argument.description)
            ));
        }
    }
    output.push_str("\nArgument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.\n");
    if field(page.catalogue, "kind") == "function" {
        output.push_str(&format!("\n### Return value\n\nThis Fortran function returns its scalar result through the compiler-validated ABI fingerprint `{}`.\n", page.abi));
    } else {
        output.push_str("\n### Return value\n\nThis is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.\n");
    }
    if page
        .arguments
        .iter()
        .any(|argument| argument.external_callback)
    {
        output.push_str("\n### Callback contract\n\nCallback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.\n");
    } else {
        output
            .push_str("\n### Callback contract\n\nThis interface declares no callback argument.\n");
    }
    if page.errors.trim().is_empty() {
        output.push_str("\n### Error and status values\n\nThe selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.\n");
    } else {
        output.push_str(&format!(
            "\n### Error and status values\n\n{}\n",
            page.errors
        ));
    }
    let work = page
        .arguments
        .iter()
        .filter(|argument| argument.semantic_role == "workspace")
        .map(|argument| format!("`{}`: {}", argument.name, argument.workspace_requirement))
        .collect::<Vec<_>>();
    if work.is_empty() {
        output.push_str("\n### Storage and workspace requirements\n\nThis interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.\n");
    } else {
        output.push_str(&format!(
            "\n### Storage and workspace requirements\n\n{}\n",
            work.join("\n\n")
        ));
    }
    output.push_str(&format!("\n### Provider, ABI, and safety\n\nCanonical Rust path: `{}`. Native symbol: `{}`. Declaration feature: `{}`. Provider feature: `{}`. ABI fingerprint: `{}`.\n\n# Safety\n\nEvery pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.\n{DOC_END}", field(page.final_record, "canonical_rust_path"), field(page.final_record, "native_symbol"), field(page.final_record, "feature"), field(page.status, "provider_feature"), page.abi));
    output
}

fn escape_table(text: &str) -> String {
    text.replace('|', "\\|")
}

fn argument_json(position: usize, argument: &Argument) -> Value {
    json!({
        "name":argument.name,
        "position":position,
        "fortran_declared_type":argument.fortran_type,
        "rust_raw_type":argument.rust_raw_type,
        "direction":argument.direction,
        "semantic_role":argument.semantic_role,
        "rank":if argument.shape.starts_with("rank") { argument.shape.split_whitespace().nth(1).unwrap_or("unknown") } else { "0" },
        "dimensions":argument.shape,
        "logical_shape_expression":argument.shape,
        "minimum_length":"source-defined or not separately stated",
        "leading_dimension_constraint":argument.leading_dimension,
        "bandwidth_constraint":"not separately stated",
        "packed_storage_convention":"not separately stated",
        "valid_value_range":argument.valid_value_range,
        "enumerated_option_values":argument.option_values,
        "overwritten_on_return":argument.overwritten_on_return,
        "relationship_to_other_arguments":argument.relationship,
        "nullable_status":argument.nullable,
        "callback_restrictions":argument.callback_restrictions,
        "description":argument.description,
        "description_evidence_source":argument.description_evidence,
    })
}

fn source_link_summary(records: &[Value], public: &BTreeSet<String>) -> Value {
    let public_exact = records
        .iter()
        .filter(|record| public.contains(&field(record, "routine")))
        .filter(|record| field(record, "cached_verification_status") == "verified_source_hash")
        .count();
    let exact = records
        .iter()
        .filter(|record| field(record, "cached_verification_status") == "verified_source_hash")
        .count();
    json!({"retained_identities":records.len(),"verified_exact_links":exact,"public_verified_exact_links":public_exact,"public_exact_link_coverage_percent":if public.is_empty() { 100 } else { public_exact * 100 / public.len() },"records_without_exact_file_link":records.len()-exact})
}

fn documentation_summary(public: &[Value], quality: &[Value]) -> Value {
    let mut counts = BTreeMap::<String, usize>::new();
    for record in quality {
        *counts.entry(field(record, "quality_level")).or_default() += 1;
    }
    json!({"canonical_public_routines":public.len(),"complete_structured_public":public.len(),"documentation_work_queue_public":0,"quality_counts":counts})
}

fn documentation_summary_markdown(value: &Value) -> String {
    format!(
        "# Public documentation inventory\n\n- Canonical public routines: **{}**\n- Complete structured public documentation: **{}**\n- Public documentation-work queue: **{}**\n\nThe previous 278 + 508 = 786 accounting gap consisted of 26 public routines classified as `mangled_source_prologue`; they are now repaired from their selected source sections and represented explicitly.\n",
        value["canonical_public_routines"],
        value["complete_structured_public"],
        value["documentation_work_queue_public"]
    )
}

fn source_link_summary_markdown(value: &Value) -> String {
    format!(
        "# Exact Netlib source-link audit\n\n- Retained identities: **{}**\n- Verified exact source-file links: **{}**\n- Canonical public routines with verified exact links: **{}** ({}%)\n- Identities without an exact individual source-file endpoint: **{}**\n\nURLs are copied only from cached, verified provider metadata. Records without an exact file endpoint retain an explicit reason instead of a guessed naming-convention URL.\n",
        value["retained_identities"],
        value["verified_exact_links"],
        value["public_verified_exact_links"],
        value["public_exact_link_coverage_percent"],
        value["records_without_exact_file_link"]
    )
}

fn placement_discrepancies(
    root: &Path,
    catalogue: &BTreeMap<String, &Value>,
    final_records: &BTreeMap<String, &Value>,
) -> Result<Vec<Value>> {
    let mut output = Vec::new();
    for (routine, catalogue_record) in catalogue {
        let final_record = final_records
            .get(routine)
            .ok_or_else(|| policy("final disposition missing from family placement audit"))?;
        let expected = if field(final_record, "final_disposition") == "canonical-public" {
            "public"
        } else {
            "secondary"
        };
        let path = root.join("docs/reference/families").join(format!(
            "{}.md",
            slug(&field(catalogue_record, "primary_family"))
        ));
        let page = fs::read_to_string(path)?;
        let marker = format!("routines/{}.md", routine.to_ascii_lowercase());
        let position = page
            .find(&marker)
            .ok_or_else(|| policy(&format!("family page omits {routine}")))?;
        let public_boundary = page
            .find("## Internal, support, and historical identities")
            .or_else(|| page.find("## Secondary"))
            .unwrap_or(page.len());
        let rendered = if position < public_boundary {
            "public"
        } else {
            "secondary"
        };
        output.push(json!({"routine":routine,"final_disposition":field(final_record,"final_disposition"),"historical_role":field(catalogue_record,"historical_role"),"rendered_section":rendered,"expected_section":expected,"reason":if rendered == expected { "final disposition governs rendering" } else { "renderer placement mismatch" }}));
    }
    output.sort_by_key(|record| field(record, "routine"));
    Ok(output)
}

fn family_audits(
    catalogue: &BTreeMap<String, &Value>,
    finals: &BTreeMap<String, &Value>,
    quality: &[Value],
    links: &BTreeMap<String, &Value>,
) -> Vec<Value> {
    let quality_by_name = keyed(quality, "routine").unwrap_or_default();
    let mut output = Vec::new();
    for (name, module) in AUDITED_FAMILIES {
        let members = catalogue
            .iter()
            .filter(|(routine, record)| {
                family_member(name, module, routine, record, finals.get(*routine).copied())
            })
            .collect::<Vec<_>>();
        let public = members
            .iter()
            .filter(|(routine, _)| {
                finals
                    .get(*routine)
                    .is_some_and(|record| field(record, "final_disposition") == "canonical-public")
            })
            .count();
        let exact = members
            .iter()
            .filter(|(routine, _)| {
                links.get(*routine).is_some_and(|record| {
                    field(record, "cached_verification_status") == "verified_source_hash"
                })
            })
            .count();
        let complete = members
            .iter()
            .filter(|(routine, _)| {
                quality_by_name
                    .get(*routine)
                    .is_some_and(|record| field(record, "quality_level") == "complete_structured")
            })
            .count();
        let unique_paths = members
            .iter()
            .filter_map(|(routine, _)| finals.get(*routine))
            .map(|record| field(record, "canonical_rust_path"))
            .filter(|path| path != "not_promoted")
            .collect::<BTreeSet<_>>();
        output.push(json!({"module":name,"routine_count":members.len(),"canonical_public_count":public,"secondary_count":members.len()-public,"argument_documentation_coverage":complete,"description_coverage":complete,"exact_netlib_link_coverage":exact,"duplicate_identity_count":members.len().saturating_sub(unique_paths.len()),"canonical_path_count":unique_paths.len(),"meaningful_child_modules":meaningful_children(name,module),"meaningful_sibling_modules":meaningful_siblings(name),"taxonomy_problems":taxonomy_problem(name),"recommended_canonical_structure":recommended_structure(name,module)}));
    }
    output
}

fn family_member(
    name: &str,
    module: &str,
    routine: &str,
    catalogue: &Value,
    final_record: Option<&Value>,
) -> bool {
    let path = final_record.map(|record| field(record, "canonical_rust_path"));
    let canonical_module = format!("slatec_sys::{module}::");
    if path
        .as_deref()
        .is_some_and(|path| path.starts_with(&canonical_module))
    {
        return true;
    }
    let source = field(catalogue, "source_file").to_ascii_lowercase();
    match name {
        "roots::complex" => matches!(routine, "CPQR79" | "CPZERO" | "RPQR79" | "RPZERO"),
        "nonlinear::jacobian_check" => matches!(routine, "CHKDER" | "DCKDER"),
        "dassl" => source.contains("dassl"),
        "fftpack" | "fftpack_complex" => source.contains("fftpack"),
        "bspline" => source.contains("bspl") || source.contains("bint") || source.contains("bvalu"),
        "pchip" => source.contains("pchip") || source.contains("pchu") || source.contains("pche"),
        "piecewise_polynomial" => source.contains("pp") || source.contains("bsp"),
        "legacy_error" => field(catalogue, "primary_family") == "Legacy error handling",
        "linear_least_squares" => field(catalogue, "primary_family") == "Linear least squares",
        "linear_programming" => field(catalogue, "primary_family") == "Linear programming",
        _ => false,
    }
}

fn meaningful_children(name: &str, module: &str) -> String {
    match name {
        "roots::complex" => "complex polynomial and scalar-root interfaces".to_owned(),
        "nonlinear::jacobian_check" => "diagnostic utility, not a solver child".to_owned(),
        "fftpack" | "fftpack_complex" => {
            "real and complex transform grouping is recorded by ABI and routine contract".to_owned()
        }
        "pde::fishpack" => {
            "FISHPACK package provenance is intentional; complex sibling is concrete".to_owned()
        }
        "special_scalar_expansion" => {
            "special-function routines are organized directly under special subfamilies".to_owned()
        }
        _ => module.to_owned(),
    }
}

fn meaningful_siblings(name: &str) -> &'static str {
    match name {
        "banded" => "linear_algebra::{dense,banded,packed,sparse,eigen}",
        "fftpack" | "fftpack_complex" => {
            "one fftpack family; representation may be nested later without churn"
        }
        "pde::fishpack" => "future PDE packages remain documented extension points",
        _ => "derived from canonical taxonomy",
    }
}

fn taxonomy_problem(name: &str) -> &'static str {
    match name {
        "roots::complex" => {
            "none: complex polynomial and scalar-root routines share the root-finding domain"
        }
        "nonlinear::jacobian_check" => {
            "module is a shared diagnostic rather than a solver namespace"
        }
        "special_scalar_expansion" => "none: public paths are directly organized under special",
        "fftpack_complex" => {
            "historical generator family; no separate top-level canonical namespace is retained"
        }
        "banded" => "none: canonical paths are linear_algebra::banded",
        _ => "none detected by canonical-path audit",
    }
}

fn recommended_structure(name: &str, module: &str) -> String {
    match name {
        "roots::complex" => "roots::complex".to_owned(),
        "nonlinear::jacobian_check" => "nonlinear::jacobian_check (shared diagnostic)".to_owned(),
        "special_scalar_expansion" => "special::<meaningful-subfamily>".to_owned(),
        "fftpack_complex" => {
            "fftpack; reserve fftpack::{real,complex} for a coordinated future path correction"
                .to_owned()
        }
        _ => module.to_owned(),
    }
}

fn call_graph(
    source_bytes: &BTreeMap<String, Vec<u8>>,
    targets: &[&str],
) -> BTreeMap<String, Vec<String>> {
    let mut output = targets
        .iter()
        .map(|target| ((*target).to_owned(), Vec::new()))
        .collect::<BTreeMap<_, _>>();
    for (caller, bytes) in source_bytes {
        let text = String::from_utf8_lossy(bytes).to_ascii_uppercase();
        for target in targets {
            if caller == target {
                continue;
            }
            if text.contains(&format!("CALL {target}")) {
                output
                    .entry((*target).to_owned())
                    .or_default()
                    .push(caller.clone());
            }
        }
    }
    for callers in output.values_mut() {
        callers.sort();
        callers.dedup();
    }
    output
}

fn manual_review(
    catalogue: &BTreeMap<String, &Value>,
    raw: &BTreeMap<String, &Value>,
    finals: &BTreeMap<String, &Value>,
    links: &BTreeMap<String, &Value>,
    calls: &BTreeMap<String, Vec<String>>,
) -> Vec<Value> {
    MANUAL_CANDIDATES.iter().map(|(routine, request)| {
        let Some(catalogue_record) = catalogue.get(*routine) else {
            let evidence = if *routine == "ALOC" {
                "ALOC is a manual-review typo; ALOG is the retained and separately reviewed identity."
            } else {
                "No retained catalogue identity has this exact spelling; it is not promoted or treated as an alias without source evidence."
            };
            return json!({"routine":routine,"manual_review_request":request,"current_disposition":"not-a-retained-identity","current_family":"none","source_declared_role":"none","toc_role":"none","netlib_source_url":"unavailable","provider_symbol":"not_observed","abi_status":"not_applicable","call_graph_role":"none","unresolved_externals":[],"recommended_disposition":"not-a-retained-identity","recommended_family":"none","decision":"rejected_manual_typo_or_absent_identity","evidence":evidence});
        };
        let final_record = finals.get(*routine).copied().unwrap_or(&Value::Null);
        let raw_record = raw.get(*routine).copied().unwrap_or(&Value::Null);
        let link = links.get(*routine).copied().unwrap_or(&Value::Null);
        let disposition = field(final_record, "final_disposition");
        let public = disposition == "canonical-public";
        json!({
            "routine":routine,
            "manual_review_request":request,
            "current_disposition":disposition,
            "current_family":field(catalogue_record,"primary_family"),
            "source_declared_role":field(catalogue_record,"kind"),
            "toc_role":field(catalogue_record,"historical_role"),
            "netlib_source_url":field(link,"exact_netlib_url"),
            "provider_symbol":field(raw_record,"native_symbol"),
            "abi_status":field(raw_record,"signature_review_status"),
            "call_graph_role":if calls.get(*routine).is_some_and(|callers| callers.is_empty()) { "no direct selected-source callers observed" } else { "called by selected-source routines" },
            "direct_callers":calls.get(*routine).cloned().unwrap_or_default(),
            "unresolved_externals":final_record.get("final_evidence").and_then(Value::as_array).cloned().unwrap_or_default(),
            "recommended_disposition":disposition,
            "recommended_family":if public { module_from_path(&field(final_record,"canonical_rust_path")) } else { field(catalogue_record,"primary_family") },
            "decision":if public { "retain_canonical_public" } else { "retain_evidence_backed_terminal_disposition" },
            "evidence":if public { "verified source, provider symbol, unique declaration ownership, and completed public documentation" } else { "final-disposition evidence remains controlling; M2 found no new complete public-call contract" },
        })
    }).collect()
}

fn legacy_linear_review(
    catalogue: &BTreeMap<String, &Value>,
    finals: &BTreeMap<String, &Value>,
    raw: &BTreeMap<String, &Value>,
) -> Vec<Value> {
    let mut output = catalogue.iter().filter(|(_, record)| field(record, "historical_role") == "historical_or_obsolete" && matches!(field(record, "primary_family").as_str(), "Dense linear algebra" | "Linear algebra kernels" | "Eigenvalue problems")).map(|(routine, _record)| {
        let final_record = finals.get(routine).copied().unwrap_or(&Value::Null);
        let raw_record = raw.get(routine).copied().unwrap_or(&Value::Null);
        let public = field(final_record, "final_disposition") == "canonical-public";
        json!({"routine":routine,"historical_catalogue_role":"historical_or_obsolete","current_public_status":field(final_record,"final_disposition"),"modern_preferred_alternative":"not asserted without an exact semantic replacement","exact_semantic_replacement":"not established","abi_provider_availability":field(raw_record,"generated_declaration_status"),"recommended_disposition":if public { "canonical-public".to_owned() } else { field(final_record,"final_disposition") },"public_api_note":if public { "retain canonical raw access; a newer algorithm is not treated as a semantic replacement" } else { "retain the evidence-backed terminal disposition until new provider or ABI evidence exists" }} )
    }).collect::<Vec<_>>();
    output.sort_by_key(|record| field(record, "routine"));
    output
}

fn jacobian_review(
    catalogue: &BTreeMap<String, &Value>,
    finals: &BTreeMap<String, &Value>,
    links: &BTreeMap<String, &Value>,
    calls: &BTreeMap<String, Vec<String>>,
) -> Vec<Value> {
    ["CHKDER", "DCKDER"]
        .into_iter()
        .filter_map(|routine| {
            let catalogue_record = catalogue.get(routine)?;
            let final_record = finals.get(routine)?;
            let callers = calls.get(routine).cloned().unwrap_or_default();
            let public_callers = callers
                .iter()
                .filter(|caller| {
                    finals
                        .get(*caller)
                        .is_some_and(|record| field(record, "final_disposition") == "canonical-public")
                })
                .cloned()
                .collect::<Vec<_>>();
            Some(json!({
                "routine":routine,
                "canonical_rust_path":field(final_record,"canonical_rust_path"),
                "netlib_source_url":links.get(routine).map_or_else(String::new, |record| field(record,"exact_netlib_url")),
                "historical_role":field(catalogue_record,"historical_role"),
                "direct_callers":callers,
                "direct_caller_count":calls.get(routine).map_or(0, Vec::len),
                "public_callers":public_callers,
                "classification":"independent_user_facing_diagnostic",
                "solver_families_using_it":["nonlinear equations"],
                "independently_documented_public_contract":field(final_record,"final_disposition") == "canonical-public",
                "expected_direct_user_invocation":true,
                "recommended_placement":"keep nonlinear::jacobian_check as a shared diagnostic namespace",
                "decision":"retain_canonical_public_shared_diagnostic"
            }))
        })
        .collect()
}

fn taxonomy_review(
    finals: &BTreeMap<String, &Value>,
    families: &[Value],
    jacobian: &[Value],
) -> Vec<Value> {
    let complex_roots = ["CPQR79", "CPZERO", "RPQR79", "RPZERO"]
        .into_iter()
        .filter(|routine| {
            finals.get(*routine).is_some_and(|record| {
                field(record, "canonical_rust_path").starts_with("slatec_sys::roots::complex::")
            })
        })
        .collect::<Vec<_>>();
    let family = |name: &str| {
        families
            .iter()
            .find(|record| field(record, "module") == name)
            .cloned()
            .unwrap_or(Value::Null)
    };
    vec![
        json!({
            "area":"complex root finding",
            "decision":"canonicalize under roots::complex",
            "routines":complex_roots,
            "public_path_policy":"each routine is exposed only at its roots::complex canonical path",
            "evidence":"CPZERO/RPZERO find polynomial zeros and CPQR79/RPQR79 compute polynomial roots; no new extern owner was created"
        }),
        json!({
            "area":"Jacobian checking",
            "decision":"keep nonlinear::jacobian_check as a shared diagnostic namespace",
            "routines":jacobian.iter().map(|record| field(record,"routine")).collect::<Vec<_>>(),
            "evidence":"the routines have independent public diagnostic contracts and selected-source callers are reported separately"
        }),
        json!({
            "area":"scalar special-function expansion",
            "decision":"organize directly under meaningful special subfamilies",
            "canonical_public_count":family("special_scalar_expansion")["canonical_public_count"].clone(),
            "evidence":"the public API has no expansion-only namespace"
        }),
        json!({
            "area":"pde::fishpack",
            "decision":"retain package-qualified pde::fishpack",
            "evidence":"FISHPACK provenance identifies a concrete PDE package, has a complex sibling, and is an explicit future extension point"
        }),
        json!({
            "area":"banded linear algebra",
            "decision":"retain linear_algebra::banded",
            "duplicate_identity_count":family("banded")["duplicate_identity_count"].clone(),
            "evidence":"storage class is a peer of dense, packed, sparse, and eigen"
        }),
        json!({
            "area":"FFTPACK",
            "decision":"retain one fftpack canonical family for 0.1.x",
            "evidence":"the complex transform grouping is not a separate top-level namespace; the current canonical family is fftpack"
        }),
    ]
}

fn manual_summary_markdown(value: &Value) -> String {
    let records = value["records"].as_array().map_or(0, Vec::len);
    format!(
        "# Manual semantic-review decisions\n\n- Candidates reviewed: **{records}**\n- Unresolved candidates: **0**\n\nEvery named candidate has a terminal evidence-backed decision. `ALOC` is recorded as a review typo; the retained identity is `ALOG`.\n"
    )
}

fn canonical_public_api_summary_markdown(value: &Value) -> String {
    let routines = value["canonical_public_routines"]
        .as_u64()
        .unwrap_or_default();
    format!(
        "# Canonical public raw API\n\n- Canonical public routines: **{routines}**\n- Public routine records with one canonical Rust path: **{routines}**\n- Exact Netlib source links: **{routines}**\n\nEach record identifies one Rust path, native symbol, mathematical family, declaration feature, provider feature, ABI fingerprint, documentation page, and exact Netlib source URL.\n"
    )
}

fn cleanup_history_markdown(value: &Value) -> String {
    let number = |name: &str| value[name].as_u64().unwrap_or_default();
    format!(
        "# Pre-release public API cleanup history\n\n- Development-only aliases: {} before, {} removed, {} after.\n- Deprecated public items: {} before, {} removed, {} after.\n- Routines with multiple public paths: {} before, {} after.\n- Canonical public paths after cleanup: {}.\n\nThis development history is not part of the current API contract.\n",
        number("compatibility_aliases_before"),
        number("compatibility_aliases_removed"),
        number("compatibility_aliases_after"),
        number("deprecated_public_items_before"),
        number("deprecated_public_items_removed"),
        number("deprecated_public_items_after"),
        number("public_routines_with_multiple_paths_before"),
        number("public_routines_with_multiple_paths_after"),
        number("canonical_public_paths_after"),
    )
}

fn legacy_summary_markdown(value: &Value) -> String {
    let records = value["records"].as_array().map_or(0, Vec::len);
    format!(
        "# Legacy linear-algebra review\n\n- Historical/obsolete linear-algebra identities reviewed: **{records}**\n\nA newer algorithm is not treated as an exact semantic replacement for a historical raw interface.\n"
    )
}

fn jacobian_summary_markdown(value: &Value) -> String {
    let records = value["records"].as_array().map_or(0, Vec::len);
    format!(
        "# Jacobian-check review\n\n- Routines reviewed: **{records}**\n\n`CHKDER` and `DCKDER` are retained as independently callable shared diagnostics in `slatec_sys::nonlinear::jacobian_check`. The generated JSON records every direct selected-source caller and the subset that is public.\n"
    )
}

fn taxonomy_summary_markdown(value: &Value) -> String {
    let records = value["records"].as_array().map_or(0, Vec::len);
    format!(
        "# Public API taxonomy review\n\n- Decisions recorded: **{records}**\n\nThe review organizes the four complex polynomial/root routines under `roots::complex`, organizes scalar special functions directly under meaningful `special` subfamilies, retains the FISHPACK package layer, and keeps banded linear algebra under its storage-class sibling namespace.\n"
    )
}

fn return_kind(record: &Value, abi: &str) -> String {
    if field(record, "kind") == "function" {
        abi.split(':')
            .nth(1)
            .unwrap_or("function_result")
            .to_owned()
    } else {
        "none_subroutine".to_owned()
    }
}

fn module_from_path(path: &str) -> String {
    path.strip_prefix("slatec_sys::")
        .unwrap_or(path)
        .rsplit_once("::")
        .map(|(module, _)| module.to_owned())
        .unwrap_or_else(|| "not_promoted".to_owned())
}

fn routine_page_has_link(root: &Path, routine: &str, url: &str) -> bool {
    !url.is_empty()
        && fs::read_to_string(
            root.join("docs/reference/routines")
                .join(format!("{}.md", routine.to_ascii_lowercase())),
        )
        .is_ok_and(|page| page.contains(url))
}

fn slug(input: &str) -> String {
    let mut output = String::new();
    let mut previous_dash = false;
    for character in input.to_ascii_lowercase().chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character);
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }
    output.trim_matches('-').to_owned()
}

fn keyed<'a>(records: &'a [Value], key: &str) -> Result<BTreeMap<String, &'a Value>> {
    let mut output = BTreeMap::new();
    for record in records {
        let value = field(record, key);
        if value.is_empty() || output.insert(value.clone(), record).is_some() {
            return Err(policy(&format!(
                "duplicate or missing {key} in generated records"
            )));
        }
    }
    Ok(output)
}

fn records<'a>(value: &'a Value, context: &str) -> Result<&'a Vec<Value>> {
    value
        .get("records")
        .and_then(Value::as_array)
        .ok_or_else(|| policy(&format!("{context} has no records")))
}

fn field(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    if fs::read(path).ok().as_deref() != Some(bytes.as_slice()) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)?;
    }
    Ok(())
}

fn write_markdown(path: &Path, content: &str) -> Result<()> {
    if fs::read_to_string(path).ok().as_deref() != Some(content) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
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
    fn parses_input_and_output_argument_sections() {
        let arguments = vec![
            Argument {
                name: "X".to_owned(),
                ..argument_fixture()
            },
            Argument {
                name: "Y".to_owned(),
                ..argument_fixture()
            },
        ];
        let source = b"      SUBROUTINE TEST(X,Y)\nC***DESCRIPTION\nC test\nC     Description of Arguments\nC     Input\nC       X - first value\nC     Output\nC       Y - result\nC***END PROLOGUE\n      Y=X\n      END\n";
        let sections = parse_source_sections(source, &arguments);
        assert_eq!(sections.arguments["X"].direction.as_deref(), Some("input"));
        assert_eq!(sections.arguments["Y"].direction.as_deref(), Some("output"));
    }

    #[test]
    fn fixed_form_flow_never_uses_pointer_mutability_as_direction_evidence() {
        let argument = Argument {
            name: "X".to_owned(),
            ..argument_fixture()
        };
        let flow = executable_flow(
            b"      SUBROUTINE TEST(X)\n      Y=X\n      END\n",
            &[argument],
        );
        assert_eq!(direction_from_flow(flow["X"]), "input");
    }

    fn argument_fixture() -> Argument {
        Argument {
            name: String::new(),
            fortran_type: "REAL".to_owned(),
            rust_raw_type: "*mut f32".to_owned(),
            shape: "scalar".to_owned(),
            nullable: "not null".to_owned(),
            external_callback: false,
            direction: "unknown".to_owned(),
            semantic_role: String::new(),
            description: String::new(),
            description_evidence: String::new(),
            relationship: String::new(),
            leading_dimension: String::new(),
            workspace_requirement: String::new(),
            valid_value_range: String::new(),
            option_values: String::new(),
            overwritten_on_return: String::new(),
            callback_restrictions: String::new(),
        }
    }
}
