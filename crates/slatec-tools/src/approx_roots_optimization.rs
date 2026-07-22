//! Deterministic domain-completion inventory for approximation, roots, and optimization.
//!
//! This report joins authoritative raw-status records with the generated
//! raw-to-safe reconciliation. It is deliberately a disposition inventory,
//! not a claim that every retained raw program unit has an equivalent safe
//! facade.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const ROOT_ROUTINES: &[&str] = &["FZERO", "DFZERO", "CPQR79", "RPQR79", "CPZERO", "RPZERO"];
const POLYNOMIAL_ROOT_ROUTINES: &[&str] = &["CPQR79", "RPQR79", "CPZERO", "RPZERO"];
const CURRENT_MILESTONE_ROUTINES: &[&str] = &[
    "BINT4", "DBINT4", "CPQR79", "RPQR79", "CPZERO", "RPZERO", "POLFIT", "DPOLFT", "PVALUE",
    "DP1VLU", "PCOEF", "DPCOEF",
];
const OBJECTIVE_MINIMIZATION_ROUTINES: &[&str] = &[];
const POLYNOMIAL_FIT_ROUTINES: &[&str] =
    &["POLFIT", "DPOLFT", "PVALUE", "DP1VLU", "PCOEF", "DPCOEF"];
const SPLINE_FIT_ROUTINES: &[&str] = &[
    "FC", "DFC", "EFC", "DEFC", "FCMN", "DFCMN", "EFCMN", "DEFCMN",
];
const POLYNOMIAL_REPRESENTATION_ROUTINES: &[&str] = &[
    "POLINT", "DPLINT", "POLYVL", "DPOLVL", "POLCOF", "DPOLCF", "PPVAL", "DPPVAL",
];

/// Result of generating the three domain inventories and their aggregate view.
#[derive(Debug)]
pub struct GenerationResult {
    /// Number of distinct routine records across all requested domains.
    pub routine_count: usize,
    /// Stable content hash over every emitted report.
    pub semantic_hash: String,
    /// Directory holding the generated reports.
    pub output_dir: PathBuf,
}

/// Generates the complete safe-coverage disposition reports for this milestone.
pub fn generate(output_dir: &Path) -> Result<GenerationResult> {
    let status = records(&repo_path("generated/raw-api/routine-status.json"))?;
    let taxonomy = taxonomy_document(&status)?;
    let coverage = records(&repo_path("generated/safe-api/raw-to-safe-coverage.json"))?
        .into_iter()
        .map(|record| Ok((string(&record, "raw_routine")?.to_owned(), record)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let mut roots_audit = status
        .iter()
        .filter(|raw| {
            string(raw, "routine")
                .ok()
                .is_some_and(|routine| ROOT_ROUTINES.contains(&routine))
                || string(raw, "primary_family").ok() == Some("Roots")
        })
        .map(|raw| roots_audit_record(raw, coverage.get(string(raw, "routine")?)))
        .collect::<Result<Vec<_>>>()?;
    roots_audit.sort_by(|left, right| left["routine"].as_str().cmp(&right["routine"].as_str()));

    let mut by_domain = BTreeMap::<&str, Vec<Value>>::from([
        ("approximation", Vec::new()),
        ("complex-root", Vec::new()),
        ("nonlinear-optimization", Vec::new()),
    ]);
    for raw in &status {
        let Some(domain) = domain(raw)? else {
            continue;
        };
        let record = domain_record(raw, coverage.get(string(raw, "routine")?))?;
        by_domain
            .get_mut(domain)
            .ok_or_else(|| CorpusError::Verification(format!("unknown report domain {domain}")))?
            .push(record);
    }
    for routines in by_domain.values_mut() {
        routines.sort_by(|left, right| left["routine"].as_str().cmp(&right["routine"].as_str()));
        validate_records(routines)?;
    }

    let all = by_domain
        .values()
        .flat_map(|records| records.iter().cloned())
        .collect::<Vec<_>>();
    let documents = [
        (
            "approx-roots-optimization-coverage.json",
            json_document("all", &all),
        ),
        (
            "approximation-complete-coverage.json",
            json_document("approximation", &by_domain["approximation"]),
        ),
        (
            "complex-root-complete-coverage.json",
            json_document("complex-root", &by_domain["complex-root"]),
        ),
        (
            "nonlinear-optimization-complete-coverage.json",
            json_document(
                "nonlinear-optimization",
                &by_domain["nonlinear-optimization"],
            ),
        ),
    ];

    fs::create_dir_all(output_dir)?;
    let mut semantic = Vec::new();
    for (name, document) in documents {
        let encoded = serde_json::to_vec(&document)?;
        fs::write(output_dir.join(name), &encoded)?;
        semantic.extend_from_slice(&encoded);
        let markdown = render_markdown(&document)?;
        let markdown_name = name.replace(".json", ".md");
        fs::write(output_dir.join(markdown_name), markdown.as_bytes())?;
        semantic.extend_from_slice(markdown.as_bytes());
    }
    let taxonomy_bytes = serde_json::to_vec(&taxonomy)?;
    fs::write(output_dir.join("coverage-taxonomy.json"), &taxonomy_bytes)?;
    semantic.extend_from_slice(&taxonomy_bytes);
    let taxonomy_markdown = render_taxonomy_markdown(&taxonomy)?;
    fs::write(
        output_dir.join("coverage-taxonomy.md"),
        taxonomy_markdown.as_bytes(),
    )?;
    semantic.extend_from_slice(taxonomy_markdown.as_bytes());
    let roots_bytes = serde_json::to_vec(&json!({
        "schema_id":"slatec.safe-api.roots-family-audit",
        "schema_version":"1.0.0",
        "scope":"all explicitly selected roots drivers plus every retained identity whose authoritative primary family is Roots",
        "records":roots_audit,
    }))?;
    fs::write(output_dir.join("roots-family-audit.json"), &roots_bytes)?;
    semantic.extend_from_slice(&roots_bytes);
    let roots_markdown = render_roots_audit_markdown(&roots_audit)?;
    fs::write(
        output_dir.join("roots-family-audit.md"),
        roots_markdown.as_bytes(),
    )?;
    semantic.extend_from_slice(roots_markdown.as_bytes());
    Ok(GenerationResult {
        routine_count: all.len(),
        semantic_hash: hash::bytes(&semantic),
        output_dir: output_dir.to_owned(),
    })
}

fn domain(raw: &Value) -> Result<Option<&'static str>> {
    let routine = string(raw, "routine")?;
    let family = string(raw, "primary_family")?;
    if matches!(family, "Approximation" | "Interpolation") {
        Ok(Some("approximation"))
    } else if ROOT_ROUTINES.contains(&routine) {
        Ok(Some("complex-root"))
    } else if OBJECTIVE_MINIMIZATION_ROUTINES.contains(&routine) {
        Ok(Some("nonlinear-optimization"))
    } else {
        Ok(None)
    }
}

fn taxonomy_class(raw: &Value) -> Result<&'static str> {
    let routine = string(raw, "routine")?;
    let family = string(raw, "primary_family")?;
    if matches!(routine, "FZERO" | "DFZERO") {
        Ok("scalar_roots")
    } else if ROOT_ROUTINES.contains(&routine) {
        Ok("polynomial_roots")
    } else if matches!(
        routine,
        "SNSQ" | "DNSQ" | "SNSQE" | "DNSQE" | "SOS" | "DSOS" | "CHKDER" | "DCKDER"
    ) {
        Ok("nonlinear_systems_and_jacobian_checking")
    } else if matches!(
        routine,
        "SNLS1" | "DNLS1" | "SNLS1E" | "DNLS1E" | "SCOV" | "DCOV"
    ) {
        Ok("nonlinear_least_squares")
    } else if matches!(
        routine,
        "WNNLS" | "DWNNLS" | "SBOLS" | "DBOLS" | "LSEI" | "DLSEI" | "SBOCLS" | "DBOCLS"
    ) {
        Ok("linear_least_squares")
    } else if matches!(routine, "SPLP" | "DSPLP") {
        Ok("linear_programming")
    } else if OBJECTIVE_MINIMIZATION_ROUTINES.contains(&routine) {
        Ok("objective_minimization_or_nonlinear_programming")
    } else if POLYNOMIAL_FIT_ROUTINES.contains(&routine) {
        Ok("polynomial_approximation_and_fitting")
    } else if SPLINE_FIT_ROUTINES.contains(&routine) {
        Ok("stateful_spline_fitting")
    } else if POLYNOMIAL_REPRESENTATION_ROUTINES.contains(&routine) {
        Ok("polynomial_representation_and_evaluation")
    } else if family == "Interpolation" {
        Ok("spline_representation_and_construction")
    } else if family == "Approximation" {
        Ok("approximation_and_fitting_support")
    } else {
        Ok("other")
    }
}

fn taxonomy_document(status: &[Value]) -> Result<Value> {
    let mut counts = BTreeMap::<String, usize>::new();
    let mut records = Vec::new();
    for raw in status {
        let class = taxonomy_class(raw)?;
        *counts.entry(class.to_owned()).or_default() += 1;
        records.push(json!({
            "routine":string(raw, "routine")?,
            "taxonomy_class":class,
            "primary_family":string(raw, "primary_family")?,
            "historical_role":string(raw, "historical_role")?,
            "raw_api_state":string(raw, "raw_api_state")?,
        }));
    }
    records.sort_by(|left, right| left["routine"].as_str().cmp(&right["routine"].as_str()));
    Ok(json!({
        "schema_id":"slatec.safe-api.coverage-taxonomy",
        "schema_version":"1.0.0",
        "scope":"complete retained raw-status inventory classified without treating equations, least squares, or linear programming as nonlinear objective minimization",
        "objective_minimization_or_nonlinear_programming":{
            "record_count":OBJECTIVE_MINIMIZATION_ROUTINES.len(),
            "conclusion":"no suitable retained SLATEC high-level driver",
            "excluded_neighboring_capabilities":["scalar and polynomial roots","nonlinear systems","nonlinear least squares","linear least squares","linear programming","interpolation","approximation and fitting","polynomial representation and evaluation","spline representation and construction"]
        },
        "counts":counts,
        "records":records,
    }))
}

fn render_taxonomy_markdown(document: &Value) -> Result<String> {
    let counts = document["counts"]
        .as_object()
        .ok_or_else(|| CorpusError::Verification("coverage taxonomy lacks counts".to_owned()))?;
    let mut markdown = String::from(
        "# Safe-coverage taxonomy\n\nThis taxonomy separates mathematical capabilities before safe-coverage counts are interpreted. In particular, nonlinear equations, nonlinear least squares, linear least squares, and linear programming are not nonlinear objective minimization.\n\n## Category counts\n\n",
    );
    for (class, count) in counts {
        markdown.push_str(&format!("- `{class}`: {count}\n"));
    }
    let objective = &document["objective_minimization_or_nonlinear_programming"];
    markdown.push_str(&format!(
        "\n## Objective minimization and nonlinear programming\n\n- Retained suitable drivers: {}.\n- Disposition: **{}**.\n- `SPLP`/`DSPLP` remain correctly classified as linear programming; the other listed neighboring capabilities are reported separately and are excluded from the nonlinear-objective count.\n",
        objective["record_count"].as_u64().unwrap_or_default(),
        objective["conclusion"].as_str().unwrap_or("unavailable"),
    ));
    Ok(markdown)
}

fn roots_audit_record(raw: &Value, coverage: Option<&Value>) -> Result<Value> {
    let routine = string(raw, "routine")?;
    let runtime_test_status = coverage_runtime_test_status(raw)?;
    let root_class = match routine {
        "FZERO" | "DFZERO" => "scalar_callback_driver",
        "RPZERO" | "CPZERO" | "RPQR79" | "CPQR79" => "polynomial_root_driver",
        _ => "roots_by_primary_family_or_inherited_evidence",
    };
    let safe_path = coverage
        .and_then(|record| record["existing_safe_public_path"].as_str())
        .filter(|path| !path.is_empty())
        .unwrap_or("none");
    Ok(json!({
        "routine":routine,
        "root_class":root_class,
        "historical_role":string(raw, "historical_role")?,
        "canonical_provider":string(raw, "canonical_provider")?,
        "source_hash":string(raw, "source_hash")?,
        "native_symbol":string(raw, "native_symbol")?,
        "generated_declaration_status":string(raw, "generated_declaration_status")?,
        "reviewed_declaration_status":string(raw, "reviewed_declaration_status")?,
        "current_rust_path":safe_path,
        "intended_canonical_rust_path":string(raw, "canonical_rust_path")?,
        "provider_feature":string(raw, "provider_feature")?,
        "documentation_status":string(raw, "documentation_status")?,
        "compile_test_status":string(raw, "compile_test_status")?,
        "link_test_status":string(raw, "link_test_status")?,
        "raw_inventory_runtime_test_status":string(raw, "runtime_test_status")?,
        "runtime_test_status":runtime_test_status,
        "runtime_test_evidence":runtime_test_evidence(routine),
        "safe_wrapper_status":string(raw, "safe_wrapper_status")?,
        "safe_coverage_kind":coverage.and_then(|record| record["coverage_kind"].as_str()).unwrap_or("not_in_public_raw_coverage"),
        "exclusion_or_deferment_reason":coverage.and_then(|record| record["reason_no_direct_wrapper"].as_str()).unwrap_or("authoritative raw status has no canonical public coverage record"),
    }))
}

fn render_roots_audit_markdown(records: &[Value]) -> Result<String> {
    let mut markdown = String::from(
        "# Roots-family audit\n\nThis report is sourced from the explicitly selected roots drivers plus every retained identity classified under the authoritative Roots family. `FZERO`/`DFZERO` are scalar callback drivers; the f32/Complex32 polynomial-root drivers are reported separately; other identities are retained only by their family evidence unless independently reviewed. `CPZERO`, `RPZERO`, `CPQR79`, and `RPQR79` have focused native safe-wrapper evidence in `polynomial_roots_native`; the raw-inventory historical status remains available separately in the JSON record. No true scalar complex callback driver, f64 polynomial-root driver, or Complex64 polynomial-root driver is present in the selected retained scope, and none is inferred.\n\n| Routine | Class | Raw path | Safe path | Provider | Docs | Link | Focused runtime | Safe coverage | Deferment |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for record in records {
        markdown.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
            string(record, "routine")?,
            string(record, "root_class")?,
            string(record, "intended_canonical_rust_path")?,
            string(record, "current_rust_path")?,
            string(record, "provider_feature")?,
            string(record, "documentation_status")?,
            string(record, "link_test_status")?,
            string(record, "runtime_test_status")?,
            string(record, "safe_coverage_kind")?,
            string(record, "exclusion_or_deferment_reason")?,
        ));
    }
    Ok(markdown)
}

fn domain_record(raw: &Value, coverage: Option<&Value>) -> Result<Value> {
    let routine = string(raw, "routine")?;
    let coverage_kind = coverage
        .and_then(|record| record["coverage_kind"].as_str())
        .unwrap_or("not_in_public_raw_coverage");
    let safe_path = coverage
        .and_then(|record| record["existing_safe_public_path"].as_str())
        .filter(|path| !path.is_empty())
        .unwrap_or("none");
    let safe_feature = coverage
        .and_then(|record| record["safe_feature"].as_str())
        .filter(|feature| !feature.is_empty())
        .unwrap_or("none");
    let final_disposition = disposition(routine, string(raw, "raw_api_state")?, coverage_kind);
    let source_file = string(raw, "source_file")?;
    let runtime_test_status = coverage_runtime_test_status(raw)?;
    Ok(json!({
        "domain":domain(raw)?.unwrap_or("unclassified"),
        "taxonomy_class":taxonomy_class(raw)?,
        "routine":routine,
        "program_unit_kind":string(raw, "program_unit_kind")?,
        "historical_role":string(raw, "historical_role")?,
        "driver_role":string(raw, "driver_role")?,
        "canonical_provider":string(raw, "canonical_provider")?,
        "source_file":source_file,
        "source_hash":string(raw, "source_hash")?,
        "netlib_source_url":netlib_url(source_file),
        "canonical_raw_path":string(raw, "canonical_rust_path")?,
        "native_symbol":string(raw, "native_symbol")?,
        "symbol_status":string(raw, "symbol_status")?,
        "raw_api_state":string(raw, "raw_api_state")?,
        "generated_declaration_status":string(raw, "generated_declaration_status")?,
        "reviewed_declaration_status":string(raw, "reviewed_declaration_status")?,
        "abi_review_status":string(raw, "signature_review_status")?,
        "safe_path":safe_path,
        "safe_feature":safe_feature,
        "raw_feature":string(raw, "feature")?,
        "provider_feature":string(raw, "provider_feature")?,
        "coverage_kind":coverage_kind,
        "direct_callers":call_graph_unavailable("callers"),
        "direct_callees":call_graph_unavailable("callees"),
        "callback_contract":callback_contract(routine),
        "mutation_contract":mutation_contract(routine),
        "workspace_contract":workspace_contract(routine),
        "native_state_contract":native_state_contract(raw, safe_path),
        "provider_compatibility":provider_compatibility(raw),
        "documentation_status":string(raw, "documentation_status")?,
        "argument_documentation_status":string(raw, "argument_documentation_status")?,
        "compile_test_status":string(raw, "compile_test_status")?,
        "link_test_status":string(raw, "link_test_status")?,
        "raw_inventory_runtime_test_status":string(raw, "runtime_test_status")?,
        "runtime_test_status":runtime_test_status.clone(),
        "runtime_test_evidence":runtime_test_evidence(routine),
        "safe_wrapper_status_raw_inventory":string(raw, "safe_wrapper_status")?,
        "final_disposition":final_disposition,
        "rationale":rationale(routine, final_disposition, coverage_kind),
        "validation_status":validation_status(raw, &runtime_test_status, safe_path),
    }))
}

fn disposition(routine: &str, raw_state: &str, coverage_kind: &str) -> &'static str {
    if CURRENT_MILESTONE_ROUTINES.contains(&routine) {
        "candidate-implemented-in-this-milestone"
    } else if coverage_kind == "direct-safe-wrapper" {
        "direct-safe-wrapper"
    } else if coverage_kind == "covered-by-higher-level-safe-api" {
        "covered-by-higher-level-safe-api"
    } else if coverage_kind == "internal-or-subsidiary" {
        "internal-subsidiary"
    } else if raw_state.starts_with("unsupported_") || raw_state == "ambiguous_symbol" {
        "blocked-by-abi"
    } else if raw_state == "catalogue_only" {
        "blocked-by-incomplete-contract"
    } else if matches!(routine, "FC" | "DFC") {
        "deferred-stateful-constrained-bspline"
    } else if matches!(routine, "EFC" | "DEFC") {
        "deferred-incremental-bspline-preprocessing"
    } else {
        "expert-raw-only"
    }
}

fn rationale(routine: &str, disposition: &str, coverage_kind: &str) -> &'static str {
    if CURRENT_MILESTONE_ROUTINES.contains(&routine) {
        "This milestone supplies a checked owned workflow and focused native validation; the raw declaration remains independently available."
    } else if disposition == "direct-safe-wrapper" {
        "A previously reviewed safe function is recorded by the canonical raw-to-safe reconciliation."
    } else if disposition == "covered-by-higher-level-safe-api" {
        "The raw routine remains public for expert use while an owned higher-level safe abstraction owns the supported workflow."
    } else if disposition == "internal-subsidiary" {
        "The authoritative raw status and safe-coverage reconciliation classify this routine as support machinery rather than a standalone safe workflow."
    } else if disposition == "blocked-by-abi" {
        "The authoritative raw status does not establish a safe ABI contract for promotion."
    } else if disposition == "deferred-stateful-constrained-bspline" {
        "The constrained B-spline fit retains persistent W/IW state, optional variance state, and caller-described constraints; no coherent owned workflow has been established."
    } else if disposition == "deferred-incremental-bspline-preprocessing" {
        "The incremental preprocessing contract shares persistent state with constrained B-spline fitting and cannot be represented as a stateless polynomial-fit API."
    } else if coverage_kind == "not_in_public_raw_coverage" {
        "The complete raw-status inventory retains this identity, but it is not a canonical public raw record and no safe workflow is inferred."
    } else {
        "A coherent checked safe workflow has not been selected; this record is an explicit expert-raw-only disposition, not an omission."
    }
}

fn coverage_runtime_test_status(raw: &Value) -> Result<String> {
    if POLYNOMIAL_ROOT_ROUTINES.contains(&string(raw, "routine")?) {
        Ok("passed_focused_polynomial_roots_native".to_owned())
    } else {
        Ok(string(raw, "runtime_test_status")?.to_owned())
    }
}

fn runtime_test_evidence(routine: &str) -> &'static str {
    if POLYNOMIAL_ROOT_ROUTINES.contains(&routine) {
        "focused GNU MinGW x86_64 native regression: manufactured real and complex roots, residuals, repeated-root case, all four safe/raw parity probes, and status-mapping unit checks"
    } else {
        "authoritative raw inventory status"
    }
}

fn validation_status(raw: &Value, runtime_test_status: &str, safe_path: &str) -> &'static str {
    if safe_path != "none" && runtime_test_status.starts_with("passed") {
        "safe_path_and_native_runtime_evidence_recorded"
    } else if safe_path != "none" {
        "safe_path_recorded_native_runtime_evidence_pending_or_representative"
    } else if string(raw, "link_test_status").ok() == Some("passed") {
        "raw_link_evidence_recorded_safe_promotion_not_selected"
    } else {
        "no_safe_validation_claim"
    }
}

fn callback_contract(routine: &str) -> &'static str {
    match routine {
        "FZERO" | "DFZERO" => "scalar_callback_contained_by_existing_roots_facade",
        "SOS" | "DSOS" | "SNSQ" | "DNSQ" | "SNSQE" | "DNSQE" | "DNLS1" | "SNLS1" => {
            "callback-bearing_solver_contract_recorded_in_family_metadata"
        }
        "CPQR79" | "RPQR79" | "CPZERO" | "RPZERO" | "BINT4" | "DBINT4" => "none",
        _ => "not_extracted_in_this_domain_inventory; consult_source_prologue",
    }
}

fn mutation_contract(routine: &str) -> &'static str {
    match routine {
        "BINT4" | "DBINT4" => {
            "nodes_and_values_are_copied_or_read_only; outputs_and_workspace_are_private"
        }
        "CPQR79" | "RPQR79" | "CPZERO" | "RPZERO" => {
            "coefficient_input_is_copied; roots_bounds_and_workspace_are_private"
        }
        _ => "source_prologue_or_existing_safe_audit_required; no_inferred_intent",
    }
}

fn workspace_contract(routine: &str) -> &'static str {
    match routine {
        "BINT4" | "DBINT4" => "T=NDATA+6; BCOEF=NDATA+2; W=5*(NDATA+2)",
        "RPQR79" => "NDEG*(NDEG+2) private real values",
        "CPQR79" => "2*NDEG*(NDEG+1) private real values",
        "RPZERO" => "6*(NDEG+1) private Complex32 values plus NDEG private f32 bounds",
        "CPZERO" => "4*(NDEG+1) private Complex32 values plus NDEG private f32 bounds",
        _ => "not_compacted_here; preserve_source_prologue_workspace_contract",
    }
}

fn native_state_contract(raw: &Value, safe_path: &str) -> &'static str {
    if safe_path == "none" {
        "safe_wrapper_not_selected; raw_state_is_authoritative"
    } else if string(raw, "primary_family").ok() == Some("Interpolation")
        || string(raw, "primary_family").ok() == Some("Approximation")
    {
        "process_global_native_runtime_lock_for_reviewed_safe_entry"
    } else {
        "process_global_native_runtime_lock_or_existing_family_policy"
    }
}

fn provider_compatibility(raw: &Value) -> String {
    format!(
        "raw feature {}; provider feature {}; slatec-sys remains provider-neutral",
        raw["feature"].as_str().unwrap_or("unavailable"),
        raw["provider_feature"].as_str().unwrap_or("unavailable")
    )
}

fn call_graph_unavailable(direction: &str) -> Value {
    json!({
        "status":"unavailable_in_committed_authoritative_inputs",
        "direction":direction,
        "provenance":"The retained raw-status and raw-to-safe inventories do not encode a complete Fortran call graph; this report does not infer callers or callees from routine names."
    })
}

fn netlib_url(source_file: &str) -> String {
    let path = source_file.strip_prefix("main-src/").unwrap_or(source_file);
    format!("https://www.netlib.org/slatec/{path}")
}

fn json_document(domain: &str, records: &[Value]) -> Value {
    let mut dispositions = BTreeMap::<String, usize>::new();
    for record in records {
        if let Some(disposition) = record["final_disposition"].as_str() {
            *dispositions.entry(disposition.to_owned()).or_default() += 1;
        }
    }
    json!({
        "schema_id":"slatec.safe-api.approx-roots-optimization-coverage",
        "schema_version":"1.0.0",
        "domain":domain,
        "record_count":records.len(),
        "disposition_counts":dispositions,
        "records":records,
    })
}

fn render_markdown(document: &Value) -> Result<String> {
    let domain = string(document, "domain")?;
    let records = document["records"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("domain report lacks records".to_owned()))?;
    let scope = if domain == "nonlinear-optimization" {
        "This report intentionally includes only genuine retained nonlinear objective-minimization or nonlinear-programming drivers. Nonlinear systems, derivative checks, nonlinear least squares, linear least squares, and linear programming are excluded and classified in `coverage-taxonomy.json`. The selected corpus currently has no suitable retained driver."
    } else {
        "This generated inventory joins canonical raw status with raw-to-safe coverage. `expert-raw-only` and blocked records are explicit decisions, not missing data. Call-graph fields are recorded as unavailable where the committed authoritative inputs do not contain a complete Fortran call graph."
    };
    let mut markdown =
        format!("# {domain} safe-coverage disposition\n\n{scope}\n\n## Disposition counts\n\n");
    for (disposition, count) in document["disposition_counts"]
        .as_object()
        .ok_or_else(|| CorpusError::Verification("domain report lacks counts".to_owned()))?
    {
        markdown.push_str(&format!("- `{disposition}`: {count}\n"));
    }
    markdown.push_str("\n## Routine records\n\n| Routine | Role | Raw path | Safe path | Provider feature | Docs | Link | Runtime | Disposition |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for record in records {
        markdown.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            string(record, "routine")?,
            string(record, "driver_role")?,
            string(record, "canonical_raw_path")?,
            string(record, "safe_path")?,
            string(record, "provider_feature")?,
            string(record, "documentation_status")?,
            string(record, "link_test_status")?,
            string(record, "runtime_test_status")?,
            string(record, "final_disposition")?,
        ));
    }
    Ok(markdown)
}

fn validate_records(records: &[Value]) -> Result<()> {
    for record in records {
        for field in [
            "routine",
            "canonical_provider",
            "source_hash",
            "canonical_raw_path",
            "native_symbol",
            "final_disposition",
            "rationale",
        ] {
            if string(record, field)?.is_empty() {
                return Err(CorpusError::Verification(format!(
                    "domain completion record lacks {field}"
                )));
            }
        }
        if ![
            "direct-safe-wrapper",
            "covered-by-higher-level-safe-api",
            "expert-raw-only",
            "internal-subsidiary",
            "blocked-by-abi",
            "blocked-by-native-state",
            "blocked-by-incomplete-contract",
            "candidate-implemented-in-this-milestone",
            "deferred-stateful-constrained-bspline",
            "deferred-incremental-bspline-preprocessing",
        ]
        .contains(&string(record, "final_disposition")?)
        {
            return Err(CorpusError::Verification(format!(
                "domain completion record has invalid final disposition for {}",
                string(record, "routine")?
            )));
        }
    }
    Ok(())
}

fn records(path: &Path) -> Result<Vec<Value>> {
    let document: Value = serde_json::from_slice(&fs::read(path)?)?;
    document["records"]
        .as_array()
        .cloned()
        .ok_or_else(|| CorpusError::Verification(format!("{} lacks records", path.display())))
}

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field]
        .as_str()
        .ok_or_else(|| CorpusError::Verification(format!("record lacks string field {field}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_milestone_routines_have_explicit_safe_dispositions() {
        for routine in CURRENT_MILESTONE_ROUTINES {
            assert_eq!(
                disposition(routine, "reviewed_public_driver", "direct-safe-wrapper"),
                "candidate-implemented-in-this-milestone"
            );
        }
    }

    #[test]
    fn output_is_deterministic() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let one = generate(first.path()).unwrap();
        let two = generate(second.path()).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        assert_eq!(one.routine_count, two.routine_count);
        for name in [
            "approx-roots-optimization-coverage.json",
            "approximation-complete-coverage.json",
            "complex-root-complete-coverage.json",
            "nonlinear-optimization-complete-coverage.json",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
