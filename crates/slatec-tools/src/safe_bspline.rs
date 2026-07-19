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
        (
            "slatec::interpolation::bspline::BSpline::interpolate_with_knots",
            "BINTK/DBINTK",
        ),
    ]
    .into_iter()
    .map(|(safe_function, routine)| {
        let entry_object = if routine.starts_with("BSQAD") {
            "selected-source-d7a112ce8c81f313.o"
        } else if routine.starts_with("BINTK") {
            "bspline-construction-bintk.o"
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
            "source_closure": if routine.starts_with("BINTK") { vec!["BINTK/DBINTK","BNFAC/DBNFAC","BNSLV/DBNSLV","BSPVN/DBSPVN","XERROR closure"] } else { vec!["BVALU/DBVALU","BSQAD/DBSQAD","INTRV/DINTRV","XERROR closure"] },
            "saved_mutable_locals": if routine.starts_with("BSQAD") { vec!["BSQAD/DBSQAD DATA-initialized Gauss nodes and weights; read-only after initialization"] } else { Vec::<&str>::new() },
            "common_blocks":[],
            "xerror_state":["XERROR J4SAVE/XERSVE process-global state reachable on native contract violation"],
            "fortran_io":[],
            "callback_state":["none"],
            "writable_symbols": if routine.starts_with("BINTK") { vec!["focused closure audit: BINTK/DBINTK plus BNFAC/DBNFAC, BNSLV/DBNSLV, and BSPVN/DBSPVN have no COMMON, SAVE, DATA, or writable numerical static storage; XERROR storage remains reachable"] } else { vec!["focused closure audit: BSQAD/DBSQAD DATA quadrature tables plus XERROR storage"] },
            "source_object_unresolved":[],
            "external_undefined_symbols":[],
            "feature_closure_mismatch":false,
            "current_class":"SerializedGlobal",
            "best_possible_class_from_slatec_source":"SerializedGlobal",
            "native_routine_reentrancy":"SerializedGlobal",
            "rust_api_concurrency":"owned spline is movable; every native entry is globally serialized",
            "provider_runtime_thread_safety":"reviewed source and external/system profiles remain serialized",
            "provider_unknowns":["external_or_system_Fortran_runtime_and_provider_contract_not_qualified"],
            "remaining_blockers": if routine.starts_with("BINTK") { vec!["process-global XERROR","provider/runtime qualification"] } else { vec!["process-global XERROR","DATA quadrature tables","provider/runtime qualification"] }
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
            "X,Y,T,N,K,BCOEF,Q,WORK",
            "BCOEF=N; Q=(2*K-1)*N; WORK=2*K",
            "included"
        ],
        [
            "DBINTK",
            "f64",
            "interpolation_construction",
            "subroutine",
            "X,Y,T,N,K,BCOEF,Q,WORK",
            "BCOEF=N; Q=(2*K-1)*N; WORK=2*K",
            "included"
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
        ],
        [
            "slatec::interpolation::bspline::BSpline::interpolate_with_knots",
            "BINTK",
            "f32/f64",
            "find BCOEF such that sum_j BCOEF_j B_j,K(X_i)=Y_i",
            "Q=(2*K-1)*N private Vec; WORK=2*K private Vec",
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
            json!({"schema_id":"slatec.safe-bspline.candidate-classification","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["candidate","status","reason"],"records":[["BVALU/DBVALU","included","scalar value and native derivative contract; no callback or persistent caller state"],["BSQAD/DBSQAD","included","bounded-order signed definite integration with exact 3*K workspace"],["BINTK/DBINTK","included","general-order exact interpolation with caller-supplied complete knots, checked Schoenberg--Whitney preflight, and private banded factorization storage"],["BINT4/DBINT4","deferred","fixed cubic endpoint-condition and knot-placement policy is materially distinct from the selected general-order constructor"],["BSINT/DBSINT","absent_from_selected_snapshot","historical search lead; no selected provider record exists"],["BSPEV/DBSPEV plus BSPDR/DBSPDR","deferred","multi-derivative output requires a checked derivative-table representation and distinct cached interval-state contract"],["BSPVN/DBSPVN,BSPVD/DBSPVD,BSPLVN,BSPLVD","deferred","basis-vector workspace and interval-state API is not required for the representation facade"],["BFQAD/DBFQAD","deferred","weighted callback containment requires a separate callback audit"],["BSPPP/DBSPPP","included_elsewhere","exact B-spline-to-PP conversion is exposed by the piecewise-polynomial feature"],["PPVAL/DPPVAL,PPQAD/DPPQAD","included_elsewhere","PP storage and evaluation are owned by the piecewise-polynomial feature"],["BSPDOC","internal_only","non-executable package documentation, not a numerical entry point"],["tensor_product,smoothing,NURBS","deferred","outside the scalar representation scope"]]}),
        ),
        (
            "bspline-source-closure.json",
            json!({"schema_id":"slatec.safe-bspline.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"manifest":"crates/slatec-src/metadata/bspline-source-closure.json","roots":["BVALU","DBVALU","BSQAD","DBSQAD","BINTK","DBINTK"],"source_ids":closure["source_ids"],"sources":closure["sources"],"profile_machines":["I1MACH","R1MACH","D1MACH"],"direct_subsidiaries":["INTRV","DINTRV","BNFAC","DBNFAC","BNSLV","DBNSLV","BSPVN","DBSPVN"],"xerror_subsidiaries":["XERMSG","XERHLT","XERCNT","XERPRN","XERSVE","XGETUA","J4SAVE","FDUMP","XGETF","XSETF"],"excluded":["BINT4","DBINT4","BSPEV","BSPDR","BSPVD","BSPLVN","BSPLVD","BFQAD","BSPPP","PPVAL","PPQAD","smoothing","tensor_product","BLAS"]}),
        ),
        (
            "bspline-storage-contract.json",
            json!({"schema_id":"slatec.safe-bspline.storage-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["operation","native_storage","safe_storage","policy"],"records":[["representation","T length N+K; A length N","owned Vec knots and coefficients","from_parts validates and retains exact order"],["BINTK/DBINTK","X,Y length N; T and BCOEF length N+K and N; Q length (2*K-1)*N; WORK length 2*K","borrowed nodes, values, and knots; owned returned knots and coefficients; private fallible Q and WORK Vecs","no sorting, automatic knots, factorization retention, or workspace exposure"],["BVALU/DBVALU","INBV plus WORK length 3*K","local INBV=1 and fallible private Vec","no search state escapes a call"],["BSQAD/DBSQAD","BQUAD plus WORK length 3*K","local scalar and fallible private Vec","K limited to 20 before native entry"],["batch evaluation","scalar BVALU calls","borrowed points and caller output","no output allocation; preserves point order"]]}),
        ),
        (
            "bspline-knot-contract.json",
            json!({"schema_id":"slatec.safe-bspline.knot-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["property","native_rule","safe_policy"],"records":[["length","T has N+K values","exact checked equality"],["order","N>=K>=1","validated before FFI"],["ordering","nondecreasing T","decrease rejected; no sorting"],["multiplicity","no run longer than K","validated before FFI"],["BINTK endpoints","T(1..K)<=X(1) and T(N+1..N+K)>=X(N)","checked before FFI"],["Schoenberg--Whitney","T(I)<X(I)<T(I+K); endpoint equality only with K endpoint knots","checked with zero-based typed error"],["basic_domain","T(K)<=X<=T(N+1) in Fortran indexing","Rust knots[K-1]..=knots[N] with positive width"],["endpoint_limit","right interior, left at right endpoint","preserved by BVALU/DBVALU"],["outside_domain","not a BVALU mode","reject by default; optional Rust-side endpoint clamp"]]}),
        ),
        (
            "bspline-status-map.json",
            json!({"schema_id":"slatec.safe-bspline.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routines","native_result","safe_interpretation"],"records":[["BINTK/DBINTK","BCOEF output; malformed or singular systems issue level-one XERMSG and return without INFO","Rust preflights all documented invalid-data and Schoenberg--Whitney cases; nonfinite coefficients or failure to reproduce nodes maps to BSplineError::SingularInterpolationSystem after scoped XERROR restoration"],["BVALU/DBVALU","scalar return; invalid contract uses level-two XERMSG","all documented invalid inputs preflighted; any contradicted postcondition is BSplineError::NativeContractViolation"],["BSQAD/DBSQAD","BQUAD output; invalid contract uses level-two XERMSG","all limits, order, dimensions, and work size preflighted"],["all exposed roots","no regular INFO status","safe API returns Result only for Rust validation, allocation, checked construction verification, or contract violations"]]}),
        ),
        (
            "bspline-native-state.json",
            json!({"schema_id":"slatec.safe-bspline.native-state","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["closure_part","state","origin","effect","policy"],"records":[["BINTK/DBINTK, BNFAC/DBNFAC, BNSLV/DBNSLV, BSPVN/DBSPVN","no COMMON/SAVE/DATA/BLOCK DATA or Fortran I/O in reviewed constructor closure","source and object audit","all numerical state is caller-private BCOEF, Q, and WORK","SerializedGlobal due reachable XERROR and unqualified provider/runtime state"],["BVALU/DBVALU and INTRV/DINTRV","no COMMON/SAVE/DATA in reviewed numeric path","source audit","caller-local INBV and work only","SerializedGlobal due reachable XERROR"],["BSQAD/DBSQAD","GPTS,GWTS","SAVE+DATA initialized Gauss tables","persistent static object storage, read-only after initialization","SerializedGlobal"],["XERROR closure","J4SAVE and XERSVE storage","SAVE+DATA","process-global error configuration and messages","SerializedGlobal"],["Fortran I/O and callbacks","none in valid B-spline path","source audit","no user callback or file protocol","does not relax global policy"]]}),
        ),
        (
            "bspline-concurrency.json",
            json!({"schema_id":"slatec.safe-bspline.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_function","backend_profile","class","lock_scope","reason"],"records":[["BSpline::interpolate_with_knots","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","reachable XERROR and unqualified provider/runtime state; constructor factorization workspace is private but does not establish native reentrancy"],["BSpline::evaluate/derivative/evaluate_into","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","reachable XERROR and unqualified provider/runtime state"],["BSpline::integrate","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","BSQAD/DBSQAD DATA tables plus XERROR and provider/runtime state"],["all B-spline methods","external_or_system_backend","BackendDependent","process_global_runtime_lock","external provider and runtime storage contract is not qualified"]]}),
        ),
        (
            "bspline-wrapper-index.json",
            json!({"schema_id":"slatec.safe-bspline.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","precision","mathematical_model","workspace_formula","runtime_policy","review_state"],"records":wrappers,"counts":{"native_user_callable_routines":6,"public_representation_types":1,"canonical_native_operations":5,"precision_pairs":2,"subsidiaries_linkage_only":18}}),
        ),
        (
            "bspline-construction-routine-inventory.json",
            json!({"schema_id":"slatec.safe-bspline-construction.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","precision","role","native_arguments","workspace","inclusion"],"records":[["BINTK","f32","general_order_exact_constructor","X,Y,T,N,K,BCOEF,Q,WORK","Q=(2*K-1)*N; WORK=2*K","included"],["DBINTK","f64","general_order_exact_constructor","X,Y,T,N,K,BCOEF,Q,WORK","Q=(2*K-1)*N; WORK=2*K","included"],["BINT4","f32","fixed_cubic_special_constructor","X,Y,NDATA,IBCL,IBCR,FBCL,FBCR,KNTOPT,T,BCOEF,N,K,W","W=5*(NDATA+2)","deferred"],["DBINT4","f64","fixed_cubic_special_constructor","X,Y,NDATA,IBCL,IBCR,FBCL,FBCR,KNTOPT,T,BCOEF,N,K,W","W=5*(NDATA+2)","deferred"],["BNFAC/DBNFAC","f32/f64","banded_factorization_subsidiary","W,NROWW,NROW,NBANDL,NBANDU,IFLAG","owned by BINTK/DBINTK","internal"],["BNSLV/DBNSLV","f32/f64","banded_substitution_subsidiary","W,NROWW,NROW,NBANDL,NBANDU,B","owned by BINTK/DBINTK","internal"],["BSPVN/DBSPVN","f32/f64","basis_subsidiary","T,JHIGH,K,INDEX,X,ILEFT,VNIKX,WORK,IWORK","WORK=2*K","internal"],["BSINT/DBSINT","none","historical_search_lead","not present in selected snapshot","none","absent"]],"source_columns":["routine","source_path","source_sha256","catalogue_role"],"source_records":candidate_sources}),
        ),
        (
            "bspline-construction-candidate-classification.json",
            json!({"schema_id":"slatec.safe-bspline-construction.candidate-classification","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["candidate","classification","reason"],"records":[["BINTK/DBINTK","selected_exact_interpolation_constructor","general order, exact collocation, explicit complete knots, no callbacks or persistent workspace"],["BINT4/DBINT4","special_knot_interpolation_constructor_deferred","fixed cubic order with endpoint-condition and knot-placement policy outside the selected general-order API"],["BNFAC/DBNFAC and BNSLV/DBNSLV","linear_system_subsidiaries","private native factorization and substitution only"],["BSPVN/DBSPVN","basis_subsidiaries","required internally by selected constructor; no public basis-vector API"],["BSINT/DBSINT","absent_from_selected_snapshot","no selected provider record"]]}),
        ),
        (
            "bspline-construction-data-contract.json",
            json!({"schema_id":"slatec.safe-bspline-construction.data-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["property","native_contract","safe_policy"],"records":[["equation","sum_j BCOEF(j)*B_j,K(X(i))=Y(i), i=1..N","returns existing owned BSpline after independent node reproduction audit"],["nodes","length N, strictly increasing","borrowed slice; finite and strictly increasing before FFI"],["values","length N","borrowed finite slice; no fitting, weights, or smoothing"],["order","N>=K>=1; order means degree+1","checked Rust usize converted to Fortran INTEGER"],["coefficient output","BCOEF length N","private owned buffer becomes returned coefficients"],["mutation","BCOEF,Q,WORK overwritten; X,Y,T read","no user buffer is passed as writable native storage"]]}),
        ),
        (
            "bspline-construction-knot-contract.json",
            json!({"schema_id":"slatec.safe-bspline-construction.knot-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["property","native_contract","safe_policy"],"records":[["representation","T is complete length N+K knot sequence","caller supplies exact sequence; copied without automatic generation, sorting, or merging"],["endpoint placement","T(1..K)<=X(1); T(N+1..N+K)>=X(N)","checked before FFI"],["solvability","T(I)<X(I)<T(I+K); endpoint equality only with K knots at X(1) or X(N)","checked as zero-based SchoenbergWhitneyViolation"],["ordering","nondecreasing knots","existing BSpline validation; no equal run longer than K"],["boundary condition","determined entirely by supplied knots","no natural, not-a-knot, periodic, or hidden endpoint policy"]]}),
        ),
        (
            "bspline-construction-workspace-contract.json",
            json!({"schema_id":"slatec.safe-bspline-construction.workspace-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["buffer","formula","ownership","overflow_policy"],"records":[["returned knots","N+K","owned exact copy","checked_add and fallible allocation"],["returned coefficients","N","owned native output","fallible allocation"],["Q","(2*K-1)*N","private native factorization","checked_mul, checked_sub, and fallible allocation"],["WORK","2*K","private basis scratch","checked_mul and fallible allocation"],["persistent state","none","all construction storage dropped after verified return","not exposed"]]}),
        ),
        (
            "bspline-construction-source-closure.json",
            json!({"schema_id":"slatec.safe-bspline-construction.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"manifest":"crates/slatec-src/metadata/bspline-source-closure.json","roots":["BINTK","DBINTK"],"direct_subsidiaries":["BNFAC","DBNFAC","BNSLV","DBNSLV","BSPVN","DBSPVN"],"shared_subsidiaries":["XERMSG","XERHLT","XERCNT","XERPRN","XERSVE","XGETUA","J4SAVE","FDUMP","XGETF","XSETF"],"source_ids":closure["source_ids"],"sources":closure["sources"],"excluded":["BINT4","DBINT4","smoothing","least_squares_fitting","tensor_product","PCHIP","public_banded_api","BLAS"]}),
        ),
        (
            "bspline-construction-native-state.json",
            json!({"schema_id":"slatec.safe-bspline-construction.native-state","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["closure_part","persistent_state","I/O","effect"],"records":[["BINTK/DBINTK","none","none","numerical state is caller-private BCOEF,Q,WORK"],["BNFAC/DBNFAC,BNSLV/DBNSLV,BSPVN/DBSPVN","none","none","no COMMON, SAVE, DATA, or callback state in the reviewed closure"],["XERROR","J4SAVE/XERSVE process-global state","error printing controlled by scoped runtime policy","requires process-wide serialization and restoration"]]}),
        ),
        (
            "bspline-construction-concurrency.json",
            json!({"schema_id":"slatec.safe-bspline-construction.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_function","backend_profile","class","lock_scope","reason"],"records":[["BSpline::interpolate_with_knots","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","XERROR is reachable and provider/runtime concurrency is not qualified"],["BSpline::interpolate_with_knots","external_or_system_backend","BackendDependent","process_global_runtime_lock","external provider and runtime contract remains unqualified"]]}),
        ),
        (
            "bspline-construction-wrapper-index.json",
            json!({"schema_id":"slatec.safe-bspline-construction.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_path","native_routines","precision","result","review_state"],"records":[["slatec::interpolation::bspline::BSpline::interpolate_with_knots","BINTK/DBINTK","f32/f64","existing owned BSpline<T>","reviewed"]],"counts":{"native_roots":2,"safe_constructors":1,"public_representation_types_added":0,"internal_subsidiaries":6}}),
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
        "# Safe B-spline construction and evaluation\n\n- Snapshot: `{snapshot}`.\n- The partial hosted `bspline` feature exposes `BINTK`/`DBINTK` exact general-order interpolation with caller-supplied complete knots, `BVALU`/`DBVALU` values and derivatives, and `BSQAD`/`DBSQAD` definite integration, in f32 and f64.\n- `BSpline::interpolate_with_knots` enforces finite strictly increasing nodes, finite values, `N >= K >= 1`, complete finite nondecreasing `T.len() = N + K`, endpoint placement, and the documented Schoenberg--Whitney condition before FFI. It never generates, sorts, inserts, or merges knots.\n- Construction owns only the returned knots, coefficients, and order. It uses exact checked private `Q=(2*K-1)*N` and `WORK=2*K` allocations, drops factorization state before returning, and verifies that the output reproduces all input nodes.\n- Evaluation and integration have exact checked private workspace `3*K`. Integration additionally rejects `K > 20`, the reviewed `BSQAD`/`DBSQAD` limit.\n- All native entry is serialized through the process-global runtime lock and scoped XERROR restoration. BINT4/DBINT4 fixed-cubic construction, basis vectors, weighted callbacks, tensor-product splines, smoothing, NURBS, arbitrary strides, adapters, and translated algorithms remain deferred.\n"
    );
    fs::write(
        output_dir.join("bspline-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    let construction_summary = format!(
        "# Safe exact univariate B-spline interpolation construction\n\n- Snapshot: `{snapshot}`.\n- Selected native family: `BINTK`/`DBINTK`, one general-order exact interpolation constructor pair. It solves the total-positive banded collocation system without pivoting and returns ordinary owned `BSpline<f32>` or `BSpline<f64>` values.\n- Public constructor: `BSpline::interpolate_with_knots(nodes, values, knots, order)`. Knots are caller supplied because the selected driver does not generate a knot policy.\n- Exact private workspace: coefficients `N`, factorization `Q=(2*K-1)*N`, scratch `WORK=2*K`. All arithmetic is checked and allocation is fallible.\n- Solvability: strictly increasing nodes, endpoint placement, and Schoenberg--Whitney support are preflighted. The driver has no INFO output; an invalid verified result maps to `SingularInterpolationSystem` after XERROR restoration.\n- `BINT4`/`DBINT4` remain deferred: their fixed cubic order, endpoint-condition switches, and automatic knot-placement policies are materially different.\n- Native calls remain `SerializedGlobal`; no callback, file I/O, source artifact, or retained factorization is introduced.\n"
    );
    fs::write(
        output_dir.join("bspline-construction-validation-summary.md"),
        construction_summary.as_bytes(),
    )?;
    bytes.extend_from_slice(construction_summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        routine_count: 5,
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
        "BVALU", "DBVALU", "BSQAD", "DBSQAD", "BINTK", "DBINTK", "BINT4", "DBINT4", "BNFAC",
        "DBNFAC", "BNSLV", "DBNSLV", "BSPEV", "DBSPEV", "BSPDR", "DBSPDR", "BSPVN", "DBSPVN",
        "BSPVD", "DBSPVD", "BSPLVN", "BSPLVD", "BFQAD", "DBFQAD", "BSPPP", "DBSPPP", "PPVAL",
        "DPPVAL", "PPQAD", "DPPQAD", "BSPDOC",
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
        assert_eq!(one.routine_count, 5);
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
            "bspline-construction-routine-inventory.json",
            "bspline-construction-candidate-classification.json",
            "bspline-construction-data-contract.json",
            "bspline-construction-knot-contract.json",
            "bspline-construction-workspace-contract.json",
            "bspline-construction-source-closure.json",
            "bspline-construction-native-state.json",
            "bspline-construction-concurrency.json",
            "bspline-construction-wrapper-index.json",
            "bspline-construction-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
