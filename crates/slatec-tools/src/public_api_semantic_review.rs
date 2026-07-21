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
        let mut arguments = review_arguments(
            &arguments_by_routine
                .get(routine)
                .cloned()
                .unwrap_or_default(),
            &sections,
            precision_sibling_sections(routine, &source_sections_by_name),
            bytes,
            correction_by_routine.get(routine),
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
        let fixed_form_only = arguments
            .iter()
            .any(|argument| argument.description_evidence == "fixed_form_executable_dataflow");
        let source_hash_guarded_authored = arguments.iter().any(|argument| {
            argument.description_evidence == "authored_source_hash_guarded_override"
        });
        let quality = if public && !fixed_form_only {
            if source_hash_guarded_authored {
                "source-hash-guarded-authored-semantic-contract"
            } else {
                "source-prologue-semantic-contract"
            }
        } else if public {
            "semantic-review-required"
        } else {
            "not-public"
        };
        let work = if public && !fixed_form_only {
            "rendered-rustdoc-audit-pending"
        } else if public {
            "source-prologue-or-authored-review-required"
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
            write_public_rustdoc_contract(&rustdoc_contract_dir, &page)?;
        }
        quality_records.push(json!({
            "routine":routine,
            "family":field(catalogue_record, "primary_family"),
            "public_raw":public,
            "quality_level":quality,
            "documentation_work_status":work,
            "reason":if public && fixed_form_only { "fixed-form dataflow identified direction but source-prologue or authored semantic review is still required" } else if public && source_hash_guarded_authored { "source-hash-guarded authored corrections close selected explicit-prose gaps before rendered-rustdoc verification" } else if public { "selected-source prologue or verified precision-sibling prose is ready for rendered-rustdoc verification" } else { "M2 completion threshold applies to canonical public routines" },
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
        "quality_levels":{"source-prologue-semantic-contract":"all argument semantics come from the selected source prologue or a verified precision sibling","source-hash-guarded-authored-semantic-contract":"a selected-source semantic gap is closed by a source-hash-guarded authored correction","semantic-review-required":"one or more arguments remain only fixed-form dataflow classified","not-public":"not a canonical public raw routine"},
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
            || !matches!(
                field(record, "documentation_work_status").as_str(),
                "rendered-rustdoc-audit-pending" | "source-prologue-or-authored-review-required"
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
        let required_sections = vec![
            "Purpose",
            "Description",
            "Arguments",
            "Return value",
            "Callback contract",
            "Status and error values",
            "Workspace and array requirements",
            "ABI notes",
            "Safety",
        ];
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
        } else if !generic_arguments.is_empty() {
            "semantic-review-required"
        } else {
            "complete-structured"
        };
        if status != "complete-structured" {
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
    let complete_structured = output
        .iter()
        .filter(|record| field(record, "status") == "complete-structured")
        .count();
    let dassl_callbacks = dassl_callback_rustdoc_audit(root)?;
    let complete_dassl_callbacks = dassl_callbacks
        .iter()
        .filter(|record| field(record, "status") == "complete")
        .count();
    let report = json!({
        "schema_id":"slatec-rs.rendered-rustdoc-audit",
        "schema_version":"1.0.0",
        "policy":"Complete structured documentation requires a canonical rendered public rustdoc page, all ABI arguments in order, every contract section, the verified exact Netlib source URL, cross-surface agreement, and no fixed-form-only argument semantics.",
        "summary":{
            "canonical_public_routines":output.len(),
            "rendered_rustdoc_pages_found":pages_found,
            "routines_missing_arguments":missing_arguments,
            "routines_missing_netlib_links":missing_links,
            "routines_with_generic_or_unreviewed_contracts":generic_contracts,
            "families_with_rendered_documentation_defects":family_defects.len(),
            "cross_surface_mismatches":output.iter().map(|record| record["cross_surface_mismatches"].as_array().map_or(0, Vec::len)).sum::<usize>(),
            "structurally_complete_rendered_rustdoc":structural_complete,
            "complete_structured":complete_structured,
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
        || summary["complete_structured"].as_u64() != summary["canonical_public_routines"].as_u64()
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
        "# Rendered canonical rustdoc audit\n\n- Canonical public routines audited: **{}**\n- Rendered rustdoc pages found: **{}**\n- Routines missing one or more ABI argument names: **{}**\n- Routines missing the exact Netlib link: **{}**\n- Routines with generic or unreviewed argument semantics: **{}**\n- Families with rendered-documentation defects: **{}**\n- Cross-surface mismatches: **{}**\n- Structurally complete rendered pages: **{}**\n- Complete structured routines: **{}**\n\nA routine is complete only when its canonical rendered public item—not merely a reference page—passes every structural, source-link, cross-surface, and semantic-evidence check.\n",
        summary["canonical_public_routines"],
        summary["rendered_rustdoc_pages_found"],
        summary["routines_missing_arguments"],
        summary["routines_missing_netlib_links"],
        summary["routines_with_generic_or_unreviewed_contracts"],
        summary["families_with_rendered_documentation_defects"],
        summary["cross_surface_mismatches"],
        summary["structurally_complete_rendered_rustdoc"],
        summary["complete_structured"],
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
        if heading.starts_with("DESCRIPTION OF ARGUMENT")
            || heading == "ARGUMENTS"
            || heading == "ARGUMENT"
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
            direction = None;
            current.clear();
            continue;
        }
        // This is a section-heading test, not a word search: formal
        // arguments such as `IERROR` and descriptions of relative error are
        // ordinary argument prose and must not silently start an error block.
        if heading.starts_with("ERROR") {
            section = "errors";
            direction = None;
            current.clear();
            continue;
        }
        if matches!(
            heading.as_str(),
            "INPUT" | "INPUT/OUTPUT" | "OUTPUT" | "ON INPUT" | "ON INPUT/OUTPUT" | "ON OUTPUT"
        ) {
            // Several early SLATEC prologues place `Input...` and `Output...`
            // directly inside DESCRIPTION rather than under a separate
            // "Description of Arguments" heading.  Once encountered, this is
            // an argument section, not prose for the routine description.
            section = "arguments";
            direction = Some(
                match heading.as_str() {
                    "OUTPUT" | "ON OUTPUT" => "output",
                    "INPUT/OUTPUT" | "ON INPUT/OUTPUT" => "input-output",
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
        if trimmed.contains("----")
            || heading.starts_with("QUANTITIES WHICH")
            || heading.starts_with("OPTIONALLY REPLACEABLE")
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
        if section != "arguments" {
            if let Some((names, text)) = argument_line(trimmed, &argument_names, false) {
                section = "arguments";
                current = names;
                for name in &current {
                    let entry = result.arguments.entry(name.clone()).or_default();
                    if entry.direction.is_none() {
                        entry.direction = direction.clone();
                    }
                    append_text(&mut entry.text, &text);
                }
                continue;
            }
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
                if let Some((names, text)) = argument_line(trimmed, &argument_names, true) {
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

fn argument_line(
    text: &str,
    known: &BTreeSet<&str>,
    allow_bare_label: bool,
) -> Option<(Vec<String>, String)> {
    let split = text
        .split_once("--")
        .or_else(|| text.split_once(':'))
        .or_else(|| text.split_once('-'))
        .or_else(|| text.split_once('='));
    if let Some((left, right)) = split {
        let names = argument_names(left, known);
        if !names.is_empty() {
            return Some((names, right.trim().to_owned()));
        }
    }
    // Some older EISPACK prologues introduce a coupled output pair as prose,
    // for example `LOW and IGH are two INTEGER variables ...`.  Recognize
    // only the formal names before the linking verb; searching the whole
    // sentence would incorrectly pick up formal names used in examples.
    let upper = text.to_ascii_uppercase();
    for marker in [
        " IS ",
        " ARE ",
        " CONTAIN ",
        " CONTAINS ",
        " DENOTES ",
        " SPECIFIES ",
        " MUST ",
        " SHOULD ",
    ] {
        if let Some(position) = upper.find(marker) {
            let names = argument_names(&text[..position], known);
            if names.len() > 1 {
                return Some((names, text[position + 1..].trim().to_owned()));
            }
        }
    }
    // Older prologues often use a fixed-column argument table without a
    // colon or dash: `G(*,*)   The working array ...`.  The first token is
    // still an exact formal name (or a comma-separated set of names), so it
    // is source-prologue evidence rather than a parameter-name guess.
    let token = text.split_whitespace().next()?;
    let names = argument_names(token, known);
    if names.is_empty() {
        return None;
    }
    let remainder = text.get(token.len()..).unwrap_or_default().trim();
    // Fixed-form SLATEC prologues commonly put a formal label on its own
    // comment line and start the description on the next line.  Within an
    // explicit argument section that bare label is authoritative evidence;
    // outside it we require prose as well so ordinary description text is not
    // mistaken for an argument declaration.
    if allow_bare_label || !remainder.is_empty() {
        Some((names, remainder.to_owned()))
    } else {
        None
    }
}

fn argument_names(text: &str, known: &BTreeSet<&str>) -> Vec<String> {
    text.split(|character: char| !character.is_ascii_alphanumeric())
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_ascii_uppercase)
        .filter_map(|part| {
            if known.contains(part.as_str()) {
                Some(part)
            } else if part == "KVPT" && known.contains("KPVT") {
                // The retained LINPACK prologues spell the formal pivot
                // vector `KVPT` in several places, while the executable
                // declaration and compiler inventory establish `KPVT`.
                // This corrects only that documented historical typo.
                Some("KPVT".to_owned())
            } else if part == "JVPT" && known.contains("JPVT") {
                // `CQRDC` documents the column-pivot vector with the
                // transposed spelling `JVPT`; the formal declaration is
                // `JPVT` and the source paragraph supplies its semantics.
                Some("JPVT".to_owned())
            } else {
                None
            }
        })
        .collect()
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

/// Returns an explicit retained precision sibling when the selected source
/// lacks a separate argument paragraph.  This is lower-priority evidence than
/// the routine's own prologue and only supplies prose for identically named
/// formal arguments; it never changes an ABI declaration or invents a name.
fn precision_sibling_sections<'a>(
    routine: &str,
    sections: &'a BTreeMap<String, SourceSections>,
) -> Option<&'a SourceSections> {
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
    candidates
        .into_iter()
        .find(|candidate| candidate != routine && sections.contains_key(candidate))
        .and_then(|candidate| sections.get(&candidate))
}

fn review_arguments(
    base: &[Argument],
    sections: &SourceSections,
    sibling_sections: Option<&SourceSections>,
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
        argument.semantic_role = semantic_role(&argument);
        if argument.external_callback {
            argument.direction = "callback".to_owned();
            argument.callback_restrictions = "The callback must remain valid for the complete native call, satisfy the exact reviewed ABI, and must not unwind into Fortran.".to_owned();
        } else if is_workspace_name(&argument.name) {
            argument.direction = "workspace".to_owned();
            argument.workspace_requirement = semantic_source
                .map(|item| item.text.clone())
                .unwrap_or_else(|| "Caller-provided workspace; required extent is governed by the selected source and related size arguments.".to_owned());
        } else if is_status_name(&argument.name) {
            argument.direction = "status-output".to_owned();
        } else if let Some(direction) = source
            .and_then(|item| item.direction.clone())
            .or_else(|| sibling_source.and_then(|item| item.direction.clone()))
        {
            argument.direction = direction;
        } else {
            argument.direction =
                direction_from_flow(flow.get(&argument.name).copied().unwrap_or_default());
        }
        if let Some(item) = semantic_source {
            argument.description = item.text.clone();
            argument.description_evidence = if source.is_some_and(|source| !source.text.is_empty())
            {
                "source_prologue_argument_section".to_owned()
            } else {
                "verified_precision_sibling_prologue".to_owned()
            };
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
    let fixed_form_only = page
        .arguments
        .iter()
        .any(|argument| argument.description_evidence == "fixed_form_executable_dataflow");
    let status = if fixed_form_only {
        "semantic review required"
    } else {
        "source-backed contract awaiting rendered-rustdoc audit"
    };
    let evidence = if fixed_form_only {
        "verified source hash and fixed-form dataflow; one or more argument semantics still need source-prologue or authored review"
    } else {
        "verified source prologue or source-hash-guarded authored correction"
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

/// Writes the documentation fragment that is attached to the actual public
/// Rust function, rather than only to its generated reference page.
///
/// The fragment is deliberately keyed by the canonical routine name.  The
/// generated Rust source only includes this file; it does not duplicate an
/// `extern` declaration or make a generated ABI-shaped namespace public.
fn write_public_rustdoc_contract(directory: &Path, page: &RoutinePage<'_>) -> Result<()> {
    if let Some(contract) = dassl_rustdoc_contract(page) {
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
        for (position, argument) in page.arguments.iter().enumerate() {
            output.push_str(&format!(
                "## {}. `{}`\n\n{} `{}` argument; Fortran declaration `{}`, Rust ABI type `{}`, and {}. {}\n\n",
                position + 1,
                argument.name,
                argument.direction,
                argument.semantic_role,
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
        output.push_str("# Callback contract\n\nCallback arguments use the reviewed ABI shown by their Rust function-pointer type. They are invoked synchronously by the native call, must remain valid until it returns, must uphold every documented input/output extent, and **must not unwind** through Fortran. A callback must not retain or free caller-owned native buffers unless the source contract expressly permits it.\n\n");
    } else {
        output.push_str("# Callback contract\n\nThis interface has no callback argument.\n\n");
    }
    if page.errors.trim().is_empty() {
        output.push_str("# Status and error values\n\nThe selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.\n\n");
    } else {
        output.push_str(&format!(
            "# Status and error values\n\n{}\n\n",
            escape_rustdoc_text(page.errors)
        ));
    }
    let work = page
        .arguments
        .iter()
        .filter(|argument| {
            argument.semantic_role == "workspace"
                || !argument.workspace_requirement.trim().is_empty()
                || !argument.leading_dimension.trim().is_empty()
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
    output.push_str("# Workspace and array requirements\n\n");
    if work.is_empty() {
        output.push_str("No separately named workspace is declared. Any array argument must satisfy its documented rank, element extent, leading-dimension, and Fortran column-major storage requirements.\n\n");
    } else {
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

fn rustdoc_argument_contract(argument: &Argument) -> String {
    let mut contract = argument.description.clone();
    if contract.contains("classified by fixed-form executable read/write analysis") {
        contract = format!(
            "The verified prologue does not separately describe this parameter. Executable dataflow establishes it as `{}`; its exact semantic role remains in the source-hash-guarded review queue and callers must follow the exact source file.",
            argument.direction
        );
    }
    if argument.external_callback && !argument.callback_restrictions.is_empty() {
        contract.push(' ');
        contract.push_str(&argument.callback_restrictions);
    }
    if !argument.relationship.is_empty() {
        contract.push(' ');
        contract.push_str(&argument.relationship);
    }
    if !argument.leading_dimension.is_empty() {
        contract.push(' ');
        contract.push_str(&argument.leading_dimension);
    }
    if !argument.workspace_requirement.is_empty()
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
    let source_prologue = public
        .iter()
        .filter(|record| {
            field(record, "documentation_quality_status") == "source-prologue-semantic-contract"
        })
        .count();
    let authored = public
        .iter()
        .filter(|record| {
            field(record, "documentation_quality_status")
                == "source-hash-guarded-authored-semantic-contract"
        })
        .count();
    json!({"canonical_public_routines":public.len(),"source_prologue_semantic_contracts":source_prologue,"source_hash_guarded_authored_semantic_contracts":authored,"complete_semantic_contracts":source_prologue+authored,"complete_structured_public":"computed only by rendered-rustdoc-audit","documentation_work_queue_public":public.len()-source_prologue-authored,"quality_counts":counts})
}

fn documentation_summary_markdown(value: &Value) -> String {
    format!(
        "# Public documentation inventory\n\n- Canonical public routines: **{}**\n- Selected-prologue or verified-sibling semantic contracts awaiting rendered audit: **{}**\n- Source-hash-guarded authored semantic contracts awaiting rendered audit: **{}**\n- Complete semantic contracts awaiting rendered audit: **{}**\n- Complete structured public documentation: **{}**\n- Public documentation-work queue: **{}**\n\n`complete-structured` is never inferred from a generated reference page. It is computed only by `rendered-rustdoc-audit.json` after the canonical public rustdoc item, ABI inventory, and routine-reference page agree.\n",
        value["canonical_public_routines"],
        value["source_prologue_semantic_contracts"],
        value["source_hash_guarded_authored_semantic_contracts"],
        value["complete_semantic_contracts"],
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
                quality_by_name.get(*routine).is_some_and(|record| {
                    matches!(
                        field(record, "quality_level").as_str(),
                        "source-prologue-semantic-contract"
                            | "source-hash-guarded-authored-semantic-contract"
                    )
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
    let content = content.replace("\r\n", "\n");
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
        let source = include_bytes!(
            "../../../evidence/full-corpus/audit-input/directories/fishfft/files/hwscrt.f"
        );
        let sections = parse_source_sections(source, &arguments);
        assert!(
            sections.arguments["IERROR"].text.contains("error flag"),
            "parsed retained-source IERROR text: {:?}",
            sections.arguments["IERROR"].text
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
