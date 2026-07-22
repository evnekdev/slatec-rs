//! Deterministic metadata for the checked tabulated-data safe facade.
//!
//! This module keeps the narrow source and ABI review for `AVINT`/`DAVINT`
//! and the `POLINT` representation family separate from the broad raw API
//! inventory.  It records the exact selected source hashes and the provider
//! feature closures used by the safe layer; generated output is evidence, not
//! an additional hand-maintained registry.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const REQUIRED_ROUTINES: &[&str] = &[
    "AVINT", "DAVINT", "POLINT", "DPLINT", "POLYVL", "DPOLVL", "POLCOF", "DPOLCF",
];

/// Summary returned after deterministic tabulated-data metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus snapshot identifier.
    pub snapshot_id: String,
    /// Stable hash over every emitted file.
    pub semantic_hash: String,
    /// Count of reviewed native entry points.
    pub routine_count: usize,
    /// Count of public safe operations, excluding precision pairing.
    pub operation_count: usize,
}

/// Returns a conservative native-state projection for every public operation.
///
/// `interpolation-general` and `quadrature-direct` deliberately select
/// source-family archives rather than a Cargo feature per routine.  The
/// projection preserves that provider fact, while the linker still retains
/// only objects reached from the referenced entry points and their native
/// dependencies.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure = provider_closure()?;
    let object_closure = closure
        .values()
        .flat_map(|ids| ids.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|id| format!("{id}.o"))
        .collect::<Vec<_>>();
    let source_closure = closure
        .values()
        .flat_map(|ids| ids.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let selected = selected_sources()?;
    let source_id = |routine: &str| -> Result<String> {
        selected
            .get(routine)
            .map(|record| record.id.clone())
            .ok_or_else(|| CorpusError::Verification(format!("missing selected {routine} source")))
    };
    [
        (
            "slatec::interpolation::tabulated::TabulatedData::interpolating_polynomial",
            vec!["POLINT", "DPLINT"],
            "POLINT",
            "checked global Newton polynomial construction",
        ),
        (
            "slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate",
            vec!["POLYVL", "DPOLVL"],
            "POLYVL",
            "checked global Newton polynomial evaluation",
        ),
        (
            "slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate_with_derivatives",
            vec!["POLYVL", "DPOLVL"],
            "POLYVL",
            "checked global Newton polynomial derivative evaluation",
        ),
        (
            "slatec::interpolation::tabulated::InterpolatingPolynomial::taylor_coefficients_at",
            vec!["POLCOF", "DPOLCF"],
            "POLCOF",
            "checked Newton-to-Taylor coefficient conversion",
        ),
        (
            "slatec::quadrature::integrate_tabulated",
            vec!["DAVINT"],
            "DAVINT",
            "checked overlapping-parabola tabulated integration",
        ),
        (
            "slatec::quadrature::integrate_tabulated_f32",
            vec!["AVINT"],
            "AVINT",
            "checked overlapping-parabola tabulated integration",
        ),
    ]
    .into_iter()
    .map(|(safe_function, routines, entry, operation)| {
        Ok(json!({
            "safe_function":safe_function,
            "native_entry_points":routines,
            "feature":"tabulated-data",
            "effective_native_families":["interpolation-general", "quadrature-direct"],
            "entry_object":format!("{}.o", source_id(entry)?),
            "object_closure":object_closure,
            "source_closure":source_closure,
            "saved_mutable_locals":[],
            "common_blocks":[],
            "xerror_state":["XERROR J4SAVE/XERSVE process-global state is reachable from reviewed legacy error paths; safe preflight does not reconfigure XERROR"],
            "fortran_io":[],
            "callback_state":["none"],
            "writable_symbols":["checked tabulated-data numerical paths have no reviewed mutable numerical state; XERROR remains process-global"],
            "source_object_unresolved":[],
            "external_undefined_symbols":["Fortran runtime and external/system provider contracts remain unqualified"],
            "feature_closure_mismatch":false,
            "current_class":"SerializedGlobal",
            "best_possible_class_from_slatec_source":"SerializedGlobal",
            "native_routine_reentrancy":"XERROR and provider/runtime qualification require the process-global native lock",
            "rust_api_concurrency":format!("owned samples and polynomial state; {operation}; every native entry is globally serialized"),
            "provider_runtime_thread_safety":"source-build, external, and system provider profiles remain serialized",
            "provider_unknowns":["external_or_system_Fortran_runtime_and_provider_contract_not_qualified"],
            "remaining_blockers":["process-global XERROR", "provider/runtime qualification"]
        }))
    })
    .collect()
}

/// Generates tabulated-data source, wrapper, selection, and validation records.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-tabulated-data-metadata requires --offline".to_owned(),
        ));
    }
    let manifest: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = manifest["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let sources = selected_sources_from(selected_corpus_dir)?;
    let source_records = REQUIRED_ROUTINES
        .iter()
        .map(|routine| {
            let source = sources.get(*routine).ok_or_else(|| {
                CorpusError::Verification(format!("selected corpus lacks tabulated {routine}"))
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
    let provider = provider_closure()?;
    let provider_features = provider
        .iter()
        .map(|(feature, ids)| json!([feature, ids]))
        .collect::<Vec<_>>();
    let inventory = json!([
        [
            "AVINT",
            "f32",
            "overlapping-parabola tabulated integration",
            "subroutine",
            "X,Y,N,XLO,XUP,ANS,IERR",
            "none",
            "included"
        ],
        [
            "DAVINT",
            "f64",
            "overlapping-parabola tabulated integration",
            "subroutine",
            "X,Y,N,XLO,XUP,ANS,IERR",
            "none",
            "included"
        ],
        [
            "POLINT",
            "f32",
            "Newton polynomial construction",
            "subroutine",
            "N,X,Y,C",
            "C=N",
            "included"
        ],
        [
            "DPLINT",
            "f64",
            "Newton polynomial construction",
            "subroutine",
            "N,X,Y,C",
            "C=N",
            "included"
        ],
        [
            "POLYVL",
            "f32",
            "Newton polynomial value and derivative evaluation",
            "subroutine",
            "NDER,XX,YFIT,YP,N,X,C,WORK,IERR",
            "WORK=2*N when NDER>0",
            "included"
        ],
        [
            "DPOLVL",
            "f64",
            "Newton polynomial value and derivative evaluation",
            "subroutine",
            "NDER,XX,YFIT,YP,N,X,C,WORK,IERR",
            "WORK=2*N when NDER>0",
            "included"
        ],
        [
            "POLCOF",
            "f32",
            "Newton polynomial Taylor coefficients",
            "subroutine",
            "XX,N,X,C,D,WORK",
            "D=N; WORK=2*N",
            "included"
        ],
        [
            "DPOLCF",
            "f64",
            "Newton polynomial Taylor coefficients",
            "subroutine",
            "XX,N,X,C,D,WORK",
            "D=N; WORK=2*N",
            "included"
        ]
    ]);
    let wrappers = json!([
        [
            "slatec::interpolation::tabulated::TabulatedData::interpolating_polynomial",
            "POLINT/DPLINT",
            "f32/f64",
            "construct global Newton interpolation polynomial",
            "C=N",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate",
            "POLYVL/DPOLVL",
            "f32/f64",
            "evaluate global Newton interpolation polynomial",
            "none when NDER=0",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::tabulated::InterpolatingPolynomial::evaluate_with_derivatives",
            "POLYVL/DPOLVL",
            "f32/f64",
            "evaluate global Newton interpolation polynomial and first derivatives",
            "YP=NDER; WORK=2*N when NDER>0",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::tabulated::InterpolatingPolynomial::taylor_coefficients_at",
            "POLCOF/DPOLCF",
            "f32/f64",
            "derive Taylor coefficients about a finite centre",
            "D=N; WORK=2*N",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::quadrature::integrate_tabulated",
            "DAVINT",
            "f64",
            "integrate arbitrarily spaced tabulated values",
            "none",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::quadrature::integrate_tabulated_f32",
            "AVINT",
            "f32",
            "integrate arbitrarily spaced tabulated values",
            "none",
            "SerializedGlobal",
            "reviewed"
        ]
    ]);
    let files = [
        (
            "tabulated-data-routine-inventory.json",
            json!({"schema_id":"slatec.safe-tabulated-data.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["routine","precision","operation","role","native_arguments","workspace","safe_status"],"records":inventory,"source_columns":["routine","source_path","source_sha256","catalogue_role","source_id"],"source_records":source_records}),
        ),
        (
            "tabulated-data-wrapper-index.json",
            json!({"schema_id":"slatec.safe-tabulated-data.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","precision","mathematical_operation","workspace_formula","runtime_policy","review_state"],"records":wrappers,"counts":{"reviewed_native_routines":8,"public_safe_operations":6,"precision_pairs":4,"checked_public_types":3,"callbacks":0}}),
        ),
        (
            "tabulated-data-candidate-classification.json",
            json!({"schema_id":"slatec.safe-tabulated-data.candidate-classification","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["candidate","status","reason"],"records":[["AVINT/DAVINT and POLINT/DPLINT/POLYVL/DPOLVL/POLCOF/DPOLCF","included","one owned finite strictly-increasing sample type covers construction, value/derivative evaluation, Taylor coefficients, and arbitrary-spacing integration without callbacks"],["BINT4/DBINT4","deferred","cubic B-spline construction has boundary and knot policy distinct from the existing checked B-spline representation"],["BSPEV/DBSPEV and related spline subsidiaries","covered_by_higher_level_safe_api","existing checked B-spline methods own knot, coefficient, and derivative contracts; a raw-array duplicate would weaken the API"],["GAUS8/DGAUS8","deferred","callback-bearing adaptive integration duplicates established QUADPACK safe options and needs no second callback design"],["SDRIV3/DDRIV3 expert events or Jacobians","deferred","ODE callback and continuation lifecycle review is a separate bounded task"],["DASSL Jacobian and mass-matrix callbacks","deferred","residual-only session intentionally avoids unreviewed callback aliasing and matrix-layout contracts"]]}),
        ),
        (
            "tabulated-data-source-closure.json",
            json!({"schema_id":"slatec.safe-tabulated-data.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"provider_features":provider_features,"reviewed_roots":REQUIRED_ROUTINES,"link_policy":"Cargo provider features compile the recorded family archive closure; the native linker retains only referenced roots and their legitimate dependencies.","xerror_policy":"valid-input preflight avoids deliberate legacy fatal paths; every native call remains serialized because XERROR state is reachable."}),
        ),
        (
            "tabulated-data-validation-contract.json",
            json!({"schema_id":"slatec.safe-tabulated-data.validation-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["operation","safe_preconditions","native_status_policy","storage_policy"],"records":[["TabulatedData::from_samples","equal lengths; at least two finite samples; abscissas strictly increasing; count fits Fortran INTEGER","no native call","owned vectors retained exactly and never sorted or deduplicated"],["interpolating_polynomial","validated samples","POLINT/DPLINT receives a private coefficient destination; invalid native duplicate-abscissa path is preflighted","native representation owns X and C copies"],["evaluate_with_derivatives","finite query; requested derivative count fits Fortran INTEGER","POLYVL/DPOLVL IERR must be 1","private YP and 2*N work storage when derivatives are requested"],["taylor_coefficients_at","finite expansion centre","POLCOF/DPOLCF has no status code; source-reviewed preconditions are retained","private D=N and WORK=2*N buffers"],["integrate_tabulated","finite ordered in-domain bounds; N=2 trapezoid special case or at least three in-interval samples","AVINT/DAVINT IERR must be 1; native invalid statuses are not deliberately invoked","read-only sample vectors and stack scalar outputs"]]}),
        ),
    ];
    fs::create_dir_all(output_dir)?;
    let mut bytes = Vec::new();
    for (name, value) in files {
        let encoded = serde_json::to_vec(&value)?;
        fs::write(output_dir.join(name), &encoded)?;
        bytes.extend_from_slice(&encoded);
    }
    let summary = format!(
        "# Safe tabulated-data operations\n\n- Snapshot: `{snapshot}`.\n- The `tabulated-data` feature provides one checked, owned sample representation for finite strictly increasing real abscissas and matching finite values. It exposes eight reviewed SLATEC roots through six public safe operations: Newton interpolation construction, value evaluation, derivative evaluation, Taylor coefficients, and f32/f64 arbitrary-spacing integration.\n- `POLINT`/`DPLINT` store the native Newton representation privately. `POLYVL`/`DPOLVL` and `POLCOF`/`DPOLCF` operate only on owned buffers; `AVINT`/`DAVINT` reads checked samples.\n- No callback, raw pointer, caller workspace, sorting, extrapolation, or retained native pointer is exposed. Every native call is serialized through the existing process-global runtime lock because XERROR and provider/runtime state remain reachable.\n- `BINT4`/`DBINT4`, raw B-spline subsidiaries, `GAUS8`/`DGAUS8`, and ODE/DAE callback expansions were considered and deliberately deferred for the recorded, different-contract reasons.\n"
    );
    fs::write(
        output_dir.join("tabulated-data-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        routine_count: REQUIRED_ROUTINES.len(),
        operation_count: 6,
    })
}

#[derive(Clone)]
struct SelectedSource {
    id: String,
    path: String,
    hash: String,
    role: String,
}

fn selected_sources() -> Result<BTreeMap<String, SelectedSource>> {
    selected_sources_from(&repo_path("generated/selected-corpus"))
}

fn selected_sources_from(selected_corpus_dir: &Path) -> Result<BTreeMap<String, SelectedSource>> {
    let selected: Value = serde_json::from_slice(&fs::read(
        selected_corpus_dir.join("selected-providers.json"),
    )?)?;
    let mut sources = BTreeMap::new();
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
                CorpusError::Verification(format!("selected tabulated {name} lacks {field}"))
            })
        };
        sources.insert(
            name.to_owned(),
            SelectedSource {
                id: required("id")?,
                path: required("source_path")?,
                hash: required("raw_sha256")?,
                role: required("catalogue_classification")?,
            },
        );
    }
    Ok(sources)
}

fn provider_closure() -> Result<BTreeMap<String, Vec<String>>> {
    let closure: Value = serde_json::from_slice(&fs::read(repo_path(
        "crates/slatec-src/metadata/family-source-closure.json",
    ))?)?;
    ["interpolation-general", "quadrature-direct"]
        .into_iter()
        .map(|feature| {
            let ids = closure["families"][feature]
                .as_array()
                .ok_or_else(|| {
                    CorpusError::Verification(format!("missing provider feature closure {feature}"))
                })?
                .iter()
                .map(|id| {
                    id.as_str().map(str::to_owned).ok_or_else(|| {
                        CorpusError::Verification(format!(
                            "provider feature {feature} has non-string id"
                        ))
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            Ok((feature.to_owned(), ids))
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
    fn output_is_deterministic_and_hash_guards_all_reviewed_roots() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = repo_path("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        assert_eq!(one.routine_count, 8);
        assert_eq!(one.operation_count, 6);
        for name in [
            "tabulated-data-routine-inventory.json",
            "tabulated-data-wrapper-index.json",
            "tabulated-data-candidate-classification.json",
            "tabulated-data-source-closure.json",
            "tabulated-data-validation-contract.json",
            "tabulated-data-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
