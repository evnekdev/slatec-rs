//! Deterministic metadata for the reviewed structured FISHPACK `POIS3D` facade.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const POIS3D_SHA256: &str = "9daf0cd2c9eab106f9b508f426003a98719d21ed29b4fde0f224300d8e88da78";

fn closure() -> Result<Value> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../slatec-src/metadata");
    Ok(serde_json::from_slice(&fs::read(
        root.join("fishpack-pois3d-source-closure.json"),
    )?)?)
}

fn source_records(closure: &Value) -> Result<Vec<Value>> {
    let records = closure["sources"].as_array().ok_or_else(|| {
        CorpusError::Verification("POIS3D closure lacks source records".to_owned())
    })?;
    if records.len() != 32
        || records
            .iter()
            .find(|source| source["id"] == "selected-source-5e4436007d71a538")
            .and_then(|source| source["sha256"].as_str())
            != Some(POIS3D_SHA256)
    {
        return Err(CorpusError::Verification(
            "POIS3D closure no longer matches the reviewed direct call graph".to_owned(),
        ));
    }
    Ok(records.clone())
}

/// Returns native-state evidence for the structured three-dimensional facade.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure = closure()?;
    let records = source_records(&closure)?;
    let sources = records
        .iter()
        .filter_map(|source| source["path"].as_str())
        .collect::<Vec<_>>();
    let objects = records
        .iter()
        .filter_map(|source| source["id"].as_str())
        .map(|id| format!("{id}.o"))
        .collect::<Vec<_>>();
    Ok(vec![json!({
        "safe_function":"slatec::differential_equations::pde::Pois3dProblem::solve",
        "native_entry_points":["POIS3D","POS3D1","TRIDQ"],
        "feature":"fishpack-pois3d",
        "effective_native_families":["fishpack-pois3d","fftpack-real"],
        "entry_object":"selected-source-5e4436007d71a538.o",
        "object_closure":objects,
        "source_closure":sources,
        "saved_mutable_locals":["POIS3D local SAVE array is a temporary coefficient-restoration buffer, not a SAVE statement or persistent mathematical state"],
        "common_blocks":[],
        "xerror_state":[],
        "fortran_io":[],
        "callback_state":["none"],
        "writable_symbols":["caller-owned A/B/C are temporarily modified and restored for NPEROD=0", "caller-owned F and W"],
        "source_object_unresolved":[],
        "external_undefined_symbols":[],
        "feature_closure_mismatch":false,
        "current_class":"SerializedGlobal",
        "best_possible_class_from_slatec_source":"BackendDependent",
        "native_routine_reentrancy":"no COMMON, SAVE statement, DATA, XERROR, I/O, or callback path found in the selected 32-unit closure; the Fortran runtime remains unqualified",
        "rust_api_concurrency":"owned grids and coefficient vectors are movable; every native solve uses the process-global runtime lock",
        "provider_runtime_thread_safety":"unqualified external and system provider/runtime contracts",
        "provider_unknowns":["compiler and external provider runtime contract is not qualified for concurrent native entry"],
        "remaining_blockers":["repository-wide process-global native serialization policy", "provider/runtime qualification"]
    })])
}

/// Summary of deterministic POIS3D metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus identity.
    pub snapshot_id: String,
    /// Stable hash of all generated artifacts.
    pub semantic_hash: String,
    /// Reviewed candidate count.
    pub candidate_count: usize,
    /// Safe wrapper count.
    pub wrapper_count: usize,
}

/// Generates the M2 artifacts without inferring a general 3D PDE surface.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-fishpack-pois3d-metadata requires --offline".to_owned(),
        ));
    }
    let selected: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = selected["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let closure = closure()?;
    let records = source_records(&closure)?;
    let source_ids = closure["source_ids"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let files = [
        (
            "fishpack-pois3d-candidate-index.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.candidate-index", "schema_version":"1.0.0", "snapshot_id":snapshot,
                "columns":["routine","selected","provider","path","sha256","precision","decision"],
                "records":[
                    ["POIS3D",true,"SLATEC relocated fishfft subset","fishfft/pois3d.f",POIS3D_SHA256,"f32","selected structured block-tridiagonal driver"],
                    ["POIS3D",false,"current Netlib fishfft provider","fishfft/pois3d.f",POIS3D_SHA256,"f32","same verified body at acquisition; selected cached source remains ABI authority"],
                    ["HW3CRT",false,"SLATEC relocated fishfft subset","fishfft/hw3crt.f",null,"f32","different six-face Cartesian driver; deliberately outside M2"],
                    ["POS3D1",false,"SLATEC main-src subset","src/pos3d1.f","bf6782774aa2a7f0ce461052904c0b310588a16da4e6a657fbb931dcfd811599","f32","transitive subsidiary only"]
                ]
            }),
        ),
        (
            "fishpack-pois3d-source-closure.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.source-closure", "schema_version":"1.0.0", "snapshot_id":snapshot,
                "manifest":"crates/slatec-src/metadata/fishpack-pois3d-source-closure.json", "roots":["POIS3D"], "source_ids":source_ids,
                "columns":["source_id","subset","path","sha256"], "records":records,
                "closure_size":32,
                "fftpack_overlap":{"family":"fftpack-real","source_units":28,"relationship":"exact POIS3D call graph reuses 28 canonical real-FFTPACK source identities; BTreeSet selection prevents duplicate objects when the broader real profile is also selected"},
                "required_symbols":["pois3d_","pos3d1_","tridq_","pimach_","rffti_","rfftf_","rfftb_","sinti_","sint_","costi_","cost_","sinqi_","sinqf_","sinqb_","cosqi_","cosqf_","cosqb_"],
                "unresolved_external_dependencies":[], "duplicate_definition_risk":"none in the composed POIS3D plus real-FFTPACK closure",
                "deferred":["arbitrary six-face boundary data","general Cartesian 3D Helmholtz facade","public GENBUN","public BLKTRI","other FISHPACK coordinate systems","generic PDE traits"]
            }),
        ),
        (
            "fishpack-pois3d-raw-contracts.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.raw-contracts", "schema_version":"1.0.0", "snapshot_id":snapshot,
                "routine":"POIS3D", "symbol":"pois3d_", "calling_convention":"Fortran external subroutine through the GNU MinGW C ABI", "fortran_integer":"i32", "scalar":"f32",
                "columns":["position","argument","direction","native_extent_or_meaning","safe_mapping"],
                "records":[
                    [1,"LPEROD","in","first-axis ghost mode 0..4","TransverseBoundary private encoding"], [2,"L","in","first unknown count >=3","Grid3::nx"], [3,"C1","in","first unscaled second-difference coefficient","Pois3dProblem::c1"],
                    [4,"MPEROD","in","second-axis ghost mode 0..4","TransverseBoundary private encoding"], [5,"M","in","second unknown count >=3","Grid3::ny"], [6,"C2","in","second unscaled second-difference coefficient","Pois3dProblem::c2"],
                    [7,"NPEROD","in","0 cyclic constants or 1 noncyclic tridiagonal","ThirdAxisOperator private encoding"], [8,"N","in","third unknown count >=3","Grid3::nz"],
                    [9,"A","in/out","N lower coefficients; temporarily overwritten and restored when cyclic","owned ThirdAxisOperator buffer"], [10,"B","in/out","N diagonal coefficients; temporarily overwritten and restored when cyclic","owned ThirdAxisOperator buffer"], [11,"C","in/out","N upper coefficients; temporarily overwritten and restored when cyclic","owned ThirdAxisOperator buffer"],
                    [12,"LDIMF","in","private first F dimension >=L","Grid3::nx"], [13,"MDIMF","in","private second F dimension >=M","Grid3::ny"], [14,"F","in/out","F(LDIMF,MDIMF,N), RHS overwritten by solution","owned x-fast Grid3"], [15,"IERROR","out","0 success; 1..10 invalid native contract","checked error or unexpected native status"], [16,"W","work","documented lower bound workspace","private checked allocation"]
                ]
            }),
        ),
        (
            "fishpack-pois3d-boundary-contract.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.boundary-contract", "schema_version":"1.0.0", "snapshot_id":snapshot,
                "columns":["code","safe_variant","lower_ghost","upper_ghost"],
                "records":[[0,"Periodic","X(0)=X(Q)","X(Q+1)=X(1)"],[1,"ZeroBoth","X(0)=0","X(Q+1)=0"],[2,"ZeroLowerReflectUpper","X(0)=0","X(Q+1)=X(Q-1)"],[3,"ReflectBoth","X(0)=X(2)","X(Q+1)=X(Q-1)"],[4,"ReflectLowerZeroUpper","X(0)=X(2)","X(Q+1)=0"]]
            }),
        ),
        (
            "fishpack-pois3d-third-axis-contract.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.third-axis-contract", "schema_version":"1.0.0", "snapshot_id":snapshot,
                "columns":["native_mode","safe_model","restriction","indexing"],
                "records":[[0,"CyclicAxisCoefficients","A(K)=C(K)=C(1), B(K)=B(1), and safe off-diagonal is nonzero","k-1 and k+1 wrap modulo N"],[1,"TridiagonalAxisCoefficients","A(1)=0 and C(N)=0","nonwrapping tridiagonal operator"]]
            }),
        ),
        (
            "fishpack-pois3d-manifest.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.manifest", "schema_version":"1.0.0", "snapshot_id":snapshot, "feature":"fishpack-pois3d",
                "safe_items":["Grid3","TransverseBoundary","CyclicAxisCoefficients","TridiagonalAxisCoefficients","ThirdAxisOperator","Pois3dProblem::new","Pois3dProblem::solve","Pois3dError"],
                "driver":{"routine":"POIS3D","source":"fishfft/pois3d.f","sha256":POIS3D_SHA256,"precision":"f32"},
                "mathematical_contract":"structured three-dimensional block-tridiagonal discrete system; no independent grid spacings or arbitrary face-value arrays", "grid_layout":"Grid3 values[z*nx*ny+y*nx+x], equivalent to contiguous Fortran F(L,M,N)",
                "workspace":"30 + L + M + 2*N + max(L,M,N) + 7*(floor((L+1)/2)+floor((M+1)/2)); checked and private", "native_statuses":{"0":"success","1":"invalid LPEROD","2":"L<3","3":"invalid MPEROD","4":"M<3","5":"invalid NPEROD","6":"N<3","7":"LDIMF<L","8":"MDIMF<M","9":"invalid cyclic constants","10":"invalid noncyclic endpoints"},
                "runtime_policy":"SerializedGlobal via the existing process-wide native lock", "deferred":["arbitrary six-face 3D boundary data","general Cartesian 3D Helmholtz","unstructured or variable-coefficient PDEs"]
            }),
        ),
        (
            "fishpack-pois3d-native-state.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.native-state", "schema_version":"1.0.0", "snapshot_id":snapshot,
                "columns":["scope","finding","policy"],
                "records":[["32-unit exact POIS3D closure","no COMMON, SAVE statement, DATA, XERROR/XERMSG, Fortran I/O, or callback path found; POIS3D local array named SAVE is a temporary restoration buffer","retain process-global lock"],["POIS3D","A/B/C are temporarily mutated and restored for cyclic mode; F and W are caller-owned mutable arrays","safe facade owns all mutable buffers"],["provider runtime","Fortran runtime remains unqualified for simultaneous foreign entry","SerializedGlobal"]]
            }),
        ),
        (
            "fishpack-pois3d-concurrency.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.concurrency", "schema_version":"1.0.0", "snapshot_id":snapshot,
                "columns":["safe_function","class","lock_scope","claim"], "records":[["Pois3dProblem::solve","SerializedGlobal","crate::runtime::lock_native around the complete FFI call","owned problems may be moved across threads but native solves never overlap"]]
            }),
        ),
        (
            "fishpack-pois3d-wrapper-index.json",
            json!({
                "schema_id":"slatec.safe-fishpack.pois3d.wrapper-index", "schema_version":"1.0.0", "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
                "columns":["safe_path","raw_routine","raw_symbol","precision","mathematical_model","storage_policy","runtime_policy"],
                "records":[["slatec::differential_equations::pde::Pois3dProblem::solve","POIS3D","pois3d_","f32","structured three-dimensional block-tridiagonal discrete system","owned Grid3 and coefficient buffers","SerializedGlobal"]]
            }),
        ),
        (
            "fishpack-pois3d-validation-summary.md",
            json!(format!(
                "# Safe structured FISHPACK POIS3D M2\n\n- Snapshot: `{snapshot}`.\n- Selected driver: `POIS3D` from `fishfft/pois3d.f` ({POIS3D_SHA256}); the current direct Netlib provider matched this selected body at acquisition.\n- Exact closure: 32 source units: `POIS3D`, `POS3D1`, `TRIDQ`, `PIMACH`, and 28 reused real-FFTPACK units.\n- Contract: an owned, checked structured block-tridiagonal system, not arbitrary six-face 3D boundary data.\n- Validation: independent dense oracle, manufactured cyclic and noncyclic systems, all transverse ghost rules, raw `IERROR=9/10`, x-fast layout, and concurrent native-call serialization.\n- Runtime: no persistent FISHPACK state was found, but native calls remain process-globally serialized.\n"
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
        candidate_count: 4,
        wrapper_count: 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_deterministic_and_keeps_the_composed_closure() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        let closure: Value = serde_json::from_slice(
            &fs::read(first.path().join("fishpack-pois3d-source-closure.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(closure["closure_size"], 32);
        assert_eq!(closure["roots"], json!(["POIS3D"]));
    }
}
