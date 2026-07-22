//! Deterministic metadata for the reviewed Cartesian FISHPACK M1 facade.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const HWSCRT_SHA256: &str = "9bcd5a3be9e6d63e7dcc33637eb37ef07ba10b727b74859d08cb4daa7f813202";

/// Returns focused native-state evidence for the one Cartesian HWSCRT facade.
///
/// The exact selected closure has no mutable FISHPACK storage, callbacks,
/// XERROR, or Fortran I/O.  It still remains under the process-wide hosted
/// native lock: this is a conservative backend/runtime policy, not a claim
/// that arbitrary external Fortran providers are reentrant.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure: Value = serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/fishpack-cartesian-2d-source-closure.json"),
    )?)?;
    let provider: Value = serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/family-source-closure.json"),
    )?)?;
    let objects = provider["families"]["fishpack-cartesian-2d"]
        .as_array()
        .ok_or_else(|| {
            CorpusError::Verification("materialized FISHPACK closure lacks source ids".to_owned())
        })?
        .iter()
        .filter_map(Value::as_str)
        .map(|id| format!("{id}.o"))
        .collect::<Vec<_>>();
    let sources = closure["sources"]
        .as_array()
        .ok_or_else(|| {
            CorpusError::Verification("FISHPACK closure lacks source records".to_owned())
        })?
        .iter()
        .filter_map(|source| source["path"].as_str())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    Ok(vec![json!({
        "safe_function":"slatec::differential_equations::pde::CartesianHelmholtz2d::solve",
        "native_entry_points":["HWSCRT"],
        "feature":"fishpack-cartesian-2d",
        "effective_native_families":["fishpack-cartesian-2d"],
        "entry_object":"selected-source-590904723b300b8f.o",
        "object_closure":objects,
        "source_closure":sources,
        "saved_mutable_locals":Vec::<&str>::new(),
        "common_blocks":[],
        "xerror_state":[],
        "fortran_io":[],
        "callback_state":["none"],
        "writable_symbols":["focused HWSCRT closure audit: no mutable COMMON, SAVE, DATA, BLOCK DATA, callback, XERROR, or Fortran I/O state"],
        "source_object_unresolved":[],
        "external_undefined_symbols":[],
        "feature_closure_mismatch":false,
        "current_class":"SerializedGlobal",
        "best_possible_class_from_slatec_source":"BackendDependent",
        "native_routine_reentrancy":"no persistent numerical state found; runtime gate intentionally remains exclusive",
        "rust_api_concurrency":"owned grids and boundary data are movable; every native solve uses the process-global runtime lock",
        "provider_runtime_thread_safety":"unqualified external and system provider/runtime contracts",
        "provider_unknowns":["compiler and external provider runtime contract is not qualified for concurrent native entry"],
        "remaining_blockers":["repository-wide process-global native serialization policy", "provider/runtime qualification"]
    })])
}

/// Concise result of Cartesian-FISHPACK metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus identity.
    pub snapshot_id: String,
    /// Stable hash of every emitted artifact.
    pub semantic_hash: String,
    /// Reviewed native candidate count.
    pub candidate_count: usize,
    /// Public safe wrapper count.
    pub wrapper_count: usize,
}

/// Generates explicit M1 metadata without inferring a broader FISHPACK API.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-fishpack-cartesian-2d-metadata requires --offline".to_owned(),
        ));
    }

    let selected: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = selected["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let closure: Value = serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/fishpack-cartesian-2d-source-closure.json"),
    )?)?;
    let sources = closure["sources"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("FISHPACK closure lacks sources".to_owned()))?;
    if sources.len() != 11
        || sources
            .iter()
            .find(|source| source["id"].as_str() == Some("fishpack-cartesian-2d-hwscrt"))
            .and_then(|source| source["sha256"].as_str())
            != Some(HWSCRT_SHA256)
    {
        return Err(CorpusError::Verification(
            "FISHPACK Cartesian M1 closure no longer matches the reviewed HWSCRT revision"
                .to_owned(),
        ));
    }

    let classified_sources = sources
        .iter()
        .map(|source| {
            let id = source["id"].as_str().unwrap_or("unknown");
            let role = match id {
                "fishpack-cartesian-2d-hwscrt" => "public_FISHPACK_driver",
                "fishpack-cartesian-2d-genbun" => "FISHPACK_subsidiary",
                "fishpack-cartesian-2d-poisd2"
                | "fishpack-cartesian-2d-poisn2"
                | "fishpack-cartesian-2d-poisp2"
                | "fishpack-cartesian-2d-cosgen"
                | "fishpack-cartesian-2d-trix"
                | "fishpack-cartesian-2d-tri3" => "cyclic_reduction_primitive",
                "fishpack-cartesian-2d-s1merg" => "structured_linear_algebra_helper",
                "fishpack-cartesian-2d-pimach" => "machine_constant",
                "selected-source-b3c3b83a9ceaaeb4" => "shared_utility_blas_scopy",
                _ => "unreviewed",
            };
            json!([id, source["subset"], source["path"], source["sha256"], role])
        })
        .collect::<Vec<_>>();

    let wrappers = vec![json!([
        "slatec::differential_equations::pde::CartesianHelmholtz2d::solve",
        "HWSCRT",
        "hwscrt_",
        "f32",
        "u_xx + u_yy + lambda*u = f on a uniform Cartesian rectangle",
        "owned_grid_and_boundary_data",
        "SerializedGlobal"
    ])];

    let files = [
        (
            "fishpack-cartesian-2d-candidate-index.json",
            json!({
                "schema_id":"slatec.safe-fishpack.cartesian-2d.candidate-index",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "columns":["routine","selected","provider","path","sha256","precision","decision"],
                "records":[
                    ["HWSCRT",true,"SLATEC relocated fishfft subset","fishfft/hwscrt.f",HWSCRT_SHA256,"f32","selected Cartesian rectangular five-point driver"],
                    ["HWSCRT",false,"standalone Netlib FISHPACK","fishpack/hwscrt.f",null,"f32","not selected: materially different later revision; selected cached SLATEC body is ABI authority"],
                    ["GENBUN",false,"SLATEC relocated fishfft subset","fishfft/genbun.f","c54069e07f3b22162ab776f7a165213b6f21e67d482d38835888a6915b872af2","f32","transitive subsidiary only; no public API"]
                ]
            }),
        ),
        (
            "fishpack-cartesian-2d-source-closure.json",
            json!({
                "schema_id":"slatec.safe-fishpack.cartesian-2d.source-closure",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "manifest":"crates/slatec-src/metadata/fishpack-cartesian-2d-source-closure.json",
                "roots":["HWSCRT"],
                "source_ids":closure["source_ids"],
                "columns":["source_id","subset","path","sha256","role"],
                "records":classified_sources,
                "closure_size":11,
                "narrow_link_probe":{
                    "binary":"fishpack_cartesian_2d_native source-backend integration test",
                    "required_symbols":["hwscrt_","genbun_","poisd2_","poisn2_","poisp2_","cosgen_","s1merg_","trix_","tri3_","pimach_","scopy_"],
                    "status":"passed"
                },
                "shared_provider_relationships":[
                    ["SCOPY","selected-source-b3c3b83a9ceaaeb4","canonical existing LIN/BLAS source identity reused"],
                    ["FFTPACK","none","HWSCRT calls GENBUN directly; no FFTPACK transform root is in this exact closure"],
                    ["XERROR","none","no XERROR/XERMSG dependency is reachable from the selected closure"]
                ],
                "unresolved_external_dependencies":[],
                "duplicate_definition_risk":"none in the reviewed narrow closure",
                "deferred":["GENBUN public API","BLKTRI public API","complex drivers"],
                "outside_this_cartesian_m1_scope":["POIS3D has its own structured-system facade","cylindrical and polar drivers have the fishpack-cylindrical-polar facade","unit-sphere and axisymmetric spherical drivers have the fishpack-spherical facade"]
            }),
        ),
        (
            "fishpack-cartesian-2d-raw-contracts.json",
            json!({
                "schema_id":"slatec.safe-fishpack.cartesian-2d.raw-contracts",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "routine":"HWSCRT",
                "symbol":"hwscrt_",
                "calling_convention":"Fortran external subroutine through the GNU MinGW C ABI",
                "fortran_integer":"i32",
                "scalar":"f32",
                "columns":["position","argument","direction","native_extent_or_meaning","safe_mapping"],
                "records":[
                    [1,"A","in","x lower endpoint","UniformAxis::lower"],
                    [2,"B","in","x upper endpoint","UniformAxis::upper"],
                    [3,"M","in","x panel count","UniformAxis::intervals"],
                    [4,"MBDCND","in","x boundary code 0..4","AxisBoundary private encoding"],
                    [5,"BDA","in","N+1 x-lower derivative samples when code 3 or 4; dummy otherwise","owned private edge buffer"],
                    [6,"BDB","in","N+1 x-upper derivative samples when code 2 or 3; dummy otherwise","owned private edge buffer"],
                    [7,"C","in","y lower endpoint","UniformAxis::lower"],
                    [8,"D","in","y upper endpoint","UniformAxis::upper"],
                    [9,"N","in","y panel count","UniformAxis::intervals"],
                    [10,"NBDCND","in","y boundary code 0..4","AxisBoundary private encoding"],
                    [11,"BDC","in","M+1 y-lower derivative samples when code 3 or 4; dummy otherwise","owned private edge buffer"],
                    [12,"BDD","in","M+1 y-upper derivative samples when code 2 or 3; dummy otherwise","owned private edge buffer"],
                    [13,"ELMBDA","in","lambda","CartesianHelmholtz2d::coefficient"],
                    [14,"F","in/out","F(IDIMF,N+1), IDIMF >= M+1","owned Grid2, x-fast storage with IDIMF=M+1"],
                    [15,"IDIMF","in","first F dimension","private checked M+1"],
                    [16,"PERTRB","out","constant subtracted from RHS for singular Poisson combinations","CartesianPdeSolution::perturbation"],
                    [17,"IERROR","out","0 success; 6 positive lambda warning after attempted solve; others invalid input","NativePdeStatus or checked PdeError"],
                    [18,"W","out/work","at most 4*(N+1)+(13+floor(log2(N+1)))*(M+1); W(1) reports actual required length","private checked workspace"]
                ]
            }),
        ),
        (
            "fishpack-cartesian-2d-boundary-contract.json",
            json!({
                "schema_id":"slatec.safe-fishpack.cartesian-2d.boundary-contract",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "columns":["code","safe_variant","lower_endpoint","upper_endpoint","edge_length","derivative_direction","F_boundary_policy"],
                "records":[
                    [0,"Periodic","identified with upper","identified with lower","none","not applicable","duplicate endpoint RHS samples must be exactly equal"],
                    [1,"Dirichlet","value","value","other-axis nodes","not applicable","safe wrapper writes supplied values to F"],
                    [2,"DirichletNeumann","value","increasing-coordinate derivative","other-axis nodes","positive x or y coordinate","safe wrapper writes lower value; upper F remains RHS"],
                    [3,"Neumann","increasing-coordinate derivative","increasing-coordinate derivative","other-axis nodes","positive x or y coordinate","F boundaries remain RHS"],
                    [4,"NeumannDirichlet","increasing-coordinate derivative","value","other-axis nodes","positive x or y coordinate","safe wrapper writes upper value; lower F remains RHS"]
                ],
                "corner_policy":"Every supplied edge includes corners. When two axes prescribe a corner value, safe construction requires exact f32 equality; derivative/RHS corner combinations use the value where HWSCRT requires both."
            }),
        ),
        (
            "fishpack-cartesian-2d-manifest.json",
            json!({
                "schema_id":"slatec.safe-fishpack.cartesian-2d.manifest",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "feature":"fishpack-cartesian-2d",
                "safe_items":["UniformAxis","Grid2","AxisBoundary","CartesianHelmholtz2d::new","CartesianHelmholtz2d::solve","CartesianPdeSolution","PdeError"],
                "driver":{"routine":"HWSCRT","source":"fishfft/hwscrt.f","sha256":HWSCRT_SHA256,"precision":"f32"},
                "mathematical_contract":"five-point finite difference for u_xx+u_yy+lambda*u=f; M and N are panel counts; solution includes both endpoints",
                "grid_layout":"Grid2 is owned x-fast row-major values[y*nx+x], identical to contiguous Fortran F(M+1,N+1) with IDIMF=M+1",
                "workspace":"4*(N+1)+(13+floor(log2(N+1)))*(M+1), checked and private",
                "singularity":"lambda=0 with each axis periodic or double-Neumann; HWSCRT subtracts PERTRB and returns a solution defined up to an additive constant",
                "positive_lambda":"IERROR=6 is returned as an attempted-solve warning status because HWSCRT may not have a solution",
                "runtime_policy":"SerializedGlobal via the existing process-wide native lock",
                "deferred":["3D Cartesian","GENBUN public API","BLKTRI public API","complex FISHPACK","generic PDE abstractions"],
                "outside_this_cartesian_m1_scope":["POIS3D has its own structured-system facade","cylindrical and polar drivers have the fishpack-cylindrical-polar facade","unit-sphere and axisymmetric spherical drivers have the fishpack-spherical facade"]
            }),
        ),
        (
            "fishpack-cartesian-2d-native-state.json",
            json!({
                "schema_id":"slatec.safe-fishpack.cartesian-2d.native-state",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "columns":["scope","finding","evidence","policy"],
                "records":[
                    ["all 11 selected sources","no COMMON, SAVE, DATA, BLOCK DATA, XERROR/XERMSG, Fortran I/O, or callback path found","focused source scan and GNU MinGW source-build object inspection","retain existing process-global lock until reentrancy is positively qualified"],
                    ["HWSCRT","F and W are caller-owned mutable arrays; BDA/BDB/BDC/BDD are input only","selected HWSCRT source body","private owned Rust buffers, no escaping aliases"],
                    ["provider runtime","Fortran runtime thread safety remains unqualified","source backend policy","SerializedGlobal"]
                ]
            }),
        ),
        (
            "fishpack-cartesian-2d-concurrency.json",
            json!({
                "schema_id":"slatec.safe-fishpack.cartesian-2d.concurrency",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "columns":["safe_function","class","lock_scope","claim"],
                "records":[["CartesianHelmholtz2d::solve","SerializedGlobal","crate::runtime::lock_native around the complete FFI call","Rust-owned problems may run from many threads but native calls never overlap"]]
            }),
        ),
        (
            "fishpack-cartesian-2d-wrapper-index.json",
            json!({
                "schema_id":"slatec.safe-fishpack.cartesian-2d.wrapper-index",
                "schema_version":"1.0.0",
                "snapshot_id":snapshot,
                "raw_ffi_profile":PROFILE,
                "columns":["safe_path","raw_routine","raw_symbol","precision","mathematical_model","storage_policy","runtime_policy"],
                "records":wrappers
            }),
        ),
        (
            "fishpack-cartesian-2d-validation-summary.md",
            json!(format!(
                "# Safe Cartesian FISHPACK M1\n\n- Snapshot: `{snapshot}`.\n- Selected driver: `HWSCRT` from `fishfft/hwscrt.f` ({HWSCRT_SHA256}), not the materially different standalone FISHPACK revision.\n- Closure: 11 source units, including the existing canonical `SCOPY`; no FFTPACK transform root, XERROR, callback, or Fortran-I/O dependency is reachable.\n- Native ABI: `hwscrt_`, single precision and 32-bit GNU Fortran `INTEGER`; the safe facade owns all edge buffers, RHS/solution storage, and workspace.\n- Validation: asymmetric Dirichlet Poisson, negative-coefficient Helmholtz, codes 2/4 mixed boundaries, periodic code 0, singular Neumann code 3 with `PERTRB`, positive-coefficient `IERROR=6`, layout, and concurrent-call serialization all execute in the source-backend native test.\n- Runtime: calls remain process-globally serialized. The focused state audit found no mutable FISHPACK source storage, but no parallel-native-execution claim is made.\n"
            )),
        ),
    ];

    fs::create_dir_all(output_dir)?;
    let mut bytes = Vec::new();
    for (name, value) in files {
        let encoded = if name.ends_with(".md") {
            value
                .as_str()
                .ok_or_else(|| CorpusError::Verification("summary must be text".to_owned()))?
                .as_bytes()
                .to_vec()
        } else {
            serde_json::to_vec(&value)?
        };
        fs::write(output_dir.join(name), &encoded)?;
        bytes.extend_from_slice(&encoded);
    }
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        candidate_count: 3,
        wrapper_count: 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_deterministic_and_keeps_the_narrow_closure() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        for name in [
            "fishpack-cartesian-2d-candidate-index.json",
            "fishpack-cartesian-2d-source-closure.json",
            "fishpack-cartesian-2d-raw-contracts.json",
            "fishpack-cartesian-2d-boundary-contract.json",
            "fishpack-cartesian-2d-manifest.json",
            "fishpack-cartesian-2d-native-state.json",
            "fishpack-cartesian-2d-concurrency.json",
            "fishpack-cartesian-2d-wrapper-index.json",
            "fishpack-cartesian-2d-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
        let closure: Value = serde_json::from_slice(
            &fs::read(
                first
                    .path()
                    .join("fishpack-cartesian-2d-source-closure.json"),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(closure["closure_size"], 11);
        assert_eq!(closure["roots"], json!(["HWSCRT"]));
    }
}
