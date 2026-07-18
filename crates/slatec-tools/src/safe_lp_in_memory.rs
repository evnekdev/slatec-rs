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
            "InfeasibleAndNoFiniteSolution",
            "native_reports_both_diagnoses_no_solution_returned"
        ]),
        json!([
            -25,
            "IterationLimit",
            "legitimate_termination_no_optimal_solution; finite_LpProgress_when_decodable"
        ]),
        json!([
            -28,
            "LpNativeFailure::InsufficientBasisWorkspace",
            "data_dependent_basis_fill_failure; nominal_LBM_preflight_cannot_fully_prevent"
        ]),
        json!(["other_documented_negative_INFO", "LpNativeFailure", "error"]),
        json!([
            "unexpected_INFO",
            "LpError::NativeContractViolation",
            "error"
        ]),
    ];
    let output_contract = vec![
        json!([
            "PRIMAL(1:N)",
            "variables",
            "optimal_and_INFO_minus_25_progress",
            "native_decision_vector; independently_checked_finite"
        ]),
        json!([
            "PRIMAL(N+1:N+M)",
            "native_row_activities",
            "optimal",
            "compared_to_independently_recomputed_Ax"
        ]),
        json!([
            "DUALS(1:M)",
            "row_multipliers",
            "optimal",
            "y_in_L_equals_cTx_minus_yT_Ax_minus_w"
        ]),
        json!([
            "DUALS(M+1:M+N)",
            "reduced_costs",
            "optimal",
            "c_minus_A_transpose_y; independently_recomputed"
        ]),
        json!([
            "IBASIS(1:M)",
            "basis.basic",
            "optimal",
            "one_based_basic_decision_or_row_activity_identifiers"
        ]),
        json!([
            "IWORK(LAMAT+1:LAMAT+N+M)",
            "basis.positions",
            "optimal",
            "IBB_basic_nonbasic_and_range_endpoint_state"
        ]),
        json!([
            "INFO=-25_PRIMAL",
            "progress",
            "iteration_limit_only",
            "rescaled_current_primal_iterate; never_dual_or_basis"
        ]),
        json!([
            "ITLP_NREDC_factorization_counts",
            "not_exposed",
            "printed_only",
            "not_a_documented_stable_output"
        ]),
    ];
    let option_audit = vec![
        json!([
            "50",
            "objective_sense",
            "deferred",
            "public_API_is_reviewed_native_minimization_only"
        ]),
        json!(["51", "legacy_printing", "prohibited_fixed_off", "KPRINT=0"]),
        json!([
            "52",
            "printing_format",
            "prohibited",
            "legacy_printing_is_disabled"
        ]),
        json!([
            "53",
            "LAMAT_LBM",
            "internal_fixed",
            "exact_checked_in_memory_workspace"
        ]),
        json!([
            "54",
            "paging_unit",
            "prohibited",
            "no_Fortran_units_or_filesystem"
        ]),
        json!([
            "55",
            "continuation",
            "prohibited_fixed_off",
            "no_persistent_file_state"
        ]),
        json!([
            "56",
            "save_unit",
            "prohibited",
            "no_Fortran_units_or_filesystem"
        ]),
        json!([
            "57",
            "save_restore",
            "prohibited_fixed_off",
            "no_external_state"
        ]),
        json!([
            "58",
            "maximum_iterations",
            "public",
            "Option_usize_checked_against_native_INTEGER_and_REAL"
        ]),
        json!([
            "59",
            "user_basis",
            "deferred",
            "input_basis_lifecycle_not_public"
        ]),
        json!([
            "60",
            "user_column_scaling",
            "deferred",
            "requires_reviewed_scaling_vector_contract"
        ]),
        json!([
            "61",
            "user_cost_scaling",
            "deferred",
            "requires_reviewed_scaling_contract"
        ]),
        json!([
            "62",
            "matrix_size_check",
            "deferred",
            "safe_but_not_needed_for_initial_controls"
        ]),
        json!([
            "63",
            "relative_feasibility_tolerance",
            "public",
            "finite_nonnegative_typed_scalar"
        ]),
        json!([
            "64",
            "pricing",
            "public",
            "typed_SteepestEdge_or_MinimumReducedCost"
        ]),
        json!([
            "65",
            "recalculation_interval",
            "deferred",
            "insufficient_user_value_for_initial_surface"
        ]),
        json!([
            "66",
            "partial_pricing_count",
            "deferred",
            "insufficient_user_value_for_initial_surface"
        ]),
        json!([
            "67",
            "native_error_tuning",
            "deferred",
            "poor_stable_semantics"
        ]),
        json!([
            "68",
            "dense_callback",
            "prohibited",
            "only_reviewed_sparse_CSC_callback_is_public"
        ]),
        json!([
            "69",
            "absolute_feasibility_tolerance",
            "public",
            "finite_nonnegative_typed_scalar"
        ]),
    ];
    let files = [
        (
            "lp-wrapper-index.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.wrapper-index","schema_version":"1.1.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","raw_symbol","precision","mathematical_model","runtime_policy","review_state"],"records":wrappers}),
        ),
        (
            "lp-in-memory-contract.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.contract","schema_version":"1.1.0","snapshot_id":snapshot,"model":"minimize c^T x subject to A x = w","columns":["property","value","enforcement"],"records":[["matrix","owned_CSC; strictly_increasing_zero_based_row_indices_per_column; no_duplicates; no_explicit_zeroes","validated_before_FFI"],["variable_bounds","IND_1_lower; IND_2_upper; IND_3_range_or_fixed; IND_4_free","typed_LpBound"],["row_activity_bounds","same_native_IND_categories_as_variables","typed_LpBound"],["resident_memory_limit","maximum_resident_nonzeros_admits_input_NNZ; LAMAT_is_internal_exact_capacity","PagingRequired_reports_NNZ_and_LAMAT_before_FFI"],["printing","KPRINT=0","fixed_internal_option"],["continuation","CONTIN=false","fixed_internal_option"],["save_restore","SAVEDT=false","fixed_internal_option"],["paging_unit_key_54","not_emitted_and_not_public","fixed"],["paging","forbidden","preflight_capacity_check_plus_no_IO_traps_and_postcall_counter"],["optimality","primal_dual_KKT_diagnostics","recomputed_without_densification"]]}),
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
            json!({"schema_id":"slatec.safe-lp-in-memory.status-map","schema_version":"1.1.0","snapshot_id":snapshot,"columns":["native_INFO","safe_mapping","output_policy"],"records":status_records}),
        ),
        (
            "lp-output-contract.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.output-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_storage","safe_output","meaningful_status","contract"],"records":output_contract}),
        ),
        (
            "lp-dual-sign-convention.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.dual-sign-convention","schema_version":"1.0.0","snapshot_id":snapshot,"model":"minimize c^T x subject to A x = w","lagrangian":"c^T x - y^T(Ax-w)","columns":["quantity","formula","bound_rule"],"records":[["row_multiplier","y","lower_w_implies_y_ge_0; upper_w_implies_y_le_0; free_w_implies_y_zero"],["variable_reduced_cost","c-A^T*y","lower_x_implies_r_ge_0; upper_x_implies_r_le_0; free_x_implies_r_zero"],["ranged_entity","combined_reduced_cost","lower_active_positive; upper_active_negative; interior_zero"],["fixed_entity","combined_reaction","sign_unrestricted; separate_bound_multipliers_not_available"],["dual_objective","sum_infimum_over_x_and_w_of_reduced_cost_times_bound_domain","only_finite_when_free_entity_reduced_cost_is_zero"]]}),
        ),
        (
            "lp-basis-contract.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.basis-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["native_storage","native_encoding","safe_decoding","validation"],"records":[["IBASIS(1:M)","one_based_identifiers_1_to_N_plus_M; first_M_basic","LpBasis.basic","nonzero_in_range_unique_exactly_M"],["IBB(1:N+M)","negative_basic; positive_nonbasic; parity_selects_range_endpoint","LpBasisPosition","sign_matches_IBASIS_membership; no_zero_indicator_on_optimal_return"],["variable_identifier","1_to_N","DecisionVariable(zero_based)","checked_subtraction"],["row_identifier","N_plus_1_to_N_plus_M","RowActivity(zero_based)","checked_subtraction"]]}),
        ),
        (
            "lp-optimality-validation.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.optimality-validation","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["check","source","policy"],"records":[["row_activities","sparse_CSC_Ax","always_recomputed"],["objective","original_c_dot_x","always_recomputed"],["primal_bounds","typed_x_and_w_bounds","reported_and_checked_when_requested"],["stationarity","native_reduced_cost_minus_c_minus_A_transpose_y","reported_and_checked_when_requested"],["dual_feasibility","bound_specific_reduced_cost_sign","reported_and_checked_when_requested"],["complementarity","reduced_cost_times_distance_to_active_bound","reported_and_checked_when_requested"],["objective_gap","primal_minus_dual_bound_infimum","reported_when_free_entity_stationarity_makes_dual_finite"],["scaling","precision_epsilon_coefficient_state_bound_objective_and_dimension_magnitudes","default_conservative_or_user_typed_tolerances"]]}),
        ),
        (
            "lp-option-audit.json",
            json!({"schema_id":"slatec.safe-lp-in-memory.option-audit","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["key","native_control","classification","safe_policy"],"records":option_audit}),
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
            json!({"schema_id":"slatec.safe-lp-in-memory.deferred","schema_version":"2.1.0","snapshot_id":snapshot,"columns":["item","reason"],"records":[["external_paging","Fortran_unit_lifecycle_filename_ownership_cleanup_and_IO_failure_recovery_are_unresolved"],["option_key_54","unit_selection_is_not_part_of_the_in_memory_API"],["save_restore","could_activate_persistent_native_state_and_paging"],["warm_starts_and_user_basis","output_basis_is_checked_but_user_supplied_basis_lifecycle_is_not_public"],["separate_lower_upper_multipliers","native_driver_returns_only_combined_reduced_costs"],["mixed_integer_quadratic_and_nonlinear_programming","different_mathematical_families"],["sensitivity_and_post_optimality_ranges","not_reliably_supplied_by_the_selected_driver_contract"],["persistent_solver_objects","callback_and_native_history_lifecycle_deferred"],["parallel_LP_solves","native_closure_remains_SerializedGlobal"],["matrix_ecosystem_adapters","core_API_uses_owned_CSC_slices_without_dependencies"]]}),
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
        "# Safe in-memory SLATEC linear programming\n\n- Snapshot: `{snapshot}`.\n- Wrappers: `SPLP` (`f32`) and `DSPLP` (`f64`).\n- Model: minimize `c^T x` subject to `A x = w`, with typed lower, upper, ranged/fixed, or free bounds on both variables and row activities.\n- Sparse protocol: owned validated CSC is delivered through the one-based `USRMAT`/`DUSRMT` callback protocol without densification or reordering.\n- In-memory contract: printing, continuation, save/restore, and option key 54 are disabled; capacity is checked before FFI. The source profile contains no paging or file-I/O implementation. ABI-compatible project traps do no I/O and turn any unexpected paging entry into a contract violation.\n- Workspace: `LAMAT=max(N+7,N+NNZ+6)`, `LBM=8*M`, `WORK=LAMAT+LBM+4*N+8*M`, and `IWORK=LAMAT+2*LBM+N+11*M`, all calculated with checked arithmetic.\n- Outputs: optimal returns decode checked `IBASIS`/`IBB` basis state, row multipliers `y`, and reduced costs `c-A^T y`; lower and upper multipliers are not fabricated. `INFO=-25` can carry labelled finite primal progress but never an optimal dual/basis result.\n- Controls: only typed iteration limit, feasibility tolerances, and pricing are emitted. Raw options, paging, save/restore, printing, dense callbacks, and user basis input remain unavailable.\n- Runtime: the complete callback/XERROR/native/status scope is process serialized. XERROR control flag and output units are restored. Avoiding paging does not make LP reentrant.\n- Validation: both precisions cover primal-dual KKT conditions, basis decoding, iteration limit, equality/fixed-bound, infeasible, no-finite-solution, callback containment, capacity rejection, and no-file paging traps. No native or source artifact is committed.\n"
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
            "lp-output-contract.json",
            "lp-dual-sign-convention.json",
            "lp-basis-contract.json",
            "lp-optimality-validation.json",
            "lp-option-audit.json",
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
