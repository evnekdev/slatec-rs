//! Deterministic inventory of reviewed SLATEC ODE candidates.
//!
//! This module is audit evidence only. It does not emit bindings, source
//! closures, Cargo features, or public ODE API records.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

#[derive(Clone, Copy)]
struct Candidate {
    name: &'static str,
    pair: &'static str,
    precision: &'static str,
    family: &'static str,
    role: &'static str,
    model: &'static str,
    callbacks: &'static str,
    continuation: &'static str,
    state: &'static str,
    io: &'static str,
    status: &'static str,
    workspace: &'static str,
    disposition: &'static str,
    reason: &'static str,
}

macro_rules! candidate {
    ($name:literal,$pair:literal,$precision:literal,$family:literal,$role:literal,$model:literal,$callbacks:literal,$continuation:literal,$state:literal,$io:literal,$status:literal,$workspace:literal,$disposition:literal,$reason:literal) => {
        Candidate {
            name: $name,
            pair: $pair,
            precision: $precision,
            family: $family,
            role: $role,
            model: $model,
            callbacks: $callbacks,
            continuation: $continuation,
            state: $state,
            io: $io,
            status: $status,
            workspace: $workspace,
            disposition: $disposition,
            reason: $reason,
        }
    };
}

// Corpus-first inventory: every record is checked against the committed FFI
// inventory before emission. It covers every public ODE driver selected from
// the catalogue and the interpolation/step/root helpers necessary to assess
// their actual contracts.
const CANDIDATES: &[Candidate] = &[
    candidate!(
        "DERKF",
        "DDERKF",
        "f32",
        "depac_runge_kutta_fehlberg",
        "easy_driver",
        "y_prime_equals_f_t_y",
        "F(t,y,yp,rpar,ipar); no documented callback abort",
        "INFO(1)=0_start_or_1_continue; work arrays persist",
        "no_COMMON_or_SAVE_in_driver; XERROR",
        "none",
        "IDID=1_step,2_target,-1..-5_recoverable,-33_terminal",
        "LRW>=33+7*N documented; implementation checks 30+7*N; LIW>=34",
        "deferred",
        "no_callback_termination_protocol"
    ),
    candidate!(
        "DDERKF",
        "DERKF",
        "f64",
        "depac_runge_kutta_fehlberg",
        "easy_driver",
        "y_prime_equals_f_t_y",
        "DF(t,y,yp,rpar,ipar); no documented callback abort",
        "INFO(1)=0_start_or_1_continue; work arrays persist",
        "no_COMMON_or_SAVE_in_driver; XERROR",
        "none",
        "IDID=1_step,2_target,-1..-5_recoverable,-33_terminal",
        "LRW>=33+7*N documented; implementation checks 30+7*N; LIW>=34",
        "deferred",
        "no_callback_termination_protocol"
    ),
    candidate!(
        "DEABM",
        "DDEABM",
        "f32",
        "depac_adams",
        "easy_driver",
        "y_prime_equals_f_t_y",
        "F(t,y,yp,rpar,ipar); no documented callback abort",
        "INFO continuation; work arrays persist",
        "no_COMMON_or_SAVE_in_driver; XERROR",
        "none",
        "IDID target_step_and_recoverable_DEPAC_codes",
        "LRW>=130+21*N; LIW>=51",
        "deferred",
        "no_callback_termination_protocol"
    ),
    candidate!(
        "DDEABM",
        "DEABM",
        "f64",
        "depac_adams",
        "easy_driver",
        "y_prime_equals_f_t_y",
        "DF(t,y,yp,rpar,ipar); no documented callback abort",
        "INFO continuation; work arrays persist",
        "no_COMMON_or_SAVE_in_driver; XERROR",
        "none",
        "IDID target_step_and_recoverable_DEPAC_codes",
        "LRW>=130+21*N; LIW>=51",
        "deferred",
        "no_callback_termination_protocol"
    ),
    candidate!(
        "DEBDF",
        "DDEBDF",
        "f32",
        "depac_bdf",
        "expert_driver",
        "stiff_y_prime_equals_f_t_y",
        "F plus optional JAC; callback termination not documented",
        "INFO continuation; work arrays plus COMMON_DEBDF1 persist",
        "COMMON_DEBDF1; XERROR",
        "none",
        "IDID target_step_and_recoverable_DEPAC_codes",
        "dense:250+10*N+N*N; banded:250+10*N+(2*ML+MU+1)*N; LIW>=56+N",
        "deferred",
        "process_global_COMMON_and_no_safe_callback_abort"
    ),
    candidate!(
        "DDEBDF",
        "DEBDF",
        "f64",
        "depac_bdf",
        "expert_driver",
        "stiff_y_prime_equals_f_t_y",
        "DF plus optional DJAC; callback termination not documented",
        "INFO continuation; work arrays plus COMMON_DDEBD1 persist",
        "COMMON_DDEBD1; XERROR",
        "none",
        "IDID target_step_and_recoverable_DEPAC_codes",
        "dense:250+10*N+N*N; banded:250+10*N+(2*ML+MU+1)*N; LIW>=56+N",
        "deferred",
        "process_global_COMMON_and_no_safe_callback_abort"
    ),
    candidate!(
        "SDRIV1",
        "DDRIV1",
        "f32",
        "sdrive_convenience",
        "easy_driver",
        "y_prime_equals_f_t_y",
        "F(N,t,y,ydot); set callback N=0 to terminate",
        "MSTATE and WORK/IWORK persist",
        "XERROR; driver state in caller arrays",
        "none",
        "MSTATE=2_success,3_excess_work,4_tolerance_adjusted,5_callback_abort,6_interpolated,7_failure",
        "LENW>=N*N+11*N+300; IWORK is private to the native adapter",
        "reviewed_safe_session",
        "owned_safe_session_with_scoped_Rust_callback"
    ),
    candidate!(
        "DDRIV1",
        "SDRIV1",
        "f64",
        "sdrive_convenience",
        "easy_driver",
        "y_prime_equals_f_t_y",
        "F(N,t,y,ydot); set callback N=0 to terminate",
        "MSTATE and WORK/IWORK persist",
        "XERROR; driver state in caller arrays",
        "none",
        "MSTATE=2_success,3_excess_work,4_tolerance_adjusted,5_callback_abort,6_interpolated,7_failure",
        "LENW>=N*N+11*N+300; IWORK is private to the native adapter",
        "reviewed_safe_session",
        "owned_safe_session_with_scoped_Rust_callback"
    ),
    candidate!(
        "SDRIV2",
        "DDRIV2",
        "f32",
        "sdrive_roots",
        "continuation_driver",
        "y_prime_equals_f_t_y_with_sign_change_roots",
        "F and G; set callback N=0 to terminate",
        "MSTATE and WORK/IWORK persist",
        "XERROR; driver state in caller arrays",
        "none; sample program only prints",
        "MSTATE=2_success,3_excess_work,4_tolerance_adjusted,5_root,6_RHS_abort,7_root_abort,8_interpolated,9_failure",
        "LENW>=16*N+2*NROOT+250 (Adams), N*N+10*N+2*NROOT+250 (Gear), or N*N+17*N+2*NROOT+250 (Automatic); LENIW>=50 (Adams) or N+50",
        "reviewed_safe_session",
        "owned_safe_session_with_indexed_zero_based_roots"
    ),
    candidate!(
        "DDRIV2",
        "SDRIV2",
        "f64",
        "sdrive_roots",
        "continuation_driver",
        "y_prime_equals_f_t_y_with_sign_change_roots",
        "F and G; set callback N=0 to terminate",
        "MSTATE and WORK/IWORK persist",
        "XERROR; driver state in caller arrays",
        "none; sample program only prints",
        "MSTATE=2_success,3_excess_work,4_tolerance_adjusted,5_root,6_RHS_abort,7_root_abort,8_interpolated,9_failure",
        "LENW>=16*N+2*NROOT+250 (Adams), N*N+10*N+2*NROOT+250 (Gear), or N*N+17*N+2*NROOT+250 (Automatic); LENIW>=50 (Adams) or N+50",
        "reviewed_safe_session",
        "owned_safe_session_with_indexed_zero_based_roots"
    ),
    candidate!(
        "SDRIV3",
        "DDRIV3",
        "f32",
        "sdrive_expert",
        "expert_driver",
        "y_prime_equals_f_t_y; optional_nonsingular_or_hybrid_mass_matrix",
        "F/JACOBN/FA/USERS/G receive mutable N and may set N=0 to stop",
        "NSTATE=1_start; caller-owned WORK/IWORK opaque continuation state",
        "no_COMMON_or_SAVE_in_driver; XERROR",
        "none",
        "NSTATE=2_success,3_work,4_tolerance,5_root,6..10_callback_abort,11_interpolated,12_failure",
        "documented_mode_dependent; SDRIV1 gives N*N+11*N+300 baseline; exact SDRIV3 formula selected by MINT/MITER/IMPL",
        "recommended",
        "only_f32_f64_RHS_only_first_scope_with_owned_session_and_panic_contained_trampolines"
    ),
    candidate!(
        "DDRIV3",
        "SDRIV3",
        "f64",
        "sdrive_expert",
        "expert_driver",
        "y_prime_equals_f_t_y; optional_nonsingular_or_hybrid_mass_matrix",
        "F/JACOBN/FA/USERS/G receive mutable N and may set N=0 to stop",
        "NSTATE=1_start; caller-owned WORK/IWORK opaque continuation state",
        "no_COMMON_or_SAVE_in_driver; XERROR",
        "none",
        "NSTATE=2_success,3_work,4_tolerance,5_root,6..10_callback_abort,11_interpolated,12_failure",
        "documented_mode_dependent; DDRIV1 gives N*N+11*N+300 baseline; exact DDRIV3 formula selected by MINT/MITER/IMPL",
        "recommended",
        "only_f32_f64_RHS_only_first_scope_with_owned_session_and_panic_contained_trampolines"
    ),
    candidate!(
        "CDRIV1",
        "none",
        "complex",
        "sdrive_complex",
        "easy_driver",
        "complex_y_prime_equals_f_t_y",
        "complex F; set callback N=0 to terminate",
        "MSTATE and caller work arrays persist",
        "XERROR",
        "none",
        "MSTATE=2_success,3_excess_work,4_tolerance_adjusted,5_callback_abort,6_interpolated,7_failure",
        "LENW>=N*N+11*N+300",
        "reviewed_safe_session",
        "owned_safe_session_with_explicit_Complex32_conversion"
    ),
    candidate!(
        "CDRIV2",
        "none",
        "complex",
        "sdrive_complex",
        "continuation_driver",
        "complex_y_prime_equals_f_t_y_with_roots",
        "complex F/G; set callback N=0 to terminate",
        "MSTATE and caller work arrays persist",
        "XERROR",
        "none",
        "MSTATE=2_success,3_excess_work,4_tolerance_adjusted,5_root,6_RHS_abort,7_root_abort,8_interpolated,9_failure",
        "LENW>=16*N+2*NROOT+250 (Adams), N*N+10*N+2*NROOT+250 (Gear), or N*N+17*N+2*NROOT+250 (Automatic); LENIW>=50 (Adams) or N+50",
        "reviewed_safe_session",
        "owned_safe_session_with_explicit_Complex32_conversion_and_indexed_zero_based_roots"
    ),
    candidate!(
        "CDRIV3",
        "none",
        "complex",
        "sdrive_complex",
        "expert_driver",
        "complex_y_prime_equals_f_t_y",
        "complex F/JACOBN/FA/USERS/G callbacks",
        "NSTATE and caller work arrays persist",
        "XERROR",
        "none",
        "same_SDRIV3_status_model",
        "complex_work_formula_in_source",
        "deferred",
        "future_real_f32_f64_family_excludes_complex_API"
    ),
    candidate!(
        "SDASSL",
        "DDASSL",
        "f32",
        "dassl_dae",
        "expert_driver",
        "residual_G_t_y_yprime_equals_zero",
        "RES(t,y,yprime,cj,delta,ires,rpar,ipar); optional JAC; IRES controls termination",
        "INFO(1)=0_start_or_1_continue; opaque work arrays persist",
        "no_COMMON_or_SAVE_in_driver_after_880387_revision; XERROR",
        "none",
        "IDID=1_target,2_step,3_tstop,-1..-12_failures",
        "dense:40+(MAXORD+4)*N+N*N; banded:40+(MAXORD+4)*N+(2*ML+MU+1)*N; LIW>=20+N",
        "deferred",
        "DAE_consistency_Jacobian_and_failure_recovery_need_separate_audit"
    ),
    candidate!(
        "DDASSL",
        "SDASSL",
        "f64",
        "dassl_dae",
        "expert_driver",
        "residual_G_t_y_yprime_equals_zero",
        "RES(t,y,yprime,cj,delta,ires,rpar,ipar); optional JAC; IRES controls termination",
        "INFO(1)=0_start_or_1_continue; opaque work arrays persist",
        "no_COMMON_or_SAVE_in_driver_after_880387_revision; XERROR",
        "none",
        "IDID=1_target,2_step,3_tstop,-1..-12_failures",
        "dense:40+(MAXORD+4)*N+N*N; banded:40+(MAXORD+4)*N+(2*ML+MU+1)*N; LIW>=20+N",
        "deferred",
        "DAE_consistency_Jacobian_and_failure_recovery_need_separate_audit"
    ),
    candidate!(
        "DERKFS",
        "DRKFS",
        "f32",
        "depac_runge_kutta_fehlberg",
        "support_routine",
        "single_RKF45_step",
        "F callback",
        "caller_work_arrays",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "caller_scratch",
        "not_public",
        "driver_subsidiary"
    ),
    candidate!(
        "DRKFS",
        "DERKFS",
        "f64",
        "depac_runge_kutta_fehlberg",
        "support_routine",
        "single_RKF45_step",
        "DF callback",
        "caller_work_arrays",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "caller_scratch",
        "not_public",
        "driver_subsidiary"
    ),
    candidate!(
        "DES",
        "DDES",
        "f32",
        "depac_adams",
        "support_routine",
        "Adams_step_and_interpolation",
        "F callback",
        "caller_work_arrays",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "caller_scratch",
        "not_public",
        "driver_subsidiary"
    ),
    candidate!(
        "DDES",
        "DES",
        "f64",
        "depac_adams",
        "support_routine",
        "Adams_step_and_interpolation",
        "DF callback",
        "caller_work_arrays",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "caller_scratch",
        "not_public",
        "driver_subsidiary"
    ),
    candidate!(
        "LSOD",
        "DLSOD",
        "f32",
        "depac_bdf",
        "support_routine",
        "BDF_step_and_linear_solve",
        "F/JAC callbacks",
        "COMMON_DEBDF1_persistent",
        "COMMON_DEBDF1",
        "none",
        "internal_return_codes",
        "driver_owned_history_and_matrix",
        "not_public",
        "global_BDF_subsidiary"
    ),
    candidate!(
        "DLSOD",
        "LSOD",
        "f64",
        "depac_bdf",
        "support_routine",
        "BDF_step_and_linear_solve",
        "DF/DJAC callbacks",
        "COMMON_DDEBD1_persistent",
        "COMMON_DDEBD1",
        "none",
        "internal_return_codes",
        "driver_owned_history_and_matrix",
        "not_public",
        "global_BDF_subsidiary"
    ),
    candidate!(
        "INTYD",
        "DINTYD",
        "f32",
        "depac_bdf",
        "interpolation_routine",
        "Nordsieck_dense_output_derivatives",
        "none",
        "requires_live_DEBDF1_COMMON_history",
        "COMMON_DEBDF1",
        "none",
        "IFLAG=0_or_negative_for_order_or_interval",
        "YH supplied but bounds from COMMON",
        "deferred",
        "global_history_makes_independent_or_concurrent_use_unsafe"
    ),
    candidate!(
        "DINTYD",
        "INTYD",
        "f64",
        "depac_bdf",
        "interpolation_routine",
        "Nordsieck_dense_output_derivatives",
        "none",
        "requires_live_DDEBD1_COMMON_history",
        "COMMON_DDEBD1",
        "none",
        "IFLAG=0_or_negative_for_order_or_interval",
        "YH supplied but bounds from COMMON",
        "deferred",
        "global_history_makes_independent_or_concurrent_use_unsafe"
    ),
    candidate!(
        "SDNTP",
        "DDNTP",
        "f32",
        "sdrive_expert",
        "support_routine",
        "sdrive_dense_interpolation",
        "none",
        "requires_SDRIV3_work_history",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "live_WORK_history",
        "not_public",
        "session_bound_subsidiary"
    ),
    candidate!(
        "DDNTP",
        "SDNTP",
        "f64",
        "sdrive_expert",
        "support_routine",
        "sdrive_dense_interpolation",
        "none",
        "requires_DDRIV3_work_history",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "live_WORK_history",
        "not_public",
        "session_bound_subsidiary"
    ),
    candidate!(
        "SDSTP",
        "DDSTP",
        "f32",
        "sdrive_expert",
        "support_routine",
        "sdrive_stepper",
        "F/JACOBN/FA/USERS callbacks",
        "caller_WORK_state",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "mode_dependent_WORK",
        "not_public",
        "expert_driver_subsidiary"
    ),
    candidate!(
        "DDSTP",
        "SDSTP",
        "f64",
        "sdrive_expert",
        "support_routine",
        "sdrive_stepper",
        "DF/DJACOBN/DFA/DUSERS callbacks",
        "caller_WORK_state",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "mode_dependent_WORK",
        "not_public",
        "expert_driver_subsidiary"
    ),
    candidate!(
        "SDZRO",
        "DDZRO",
        "f32",
        "sdrive_expert",
        "support_routine",
        "sign_change_root_location",
        "G callback",
        "caller_WORK_state",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "root_scratch_in_WORK",
        "not_public",
        "expert_driver_subsidiary"
    ),
    candidate!(
        "DDZRO",
        "SDZRO",
        "f64",
        "sdrive_expert",
        "support_routine",
        "sign_change_root_location",
        "DG callback",
        "caller_WORK_state",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "root_scratch_in_WORK",
        "not_public",
        "expert_driver_subsidiary"
    ),
    candidate!(
        "SDAINI",
        "DDAINI",
        "f32",
        "dassl_dae",
        "support_routine",
        "DASSL_initialization",
        "RES/JAC callbacks",
        "caller_WORK_state",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "DASSL_WORK",
        "not_public",
        "DAE_driver_subsidiary"
    ),
    candidate!(
        "DDAINI",
        "SDAINI",
        "f64",
        "dassl_dae",
        "support_routine",
        "DASSL_initialization",
        "RES/JAC callbacks",
        "caller_WORK_state",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "DASSL_WORK",
        "not_public",
        "DAE_driver_subsidiary"
    ),
    candidate!(
        "SDASTP",
        "DDASTP",
        "f32",
        "dassl_dae",
        "support_routine",
        "DASSL_BDF_step",
        "RES/JAC callbacks",
        "caller_WORK_state",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "DASSL_WORK",
        "not_public",
        "DAE_driver_subsidiary"
    ),
    candidate!(
        "DDASTP",
        "SDASTP",
        "f64",
        "dassl_dae",
        "support_routine",
        "DASSL_BDF_step",
        "RES/JAC callbacks",
        "caller_WORK_state",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "DASSL_WORK",
        "not_public",
        "DAE_driver_subsidiary"
    ),
    candidate!(
        "SDATRP",
        "DDATRP",
        "f32",
        "dassl_dae",
        "interpolation_routine",
        "DASSL_history_interpolation",
        "none",
        "requires_live_DASSL_work_history",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "DASSL_WORK",
        "not_public",
        "session_bound_subsidiary"
    ),
    candidate!(
        "DDATRP",
        "SDATRP",
        "f64",
        "dassl_dae",
        "interpolation_routine",
        "DASSL_history_interpolation",
        "none",
        "requires_live_DASSL_work_history",
        "no_COMMON_or_SAVE",
        "none",
        "internal_return_codes",
        "DASSL_WORK",
        "not_public",
        "session_bound_subsidiary"
    ),
];

/// Summary returned after generating the ODE audit.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus snapshot.
    pub snapshot_id: String,
    /// Stable hash of all emitted audit records.
    pub semantic_hash: String,
    /// Number of inventoried candidates.
    pub candidate_count: usize,
}

/// Generates deterministic, audit-only ODE metadata.
pub fn generate(
    ffi_dir: &Path,
    selected_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-ode-audit requires --offline".to_owned(),
        ));
    }
    let inventory = read(&ffi_dir.join("interface-inventory.json"))?;
    let selected = read(&selected_dir.join("selected-providers.json"))?;
    let snapshot_id = string(&selected, "snapshot_id")?;
    let ffi = ffi_records(&inventory)?;
    validate(&ffi)?;
    let mut records = CANDIDATES
        .iter()
        .map(|c| {
            let row = ffi.get(c.name).expect("validated candidate");
            json!([
                c.name,
                c.pair,
                c.precision,
                row.0,
                row.1,
                row.2,
                c.family,
                c.role,
                c.model,
                c.callbacks,
                c.continuation,
                c.disposition,
                c.reason
            ])
        })
        .collect::<Vec<_>>();
    records.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    let callback_rows = CANDIDATES
        .iter()
        .filter(|c| c.callbacks != "none")
        .map(|c| {
            json!([
                c.name,
                c.callbacks,
                callback_safety(c),
                if c.disposition == "recommended" {
                    "future_trampoline_catches_panics_sets_N_zero_and_rethrows_after_return"
                } else {
                    "not_public_or_deferred"
                }
            ])
        })
        .collect::<Vec<_>>();
    let continuation_rows = CANDIDATES
        .iter()
        .filter(|c| c.continuation != "caller_work_arrays")
        .map(|c| {
            json!([
                c.name,
                c.continuation,
                session_recommendation(c),
                "target_may_change_only_under_documented_continuation_state"
            ])
        })
        .collect::<Vec<_>>();
    let global_rows = CANDIDATES
        .iter()
        .map(|c| json!([c.name, c.state, c.io, global_class(c)]))
        .collect::<Vec<_>>();
    let status_rows = CANDIDATES
        .iter()
        .map(|c| {
            json!([
                c.name,
                c.status,
                if c.disposition == "recommended" {
                    "preserve_all_NSTATE_codes"
                } else {
                    "audit_only"
                }
            ])
        })
        .collect::<Vec<_>>();
    let workspace_rows = CANDIDATES
        .iter()
        .map(|c| {
            json!([
                c.name,
                c.workspace,
                if c.workspace.contains("mode_dependent") {
                    "unresolved_until_future_restricted_scope"
                } else {
                    "source_prologue_or_driver_contract"
                }
            ])
        })
        .collect::<Vec<_>>();
    let outputs = vec![
        (
            "ode-candidate-index.json",
            json!({"schema_id":"slatec.ode.candidate-index","schema_version":"1.0.0","snapshot_id":snapshot_id,"raw_ffi_profile":PROFILE,"discovery":"selected-corpus public/provider inventory plus source-prologue ODE, DAE, interpolation, root, and stepper audit","columns":["routine","precision_pair","precision","program_unit_id","source_subset","source_path","family","role","mathematical_form","callback_protocol","continuation","disposition","reason"],"records":records}),
        ),
        (
            "ode-family-index.json",
            json!({"schema_id":"slatec.ode.family-index","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["family","routines","capability","selection"],"records":[["depac_runge_kutta_fehlberg","DERKF/DDERKF","nonstiff_RKF45; scalar_or_vector_tolerances; interval_or_step","deferred_no_callback_abort"],["depac_adams","DEABM/DDEABM","nonstiff_Adams_orders_1_to_12","deferred_no_callback_abort"],["depac_bdf","DEBDF/DDEBDF","stiff_BDF_orders_1_to_5; dense_or_banded_Jacobian","deferred_COMMON"],["sdrive","SDRIV1/2/3 and DDRIV1/2/3","nonstiff_or_stiff; roots; interpolation; optional_mass_matrix","SDRIV3_DDRIV3_recommended"],["dassl","SDASSL/DDASSL","implicit_DAE_BDF_orders_1_to_5; dense_or_banded_Jacobian","deferred_separate_DAE_scope"]]}),
        ),
        (
            "ode-callback-audit.json",
            json!({"schema_id":"slatec.ode.callback-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["routine","native_protocol","containment","future_wrapper_action"],"records":callback_rows}),
        ),
        (
            "ode-continuation-audit.json",
            json!({"schema_id":"slatec.ode.continuation-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["routine","persistence","recommended_API","restriction"],"records":continuation_rows}),
        ),
        (
            "ode-global-state-audit.json",
            json!({"schema_id":"slatec.ode.global-state-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["routine","state","external_io_or_printing","classification"],"records":global_rows}),
        ),
        (
            "ode-status-audit.json",
            json!({"schema_id":"slatec.ode.status-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["routine","native_status","future_mapping"],"records":status_rows}),
        ),
        (
            "ode-tolerance-audit.json",
            json!({"schema_id":"slatec.ode.tolerance-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["family","model","future_type"],"records":[["DEPAC","INFO(2) selects scalar_or_vector RTOL_and_ATOL; positive ATOL required for zero components","OdeTolerance::Scalar_or_Vector"],["SDRIVE","EPS plus IERROR selects absolute_relative_or_component_weights EWT","SdriveTolerance owned_by_session"],["DASSL","INFO selects scalar_or_vector RTOL_and_ATOL; routine may enlarge tolerances","OdeTolerance::Scalar_or_Vector"]]}),
        ),
        (
            "ode-jacobian-audit.json",
            json!({"schema_id":"slatec.ode.jacobian-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["family","protocol","storage","scope"],"records":[["DEBDF/DDEBDF","optional JAC/DJAC; finite-difference fallback","dense N_by_N or banded (2*ML+MU+1)_by_N","deferred_COMMON"],["SDRIV3/DDRIV3","JACOBN optional; FA for mass matrix","dense_or_banded selected by MITER/IMPL/ML/MU","future_RHS_only_scope"],["SDASSL/DDASSL","JAC optional, receives CJ iteration coefficient","dense_or_banded Fortran column-major work matrix","deferred_DAE_scope"]]}),
        ),
        (
            "ode-interpolation-audit.json",
            json!({"schema_id":"slatec.ode.interpolation-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["family","capability","state_requirement","result"],"records":[["DERKF/DEABM","target_or_intermediate_step returns; no standalone dense-output routine","live_driver_work","not_first_scope"],["DEBDF","INTYD/DINTYD Kth derivative interpolation","global COMMON history","unsafe_independent_use"],["SDRIV3","NTASK=1 interpolates target; NSTATE=11 reports interpolation","live owned session WORK","future_session_bound"],["DASSL","driver interpolation to TOUT; SDATRP/DDATRP subsidiary","live owned session WORK","future_session_bound"]]}),
        ),
        (
            "ode-event-audit.json",
            json!({"schema_id":"slatec.ode.event-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["family","event_support","limits","scope"],"records":[["SDRIV2/3","G sign-change roots; NROOT; root index in IWORK(6)","at_most_one_root_per_equation_per_internal_step; no_direction_or_terminal_filter","future_deferred_after_RHS_only"],["DEPAC","none","discontinuities require stop/restart","deferred"],["DASSL","none","no event callback","deferred"]]}),
        ),
        (
            "ode-workspace-audit.json",
            json!({"schema_id":"slatec.ode.workspace-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["routine","formula","provenance"],"records":workspace_rows}),
        ),
        (
            "ode-source-closure-audit.json",
            json!({"schema_id":"slatec.ode.source-closure-audit","schema_version":"1.0.0","snapshot_id":snapshot_id,"columns":["family","roots","genuine_subsidiaries","practicality"],"records":[["DERKF/DDERKF","DERKF,DDERKF","DERKFS/DRKFS,DEFEHL/DFEHL,HSTART/DHSTRT,HVNRM/DHVNRM,machine_constants,XERROR","narrow_but_callback_abort_unsuitable"],["SDRIV3/DDRIV3","SDRIV3,DDRIV3","SDSTP/DDSTP,SDNTP/DDNTP,SDZRO/DDZRO,LINPACK_dense_banded,machine_constants,XERROR","practical_after_restricted_RHS_only_contract"],["DASSL","SDASSL,DDASSL","initialization,step,interpolation,weights,norms,LINPACK,machine_constants,XERROR","narrow_but_separate_DAE_milestone"],["DEBDF","DEBDF,DDEBDF","LSOD/DLSOD,INTYD/DINTYD,LINPACK,XERROR","COMMON_blocks_prevent_safe_selection"]]}),
        ),
        (
            "ode-selection.json",
            json!({"schema_id":"slatec.ode.selection","schema_version":"1.0.0","snapshot_id":snapshot_id,"recommendation":"sdrive_expert_rhs_only","recommended_routines":["SDRIV3","DDRIV3"],"first_scope":"real f32/f64 explicit y_prime_equals_f_t_y only, owned non-cloneable OdeSession, caller-controlled continuation, panic-contained F trampoline that sets native N=0, native runtime lock, scoped XERROR restoration","deferred":["DERKF/DDERKF and DEABM/DDEABM: no callback abort","DEBDF/DDEBDF and INTYD/DINTYD: COMMON history","SDASSL/DDASSL: DAE/Jacobian/consistent-initial-state scope","CDRIV*: complex API out of scope"],"interoperability":"slices and lightweight checked views; internal mutable workspace; optional nalgebra/ndarray/faer adapters behind separate features"}),
        ),
    ];
    let summary = format!(
        "# SLATEC ODE-family audit\n\n- Snapshot: `{snapshot_id}`.\n- Public drivers span DEPAC RKF/Adams/BDF, SDRIVE, and DASSL DAE families.\n- **Selection:** `SDRIV3`/`DDRIV3`, restricted initially to real RHS-only IVPs in an owned non-cloneable session. Their documented mutable-`N` callback abort supports panic and user-error containment; caller work arrays hold continuation state and the executable driver has no COMMON or external I/O.\n- DEPAC RKF/Adams drivers are deferred because their RHS callbacks have no documented native abort signal. DEBDF and `INTYD` use process-global COMMON history. DASSL is a distinct DAE/Jacobian/consistent-initial-state milestone.\n- SDRIVE supports sign-change roots but no direction/terminal filtering; events remain deferred from the first wrapper scope.\n- No public ODE feature, raw declaration, provider closure, native source, or translated algorithm is added.\n"
    );
    let mut bytes = Vec::new();
    for (name, value) in outputs {
        let text = canonical(&value)?;
        bytes.extend_from_slice(name.as_bytes());
        bytes.extend_from_slice(text.as_bytes());
        write(&output_dir.join(name), &text)?;
    }
    write(&output_dir.join("ode-audit-summary.md"), &summary)?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id,
        semantic_hash: hash::bytes(&bytes),
        candidate_count: CANDIDATES.len(),
    })
}

fn callback_safety(candidate: &Candidate) -> &'static str {
    if candidate.callbacks.contains("set callback N=0") || candidate.callbacks.contains("set N=0") {
        "native_documented_abort_after_callback"
    } else {
        "no_documented_abort_or_not_applicable"
    }
}
fn session_recommendation(candidate: &Candidate) -> &'static str {
    if candidate.disposition == "recommended" {
        "owned_non_cloneable_OdeSession"
    } else {
        "not_public"
    }
}
fn global_class(candidate: &Candidate) -> &'static str {
    if candidate.state.starts_with("COMMON") {
        "unsafe_even_with_serialization_for_independent_history"
    } else {
        "serialized_XERROR_or_caller_state"
    }
}

fn ffi_records(inventory: &Value) -> Result<BTreeMap<String, (String, String, String)>> {
    let columns = inventory
        .get("columns")
        .and_then(Value::as_array)
        .ok_or_else(|| CorpusError::Verification("FFI inventory columns missing".to_owned()))?;
    let index = |name: &str| {
        columns
            .iter()
            .position(|v| v.as_str() == Some(name))
            .ok_or_else(|| {
                CorpusError::Verification(format!("FFI inventory column {name} missing"))
            })
    };
    let id = index("program_unit_id")?;
    let name = index("normalized_name")?;
    let subset = index("source_subset")?;
    let path = index("source_path")?;
    let mut result = BTreeMap::new();
    for row in inventory
        .get("records")
        .and_then(Value::as_array)
        .ok_or_else(|| CorpusError::Verification("FFI inventory records missing".to_owned()))?
    {
        let row = row
            .as_array()
            .ok_or_else(|| CorpusError::Verification("FFI record is not an array".to_owned()))?;
        let value = |i: usize| {
            row.get(i)
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or_else(|| CorpusError::Verification("FFI record string missing".to_owned()))
        };
        result.insert(value(name)?, (value(id)?, value(subset)?, value(path)?));
    }
    Ok(result)
}
fn validate(ffi: &BTreeMap<String, (String, String, String)>) -> Result<()> {
    let names = CANDIDATES.iter().map(|c| c.name).collect::<BTreeSet<_>>();
    for candidate in CANDIDATES {
        if !ffi.contains_key(candidate.name) {
            return Err(CorpusError::Verification(format!(
                "ODE candidate {} missing from FFI inventory",
                candidate.name
            )));
        }
        if candidate.pair != "none" && !names.contains(candidate.pair) {
            return Err(CorpusError::Verification(format!(
                "ODE pair {} for {} missing",
                candidate.pair, candidate.name
            )));
        }
    }
    if CANDIDATES
        .iter()
        .any(|c| c.name == "SPLP" || c.name == "DSPLP")
    {
        return Err(CorpusError::Verification(
            "LP candidate leaked into ODE audit".to_owned(),
        ));
    }
    Ok(())
}
fn read(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path)?)
        .map_err(|e| CorpusError::Verification(format!("invalid JSON {}: {e}", path.display())))
}
fn string(value: &Value, key: &str) -> Result<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("missing string {key}")))
}
fn canonical(value: &Value) -> Result<String> {
    serde_json::to_string(value)
        .map_err(|e| CorpusError::Verification(format!("cannot serialize audit JSON: {e}")))
}
fn write(path: &Path, text: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    fn root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }
    #[test]
    fn public_drivers_and_pairs_are_classified() {
        let names = CANDIDATES.iter().map(|c| c.name).collect::<BTreeSet<_>>();
        for name in [
            "DERKF", "DDERKF", "DEABM", "DDEABM", "DEBDF", "DDEBDF", "SDRIV1", "DDRIV1", "SDRIV2",
            "DDRIV2", "SDRIV3", "DDRIV3", "CDRIV1", "CDRIV2", "CDRIV3", "SDASSL", "DDASSL",
            "INTYD", "DINTYD",
        ] {
            assert!(names.contains(name));
        }
        assert!(CANDIDATES.iter().all(|c| {
            c.pair == "none"
                || CANDIDATES
                    .iter()
                    .any(|p| p.name == c.pair && p.pair == c.name)
        }));
    }
    #[test]
    fn callbacks_globals_and_lp_exclusion_are_recorded() {
        assert!(
            CANDIDATES
                .iter()
                .filter(|c| c.callbacks != "none")
                .all(|c| !c.callbacks.is_empty())
        );
        assert!(CANDIDATES.iter().any(|c| c.state.contains("COMMON")));
        assert!(
            CANDIDATES
                .iter()
                .all(|c| c.name != "SPLP" && c.name != "DSPLP")
        );
    }
    #[test]
    fn selected_pair_exists() {
        let selected = CANDIDATES
            .iter()
            .filter(|c| c.disposition == "recommended")
            .map(|c| c.name)
            .collect::<BTreeSet<_>>();
        assert_eq!(selected, BTreeSet::from(["SDRIV3", "DDRIV3"]));
    }
    #[test]
    fn generation_is_deterministic() {
        let root = root();
        let one = tempdir().unwrap();
        let two = tempdir().unwrap();
        let a = generate(
            &root.join("generated/ffi"),
            &root.join("generated/selected-corpus"),
            one.path(),
            true,
        )
        .unwrap();
        let b = generate(
            &root.join("generated/ffi"),
            &root.join("generated/selected-corpus"),
            two.path(),
            true,
        )
        .unwrap();
        assert_eq!(a.semantic_hash, b.semantic_hash);
        for name in [
            "ode-candidate-index.json",
            "ode-family-index.json",
            "ode-callback-audit.json",
            "ode-continuation-audit.json",
            "ode-global-state-audit.json",
            "ode-status-audit.json",
            "ode-tolerance-audit.json",
            "ode-jacobian-audit.json",
            "ode-interpolation-audit.json",
            "ode-event-audit.json",
            "ode-workspace-audit.json",
            "ode-source-closure-audit.json",
            "ode-selection.json",
            "ode-audit-summary.md",
        ] {
            assert_eq!(
                fs::read(one.path().join(name)).unwrap(),
                fs::read(two.path().join(name)).unwrap()
            );
        }
    }
}
