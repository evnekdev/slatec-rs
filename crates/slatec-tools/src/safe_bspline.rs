//! Deterministic metadata from the reviewed scalar B-spline source contracts.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Returns focused native-state projections for the safe B-spline methods.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure = closure()?;
    let objects = closure["source_ids"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("B-spline closure lacks source ids".to_owned()))?
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
        ("slatec::interpolation::bspline::BSpline::evaluate", "BVALU/DBVALU"),
        ("slatec::interpolation::bspline::BSpline::derivative", "BVALU/DBVALU"),
        ("slatec::interpolation::bspline::BSpline::evaluate_into", "BVALU/DBVALU"),
        ("slatec::interpolation::bspline::BSpline::integrate", "BSQAD/DBSQAD"),
    ]
    .into_iter()
    .map(|(safe_function, routine)| {
        let entry_object = if routine.starts_with("BSQAD") {
            "selected-source-d7a112ce8c81f313.o"
        } else {
            "selected-source-287a4838efb0e436.o"
        };
        json!({
            "safe_function":safe_function,
            "native_entry_points":routine.split('/').collect::<Vec<_>>(),
            "feature":"bspline",
            "effective_native_families":["bspline"],
            "entry_object":entry_object,
            "object_closure":objects,
            "source_closure":["BVALU/DBVALU","BSQAD/DBSQAD","INTRV/DINTRV","XERROR closure"],
            "saved_mutable_locals": if routine.starts_with("BSQAD") { vec!["BSQAD/DBSQAD DATA-initialized Gauss nodes and weights; read-only after initialization"] } else { Vec::<&str>::new() },
            "common_blocks":[],
            "xerror_state":["XERROR J4SAVE/XERSVE process-global state reachable on native contract violation"],
            "fortran_io":[],
            "callback_state":["none"],
            "writable_symbols":["focused closure audit: BSQAD/DBSQAD DATA quadrature tables plus XERROR storage"],
            "source_object_unresolved":[],
            "external_undefined_symbols":[],
            "feature_closure_mismatch":false,
            "current_class":"SerializedGlobal",
            "best_possible_class_from_slatec_source":"SerializedGlobal",
            "native_routine_reentrancy":"SerializedGlobal",
            "rust_api_concurrency":"owned spline is movable; every native entry is globally serialized",
            "provider_runtime_thread_safety":"reviewed source and external/system profiles remain serialized",
            "provider_unknowns":["external_or_system_Fortran_runtime_and_provider_contract_not_qualified"],
            "remaining_blockers":["process-global XERROR","DATA quadrature tables","provider/runtime qualification"]
        })
    })
    .collect())
}

/// Concise result from B-spline metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// The selected-corpus snapshot identifier.
    pub snapshot_id: String,
    /// Stable hash over every emitted file.
    pub semantic_hash: String,
    /// Count of reviewed native user-callable B-spline roots.
    pub routine_count: usize,
}

/// Generates B-spline audit and safe-wrapper metadata from explicit reviewed records.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-bspline-metadata requires --offline".to_owned(),
        ));
    }
    let manifest: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = manifest["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let closure = closure()?;
    let candidate_sources = candidate_source_records(selected_corpus_dir)?;
    let inventory = json!([
        [
            "BVALU",
            "f32",
            "value_or_derivative",
            "function",
            "T,A,N,K,IDERIV,X,INBV,WORK",
            "3*K",
            "included"
        ],
        [
            "DBVALU",
            "f64",
            "value_or_derivative",
            "function",
            "T,A,N,K,IDERIV,X,INBV,WORK",
            "3*K",
            "included"
        ],
        [
            "BSQAD",
            "f32",
            "definite_integration",
            "subroutine",
            "T,BCOEF,N,K,X1,X2,BQUAD,WORK",
            "3*K; K<=20",
            "included"
        ],
        [
            "DBSQAD",
            "f64",
            "definite_integration",
            "subroutine",
            "T,BCOEF,N,K,X1,X2,BQUAD,WORK",
            "3*K; K<=20",
            "included"
        ],
        [
            "BINTK",
            "f32",
            "interpolation_construction",
            "subroutine",
            "interpolation knots, values, coefficients, work",
            "banded work",
            "included_by_piecewise_polynomial"
        ],
        [
            "DBINTK",
            "f64",
            "interpolation_construction",
            "subroutine",
            "interpolation knots, values, coefficients, work",
            "banded work",
            "included_by_piecewise_polynomial"
        ],
        [
            "BINT4",
            "f32",
            "interpolation_construction",
            "subroutine",
            "broader construction controls",
            "documented work array",
            "deferred"
        ],
        [
            "DBINT4",
            "f64",
            "interpolation_construction",
            "subroutine",
            "broader construction controls",
            "documented work array",
            "deferred"
        ],
        [
            "BSPVD",
            "f32",
            "basis_and_derivatives",
            "subroutine",
            "basis workspace and interval state",
            "caller workspace",
            "deferred"
        ],
        [
            "DBSPVD",
            "f64",
            "basis_and_derivatives",
            "subroutine",
            "basis workspace and interval state",
            "caller workspace",
            "deferred"
        ],
        [
            "BSPVN",
            "f32",
            "basis_values",
            "subroutine",
            "basis workspace and interval state",
            "caller workspace",
            "deferred"
        ],
        [
            "DBSPVN",
            "f64",
            "basis_values",
            "subroutine",
            "basis workspace and interval state",
            "caller workspace",
            "deferred"
        ],
        [
            "BFQAD",
            "f32",
            "weighted_integration",
            "subroutine",
            "user callback plus spline data",
            "callback work",
            "deferred"
        ],
        [
            "DBFQAD",
            "f64",
            "weighted_integration",
            "subroutine",
            "user callback plus spline data",
            "callback work",
            "deferred"
        ],
        [
            "BSPPP",
            "f32",
            "representation_conversion",
            "subroutine",
            "B-to-PP arrays",
            "caller storage",
            "included_by_piecewise_polynomial"
        ],
        [
            "DBSPPP",
            "f64",
            "representation_conversion",
            "subroutine",
            "B-to-PP arrays",
            "caller storage",
            "included_by_piecewise_polynomial"
        ],
        [
            "PPQAD",
            "f32",
            "piecewise_polynomial_integration",
            "subroutine",
            "PP representation",
            "caller storage",
            "included_by_piecewise_polynomial"
        ],
        [
            "DPPQAD",
            "f64",
            "piecewise_polynomial_integration",
            "subroutine",
            "PP representation",
            "caller storage",
            "included_by_piecewise_polynomial"
        ],
        [
            "BSPEV",
            "f32",
            "value_and_derivative_vector",
            "subroutine",
            "T,AD,N,K,NDERIV,X,INEV,SVALUE,WORK",
            "AD=(2*N-NDERIV+1)*NDERIV/2; WORK=3*K",
            "deferred"
        ],
        [
            "DBSPEV",
            "f64",
            "value_and_derivative_vector",
            "subroutine",
            "T,AD,N,K,NDERIV,X,INEV,SVALUE,WORK",
            "AD=(2*N-NDERIV+1)*NDERIV/2; WORK=3*K",
            "deferred"
        ],
        [
            "BSPDR",
            "f32",
            "derivative_difference_table",
            "subroutine",
            "T,A,N,K,NDERIV,AD",
            "AD=(2*N-NDERIV+1)*NDERIV/2",
            "deferred"
        ],
        [
            "DBSPDR",
            "f64",
            "derivative_difference_table",
            "subroutine",
            "T,A,N,K,NDERIV,AD",
            "AD=(2*N-NDERIV+1)*NDERIV/2",
            "deferred"
        ],
        [
            "BSPDOC",
            "all",
            "package_documentation",
            "non_executable_documentation",
            "none",
            "none",
            "internal_only"
        ],
        [
            "BSPLVN",
            "f32",
            "basis_subsidiary",
            "subroutine",
            "basis recursion state and work",
            "caller workspace",
            "internal_only"
        ],
        [
            "BSPLVD",
            "f32",
            "basis_derivative_subsidiary",
            "subroutine",
            "basis recursion state and work",
            "caller workspace",
            "internal_only"
        ],
        [
            "PPVAL",
            "f32",
            "piecewise_polynomial_evaluation",
            "function",
            "PP representation and point",
            "none",
            "included_by_piecewise_polynomial"
        ],
        [
            "DPPVAL",
            "f64",
            "piecewise_polynomial_evaluation",
            "function",
            "PP representation and point",
            "none",
            "included_by_piecewise_polynomial"
        ]
    ]);
    let wrappers = json!([
        [
            "slatec::interpolation::bspline::BSpline::evaluate",
            "BVALU",
            "f32/f64",
            "sum_j A_j B_j,K(x)",
            "3*K private Vec",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::bspline::BSpline::derivative",
            "BVALU",
            "f32/f64",
            "d^IDERIV/dx^IDERIV sum_j A_j B_j,K(x)",
            "3*K private Vec",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::bspline::BSpline::evaluate_into",
            "BVALU",
            "f32/f64",
            "ordered scalar evaluations",
            "3*K per private native call",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::interpolation::bspline::BSpline::integrate",
            "BSQAD",
            "f32/f64",
            "integral_lower^upper sum_j A_j B_j,K(x) dx",
            "3*K private Vec; K<=20",
            "SerializedGlobal",
            "reviewed"
        ]
    ]);
    let files = [
        (
            "bspline-routine-inventory.json",
            json!({"schema_id":"slatec.safe-bspline.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["routine","precision","operation","role","native_arguments","workspace","safe_status"],"records":inventory,"source_columns":["routine","source_path","source_sha256","catalogue_role"],"source_records":candidate_sources}),
        ),
        (
            "bspline-candidate-classification.json",
            json!({"schema_id":"slatec.safe-bspline.candidate-classification","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["candidate","status","reason"],"records":[["BVALU/DBVALU","included","scalar value and native derivative contract; no callback or persistent caller state"],["BSQAD/DBSQAD","included","bounded-order signed definite integration with exact 3*K workspace"],["BINTK/DBINTK,BINT4/DBINT4","deferred","interpolation-knot, banded-workspace, and endpoint construction surface requires a separate reviewed constructor"],["BSPEV/DBSPEV plus BSPDR/DBSPDR","deferred","multi-derivative output requires a checked derivative-table representation and distinct cached interval-state contract"],["BSPVN/DBSPVN,BSPVD/DBSPVD,BSPLVN,BSPLVD","deferred","basis-vector workspace and interval-state API is not required for the representation facade"],["BFQAD/DBFQAD","deferred","weighted callback containment requires a separate callback audit"],["BSPPP/DBSPPP","included_elsewhere","exact B-spline-to-PP conversion is exposed by the piecewise-polynomial feature"],["PPVAL/DPPVAL,PPQAD/DPPQAD","included_elsewhere","PP storage and evaluation are owned by the piecewise-polynomial feature"],["BSPDOC","internal_only","non-executable package documentation, not a numerical entry point"],["tensor_product,smoothing,NURBS","deferred","outside the scalar representation scope"]]}),
        ),
        (
            "bspline-source-closure.json",
            json!({"schema_id":"slatec.safe-bspline.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"manifest":"crates/slatec-src/metadata/bspline-source-closure.json","roots":["BVALU","DBVALU","BSQAD","DBSQAD"],"source_ids":closure["source_ids"],"sources":closure["sources"],"profile_machines":["I1MACH","R1MACH","D1MACH"],"direct_subsidiaries":["INTRV","DINTRV"],"xerror_subsidiaries":["XERMSG","XERHLT","XERCNT","XERPRN","XERSVE","XGETUA","J4SAVE","FDUMP","XGETF","XSETF"],"excluded":["BINTK","BINT4","BSPEV","BSPDR","BSPVN","BSPVD","BSPLVN","BSPLVD","BFQAD","BSPPP","PPVAL","PPQAD","smoothing","tensor_product","BLAS"]}),
        ),
        (
            "bspline-storage-contract.json",
            json!({"schema_id":"slatec.safe-bspline.storage-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["operation","native_storage","safe_storage","policy"],"records":[["representation","T length N+K; A length N","owned Vec knots and coefficients","from_parts validates and retains exact order"],["BVALU/DBVALU","INBV plus WORK length 3*K","local INBV=1 and fallible private Vec","no search state escapes a call"],["BSQAD/DBSQAD","BQUAD plus WORK length 3*K","local scalar and fallible private Vec","K limited to 20 before native entry"],["batch evaluation","scalar BVALU calls","borrowed points and caller output","no output allocation; preserves point order"]]}),
        ),
        (
            "bspline-knot-contract.json",
            json!({"schema_id":"slatec.safe-bspline.knot-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["property","native_rule","safe_policy"],"records":[["length","T has N+K values","exact checked equality"],["order","N>=K>=1","validated before FFI"],["ordering","nondecreasing T","decrease rejected; no sorting"],["multiplicity","no run longer than K","validated before FFI"],["basic_domain","T(K)<=X<=T(N+1) in Fortran indexing","Rust knots[K-1]..=knots[N] with positive width"],["endpoint_limit","right interior, left at right endpoint","preserved by BVALU/DBVALU"],["outside_domain","not a BVALU mode","reject by default; optional Rust-side endpoint clamp"]]}),
        ),
        (
            "bspline-status-map.json",
            json!({"schema_id":"slatec.safe-bspline.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routines","native_result","safe_interpretation"],"records":[["BVALU/DBVALU","scalar return; invalid contract uses level-two XERMSG","all documented invalid inputs preflighted; any contradicted postcondition is BSplineError::NativeContractViolation"],["BSQAD/DBSQAD","BQUAD output; invalid contract uses level-two XERMSG","all limits, order, dimensions, and work size preflighted"],["all exposed roots","no regular INFO status","safe API returns Result only for Rust validation, allocation, or contract violations"]]}),
        ),
        (
            "bspline-native-state.json",
            json!({"schema_id":"slatec.safe-bspline.native-state","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["closure_part","state","origin","effect","policy"],"records":[["BVALU/DBVALU and INTRV/DINTRV","no COMMON/SAVE/DATA in reviewed numeric path","source audit","caller-local INBV and work only","SerializedGlobal due reachable XERROR"],["BSQAD/DBSQAD","GPTS,GWTS","SAVE+DATA initialized Gauss tables","persistent static object storage, read-only after initialization","SerializedGlobal"],["XERROR closure","J4SAVE and XERSVE storage","SAVE+DATA","process-global error configuration and messages","SerializedGlobal"],["Fortran I/O and callbacks","none in valid B-spline path","source audit","no user callback or file protocol","does not relax global policy"]]}),
        ),
        (
            "bspline-concurrency.json",
            json!({"schema_id":"slatec.safe-bspline.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_function","backend_profile","class","lock_scope","reason"],"records":[["BSpline::evaluate/derivative/evaluate_into","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","reachable XERROR and unqualified provider/runtime state"],["BSpline::integrate","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","BSQAD/DBSQAD DATA tables plus XERROR and provider/runtime state"],["all B-spline methods","external_or_system_backend","BackendDependent","process_global_runtime_lock","external provider and runtime storage contract is not qualified"]]}),
        ),
        (
            "bspline-wrapper-index.json",
            json!({"schema_id":"slatec.safe-bspline.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","precision","mathematical_model","workspace_formula","runtime_policy","review_state"],"records":wrappers,"counts":{"native_user_callable_routines":4,"public_representation_types":1,"canonical_native_operations":4,"precision_pairs":2,"subsidiaries_linkage_only":12}}),
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
        "# Safe B-spline construction and evaluation\n\n- Snapshot: `{snapshot}`.\n- The partial `bspline` feature exposes existing scalar B-representations only: `BVALU`/`DBVALU` values and derivatives, plus `BSQAD`/`DBSQAD` definite integration, in f32 and f64.\n- `BSpline` owns exact native knots, coefficients, and order. Validation enforces `T.len() = N + K`, `N >= K >= 1`, finite nondecreasing knots, knot multiplicity at most `K`, a positive basic domain, and finite coefficients. No sorting, duplicate merging, knot insertion, conversion, or fitting occurs.\n- Evaluation and integration have exact checked private workspace `3*K`. Integration additionally rejects `K > 20`, the reviewed `BSQAD`/`DBSQAD` limit.\n- `BVALU` interval search starts from local `INBV=1` every call. All native entry is serialized through the process-global runtime lock and scoped XERROR restoration.\n- Interpolation construction, basis vectors, weighted callbacks, PP conversion, tensor-product splines, smoothing, NURBS, arbitrary strides, adapters, and translated algorithms remain deferred.\n"
    );
    fs::write(
        output_dir.join("bspline-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        routine_count: 4,
    })
}

fn closure() -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/bspline-source-closure.json"),
    )?)?)
}

fn candidate_source_records(selected_corpus_dir: &Path) -> Result<Vec<Value>> {
    let selected: Value = serde_json::from_slice(&fs::read(
        selected_corpus_dir.join("selected-providers.json"),
    )?)?;
    let required = [
        "BVALU", "DBVALU", "BSQAD", "DBSQAD", "BINTK", "DBINTK", "BINT4", "DBINT4", "BSPEV",
        "DBSPEV", "BSPDR", "DBSPDR", "BSPVN", "DBSPVN", "BSPVD", "DBSPVD", "BSPLVN", "BSPLVD",
        "BFQAD", "DBFQAD", "BSPPP", "DBSPPP", "PPVAL", "DPPVAL", "PPQAD", "DPPQAD", "BSPDOC",
    ];
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
            CorpusError::Verification(format!("selected B-spline record {name} lacks source_path"))
        })?;
        let sha256 = record["raw_sha256"].as_str().ok_or_else(|| {
            CorpusError::Verification(format!("selected B-spline record {name} lacks raw_sha256"))
        })?;
        let role = record["catalogue_classification"].as_str().ok_or_else(|| {
            CorpusError::Verification(format!(
                "selected B-spline record {name} lacks catalogue role"
            ))
        })?;
        records.insert(name, (path, sha256, role));
    }
    required
        .iter()
        .map(|name| {
            let (path, sha256, role) = records.get(name).copied().ok_or_else(|| {
                CorpusError::Verification(format!(
                    "selected corpus lacks reviewed B-spline candidate {name}"
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
        assert_eq!(one.routine_count, 4);
        for name in [
            "bspline-routine-inventory.json",
            "bspline-candidate-classification.json",
            "bspline-source-closure.json",
            "bspline-storage-contract.json",
            "bspline-knot-contract.json",
            "bspline-status-map.json",
            "bspline-native-state.json",
            "bspline-concurrency.json",
            "bspline-wrapper-index.json",
            "bspline-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
