//! Deterministic metadata for the reviewed residual-only DASSL sessions.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Returns focused native-state evidence for each public DASSL operation.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure: Value = serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/dassl-source-closure.json"),
    )?)?;
    let source_ids = closure["source_ids"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("DASSL closure lacks source ids".to_owned()))?
        .iter()
        .filter_map(Value::as_str)
        .map(|id| format!("{id}.o"))
        .collect::<Vec<_>>();
    let source_paths = closure["sources"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("DASSL closure lacks sources".to_owned()))?
        .iter()
        .filter_map(|source| source["path"].as_str())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    Ok([("f32", "SDASSL"), ("f64", "DDASSL")]
        .into_iter()
        .map(|(precision, routine)| {
            json!({
                "safe_function":format!("slatec::dassl::DaeSession::<{precision}, F>::advance_to"),
                "native_entry_points":[routine],
                "feature":"dassl",
                "effective_native_families":["dassl"],
                "entry_object":format!("dassl-source-{}.o", routine.to_ascii_lowercase()),
                "object_closure":source_ids,
                "source_closure":source_paths,
                "saved_mutable_locals":["DDAINI/SDAINI DATA MAXIT,MJAC,DAMP", "DDASTP/SDASTP DATA MAXIT,XRATE"],
                "common_blocks":[],
                "xerror_state":["XERROR J4SAVE/XERSVE process-global state"],
                "fortran_io":["formatted internal WRITE diagnostics only; no OPEN/CLOSE paging or retained file protocol"],
                "callback_state":["thread-local residual context installed only under process runtime lock"],
                "writable_symbols":["focused source audit identifies saved DATA locals; object-level evidence is required for any future concurrency relaxation"],
                "source_object_unresolved":["broad native-origin audit status is recorded separately"],
                "external_undefined_symbols":["GNU Fortran runtime and BLAS provider contract"],
                "feature_closure_mismatch":false,
                "native_routine_reentrancy":"SerializedGlobal",
                "rust_api_concurrency":"session ownership permits movement only when F is Send; &mut self prevents concurrent same-session use",
                "provider_runtime_thread_safety":"reviewed and external backends remain serialized; no DASSL parallel claim",
                "provider_unknowns":["Fortran runtime and linked BLAS provider are not independently parallel-qualified"],
                "remaining_blockers":["SAVE/DATA mutable locals", "process-global XERROR", "callback dispatch", "provider/runtime qualification"]
            })
        })
        .collect())
}

/// Concise result of DASSL metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus identity.
    pub snapshot_id: String,
    /// Stable hash of emitted bytes.
    pub semantic_hash: String,
    /// Precision-specific public wrappers.
    pub wrapper_count: usize,
}

/// Generates DASSL audit and safe-wrapper metadata from reviewed explicit facts.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-dassl-metadata requires --offline".to_owned(),
        ));
    }
    let selected: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = selected["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let closure: Value = serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/dassl-source-closure.json"),
    )?)?;
    let sources = closure["sources"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("DASSL closure lacks sources".to_owned()))?;
    let inventory = sources
        .iter()
        .map(|source| {
            let path = source["path"].as_str().unwrap_or("unknown");
            let name = path
                .rsplit('/')
                .next()
                .unwrap_or("unknown")
                .trim_end_matches(".f")
                .to_ascii_uppercase();
            let role = match name.as_str() {
                "SDASSL" | "DDASSL" => "public_driver",
                "SDAINI" | "DDAINI" | "SDAJAC" | "DDAJAC" | "SDASTP" | "DDASTP" | "SDATRP"
                | "DDATRP" | "SDAWTS" | "DDAWTS" | "SDANRM" | "DDANRM" | "SDASLV" | "DDASLV" => {
                    "DASSL_subsidiary"
                }
                _ => "reviewed_transitive_dependency",
            };
            json!([name, source["id"], path, source["sha256"], role])
        })
        .collect::<Vec<_>>();
    let wrappers = vec![
        json!([
            "slatec::dassl::DaeSession::<f32, F>::advance_to",
            "SDASSL",
            "sdassl_",
            "f32",
            "G(t,y,y_prime)=0",
            "owned_residual_only_dense_finite_difference_session",
            "SerializedGlobal"
        ]),
        json!([
            "slatec::dassl::DaeSession::<f64, F>::advance_to",
            "DDASSL",
            "ddassl_",
            "f64",
            "G(t,y,y_prime)=0",
            "owned_residual_only_dense_finite_difference_session",
            "SerializedGlobal"
        ]),
    ];
    let files = [
        (
            "dassl-routine-inventory.json",
            json!({"schema_id":"slatec.safe-dassl.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["routine","source_id","source_path","sha256","role"],"records":inventory}),
        ),
        (
            "dassl-source-closure.json",
            json!({"schema_id":"slatec.safe-dassl.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"roots":["SDASSL","DDASSL"],"source_ids":closure["source_ids"],"dense_mode":"INFO(6)=0; DDAJAC still contains hard-referenced dense and banded LINPACK paths","narrow_link_probe":{"example":"link_dassl","required_symbol":"ddassl_","status":"passed","excluded_root_symbols":["ddriv3_","sdriv3_","dsplp_","splp_"]},"excluded":["user_Jacobian","banded_public_mode","events","complex_DASSL","other_ODE_drivers"]}),
        ),
        (
            "dassl-callback-contract.json",
            json!({"schema_id":"slatec.safe-dassl.callback-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","native_residual","rust_residual","IRES_mapping","containment"],"records":[["SDASSL","RES(T,Y,YPRIME,DELTA,IRES,RPAR,IPAR)","FnMut(f32,&[f32],&[f32],&mut[f32])->Result<ResidualAction,E>","Continue=0; RecoverableFailure=-1; FatalFailure_or_Rust_error_or_panic=-2","catch_unwind; exact slices; nonfinite rejection; thread-local context under lock"],["DDASSL","RES(T,Y,YPRIME,DELTA,IRES,RPAR,IPAR)","FnMut(f64,&[f64],&[f64],&mut[f64])->Result<ResidualAction,E>","Continue=0; RecoverableFailure=-1; FatalFailure_or_Rust_error_or_panic=-2","catch_unwind; exact slices; nonfinite rejection; thread-local context under lock"]]}),
        ),
        (
            "dassl-workspace.json",
            json!({"schema_id":"slatec.safe-dassl.workspace","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["array_or_region","formula_or_index","restricted_mode","ownership_or_meaning"],"records":[["RWORK","40+(MAXORD+4)*NEQ+NEQ^2","dense internally-differenced Jacobian; MAXORD in 1..=5","owned opaque continuation history"],["IWORK","20+NEQ","dense internally-differenced Jacobian","owned opaque continuation history"],["IWORK(11)","zero_based_10","all restricted modes","cumulative_internal_steps"],["IWORK(12)","zero_based_11","all restricted modes","cumulative_residual_evaluations"],["IWORK(13)","zero_based_12","all restricted modes","cumulative_iteration_matrix_evaluations"],["IWORK(14)","zero_based_13","all restricted modes","cumulative_error_test_failures"],["IWORK(15)","zero_based_14","all restricted modes","cumulative_convergence_failures"]]}),
        ),
        (
            "dassl-tolerance-contract.json",
            json!({"schema_id":"slatec.safe-dassl.tolerance-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_mode","INFO(2)","native_storage","error_weight"],"records":[["Scalar",0,"RTOL(1),ATOL(1)","RTOL*abs(Y(i))+ATOL"],["Vector",1,"RTOL(i),ATOL(i)","RTOL(i)*abs(Y(i))+ATOL(i)"]]}),
        ),
        (
            "dassl-option-audit.json",
            json!({"schema_id":"slatec.safe-dassl.option-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_option","INFO_or_workspace","decision"],"records":[["stop_time","INFO(4)=1; RWORK(1)","deferred: requested-output mode rejects TOUT beyond TSTOP, so it has no useful distinct safe advancement semantics"],["maximum_step","INFO(7)=1; RWORK(2)","exposed"],["initial_step","INFO(8)=1; RWORK(3)","exposed"],["maximum_order","INFO(9)=1; IWORK(3); 1..=5","exposed"],["intermediate_output","INFO(3)","deferred"],["user_Jacobian","INFO(5)=1","prohibited"],["banded_Jacobian","INFO(6)=1","prohibited"],["consistent_initial_derivative","INFO(11)=1","deferred"]]}),
        ),
        (
            "dassl-status-map.json",
            json!({"schema_id":"slatec.safe-dassl.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["IDID","safe_mapping","state_meaning"],"records":[[3,"ReachedTarget","successful requested output time"],[2,"ReachedStopTime","successful TSTOP return"],[-1,"ExcessiveWork","recoverable with INFO(1)=1"],[-2,"AccuracyAdjusted","recoverable with adjusted tolerances and INFO(1)=1"],[-3,"ErrorWeightFailure","tolerances must be corrected"],[-6,"RepeatedErrorTestFailure","terminal in initial safe API"],[-7,"RepeatedConvergenceFailure","terminal"],[-8,"SingularIterationMatrix","terminal"],[-9,"RepeatedErrorOrConvergenceFailure","terminal"],[-10,"RepeatedRecoverableResidualFailure","terminal"],[-11,"FatalResidualFailure","terminal"],[-12,"InitialDerivativeFailure","not exposed; terminal contract violation"],[-33,"InvalidNativeInput","native contract violation"]]}),
        ),
        (
            "dassl-recovery-map.json",
            json!({"schema_id":"slatec.safe-dassl.recovery-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["status","session_lifecycle","allowed_action"],"records":[["ReachedTarget","Ready","continue same direction"],["ReachedStopTime","Ready","defensive mapping; not reachable while INFO(4)=0"],["ExcessiveWork","Recoverable","retry advance_to"],["AccuracyAdjusted","NeedsToleranceUpdate","set_tolerance then retry"],["ErrorWeightFailure","NeedsToleranceUpdate","set_tolerance then retry"],["terminal native or callback failure","Failed","restart_from only"]]}),
        ),
        (
            "dassl-native-state.json",
            json!({"schema_id":"slatec.safe-dassl.native-state","schema_version":"1.0.0","snapshot_id":snapshot,"broad_native_origin_retry":{"status":"interrupted_without_terminal_report","command":"SLATEC_SOURCE_CACHE=<verified-cache> SLATEC_GFORTRAN=<reviewed-gnu-mingw> cargo run -p slatec-tools -- generate-native-origin-audit --output-dir generated/safe-api","stage_reached":"slatec-tools compiled; native-origin worker and GNU Fortran child observed","exit_status":"unavailable_from_terminal_bridge","generated_partial_artifacts":"none observed; pre-existing audit artifacts unchanged","fallback":"focused hash-verified DASSL source-closure and source-build inspection completed; no concurrency classification was relaxed"},"object_evidence":{"archive":"source-build libslatec_family.a","members":["dassl-source-ddaini.o","dassl-source-ddastp.o","dassl-source-sdaini.o","dassl-source-sdastp.o"],"writable_local_symbols":["damp.2","maxit.1","mjac.0","maxit.0","xrate.1"],"tool":"x86_64-w64-mingw32-gcc-nm -a"},"columns":["routine","origin","storage","effect","classification"],"records":[["SDAINI/DDAINI","DATA","MAXIT,MJAC,DAMP","saved mutable initialization constants","SerializedGlobal"],["SDASTP/DDASTP","DATA","MAXIT,XRATE","saved mutable step-control state","SerializedGlobal"],["XERROR","SAVE/runtime","J4SAVE/XERSVE","process-global error configuration","SerializedGlobal"],["residual_dispatch","Rust TLS","ACTIVE_DASSL_CONTEXT","callback context guarded by runtime lock","SerializedGlobal"]]}),
        ),
        (
            "dassl-concurrency.json",
            json!({"schema_id":"slatec.safe-dassl.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_function","native_routine","class","lock_scope","callback","XERROR","reason"],"records":[["slatec::dassl::DaeSession::<f32,F>::advance_to","SDASSL","SerializedGlobal","process_global","thread_local under lock","scoped restoration","SAVE/DATA, XERROR, callback dispatch, and provider/runtime uncertainty"],["slatec::dassl::DaeSession::<f64,F>::advance_to","DDASSL","SerializedGlobal","process_global","thread_local under lock","scoped restoration","SAVE/DATA, XERROR, callback dispatch, and provider/runtime uncertainty"]]}),
        ),
        (
            "dassl-deferred.json",
            json!({"schema_id":"slatec.safe-dassl.deferred","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["item","reason"],"records":[["user_Jacobian","callback ABI and dense matrix ownership need a separate reviewed milestone"],["banded_or_sparse_Jacobian","layout and workspace mode require separate checked views"],["events_and_roots","DASSL driver scope has no reviewed root callback in this family"],["consistent_initial_conditions","INFO(11) semantics require a separate lifecycle and mathematical contract"],["mass_matrix_convenience","residual formulation remains primary without a new matrix API"],["complex_DAEs","Fortran complex ABI is deferred"],["parallel_native_execution","saved DATA, XERROR, callback dispatch, and provider/runtime state require global serialization"],["ecosystem_adapters","core API deliberately uses slices and Vec without dependencies"]]}),
        ),
        (
            "dassl-wrapper-index.json",
            json!({"schema_id":"slatec.safe-dassl.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","raw_symbol","precision","mathematical_model","session_policy","runtime_policy"],"records":wrappers}),
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
        "# Safe residual-only DASSL sessions\n\n- Snapshot: `{snapshot}`.\n- Drivers: `SDASSL` (`f32`) and `DDASSL` (`f64`) for real index-1 `G(t,y,y')=0` problems.\n- Scope: owned continuation sessions, scalar/vector tolerances, internally differenced dense iteration matrices, and requested-output advancement only. User Jacobians, banded storage, root finding, consistent-initial-condition calculation, and complex drivers remain deferred.\n- Callback: `IRES=0` continues, `-1` requests documented recoverable residual failure, and `-2` ends the native operation. Rust errors, panics, malformed calls, and non-finite residuals are contained without unwinding across Fortran.\n- Workspace: `RWORK=40+(MAXORD+4)*NEQ+NEQ^2`; `IWORK=20+NEQ`; `MAXORD` is 1 through 5.\n- Runtime: all calls remain `SerializedGlobal`; `SDAINI/DDAINI` and `SDASTP/DDASTP` use saved DATA state and reachable XERROR is process-global. There is no DASSL external-file protocol in the selected closure.\n- Native-origin audit: focused DASSL closure inspection is complete; broad audit retry status is reported independently and never weakens serialization.\n"
    );
    fs::write(
        output_dir.join("dassl-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        wrapper_count: 2,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        assert_eq!(
            fs::read(first.path().join("dassl-status-map.json")).unwrap(),
            fs::read(second.path().join("dassl-status-map.json")).unwrap()
        );
    }
}
