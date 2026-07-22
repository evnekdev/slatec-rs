//! Deterministic metadata for the reviewed safe SDRIVE expert sessions.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Summary returned after generating SDRIVE safe-API metadata.
#[derive(Debug)]
pub struct ResultSummary {
    /// Shared selected-corpus snapshot identity.
    pub snapshot_id: String,
    /// Stable hash of every generated file.
    pub semantic_hash: String,
    /// Number of reviewed precision-specific wrappers.
    pub wrapper_count: usize,
}

/// Generates compact, offline-only metadata for `SDRIV3` and `DDRIV3`.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-ode-sdrive-metadata requires --offline".to_owned(),
        ));
    }
    let selected: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = selected["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let candidates = vec![
        json!([
            "DDRIV3",
            "ddriv3_",
            "f64",
            "full-provider-acd5eb7788a2521c",
            "src/ddriv3.f",
            "expert_driver",
            "explicit_real_initial_value_problem",
            "reviewed"
        ]),
        json!([
            "SDRIV3",
            "sdriv3_",
            "f32",
            "full-provider-8499468854722fca",
            "src/sdriv3.f",
            "expert_driver",
            "explicit_real_initial_value_problem",
            "reviewed"
        ]),
    ];
    let wrappers = vec![
        json!([
            "slatec::ode::OdeSession::<f32, F, E>::integrate_to",
            "SDRIV3",
            "sdriv3_",
            "full-provider-8499468854722fca",
            "f32",
            "y_prime_equals_f_t_y",
            "owned_non_cloneable_session; RHS_only; NTASK_3",
            "SerializedGlobal",
            "reviewed"
        ]),
        json!([
            "slatec::ode::OdeSession::<f64, F, E>::integrate_to",
            "DDRIV3",
            "ddriv3_",
            "full-provider-acd5eb7788a2521c",
            "f64",
            "y_prime_equals_f_t_y",
            "owned_non_cloneable_session; RHS_only; NTASK_3",
            "SerializedGlobal",
            "reviewed"
        ]),
    ];
    let files = [
        (
            "ode-sdrive-candidate-index.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.candidate-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["raw_routine","raw_symbol","precision","program_unit_id","source_path","role","mathematical_model","review_state"],"records":candidates}),
        ),
        (
            "ode-sdrive-wrapper-index.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","mathematical_model","session_policy","runtime_policy","review_state"],"records":wrappers}),
        ),
        (
            "ode-sdrive-status-map.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_nstate","safe_result","session_state","meaning"],"records":[[2,"ReachedTarget","Ready","exact_target_return"],[3,"ExcessWork","Recoverable","internal_step_limit; current_time_and_state_are_meaningful"],[4,"ToleranceAdjusted","Recoverable","native_relaxed_EPS; current_time_and_state_are_meaningful"],[6,"OdeError::Callback_or_native_contract_violation","Failed","F_set_N_to_zero"],[5,"not_exposed","not_reachable","roots_disabled_with_NROOT_zero"],[7, "not_exposed","not_reachable","root_callback_disabled"],[8,"not_exposed","not_reachable","Jacobian_callback_disabled"],[9,"not_exposed","not_reachable","mass_callback_disabled"],[10,"not_exposed","not_reachable","USERS_callback_disabled"],[11,"not_exposed","not_reachable","interpolation_mode_disabled"],[12,"OdeError::NativeContractViolation","Failed","unreviewed_terminal_native_failure"]]}),
        ),
        (
            "ode-sdrive-callback-contract.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.callback-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","native_callback","rust_callback","termination","containment"],"records":[["SDRIV3","F(N,T,Y,YDOT)","FnMut(f32,&[f32],&mut[f32])->Result<(),E>","set_N_to_zero","require_session_N_and_nonaliasing_buffers; catch_unwind; store_error_or_panic; reject_nonfinite_output; no_unwind_through_Fortran"],["DDRIV3","F(N,T,Y,YDOT)","FnMut(f64,&[f64],&mut[f64])->Result<(),E>","set_N_to_zero","require_session_N_and_nonaliasing_buffers; catch_unwind; store_error_or_panic; reject_nonfinite_output; no_unwind_through_Fortran"]]}),
        ),
        (
            "ode-sdrive-tolerance-map.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.tolerance-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["rust_input","native_control","weight"],"records":[["relative","EPS","scalar_relative_accuracy"],["OdeTolerance::Scalar","IERROR=3; EWT(1)","max(abs(Y(i)),EWT(1))"],["OdeTolerance::Vector","IERROR=4; EWT(i)","max(abs(Y(i)),EWT(i))"]]}),
        ),
        (
            "ode-sdrive-options.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.options","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["rust_option","native_control","validation","scope"],"records":[["method=Adams","MINT=1","maximum_order_in_1_to_12","RHS_only"],["method=Bdf","MINT=2","maximum_order_in_1_to_5","RHS_only_functional_iteration"],["maximum_steps","MXSTEP","positive_native_INTEGER","per_call"],["maximum_step","HMAX","positive_finite","per_call"],["integration_mode","NTASK=3","fixed","exact_target_only"],["roots","NROOT=0","fixed","events_deferred"],["Jacobian","MITER=0","fixed","Jacobian_deferred"],["mass_matrix","IMPL=0","fixed","mass_matrices_deferred"]]}),
        ),
        (
            "ode-sdrive-workspace.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.workspace","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["array","formula","restricted_controls","ownership"],"records":[["WORK","(MXORD+4)*N+250","NROOT=0; MITER=0; IMPL=0","owned_opaque_session_history"],["IWORK","50","NROOT=0; MITER=0","owned_opaque_session_history"],["MXORD","12_Adams_or_5_BDF_default","checked_usize_to_INTEGER","validated_option"]]}),
        ),
        (
            "ode-sdrive-session-state.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.session-state","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["state","entry","allowed_next_operation","transition"],"records":[["Ready","new_or_NSTATE_2","same_direction_integrate_to","native_call"],["Recoverable","NSTATE_3_or_4","same_direction_integrate_to","native_call"],["Failed","callback_error; callback_panic; nonfinite_derivative; native_contract_violation","none","terminal"]]}),
        ),
        (
            "ode-sdrive-concurrency.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.concurrency","schema_version":"1.2.0","snapshot_id":snapshot,"columns":["safe_function","native_routine","class","lock_scope","callback","XERROR","nested_policy","mutable_global_state_records","Send","Sync","reason"],"records":[["slatec::ode::OdeSession::<f32, F, E>::integrate_to","SDRIV3","SerializedGlobal","process_global","thread_local_context_under_runtime_lock","scoped_XGETF_XSETF_restoration","reject_active_context",["native-origin-source-closure-complete","native-origin-compiler-storage-model","native-origin-writable-symbol-inspection","native-origin-common-block-audit","xerror-global-state-audit","native-ode-serialization-validation","sdriv3-ddriv3-callback-context","sdstp-data-ier"],"conditional_on_callback","no","saved_IER_XERROR_thread_local_callback_dispatch_and_provider_runtime_unknowns_require_global_serialization"],["slatec::ode::OdeSession::<f64, F, E>::integrate_to","DDRIV3","SerializedGlobal","process_global","thread_local_context_under_runtime_lock","scoped_XGETF_XSETF_restoration","reject_active_context",["native-origin-source-closure-complete","native-origin-compiler-storage-model","native-origin-writable-symbol-inspection","native-origin-common-block-audit","xerror-global-state-audit","native-ode-serialization-validation","sdriv3-ddriv3-callback-context","ddstp-data-ier"],"conditional_on_callback","no","saved_IER_XERROR_thread_local_callback_dispatch_and_provider_runtime_unknowns_require_global_serialization"]]}),
        ),
        (
            "ode-sdrive-source-closure.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.source-closure","schema_version":"1.1.0","snapshot_id":snapshot,"roots":["SDRIV3","DDRIV3"],"genuine_subsidiaries":["SDSTP","DDSTP","SDNTL","DDNTL","SDPSC","DDPSC","SDPST","DDPST","SDCOR","DDCOR","SDSCL","DDSCL","SDCST","DDCST","SDNTP","DDNTP","SDZRO","DDZRO","LINPACK_dense_banded","BLAS_level1","machine_constants","XERROR"],"unavoidable_co_retained_symbols":["SDNTP","DDNTP","SDZRO","DDZRO","dense_and_banded_LINPACK_paths"],"narrow_link_probe":{"example":"link_ode_sdrive","required_symbol":"ddriv3_","excluded_root_symbols":["sdassl_","ddassl_","derkf_","dderkf_","deabm_","ddeabm_","debdf_","ddebdf_","splp_","dsplp_"],"status":"passed"},"excluded":["DASSL","DEPAC","complex_SDRIVE","root_API","Jacobian_API","mass_matrix_API"]}),
        ),
        (
            "ode-sdrive-deferred.json",
            json!({"schema_id":"slatec.safe-ode-sdrive.deferred","schema_version":"1.1.0","snapshot_id":snapshot,"columns":["item","reason"],"records":[["SDRIV1_DDRIV1","covered_by_Driv1Session_outside_SDRIV3_OdeSession_scope"],["SDRIV2_DDRIV2","covered_by_Driv2Session_with_indexed_events_outside_SDRIV3_OdeSession_scope"],["roots_and_events","SDRIV3_NROOT_is_fixed_to_zero; reviewed_DRIV2_sessions_cover_their_own_root_protocol"],["analytic_and_finite_difference_Jacobians","MITER_is_fixed_to_zero"],["banded_Jacobians","matrix_view_and_layout_audit_deferred"],["mass_matrices_and_implicit_ODEs","IMPL_is_fixed_to_zero"],["SDASSL_DDASSL","DAE_consistency_and_residual_protocol_out_of_scope"],["dense_output_and_interpolation","NTASK_1_and_NSTATE_11_deferred"],["complex_SDRIVE","CDRIV1_CDRIV2_covered_by_owned_sessions; CDRIV3_remains_unresolved"],["session_clone_or_serialization","opaque_WORK_history_is_backend_specific"],["lock_free_or_family_lock_concurrency","runtime_is_process_serialized"],["ecosystem_adapters","core_API_uses_slices_and_Vec_without_dependencies"]]}),
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
        "# Safe SDRIVE expert ODE sessions\n\n- Snapshot: `{snapshot}`.\n- Wrappers: `OdeSession<f32, _, _>` over `SDRIV3` and `OdeSession<f64, _, _>` over `DDRIV3`.\n- Scope: real explicit `y'(t) = f(t, y)` only; roots, Jacobians, mass matrices, DAEs, and interpolation are not exposed.\n- Callback: a panic, error, malformed native callback request (wrong `N`, null, non-finite time, or aliased buffers), or non-finite derivative sets native `N = 0`; no Rust panic crosses Fortran.\n- State: session-owned `WORK`, `IWORK`, `NSTATE`, time, and state support same-direction continuation only.\n- Runtime: native calls are globally serialized and XERROR control is restored by scope. `SDSTP` and `DDSTP` each declare initialized `IER` local storage (`DATA IER /.FALSE./`), which GNU MinGW emits as local writable `ier.0` `.bss`; reset paths do not make that state reentrant.\n- I/O: the selected drivers have formatted `WRITE` diagnostics on error paths but no `OPEN`, `CLOSE`, or retained external-file protocol. Typed input validation and scoped XERROR handling avoid those paths for valid safe calls; serialization remains mandatory.\n- Workspace: `WORK = (MXORD + 4) * N + 250`, `IWORK = 50` for `NROOT=0`, `MITER=0`, and `IMPL=0`.\n- Native execution validation: complete against the explicit hash-verified cache, including `DDCOR` and `DGBFA`; analytic, continuation, callback failure/panic/non-finite, tolerance, excess-work, nested-call, serialization, XERROR-restoration, and cross-session contamination tests pass. The `link_ode_sdrive` probe retains `ddriv3_` and no DASSL, DEPAC, or LP root symbol. No source or native artifact is committed.\n"
    );
    fs::write(
        output_dir.join("ode-sdrive-validation-summary.md"),
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
        for name in [
            "ode-sdrive-candidate-index.json",
            "ode-sdrive-wrapper-index.json",
            "ode-sdrive-status-map.json",
            "ode-sdrive-callback-contract.json",
            "ode-sdrive-tolerance-map.json",
            "ode-sdrive-options.json",
            "ode-sdrive-workspace.json",
            "ode-sdrive-session-state.json",
            "ode-sdrive-concurrency.json",
            "ode-sdrive-source-closure.json",
            "ode-sdrive-deferred.json",
            "ode-sdrive-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
