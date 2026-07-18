//! Deterministic metadata for the in-memory-only SPLP/DSPLP wrappers.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Summary returned after generating safe in-memory LP metadata.
#[derive(Debug)]
pub struct ResultSummary {
    /// Shared selected-corpus snapshot identity.
    pub snapshot_id: String,
    /// Stable hash of every generated file.
    pub semantic_hash: String,
    /// Number of precision-specific wrappers.
    pub wrapper_count: usize,
}

/// Generates compact, offline-only metadata for the safe SPLP/DSPLP subset.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-lp-in-memory-metadata requires --offline".to_owned(),
        ));
    }
    let selected: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = selected["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;

    let wrappers = vec![
        json!([
            "slatec::linear_programming::LinearProgram::<f32>::solve",
            "SPLP",
            "splp_",
            "f32",
            "minimize_cTx_subject_to_Ax_equals_w_with_typed_x_and_w_bounds",
            "SerializedGlobal",
            "reviewed"
        ]),
        json!([
            "slatec::linear_programming::LinearProgram::<f64>::solve",
            "DSPLP",
            "dsplp_",
            "f64",
            "minimize_cTx_subject_to_Ax_equals_w_with_typed_x_and_w_bounds",
            "SerializedGlobal",
            "reviewed"
        ]),
    ];
    let status_records = vec![
        json!([1, "Optimal", "solution_and_row_activities_returned"]),
        json!([
            -1,
            "Infeasible",
            "legitimate_termination_no_solution_returned"
        ]),
        json!([
            -2,
            "NoFiniteSolution",
            "legitimate_termination_no_solution_returned"
        ]),
        json!([
            -3,
            "InfeasibleOrNoFiniteSolution",
            "native_contract_does_not_distinguish_no_solution_returned"
        ]),
        json!([
            -25,
            "IterationLimit",
            "legitimate_termination_no_solution_returned"
        ]),
        json!([-28, "LpNativeFailure::InsufficientBasisWorkspace", "error"]),
        json!(["other_documented_negative_INFO", "LpNativeFailure", "error"]),
        json!([
            "unexpected_INFO",
            "LpError::NativeContractViolation",
            "error"
        ]),
    ];
    let files = [
        (
            "lp-wrapper-index.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","raw_symbol","precision","mathematical_model","runtime_policy","review_state"],"records":wrappers}),
        ),
        (
            "lp-in-memory-contract.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.contract","schema_version":"1.0.0","snapshot_id":snapshot,"model":"minimize c^T x subject to A x = w","columns":["property","value","enforcement"],"records":[["matrix","owned_CSC; strictly_increasing_zero_based_row_indices_per_column; no_duplicates; no_explicit_zeroes","validated_before_FFI"],["variable_bounds","IND_1_lower; IND_2_upper; IND_3_range_or_fixed; IND_4_free","typed_LpBound"],["row_activity_bounds","same_native_IND_categories_as_variables","typed_LpBound"],["printing","KPRINT=0","fixed_internal_option"],["continuation","CONTIN=false","fixed_internal_option"],["save_restore","SAVEDT=false","fixed_internal_option"],["paging_unit_key_54","not_emitted_and_not_public","fixed"],["paging","forbidden","preflight_capacity_check_plus_no_IO_traps_and_postcall_counter"]]}),
        ),
        (
            "lp-workspace.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.workspace","schema_version":"1.0.0","snapshot_id":snapshot,"symbols":{"M":"row_count","N":"variable_count","NNZ":"stored_nonzero_count"},"columns":["region","formula","derivation","checked"],"records":[["LAMAT","max(N+7,N+NNZ+6)","SPLP_DSPLP_staging_and_option_53","checked_add"],["LBM","8*M","SPLP_DSPLP_documented_basis_region","checked_mul"],["WORK","LAMAT+LBM+4*N+8*M","SPLP_DSPLP_prologue","checked_add_and_checked_mul"],["IWORK","LAMAT+2*LBM+N+11*M","SPLP_DSPLP_prologue","checked_add_and_checked_mul"],["callback_column_scratch","maximum_column_nnz","owned_matrix_borrowed_during_callback","validated_native_INTEGER_conversion"]]}),
        ),
        (
            "lp-callback-contract.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.callback-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","native_callback","indexing","delivery","containment"],"records":[["SPLP","USRMAT","one_based_rows_and_columns","IFLAG_1_initialize; IFLAG_2_sequential_columns; IFLAG_3_finish; INDCAT_0_assignment","thread_local_context_under_global_lock; validate_requests_and_capacity; catch_unwind; no_unwind_through_Fortran"],["DSPLP","DUSRMT","one_based_rows_and_columns","IFLAG_1_initialize; IFLAG_2_sequential_columns; IFLAG_3_finish; INDCAT_0_assignment","thread_local_context_under_global_lock; validate_requests_and_capacity; catch_unwind; no_unwind_through_Fortran"]]}),
        ),
        (
            "lp-status-map.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_INFO","safe_mapping","output_policy"],"records":status_records}),
        ),
        (
            "lp-concurrency.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_function","native_routine","class","lock_scope","callback","XERROR","paging","reason"],"records":[["slatec::linear_programming::LinearProgram::<f32>::solve","SPLP","SerializedGlobal","callback_registration_through_XERROR_restoration_and_cleanup","thread_local_context_under_lock","flag_and_output_units_scoped_and_restored","forbidden_no_IO_traps","LA05_saved_state_and_COMMON; XERROR; callback_dispatch; provider_runtime_unknowns"],["slatec::linear_programming::LinearProgram::<f64>::solve","DSPLP","SerializedGlobal","callback_registration_through_XERROR_restoration_and_cleanup","thread_local_context_under_lock","flag_and_output_units_scoped_and_restored","forbidden_no_IO_traps","LA05_saved_state_and_COMMON; XERROR; callback_dispatch; provider_runtime_unknowns"]]}),
        ),
        (
            "lp-source-closure.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.source-closure","schema_version":"1.1.0","snapshot_id":snapshot,"roots":["SPLP","DSPLP"],"manifest":"crates/slatec-src/metadata/lp-in-memory-source-closure.json","provider_source_count":372,"inspected_object_count_with_profile":375,"included_groups":["LP_drivers_and_subsidiaries","LA05_in_memory_basis_solver","MC20_sparse_ordering","BLAS_level1","machine_constants","XERROR"],"excluded_implementations":["DPRWPG","PRWPGE","DPRWVR","PRWVIR","SOPENM","SCLOSM"],"project_symbols":["ABI_compatible_forbidden_entry_no_IO_stubs_for_excluded_paging_symbols"],"mutable_state":{"COMMON":["LA05DD","LA05DS"],"saved_locals":"present_and_writable","classification":"SerializedGlobal"},"narrow_link_probe":{"example":"link_linear_programming_in_memory","required":["dsplp_","splp_"],"retained_project_traps":["dprwpg_","prwpge_","dprwvr_","prwvir_","sopenm_","sclosm_"],"excluded_roots":["ddriv3_","ddassl_","dnls1_","dbocls_"],"status":"passed"},"duplicate_symbol_policy":"one_owner_per_symbol"}),
        ),
        (
            "lp-paging-policy.json",
            crate::runtime_storage_policy::lp_paging_policy(snapshot),
        ),
        (
            "linear-programming-deferred.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.deferred","schema_version":"2.0.0","snapshot_id":snapshot,"columns":["item","reason"],"records":[["external_paging","Fortran_unit_lifecycle_filename_ownership_cleanup_and_IO_failure_recovery_are_unresolved"],["option_key_54","unit_selection_is_not_part_of_the_in_memory_API"],["save_restore","could_activate_persistent_native_state_and_paging"],["warm_starts_and_user_basis","basis_protocol_not_exposed_in_initial_safe_API"],["mixed_integer_quadratic_and_nonlinear_programming","different_mathematical_families"],["sensitivity_and_post_optimality_ranges","not_reliably_supplied_by_the_selected_driver_contract"],["persistent_solver_objects","callback_and_native_history_lifecycle_deferred"],["parallel_LP_solves","native_closure_remains_SerializedGlobal"],["matrix_ecosystem_adapters","core_API_uses_owned_CSC_slices_without_dependencies"]]}),
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
        "# Safe in-memory SLATEC linear programming\n\n- Snapshot: `{snapshot}`.\n- Wrappers: `SPLP` (`f32`) and `DSPLP` (`f64`).\n- Model: minimize `c^T x` subject to `A x = w`, with typed lower, upper, ranged/fixed, or free bounds on both variables and row activities.\n- Sparse protocol: owned validated CSC is delivered through the one-based `USRMAT`/`DUSRMT` callback protocol without densification or reordering.\n- In-memory contract: printing, continuation, save/restore, and option key 54 are disabled; capacity is checked before FFI. The source profile contains no paging or file-I/O implementation. ABI-compatible project traps do no I/O and turn any unexpected paging entry into a contract violation.\n- Workspace: `LAMAT=max(N+7,N+NNZ+6)`, `LBM=8*M`, `WORK=LAMAT+LBM+4*N+8*M`, and `IWORK=LAMAT+2*LBM+N+11*M`, all calculated with checked arithmetic.\n- Runtime: the complete callback/XERROR/native/status scope is process serialized. XERROR control flag and output units are restored. Avoiding paging does not make LP reentrant.\n- Validation: both precisions cover optimal, equality/fixed-bound, infeasible, and no-finite-solution cases; malformed sparse inputs, capacity rejection, callback protocol errors, and callback panic containment are covered. No native or source artifact is committed.\n"
    );
    fs::write(
        output_dir.join("lp-validation-summary.md"),
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
    fn generation_is_deterministic_and_keeps_paging_deferred() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        for name in [
            "lp-wrapper-index.json",
            "lp-in-memory-contract.json",
            "lp-workspace.json",
            "lp-callback-contract.json",
            "lp-status-map.json",
            "lp-concurrency.json",
            "lp-source-closure.json",
            "lp-paging-policy.json",
            "linear-programming-deferred.json",
            "lp-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
        let paging: Value =
            serde_json::from_slice(&fs::read(first.path().join("lp-paging-policy.json")).unwrap())
                .unwrap();
        assert_eq!(
            paging["status"],
            "safe_in_memory_only; external_paging_deferred"
        );
        assert!(
            serde_json::to_string(&paging)
                .unwrap()
                .contains("option_KEY_54_not_emitted")
        );
    }
}
