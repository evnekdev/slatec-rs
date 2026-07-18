//! Deterministic metadata from explicit reviewed real-FFTPACK plan specifications.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Summary returned after generating real-FFTPACK plan metadata.
#[derive(Debug)]
pub struct ResultSummary {
    /// Selected-corpus snapshot identity.
    pub snapshot_id: String,
    /// Stable hash of every emitted artifact.
    pub semantic_hash: String,
    /// Number of native user-callable routines represented by reviewed specs.
    pub routine_count: usize,
}

/// Generates metadata only from explicit reviewed specifications.
///
/// This command intentionally does not infer counterparts from routine names:
/// the source snapshot was audited and contains the listed `f32` entry points
/// but no real-FFTPACK `f64` entry points.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-fftpack-metadata requires --offline".to_owned(),
        ));
    }
    let selected: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = selected["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let source_closure: Value = serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/fftpack-real-source-closure.json"),
    )?)?;

    // This is the generator input.  Every row is explicit audit evidence; no
    // public API or raw declaration is synthesized by prefix substitution.
    let routines = vec![
        row(
            "RFFTI",
            "rffti_",
            "real_periodic",
            "initialize",
            "rffti.f",
            "N,WSAVE",
            "WSAVE",
            "2*N+15",
            "N>=1",
            "workspace_only",
            "RFFTI1",
        ),
        row(
            "RFFTF",
            "rfftf_",
            "real_periodic",
            "forward",
            "rfftf.f",
            "N,R,WSAVE",
            "R",
            "2*N+15",
            "N>=1",
            "RFFTB(RFFTF(x))=N*x",
            "RFFTF1",
        ),
        row(
            "RFFTB",
            "rfftb_",
            "real_periodic",
            "backward",
            "rfftb.f",
            "N,R,WSAVE",
            "R",
            "2*N+15",
            "N>=1",
            "RFFTB(RFFTF(x))=N*x",
            "RFFTB1",
        ),
        row(
            "EZFFTI",
            "ezffti_",
            "easy_real_fourier",
            "initialize",
            "ezffti.f",
            "N,WSAVE",
            "WSAVE",
            "3*N+15",
            "N>=1",
            "workspace_only",
            "EZFFT1",
        ),
        row(
            "EZFFTF",
            "ezfftf_",
            "easy_real_fourier",
            "forward",
            "ezfftf.f",
            "N,R,AZERO,A,B,WSAVE",
            "AZERO,A,B,WSAVE; R_read_only",
            "3*N+15",
            "N>=1",
            "series_coefficients_normalized_by_N",
            "RFFTF",
        ),
        row(
            "EZFFTB",
            "ezfftb_",
            "easy_real_fourier",
            "backward",
            "ezfftb.f",
            "N,R,AZERO,A,B,WSAVE",
            "R,WSAVE; coefficients_read_only",
            "3*N+15",
            "N>=1",
            "synthesis_of_EZFFTF_series",
            "RFFTB",
        ),
        row(
            "SINTI",
            "sinti_",
            "full_sine",
            "initialize",
            "sinti.f",
            "N,WSAVE",
            "WSAVE",
            "floor(7*N/2)+16",
            "N>=1",
            "workspace_only",
            "RFFTI",
        ),
        row(
            "SINT",
            "sint_",
            "full_sine",
            "transform",
            "sint.f",
            "N,X,WSAVE",
            "X,WSAVE_scratch",
            "floor(7*N/2)+16",
            "N>=1",
            "SINT(SINT(x))=2*(N+1)*x",
            "RFFTF",
        ),
        row(
            "COSTI",
            "costi_",
            "full_cosine",
            "initialize",
            "costi.f",
            "N,WSAVE",
            "WSAVE",
            "3*N+15",
            "N>=2",
            "workspace_only",
            "RFFTI",
        ),
        row(
            "COST",
            "cost_",
            "full_cosine",
            "transform",
            "cost.f",
            "N,X,WSAVE",
            "X,WSAVE_scratch",
            "3*N+15",
            "N>=2",
            "COST(COST(x))=2*(N-1)*x",
            "RFFTF",
        ),
        row(
            "SINQI",
            "sinqi_",
            "quarter_wave_sine",
            "initialize",
            "sinqi.f",
            "N,WSAVE",
            "WSAVE",
            "3*N+15",
            "N>=1",
            "workspace_only",
            "COSQI",
        ),
        row(
            "SINQF",
            "sinqf_",
            "quarter_wave_sine",
            "forward",
            "sinqf.f",
            "N,X,WSAVE",
            "X",
            "3*N+15",
            "N>=1",
            "SINQB(SINQF(x))=4*N*x",
            "COSQF",
        ),
        row(
            "SINQB",
            "sinqb_",
            "quarter_wave_sine",
            "backward",
            "sinqb.f",
            "N,X,WSAVE",
            "X",
            "3*N+15",
            "N>=1",
            "SINQF(SINQB(x))=4*N*x",
            "COSQB",
        ),
        row(
            "COSQI",
            "cosqi_",
            "quarter_wave_cosine",
            "initialize",
            "cosqi.f",
            "N,WSAVE",
            "WSAVE",
            "3*N+15",
            "N>=1",
            "workspace_only",
            "RFFTI",
        ),
        row(
            "COSQF",
            "cosqf_",
            "quarter_wave_cosine",
            "forward",
            "cosqf.f",
            "N,X,WSAVE",
            "X",
            "3*N+15",
            "N>=1",
            "COSQB(COSQF(x))=4*N*x",
            "COSQF1",
        ),
        row(
            "COSQB",
            "cosqb_",
            "quarter_wave_cosine",
            "backward",
            "cosqb.f",
            "N,X,WSAVE",
            "X",
            "3*N+15",
            "N>=1",
            "COSQF(COSQB(x))=4*N*x; executable_source_uses_factor_4",
            "COSQB1",
        ),
    ];
    if routines.len() != 16 {
        return Err(CorpusError::Verification(
            "incomplete reviewed FFTPACK plan specification".to_owned(),
        ));
    }

    let families = vec![
        json!([
            "real_periodic",
            "RealFftPlan",
            "RFFTI/RFFTF/RFFTB",
            "f32",
            "2*N+15",
            "packed_RFFTF_native_format"
        ]),
        json!([
            "easy_real_fourier",
            "EasyRealFftPlan",
            "EZFFTI/EZFFTF/EZFFTB",
            "f32",
            "3*N+15",
            "separate_mean_cosine_sine_arrays"
        ]),
        json!([
            "full_sine",
            "SineTransformPlan",
            "SINTI/SINT",
            "f32",
            "floor(7*N/2)+16",
            "self_inverse_scale_2*(N+1)"
        ]),
        json!([
            "full_cosine",
            "CosineTransformPlan",
            "COSTI/COST",
            "f32",
            "3*N+15",
            "self_inverse_scale_2*(N-1)"
        ]),
        json!([
            "quarter_wave_sine",
            "QuarterWaveSinePlan",
            "SINQI/SINQF/SINQB",
            "f32",
            "3*N+15",
            "forward_backward_scale_4*N"
        ]),
        json!([
            "quarter_wave_cosine",
            "QuarterWaveCosinePlan",
            "COSQI/COSQF/COSQB",
            "f32",
            "3*N+15",
            "forward_backward_scale_4*N"
        ]),
    ];
    let wrappers = vec![
        json!([
            "RealFftPlan",
            "new,forward,backward,spectrum",
            "RFFTI/RFFTF/RFFTB",
            "f32",
            "SerializedGlobal"
        ]),
        json!([
            "EasyRealFftPlan",
            "new,forward,backward",
            "EZFFTI/EZFFTF/EZFFTB",
            "f32",
            "SerializedGlobal"
        ]),
        json!([
            "SineTransformPlan",
            "new,transform",
            "SINTI/SINT",
            "f32",
            "SerializedGlobal"
        ]),
        json!([
            "CosineTransformPlan",
            "new,transform",
            "COSTI/COST",
            "f32",
            "SerializedGlobal"
        ]),
        json!([
            "QuarterWaveSinePlan",
            "new,forward,backward",
            "SINQI/SINQF/SINQB",
            "f32",
            "SerializedGlobal"
        ]),
        json!([
            "QuarterWaveCosinePlan",
            "new,forward,backward",
            "COSQI/COSQF/COSQB",
            "f32",
            "SerializedGlobal"
        ]),
    ];
    let files = [
        (
            "fftpack-routine-inventory.json",
            json!({"schema_id":"slatec.safe-fftpack.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["routine","symbol","family","role","source_path","arguments","mutated_arguments","workspace","minimum_length","normalization","direct_dependency"],"records":routines}),
        ),
        (
            "fftpack-family-summary.json",
            json!({"schema_id":"slatec.safe-fftpack.family-summary","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["family","rust_plan","native_routines","precision","workspace","storage"],"records":families,"unavailable_precision":["f64_real_fftpack_entry_points_absent_from_authoritative_selected_snapshot"]}),
        ),
        (
            "fftpack-source-closure.json",
            json!({"schema_id":"slatec.safe-fftpack.source-closure","schema_version":"1.2.0","snapshot_id":snapshot,"manifest":"crates/slatec-src/metadata/fftpack-real-source-closure.json","roots":["RFFTI","RFFTF","RFFTB","EZFFTI","EZFFTF","EZFFTB","SINTI","SINT","COSTI","COST","SINQI","SINQF","SINQB","COSQI","COSQF","COSQB"],"source_ids":source_closure["source_ids"],"sources":source_closure["sources"],"narrow_link_probe":{"example":"link_fftpack_real","required_symbols":["rffti_","rfftf_","rfftb_"],"retained_real_fftpack_roots":["ezffti_","ezfftf_","ezfftb_","sinti_","sint_","costi_","cost_","sinqi_","sinqf_","sinqb_","cosqi_","cosqf_","cosqb_"],"retention_reason":"all_public_plan_methods_share_one_Rust_module_object; this_is_not_whole_archive_linking","excluded_root_symbols":["cffti_","cfftf_","cfftb_"],"status":"source_build_passed"},"excluded":["complex_FFTPACK","multidimensional_FFTPACK","FISHPACK","BLAS","XERROR"]}),
        ),
        (
            "fftpack-workspace.json",
            json!({"schema_id":"slatec.safe-fftpack.workspace","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["family","formula","checked_arithmetic","ownership"],"records":[["real_periodic","2*N+15","checked_mul_then_checked_add","plan_owned"],["easy_real_fourier","3*N+15","checked_mul_then_checked_add","plan_owned"],["full_sine","floor(7*N/2)+16","checked_mul_then_checked_div_then_checked_add","plan_owned"],["full_cosine","3*N+15","checked_mul_then_checked_add","plan_owned"],["quarter_wave_sine","3*N+15","checked_mul_then_checked_add","plan_owned"],["quarter_wave_cosine","3*N+15","checked_mul_then_checked_add","plan_owned"]]}),
        ),
        (
            "fftpack-normalization.json",
            json!({"schema_id":"slatec.safe-fftpack.normalization","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["family","native_relation","safe_policy"],"records":[["RFFTF_RFFTB","RFFTB(RFFTF(x))=N*x","no_implicit_scaling"],["EZFFTF_EZFFTB","synthesis_reconstructs_normalized_series","no_implicit_scaling"],["SINT","SINT(SINT(x))=2*(N+1)*x","no_implicit_scaling"],["COST","COST(COST(x))=2*(N-1)*x","no_implicit_scaling"],["SINQF_SINQB","composition=4*N*x","no_implicit_scaling"],["COSQF_COSQB","composition=4*N*x","no_implicit_scaling; COSQB_source_factor_4_overrides_legacy_prologue_factor_2"]]}),
        ),
        (
            "fftpack-storage-contract.json",
            json!({"schema_id":"slatec.safe-fftpack.storage-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["family","caller_buffer","workspace","contract"],"records":[["real_periodic","mutable_contiguous_length_N","plan_owned","in_place_packed_spectrum"],["easy_real_fourier","forward_read_only_length_N; backward_mutable_length_N","plan_owned","explicit_owned_mean_cosine_sine_result"],["sine_cosine_quarter_wave","mutable_contiguous_length_N","plan_owned","in_place_native_coefficient_order"]]}),
        ),
        (
            "fftpack-native-state.json",
            json!({"schema_id":"slatec.safe-fftpack.native-state","schema_version":"1.2.0","snapshot_id":snapshot,"object_imports":["cosf","sinf"],"object_import_resolution":"GNU_MinGW_C_math_runtime; narrow_source_build_link_probe_passed","columns":["source","storage","origin","access","effect","policy"],"records":[["rffti1.f","NTRYH(4); object ntryh.0 in .data","SAVE+DATA","no_source_writer; object_section_writable","factor_selection_table","SerializedGlobal_until_parallel_native_audit"],["ezfft1.f","NTRYH(4); object ntryh.0 in .data","SAVE+DATA","no_source_writer; object_section_writable","factor_selection_table","SerializedGlobal_until_parallel_native_audit"],["complete_closure","COMMON_XERROR_IO_callbacks","none_found","not_reachable","no_direct_global_error_or_IO_path","still_uses_existing_hosted_runtime_lock"]]}),
        ),
        (
            "fftpack-concurrency.json",
            json!({"schema_id":"slatec.safe-fftpack.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["backend_profile","class","lock_scope","reason"],"records":[["slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","first_safe_facade retains existing conservative lock while saved_DATA tables and object evidence are recorded"],["external_or_system_backend","BackendDependent","process_global_runtime_lock","provider_identity_and_runtime_contract_not_qualified"]]}),
        ),
        (
            "fftpack-wrapper-index.json",
            json!({"schema_id":"slatec.safe-fftpack.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["rust_plan","methods","native_routines","precision","runtime_policy"],"records":wrappers,"counts":{"native_user_callable_routines":16,"public_plan_types":6,"canonical_transform_methods":10,"helper_accessors":3}}),
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
        "# Safe real FFTPACK plans\n\n- Snapshot: `{snapshot}`.\n- Reviewed native entry points: 16 single-precision routines in six real transform families. No double-precision real FFTPACK counterparts occur in this selected source snapshot.\n- Plans own initialized workspaces, use exact native contiguous buffers, preserve native normalization, and do not copy in-place transform data.\n- `RFFTF` uses a documented packed spectrum; `EZFFTF` exposes explicit mean/cosine/sine arrays.\n- The closure has no COMMON, XERROR, callback, or Fortran-I/O path. `RFFTI1` and `EZFFT1` retain compile-time DATA/SAVE factor tables that are read-only in the audited source. This first facade stays `SerializedGlobal` under the existing hosted runtime policy.\n- The executable `COSQB` path uses a factor of 4, despite an older prologue formula that displays 2; native small-N direct-oracle tests and the stated `4*N` composition relation confirm the source behavior.\n- Complex FFTPACK, multidimensional transforms, FISHPACK, adapters, and translated algorithms remain deferred.\n"
    );
    fs::write(
        output_dir.join("fftpack-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        routine_count: 16,
    })
}

// The compact inventory schema intentionally mirrors the complete reviewed
// native contract. Keeping the fields explicit at each call site makes a
// missing audit decision visible rather than hiding it behind name-derived
// defaults.
#[allow(clippy::too_many_arguments)]
fn row(
    routine: &str,
    symbol: &str,
    family: &str,
    role: &str,
    source_path: &str,
    arguments: &str,
    mutated_arguments: &str,
    workspace: &str,
    minimum_length: &str,
    normalization: &str,
    dependency: &str,
) -> Value {
    json!([
        routine,
        symbol,
        family,
        role,
        source_path,
        arguments,
        mutated_arguments,
        workspace,
        minimum_length,
        normalization,
        dependency
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_deterministic_and_never_invents_double_precision() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        for name in [
            "fftpack-routine-inventory.json",
            "fftpack-family-summary.json",
            "fftpack-source-closure.json",
            "fftpack-workspace.json",
            "fftpack-normalization.json",
            "fftpack-storage-contract.json",
            "fftpack-native-state.json",
            "fftpack-concurrency.json",
            "fftpack-wrapper-index.json",
            "fftpack-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
        let inventory: Value = serde_json::from_slice(
            &fs::read(first.path().join("fftpack-routine-inventory.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(inventory["records"].as_array().unwrap().len(), 16);
        assert!(!serde_json::to_string(&inventory).unwrap().contains("DRFF"));
    }
}
