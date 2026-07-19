//! Deterministic metadata for reviewed LINPACK general-band diagnostics.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Focused native-state projections for general-band operations.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure = closure()?;
    let objects = closure["source_ids"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("banded closure lacks source ids".to_owned()))?
        .iter()
        .filter_map(Value::as_str)
        .map(|id| format!("{id}.o"))
        .collect::<Vec<_>>();
    Ok([
        (
            "slatec::linear_algebra::banded::BandMatrixRef::factorize_with_condition_estimate",
            "SGBCO/DGBCO",
        ),
        (
            "slatec::linear_algebra::banded::BandLu32::scaled_determinant",
            "SGBDI",
        ),
        (
            "slatec::linear_algebra::banded::BandLu64::scaled_determinant",
            "DGBDI",
        ),
    ]
    .into_iter()
    .map(|(safe_function, routine)| {
        json!({
            "safe_function":safe_function,
            "native_entry_points":routine.split('/').collect::<Vec<_>>(),
            "feature":"banded-linear-systems",
            "effective_native_families":["banded-linear-systems"],
            "entry_object": if routine.starts_with("SGBDI") { "selected-source-df39792535ee35b6.o" } else if routine.starts_with("DGBDI") { "selected-source-78cf067dd0cb80c7.o" } else if routine.starts_with("SGBCO") { "selected-source-750bded799b3956a.o" } else { "ode-source-sgbfa.o" },
            "object_closure":objects,
            "source_closure":["SGBFA/DGBFA", "SGBSL/DGBSL", "SGBCO/DGBCO", "SGBDI/DGBDI", "SASUM/DASUM", "SAXPY/DAXPY", "SSCAL/DSCAL", "SDOT/DDOT", "ISAMAX/IDAMAX"],
            "saved_mutable_locals":Vec::<&str>::new(),
            "common_blocks":[],
            "xerror_state":[],
            "fortran_io":[],
            "callback_state":["none"],
            "writable_symbols":["focused source audit: no mutable COMMON, SAVE, DATA, BLOCK DATA, callback, XERROR, or I/O state in the numerical closure"],
            "source_object_unresolved":[],
            "external_undefined_symbols":[],
            "feature_closure_mismatch":false,
            "current_class":"SerializedGlobal",
            "best_possible_class_from_slatec_source":"BackendDependent",
            "native_routine_reentrancy":"no persistent numerical state found; runtime gate intentionally remains exclusive",
            "rust_api_concurrency":"borrowed matrix input and owned factors are movable; all native calls use the process-global runtime lock",
            "provider_runtime_thread_safety":"unqualified external and system provider/runtime contracts",
            "provider_unknowns":["compiler and external provider runtime contract is not qualified for concurrent native entry"],
            "remaining_blockers":["repository-wide process-global native serialization policy", "provider/runtime qualification"]
        })
    })
    .collect())
}

/// Summary of deterministic banded-diagnostics metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus snapshot identifier.
    pub snapshot_id: String,
    /// Stable hash over emitted files.
    pub semantic_hash: String,
}

/// Generates complete reviewed metadata for the safe general-band diagnostics.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-banded-metadata requires --offline".to_owned(),
        ));
    }
    let manifest: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = manifest["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let closure = closure()?;
    let inventory = json!({
        "schema_id":"slatec.safe-banded.condition-routine-inventory",
        "schema_version":"1.0.0",
        "snapshot_id":snapshot,
        "raw_ffi_profile":PROFILE,
        "columns":["routine","precision","role","factorization","storage","pivot","outputs","workspace","dependencies","state","safe_operation"],
        "records":[
            ["SGBCO","f32","user_callable","factors private expanded storage","LDA>=2*ML+MU+1; original diagonals begin at ML+1 (one-based)","output IPVT, one-based","RCOND reciprocal estimated 1-norm condition","Z(N)","SGBFA,SASUM,SAXPY,SDOT,SSCAL","no COMMON/SAVE/DATA/I/O/XERROR","BandMatrixRef::factorize_with_condition_estimate"],
            ["DGBCO","f64","user_callable","factors private expanded storage","LDA>=2*ML+MU+1; original diagonals begin at ML+1 (one-based)","output IPVT, one-based","RCOND reciprocal estimated 1-norm condition","Z(N)","DGBFA,DASUM,DAXPY,DDOT,DSCAL","no COMMON/SAVE/DATA/I/O/XERROR","BandMatrixRef::factorize_with_condition_estimate"],
            ["SGBDI","f32","user_callable","uses existing SGBFA/SGBCO factors","same expanded LINPACK storage","input IPVT, one-based","DET(1),DET(2): mantissa and decimal exponent","none","none","no COMMON/SAVE/DATA/I/O/XERROR","BandLu32::scaled_determinant"],
            ["DGBDI","f64","user_callable","uses existing DGBFA/DGBCO factors","same expanded LINPACK storage","input IPVT, one-based","DET(1),DET(2): mantissa and decimal exponent","none","none","no COMMON/SAVE/DATA/I/O/XERROR","BandLu64::scaled_determinant"]
        ],
        "sources":closure["sources"]
    });
    let files = [
        ("banded-condition-routine-inventory.json", inventory),
        (
            "banded-condition-contract.json",
            json!({"schema_id":"slatec.safe-banded.condition-contract","schema_version":"1.0.0","snapshot_id":snapshot,"form":"factorize then estimate reciprocal condition in the original matrix 1-norm","columns":["property","native_contract","safe_policy"],"records":[["factorization","SGBCO/DGBCO call SGBFA/DGBFA internally","return owned BandLu only after an exact zero U diagonal is rejected"],["norm","maximum absolute column sum of original compact matrix","documented as reciprocal 1-norm condition estimate"],["rcond zero","exact singularity or estimate underflow","exact singularity is BandError::Singular; successful zero is preserved"],["workspace","Z length N","fallible private allocation with checked N conversion"],["mutation","ABD and IPVT overwritten","only private expanded copy is passed to Fortran"]]}),
        ),
        (
            "banded-determinant-contract.json",
            json!({"schema_id":"slatec.safe-banded.determinant-contract","schema_version":"1.0.0","snapshot_id":snapshot,"formula":"det(A) = mantissa * 10^exponent10","columns":["property","native_contract","safe_policy"],"records":[["input","factors and one-based pivots from GBFA/GBCO","private compatible BandLu factors only"],["normalization","nonzero 1 <= abs(DET(1)) < 10","typed ScaledDeterminant preserves native pair"],["sign","each IPVT(I) != I negates the mantissa","native pivot sign is preserved"],["zero","DET(1)=0; exponent is mathematically irrelevant","mantissa zero remains explicit; no direct unscaled value API"],["mutation","SGBDI/DGBDI only read ABD and IPVT","safe method takes &self"]]}),
        ),
        (
            "banded-diagnostics-source-closure.json",
            json!({"schema_id":"slatec.safe-banded.diagnostics-source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"manifest":"crates/slatec-src/metadata/banded-linear-systems-source-closure.json","roots":["SGBFA","DGBFA","SGBSL","DGBSL","SGBCO","DGBCO","SGBDI","DGBDI"],"source_ids":closure["source_ids"],"sources":closure["sources"],"excluded":["dense LINPACK","packed LINPACK","symmetric-band","sparse","eigenvalue","XERROR","Fortran I/O"]}),
        ),
        (
            "banded-diagnostics-native-state.json",
            json!({"schema_id":"slatec.safe-banded.diagnostics-native-state","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["closure_part","finding","effect","policy"],"records":[["SGBFA/DGBFA/SGBSL/DGBSL/SGBCO/DGBCO/SGBDI/DGBDI and BLAS dependencies","no COMMON, SAVE, DATA, BLOCK DATA, callback, XERROR, or Fortran I/O found in focused source audit","numerical storage is caller-provided or automatic","global serialization retained because backend/runtime concurrency remains unqualified"],["compiled source backend","no reviewed writable numerical global expected; object inspection remains an explicit source-build validation","no parallel claim","SerializedGlobal"]]}),
        ),
        (
            "banded-diagnostics-concurrency.json",
            json!({"schema_id":"slatec.safe-banded.diagnostics-concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_operation","backend_profile","class","lock","reason"],"records":[["factorize/factorize_with_condition_estimate/scaled_determinant","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","repository policy and unqualified provider/runtime concurrency"],["same","external_or_system_backend","BackendDependent","process_global_runtime_lock","external provider/runtime concurrency unknown"]]}),
        ),
        (
            "banded-diagnostics-wrapper-index.json",
            json!({"schema_id":"slatec.safe-banded.diagnostics-wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_path","raw_routine","precision","storage","result","concurrency"],"records":[["BandMatrixRef::factorize_with_condition_estimate","SGBCO/DGBCO","f32/f64","compact copied to expanded LU","(BandLu, ReciprocalCondition)","SerializedGlobal"],["BandLu32::scaled_determinant","SGBDI","f32","private expanded LU","ScaledDeterminant<f32>","SerializedGlobal"],["BandLu64::scaled_determinant","DGBDI","f64","private expanded LU","ScaledDeterminant<f64>","SerializedGlobal"]]}),
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
        "# Safe LINPACK general-band diagnostics\n\n- Snapshot: `{snapshot}`.\n- `SGBCO`/`DGBCO` are included because they factor the private expanded copy and estimate reciprocal 1-norm conditioning. Exact singular factors are rejected; an otherwise valid zero estimate is retained.\n- `SGBDI`/`DGBDI` read existing private factors and return `mantissa * 10^exponent10`; no unscaled determinant is made primary.\n- The closure contains only general-band LINPACK and its genuine BLAS dependencies. Focused source audit found no mutable numerical global, XERROR, callback, or file path, but the public API remains `SerializedGlobal` under the repository runtime policy.\n"
    );
    fs::write(
        output_dir.join("banded-diagnostics-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
    })
}

fn closure() -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/banded-linear-systems-source-closure.json"),
    )?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_deterministic() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        for name in [
            "banded-condition-routine-inventory.json",
            "banded-condition-contract.json",
            "banded-determinant-contract.json",
            "banded-diagnostics-source-closure.json",
            "banded-diagnostics-native-state.json",
            "banded-diagnostics-concurrency.json",
            "banded-diagnostics-wrapper-index.json",
            "banded-diagnostics-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
