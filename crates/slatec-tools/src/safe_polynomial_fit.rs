//! Deterministic evidence for the checked polynomial-fitting safe facade.
//!
//! This source-hash-guarded inventory deliberately distinguishes polynomial
//! least-squares fitting from global polynomial interpolation and from the
//! stateful constrained B-spline fitting family.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const REQUIRED_ROUTINES: &[&str] = &["POLFIT", "DPOLFT", "PVALUE", "DP1VLU", "PCOEF", "DPCOEF"];

/// Summary of deterministic polynomial-fitting metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus snapshot identifier.
    pub snapshot_id: String,
    /// Stable content hash across every output.
    pub semantic_hash: String,
    /// Number of reviewed native entry points.
    pub routine_count: usize,
    /// Number of public safe operations.
    pub operation_count: usize,
}

/// Returns conservative native-state records for every public fitting method.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let selected: Value = serde_json::from_slice(&fs::read(repo_path(
        "generated/selected-corpus/selected-providers.json",
    ))?)?;
    let sources = sources(&selected)?;
    let closure = provider_closure()?;
    let object_closure = closure
        .iter()
        .map(|id| format!("{id}.o"))
        .collect::<Vec<_>>();
    let roots = [
        (
            "slatec::interpolation::approximation::PolynomialFit::fit",
            vec!["POLFIT", "DPOLFT"],
            "POLFIT",
            "checked unit-weight polynomial least-squares construction",
        ),
        (
            "slatec::interpolation::approximation::PolynomialFit::fit_weighted",
            vec!["POLFIT", "DPOLFT"],
            "POLFIT",
            "checked positive-weight polynomial least-squares construction",
        ),
        (
            "slatec::interpolation::approximation::PolynomialFit::evaluate",
            vec!["PVALUE", "DP1VLU"],
            "PVALUE",
            "checked orthogonal polynomial evaluation",
        ),
        (
            "slatec::interpolation::approximation::PolynomialFit::evaluate_with_derivatives",
            vec!["PVALUE", "DP1VLU"],
            "PVALUE",
            "checked orthogonal polynomial derivative evaluation",
        ),
        (
            "slatec::interpolation::approximation::PolynomialFit::evaluate_into",
            vec!["PVALUE", "DP1VLU"],
            "PVALUE",
            "checked batch orthogonal polynomial evaluation",
        ),
        (
            "slatec::interpolation::approximation::PolynomialFit::power_coefficients",
            vec!["PCOEF", "DPCOEF"],
            "PCOEF",
            "checked origin power-coefficient conversion",
        ),
        (
            "slatec::interpolation::approximation::PolynomialFit::power_coefficients_at",
            vec!["PCOEF", "DPCOEF"],
            "PCOEF",
            "checked finite-origin Taylor-coefficient conversion",
        ),
    ];
    roots
        .into_iter()
        .map(|(safe_function, native_entry_points, entry, operation)| {
            let source = sources.get(entry).ok_or_else(|| {
                CorpusError::Verification(format!("missing selected polynomial-fit {entry} source"))
            })?;
            Ok(json!({
                "safe_function":safe_function,
                "native_entry_points":native_entry_points,
                "feature":"approximation-polynomial-fitting",
                "effective_native_families":["approximation-core"],
                "entry_object":format!("{}.o", source.id),
                "object_closure":object_closure.clone(),
                "source_closure":closure.clone(),
                "saved_mutable_locals":[],
                "common_blocks":[],
                "xerror_state":["XERROR J4SAVE/XERSVE process-global state is reachable from reviewed legacy error paths; safe preflight does not reconfigure XERROR"],
                "fortran_io":[],
                "callback_state":["none"],
                "writable_symbols":["POLFIT/DPOLFT representation and all evaluation buffers are caller-private; XERROR remains process-global"],
                "source_object_unresolved":[],
                "external_undefined_symbols":["Fortran runtime and external/system provider contracts remain unqualified"],
                "feature_closure_mismatch":false,
                "current_class":"SerializedGlobal",
                "best_possible_class_from_slatec_source":"SerializedGlobal",
                "native_routine_reentrancy":"legacy XERROR and provider/runtime qualification require the process-global native lock",
                "rust_api_concurrency":format!("immutable owned polynomial fit; {operation}; every native entry is globally serialized"),
                "provider_runtime_thread_safety":"source-build, external, and system provider profiles remain serialized",
                "provider_unknowns":["external_or_system_Fortran_runtime_and_provider_contract_not_qualified"],
                "remaining_blockers":["process-global XERROR", "provider/runtime qualification"]
            }))
        })
        .collect()
}

/// Generates exact source, wrapper, provider, and disposition records.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-polynomial-fit-metadata requires --offline".to_owned(),
        ));
    }
    let selected: Value = serde_json::from_slice(&fs::read(
        selected_corpus_dir.join("selected-providers.json"),
    )?)?;
    let snapshot = selected["records"]
        .as_array()
        .and_then(|records| records.first())
        .and_then(|record| record["snapshot_id"].as_str())
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot id".to_owned()))?;
    let sources = sources(&selected)?;
    let source_records = REQUIRED_ROUTINES
        .iter()
        .map(|routine| {
            let source = sources.get(*routine).ok_or_else(|| {
                CorpusError::Verification(format!("selected corpus lacks polynomial-fit {routine}"))
            })?;
            Ok(json!([
                routine,
                source.path,
                source.hash,
                source.role,
                source.id,
            ]))
        })
        .collect::<Result<Vec<_>>>()?;
    let provider_closure = provider_closure()?;
    let inventory = json!([
        [
            "POLFIT",
            "f32",
            "weighted polynomial least-squares construction",
            "historically_user_callable_driver",
            "N,X,Y,W,MAXDEG,NDEG,EPS,R,IERR,A",
            "A=3*N+3*MAXDEG+3; R=N",
            "included"
        ],
        [
            "DPOLFT",
            "f64",
            "weighted polynomial least-squares construction",
            "historically_user_callable_driver",
            "N,X,Y,W,MAXDEG,NDEG,EPS,R,IERR,A",
            "A=3*N+3*MAXDEG+3; R=N",
            "included"
        ],
        [
            "PVALUE",
            "f32",
            "orthogonal fit value and derivatives",
            "historically_user_callable_driver",
            "L,NDER,X,YFIT,YP,A",
            "YP=max(1,NDER)",
            "included"
        ],
        [
            "DP1VLU",
            "f64",
            "orthogonal fit value and derivatives",
            "historically_user_callable_driver",
            "L,NDER,X,YFIT,YP,A",
            "YP=max(1,NDER)",
            "included"
        ],
        [
            "PCOEF",
            "f32",
            "origin power-coefficient conversion",
            "historically_user_callable_driver",
            "L,C,TC,A",
            "TC=L+1",
            "included"
        ],
        [
            "DPCOEF",
            "f64",
            "origin power-coefficient conversion",
            "historically_user_callable_driver",
            "L,C,TC,A",
            "TC=L+1",
            "included"
        ],
    ]);
    let wrappers = json!([
        [
            "slatec::interpolation::approximation::PolynomialFit::fit",
            "POLFIT/DPOLFT",
            "f32/f64",
            "construct immutable unit-weight least-squares polynomial fit",
            "A=3*N+3*MAXDEG+3; R=N",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::approximation::PolynomialFit::fit_weighted",
            "POLFIT/DPOLFT",
            "f32/f64",
            "construct immutable positive-weight least-squares polynomial fit",
            "A=3*N+3*MAXDEG+3; R=N",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::approximation::PolynomialFit::evaluate",
            "PVALUE/DP1VLU",
            "f32/f64",
            "evaluate selected orthogonal polynomial fit",
            "YP=1",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::approximation::PolynomialFit::evaluate_with_derivatives",
            "PVALUE/DP1VLU",
            "f32/f64",
            "evaluate selected fit and first derivatives",
            "YP=max(1,NDER)",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::approximation::PolynomialFit::evaluate_into",
            "PVALUE/DP1VLU",
            "f32/f64",
            "batch evaluate selected fit",
            "private YP=1 per query",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::approximation::PolynomialFit::power_coefficients",
            "PCOEF/DPCOEF",
            "f32/f64",
            "convert selected fit to ascending powers at origin",
            "TC=L+1",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::approximation::PolynomialFit::power_coefficients_at",
            "PCOEF/DPCOEF",
            "f32/f64",
            "convert selected fit to ascending Taylor powers at a finite requested origin",
            "TC=L+1",
            "SerializedGlobal",
            "reviewed"
        ],
    ]);
    let audit = json!([
        [
            "POLFIT/DPOLFT",
            "included",
            "source-backed stateless fit representation with finite input preflight, private A/R work storage, source-defined degree policies, and f32/f64 pairing"
        ],
        [
            "PVALUE/DP1VLU",
            "included",
            "source-backed evaluation and derivative pair; the retained single-precision identity is PVALUE, not P1VLU"
        ],
        [
            "PCOEF/DPCOEF",
            "included",
            "source-backed conversion to ascending powers about origin using a private representation copy"
        ],
        [
            "P1VLU",
            "not_retained_identity",
            "the selected corpus has no P1VLU program unit; POLFIT's single-precision evaluator is PVALUE and its f64 counterpart is DP1VLU"
        ],
        [
            "FC/DFC",
            "deferred_stateful_constrained_bspline",
            "source contract has constrained B-spline modes, persistent W/IW state, optional variance state, and caller-selected constraint layout; no single coherent owned high-level workflow is proven"
        ],
        [
            "EFC/DEFC",
            "deferred_incremental_bspline_preprocessing",
            "source contract accumulates data groups through MDEIN/MDEOUT and shares persistent state with the constrained fitting workflow; it is not a stateless polynomial fit"
        ],
        [
            "FCMN/DFCMN/EFCMN/DEFCMN",
            "internal_subsidiaries",
            "support routines for the deferred stateful constrained B-spline workflows"
        ],
    ]);
    let files = [
        (
            "polynomial-fit-routine-inventory.json",
            json!({"schema_id":"slatec.safe-polynomial-fit.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["routine","precision","operation","role","native_arguments","workspace","safe_status"],"records":inventory,"source_columns":["routine","source_path","source_sha256","catalogue_role","source_id"],"source_records":source_records}),
        ),
        (
            "polynomial-fit-wrapper-index.json",
            json!({"schema_id":"slatec.safe-polynomial-fit.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","precision","mathematical_model","workspace_formula","runtime_policy","review_state"],"records":wrappers,"counts":{"reviewed_native_routines":6,"public_safe_operations":7,"precision_pairs":3,"checked_public_types":6,"callbacks":0}}),
        ),
        (
            "polynomial-fit-source-closure.json",
            json!({"schema_id":"slatec.safe-polynomial-fit.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"provider_features":{"approximation-core":provider_closure},"reviewed_roots":REQUIRED_ROUTINES,"link_policy":"The approximation-core provider archive is selected by feature; the native linker retains only the referenced fit roots and their legitimate dependencies.","xerror_policy":"Safe preflight excludes documented invalid-input paths. Native calls remain globally serialized because legacy XERROR and provider/runtime state are reachable."}),
        ),
        (
            "polynomial-fit-candidate-audit.json",
            json!({"schema_id":"slatec.safe-polynomial-fit.candidate-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["candidate","disposition","reason"],"records":audit}),
        ),
        (
            "polynomial-fit-validation-contract.json",
            json!({"schema_id":"slatec.safe-polynomial-fit.validation-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["operation","safe_preconditions","native_status_policy","storage_policy"],"records":[["fit/fit_weighted","equal nonempty finite X/Y; positive finite explicit weights; MAXDEG<N or MAXDEG<N-1 for F test; finite positive RMS tolerance","IERR=1 complete; IERR=3 retains documented best max-degree fit; IERR=4 retains documented best F-test candidate; all other statuses reject","every native array is a private copy; A=3*N+3*MAXDEG+3 and R=N"],["evaluate/evaluate_into","finite query; derivative count fits INTEGER; output length equals query count","PVALUE/DP1VLU have no status output; source-reviewed representation and private A copy are used","private A copy and YP=max(1,NDER); batch holds the native lock once"],["power_coefficients/power_coefficients_at","selected degree fits INTEGER; requested Taylor origin is finite","PCOEF/DPCOEF have no status output; source-reviewed L+1 coefficient extent is used","private A copy and private TC=L+1"]]}),
        ),
    ];
    fs::create_dir_all(output_dir)?;
    let mut bytes = Vec::new();
    for (name, document) in files {
        let encoded = serde_json::to_vec(&document)?;
        fs::write(output_dir.join(name), &encoded)?;
        bytes.extend_from_slice(&encoded);
    }
    let summary = format!(
        "# Safe polynomial fitting\n\n- Snapshot: `{snapshot}`.\n- `approximation-polynomial-fitting` exposes an immutable f32/f64 weighted least-squares representation over `POLFIT`/`DPOLFT`, with source-defined all-degree, RMS-tolerance, and F-test selection.\n- `PVALUE`/`DP1VLU` evaluate values and derivatives; `PCOEF`/`DPCOEF` return ordinary origin coefficients or a Taylor expansion about any finite requested origin. The single-precision evaluator is `PVALUE`; `P1VLU` is not a retained SLATEC identity.\n- Fitting is intentionally distinct from the checked global interpolation representation: sample abscissas may be unordered or repeated. All native storage is private and every call holds the process-global runtime lock.\n- `FC`/`DFC` and `EFC`/`DEFC` remain explicitly deferred as stateful constrained B-spline workflows; no provider or safe feature is promised for them.\n"
    );
    fs::write(
        output_dir.join("polynomial-fit-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        routine_count: REQUIRED_ROUTINES.len(),
        operation_count: 7,
    })
}

#[derive(Clone)]
struct Source {
    id: String,
    path: String,
    hash: String,
    role: String,
}

fn sources(selected: &Value) -> Result<BTreeMap<String, Source>> {
    let mut result = BTreeMap::new();
    for record in selected["records"].as_array().ok_or_else(|| {
        CorpusError::Verification("selected corpus lacks provider records".to_owned())
    })? {
        let Some(name) = record["normalized_name"].as_str() else {
            continue;
        };
        if !REQUIRED_ROUTINES.contains(&name) {
            continue;
        }
        let required = |field: &str| {
            record[field].as_str().map(str::to_owned).ok_or_else(|| {
                CorpusError::Verification(format!("selected polynomial-fit {name} lacks {field}"))
            })
        };
        result.insert(
            name.to_owned(),
            Source {
                id: required("id")?,
                path: required("source_path")?,
                hash: required("raw_sha256")?,
                role: required("catalogue_classification")?,
            },
        );
    }
    Ok(result)
}

fn provider_closure() -> Result<Vec<String>> {
    let closure: Value = serde_json::from_slice(&fs::read(repo_path(
        "crates/slatec-src/metadata/family-source-closure.json",
    ))?)?;
    closure["families"]["approximation-core"]
        .as_array()
        .ok_or_else(|| {
            CorpusError::Verification("missing approximation-core provider closure".to_owned())
        })?
        .iter()
        .map(|value| {
            value.as_str().map(str::to_owned).ok_or_else(|| {
                CorpusError::Verification("non-string approximation-core source id".to_owned())
            })
        })
        .collect()
}

fn repo_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_deterministic_and_source_hash_guarded() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = repo_path("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        assert_eq!(one.routine_count, 6);
        assert_eq!(one.operation_count, 7);
        for name in [
            "polynomial-fit-routine-inventory.json",
            "polynomial-fit-wrapper-index.json",
            "polynomial-fit-source-closure.json",
            "polynomial-fit-candidate-audit.json",
            "polynomial-fit-validation-contract.json",
            "polynomial-fit-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
