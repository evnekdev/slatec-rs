//! Deterministic reconciliation of canonical raw coverage with the safe API.
//!
//! A callable raw declaration is not automatically a safe-wrapper candidate.
//! This report makes that distinction explicit for every canonical public raw
//! routine and records the one selected safe expansion as a bounded decision.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const SAFE_API: &str = "generated/safe-api";
const RAW_API: &str = "generated/raw-api";
const EXTERNAL_ECOSYSTEM_FAMILIES: &[&str] = &["Dense linear algebra", "Eigenvalue problems"];
const HIGHER_LEVEL_ROUTINES: &[&str] = &[
    "CHFDV", "CHFEV", "DCHFDV", "DCHFEV", "BSPDR", "BSPEV", "BSPVD", "BSPVN", "DBSPDR", "DBSPEV",
    "DBSPVD", "DBSPVN", "INTRV", "DINTRV", "RFFTB1", "RFFTF1", "RFFTI1", "QC25C", "DQC25C",
    "QC25S", "DQC25S", "QMOMO", "DQMOMO",
];
const SAFE_DESIGN_BLOCKED: &[&str] = &["GAUS8", "DGAUS8"];
const FUTURE_CANDIDATES: &[&str] = &[];

/// Summary of reconciliation generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// Number of canonical public raw routines reconciled.
    pub raw_count: usize,
    /// Number of direct safe coverage records.
    pub direct_safe_count: usize,
    /// Stable hash over the four emitted files.
    pub semantic_hash: String,
}

/// Generates canonical raw-to-safe coverage and the bounded next-expansion decision.
pub fn generate(output_dir: &Path) -> Result<ResultSummary> {
    let raw = records(&repo_path(RAW_API).join("public-api-coverage.json"))?;
    let status = records(&repo_path(RAW_API).join("routine-status.json"))?;
    let functions = records(&repo_path(SAFE_API).join("function-index.json"))?;
    let status_by_routine = status
        .into_iter()
        .map(|record| Ok((string(&record, "routine")?.to_owned(), record)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let safe_by_routine = safe_index(&functions)?;

    let mut coverage = raw
        .iter()
        .map(|raw_record| coverage_record(raw_record, &status_by_routine, &safe_by_routine))
        .collect::<Result<Vec<_>>>()?;
    coverage.sort_by(|left, right| {
        left["raw_routine"]
            .as_str()
            .cmp(&right["raw_routine"].as_str())
    });
    validate_coverage(&coverage, raw.len())?;
    let counts = coverage_counts(&coverage)?;
    let direct_safe_count = counts
        .get("direct-safe-wrapper")
        .copied()
        .unwrap_or_default();
    let selection = selection_document(&counts);

    let json_document = json!({
        "schema_id":"slatec.safe-api.raw-to-safe-coverage",
        "schema_version":"1.0.0",
        "coverage_policy":"A canonical raw path remains available even when no safe facade is appropriate. Dense, banded, packed, sparse, and eigenproblem APIs are intentionally not expanded as safe APIs where a mature Rust ecosystem owns the abstraction.",
        "record_count":coverage.len(),
        "counts":counts,
        "records":coverage,
    });
    let markdown = render_coverage_markdown(&coverage, &counts)?;
    let selection_markdown = render_selection_markdown(&selection);
    fs::create_dir_all(output_dir)?;
    let documents = [
        (
            "raw-to-safe-coverage.json",
            serde_json::to_vec(&json_document)?,
        ),
        ("raw-to-safe-coverage.md", markdown.into_bytes()),
        (
            "next-expansion-selection.json",
            serde_json::to_vec(&selection)?,
        ),
        (
            "next-expansion-selection.md",
            selection_markdown.into_bytes(),
        ),
    ];
    let mut bytes = Vec::new();
    for (name, document) in documents {
        fs::write(output_dir.join(name), &document)?;
        bytes.extend_from_slice(&document);
    }
    Ok(ResultSummary {
        raw_count: raw.len(),
        direct_safe_count,
        semantic_hash: hash::bytes(&bytes),
    })
}

#[derive(Clone, Default)]
struct SafeCoverage {
    paths: BTreeSet<String>,
    features: BTreeSet<String>,
    precisions: BTreeSet<String>,
}

fn safe_index(functions: &[Value]) -> Result<BTreeMap<String, SafeCoverage>> {
    let mut index = BTreeMap::<String, SafeCoverage>::new();
    for function in functions {
        let path = string(function, "rust_path")?.to_owned();
        let feature = string(function, "feature")?.to_owned();
        let precision = string(function, "precision")?.to_owned();
        for routine in string(function, "fortran_routine")?.split('/') {
            let record = index.entry(routine.to_owned()).or_default();
            record.paths.insert(path.clone());
            record.features.insert(feature.clone());
            record.precisions.insert(precision.clone());
        }
    }
    Ok(index)
}

fn coverage_record(
    raw: &Value,
    status: &BTreeMap<String, Value>,
    safe: &BTreeMap<String, SafeCoverage>,
) -> Result<Value> {
    let routine = string(raw, "routine")?;
    let canonical_path = string(raw, "canonical_rust_path")?;
    let raw_status = status.get(routine).ok_or_else(|| {
        CorpusError::Verification(format!("raw coverage lacks status for {routine}"))
    })?;
    let family = string(raw_status, "primary_family")?;
    let driver_role = string(raw_status, "driver_role")?;
    let raw_state = string(raw_status, "raw_api_state")?;
    let direct = direct_safe_coverage(routine, safe);
    let (coverage_kind, paths, features, abstraction, reason, disposition, priority) = if let Some(
        coverage,
    ) = direct
    {
        (
            "direct-safe-wrapper",
            coverage.paths,
            coverage.features,
            None,
            "A reviewed safe function index maps this exact raw routine or its reviewed f32/f64 precision pair.",
            "maintain_checked_safe_wrapper",
            "maintain",
        )
    } else if EXTERNAL_ECOSYSTEM_FAMILIES.contains(&family) {
        (
            "intentionally-excluded-external-ecosystem",
            BTreeSet::new(),
            BTreeSet::new(),
            None,
            "Safe dense, banded, packed, sparse, and eigenproblem APIs are intentionally left to established Rust numerical crates; the reviewed raw path remains available.",
            "retain_raw_only",
            "none",
        )
    } else if unsupported_raw_state(raw_state) {
        (
            "blocked-by-raw-abi",
            BTreeSet::new(),
            BTreeSet::new(),
            None,
            "The raw status record does not establish a safe ABI contract for promotion.",
            "retain_raw_only_until_abi_review",
            "blocked",
        )
    } else if FUTURE_CANDIDATES.contains(&routine) {
        (
            "candidate-for-safe-expansion",
            BTreeSet::new(),
            BTreeSet::new(),
            None,
            "The cubic B-spline constructor is a credible future safe candidate, but its boundary and knot policy is distinct from the current checked B-spline type.",
            "defer_to_dedicated_bspline_constructor_review",
            "later",
        )
    } else if SAFE_DESIGN_BLOCKED.contains(&routine) {
        (
            "blocked-by-safe-design",
            BTreeSet::new(),
            BTreeSet::new(),
            Some("existing QUADPACK options-based integration"),
            "A routine-named callback facade would duplicate the existing safe quadrature abstraction and callback containment policy.",
            "retain_raw_only",
            "none",
        )
    } else if higher_level_safe_coverage(routine, canonical_path) {
        (
            "covered-by-higher-level-safe-api",
            BTreeSet::new(),
            BTreeSet::new(),
            Some(higher_level_abstraction(routine, canonical_path)),
            "The raw primitive is retained for expert use, while its supported workflow is expressed through an owned checked safe abstraction.",
            "maintain_higher_level_abstraction",
            "maintain",
        )
    } else if driver_role != "historically_user_callable_driver" {
        (
            "internal-or-subsidiary",
            BTreeSet::new(),
            BTreeSet::new(),
            None,
            "The authoritative raw status records this identity as a non-driver support unit.",
            "retain_raw_only",
            "none",
        )
    } else {
        (
            "expert-raw-only",
            BTreeSet::new(),
            BTreeSet::new(),
            None,
            "No coherent checked safe abstraction has been selected; preserving the reviewed raw path avoids inventing a routine-shaped facade without a full safe-design audit.",
            "retain_raw_only_pending_family_review",
            "none",
        )
    };
    Ok(json!({
        "raw_routine":routine,
        "raw_canonical_path":canonical_path,
        "mathematical_family":family,
        "existing_safe_public_path":join(&paths),
        "coverage_kind":coverage_kind,
        "safe_feature":join(&features),
        "higher_level_abstraction":abstraction,
        "reason_no_direct_wrapper":reason,
        "recommended_disposition":disposition,
        "priority":priority,
        "raw_api_state":raw_state,
        "driver_role":driver_role,
    }))
}

fn direct_safe_coverage(
    routine: &str,
    safe: &BTreeMap<String, SafeCoverage>,
) -> Option<SafeCoverage> {
    if let Some(found) = safe.get(routine) {
        return Some(found.clone());
    }
    // Safe inventories record precision-generic methods once. Where a method
    // is explicitly documented as f32/f64 and the native double routine is
    // conventionally the `D`-prefixed companion, its one safe path covers both
    // reviewed forms without duplicating the Rust method in the index.
    let companion = routine.strip_prefix('D')?;
    let found = safe.get(companion)?;
    let paired = found.precisions.contains("f32/f64");
    paired.then(|| found.clone())
}

fn higher_level_safe_coverage(routine: &str, path: &str) -> bool {
    HIGHER_LEVEL_ROUTINES.contains(&routine)
        || path.contains("::quadrature::callbacks::dq")
        || path.contains("::quadrature::callbacks::q")
        || path.contains("::fftpack::rfft")
}

fn higher_level_abstraction(routine: &str, path: &str) -> &'static str {
    if path.contains("::quadrature::") {
        "slatec::quadrature options-based adaptive integration"
    } else if path.contains("::fftpack::") {
        "slatec::fftpack owned transform plans"
    } else if routine.starts_with("CH") || routine.starts_with("DCH") || routine.contains("PCH") {
        "slatec::pchip::PiecewiseCubicHermite"
    } else {
        "slatec::interpolation::bspline::BSpline"
    }
}

fn unsupported_raw_state(state: &str) -> bool {
    state.starts_with("unsupported_") || state == "ambiguous_symbol" || state == "missing_symbol"
}

fn coverage_counts(records: &[Value]) -> Result<BTreeMap<String, usize>> {
    let mut counts = BTreeMap::new();
    for record in records {
        *counts
            .entry(string(record, "coverage_kind")?.to_owned())
            .or_default() += 1;
    }
    Ok(counts)
}

fn validate_coverage(records: &[Value], expected_count: usize) -> Result<()> {
    if records.len() != expected_count {
        return Err(CorpusError::Verification(
            "raw-to-safe reconciliation does not cover every public raw routine".to_owned(),
        ));
    }
    let mut seen = BTreeSet::new();
    for record in records {
        let routine = string(record, "raw_routine")?;
        if !seen.insert(routine) {
            return Err(CorpusError::Verification(format!(
                "raw-to-safe reconciliation duplicates {routine}"
            )));
        }
        let kind = string(record, "coverage_kind")?;
        if ![
            "direct-safe-wrapper",
            "covered-by-higher-level-safe-api",
            "expert-raw-only",
            "internal-or-subsidiary",
            "blocked-by-raw-abi",
            "blocked-by-safe-design",
            "intentionally-excluded-external-ecosystem",
            "candidate-for-safe-expansion",
        ]
        .contains(&kind)
        {
            return Err(CorpusError::Verification(format!(
                "raw-to-safe reconciliation has unknown coverage kind {kind}"
            )));
        }
        if string(record, "reason_no_direct_wrapper")?.is_empty()
            || string(record, "recommended_disposition")?.is_empty()
        {
            return Err(CorpusError::Verification(format!(
                "raw-to-safe reconciliation lacks a disposition for {routine}"
            )));
        }
    }
    Ok(())
}

fn selection_document(counts: &BTreeMap<String, usize>) -> Value {
    json!({
        "schema_id":"slatec.safe-api.next-expansion-selection",
        "schema_version":"1.0.0",
        "selection_criteria":["existing checked interpolation and quadrature module structure","reviewed ABI and provider feature closures","no callback or retained pointer contract","shared owned sample type","focused f32/f64 runtime tests practical"],
        "selected_batch":{
            "name":"checked tabulated data and global polynomial operations",
            "reviewed_raw_routines":["AVINT","DAVINT","POLINT","DPLINT","POLYVL","DPOLVL","POLCOF","DPOLCF"],
            "public_operations":["TabulatedData::interpolating_polynomial","InterpolatingPolynomial::evaluate","InterpolatingPolynomial::evaluate_with_derivatives","InterpolatingPolynomial::taylor_coefficients_at","quadrature::integrate_tabulated","quadrature::integrate_tabulated_f32"],
            "shared_infrastructure":["owned Vec storage","Fortran INTEGER conversion","process-global native serialization","safe preflight of every reviewed legacy-error contract"],
            "reason":"A single checked data type supports two distinctive SLATEC workflows—global polynomial interpolation and arbitrary-spacing tabulated integration—without duplicating existing callback APIs or exposing workspace arrays."
        },
        "rejected_candidates":[
            {"candidate":"BINT4/DBINT4","reason":"promoted to the typed BSpline::interpolate_cubic workflow; its source boundary and knot policies are no longer an unclassified candidate","disposition":"implemented in current milestone"},
            {"candidate":"BSPEV/DBSPEV and associated spline primitives","reason":"already represented by owned B-spline and piecewise-polynomial safe methods; raw-array duplication would regress storage safety","disposition":"covered by higher-level API"},
            {"candidate":"GAUS8/DGAUS8","reason":"callback-bearing routine would duplicate established QUADPACK options-based integration and callback containment","disposition":"raw only"},
            {"candidate":"SDRIV3/DDRIV3 events and analytic Jacobians","reason":"requires a dedicated continuation, callback, and event-lifecycle audit","disposition":"deferred"},
            {"candidate":"DASSL Jacobian or mass-matrix callbacks","reason":"matrix layout and callback aliasing remain outside the residual-only session contract","disposition":"deferred"},
            {"candidate":"dense, banded, packed, sparse, and eigenproblem algorithms","reason":"safe facades are intentionally excluded in favour of mature Rust numerical ecosystem crates","disposition":"external ecosystem"}
        ],
        "coverage_counts_after_selection":counts,
    })
}

fn render_coverage_markdown(records: &[Value], counts: &BTreeMap<String, usize>) -> Result<String> {
    let mut markdown = String::from(
        "# Raw-to-safe coverage reconciliation\n\nEvery canonical public raw routine has exactly one safe-coverage disposition. A raw path remains public independently of whether a safe wrapper is appropriate. Dense, banded, packed, sparse, and eigenproblem safe APIs are intentionally left to the established Rust numerical ecosystem; this policy does not remove the reviewed `slatec-sys` path.\n\n## Counts\n\n",
    );
    for (kind, count) in counts {
        markdown.push_str(&format!("- `{kind}`: {count}\n"));
    }
    markdown.push_str("\n## Canonical raw routine records\n\n| Raw routine | Canonical raw path | Family | Safe path | Coverage | Feature | Higher abstraction | Disposition | Priority |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for record in records {
        markdown.push_str(&format!(
            "| `{}` | `{}` | {} | {} | `{}` | {} | {} | `{}` | `{}` |\n",
            string(record, "raw_routine")?,
            string(record, "raw_canonical_path")?,
            string(record, "mathematical_family")?,
            markdown_value(record, "existing_safe_public_path"),
            string(record, "coverage_kind")?,
            markdown_value(record, "safe_feature"),
            markdown_value(record, "higher_level_abstraction"),
            string(record, "recommended_disposition")?,
            string(record, "priority")?,
        ));
    }
    Ok(markdown)
}

fn render_selection_markdown(selection: &Value) -> String {
    let selected = &selection["selected_batch"];
    let mut markdown = String::from("# Next safe expansion selection\n\n");
    markdown.push_str(&format!(
        "Selected batch: **{}**. {}\n\n",
        selected["name"].as_str().unwrap_or("unknown"),
        selected["reason"].as_str().unwrap_or(""),
    ));
    markdown.push_str("## Rejected alternatives\n\n");
    for candidate in selection["rejected_candidates"]
        .as_array()
        .into_iter()
        .flatten()
    {
        markdown.push_str(&format!(
            "- `{}`: {} ({})\n",
            candidate["candidate"].as_str().unwrap_or("unknown"),
            candidate["reason"].as_str().unwrap_or(""),
            candidate["disposition"].as_str().unwrap_or(""),
        ));
    }
    markdown
}

fn markdown_value(record: &Value, field: &str) -> String {
    record[field]
        .as_str()
        .filter(|value| !value.is_empty())
        .unwrap_or("—")
        .to_owned()
}

fn join(values: &BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join("; ")
}

fn records(path: &Path) -> Result<Vec<Value>> {
    let value: Value = serde_json::from_slice(&fs::read(path)?)?;
    value["records"]
        .as_array()
        .cloned()
        .ok_or_else(|| CorpusError::Verification(format!("{} lacks records", path.display())))
}

fn string<'a>(record: &'a Value, field: &str) -> Result<&'a str> {
    record[field].as_str().ok_or_else(|| {
        CorpusError::Verification(format!("coverage record lacks string field {field}"))
    })
}

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconciliation_is_deterministic_and_covers_every_public_raw_identity() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let one = generate(first.path()).unwrap();
        let two = generate(second.path()).unwrap();
        assert_eq!(one.raw_count, 821);
        assert_eq!(one.semantic_hash, two.semantic_hash);
        for name in [
            "raw-to-safe-coverage.json",
            "raw-to-safe-coverage.md",
            "next-expansion-selection.json",
            "next-expansion-selection.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
