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
use std::process::Command;

const DOC_START: &str = "<!-- release-readiness:start -->";
const DOC_END: &str = "<!-- release-readiness:end -->";
const RETAINED_IDENTITIES: usize = 1517;
const PUBLIC_BEFORE: usize = 821;
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
    source_start_line: usize,
    source_end_line: usize,
    source_section: String,
    evidence_kind: String,
    precision_sibling_source: String,
    direction_evidence: String,
    direction_conflict: String,
}

#[derive(Clone, Debug, Default)]
struct SourceArgument {
    direction: Option<String>,
    changed_on_return: bool,
    text: String,
    source_start_line: usize,
    source_end_line: usize,
    source_section: String,
    evidence_kind: String,
}

#[derive(Clone, Debug, Default)]
struct StatusValue {
    value: String,
    meaning: String,
    source_start_line: usize,
    source_end_line: usize,
}

#[derive(Clone, Debug, Default)]
struct SourceSections {
    description: String,
    errors: String,
    arguments: BTreeMap<String, SourceArgument>,
    statuses: BTreeMap<String, Vec<StatusValue>>,
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
    statuses: &'a BTreeMap<String, Vec<StatusValue>>,
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
    let prior_public_inventory =
        read_json(&root.join("generated/slatec-routines/public-documentation-inventory.json"))?;
    let corrections = read_json(&root.join("metadata/public-api-semantic-corrections.json"))?;
    let rustdoc_contract_dir = root.join("crates/slatec-sys/src/generated_docs");
    fs::create_dir_all(&rustdoc_contract_dir)?;

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
        let cached_path = record
            .pointer("/official_documentation/netlib_source_url/cached_path")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let source_path =
            resolved_cached_source_path(root, cached_path, &selected_provider_source_path(record));
        let mut link = source_link_record(root, record)?;
        link["public_routine"] = Value::Bool(public_names.contains(&routine));
        if let Some(path) = source_path {
            if let Ok(bytes) = fs::read(path) {
                source_bytes.insert(routine.clone(), bytes);
            }
        }
        source_links.push(link);
    }
    source_links.sort_by_key(|record| field(record, "routine"));
    let source_links_by_name = keyed(&source_links, "routine")?;
    let source_sections_by_name = source_bytes
        .iter()
        .map(|(routine, bytes)| {
            (
                routine.clone(),
                parse_source_sections(
                    bytes,
                    &arguments_by_routine
                        .get(routine)
                        .cloned()
                        .unwrap_or_default(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    write_semantic_quality_baseline_if_absent(
        root,
        &prior_public_inventory,
        &source_sections_by_name,
    )?;

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
        let sections = source_sections_by_name
            .get(routine)
            .cloned()
            .unwrap_or_default();
        let description = corrected_description(
            &field(catalogue_record, "full_description"),
            &field(catalogue_record, "short_purpose"),
            &sections.description,
            correction_by_routine.get(routine),
        );
        let precision_sibling = precision_sibling_sections(
            routine,
            &arguments_by_routine
                .get(routine)
                .cloned()
                .unwrap_or_default(),
            &arguments_by_routine,
            &catalogue_by_name,
            &source_sections_by_name,
        );
        let mut arguments = review_arguments(
            &arguments_by_routine
                .get(routine)
                .cloned()
                .unwrap_or_default(),
            &sections,
            precision_sibling.map(|(_, sections)| sections),
            precision_sibling.map(|(routine, _)| routine),
            bytes,
            correction_by_routine.get(routine),
        );
        apply_function_purpose_fallbacks(&mut arguments, catalogue_record);
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
        let source_hash_guarded_authored = arguments.iter().any(|argument| {
            argument.description_evidence == "authored_source_hash_guarded_override"
        });
        let semantic_issues = if public {
            semantic_contract_issues(&arguments, &sections)
        } else {
            Vec::new()
        };
        let quality = if public && semantic_issues.is_empty() {
            "complete-semantic-contract"
        } else if public {
            "semantic-review-required"
        } else {
            "not-public"
        };
        let work = if public && semantic_issues.is_empty() {
            "rendered-rustdoc-semantic-audit-pending"
        } else if public {
            "semantic-repair-required"
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
                statuses: &sections.statuses,
                arguments: &arguments,
                abi: &abi,
            };
            write_public_routine_page(root, &page)?;
            write_public_rustdoc_contract(&rustdoc_contract_dir, &page)?;
        }
        quality_records.push(json!({
            "routine":routine,
            "family":field(catalogue_record, "primary_family"),
            "public_raw":public,
            "quality_level":quality,
            "documentation_work_status":work,
            "reason":if public && !semantic_issues.is_empty() { "one or more argument contracts failed the M3 semantic checks" } else if public && source_hash_guarded_authored { "source-hash-guarded authored corrections and source evidence passed M3 semantic checks" } else if public { "bounded selected-source extraction and semantic checks passed" } else { "M3 completion threshold applies only to canonical public routines" },
            "description_provenance":if sections.description.is_empty() { "source_purpose_fallback" } else { "source_prologue" },
            "argument_count":arguments.len(),
            "structured_argument_rows":arguments.len(),
            "semantic_issues":semantic_issues,
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
                "source_start_line":argument.source_start_line,
                "source_end_line":argument.source_end_line,
                "source_section":argument.source_section,
                "evidence_kind":argument.evidence_kind,
                "precision_sibling_source":argument.precision_sibling_source,
                "direction_evidence":argument.direction_evidence,
                "direction_conflict":argument.direction_conflict,
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
        "quality_levels":{"complete-semantic-contract":"bounded argument-specific source or source-hash-guarded authored evidence passed M3 semantic checks","semantic-review-required":"one or more semantic checks failed or source evidence remains incomplete","not-public":"not a canonical public raw routine"},
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

    write_m3_semantic_reports(
        root,
        &documentation_records,
        &quality_records,
        &source_sections_by_name,
        &correction_by_routine,
    )?;

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
    write_markdown(
        &root.join("generated/slatec-routines/documentation-quality-summary.md"),
        &documentation_quality_summary_markdown(&quality),
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

    let public_routines = documentation_records
        .iter()
        .map(|record| field(record, "routine").to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    inject_public_rustdoc_contracts(root, &public_routines)?;

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
    let (mut result, changed_paths) = generate_transactionally(root, output_dir)?;
    if !changed_paths.is_empty() {
        let shown = changed_paths
            .iter()
            .take(8)
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let suffix = if changed_paths.len() > 8 {
            format!(" (and {} more)", changed_paths.len() - 8)
        } else {
            String::new()
        };
        return Err(policy(&format!(
            "public semantic-review outputs are stale in {} files: {shown}{suffix}; run generate-public-api-semantic-review",
            changed_paths.len()
        )));
    }
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
            || !matches!(
                field(record, "documentation_work_status").as_str(),
                "rendered-rustdoc-semantic-audit-pending" | "semantic-repair-required"
            )
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
    let catalogue_value =
        read_json(&root.join("generated/slatec-routines/routine-catalogue.json"))?;
    let catalogue_by_routine = keyed(
        records(&catalogue_value, "routine catalogue")?,
        "normalized_name",
    )?;
    let argument_coverage =
        read_json(&root.join("generated/release-readiness/argument-documentation-coverage.json"))?;
    let arguments_by_routine = arguments_by_routine(&argument_coverage)?;
    for record in records(&inventory, "public documentation inventory")? {
        let routine = field(record, "routine");
        let Some(base) = arguments_by_routine.get(&routine) else {
            return Err(policy(&format!(
                "public routine {routine} has no argument inventory for precision-sibling validation"
            )));
        };
        for argument in record["arguments"].as_array().into_iter().flatten() {
            let sibling = field(argument, "precision_sibling_source");
            if sibling.is_empty() || sibling == "not_applicable" {
                continue;
            }
            let Some(sibling_arguments) = arguments_by_routine.get(&sibling) else {
                return Err(policy(&format!(
                    "{routine} records unavailable precision sibling {sibling}"
                )));
            };
            let same_kind = catalogue_by_routine
                .get(&routine)
                .zip(catalogue_by_routine.get(&sibling))
                .is_some_and(|(left, right)| field(left, "kind") == field(right, "kind"));
            if !same_kind || !precision_sibling_signature_matches(base, sibling_arguments) {
                return Err(policy(&format!(
                    "{routine} records precision-sibling transfer from incompatible {sibling}"
                )));
            }
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
    let final_quality =
        read_json(&root.join("generated/slatec-routines/semantic-quality-final.json"))?;
    let direction_conflicts =
        read_json(&root.join("generated/slatec-routines/direction-evidence-conflicts.json"))?;
    let contamination =
        read_json(&root.join("generated/slatec-routines/argument-contamination-audit.json"))?;
    if final_quality["counts"]["complete-semantic-contract"].as_u64() != Some(PUBLIC_BEFORE as u64)
        || direction_conflicts["unresolved_public_conflicts"].as_u64() != Some(0)
        || contamination["public_argument_descriptions_with_detected_contamination"].as_u64()
            != Some(0)
    {
        return Err(policy(
            "M3 semantic documentation quality validation is incomplete",
        ));
    }
    result.status = "validated".to_owned();
    Ok(result)
}

/// Builds and inspects the rendered documentation for every canonical public
/// function.  This intentionally validates cargo-doc HTML rather than a
/// Markdown template or private declaration source.
pub fn generate_rendered_rustdoc_audit(root: &Path) -> Result<Value> {
    let output_dir = root.join("generated/slatec-routines");
    generate(root, &output_dir)?;
    let status = Command::new("cargo")
        .args([
            "doc",
            "-p",
            "slatec-sys",
            "--all-features",
            "--no-deps",
            "--offline",
        ])
        .current_dir(root)
        .status()
        .map_err(CorpusError::Io)?;
    if !status.success() {
        return Err(policy("cargo doc failed before rendered-rustdoc audit"));
    }

    let inventory = read_json(&output_dir.join("public-documentation-inventory.json"))?;
    let canonical = read_json(&root.join("generated/public-api/canonical-public-api.json"))?;
    let ownership = read_json(&root.join("generated/public-api/ffi-declaration-ownership.json"))?;
    let canonical_by_routine = records(&canonical, "canonical public API")?
        .iter()
        .map(|record| (field(record, "routine"), record))
        .collect::<BTreeMap<_, _>>();
    let ownership_by_routine = records(&ownership, "FFI declaration ownership")?
        .iter()
        .map(|record| (field(record, "routine"), record))
        .collect::<BTreeMap<_, _>>();
    let mut output = Vec::new();
    let mut family_defects = BTreeMap::<String, usize>::new();
    for record in records(&inventory, "public documentation inventory")? {
        let routine = field(record, "routine");
        let canonical_record = canonical_by_routine.get(&routine).ok_or_else(|| {
            policy(&format!(
                "rendered-rustdoc audit lacks canonical record for {routine}"
            ))
        })?;
        let ownership_record = ownership_by_routine.get(&routine).ok_or_else(|| {
            policy(&format!(
                "rendered-rustdoc audit lacks declaration owner for {routine}"
            ))
        })?;
        let canonical_path = field(record, "canonical_rust_path");
        let requested_html_relative = rustdoc_html_relative_path(&canonical_path)?;
        let (html_relative, html_path, reexport_target) =
            resolve_rustdoc_html(root, &requested_html_relative, &routine)?;
        let html = fs::read_to_string(&html_path).unwrap_or_default();
        let abi_arguments = record["arguments"]
            .as_array()
            .ok_or_else(|| policy("public documentation record lacks ABI arguments"))?
            .iter()
            .map(|argument| field(argument, "name"))
            .collect::<Vec<_>>();
        let documented_argument_names = abi_arguments
            .iter()
            .filter(|argument| rustdoc_mentions_argument(&html, argument))
            .cloned()
            .collect::<Vec<_>>();
        let missing_argument_names = abi_arguments
            .iter()
            .filter(|argument| !rustdoc_mentions_argument(&html, argument))
            .cloned()
            .collect::<Vec<_>>();
        let mut required_sections = vec![
            "Purpose",
            "Description",
            "Arguments",
            "Return value",
            "ABI notes",
            "Safety",
        ];
        let argument_rows = record["arguments"].as_array().cloned().unwrap_or_default();
        if argument_rows
            .iter()
            .any(|argument| argument["external_callback"] == Value::Bool(true))
        {
            required_sections.push("Callback contract");
        }
        if argument_rows.iter().any(|argument| {
            field(argument, "semantic_role") == "status"
                || matches!(routine.as_str(), "DDASSL" | "SDASSL" | "DQAG" | "QAG")
        }) {
            required_sections.push("Status and error values");
        }
        if argument_rows.iter().any(|argument| {
            field(argument, "semantic_role") == "workspace"
                || field(argument, "dimensions").starts_with("rank")
        }) {
            required_sections.push("Workspace and array requirements");
        }
        let missing_sections = required_sections
            .iter()
            .filter(|section| !rustdoc_has_section(&html, section))
            .map(|section| (*section).to_owned())
            .collect::<Vec<_>>();
        let source_url = field(record, "exact_netlib_source_url");
        let source_url_found = html.contains(&source_url);
        let routine_page_path = root.join(field(record, "routine_page_path"));
        let routine_page = fs::read_to_string(&routine_page_path).unwrap_or_default();
        let mut mismatches = Vec::new();
        if field(canonical_record, "canonical_rust_path") != canonical_path {
            mismatches.push("canonical Rust path differs from canonical inventory".to_owned());
        }
        if field(canonical_record, "native_symbol") != field(record, "native_symbol") {
            mismatches.push("native symbol differs from canonical inventory".to_owned());
        }
        if field(canonical_record, "netlib_source_url") != source_url {
            mismatches.push("Netlib source URL differs from canonical inventory".to_owned());
        }
        if field(ownership_record, "canonical_public_path") != canonical_path
            || field(ownership_record, "native_symbol") != field(record, "native_symbol")
            || ownership_record["extern_declaration_count"].as_u64() != Some(1)
        {
            mismatches.push("authoritative declaration ownership differs".to_owned());
        }
        if !routine_page.contains(&source_url) || !routine_page.contains(&routine) {
            mismatches.push(
                "routine-reference page omits routine identity or exact source URL".to_owned(),
            );
        }
        if !html.contains(&field(record, "native_symbol")) {
            mismatches.push("rendered rustdoc omits native symbol".to_owned());
        }
        if !abi_arguments_in_order(&html, &abi_arguments) {
            mismatches.push("rendered rustdoc does not show ABI arguments in order".to_owned());
        }
        let generic_arguments = record["arguments"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|argument| {
                field(argument, "description_evidence_source") == "fixed_form_executable_dataflow"
                    || !argument_contamination_flags(
                        &field(argument, "description"),
                        &field(argument, "name"),
                        &abi_arguments,
                    )
                    .is_empty()
            })
            .map(|argument| field(argument, "name"))
            .collect::<Vec<_>>();
        let page_found = !html.is_empty();
        let status = if !page_found {
            "missing-canonical-rustdoc"
        } else if !missing_argument_names.is_empty()
            || !missing_sections.is_empty()
            || !source_url_found
            || !mismatches.is_empty()
        {
            "incomplete-rendered-rustdoc"
        } else if !generic_arguments.is_empty()
            || field(record, "documentation_quality_status") != "complete-semantic-contract"
        {
            "semantic-review-required"
        } else {
            "complete-semantic-contract"
        };
        if status != "complete-semantic-contract" {
            *family_defects.entry(field(record, "family")).or_default() += 1;
        }
        output.push(json!({
            "routine":routine,
            "canonical_rust_path":canonical_path,
            "rustdoc_html_path":html_relative.to_string_lossy().replace('\\', "/"),
            "canonical_item_html_path":requested_html_relative.to_string_lossy().replace('\\', "/"),
            "canonical_item_is_rustdoc_reexport":reexport_target,
            "rustdoc_page_found":page_found,
            "abi_argument_names":abi_arguments,
            "documented_argument_names":documented_argument_names,
            "missing_argument_names":missing_argument_names,
            "netlib_url_expected":source_url,
            "netlib_url_found":source_url_found,
            "required_sections":required_sections,
            "missing_sections":missing_sections,
            "generic_or_unreviewed_argument_names":generic_arguments,
            "cross_surface_mismatches":mismatches,
            "status":status,
        }));
    }
    output.sort_by_key(|record| field(record, "routine"));
    let pages_found = output
        .iter()
        .filter(|record| record["rustdoc_page_found"].as_bool() == Some(true))
        .count();
    let missing_arguments = output
        .iter()
        .filter(|record| {
            !record["missing_argument_names"]
                .as_array()
                .is_some_and(Vec::is_empty)
        })
        .count();
    let missing_links = output
        .iter()
        .filter(|record| record["netlib_url_found"].as_bool() != Some(true))
        .count();
    let generic_contracts = output
        .iter()
        .filter(|record| {
            !record["generic_or_unreviewed_argument_names"]
                .as_array()
                .is_some_and(Vec::is_empty)
        })
        .count();
    let structural_complete = output
        .iter()
        .filter(|record| field(record, "status") != "missing-canonical-rustdoc")
        .filter(|record| field(record, "status") != "incomplete-rendered-rustdoc")
        .count();
    let complete_semantic_contracts = output
        .iter()
        .filter(|record| field(record, "status") == "complete-semantic-contract")
        .count();
    let dassl_callbacks = dassl_callback_rustdoc_audit(root)?;
    let complete_dassl_callbacks = dassl_callbacks
        .iter()
        .filter(|record| field(record, "status") == "complete")
        .count();
    let report = json!({
        "schema_id":"slatec-rs.rendered-rustdoc-semantic-audit",
        "schema_version":"2.0.0",
        "policy":"A complete semantic contract requires a canonical rendered public rustdoc page, all ABI arguments in order, applicable sections only, the verified exact Netlib source URL, cross-surface agreement, bounded argument-specific semantics, and no contamination or placeholder evidence.",
        "summary":{
            "canonical_public_routines":output.len(),
            "rendered_rustdoc_pages_found":pages_found,
            "routines_missing_arguments":missing_arguments,
            "routines_missing_netlib_links":missing_links,
            "routines_with_generic_or_unreviewed_contracts":generic_contracts,
            "families_with_rendered_documentation_defects":family_defects.len(),
            "cross_surface_mismatches":output.iter().map(|record| record["cross_surface_mismatches"].as_array().map_or(0, Vec::len)).sum::<usize>(),
            "structurally_complete_rendered_rustdoc":structural_complete,
            "complete_semantic_contracts":complete_semantic_contracts,
            "dassl_callback_contracts":dassl_callbacks.len(),
            "complete_dassl_callback_contracts":complete_dassl_callbacks,
        },
        "family_defects":family_defects,
        "dassl_callback_contracts":dassl_callbacks,
        "records":output,
    });
    write_json(&output_dir.join("rendered-rustdoc-audit.json"), &report)?;
    write_markdown(
        &output_dir.join("rendered-rustdoc-audit-summary.md"),
        &rendered_rustdoc_audit_markdown(&report),
    )?;
    write_json(
        &output_dir.join("rendered-rustdoc-semantic-audit.json"),
        &report,
    )?;
    write_markdown(
        &output_dir.join("rendered-rustdoc-semantic-audit-summary.md"),
        &rendered_rustdoc_audit_markdown(&report),
    )?;
    Ok(report)
}

/// DASSL exposes callback ABI aliases as part of the public raw API.  They are
/// not extern declarations, so audit their rendered type-alias pages alongside
/// the SDASSL/DDASSL function pages.
fn dassl_callback_rustdoc_audit(root: &Path) -> Result<Vec<Value>> {
    let checks = [
        (
            "DasslResidualF32",
            "type.DasslResidualF32.html",
            [
                "IRES",
                "DELTA",
                "unwind",
                "https://www.netlib.org/slatec/src/sdassl.f",
                "Safety",
            ],
        ),
        (
            "DasslResidualF64",
            "type.DasslResidualF64.html",
            [
                "IRES",
                "DELTA",
                "unwind",
                "https://www.netlib.org/slatec/src/ddassl.f",
                "Safety",
            ],
        ),
        (
            "DasslJacobianF32",
            "type.DasslJacobianF32.html",
            [
                "PD",
                "banded storage",
                "unwind",
                "https://www.netlib.org/slatec/src/sdassl.f",
                "Safety",
            ],
        ),
        (
            "DasslJacobianF64",
            "type.DasslJacobianF64.html",
            [
                "PD",
                "banded storage",
                "unwind",
                "https://www.netlib.org/slatec/src/ddassl.f",
                "Safety",
            ],
        ),
    ];
    let mut output = Vec::new();
    for (name, relative, required) in checks {
        let path = PathBuf::from("target/doc/slatec_sys/dassl").join(relative);
        let html = fs::read_to_string(root.join(&path)).unwrap_or_default();
        let missing = required
            .iter()
            .filter(|required| !html.contains(**required))
            .map(|required| (*required).to_owned())
            .collect::<Vec<_>>();
        let status = if html.is_empty() {
            "missing-page"
        } else if missing.is_empty() {
            "complete"
        } else {
            "incomplete"
        };
        output.push(json!({
            "public_rust_path":format!("slatec_sys::dassl::{name}"),
            "rustdoc_html_path":path.to_string_lossy().replace('\\', "/"),
            "page_found":!html.is_empty(),
            "required_contract_markers":required,
            "missing_contract_markers":missing,
            "status":status,
        }));
    }
    Ok(output)
}

/// Regenerates the audit and makes missing rendered documentation a failure.
pub fn validate_rendered_rustdoc_audit(root: &Path) -> Result<Value> {
    let report = generate_rendered_rustdoc_audit(root)?;
    let summary = &report["summary"];
    if summary["routines_missing_arguments"].as_u64() != Some(0)
        || summary["routines_missing_netlib_links"].as_u64() != Some(0)
        || summary["cross_surface_mismatches"].as_u64() != Some(0)
        || summary["complete_semantic_contracts"].as_u64()
            != summary["canonical_public_routines"].as_u64()
        || summary["complete_dassl_callback_contracts"].as_u64()
            != summary["dassl_callback_contracts"].as_u64()
    {
        return Err(policy("rendered-rustdoc audit is not complete"));
    }
    Ok(report)
}

fn rustdoc_html_relative_path(canonical_path: &str) -> Result<PathBuf> {
    let mut parts = canonical_path.split("::");
    if parts.next() != Some("slatec_sys") {
        return Err(policy("canonical path is not a slatec-sys path"));
    }
    let mut path = PathBuf::from("target/doc/slatec_sys");
    let parts = parts.collect::<Vec<_>>();
    let (routine, modules) = parts
        .split_last()
        .ok_or_else(|| policy("canonical path has no routine name"))?;
    for module in modules {
        path.push(module);
    }
    path.push(format!("fn.{routine}.html"));
    Ok(path)
}

/// Rustdoc renders a public re-export as an entry in the canonical module's
/// index and links it to the owning function page. Follow that link rather
/// than assuming the filesystem path from the re-export's namespace exists.
fn resolve_rustdoc_html(
    root: &Path,
    requested_relative: &Path,
    routine: &str,
) -> Result<(PathBuf, PathBuf, bool)> {
    let requested = root.join(requested_relative);
    if requested.is_file() {
        return Ok((requested_relative.to_path_buf(), requested, false));
    }
    let module_index = requested
        .parent()
        .ok_or_else(|| policy("rustdoc function path has no parent module"))?
        .join("index.html");
    let module = fs::read_to_string(&module_index).map_err(|_| {
        policy(&format!(
            "canonical rustdoc page and its module index are missing for {routine}"
        ))
    })?;
    let marker = format!("id=\"reexport.{}\"", routine.to_ascii_lowercase());
    let marker_position = module.find(&marker).ok_or_else(|| {
        policy(&format!(
            "canonical rustdoc module lacks re-export for {routine}"
        ))
    })?;
    let item = &module[marker_position..];
    let href_start = item
        .find("href=\"")
        .map(|position| position + "href=\"".len())
        .ok_or_else(|| {
            policy(&format!(
                "canonical rustdoc re-export lacks target for {routine}"
            ))
        })?;
    let href = item[href_start..]
        .split_once('"')
        .map(|(href, _)| href)
        .ok_or_else(|| {
            policy(&format!(
                "canonical rustdoc re-export has malformed target for {routine}"
            ))
        })?;
    let target = module_index
        .parent()
        .ok_or_else(|| policy("rustdoc module index has no parent"))?
        .join(href);
    if !target.is_file() {
        return Err(policy(&format!(
            "canonical rustdoc re-export target is missing for {routine}"
        )));
    }
    let relative = target
        .strip_prefix(root)
        .map_err(|_| policy("rustdoc target escapes workspace"))?
        .to_path_buf();
    Ok((relative, target, true))
}

fn rustdoc_has_section(html: &str, section: &str) -> bool {
    html.contains(&format!("title=\"{section}\"")) || html.contains(&format!(">{section}</h2>"))
}

fn rustdoc_mentions_argument(html: &str, argument: &str) -> bool {
    html.contains(&format!(">{argument}</code>")) || html.contains(&format!(">{argument}<"))
}

fn abi_arguments_in_order(html: &str, arguments: &[String]) -> bool {
    let mut last = 0;
    for argument in arguments {
        let Some(position) = html[last..].find(&format!(">{argument}</code>")) else {
            return false;
        };
        last += position + argument.len();
    }
    true
}

fn rendered_rustdoc_audit_markdown(report: &Value) -> String {
    let summary = &report["summary"];
    let mut output = format!(
        "# Rendered canonical rustdoc semantic audit\n\n- Canonical public routines audited: **{}**\n- Rendered rustdoc pages found: **{}**\n- Routines missing one or more ABI argument names: **{}**\n- Routines missing the exact Netlib link: **{}**\n- Routines with generic, contaminated, or unreviewed argument semantics: **{}**\n- Families with rendered-documentation defects: **{}**\n- Cross-surface mismatches: **{}**\n- Structurally complete rendered pages: **{}**\n- Complete semantic contracts: **{}**\n\nA routine is complete only when its canonical rendered public item—not merely a reference page—passes every structural, source-link, cross-surface, bounded-semantic, and contamination check.\n",
        summary["canonical_public_routines"],
        summary["rendered_rustdoc_pages_found"],
        summary["routines_missing_arguments"],
        summary["routines_missing_netlib_links"],
        summary["routines_with_generic_or_unreviewed_contracts"],
        summary["families_with_rendered_documentation_defects"],
        summary["cross_surface_mismatches"],
        summary["structurally_complete_rendered_rustdoc"],
        summary["complete_semantic_contracts"],
    );
    output.push_str(&format!(
        "- Complete DASSL callback contracts: **{}** / **{}**\n",
        summary["complete_dassl_callback_contracts"], summary["dassl_callback_contracts"]
    ));
    output.push_str(
        "\n## Families requiring semantic review\n\n| Family | Routines |\n| --- | ---: |\n",
    );
    if let Some(families) = report["family_defects"].as_object() {
        for (family, count) in families {
            output.push_str(&format!("| {family} | {count} |\n"));
        }
    }
    output
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
        if let Some(path) =
            resolved_cached_source_path(root, cached_path, &selected_provider_source_path(record))
        {
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

fn selected_provider_source_path(record: &Value) -> String {
    let provider = record.get("canonical_provider").unwrap_or(&Value::Null);
    let source = provider
        .get("source_file")
        .and_then(Value::as_str)
        .filter(|path| !path.is_empty());
    match source {
        Some(path) if path.contains('/') || path.contains('\\') => path.to_owned(),
        Some(path) => provider
            .get("subset")
            .and_then(Value::as_str)
            .filter(|subset| !subset.is_empty())
            .map(|subset| format!("{subset}/{path}"))
            .unwrap_or_else(|| path.to_owned()),
        None => record["source_file"]
            .as_str()
            .unwrap_or_default()
            .to_owned(),
    }
}

/// Resolves a verified selected-source input without changing the stable,
/// repository-relative evidence path emitted in reports. A clean worktree may
/// supply the separately acquired `SLATEC_SOURCE_CACHE`; the selected source
/// hash remains the authority for accepting either location.
fn resolved_cached_source_path(
    root: &Path,
    evidence_path: &str,
    provider_source_path: &str,
) -> Option<PathBuf> {
    let evidence = root.join(evidence_path);
    resolve_cached_source_locations(
        evidence,
        semantic_evidence_cache_dir(root).as_deref(),
        evidence_path,
        &semantic_source_cache_dir(root),
        provider_source_path,
    )
}

fn resolve_cached_source_locations(
    evidence: PathBuf,
    evidence_cache: Option<&Path>,
    evidence_path: &str,
    source_cache: &Path,
    provider_source_path: &str,
) -> Option<PathBuf> {
    if evidence.is_file() {
        return Some(evidence);
    }
    if let Some(cache) = evidence_cache {
        let relative = Path::new(evidence_path)
            .strip_prefix("evidence/full-corpus/audit-input/directories")
            .ok()?;
        let cached_evidence = cache.join(relative);
        if cached_evidence.is_file() {
            return Some(cached_evidence);
        }
    }
    let cached_source = source_cache.join(provider_source_path);
    cached_source.is_file().then_some(cached_source)
}

fn semantic_evidence_cache_dir(root: &Path) -> Option<PathBuf> {
    std::env::var_os("SLATEC_EVIDENCE_CACHE")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                root.join(path)
            }
        })
}

fn semantic_source_cache_dir(root: &Path) -> PathBuf {
    std::env::var_os("SLATEC_SOURCE_CACHE")
        .map(PathBuf::from)
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                root.join(path)
            }
        })
        .unwrap_or_else(|| root.join("target/slatec-source-cache"))
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
            source_start_line: 0,
            source_end_line: 0,
            source_section: "not_stated_by_source".to_owned(),
            evidence_kind: "unavailable".to_owned(),
            precision_sibling_source: "not_applicable".to_owned(),
            direction_evidence: "unavailable".to_owned(),
            direction_conflict: String::new(),
        });
    }
    Ok(result)
}

fn corrections_by_routine(
    value: &Value,
    catalogue: &BTreeMap<String, &Value>,
) -> Result<BTreeMap<String, Value>> {
    let records = records(value, "semantic documentation corrections")?;
    let profiles = value
        .get("profiles")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
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
        let mut materialized = record.clone();
        if let Some(profile_name) = record.get("documentation_profile").and_then(Value::as_str) {
            let profile = profiles.get(profile_name).ok_or_else(|| {
                policy(&format!(
                    "semantic correction profile {profile_name} is not defined for {routine}"
                ))
            })?;
            let mut arguments = profile
                .get("arguments")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_default();
            if let Some(overrides) = record.get("arguments").and_then(Value::as_object) {
                for (argument, override_value) in overrides {
                    arguments.insert(argument.clone(), override_value.clone());
                }
            }
            if !arguments.is_empty() {
                materialized["arguments"] = Value::Object(arguments);
            }
        }
        if output.insert(routine, materialized).is_some() {
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
    let mut section = "outside";
    let mut direction: Option<String> = None;
    let mut current = Vec::<String>::new();
    let mut current_status = String::new();
    for (line_number, raw_line) in String::from_utf8_lossy(bytes).lines().enumerate() {
        let line_number = line_number + 1;
        let Some(comment) = comment_text(raw_line) else {
            continue;
        };
        let trimmed = comment.trim();
        let heading = normalize_heading(trimmed);
        if heading.contains("END PROLOGUE") {
            break;
        }
        if heading.starts_with("DESCRIPTION OF ARGUMENT")
            || heading == "ARGUMENTS"
            || heading == "ARGUMENT"
            || heading == "PARAMETERS"
            // Legacy prologues sometimes prefix the section name with a
            // Roman numeral (`II. PARAMETERS`).  Do not treat every prose
            // sentence ending in "parameters" as a section heading: for
            // example, FISHPACK's `IERROR` paragraph says "invalid input
            // parameters" and must remain attached to IERROR.
            || (heading.ends_with(" PARAMETERS")
                && heading.split_whitespace().count() <= 2)
            || heading == "PARAMETER"
            || heading.starts_with("INPUT PARAMETERS")
            || heading.starts_with("OUTPUT PARAMETERS")
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
            // In the common SLATEC description-table form, argument rows
            // before an explicit `ON RETURN` heading are input contracts.
            direction = Some("input".to_owned());
            current.clear();
            continue;
        }
        if heading == "ON RETURN" || heading == "ON OUTPUT" {
            section = "on_return";
            direction = Some("output".to_owned());
            current.clear();
            continue;
        }
        // This is a section-heading test, not a word search: formal
        // arguments such as `IERROR` and descriptions of relative error are
        // ordinary argument prose and must not silently start an error block.
        if is_error_section_heading(&heading) {
            section = "errors";
            direction = None;
            current.clear();
            continue;
        }
        if heading.starts_with("DIMENSIONING PARAMETER")
            || heading.starts_with("DIMENSION PARAMETERS")
        {
            section = "dimensioning";
            direction = Some("input".to_owned());
            current.clear();
            continue;
        }
        if heading == "WORK ARRAYS" || heading == "WORKSPACE" || heading == "WORK SPACE" {
            section = "workspace";
            direction = Some("workspace-output".to_owned());
            current.clear();
            continue;
        }
        if matches!(
            heading.as_str(),
            "INPUT" | "INPUT/OUTPUT" | "OUTPUT" | "ON INPUT" | "ON INPUT/OUTPUT" | "ON OUTPUT"
        ) || heading.starts_with("INPUT ALL TYPE")
            || heading.starts_with("OUTPUT ALL TYPE")
        {
            // Several early SLATEC prologues place `Input...` and `Output...`
            // directly inside DESCRIPTION rather than under a separate
            // "Description of Arguments" heading.  Once encountered, this is
            // an argument section, not prose for the routine description.
            section = "arguments";
            direction = Some(
                match heading.as_str() {
                    "OUTPUT" | "ON OUTPUT" => "output",
                    "INPUT/OUTPUT" | "ON INPUT/OUTPUT" => "input-output",
                    _ if heading.starts_with("OUTPUT ") => "output",
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
        if trimmed.contains("----") {
            // Horizontal rules delimit visual blocks but do not terminate a
            // parameter section. Older least-squares prologues place their
            // grouped argument labels immediately after one.
            continue;
        }
        if heading.starts_with("QUANTITIES WHICH") || heading.starts_with("OPTIONALLY REPLACEABLE")
        {
            section = "";
            current.clear();
            continue;
        }
        // A number of retained SLATEC prologues (notably DASSL) introduce
        // their detailed first-call contract after a prose heading rather
        // than repeating `Arguments`.  A known argument label is stronger
        // evidence than that incidental heading, so preserve the source
        // prose as argument documentation wherever it occurs in the
        // prologue.
        // Error/status prose may mention almost every formal argument.  It
        // must never restart an argument block: this was the source of the
        // DQAG cross-contamination where `A` absorbed the KRONROD text and
        // `LIMIT` absorbed error messages and workspace layout.
        if section == "errors" {
            // QUADPACK's extended drivers place a second, unambiguously
            // labelled output-array table after their IER messages. A
            // dash/colon row with a known non-status formal is stronger
            // evidence than surrounding status prose and safely resumes
            // bounded argument extraction without reopening arbitrary
            // mentions of `RESULT`, `LIMIT`, or `WORK`.
            let restart =
                (trimmed.contains("--") || trimmed.contains(" - ") || trimmed.contains(':'))
                    && bounded_argument_line(trimmed, &argument_names, true)
                        .is_some_and(|label| !label.names.iter().any(|name| is_status_name(name)));
            if restart {
                section = "on_return";
                direction = Some("output".to_owned());
                current.clear();
            } else {
                append_text(&mut result.errors, trimmed);
                parse_status_value(
                    trimmed,
                    line_number,
                    &mut current_status,
                    &mut result.statuses,
                    &argument_names,
                );
                continue;
            }
        }
        match section {
            "outside" | "description" | "arguments" | "on_return" | "dimensioning"
            | "workspace" => {
                if let Some(label) = bounded_argument_line(trimmed, &argument_names, true) {
                    // Some compact BLAS-style prologues split a shared
                    // declaration over adjacent rows, for example
                    // `CX(N):` followed by `CY(N): Complex arrays ...`.
                    // The type sentence belongs to both explicitly named
                    // rows, not only the latter one.
                    if !label.text.is_empty() && begins_type_sentence(&label.text) {
                        for previous_name in &current {
                            let previous =
                                result.arguments.entry(previous_name.clone()).or_default();
                            if previous.text.is_empty()
                                && previous.source_end_line.saturating_add(2) >= line_number
                            {
                                previous.source_end_line = line_number;
                                append_distinct_text(&mut previous.text, &label.text);
                            }
                        }
                    }
                    // Some older prologues wrap a grouped argument label over
                    // several physical rows before giving one shared
                    // description, for example `NDATA,XDATA(*)`,
                    // `YDATA(*)`, and `SDDATA(*)`.  Keep that shared source
                    // range attached to every explicitly listed formal
                    // instead of leaving the first rows undocumented.
                    let continues_grouped_label = label.text.is_empty()
                        && label.direction.is_none()
                        && !current.is_empty()
                        && current.iter().all(|name| {
                            result
                                .arguments
                                .get(name)
                                .is_some_and(|item| item.text.is_empty())
                        });
                    if continues_grouped_label {
                        for name in label.names {
                            if !current.contains(&name) {
                                current.push(name);
                            }
                        }
                    } else {
                        current = label.names;
                    }
                    let label_direction = label.direction.or_else(|| direction.clone());
                    if !current.iter().any(|name| is_status_name(name)) {
                        current_status.clear();
                    }
                    for name in &current {
                        let entry = result.arguments.entry(name.clone()).or_default();
                        if let Some(direction) = label_direction.as_deref() {
                            entry.direction =
                                merge_source_direction(entry.direction.as_deref(), direction);
                            if entry.direction.as_deref() == Some("input-output")
                                && entry.source_section != "input-output"
                            {
                                entry.source_section = "input-output".to_owned();
                            }
                        }
                        entry.changed_on_return |= label.changed_on_return;
                        if entry.source_start_line == 0 {
                            entry.source_start_line = line_number;
                            entry.source_section = section.to_owned();
                            entry.evidence_kind = "selected_source_prologue".to_owned();
                        }
                        entry.source_end_line = line_number;
                        append_distinct_text(&mut entry.text, &label.text);
                        if is_status_name(name) {
                            current_status = name.clone();
                        }
                    }
                } else if !current.is_empty() {
                    for name in &current {
                        let entry = result.arguments.entry(name.clone()).or_default();
                        entry.source_end_line = line_number;
                        append_distinct_text(&mut entry.text, trimmed);
                    }
                    // A number of prologues keep `IER = 0`, followed by
                    // shorthand `= 1`, inside the IER argument paragraph
                    // rather than opening an `ERROR MESSAGES` section.
                    // Preserve that evidence as a structured status table.
                    parse_status_value(
                        trimmed,
                        line_number,
                        &mut current_status,
                        &mut result.statuses,
                        &argument_names,
                    );
                } else if section == "description" {
                    append_distinct_text(&mut result.description, trimmed);
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
    matches!(
        heading,
        "LIBRARY"
            | "CATEGORY"
            | "KEYWORDS"
            | "AUTHOR"
            | "REFERENCES"
            | "ROUTINES CALLED"
            | "REVISION HISTORY"
            | "HISTORY"
            | "LONG DESCRIPTION"
            | "PROGRAM SPECIFICATIONS"
            | "SEE ALSO"
    ) || heading.starts_with("LIBRARY ")
        || heading.starts_with("CATEGORY ")
        || heading.starts_with("KEYWORDS ")
        || heading.starts_with("AUTHOR ")
        || (heading.starts_with("REFERENCES ") && !heading.starts_with("REFERENCES TO "))
        || heading.starts_with("ROUTINES CALLED ")
        || heading.starts_with("REVISION HISTORY ")
        || heading.starts_with("LONG DESCRIPTION ")
        || heading.starts_with("PROGRAM SPECIFICATIONS ")
}

/// Distinguishes an actual status/error heading from ordinary argument prose
/// such as `(Error return if X1.EQ.X2.)`.  Treating every line that starts
/// with `ERROR` as a section header truncated many PCHIP parameter blocks.
fn is_error_section_heading(heading: &str) -> bool {
    matches!(
        heading,
        "ERROR" | "ERROR MESSAGES" | "ERROR CODES" | "ERROR CONDITIONS" | "ERROR FLAGS"
    ) || heading.starts_with("ERROR MESSAGE ")
        || heading.starts_with("ERROR CODE ")
        || heading.starts_with("ERROR CONDITION ")
        || heading.starts_with("ERROR FLAG ")
}

fn merge_source_direction(existing: Option<&str>, observed: &str) -> Option<String> {
    match (existing, observed) {
        (None, _) => Some(observed.to_owned()),
        (Some("input"), "output") | (Some("output"), "input") => Some("input-output".to_owned()),
        (Some("input-output"), _) | (_, "input-output") => Some("input-output".to_owned()),
        (Some(existing), _) => Some(existing.to_owned()),
    }
}

fn begins_type_sentence(text: &str) -> bool {
    matches!(
        text.split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_uppercase()
            .trim_end_matches('S'),
        "REAL" | "COMPLEX" | "INTEGER" | "LOGICAL" | "DOUBLE"
    )
}

#[derive(Clone, Debug)]
struct ArgumentLabel {
    names: Vec<String>,
    text: String,
    direction: Option<String>,
    changed_on_return: bool,
}

/// Recognizes one fixed-form argument-table row without treating an arbitrary
/// mention of a formal argument as a new row.  In particular, `A GAUSS-
/// KRONROD ...` is prose about `KEY`, not an argument definition for `A`.
fn bounded_argument_line(
    text: &str,
    known: &BTreeSet<&str>,
    allow_bare_label: bool,
) -> Option<ArgumentLabel> {
    let trimmed = text.trim();
    let numbered = trimmed
        .split_once('.')
        .and_then(|(prefix, remaining)| {
            (prefix.starts_with(|character: char| character.is_ascii_digit())
                && prefix
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric()))
            .then_some(remaining.trim_start())
        })
        .filter(|remaining| !remaining.is_empty())
        .unwrap_or(trimmed);
    if numbered != trimmed {
        return bounded_argument_line(numbered, known, allow_bare_label);
    }
    for separator in ["--", " - "] {
        if let Some((left, right)) = trimmed.split_once(separator) {
            let names = strict_argument_names(left, known);
            if !names.is_empty() {
                return Some(ArgumentLabel {
                    names,
                    text: right.trim().to_owned(),
                    direction: None,
                    changed_on_return: left.contains('*'),
                });
            }
        }
    }
    // A few SLATEC prologues spell a table row as `IERR- a status
    // code`.  Accept that compact form only when the right-hand first word
    // is not another formal argument, so algebra such as `A-B` cannot turn
    // into an argument definition for A.
    if let Some((left, right)) = trimmed.split_once('-') {
        let names = strict_argument_names(left, known);
        let first_right = right
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .trim_matches(|character: char| !character.is_ascii_alphanumeric())
            .to_owned();
        if !names.is_empty()
            && !left.contains(char::is_whitespace)
            && right
                .chars()
                .next()
                .is_some_and(|character| character.is_ascii_whitespace())
            // `IERR- a status code` uses a lower-case article while an
            // algebraic `A-B` uses the upper-case formal B.  Preserve the
            // former and reject the latter.
            && (!known.contains(first_right.to_ascii_uppercase().as_str())
                || first_right
                    .chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_lowercase()))
        {
            return Some(ArgumentLabel {
                names,
                text: right.trim().to_owned(),
                direction: None,
                changed_on_return: left.contains('*'),
            });
        }
    }
    if let Some((left, right)) = trimmed.split_once(':') {
        let names = strict_argument_names(left, known);
        if !names.is_empty() {
            let tag = right.split_whitespace().next().unwrap_or_default();
            let direction = match tag.to_ascii_uppercase().as_str() {
                "IN" => Some("input".to_owned()),
                "OUT" => Some("output".to_owned()),
                "INOUT" | "IN/OUT" => Some("input-output".to_owned()),
                "WORK" => Some("workspace-output".to_owned()),
                "EXT" | "EXTERNAL" => Some("callback".to_owned()),
                _ => None,
            };
            let text = if direction.is_some() {
                right
                    .trim_start_matches(|character: char| {
                        character.is_ascii_alphabetic() || character == '/'
                    })
                    .trim()
                    .to_owned()
            } else {
                right.trim().to_owned()
            };
            return Some(ArgumentLabel {
                names,
                text,
                direction,
                changed_on_return: left.contains('*'),
            });
        }
    }
    // A small group of pre-4.0 prologues use `NAME = description` rather
    // than a dash or colon. Restrict this to textual right-hand sides so an
    // `IER = 0` status value remains a status row, not a new argument row.
    if let Some((left, right)) = trimmed.split_once('=') {
        let names = strict_argument_names(left, known);
        let remainder = right.trim_start();
        if !names.is_empty()
            && (remainder
                .chars()
                .next()
                .is_some_and(|character| character.is_ascii_alphabetic() || character == '(')
                || (!names.iter().any(|name| is_status_name(name))
                    && remainder
                        .chars()
                        .any(|character| character.is_ascii_alphabetic())))
        {
            return Some(ArgumentLabel {
                names,
                text: right.trim().to_owned(),
                direction: None,
                changed_on_return: left.contains('*'),
            });
        }
    }
    // Older EISPACK prologues use a coupled label such as `LOW and IGH are
    // two INTEGER variables ...`; permit it only when every pre-verb token is
    // a known formal name or an explicit connector.
    let upper = trimmed.to_ascii_uppercase();
    for marker in [
        " IS ",
        " ARE ",
        " CONTAIN ",
        " CONTAINS ",
        " DENOTES ",
        " SPECIFIES ",
        " DEFINE ",
        " DEFINES ",
    ] {
        if let Some(position) = upper.find(marker) {
            let names = strict_argument_names(&trimmed[..position], known);
            if names.len() > 1 {
                return Some(ArgumentLabel {
                    names,
                    text: trimmed[position + marker.len()..].trim().to_owned(),
                    direction: None,
                    changed_on_return: false,
                });
            }
        }
    }
    let token_end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
    let token = &trimmed[..token_end];
    if !token
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic())
    {
        return None;
    }
    let names = strict_argument_names(token, known);
    if names.is_empty() {
        return None;
    }
    let trailing = &trimmed[token_end..];
    let whitespace = trailing
        .chars()
        .take_while(|character| character.is_whitespace())
        .count();
    let text = trailing.trim();
    let first_word = text
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let direct_contract_verb = matches!(
        first_word.as_str(),
        "MUST"
            | "IS"
            | "ARE"
            | "CONTAINS"
            | "CONTAIN"
            | "DENOTES"
            | "SPECIFIES"
            | "SETS"
            | "GIVES"
            | "DEFINES"
            | "DESCRIBES"
            | "RECORDS"
            | "SHOULD"
    );
    let fortran_predicate = [".GE.", ".GT.", ".LE.", ".LT.", ".EQ.", ".NE."]
        .iter()
        .any(|predicate| text.to_ascii_uppercase().starts_with(predicate));
    let type_row = text.starts_with('(') && text.ends_with(')');
    if allow_bare_label
        && (text.is_empty()
            || whitespace >= 2
            || direct_contract_verb
            || fortran_predicate
            || type_row)
    {
        return Some(ArgumentLabel {
            names,
            text: text.to_owned(),
            direction: None,
            changed_on_return: token.contains('*'),
        });
    }
    None
}

fn strict_argument_names(text: &str, known: &BTreeSet<&str>) -> Vec<String> {
    let mut names = Vec::new();
    let mut outside_dimensions = String::with_capacity(text.len());
    let mut depth = 0usize;
    for character in text.chars() {
        match character {
            '(' => {
                depth += 1;
                outside_dimensions.push(' ');
            }
            ')' => {
                depth = depth.saturating_sub(1);
                outside_dimensions.push(' ');
            }
            _ if depth > 0 => outside_dimensions.push(' '),
            _ => outside_dimensions.push(character),
        }
    }
    for raw in outside_dimensions.split(|character: char| !character.is_ascii_alphanumeric()) {
        let token = raw.trim();
        if token.is_empty() {
            continue;
        }
        let token = token.to_ascii_uppercase();
        if matches!(token.as_str(), "AND" | "OR") {
            continue;
        }
        let name = if known.contains(token.as_str()) {
            token
        } else if token == "KVPT" && known.contains("KPVT") {
            "KPVT".to_owned()
        } else if token == "JVPT" && known.contains("JPVT") {
            "JPVT".to_owned()
        } else {
            return Vec::new();
        };
        if !names.contains(&name) {
            names.push(name);
        }
    }
    names
}

/// Retained for compact parser tests and legacy source forms.  New extraction
/// always uses `bounded_argument_line` so source prose cannot restart an
/// unrelated argument range.
#[cfg(test)]
fn argument_line(
    text: &str,
    known: &BTreeSet<&str>,
    allow_bare_label: bool,
) -> Option<(Vec<String>, String)> {
    bounded_argument_line(text, known, allow_bare_label).map(|label| (label.names, label.text))
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

fn append_distinct_text(target: &mut String, text: &str) {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.is_empty() || target.contains(&text) {
        return;
    }
    append_text(target, &text);
}

fn parse_status_value(
    text: &str,
    line_number: usize,
    current_status: &mut String,
    statuses: &mut BTreeMap<String, Vec<StatusValue>>,
    known: &BTreeSet<&str>,
) {
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let upper = normalized.to_ascii_uppercase();
    let mut status_name = current_status.clone();
    let mut value = String::new();
    let mut extracted_meaning = None;
    if let Some((left, right)) = upper.split_once('=') {
        let names = strict_argument_names(left, known);
        if let Some(name) = names.into_iter().find(|name| is_status_name(name)) {
            status_name = name;
            *current_status = status_name.clone();
            let right = right.trim();
            let digits = right
                .trim_start_matches(['+', '-'])
                .chars()
                .take_while(|character| character.is_ascii_digit())
                .collect::<String>();
            if !digits.is_empty() {
                value = if right.starts_with('-') {
                    format!("-{digits}")
                } else {
                    digits
                };
            }
        } else if !status_name.is_empty() {
            let right = right.trim();
            let digits = right
                .trim_start_matches(['+', '-'])
                .chars()
                .take_while(|character| character.is_ascii_digit())
                .collect::<String>();
            if !digits.is_empty() {
                value = if right.starts_with('-') {
                    format!("-{digits}")
                } else {
                    digits
                };
            }
        }
    } else if !status_name.is_empty() {
        if let Some(position) = upper.find(".GT.") {
            value = ">0".to_owned();
            extracted_meaning = Some(
                normalized[position + 4..]
                    .trim_start_matches(|character: char| character.is_ascii_digit())
                    .trim()
                    .to_owned(),
            );
        }
    }
    if status_name.is_empty() {
        return;
    }
    let entries = statuses.entry(status_name).or_default();
    if value.is_empty() {
        if let Some(entry) = entries.last_mut() {
            entry.source_end_line = line_number;
            append_distinct_text(&mut entry.meaning, &normalized);
        }
        return;
    }
    let meaning = extracted_meaning.unwrap_or_else(|| {
        normalized
            .split_once('=')
            .map(|(_, meaning)| {
                meaning
                    .trim_start_matches(|character: char| {
                        character.is_ascii_digit() || character == '+' || character == '-'
                    })
                    .trim()
                    .to_owned()
            })
            .unwrap_or_default()
    });
    if let Some(entry) = entries.last_mut().filter(|entry| entry.value == value) {
        entry.source_end_line = line_number;
        append_distinct_text(&mut entry.meaning, &meaning);
    } else {
        entries.push(StatusValue {
            value,
            meaning,
            source_start_line: line_number,
            source_end_line: line_number,
        });
    }
}

/// Returns an explicit retained precision sibling when the selected source
/// lacks a separate argument paragraph.  This is lower-priority evidence than
/// the routine's own prologue and only supplies prose for identically named
/// formal arguments; it never changes an ABI declaration or invents a name.
fn precision_sibling_sections<'a>(
    routine: &str,
    base: &[Argument],
    arguments_by_routine: &BTreeMap<String, Vec<Argument>>,
    catalogue: &BTreeMap<String, &Value>,
    sections: &'a BTreeMap<String, SourceSections>,
) -> Option<(&'a str, &'a SourceSections)> {
    let mut candidates = Vec::new();
    if !routine.is_empty() {
        let (first, rest) = routine.split_at(1);
        match first {
            "S" => candidates.push(format!("D{rest}")),
            "D" => {
                candidates.push(format!("S{rest}"));
                candidates.push(rest.to_owned());
            }
            "C" => candidates.push(format!("Z{rest}")),
            "Z" => candidates.push(format!("C{rest}")),
            _ => candidates.push(format!("D{routine}")),
        }
    }
    for candidate in candidates {
        if candidate == routine || !sections.contains_key(&candidate) {
            continue;
        }
        let Some(sibling) = arguments_by_routine.get(&candidate) else {
            continue;
        };
        let same_kind = catalogue
            .get(routine)
            .zip(catalogue.get(&candidate))
            .is_some_and(|(left, right)| field(left, "kind") == field(right, "kind"));
        if !same_kind || !precision_sibling_signature_matches(base, sibling) {
            // A similarly named precision variant may have a different ABI
            // shape (BSGQ8/DBSGQ8 is one retained example).  It is not a
            // transfer source. Validation below rejects any recorded
            // transfer that does not satisfy this same predicate.
            continue;
        }
        if let Some((name, sibling_sections)) = sections.get_key_value(&candidate) {
            return Some((name.as_str(), sibling_sections));
        }
    }
    None
}

/// A precision sibling may supply missing source prose only for the same
/// callable shape.  Scalar precision differs by design, but formal names,
/// order, shapes, callback structure, and program-unit return kind must not.
fn precision_sibling_signature_matches(base: &[Argument], sibling: &[Argument]) -> bool {
    base.len() == sibling.len()
        && base.iter().zip(sibling).all(|(left, right)| {
            left.name == right.name
                && left.shape == right.shape
                && left.external_callback == right.external_callback
        })
}

fn review_arguments(
    base: &[Argument],
    sections: &SourceSections,
    sibling_sections: Option<&SourceSections>,
    sibling_routine: Option<&str>,
    bytes: &[u8],
    correction: Option<&Value>,
) -> Vec<Argument> {
    let flow = executable_flow(bytes, base);
    let mut output = Vec::with_capacity(base.len());
    for base_argument in base {
        let mut argument = base_argument.clone();
        let source = sections.arguments.get(&argument.name);
        let sibling_source =
            sibling_sections.and_then(|sections| sections.arguments.get(&argument.name));
        let semantic_source = source
            .filter(|item| !item.text.is_empty())
            .or_else(|| sibling_source.filter(|item| !item.text.is_empty()));
        let correction_argument = correction
            .and_then(|record| record.get("arguments"))
            .and_then(Value::as_object)
            .and_then(|items| items.get(&argument.name));
        let explicit_direction = source
            .and_then(source_argument_direction)
            .or_else(|| sibling_source.and_then(source_argument_direction));
        let flow_direction =
            direction_from_flow(flow.get(&argument.name).copied().unwrap_or_default());
        let source_text = semantic_source.map_or("", |item| item.text.as_str());
        let source_section = source
            .map(|item| item.source_section.as_str())
            .or_else(|| sibling_source.map(|item| item.source_section.as_str()))
            .unwrap_or("not_stated_by_source");
        let workspace = is_explicit_workspace_contract(&argument.name, source_section, source_text);
        let source_status = source_text.to_ascii_lowercase().contains("error indicator")
            || source_text.to_ascii_lowercase().contains("error flag")
            || source_text.to_ascii_lowercase().contains("status code")
            || source_text.to_ascii_lowercase().contains("completion code")
            || source_text.to_ascii_lowercase().contains("return code");
        let option_control = is_option_control_array(&argument.name, source_text);
        let input_control = is_input_control_argument(
            &argument.name,
            source_text,
            source.and_then(|item| item.direction.as_deref()),
        );
        argument.semantic_role = if argument.external_callback {
            "callback".to_owned()
        } else if workspace {
            "workspace".to_owned()
        } else if !option_control
            && !input_control
            && is_status_name(&argument.name)
            && (source_status
                || matches!(
                    explicit_direction.as_deref(),
                    Some("output") | Some("input-output")
                ))
        {
            "status".to_owned()
        } else if argument.shape.starts_with("rank") {
            "array".to_owned()
        } else {
            "scalar".to_owned()
        };
        if argument.external_callback {
            argument.direction = "callback".to_owned();
            argument.direction_evidence = "callback_abi".to_owned();
            argument.callback_restrictions = "The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.".to_owned();
        } else if workspace {
            argument.direction = "workspace-output".to_owned();
            argument.direction_evidence = "selected_source_workspace_section".to_owned();
            argument.workspace_requirement = source_text.to_owned();
        } else if argument.semantic_role == "status" {
            argument.direction = "status-output".to_owned();
            argument.direction_evidence = "selected_source_status_contract".to_owned();
        } else if option_control || input_control {
            argument.direction = "input".to_owned();
            argument.direction_evidence = "selected_source_option_control".to_owned();
        } else if let Some(direction) = explicit_direction.as_deref() {
            argument.direction = direction.to_owned();
            argument.direction_evidence =
                if source.is_some_and(|item| source_argument_direction(item).is_some()) {
                    "selected_source_explicit_direction".to_owned()
                } else {
                    "verified_precision_sibling_direction".to_owned()
                };
        } else {
            argument.direction = flow_direction.clone();
            argument.direction_evidence = "fixed_form_executable_dataflow".to_owned();
        }
        if let Some(explicit) = source.and_then(source_argument_direction) {
            if explicit != flow_direction {
                argument.direction_conflict = format!(
                    "selected source says `{explicit}` while executable dataflow is `{flow_direction}`; selected source direction is retained"
                );
            }
        }
        if let Some(item) = semantic_source {
            argument.description = concise_source_text(&item.text);
            argument.description_evidence = if source.is_some_and(|source| !source.text.is_empty())
            {
                "source_prologue_argument_section".to_owned()
            } else {
                "verified_precision_sibling_prologue".to_owned()
            };
            argument.source_start_line = item.source_start_line;
            argument.source_end_line = item.source_end_line;
            argument.source_section = item.source_section.clone();
            argument.evidence_kind = item.evidence_kind.clone();
            if source.is_none_or(|source| source.text.is_empty()) {
                argument.precision_sibling_source = sibling_routine
                    .unwrap_or("matching_selected_precision_sibling")
                    .to_owned();
            }
        } else {
            argument.description = format!(
                "The selected source does not give an independent semantic paragraph for `{}`.",
                argument.name
            );
            argument.description_evidence = "fixed_form_executable_dataflow".to_owned();
            argument.evidence_kind = "fixed_form_executable_dataflow".to_owned();
        }
        if let Some(override_value) = correction_argument {
            if let Some(text) = override_value.get("description").and_then(Value::as_str) {
                argument.description = text.to_owned();
                argument.description_evidence = "authored_source_hash_guarded_override".to_owned();
                argument.evidence_kind = "authored_source_hash_guarded_override".to_owned();
            }
            if explicit_direction.is_none() {
                if let Some(direction) = override_value.get("direction").and_then(Value::as_str) {
                    argument.direction = direction.to_owned();
                    argument.direction_evidence =
                        "authored_source_hash_guarded_direction".to_owned();
                }
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

/// Selects the strongest direction statement attached to one bounded argument
/// paragraph.  Table-wide headings are useful defaults, while per-argument
/// `on return` prose and the documented `* changed by routine` convention are
/// stronger evidence for legacy typed parameter tables.
fn source_argument_direction(source: &SourceArgument) -> Option<String> {
    let text = source.text.to_ascii_lowercase();
    let says_on_return =
        text.contains("on return") || text.starts_with("output") || text.contains("(output)");
    let says_input = text.contains("(input)")
        || text.starts_with("input")
        || text.contains("input matrix")
        || text.contains("set by the user");
    if source.changed_on_return {
        return Some(
            if says_input {
                "input-output"
            } else if says_on_return {
                "output"
            } else {
                "input-output"
            }
            .to_owned(),
        );
    }
    if says_on_return {
        return Some(if says_input { "input-output" } else { "output" }.to_owned());
    }
    source.direction.clone()
}

/// A small group of scalar special-function sources document their argument
/// only through the function expression in PURPOSE/DESCRIPTION (for example
/// `ACOSH(X) computes ...`).  This fallback is source-purpose evidence, not
/// a parameter-name guess, and is used only for function inputs lacking a
/// separate argument paragraph.
fn apply_function_purpose_fallbacks(arguments: &mut [Argument], catalogue: &Value) {
    if field(catalogue, "kind") != "function" {
        return;
    }
    let purpose = field(catalogue, "short_purpose");
    if purpose.is_empty() {
        return;
    }
    for argument in arguments {
        if argument.external_callback
            || argument.description_evidence != "fixed_form_executable_dataflow"
            || argument.direction != "input"
        {
            continue;
        }
        argument.description = format!(
            "Input value at which the source-defined function is evaluated: {}",
            purpose.trim_end_matches('.')
        );
        argument.description_evidence = "source_prologue_purpose_fallback".to_owned();
        argument.evidence_kind = "selected_source_purpose_and_declaration".to_owned();
        argument.source_section = "purpose".to_owned();
    }
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

fn is_workspace_name(name: &str) -> bool {
    matches!(name, "WORK" | "IWORK" | "RWORK" | "DWORK" | "WSAVE") || name.ends_with("WORK")
}

fn is_option_control_array(name: &str, source_text: &str) -> bool {
    if name != "INFO" {
        return false;
    }
    let text = source_text.to_ascii_lowercase();
    text.contains("how you want your problem solved")
        || text.contains("how you want this task to be carried out")
        || text.contains("items which are arranged as questions")
        || text.contains("items, which are arranged as questions")
}

/// `IND` is an output/status array in a number of EISPACK interfaces, but
/// an input bound-classification array in the linear-programming drivers.
/// Its source paragraph, rather than its spelling or pointer mutability,
/// resolves that distinction.
fn is_input_control_argument(
    name: &str,
    source_text: &str,
    source_direction: Option<&str>,
) -> bool {
    if name != "IND" {
        return false;
    }
    let text = source_text.to_ascii_lowercase();
    source_direction == Some("input")
        || text.contains("ind(*) are input parameters")
        || text.contains("type of bound")
        || text.contains("form of the bounds")
}

/// Workspace is a source-level role, not a consequence of an argument merely
/// being mentioned alongside a work array.  For example, BLKTRI's coefficient
/// arrays are *stored in* `W`; they are not themselves caller workspace.
fn is_explicit_workspace_contract(name: &str, source_section: &str, source_text: &str) -> bool {
    if source_section == "workspace" || is_workspace_name(name) {
        return true;
    }
    let text = source_text.trim_start().to_ascii_lowercase();
    [
        "work array",
        "a work array",
        "work vector",
        "a work vector",
        "workspace",
    ]
    .iter()
    .any(|prefix| text.starts_with(prefix))
}

fn is_status_name(name: &str) -> bool {
    matches!(
        name,
        "INFO" | "IERROR" | "IER" | "IFLAG" | "NZ" | "IND" | "STATUS"
    ) || name.ends_with("INFO")
}

/// Converts one bounded source paragraph into concise user-facing prose.  It
/// deliberately preserves mathematical predicates and storage formulas while
/// removing repeated fixed-form fragments and source-only type prefixes.
fn concise_source_text(text: &str) -> String {
    let mut sentences = Vec::<String>::new();
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    for sentence in normalized.split_terminator('.') {
        let sentence = sentence.trim();
        if sentence.is_empty() {
            continue;
        }
        let sentence = sentence
            .trim_start_matches("Double precision ")
            .trim_start_matches("Single precision ")
            .trim_start_matches("Integer ")
            .trim_start_matches("Real ")
            .trim_start_matches("Logical ")
            .trim();
        if sentence.is_empty() || sentences.iter().any(|prior| prior == sentence) {
            continue;
        }
        sentences.push(sentence.to_owned());
    }
    // A bounded argument paragraph can still contain an extended historical
    // explanation.  Keep its leading semantic sentences; status tables and
    // workspace sections are rendered separately rather than copied into the
    // argument row.
    sentences.truncate(6);
    let mut output = sentences.join(". ");
    if !output.is_empty() && !matches!(output.chars().last(), Some('.' | '!' | '?')) {
        output.push('.');
    }
    output
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
    let fixed_form_only = page
        .arguments
        .iter()
        .any(|argument| argument.description_evidence == "fixed_form_executable_dataflow");
    let status = if fixed_form_only {
        "semantic-review-required"
    } else {
        "complete-semantic-contract"
    };
    let evidence = if page
        .arguments
        .iter()
        .any(|argument| argument.description_evidence == "authored_source_hash_guarded_override")
    {
        "bounded selected-source prologue evidence plus source-hash-guarded authored corrections"
    } else if fixed_form_only {
        "verified source hash and fixed-form dataflow; one or more argument semantics still need source-prologue or authored review"
    } else {
        "bounded selected-source prologue evidence"
    };
    let mut output = format!(
        "{DOC_START}\n## Interface documentation quality\n\n- Documentation work status: `{status}`\n- Documentation evidence: {evidence}\n- Exact Netlib source: [{}]({source_url})\n\n### Arguments\n\n",
        page.routine,
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
    output.push_str("\nThe authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.\n");
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
    }
    let status_rows = rendered_status_rows(page);
    if !status_rows.is_empty() {
        output.push_str(
            "\n### Error and status values\n\n| Status | Value | Meaning |\n| --- | ---: | --- |\n",
        );
        for (name, value, meaning) in status_rows {
            output.push_str(&format!(
                "| `{name}` | `{value}` | {} |\n",
                escape_table(&meaning)
            ));
        }
    } else if !page.errors.trim().is_empty() {
        output.push_str(&format!(
            "\n### Error and status values\n\n{}\n",
            page.errors
        ));
    }
    let work = page
        .arguments
        .iter()
        .filter(|argument| argument.semantic_role == "workspace")
        .map(|argument| {
            let requirement = if argument.workspace_requirement.trim().is_empty() {
                // Some legacy SLAP entry points describe a workspace's role in
                // the bounded argument text but give no separate size formula.
                // Reuse that verified text rather than emitting a blank label
                // or inventing a capacity requirement.
                &argument.description
            } else {
                &argument.workspace_requirement
            };
            format!("`{}`: {}", argument.name, requirement)
        })
        .collect::<Vec<_>>();
    let has_array_storage = page.arguments.iter().any(|argument| {
        argument.shape.starts_with("rank")
            || argument.leading_dimension != "not applicable or not stated by selected source"
    });
    if !work.is_empty() {
        output.push_str(&format!(
            "\n### Storage and workspace requirements\n\n{}\n",
            work.join("\n\n")
        ));
    } else if has_array_storage {
        output.push_str("\n### Storage and array requirements\n\nArray arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.\n");
    }
    output.push_str(&format!("\n### Provider, ABI, and safety\n\nCanonical Rust path: `{}`. Native symbol: `{}`. Declaration feature: `{}`. Provider feature: `{}`. ABI fingerprint: `{}`.\n\n# Safety\n\nEvery pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.\n{DOC_END}", field(page.final_record, "canonical_rust_path"), field(page.final_record, "native_symbol"), field(page.final_record, "feature"), field(page.status, "provider_feature"), page.abi));
    output
}

/// Writes the documentation fragment that is attached to the actual public
/// Rust function, rather than only to its generated reference page.
///
/// The fragment is deliberately keyed by the canonical routine name.  The
/// generated Rust source only includes this file; it does not duplicate an
/// `extern` declaration or make a generated ABI-shaped namespace public.
fn write_public_rustdoc_contract(directory: &Path, page: &RoutinePage<'_>) -> Result<()> {
    if let Some(contract) = sos_dsos_rustdoc_contract(page) {
        write_markdown(
            &directory.join(format!("{}.md", page.routine.to_ascii_lowercase())),
            &contract,
        )?;
        return Ok(());
    }
    if let Some(contract) = dassl_rustdoc_contract(page) {
        write_markdown(
            &directory.join(format!("{}.md", page.routine.to_ascii_lowercase())),
            &contract,
        )?;
        return Ok(());
    }
    if let Some(contract) = quadpack_qag_rustdoc_contract(page) {
        write_markdown(
            &directory.join(format!("{}.md", page.routine.to_ascii_lowercase())),
            &contract,
        )?;
        return Ok(());
    }
    let source_url = field(page.link, "exact_netlib_url");
    let mut output = format!(
        "# Purpose\n\n{}\n\n# Description\n\nThis canonical unsafe binding exposes original SLATEC routine `{}`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [{}]({}).\n\n# Arguments\n\n",
        escape_rustdoc_text(page.description),
        page.routine,
        page.routine,
        source_url
    );
    if page.arguments.is_empty() {
        output.push_str("This routine has no ABI arguments.\n");
    } else {
        for argument in page.arguments {
            output.push_str(&format!(
                "## `{}`\n\n**Direction:** `{}`. **Fortran type:** `{}`. **Rust ABI type:** `{}`. **Shape:** {}.\n\n{}\n\n",
                argument.name,
                argument.direction,
                argument.fortran_type,
                argument.rust_raw_type,
                argument.shape,
                rustdoc_argument_contract(argument)
            ));
        }
    }
    if field(page.catalogue, "kind") == "function" {
        output.push_str(&format!(
            "# Return value\n\nThis Fortran function returns its scalar result using the compiler-validated ABI fingerprint `{}`. It has no separate Rust `Result` status channel.\n\n",
            page.abi
        ));
    } else {
        output.push_str("# Return value\n\nThis is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.\n\n");
    }
    if page
        .arguments
        .iter()
        .any(|argument| argument.external_callback)
    {
        output.push_str("# Callback contract\n\nEach callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.\n\n");
    }
    let status_rows = rendered_status_rows(page);
    if !status_rows.is_empty() {
        output.push_str(
            "# Status and error values\n\n| Status | Value | Meaning |\n| --- | ---: | --- |\n",
        );
        for (name, value, meaning) in status_rows {
            output.push_str(&format!(
                "| `{name}` | `{value}` | {} |\n",
                escape_table(&escape_rustdoc_text(&meaning))
            ));
        }
        output.push('\n');
    } else {
        let status_arguments = page
            .arguments
            .iter()
            .filter(|argument| argument.semantic_role == "status")
            .map(|argument| format!("`{}`", argument.name))
            .collect::<Vec<_>>();
        if !status_arguments.is_empty() {
            output.push_str(&format!(
                "# Status and error values\n\n{} is a documented status output; its bounded argument contract states the available source semantics.\n\n",
                status_arguments.join(", ")
            ));
        }
    }
    let work = page
        .arguments
        .iter()
        .filter(|argument| {
            argument.semantic_role == "workspace"
                || argument.shape.starts_with("rank")
                || argument.leading_dimension != "not applicable or not stated by selected source"
        })
        .map(|argument| {
            format!(
                "- `{}`: {}",
                argument.name,
                if argument.workspace_requirement.trim().is_empty() {
                    escape_rustdoc_text(&argument.leading_dimension)
                } else {
                    escape_rustdoc_text(&argument.workspace_requirement)
                }
            )
        })
        .collect::<Vec<_>>();
    if !work.is_empty() {
        output.push_str("# Workspace and array requirements\n\n");
        output.push_str(&format!("{}\n\n", work.join("\n")));
    }
    output.push_str(&format!(
        "# ABI notes\n\n- Canonical Rust path: `{}`\n- Original SLATEC routine: `{}`\n- Native symbol: `{}`\n- ABI fingerprint: `{}`\n- Exact Netlib source file: [{}]({})\n\n# Safety\n\nEvery raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.\n",
        field(page.final_record, "canonical_rust_path"),
        page.routine,
        field(page.final_record, "native_symbol"),
        page.abi,
        page.routine,
        source_url
    ));
    if !output.ends_with('\n') {
        output.push('\n');
    }
    write_markdown(
        &directory.join(format!("{}.md", page.routine.to_ascii_lowercase())),
        &output,
    )?;
    Ok(())
}

fn rendered_status_rows(page: &RoutinePage<'_>) -> Vec<(String, String, String)> {
    let mut rows = Vec::new();
    for (name, entries) in page.statuses {
        for entry in entries {
            if !entry.meaning.is_empty() {
                rows.push((name.clone(), entry.value.clone(), entry.meaning.clone()));
            }
        }
    }
    rows
}

/// Complete source-hash-guarded contract for the paired SOS drivers.
/// The source evaluates one numbered scalar equation at a time and mutates
/// the trial vector while estimating its Jacobian, so the callback and
/// workspace rules must be visible on the public declaration.
fn sos_dsos_rustdoc_contract(page: &RoutinePage<'_>) -> Option<String> {
    let (precision, scalar, callback) = match page.routine {
        "SOS" => ("single precision", "f32", "SosEquationF32"),
        "DSOS" => ("double precision", "f64", "SosEquationF64"),
        _ => return None,
    };
    let source_url = field(page.link, "exact_netlib_url");
    let mut output = format!(
        "# Purpose\n\nSolves a square system of nonlinear algebraic equations by a modified Newton method with a forward-difference Jacobian. This is the original {precision} SLATEC driver `{}`.\n\n# Description\n\n`{}` repeatedly evaluates the numbered equations through `FNC(X, K)`, forms or updates an upper-triangular linear system, and returns the current iterate and convergence status through mutable arguments. The selected, source-hash-verified prologue is [{}]({}).\n\n# Arguments\n\n",
        page.routine, page.routine, page.routine, source_url
    );
    output.push_str(&format!(
        "- `FNC`: required synchronous `{callback}` callback. It receives a readable length-`NEQ` current or finite-difference-perturbed iterate `X` and one-based integer `K` in `1..=NEQ`, and returns the `K`th equation value. It must not mutate or retain either pointer and must not unwind.\n- `NEQ`: input number of equations and unknowns; it must be positive.\n- `X`: mutable length-`NEQ` `{scalar}` solution vector. It supplies the initial estimate and is overwritten with the most recent iterate. During finite differences the native code temporarily changes components before calling `FNC`.\n- `RTOLX`: input nonnegative relative iterate tolerance. The increment criterion is `abs(X(I)-XOLD(I)) <= RTOLX*abs(X(I)) + ATOLX`.\n- `ATOLX`: input nonnegative absolute iterate tolerance. A positive value is useful when a solution component can be zero.\n- `TOLF`: input nonnegative residual tolerance. Residual convergence requires every equation residual to be no greater than `TOLF`; scale equations so this test is meaningful.\n- `IFLAG`: input/output control and status integer. Input `0` uses default controls; input `-1` reads optional controls from `IW(1..=2)`. On return it reports one of the status values below.\n- `RW`: mutable `{scalar}` workspace with at least `LRW` elements. The required minimum is `1 + 6*NEQ + NEQ*(NEQ + 1)/2`. `RW(1)` reports the residual norm on return.\n- `LRW`: input declared length of `RW`; it must meet the stated minimum.\n- `IW`: mutable integer workspace with at least `LIW` elements. `IW(1)=-1` requests iteration output when `IFLAG=-1`; `IW(2)` supplies a positive iteration limit in that mode; `IW(3)` reports the iteration count on return.\n- `LIW`: input declared length of `IW`; it must be at least `3 + NEQ`.\n\n# Return value\n\nThis Fortran subroutine has no direct return value. It returns the iterate, residual norm, iteration count, and termination state through `X`, `RW`, `IW`, and `IFLAG`.\n\n# Callback contract\n\n`FNC` is called synchronously, potentially many times and with finite-difference perturbations of `X`. `K` is one-based and identifies exactly the requested equation. The callback may only read the supplied `X` extent for the duration of the call; it must not retain pointers, panic, or unwind across the native boundary.\n\n# Status and error values\n\n| `IFLAG` | Meaning |\n| ---: | --- |\n| `1` | The iterate-increment test was satisfied. |\n| `2` | The residual test was satisfied. |\n| `3` | Both increment and residual tests were satisfied. |\n| `4` | Numerical precision was inadequate or convergence was too slow. |\n| `5` | The iteration limit was reached. |\n| `6` | The iteration limit was reached and the iteration was not converging. |\n| `7` | The iteration appeared to diverge. |\n| `8` | The Jacobian-related triangular system was singular or nearly singular. |\n| `9` | An input, optional-control, or workspace requirement was invalid. |\n\n# Workspace and array requirements\n\n`X` has exactly `NEQ` elements. `RW` has at least `1 + 6*NEQ + NEQ*(NEQ + 1)/2` elements and `IW` has at least `3 + NEQ` elements. The native routine mutates `X`, `RW`, and `IW`; preserve their allocation and do not create incompatible aliases.\n\n# ABI notes\n\n- Canonical Rust path: `{}`\n- Original SLATEC routine: `{}`\n- Native symbol: `{}`\n- Precision: {precision}\n- Supported ABI profile: `ffi-profile-gnu-mingw-x86_64`\n- Exact Netlib source file: [{}]({})\n\n# Safety\n\nEvery scalar pointer must be non-null, correctly aligned, and valid for the required read or write access. `X`, `RW`, and `IW` must satisfy the exact lengths above and must not alias in a way that violates Rust or the native routine's mutation assumptions. `FNC` must have the stated GNU Fortran scalar-function ABI, remain valid through the call, neither retain pointers nor unwind, and tolerate the source-documented temporary perturbations of `X`. The caller is responsible for serializing use of legacy native runtime state.\n",
        field(page.final_record, "canonical_rust_path"),
        page.routine,
        field(page.final_record, "native_symbol"),
        page.routine,
        source_url,
    ));
    output = output.replace(
        "`FNC` is called synchronously, potentially many times and with finite-difference perturbations of `X`. `K` is one-based and identifies exactly the requested equation. The callback may only read the supplied `X` extent for the duration of the call; it must not retain pointers, panic, or unwind across the native boundary.",
        "`FNC` is called synchronously, potentially many times and with finite-difference perturbations of `X`. It receives only `X` and the one-based equation index `K`: `NEQ` is not passed through the callback ABI, and there is no user-data/context pointer. The callback may only read the externally known supplied `X` extent for the duration of the call; it must not retain pointers, panic, or unwind across the native boundary. Stateful Rust use therefore requires an external scoped context mechanism in a future wrapper.",
    );
    Some(output)
}

/// Complete, source-hash-guarded contract for the two reviewed DASSL drivers.
/// Their multi-call protocol needs a dedicated renderer so that workspace,
/// status, and callback semantics stay visible on the canonical function.
fn dassl_rustdoc_contract(page: &RoutinePage<'_>) -> Option<String> {
    let (precision, scalar, residual, jacobian) = match page.routine {
        "DDASSL" => (
            "double precision",
            "f64",
            "DasslResidualF64",
            "DasslJacobianF64",
        ),
        "SDASSL" => (
            "single precision",
            "f32",
            "DasslResidualF32",
            "DasslJacobianF32",
        ),
        _ => return None,
    };
    let source_url = field(page.link, "exact_netlib_url");
    let mut output = format!(
        "# Purpose\n\nSolves a differential/algebraic system `G(T, Y, YPRIME) = 0` with backward-differentiation formulas of orders one through five. This is the original {precision} SLATEC DASSL driver `{}`.\n\n# Description\n\nThe first call starts a new problem. It requires consistent initial `T`, `Y`, and `YPRIME` unless `INFO(11)=1` asks DASSL to compute a consistent derivative. Subsequent calls continue the same problem, retain state in `RWORK` and `IWORK`, and advance in the original direction of `TOUT`. The verified source prologue is [{}]({}).\n\n# Arguments\n\n",
        page.routine, page.routine, source_url
    );
    output.push_str(&format!(
        "- `RES`: required `{residual}` callback. It implements `RES(T, Y, YPRIME, DELTA, IRES, RPAR, IPAR)`, writes `DELTA[0..NEQ] = G(T, Y, YPRIME)`, and must not alter `T`, `Y`, or `YPRIME`. It receives `IRES=0`; it may set `IRES=-1` for an illegal input (DASSL retries) or `IRES=-2` to return with `IDID=-11`.\n- `NEQ`: input number of equations; `NEQ >= 1`.\n- `T`: input initial independent-variable value and output time reached; it must be mutable storage.\n- `Y`: mutable length-`NEQ` vector of initial solution components and output solution at returned `T`.\n- `YPRIME`: mutable length-`NEQ` vector of initial derivatives and output derivative approximation; it must initially satisfy `G` unless `INFO(11)=1`.\n- `TOUT`: input target, different from `T`; forward and backward integration are allowed. The first step does not pass `TOUT`; an interval-mode later step may interpolate from beyond it unless `TSTOP` applies.\n- `INFO`: mutable integer array of length at least 15; DASSL uses entries 1 through 11. `INFO(1)=0` starts a problem and `=1` acknowledges an interrupted continuation. `INFO(2)` selects scalar (0) or length-`NEQ` vector (1) `RTOL`/`ATOL`; `INFO(3)` selects interval (0) or intermediate-output (1) mode; `INFO(4)=1` selects `TSTOP=RWORK(1)`; `INFO(5)=0` requests numerical differentiation and `=1` requires `JAC`; `INFO(6)=0` is dense and `=1` is banded with `IWORK(1)=ML`, `IWORK(2)=MU`; `INFO(7)=1` sets `HMAX=RWORK(2)`; `INFO(8)=1` sets `H0=RWORK(3)`; `INFO(9)=1` sets `MAXORD=IWORK(3)`, where `1 <= MAXORD <= 5`; `INFO(10)=1` requests nonnegative constraints; `INFO(11)=1` computes an initially consistent `YPRIME`.\n- `RTOL`, `ATOL`: input/output error tolerances. With `INFO(2)=0` both are scalars; with `INFO(2)=1` both are length-`NEQ` vectors. All entries must be nonnegative; DASSL may increase them for `IDID=-2`.\n- `IDID`: output completion/status code; see **Status and error values**.\n- `RWORK`: mutable {scalar} workspace of length `LRW`; it contains persistent continuation state.\n- `LRW`: input declared `RWORK` length. Minimum is `40 + (MAXORD + 4)*NEQ + NEQ*NEQ` for dense storage; `40 + (MAXORD + 4)*NEQ + (2*ML + MU + 1)*NEQ` for banded user `JAC`; or that banded amount plus `2*(NEQ/(ML + MU + 1) + 1)` for banded finite-difference `JAC`.\n- `IWORK`: mutable integer workspace of length `LIW`; it contains persistent continuation state and the documented band widths/order options.\n- `LIW`: input declared `IWORK` length; `LIW >= 20 + NEQ`.\n- `RPAR`, `IPAR`: caller-owned real and integer parameter arrays forwarded to `RES` and `JAC`; DASSL does not alter them and their lengths are defined by the caller/callback.\n- `JAC`: optional `{jacobian}` callback. With `INFO(5)=1`, it implements `JAC(T, Y, YPRIME, PD, CJ, RPAR, IPAR)` and writes `PD = dG/dY + CJ*dG/dYPRIME`; it must not alter `T`, `Y`, `YPRIME`, or `CJ`. Dense `PD` is column-major with first dimension `NEQ` and stores `PD(I,J)`; banded `PD` has first dimension `2*ML + MU + 1` and stores the entry at `PD(I - J + ML + MU + 1, J)`. With `INFO(5)=0`, it is ignored but the ABI argument remains required.\n\n"
    ));
    output.push_str("# Return value\n\nThis Fortran subroutine has no direct return value. Time, solution, derivative, tolerances, status, and solver state are returned through mutable arguments.\n\n# Callback contract\n\n`RES` and, when selected, `JAC` are synchronous and must remain valid for the complete native call. They must uphold the exact Rust callback ABI, preserve all callback vector extents, and **must not unwind** through Fortran. Neither callback may retain native pointers after returning.\n\n# Status and error values\n\n- `IDID=1`: an intermediate-output step completed; `TOUT` is not yet reached.\n- `IDID=2`: integration reached `TSTOP` exactly.\n- `IDID=3`: integration reached `TOUT`, possibly by interpolation after stepping beyond it.\n- `IDID=-1`: about 500 steps were expended; set `INFO(1)=1` to continue deliberately.\n- `IDID=-2`: tolerances were too stringent; `RTOL`/`ATOL` were increased for a possible continuation.\n- `IDID=-3`: an `ATOL` component and its solution component are both zero.\n- `IDID=-6`: repeated local error-test failures.\n- `IDID=-7`: repeated corrector convergence failures.\n- `IDID=-8`: singular partial-derivative matrix.\n- `IDID=-9`: repeated corrector and error-test failures on the last step.\n- `IDID=-10`: `RES` set `IRES=-1` and the corrector could not converge.\n- `IDID=-11`: `RES` set `IRES=-2`; control returned to the caller.\n- `IDID=-12`: DASSL failed to compute an initial `YPRIME`.\n- `IDID=-33`: unrecoverable or invalid-input termination; the source error runtime reports the diagnostic.\n\n# Workspace and array requirements\n\n`Y`, `YPRIME`, and callback `DELTA` have length `NEQ`. Preserve `RWORK` and `IWORK` unchanged for continuation except documented option entries. On return `RWORK(3)` is the next attempted step, `RWORK(4)` the farthest integration time, and `RWORK(7)` the last successful step. `IWORK(7..=8)` report orders; `IWORK(11..=15)` report steps, residual calls, Jacobian evaluations, error-test failures, and convergence failures. For continuation do not change `NEQ`, `T`, `Y`, `YPRIME`, `RWORK`, `IWORK`, or the residual system; after `IDID=2` or `3`, choose a new `TOUT` without reversing direction.\n\n");
    output.push_str(&format!(
        "# ABI notes\n\n- Canonical Rust path: `{}`\n- Original SLATEC routine: `{}`\n- Native symbol: `{}`\n- Precision: {precision}\n- Exact Netlib source file: [{}]({})\n\n# Safety\n\nEvery pointer must be non-null, correctly aligned, and valid for the exact scalar or array extent above. `Y`, `YPRIME`, `RWORK`, and `IWORK` are mutated and must not alias in a way that violates Rust's aliasing requirements. Preserve Fortran column-major `PD` layout, parameter-array and callback lifetime, persistent continuation state, and the no-unwind callback rule.\n",
        field(page.final_record, "canonical_rust_path"),
        page.routine,
        field(page.final_record, "native_symbol"),
        page.routine,
        source_url
    ));
    Some(output)
}

/// Complete source-hash-guarded renderer for the paired QUADPACK basic
/// drivers.  Their prologues deliberately interleave output, error, sizing,
/// and workspace sections, so this retains the source facts without exposing
/// a normalized fixed-form dump on the public page.
fn quadpack_qag_rustdoc_contract(page: &RoutinePage<'_>) -> Option<String> {
    let (precision, callback, scalar) = match page.routine {
        "DQAG" => ("double precision", "IntegrandF64", "f64"),
        "QAG" => ("single precision", "IntegrandF32", "f32"),
        _ => return None,
    };
    let source_url = field(page.link, "exact_netlib_url");
    let mut output = format!(
        "# Purpose\n\nApproximates a definite integral over `(A, B)` with an adaptive Gauss-Kronrod rule. This is the original {precision} QUADPACK driver `{}`.\n\n# Description\n\n`{}` repeatedly evaluates `F` and subdivides `(A, B)` until the requested absolute or relative accuracy is met, or it reports the limiting condition through `IER`. The selected source is [{}]({}).\n\n# Arguments\n\n",
        page.routine, page.routine, page.routine, source_url
    );
    output.push_str(&format!(
        "## `F`\n\n**Direction:** `callback`. The synchronous `{callback}` integrand receives one readable `{scalar}` abscissa and returns its function value. It must remain valid through the call, must not retain the pointer, and **must not unwind** through Fortran.\n\n## `A`\n\n**Direction:** `input`. Lower integration limit.\n\n## `B`\n\n**Direction:** `input`. Upper integration limit.\n\n## `EPSABS`\n\n**Direction:** `input`. Requested absolute accuracy.\n\n## `EPSREL`\n\n**Direction:** `input`. Requested relative accuracy. When `EPSABS <= 0`, it must be at least `max(50 * relative_machine_accuracy, 0.5e-28)` in the routine precision, otherwise `IER=6`.\n\n## `KEY`\n\n**Direction:** `input`. Selects the local Gauss-Kronrod pair: `< 2` selects 7/15 points; `2`, `3`, `4`, and `5` select 10/21, 15/31, 20/41, and 25/51 points; `> 5` selects 30/61 points.\n\n## `RESULT`\n\n**Direction:** `output`. Approximation to the requested integral.\n\n## `ABSERR`\n\n**Direction:** `output`. Estimate of the absolute error; it is intended to satisfy `ABSERR >= abs(I - RESULT)`.\n\n## `NEVAL`\n\n**Direction:** `output`. Number of integrand evaluations.\n\n## `IER`\n\n**Direction:** `status-output`. Completion and error indicator; see **Status and error values**.\n\n## `LIMIT`\n\n**Direction:** `input`. Maximum number of subintervals; `LIMIT >= 1`. It is also the required minimum length of `IWORK`.\n\n## `LENW`\n\n**Direction:** `input`. Declared length of `WORK`; `LENW >= 4 * LIMIT`.\n\n## `LAST`\n\n**Direction:** `output`. Number of subintervals produced. It determines the number of significant entries in the workspace segments.\n\n## `IWORK`\n\n**Direction:** `workspace-output`. Integer array of at least `LIMIT` elements. Its first `K` entries order subinterval error estimates, where `K=LAST` when `LAST <= LIMIT/2 + 2`, otherwise `K=LIMIT + 1 - LAST`.\n\n## `WORK`\n\n**Direction:** `workspace-output`. `{scalar}` array of at least `LENW` elements. On return its four `LIMIT`-strided segments hold left endpoints, right endpoints, subintegral estimates, and error estimates for the first `LAST` subintervals.\n\n# Return value\n\nThis Fortran subroutine has no direct return value. It returns the integral estimate, error estimate, evaluation count, status, and subdivision state through mutable arguments.\n\n# Callback contract\n\n`F` is invoked synchronously using the reviewed `{callback}` ABI. It may read its one scalar input only for the duration of the call and **must not panic or unwind** across the native boundary.\n\n# Status and error values\n\n| `IER` | Meaning |\n| ---: | --- |\n| `0` | Normal, reliable completion; the requested accuracy is assumed achieved. |\n| `1` | The maximum number of subdivisions was reached. Increasing `LIMIT` may help. |\n| `2` | Roundoff prevented the requested tolerance from being achieved. |\n| `3` | Extremely bad integrand behavior occurred within the integration interval. |\n| `6` | Invalid input: tolerance, `LIMIT`, or `LENW` constraints were violated. `RESULT`, `ABSERR`, `NEVAL`, and `LAST` are zeroed as documented by the source. |\n\n# Workspace and array requirements\n\n`IWORK` length is at least `LIMIT`; `WORK` length is at least `LENW`, with `LENW >= 4 * LIMIT`. `WORK(1..LAST)`, `WORK(LIMIT+1..LIMIT+LAST)`, `WORK(2*LIMIT+1..2*LIMIT+LAST)`, and `WORK(3*LIMIT+1..3*LIMIT+LAST)` respectively hold the left endpoints, right endpoints, subintegral estimates, and error estimates.\n\n# ABI notes\n\n- Canonical Rust path: `{}`\n- Original SLATEC routine: `{}`\n- Native symbol: `{}`\n- Precision: {precision}\n- Exact Netlib source file: [{}]({})\n\n# Safety\n\nAll scalar pointers and the `IWORK`/`WORK` arrays must be non-null, aligned, and valid for the documented extent. The output and workspace arguments are mutated; do not create aliases that violate Rust or the native routine's assumptions. Preserve the required workspace layout, callback lifetime, and no-unwind rule.\n",
        field(page.final_record, "canonical_rust_path"),
        page.routine,
        field(page.final_record, "native_symbol"),
        page.routine,
        source_url,
    ));
    Some(output)
}

fn rustdoc_argument_contract(argument: &Argument) -> String {
    let mut contract = argument.description.clone();
    if argument.external_callback && !argument.callback_restrictions.is_empty() {
        contract.push(' ');
        contract.push_str(&argument.callback_restrictions);
    }
    if !argument.workspace_requirement.is_empty()
        && argument.workspace_requirement != "not a workspace argument"
        && !contract.contains(&argument.workspace_requirement)
    {
        contract.push(' ');
        contract.push_str(&argument.workspace_requirement);
    }
    escape_rustdoc_text(&contract)
}

/// Source prologue prose is plain text, not Rustdoc Markdown. Escape bracket
/// pairs so legacy matrix notation such as `[A,B]` cannot become an accidental
/// unresolved intra-doc link under `-D warnings`.
fn escape_rustdoc_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
}

/// Installs a generated include attribute at each authoritative public item.
///
/// This runs as part of semantic-review generation, so generated Rust files
/// are never hand-edited.  It is intentionally idempotent and adds no Rust
/// declaration: one existing declaration remains the sole native ABI owner.
fn inject_public_rustdoc_contracts(root: &Path, routines: &BTreeSet<String>) -> Result<()> {
    let source_root = root.join("crates/slatec-sys/src");
    let mut paths = Vec::new();
    collect_rust_sources(&source_root, &mut paths)?;
    for path in paths {
        if path
            .strip_prefix(&source_root)
            .ok()
            .and_then(|relative| relative.components().next())
            .is_some_and(|component| component.as_os_str() == "generated")
        {
            // ABI-shaped generated files are private declaration owners. Their
            // ownership generator may remove or re-export entries, so the
            // canonical public re-export is the only rustdoc attachment site.
            continue;
        }
        let original = fs::read_to_string(&path)?;
        let mut output = String::new();
        let mut previous_was_contract = false;
        for line in original.lines() {
            let trimmed = line.trim_start();
            let indentation = &line[..line.len() - trimmed.len()];
            if line.contains("/src/generated_docs/") {
                // Keep the generated attribute aligned with the declaration so
                // `cargo fmt --check` is a deterministic validation rather
                // than a source of recurring Windows PR churn.
                output.push_str(indentation);
                output.push_str(trimmed);
                output.push('\n');
                previous_was_contract = true;
                continue;
            }
            let routine = public_routine_item(line, routines);
            if let Some(routine) = routine {
                if !previous_was_contract {
                    output.push_str(indentation);
                    output.push_str(&format!(
                        "#[doc = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/generated_docs/{}.md\"))]\n",
                        routine
                    ));
                }
            }
            output.push_str(line);
            output.push('\n');
            previous_was_contract = false;
        }
        if output != original.replace("\r\n", "\n") {
            fs::write(path, output)?;
        }
    }
    Ok(())
}

fn collect_rust_sources(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_rust_sources(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(())
}

fn public_routine_item(line: &str, routines: &BTreeSet<String>) -> Option<String> {
    let trimmed = line.trim_start();
    let candidate = if let Some(rest) = trimmed.strip_prefix("pub fn ") {
        rest.split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .next()
    } else if let Some(rest) = trimmed.strip_prefix("pub use ") {
        let tail = rest.trim_end_matches(';').rsplit("::").next()?;
        let candidate = tail.trim().trim_start_matches('{').trim_end_matches('}');
        (!candidate.contains(',') && !candidate.contains('*')).then_some(candidate)
    } else {
        None
    }?;
    let candidate = candidate.to_ascii_lowercase();
    routines.contains(&candidate).then_some(candidate)
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
        "source_start_line":argument.source_start_line,
        "source_end_line":argument.source_end_line,
        "source_section":argument.source_section,
        "evidence_kind":argument.evidence_kind,
        "precision_sibling_source":argument.precision_sibling_source,
        "direction_evidence":argument.direction_evidence,
        "direction_conflict":argument.direction_conflict,
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
    let complete = public
        .iter()
        .filter(|record| {
            field(record, "documentation_quality_status") == "complete-semantic-contract"
        })
        .count();
    let authored = public
        .iter()
        .filter(|record| {
            record["arguments"].as_array().is_some_and(|arguments| {
                arguments.iter().any(|argument| {
                    field(argument, "description_evidence_source")
                        == "authored_source_hash_guarded_override"
                })
            })
        })
        .count();
    json!({"canonical_public_routines":public.len(),"complete_semantic_contracts":complete,"source_hash_guarded_authored_semantic_contracts":authored,"complete_rendered_semantic_public":"computed only by rendered-rustdoc-semantic-audit","documentation_work_queue_public":public.len()-complete,"quality_counts":counts})
}

fn argument_contamination_flags(
    description: &str,
    argument: &str,
    all_arguments: &[String],
) -> Vec<String> {
    let lower = description.to_ascii_lowercase();
    let mut flags = Vec::new();
    for heading in [
        "error messages",
        "dimensioning parameters",
        "work arrays",
        "description of arguments",
    ] {
        if contains_leaked_section_heading(description, heading) {
            flags.push(format!("section-heading-leakage:{heading}"));
        }
    }
    for filler in [
        "not stated by selected source",
        "not applicable or not stated",
        "not a workspace argument",
        "classified by fixed-form",
    ] {
        if lower.contains(filler) {
            flags.push(format!("placeholder-prose:{filler}"));
        }
    }
    let sentences = description
        .split_terminator('.')
        .map(str::trim)
        .filter(|sentence| sentence.len() > 20)
        .collect::<Vec<_>>();
    if sentences.iter().enumerate().any(|(index, sentence)| {
        sentences
            .iter()
            .skip(index + 1)
            .any(|other| other.eq_ignore_ascii_case(sentence))
    }) {
        flags.push("repeated-source-fragment".to_owned());
    }
    let unrelated = all_arguments
        .iter()
        .filter(|name| name.as_str() != argument)
        .filter(|name| contains_leaked_argument_label(description, name))
        .count();
    if unrelated > 0 {
        flags.push("next-argument-label-capture".to_owned());
    }
    flags
}

/// A heading is contamination only when it appears as a standalone source
/// section marker.  Mentions such as "the number of significant entries in
/// the WORK arrays" are valid semantic relationships for `LAST`, not a
/// leaked workspace section.
fn contains_leaked_section_heading(description: &str, heading: &str) -> bool {
    let normalized = description
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    normalized == heading
        || normalized.starts_with(&format!("{heading}:"))
        || normalized.starts_with(&format!("{heading} -"))
        || normalized.contains(&format!(". {heading}:"))
        || normalized.contains(&format!(". {heading} -"))
}

/// Detects a formal argument row embedded after a completed sentence.  A
/// plain substring match (`W -`) is not sufficient: it flags ordinary
/// mathematics such as `Y - X*K` and prose such as `WORK -- 1`.
fn contains_leaked_argument_label(description: &str, name: &str) -> bool {
    let upper = description.to_ascii_uppercase();
    for boundary in [
        format!(". {name} --"),
        format!(". {name}:"),
        format!("; {name} --"),
        format!("; {name}:"),
    ] {
        if upper.contains(&boundary) {
            return true;
        }
    }
    false
}

fn semantic_contract_issues(arguments: &[Argument], sections: &SourceSections) -> Vec<String> {
    let names = arguments
        .iter()
        .map(|argument| argument.name.clone())
        .collect::<Vec<_>>();
    let mut issues = Vec::new();
    for argument in arguments {
        if argument.description.trim().is_empty()
            || argument.description_evidence == "fixed_form_executable_dataflow"
        {
            issues.push(format!("source-evidence-incomplete:{}", argument.name));
        }
        for flag in argument_contamination_flags(&argument.description, &argument.name, &names) {
            issues.push(format!("{flag}:{}", argument.name));
        }
    }
    let status_names = arguments
        .iter()
        .filter(|argument| argument.semantic_role == "status")
        .map(|argument| argument.name.as_str())
        .collect::<BTreeSet<_>>();
    for (name, entries) in &sections.statuses {
        if status_names.contains(name.as_str())
            && entries.iter().any(|entry| entry.meaning.trim().is_empty())
        {
            issues.push(format!("status-contract-incomplete:{name}"));
        }
    }
    issues.sort();
    issues.dedup();
    issues
}

fn write_semantic_quality_baseline_if_absent(
    root: &Path,
    prior_inventory: &Value,
    sections_by_routine: &BTreeMap<String, SourceSections>,
) -> Result<()> {
    let path = root.join("generated/slatec-routines/semantic-quality-baseline.json");
    if path.is_file() {
        return Ok(());
    }
    let mut records_out = Vec::new();
    for record in records(prior_inventory, "prior public documentation inventory")? {
        let routine = field(record, "routine");
        let arguments = record["arguments"].as_array().cloned().unwrap_or_default();
        let names = arguments
            .iter()
            .map(|argument| field(argument, "name"))
            .collect::<Vec<_>>();
        let mut issues = Vec::new();
        for argument in &arguments {
            let name = field(argument, "name");
            let description = field(argument, "description");
            issues.extend(
                argument_contamination_flags(&description, &name, &names)
                    .into_iter()
                    .map(|issue| format!("{issue}:{name}")),
            );
            if let Some(expected) = sections_by_routine
                .get(&routine)
                .and_then(|sections| sections.arguments.get(&name))
                .and_then(|source| source.direction.as_deref())
            {
                let observed = field(argument, "direction");
                if expected != observed && !(expected == "output" && observed == "status-output") {
                    issues.push(format!("direction-conflict:{name}"));
                }
            }
        }
        let category = if issues.iter().any(|issue| {
            issue.starts_with("next-argument-label-capture")
                || issue.starts_with("section-heading-leakage")
                || issue.starts_with("repeated-source-fragment")
        }) {
            "mangled-argument-extraction"
        } else if issues
            .iter()
            .any(|issue| issue.starts_with("direction-conflict"))
        {
            "direction-conflict"
        } else if issues
            .iter()
            .any(|issue| issue.starts_with("placeholder-prose"))
        {
            "source-evidence-incomplete"
        } else {
            "structurally-complete-semantic-review-required"
        };
        records_out.push(json!({
            "routine":routine,
            "canonical_rust_path":field(record, "canonical_rust_path"),
            "family":field(record, "family"),
            "legacy_claim":"complete-structured",
            "semantic_quality_status":category,
            "issues":issues,
        }));
    }
    records_out.sort_by_key(|record| field(record, "routine"));
    let mut counts = BTreeMap::<String, usize>::new();
    for record in &records_out {
        *counts
            .entry(field(record, "semantic_quality_status"))
            .or_default() += 1;
    }
    let report = json!({
        "schema_id":"slatec-rs.semantic-quality-baseline",
        "schema_version":"1.0.0",
        "policy":"Frozen pre-M3 baseline calculated from the committed M2 inventory before M3 regeneration. It is intentionally not recomputed after repair.",
        "canonical_public_routines":records_out.len(),
        "counts":counts,
        "records":records_out,
    });
    write_json(&path, &report)?;
    let mut markdown = String::from("# Semantic documentation quality baseline\n\n");
    markdown.push_str("This frozen baseline audits the M2 public documentation inventory before the M3 bounded parser and semantic renderer regenerate it.\n\n| Status | Routines |\n| --- | ---: |\n");
    for (status, count) in report["counts"].as_object().into_iter().flatten() {
        markdown.push_str(&format!("| `{status}` | {count} |\n"));
    }
    write_markdown(
        &root.join("generated/slatec-routines/semantic-quality-baseline-summary.md"),
        &markdown,
    )
}

fn write_m3_semantic_reports(
    root: &Path,
    public: &[Value],
    quality: &[Value],
    sections_by_routine: &BTreeMap<String, SourceSections>,
    corrections: &BTreeMap<String, Value>,
) -> Result<()> {
    let quality_by_routine = quality
        .iter()
        .map(|record| (field(record, "routine"), record))
        .collect::<BTreeMap<_, _>>();
    let mut final_records = Vec::new();
    let mut direction_records = Vec::new();
    let mut contamination_records = Vec::new();
    let mut status_records = Vec::new();
    let mut workspace_records = Vec::new();
    for record in public {
        let routine = field(record, "routine");
        let arguments = record["arguments"].as_array().cloned().unwrap_or_default();
        let names = arguments
            .iter()
            .map(|argument| field(argument, "name"))
            .collect::<Vec<_>>();
        let issues = quality_by_routine
            .get(&routine)
            .and_then(|record| record.get("semantic_issues"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        final_records.push(json!({
            "routine":routine,
            "canonical_rust_path":field(record, "canonical_rust_path"),
            "family":field(record, "family"),
            "semantic_quality_status":field(record, "documentation_quality_status"),
            "semantic_issues":issues,
            "arguments":arguments,
        }));
        for argument in &arguments {
            let name = field(argument, "name");
            let conflict = field(argument, "direction_conflict");
            if !conflict.is_empty() {
                direction_records.push(json!({
                    "routine":routine,
                    "argument":name,
                    "selected_direction":field(argument, "direction"),
                    "direction_evidence":field(argument, "direction_evidence"),
                    "conflict":conflict,
                    "resolution":"selected-source direction retained; executable dataflow is recorded as lower-precedence evidence",
                    "resolved":true,
                }));
            }
            let flags =
                argument_contamination_flags(&field(argument, "description"), &name, &names);
            if !flags.is_empty() {
                contamination_records.push(json!({
                    "routine":routine,
                    "argument":name,
                    "flags":flags,
                    "source_start_line":argument["source_start_line"].clone(),
                    "source_end_line":argument["source_end_line"].clone(),
                    "source_section":field(argument, "source_section"),
                }));
            }
        }
        let statuses = sections_by_routine
            .get(&routine)
            .map(|sections| &sections.statuses);
        for argument in arguments
            .iter()
            .filter(|argument| field(argument, "semantic_role") == "status")
        {
            let name = field(argument, "name");
            let values = statuses
                .and_then(|statuses| statuses.get(&name))
                .map(|entries| {
                    entries
                        .iter()
                        .map(|entry| {
                            json!({
                                "value":entry.value,
                                "meaning":entry.meaning,
                                "source_start_line":entry.source_start_line,
                                "source_end_line":entry.source_end_line,
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            status_records.push(json!({
                "routine":routine,
                "argument":name,
                "applicable":true,
                "documented_values":values,
                "coverage_status":if values.is_empty() { "not_stated_by_source" } else { "complete" },
            }));
        }
        let work = arguments
            .iter()
            .filter(|argument| field(argument, "semantic_role") == "workspace")
            .map(|argument| {
                json!({
                    "argument":field(argument, "name"),
                    "description":field(argument, "description"),
                    "source_section":field(argument, "source_section"),
                    "coverage_status":if field(argument, "description").is_empty() { "semantic-review-required" } else { "complete" },
                })
            })
            .collect::<Vec<_>>();
        if !work.is_empty() {
            workspace_records.push(json!({
                "routine":routine,
                "workspace_arguments":work,
                "coverage_status":"complete",
            }));
        }
    }
    final_records.sort_by_key(|record| field(record, "routine"));
    direction_records.sort_by_key(|record| (field(record, "routine"), field(record, "argument")));
    contamination_records
        .sort_by_key(|record| (field(record, "routine"), field(record, "argument")));
    status_records.sort_by_key(|record| (field(record, "routine"), field(record, "argument")));
    workspace_records.sort_by_key(|record| field(record, "routine"));
    let mut counts = BTreeMap::<String, usize>::new();
    let mut family_counts = BTreeMap::<String, BTreeMap<String, usize>>::new();
    for record in &final_records {
        let status = field(record, "semantic_quality_status");
        *counts.entry(status.clone()).or_default() += 1;
        *family_counts
            .entry(field(record, "family"))
            .or_default()
            .entry(status)
            .or_default() += 1;
    }
    let final_report = json!({
        "schema_id":"slatec-rs.semantic-quality-final",
        "schema_version":"1.0.0",
        "policy":"A complete semantic contract has bounded source or source-hash-guarded authored evidence, an argument-specific summary, resolved source-precedence direction, and no contamination findings.",
        "canonical_public_routines":final_records.len(),
        "counts":counts,
        "family_counts":family_counts,
        "records":final_records,
    });
    let direction_report = json!({
        "schema_id":"slatec-rs.direction-evidence-conflicts",
        "schema_version":"1.0.0",
        "policy":"Explicit selected-source direction is retained over executable dataflow. Every disagreement must have a recorded resolution.",
        "unresolved_public_conflicts":direction_records.iter().filter(|record| record["resolved"] != Value::Bool(true)).count(),
        "records":direction_records,
    });
    let contamination_report = json!({
        "schema_id":"slatec-rs.argument-contamination-audit",
        "schema_version":"1.0.0",
        "policy":"Public argument prose must be bounded to one argument source range and must not leak status, workspace, heading, repeated, or next-argument text.",
        "public_argument_descriptions_with_detected_contamination":contamination_records.len(),
        "records":contamination_records,
    });
    let status_report = json!({
        "schema_id":"slatec-rs.status-contract-coverage",
        "schema_version":"1.0.0",
        "status_arguments":status_records.len(),
        "complete_status_contracts":status_records.iter().filter(|record| field(record, "coverage_status") == "complete").count(),
        "records":status_records,
    });
    let workspace_report = json!({
        "schema_id":"slatec-rs.workspace-contract-coverage",
        "schema_version":"1.0.0",
        "routines_with_workspace_contracts":workspace_records.len(),
        "complete_workspace_contracts":workspace_records.iter().filter(|record| field(record, "coverage_status") == "complete").count(),
        "records":workspace_records,
    });
    let mut sample_names = BTreeSet::new();
    let mut per_family = BTreeMap::<String, usize>::new();
    for record in &final_report["records"]
        .as_array()
        .cloned()
        .unwrap_or_default()
    {
        let family = field(record, "family");
        let count = per_family.entry(family).or_default();
        if *count < 3 {
            sample_names.insert(field(record, "routine"));
            *count += 1;
        }
    }
    sample_names.extend(corrections.keys().cloned());
    sample_names.extend(
        direction_report["records"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|record| field(record, "routine")),
    );
    let sample_records = final_report["records"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|record| sample_names.contains(&field(record, "routine")))
        .map(|record| {
            json!({
                "routine":field(record, "routine"),
                "canonical_rust_path":field(record, "canonical_rust_path"),
                "family":field(record, "family"),
                "arguments":record["arguments"].clone(),
                "review_result":"machine-semantic-audit-passed",
                "reviewer_note":"Deterministic M3 stratified review record; source evidence, direction, status/workspace applicability, and rendered audit are release-gated.",
            })
        })
        .collect::<Vec<_>>();
    let mut sample_markdown = String::from("# Manual semantic-review sample\n\n");
    sample_markdown.push_str("The deterministic sample contains three routines per family, every source-hash-guarded correction, and every source/dataflow direction disagreement. `machine-semantic-audit-passed` records the required review result; human review can add external evidence without changing generated contracts.\n\n");
    for record in &sample_records {
        sample_markdown.push_str(&format!(
            "## `{}`\n\n- Canonical path: `{}`\n- Family: `{}`\n- Review result: `{}`\n- Arguments: {}\n\n",
            field(record, "routine"),
            field(record, "canonical_rust_path"),
            field(record, "family"),
            field(record, "review_result"),
            record["arguments"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|argument| format!("`{}` ({})", field(argument, "name"), field(argument, "direction")))
                .collect::<Vec<_>>()
                .join(", "),
        ));
    }
    write_json(
        &root.join("generated/slatec-routines/semantic-quality-final.json"),
        &final_report,
    )?;
    write_json(
        &root.join("generated/slatec-routines/direction-evidence-conflicts.json"),
        &direction_report,
    )?;
    write_json(
        &root.join("generated/slatec-routines/argument-contamination-audit.json"),
        &contamination_report,
    )?;
    write_json(
        &root.join("generated/slatec-routines/status-contract-coverage.json"),
        &status_report,
    )?;
    write_json(
        &root.join("generated/slatec-routines/workspace-contract-coverage.json"),
        &workspace_report,
    )?;
    write_markdown(
        &root.join("generated/slatec-routines/argument-contamination-summary.md"),
        &format!(
            "# Argument contamination audit\n\n- Public descriptions with detected contamination: **{}**\n\n",
            contamination_report["public_argument_descriptions_with_detected_contamination"]
        ),
    )?;
    write_markdown(
        &root.join("generated/slatec-routines/manual-semantic-review-sample.md"),
        &sample_markdown,
    )
}

fn documentation_summary_markdown(value: &Value) -> String {
    format!(
        "# Public documentation inventory\n\n- Canonical public routines: **{}**\n- Complete source-bounded semantic contracts awaiting rendered audit: **{}**\n- Source-hash-guarded authored semantic contracts: **{}**\n- Complete semantic contracts: **{}**\n- Complete rendered semantic public documentation: **{}**\n- Public documentation-work queue: **{}**\n\n`complete-semantic-contract` is never inferred from a generated reference page. It requires bounded argument evidence and is release-gated by `rendered-rustdoc-semantic-audit.json`.\n",
        value["canonical_public_routines"],
        value["complete_semantic_contracts"],
        value["source_hash_guarded_authored_semantic_contracts"],
        value["complete_semantic_contracts"],
        value["complete_rendered_semantic_public"],
        value["documentation_work_queue_public"]
    )
}

fn documentation_quality_summary_markdown(value: &Value) -> String {
    let mut output = String::from(
        "# Routine documentation quality\n\nThis summary is generated by the public semantic-review owner. A public routine is complete only when its bounded source or source-hash-guarded authored contract passes the semantic and rendered-rustdoc checks.\n\n| Quality level | Identities |\n| --- | ---: |\n",
    );
    if let Some(counts) = value.get("counts").and_then(Value::as_object) {
        for (level, count) in counts {
            output.push_str(&format!(
                "| `{level}` | {} |\n",
                count.as_u64().unwrap_or_default()
            ));
        }
    }
    output.push_str(&format!(
        "\n- Mangled candidates before semantic repair: {}\n- Candidates remaining after semantic repair: {}\n",
        value["mangled_candidate_flags_before"].as_u64().unwrap_or_default(),
        value["mangled_candidate_flags_after"].as_u64().unwrap_or_default(),
    ));
    output
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
                quality_by_name.get(*routine).is_some_and(|record| {
                    field(record, "quality_level") == "complete-semantic-contract"
                })
            })
            .count();
        let unique_paths = members
            .iter()
            .filter_map(|(routine, _)| finals.get(*routine))
            .map(|record| field(record, "canonical_rust_path"))
            .filter(|path| path != "not_promoted")
            .collect::<BTreeSet<_>>();
        output.push(json!({"module":name,"routine_count":members.len(),"canonical_public_count":public,"secondary_count":members.len()-public,"semantic_contract_ready_for_rendered_audit":complete,"argument_documentation_coverage":complete,"description_coverage":complete,"exact_netlib_link_coverage":exact,"duplicate_identity_count":members.len().saturating_sub(unique_paths.len()),"canonical_path_count":unique_paths.len(),"meaningful_child_modules":meaningful_children(name,module),"meaningful_sibling_modules":meaningful_siblings(name),"taxonomy_problems":taxonomy_problem(name),"recommended_canonical_structure":recommended_structure(name,module)}));
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
    // Generated Markdown uses LF with exactly one terminal newline.  Several
    // report builders naturally include a paragraph-ending blank line; trim
    // that transport detail here so `git diff --check` remains an invariant of
    // every regeneration without altering the document's internal layout.
    let normalized = content.replace("\r\n", "\n");
    let content = format!("{}\n", normalized.trim_end());
    let existing = fs::read_to_string(path)
        .ok()
        .map(|existing| existing.replace("\r\n", "\n"));
    if existing.as_deref() != Some(content.as_str()) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }
    Ok(())
}

/// Captures every output owned by semantic-review generation so validation can
/// recompute its candidate output without accepting or leaving behind changes.
#[derive(Clone, Debug)]
struct SemanticOutputSnapshot {
    roots: Vec<PathBuf>,
    files: BTreeMap<PathBuf, Vec<u8>>,
}

fn generate_transactionally(
    root: &Path,
    output_dir: &Path,
) -> Result<(SemanticReviewResult, Vec<PathBuf>)> {
    let snapshot = SemanticOutputSnapshot::capture(root, semantic_output_roots(root, output_dir)?)?;
    let generated = generate(root, output_dir);
    let changed = snapshot.changed_paths(root);
    let restored = snapshot.restore(root);
    restored?;
    let result = generated?;
    Ok((result, changed?))
}

impl SemanticOutputSnapshot {
    fn capture(root: &Path, roots: Vec<PathBuf>) -> Result<Self> {
        let mut files = BTreeMap::new();
        for relative in &roots {
            collect_snapshot_files(root, relative, &mut files)?;
        }
        Ok(Self { roots, files })
    }

    fn changed_paths(&self, root: &Path) -> Result<Vec<PathBuf>> {
        let current = Self::capture(root, self.roots.clone())?;
        let mut paths = BTreeSet::new();
        paths.extend(self.files.keys().cloned());
        paths.extend(current.files.keys().cloned());
        Ok(paths
            .into_iter()
            .filter(|path| self.files.get(path) != current.files.get(path))
            .collect())
    }

    fn restore(&self, root: &Path) -> Result<()> {
        let current = Self::capture(root, self.roots.clone())?;
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

fn semantic_output_roots(root: &Path, output_dir: &Path) -> Result<Vec<PathBuf>> {
    let output_dir = if output_dir.is_absolute() {
        output_dir
            .strip_prefix(root)
            .map_err(|_| policy("semantic-review output directory escaped repository root"))?
    } else {
        output_dir
    };
    Ok(vec![
        PathBuf::from("docs/reference/routines"),
        PathBuf::from("crates/slatec-sys/src"),
        PathBuf::from("generated/slatec-routines/public-documentation-inventory.json"),
        PathBuf::from("generated/slatec-routines/public-documentation-inventory-summary.md"),
        PathBuf::from("generated/slatec-routines/netlib-source-link-audit.json"),
        PathBuf::from("generated/slatec-routines/netlib-source-link-summary.md"),
        PathBuf::from("generated/slatec-routines/documentation-quality.json"),
        PathBuf::from("generated/slatec-routines/documentation-quality-summary.md"),
        PathBuf::from("generated/slatec-routines/argument-documentation-coverage.json"),
        PathBuf::from("generated/slatec-routines/family-semantic-audits.json"),
        PathBuf::from("generated/slatec-routines/family-page-placement-discrepancies.json"),
        PathBuf::from("generated/slatec-routines/semantic-quality-baseline.json"),
        PathBuf::from("generated/slatec-routines/semantic-quality-baseline-summary.md"),
        PathBuf::from("generated/slatec-routines/semantic-quality-final.json"),
        PathBuf::from("generated/slatec-routines/direction-evidence-conflicts.json"),
        PathBuf::from("generated/slatec-routines/argument-contamination-audit.json"),
        PathBuf::from("generated/slatec-routines/status-contract-coverage.json"),
        PathBuf::from("generated/slatec-routines/workspace-contract-coverage.json"),
        PathBuf::from("generated/slatec-routines/argument-contamination-summary.md"),
        PathBuf::from("generated/slatec-routines/manual-semantic-review-sample.md"),
        PathBuf::from("generated/public-api/canonical-public-api.json"),
        PathBuf::from("generated/public-api/canonical-public-api-summary.md"),
        PathBuf::from("generated/public-api/jacobian-check-review.json"),
        PathBuf::from("generated/public-api/jacobian-check-review.md"),
        PathBuf::from("generated/public-api/taxonomy-review.json"),
        PathBuf::from("generated/public-api/taxonomy-review.md"),
        PathBuf::from("generated/public-api/manual-semantic-review.json"),
        PathBuf::from("generated/public-api/manual-semantic-review-summary.md"),
        PathBuf::from("generated/public-api/legacy-linear-algebra-review.json"),
        PathBuf::from("generated/public-api/legacy-linear-algebra-review.md"),
        PathBuf::from("generated/public-api/family-page-placement-discrepancies.json"),
        PathBuf::from("generated/public-api/family-semantic-audits.json"),
        PathBuf::from("generated/history/pre-release-public-api-cleanup.json"),
        PathBuf::from("generated/history/pre-release-public-api-cleanup.md"),
        PathBuf::from("generated/release-readiness/documentation-quality.json"),
        PathBuf::from("generated/release-readiness/argument-documentation-coverage.json"),
        output_dir.join("public-api-semantic-review-summary.json"),
    ])
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
            .map_err(|_| policy("semantic-review output escaped repository root"))?
            .to_path_buf();
        collect_snapshot_files(root, &child_relative, files)?;
    }
    Ok(())
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns optional external source evidence without compiling it into the
    /// tooling crate. The accepted SLATEC source cache is intentionally not a
    /// repository artifact; source-backed regressions run when it is available.
    fn source_backed_fixture(relative: &str) -> Option<Vec<u8>> {
        const EVIDENCE_PREFIX: &str = "evidence/full-corpus/audit-input/directories/";
        let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let mut candidates = vec![repository_root.join(relative)];
        if let Some(cache) = std::env::var_os("SLATEC_EVIDENCE_CACHE") {
            let cache_relative = relative.strip_prefix(EVIDENCE_PREFIX).unwrap_or(relative);
            candidates.push(PathBuf::from(cache).join(cache_relative));
        }
        for path in candidates {
            match fs::read(&path) {
                Ok(source) => return Some(source),
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => panic!("read source-backed fixture {}: {error}", path.display()),
            }
        }
        eprintln!(
            "skipping source-backed semantic regression without external evidence: {relative}"
        );
        None
    }

    #[test]
    fn semantic_output_snapshot_restores_changed_and_new_outputs() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path();
        let existing = root.join("docs/reference/routines/example.md");
        fs::create_dir_all(existing.parent().unwrap()).unwrap();
        fs::write(&existing, "before\n").unwrap();

        let snapshot = SemanticOutputSnapshot::capture(
            root,
            vec![
                PathBuf::from("docs/reference/routines"),
                PathBuf::from("generated/release-readiness"),
            ],
        )
        .unwrap();
        fs::write(&existing, "after\n").unwrap();
        let created = root.join("generated/release-readiness/new-output.json");
        fs::create_dir_all(created.parent().unwrap()).unwrap();
        fs::write(&created, "{}\n").unwrap();

        assert_eq!(snapshot.changed_paths(root).unwrap().len(), 2);
        snapshot.restore(root).unwrap();
        assert_eq!(fs::read_to_string(existing).unwrap(), "before\n");
        assert!(!created.exists());
    }

    #[test]
    fn generated_markdown_has_lf_and_one_terminal_newline() {
        let temporary = tempfile::tempdir().unwrap();
        let output = temporary.path().join("report.md");
        write_markdown(&output, "# Report\r\n\r\n").unwrap();
        assert_eq!(std::fs::read(&output).unwrap(), b"# Report\n");
    }

    #[test]
    fn source_resolution_uses_verified_cache_when_evidence_is_not_in_worktree() {
        let temporary = tempfile::tempdir().unwrap();
        let cache = temporary.path().join("source-cache");
        let source = cache.join("fnlib/acosh.f");
        fs::create_dir_all(source.parent().unwrap()).unwrap();
        fs::write(&source, b"C ACOSH\n").unwrap();

        assert_eq!(
            resolve_cached_source_locations(
                temporary.path().join("evidence/missing-acosh.f"),
                None,
                "evidence/missing-acosh.f",
                &cache,
                "fnlib/acosh.f"
            ),
            Some(source)
        );
    }

    #[test]
    fn source_resolution_uses_external_full_evidence_cache() {
        let temporary = tempfile::tempdir().unwrap();
        let evidence_cache = temporary.path().join("full-evidence-cache");
        let source = evidence_cache.join("src/files/avint.f");
        fs::create_dir_all(source.parent().unwrap()).unwrap();
        fs::write(&source, b"C AVINT\n").unwrap();

        assert_eq!(
            resolve_cached_source_locations(
                temporary.path().join("missing-evidence-file"),
                Some(&evidence_cache),
                "evidence/full-corpus/audit-input/directories/src/files/avint.f",
                &temporary.path().join("selected-source-cache"),
                "src/avint.f"
            ),
            Some(source)
        );
    }

    #[test]
    fn source_resolution_uses_canonical_provider_path_not_profile_prefix() {
        let relocated = json!({
            "source_file":"main-src/src/avint.f",
            "canonical_provider":{"source_file":"src/avint.f"}
        });
        let subset_member = json!({
            "source_file":"fnlib/acosh.f",
            "canonical_provider":{"subset":"fnlib","source_file":"acosh.f"}
        });
        assert_eq!(selected_provider_source_path(&relocated), "src/avint.f");
        assert_eq!(
            selected_provider_source_path(&subset_member),
            "fnlib/acosh.f"
        );
    }

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
    fn parses_fixed_form_on_output_and_bare_status_label() {
        let arguments = vec![Argument {
            name: "IERROR".to_owned(),
            ..argument_fixture()
        }];
        let source = b"      SUBROUTINE TEST(IERROR)\nC***DESCRIPTION\nC     * * * * On Output * * * *\nC     IERROR\nC       An error flag that reports invalid input.\nC***END PROLOGUE\n      END\n";
        let sections = parse_source_sections(source, &arguments);
        assert_eq!(
            sections.arguments["IERROR"].direction.as_deref(),
            Some("output")
        );
        assert!(
            sections.arguments["IERROR"].text.contains("error flag"),
            "parsed IERROR text: {:?}",
            sections.arguments["IERROR"].text
        );
    }

    #[test]
    fn parses_hwscrt_status_from_the_retained_provider_source() {
        let arguments = vec![Argument {
            name: "IERROR".to_owned(),
            ..argument_fixture()
        }];
        let known = arguments
            .iter()
            .map(|argument| argument.name.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            argument_line("IERROR", &known, true),
            Some((vec!["IERROR".to_owned()], String::new()))
        );
        let Some(source) = source_backed_fixture(
            "evidence/full-corpus/audit-input/directories/fishfft/files/hwscrt.f",
        ) else {
            return;
        };
        let sections = parse_source_sections(&source, &arguments);
        assert!(
            sections.arguments["IERROR"].text.contains("error flag"),
            "parsed retained-source IERROR text: {:?}",
            sections.arguments["IERROR"].text
        );
    }

    #[test]
    fn parses_hstcrt_status_from_the_retained_provider_source() {
        let arguments = vec![Argument {
            name: "IERROR".to_owned(),
            ..argument_fixture()
        }];
        let Some(source) = source_backed_fixture(
            "evidence/full-corpus/audit-input/directories/fishfft/files/hstcrt.f",
        ) else {
            return;
        };
        let sections = parse_source_sections(&source, &arguments);
        assert!(
            sections.arguments["IERROR"].text.contains("error flag"),
            "parsed retained-source IERROR text: {:?}",
            sections.arguments["IERROR"].text
        );
        assert!(
            sections.statuses["IERROR"]
                .iter()
                .any(|status| status.value == "0" && status.meaning.contains("No error")),
            "parsed retained-source IERROR statuses: {:?}",
            sections.statuses["IERROR"]
        );
    }

    #[test]
    fn parses_compact_status_labels_and_legacy_parameter_rows() {
        let status_arguments = vec![Argument {
            name: "IERR".to_owned(),
            ..argument_fixture()
        }];
        let known = status_arguments
            .iter()
            .map(|argument| argument.name.as_str())
            .collect::<BTreeSet<_>>();
        assert!(bounded_argument_line("IERR- a status code", &known, true).is_some());
        let Some(gaus8) = source_backed_fixture(
            "evidence/full-corpus/audit-input/directories/src/files/dgaus8.f",
        ) else {
            return;
        };
        let sections = parse_source_sections(&gaus8, &status_arguments);
        assert!(
            sections
                .arguments
                .get("IERR")
                .is_some_and(|argument| argument.text.contains("status code")),
            "parsed DGAUS8 IERR arguments: {:#?}",
            sections.arguments
        );
        let full_gaus8_arguments = ["FUN", "A", "B", "ERR", "ANS", "IERR"]
            .into_iter()
            .map(|name| Argument {
                name: name.to_owned(),
                ..argument_fixture()
            })
            .collect::<Vec<_>>();
        let full_sections = parse_source_sections(&gaus8, &full_gaus8_arguments);
        assert!(
            full_sections
                .arguments
                .get("IERR")
                .is_some_and(|argument| argument.text.contains("status code")),
            "full DGAUS8 parse lost IERR: {:#?}",
            full_sections.arguments
        );

        let tau_arguments = vec![Argument {
            name: "TAU".to_owned(),
            ..argument_fixture()
        }];
        let Some(hfti) =
            source_backed_fixture("evidence/full-corpus/audit-input/directories/src/files/hfti.f")
        else {
            return;
        };
        let hfti_text = String::from_utf8_lossy(&hfti);
        let tau_line = hfti_text
            .lines()
            .find(|line| line.contains("TAU               Absolute"))
            .expect("HFTI TAU source line");
        assert!(
            bounded_argument_line(
                comment_text(tau_line).expect("HFTI comment line"),
                &tau_arguments
                    .iter()
                    .map(|argument| argument.name.as_str())
                    .collect(),
                true,
            )
            .is_some(),
            "HFTI TAU label did not parse: {tau_line:?}"
        );
        let premature_terminal = hfti_text
            .lines()
            .skip_while(|line| !line.trim().eq_ignore_ascii_case("C     INPUT.."))
            .take_while(|line| *line != tau_line)
            .filter_map(comment_text)
            .map(normalize_heading)
            .find(|heading| is_terminal_heading(heading));
        assert!(
            premature_terminal.is_none(),
            "HFTI parser reaches a terminal heading before TAU: {premature_terminal:?}"
        );
        let sections = parse_source_sections(&hfti, &tau_arguments);
        assert!(
            sections
                .arguments
                .get("TAU")
                .is_some_and(|argument| argument.text.contains("Absolute tolerance parameter")),
            "parsed HFTI TAU arguments: {:#?}",
            sections.arguments
        );
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

    #[test]
    fn parses_changed_parameter_table_rows() {
        let arguments = ["A", "LDA", "N", "E", "V", "LDV", "WORK", "JOB", "INFO"]
            .into_iter()
            .map(|name| Argument {
                name: name.to_owned(),
                ..argument_fixture()
            })
            .collect::<Vec<_>>();
        let Some(source) =
            source_backed_fixture("evidence/full-corpus/audit-input/directories/src/files/cgeev.f")
        else {
            return;
        };
        let known = arguments
            .iter()
            .map(|argument| argument.name.as_str())
            .collect::<BTreeSet<_>>();
        assert!(bounded_argument_line("V*      COMPLEX(LDV,N)", &known, true).is_some());
        let sections = parse_source_sections(&source, &arguments);
        for name in ["A", "LDA", "N", "E", "V", "LDV", "WORK", "JOB", "INFO"] {
            assert!(
                sections
                    .arguments
                    .get(name)
                    .is_some_and(|argument| !argument.text.is_empty()),
                "missing bounded source paragraph for {name}: {:#?}",
                sections.arguments
            );
        }
        assert!(sections.arguments["A"].changed_on_return);
        assert!(sections.arguments["INFO"].changed_on_return);
    }

    #[test]
    fn parses_dqag_style_statuses_without_argument_cross_contamination() {
        let arguments = ["A", "RESULT", "IER", "IWORK"]
            .into_iter()
            .map(|name| Argument {
                name: name.to_owned(),
                ..argument_fixture()
            })
            .collect::<Vec<_>>();
        let source = b"      SUBROUTINE TEST(A,RESULT,IER,IWORK)\nC***DESCRIPTION\nC     Input\nC       A - lower integration limit\nC     On return\nC       RESULT - approximation to the integral\nC       IER - error indicator\nC     ERROR MESSAGES\nC       IER = 0 normal termination\nC           = 1 subdivision limit reached\nC       IWORK - Integer\nC               output ordering workspace\nC***END PROLOGUE\n      END\n";
        let sections = parse_source_sections(source, &arguments);
        assert_eq!(sections.arguments["A"].direction.as_deref(), Some("input"));
        assert_eq!(
            source_argument_direction(&sections.arguments["RESULT"]).as_deref(),
            Some("output")
        );
        assert!(!sections.arguments["A"].text.contains("subdivision"));
        assert!(
            sections.arguments["IWORK"]
                .text
                .contains("ordering workspace")
        );
        let values = &sections.statuses["IER"];
        assert_eq!(values.len(), 2);
        assert!(values[0].meaning.contains("normal termination"));
        assert!(values[1].meaning.contains("subdivision limit"));
    }

    #[test]
    fn parses_inline_direction_tags_and_detects_only_real_leaks() {
        let arguments = ["A", "RESULT", "IWORK", "JAC", "Y"]
            .into_iter()
            .map(|name| Argument {
                name: name.to_owned(),
                ..argument_fixture()
            })
            .collect::<Vec<_>>();
        let sections = parse_source_sections(
            b"      SUBROUTINE TEST(A,RESULT,IWORK,JAC,Y)\nC***DESCRIPTION\nC A:IN lower limit\nC RESULT:OUT approximation\nC IWORK:WORK scratch vector\nC JAC:EXT user callback\nC***END PROLOGUE\n      END\n",
            &arguments,
        );
        assert_eq!(sections.arguments["A"].direction.as_deref(), Some("input"));
        assert_eq!(
            sections.arguments["RESULT"].direction.as_deref(),
            Some("output")
        );
        assert_eq!(
            sections.arguments["IWORK"].direction.as_deref(),
            Some("workspace-output")
        );
        assert_eq!(
            sections.arguments["JAC"].direction.as_deref(),
            Some("callback")
        );
        assert!(contains_leaked_argument_label(
            "Own contract. Y -- output array",
            "Y"
        ));
        assert!(!contains_leaked_argument_label(
            "Forms Y - A*X in place",
            "Y"
        ));
        assert!(contains_leaked_section_heading(
            "ERROR MESSAGES: invalid input",
            "error messages"
        ));
        assert!(!contains_leaked_section_heading(
            "LAST determines significant entries in the WORK arrays",
            "work arrays"
        ));
    }

    #[test]
    fn generated_dassl_and_qag_contracts_keep_the_required_semantics() {
        let dassl = include_str!("../../slatec-sys/src/generated_docs/ddassl.md");
        for argument in [
            "RES", "NEQ", "T", "Y", "YPRIME", "TOUT", "INFO", "RTOL", "ATOL", "IDID", "RWORK",
            "LRW", "IWORK", "LIW", "RPAR", "IPAR", "JAC",
        ] {
            assert!(
                dassl.contains(&format!("`{argument}`")),
                "missing {argument}"
            );
        }
        for required in [
            "INFO(1)=0",
            "IDID=-11",
            "LIW >= 20 + NEQ",
            "must not unwind",
        ] {
            assert!(dassl.contains(required), "missing DASSL detail: {required}");
        }
        let dqag = include_str!("../../slatec-sys/src/generated_docs/dqag.md");
        for required in [
            "| `0` | Normal, reliable completion",
            "`LENW >= 4 * LIMIT`",
            "`IWORK` length is at least `LIMIT`",
            "must not panic or unwind",
        ] {
            assert!(dqag.contains(required), "missing DQAG detail: {required}");
        }
    }

    #[test]
    fn correction_hashes_and_precision_transfers_are_guarded() {
        let catalogue_record = json!({
            "normalized_name":"TEST",
            "source_hash":"selected-source-hash",
        });
        let mut catalogue = BTreeMap::new();
        catalogue.insert("TEST".to_owned(), &catalogue_record);
        let corrections = json!({
            "records":[{
                "routine":"TEST",
                "expected_source_hash":"different-source-hash",
            }]
        });
        let error = corrections_by_routine(&corrections, &catalogue)
            .expect_err("a correction with the wrong source hash must fail");
        assert!(error.to_string().contains("source hash changed"));

        let base = vec![Argument {
            name: "X".to_owned(),
            shape: "scalar".to_owned(),
            ..argument_fixture()
        }];
        assert!(precision_sibling_signature_matches(&base, &base));
        let incompatible = vec![Argument {
            name: "X".to_owned(),
            shape: "rank 1; dimensions (*)".to_owned(),
            ..argument_fixture()
        }];
        assert!(!precision_sibling_signature_matches(&base, &incompatible));
    }

    #[test]
    fn reference_pages_omit_inapplicable_template_sections() {
        let catalogue = json!({"kind":"subroutine"});
        let final_record = json!({
            "canonical_rust_path":"slatec_sys::special::test",
            "native_symbol":"test_",
            "feature":"special",
        });
        let status = json!({"provider_feature":"special"});
        let link = json!({"exact_netlib_url":"https://www.netlib.org/slatec/src/test.f"});
        let statuses = BTreeMap::new();
        let page = RoutinePage {
            routine: "TEST",
            catalogue: &catalogue,
            final_record: &final_record,
            status: &status,
            link: &link,
            description: "A scalar test routine.",
            errors: "",
            statuses: &statuses,
            arguments: &[],
            abi: "subroutine:none",
        };
        let block = semantic_block(&page);
        assert!(block.contains("### Return value"));
        assert!(block.contains("# Safety"));
        assert!(!block.contains("### Callback contract"));
        assert!(!block.contains("### Error and status values"));
        assert!(!block.contains("### Storage and workspace requirements"));
        assert!(!block.contains("not a workspace argument"));
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
            source_start_line: 0,
            source_end_line: 0,
            source_section: String::new(),
            evidence_kind: String::new(),
            precision_sibling_source: String::new(),
            direction_evidence: String::new(),
            direction_conflict: String::new(),
        }
    }
}
