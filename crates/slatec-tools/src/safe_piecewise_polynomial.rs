//! Deterministic metadata for the reviewed SLATEC PP representation facade.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Returns focused native-state projections for the PP safe methods.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure = closure()?;
    let objects = closure["source_ids"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("PP closure lacks source ids".to_owned()))?
        .iter()
        .filter_map(Value::as_str)
        .map(|id| format!("{id}.o"))
        .chain(
            ["profile-i1mach.o", "profile-r1mach.o", "profile-d1mach.o"]
                .into_iter()
                .map(str::to_owned),
        )
        .collect::<Vec<_>>();
    Ok([
        ("slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate", "PPVAL/DPPVAL"),
        ("slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative", "PPVAL/DPPVAL"),
        ("slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into", "PPVAL/DPPVAL"),
        ("slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::integrate", "PPQAD/DPPQAD"),
        ("slatec::interpolation::bspline::BSpline::to_piecewise_polynomial", "BSPPP/DBSPPP"),
    ]
    .into_iter()
    .map(|(safe_function, routine)| {
        json!({
            "safe_function":safe_function,
            "native_entry_points":routine.split('/').collect::<Vec<_>>(),
            "feature":"piecewise-polynomial",
            "effective_native_families":["piecewise-polynomial"],
            "entry_object": if routine.starts_with("PPVAL") { "selected-source-aee9eab352c005c2.o" } else if routine.starts_with("PPQAD") { "selected-source-eb3fbb61e4e340e4.o" } else { "selected-source-95899b1bd72786ac.o" },
            "object_closure":objects,
            "source_closure":["PPVAL/DPPVAL","PPQAD/DPPQAD","BSPPP/DBSPPP","INTRV/DINTRV","BSPDR/DBSPDR","BSPEV/DBSPEV","BSPVN/DBSPVN","XERROR closure"],
            "saved_mutable_locals":Vec::<&str>::new(),
            "common_blocks":[],
            "xerror_state":["XERROR J4SAVE/XERSVE process-global state is reachable on contradicted native contracts"],
            "fortran_io":[],
            "callback_state":["none"],
            "writable_symbols":["focused closure audit: XERROR storage; PP/BSP numeric path has no mutable COMMON, SAVE, or DATA state"],
            "source_object_unresolved":[],
            "external_undefined_symbols":[],
            "feature_closure_mismatch":false,
            "current_class":"SerializedGlobal",
            "best_possible_class_from_slatec_source":"SerializedGlobal",
            "native_routine_reentrancy":"SerializedGlobal",
            "rust_api_concurrency":"owned PP values are movable; every native entry is globally serialized",
            "provider_runtime_thread_safety":"reviewed source and external/system profiles remain serialized",
            "provider_unknowns":["external_or_system_Fortran_runtime_and_provider_contract_not_qualified"],
            "remaining_blockers":["process-global XERROR","provider/runtime qualification"]
        })
    })
    .collect())
}

/// Concise result from PP metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// The selected-corpus snapshot identifier.
    pub snapshot_id: String,
    /// Stable hash over every emitted file.
    pub semantic_hash: String,
    /// Count of reviewed native PP roots.
    pub routine_count: usize,
}

/// Generates the PP audit and safe-wrapper metadata from explicit contracts.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-piecewise-polynomial-metadata requires --offline".to_owned(),
        ));
    }
    let manifest: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = manifest["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let closure = closure()?;
    let sources = source_records(selected_corpus_dir)?;
    let inventory = json!([
        [
            "PPVAL",
            "f32",
            "evaluation_or_derivative",
            "function",
            "LDC,C,XI,LXI,K,IDERIV,X,INPPV",
            "none",
            "included"
        ],
        [
            "DPPVAL",
            "f64",
            "evaluation_or_derivative",
            "function",
            "LDC,C,XI,LXI,K,IDERIV,X,INPPV",
            "none",
            "included"
        ],
        [
            "PPQAD",
            "f32",
            "definite_integration",
            "subroutine",
            "LDC,C,XI,LXI,K,X1,X2,PQUAD",
            "none",
            "included"
        ],
        [
            "DPPQAD",
            "f64",
            "definite_integration",
            "subroutine",
            "LDC,C,XI,LXI,K,X1,X2,PQUAD",
            "none",
            "included"
        ],
        [
            "BSPPP",
            "f32",
            "B_spline_to_PP_conversion",
            "subroutine",
            "T,A,N,K,LDC,C,XI,LXI,WORK",
            "WORK=K*(N+3); C=K*(N-K+1); XI=N-K+2",
            "included"
        ],
        [
            "DBSPPP",
            "f64",
            "B_spline_to_PP_conversion",
            "subroutine",
            "T,A,N,K,LDC,C,XI,LXI,WORK",
            "WORK=K*(N+3); C=K*(N-K+1); XI=N-K+2",
            "included"
        ],
        [
            "PCHIM/DPCHIM",
            "f32/f64",
            "Hermite_derivative_construction",
            "subroutine",
            "PCHIP knots, values, derivatives",
            "PCHIP-owned arrays",
            "deferred_conversion"
        ],
        [
            "PP_to_B_spline",
            "none",
            "inverse_representation_conversion",
            "none",
            "not a reviewed selected root",
            "not applicable",
            "deferred"
        ]
    ]);
    let wrappers = json!([
        [
            "slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate",
            "PPVAL",
            "f32/f64",
            "PP Taylor evaluation",
            "none",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative",
            "PPVAL",
            "f32/f64",
            "PP Taylor derivative evaluation",
            "none",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into",
            "PPVAL",
            "f32/f64",
            "PP batch evaluation",
            "none per scalar call",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::integrate",
            "PPQAD",
            "f32/f64",
            "exact PP definite integration",
            "none",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::bspline::BSpline::to_piecewise_polynomial",
            "BSPPP",
            "f32/f64",
            "exact B-spline to PP conversion",
            "K*(N+3), K*(N-K+1), N-K+2",
            "SerializedGlobal",
            "reviewed"
        ]
    ]);
    let files = [
        (
            "piecewise-polynomial-routine-inventory.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["routine","precision","operation","role","native_arguments","workspace","safe_status"],"records":inventory,"source_columns":["routine","source_path","source_sha256","catalogue_role"],"source_records":sources}),
        ),
        (
            "piecewise-polynomial-candidate-classification.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.candidate-classification","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["candidate","status","reason"],"records":[["PPVAL/DPPVAL","included","real scalar PP evaluation and derivative contract is complete"],["PPQAD/DPPQAD","included","real exact definite integration contract is complete"],["BSPPP/DBSPPP","included","exact B-spline-to-PP conversion has reviewed capacities and output layout"],["PCHIP to PP","deferred","PCHIP owns Hermite data, not the reviewed PP array layout; computing coefficients in Rust would translate a numerical representation conversion"],["PP to B-spline","deferred","no audited selected inverse conversion root or safe construction policy"],["PP forms with vector, multidimensional, fitting, or arbitrary-stride storage","deferred","outside the bounded owned univariate representation"]]}),
        ),
        (
            "piecewise-polynomial-storage-contract.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.storage-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["operation","native_storage","safe_storage","policy"],"records":[["representation","XI length LXI+1; C LDC by LXI; LDC>=K","owned flat Vec with order * piece_count coefficients","exact PP layout; no sorting, merging, or coefficient reshaping"],["evaluation","INPPV local interval state","local INPPV=1 per call","callers cannot retain stale native search state"],["integration","PPQAD scalar result","local scalar","finite in-domain bounds preflighted"],["BSPPP conversion","C capacity K*(N-K+1); XI N-K+2; WORK K*(N+3)","fallible private Vec allocations","native output is validated before becoming public"]]}),
        ),
        (
            "piecewise-polynomial-domain-contract.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.domain-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["property","native_rule","safe_policy"],"records":[["breakpoints","LXI>=1 and XI is strictly increasing","at least two finite strictly increasing breakpoints"],["coefficients","C(I,J) is the (I-1)st right derivative at XI(J)","flat columns preserve exact right-Taylor derivative convention"],["breakpoint_side","right interval at interior XI; final interval at final XI","preserved through PPVAL/DPPVAL"],["evaluation_domain","native permits extrapolation","default Error; explicit Clamp clamps coordinates in Rust and never enters native extrapolation"],["integration_domain","native permits endpoint extrapolation","default Error; explicit Clamp clamps both endpoints; reversed limits retain sign"],["derivative","0<=IDERIV<K","typed usize preflighted before native conversion"]]}),
        ),
        (
            "piecewise-polynomial-source-closure.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"manifest":"crates/slatec-src/metadata/piecewise-polynomial-source-closure.json","roots":["PPVAL","DPPVAL","PPQAD","DPPQAD","BSPPP","DBSPPP"],"source_ids":closure["source_ids"],"sources":closure["sources"],"direct_subsidiaries":["INTRV","DINTRV","BSPDR","DBSPDR","BSPEV","DBSPEV","BSPVN","DBSPVN"],"profile_machines":["I1MACH","R1MACH","D1MACH"],"xerror_subsidiaries":["XERMSG","XERHLT","XERCNT","XERPRN","XERSVE","XGETUA","J4SAVE","FDUMP","XGETF","XSETF"],"excluded":["PCHIP conversion","PP-to-B-spline conversion","multidimensional PP","smoothing","callbacks","BLAS"]}),
        ),
        (
            "piecewise-polynomial-native-state.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.native-state","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["closure_part","state","origin","effect","policy"],"records":[["PPVAL/DPPVAL, PPQAD/DPPQAD, BSPPP/DBSPPP numeric path","no mutable COMMON/SAVE/DATA","focused source and object audit","all numerical and conversion state is caller-owned or call-local","SerializedGlobal due reachable XERROR"],["XERROR closure","J4SAVE and XERSVE storage","SAVE+DATA","process-global error configuration and messages","SerializedGlobal"],["Fortran I/O and callbacks","none in valid PP path","source audit","no file protocol or user callback","does not relax global policy"]]}),
        ),
        (
            "piecewise-polynomial-concurrency.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_function","backend_profile","class","lock_scope","reason"],"records":[["PiecewisePolynomial::evaluate/derivative/evaluate_into/integrate","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","reachable XERROR and unqualified provider/runtime state"],["BSpline::to_piecewise_polynomial","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","reachable XERROR and unqualified provider/runtime state"],["all PP methods","external_or_system_backend","BackendDependent","process_global_runtime_lock","external provider and runtime storage contract is not qualified"]]}),
        ),
        (
            "piecewise-polynomial-wrapper-index.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","precision","mathematical_model","workspace_formula","runtime_policy","review_state"],"records":wrappers,"counts":{"native_user_callable_routines":6,"public_representation_types":1,"canonical_native_operations":5,"precision_pairs":3,"subsidiaries_linkage_only":8}}),
        ),
        (
            "piecewise-polynomial-conversion-map.json",
            json!({"schema_id":"slatec.safe-piecewise-polynomial.conversion-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["from","to","native_root","status","reason"],"records":[["BSpline","PiecewisePolynomial","BSPPP/DBSPPP","included","exact native conversion; duplicate B-spline knots become distinct PP breakpoints"],["PiecewiseCubicHermite","PiecewisePolynomial","none","deferred","no reviewed native PP conversion; Rust Hermite coefficient reconstruction is intentionally not translated"],["PiecewisePolynomial","BSpline","none","deferred","inverse construction has no reviewed selected root or stable policy"]]}),
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
        "# Safe piecewise-polynomial interpolation\n\n- Snapshot: `{snapshot}`.\n- The `piecewise-polynomial` feature exposes owned real univariate PP form in f32 and f64: scalar and batch `PPVAL`/`DPPVAL` evaluation, derivative evaluation, and exact `PPQAD`/`DPPQAD` definite integration.\n- A piece stores strictly increasing breakpoints and exact native right-Taylor derivative columns. At an interior breakpoint evaluation uses the right piece; the last endpoint uses the final piece. Extrapolation is rejected by default and optional clamping is Rust-side.\n- `BSpline::to_piecewise_polynomial` uses exact `BSPPP`/`DBSPPP` with checked capacities `K*(N+3)`, `K*(N-K+1)`, and `N-K+2`. PCHIP conversion and PP-to-B-spline conversion remain deferred rather than translating coefficients.\n- PP numeric paths have no reviewed mutable numerical COMMON, SAVE, DATA, callback, or I/O state, but reachable XERROR and provider/runtime uncertainty keep every native call under the process-global runtime lock.\n"
    );
    fs::write(
        output_dir.join("piecewise-polynomial-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        routine_count: 6,
    })
}

fn closure() -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/piecewise-polynomial-source-closure.json"),
    )?)?)
}

fn source_records(selected_corpus_dir: &Path) -> Result<Vec<Value>> {
    let selected: Value = serde_json::from_slice(&fs::read(
        selected_corpus_dir.join("selected-providers.json"),
    )?)?;
    let required = ["PPVAL", "DPPVAL", "PPQAD", "DPPQAD", "BSPPP", "DBSPPP"];
    let mut records = BTreeMap::<&str, (&str, &str, &str)>::new();
    for record in selected["records"].as_array().ok_or_else(|| {
        CorpusError::Verification("selected corpus lacks provider records".to_owned())
    })? {
        let Some(name) = record["normalized_name"].as_str() else {
            continue;
        };
        if !required.contains(&name) {
            continue;
        }
        let path = record["source_path"].as_str().ok_or_else(|| {
            CorpusError::Verification(format!("selected PP record {name} lacks source_path"))
        })?;
        let sha256 = record["raw_sha256"].as_str().ok_or_else(|| {
            CorpusError::Verification(format!("selected PP record {name} lacks raw_sha256"))
        })?;
        let role = record["catalogue_classification"].as_str().ok_or_else(|| {
            CorpusError::Verification(format!("selected PP record {name} lacks catalogue role"))
        })?;
        records.insert(name, (path, sha256, role));
    }
    required
        .iter()
        .map(|name| {
            let (path, sha256, role) = records.get(name).copied().ok_or_else(|| {
                CorpusError::Verification(format!(
                    "selected corpus lacks reviewed PP candidate {name}"
                ))
            })?;
            Ok(json!([name, path, sha256, role]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_deterministic_and_has_all_reviewed_roots() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        assert_eq!(one.routine_count, 6);
        for name in [
            "piecewise-polynomial-routine-inventory.json",
            "piecewise-polynomial-candidate-classification.json",
            "piecewise-polynomial-storage-contract.json",
            "piecewise-polynomial-domain-contract.json",
            "piecewise-polynomial-source-closure.json",
            "piecewise-polynomial-native-state.json",
            "piecewise-polynomial-concurrency.json",
            "piecewise-polynomial-wrapper-index.json",
            "piecewise-polynomial-conversion-map.json",
            "piecewise-polynomial-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
