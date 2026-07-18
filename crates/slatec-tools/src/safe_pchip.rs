//! Deterministic metadata from the reviewed PCHIP source and safe-wrapper contracts.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Returns the focused, complete native-state projection for each PCHIP safe
/// operation.
///
/// This supplements the broad native-origin audit while its archive evidence
/// remains intentionally scoped to pre-existing families.  The records are
/// conservative: every PCHIP wrapper remains globally serialized because the
/// reviewed closure contains writable `SAVE`/`DATA` storage and reaches
/// XERROR.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure: Value = serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/pchip-source-closure.json"),
    )?)?;
    let source_ids = closure["source_ids"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("PCHIP closure lacks source ids".to_owned()))?
        .iter()
        .filter_map(Value::as_str)
        .map(|id| format!("{id}.o"))
        .chain(
            ["profile-i1mach.o", "profile-r1mach.o", "profile-d1mach.o"]
                .into_iter()
                .map(str::to_owned),
        )
        .collect::<Vec<_>>();
    let source_paths = closure["sources"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("PCHIP closure lacks source records".to_owned()))?
        .iter()
        .filter_map(|source| source["path"].as_str())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let entries = [
        (
            "slatec::pchip::PiecewiseCubicHermite::monotone",
            "PCHIM",
            "selected-source-fbdec12fd32d6ccf.o",
            "PCHIM/DPCHIM SAVE+DATA ZERO,THREE",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::monotone_with_conditions",
            "PCHIC",
            "selected-source-18f92ad0315f81ff.o",
            "PCHIC/DPCHIC SAVE+DATA ZERO and transitive PCHSW state",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::spline",
            "PCHSP",
            "selected-source-bedb53cb09e62fa4.o",
            "PCHSP/DPCHSP SAVE+DATA endpoint constants",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::evaluate_into",
            "PCHFE",
            "selected-source-2dc2db8502f70104.o",
            "PCHFE/DPCHFE reaches XERROR on native-contract failure",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::evaluate_with_derivatives_into",
            "PCHFD",
            "selected-source-e3662e79a9cf4bf4.o",
            "PCHFD/DPCHFD reaches XERROR on native-contract failure",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::integrate",
            "PCHIA",
            "selected-source-524cd6064530283f.o",
            "PCHIA/DPCHIA SAVE+DATA ZERO and XERROR",
        ),
    ];
    Ok(entries
        .into_iter()
        .map(|(safe_function, routine, entry_object, local_state)| {
            json!({
                "safe_function":safe_function,
                "native_entry_points":[routine, format!("D{routine}")],
                "feature":"pchip",
                "effective_native_families":["pchip"],
                "entry_object":entry_object,
                "object_closure":source_ids.clone(),
                "source_closure":source_paths.clone(),
                "saved_mutable_locals":[local_state],
                "common_blocks":[],
                "xerror_state":["XERROR J4SAVE/XERSVE process-global state"],
                "fortran_io":[],
                "callback_state":["none"],
                "writable_symbols":["focused PCHIP GNU MinGW object audit: writable SAVE/DATA symbols present in closure"],
                "source_object_unresolved":[],
                "external_undefined_symbols":[],
                "feature_closure_mismatch":false,
                "native_routine_reentrancy":"SerializedGlobal",
                "rust_api_concurrency":"owned curve values are movable subject to Rust ownership; native calls remain serialized",
                "provider_runtime_thread_safety":"reviewed source build remains serialized; external and system backends are BackendDependent",
                "provider_unknowns":["external_or_system_Fortran_runtime_and_provider_contract_not_qualified"],
                "remaining_blockers":["SAVE/DATA writable storage","process-global XERROR","provider/runtime qualification"]
            })
        })
        .collect())
}

/// Concise result from PCHIP metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// The selected-corpus snapshot identifier.
    pub snapshot_id: String,
    /// Stable hash over every emitted file.
    pub semantic_hash: String,
    /// Count of native public PCHIP routines represented by the audit.
    pub routine_count: usize,
}

/// Generates PCHIP metadata only from explicit source-audit classifications.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-pchip-metadata requires --offline".to_owned(),
        ));
    }
    let manifest: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = manifest["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let selected: Value = serde_json::from_slice(&fs::read(
        selected_corpus_dir.join("selected-providers.json"),
    )?)?;
    let mut inventory = selected["records"]
        .as_array()
        .ok_or_else(|| {
            CorpusError::Verification("selected provider inventory lacks records".to_owned())
        })?
        .iter()
        .filter(|record| record["source_subset"] == "pchip")
        .map(|record| {
            let routine = record["normalized_name"].as_str().unwrap_or("unknown");
            json!([
                routine,
                record["source_path"],
                record["normalized_sha256"],
                role(routine),
                public_status(routine),
                state(routine)
            ])
        })
        .collect::<Vec<_>>();
    inventory.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    if inventory.len() != 41 {
        return Err(CorpusError::Verification(format!(
            "expected 41 selected PCHIP program units, found {}",
            inventory.len()
        )));
    }
    let closure: Value = serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/pchip-source-closure.json"),
    )?)?;
    let files = [
        (
            "pchip-routine-inventory.json",
            json!({"schema_id":"slatec.safe-pchip.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["routine","source_path","sha256","role","safe_status","native_state"],"records":inventory}),
        ),
        (
            "pchip-family-summary.json",
            json!({"schema_id":"slatec.safe-pchip.family-summary","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["family","precision","roots","safe_surface","deferred"],"records":[["monotone_hermite","f32","PCHIM,PCHIC,PCHFD,PCHFE,PCHIA","monotone; controlled; evaluate; derivative; integrate","PCHBS,PCHCM,PCHKT,PCHDOC"],["monotone_hermite","f64","DPCHIM,DPCHIC,DPCHFD,DPCHFE,DPCHIA","monotone; controlled; evaluate; derivative; integrate","DPCHBS,DPCHCM,DPCHKT"],["cubic_spline_hermite","f32","PCHSP","typed endpoint conditions","general spline abstraction"],["cubic_spline_hermite","f64","DPCHSP","typed endpoint conditions","general spline abstraction"]]}),
        ),
        (
            "pchip-source-closure.json",
            json!({"schema_id":"slatec.safe-pchip.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"manifest":"crates/slatec-src/metadata/pchip-source-closure.json","roots":["PCHIM","DPCHIM","PCHIC","DPCHIC","PCHSP","DPCHSP","PCHFE","DPCHFE","PCHFD","DPCHFD","PCHIA","DPCHIA"],"source_ids":closure["source_ids"],"sources":closure["sources"],"profile_machines":["I1MACH","R1MACH","D1MACH"],"narrow_link_probe":{"example":"link_pchip","required_symbols":["pchim_","dpchim_","pchfd_","dpchfd_"],"excluded":["B-splines","smoothing_splines","multidimensional_interpolation","BLAS"]}}),
        ),
        (
            "pchip-storage-contract.json",
            json!({"schema_id":"slatec.safe-pchip.storage-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["operation","native_storage","safe_storage","policy"],"records":[["construct","X,F,D with INCFD","owned contiguous Vec; INCFD=1","copied once; no sort or merge"],["PCHIC","WK length 2*(N-1)","private fallible Vec","exact checked formula"],["PCHSP","WK length 2*N","private fallible Vec","exact checked formula"],["PCHFE","XE,FE","borrowed points and caller output","no allocation in evaluate_into"],["PCHFD","XE,FE,DE","borrowed points and caller outputs","no allocation in derivative batch"]]}),
        ),
        (
            "pchip-endpoint-contract.json",
            json!({"schema_id":"slatec.safe-pchip.endpoint-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","native_codes","safe_type","semantics"],"records":[["PCHSP/DPCHSP","0..4","EndpointCondition","not-a-knot; first/second derivative; three/four-point"],["PCHIC/DPCHIC","0,+/-1..+/-5","MonotoneEndpointCondition","sign requests monotonicity adjustment; magnitude chooses endpoint formula"],["PCHIC/DPCHIC","SWITCH=0,positive,negative","SwitchPointPolicy","force extrema; bounded deviation; unrestricted three-point treatment"]]}),
        ),
        (
            "pchip-extrapolation-contract.json",
            json!({"schema_id":"slatec.safe-pchip.extrapolation-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routine","native_behavior","safe_default","allow_report"],"records":[["PCHFE/DPCHFE","nearest endpoint cubic; IERR=count","Reject","EvaluationReport.extrapolated_points"],["PCHFD/DPCHFD","nearest endpoint cubic; IERR=count","Reject","EvaluationReport.extrapolated_points"],["PCHIA/DPCHIA","endpoint cubic; IERR bit 1 lower bit 2 upper","Reject","IntegrationReport endpoint booleans"]]}),
        ),
        (
            "pchip-status-map.json",
            json!({"schema_id":"slatec.safe-pchip.status-map","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["routines","status","safe_interpretation"],"records":[["PCHIM/DPCHIM",">0","ConstructionReport::MonotonicitySwitches"],["PCHIC/DPCHIC","1..3","ConstructionReport::EndpointAdjusted"],["PCHFE/DPCHFE,PCHFD/DPCHFD",">0","EvaluationReport extrapolation count"],["PCHIA/DPCHIA","1..3","IntegrationReport endpoint extrapolation flags"],["all exposed roots","negative","PchipError::NativeFailure; public preflight should prevent routine input statuses"]]}),
        ),
        (
            "pchip-native-state.json",
            json!({"schema_id":"slatec.safe-pchip.native-state","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["closure_part","state","origin","object_evidence","effect","policy"],"records":[["PCHIM/DPCHIM","ZERO,THREE","SAVE+DATA","selected-source-fbdec12fd32d6ccf.o and selected-source-43b189a16cf89f3c.o: .bss zero; .data three","mutable saved locals persist across calls","SerializedGlobal"],["PCHIC/DPCHIC and PCHIA/DPCHIA","ZERO","SAVE+DATA","selected-source-18f92ad0315f81ff.o, selected-source-eecea5e48d4f2772.o, selected-source-524cd6064530283f.o, selected-source-9e0c9d79ef68b955.o: .bss zero","mutable saved locals persist across calls","SerializedGlobal"],["PCHSP/DPCHSP","HALF,ONE,TWO,THREE,ZERO","SAVE+DATA","selected-source-bedb53cb09e62fa4.o and selected-source-d122ae6d259c5176.o: .data half/one/two/three; .bss zero","mutable saved locals persist across calls","SerializedGlobal"],["PCHSW/DPCHSW","FACT,ONE,THIRD,THREE,TWO,ZERO","SAVE+DATA","selected-source-93edceb591bd748e.o and selected-source-c0533447d7ecab5d.o: .data fact/one/third/three/two; .bss zero","mutable saved locals persist across calls","SerializedGlobal"],["XERROR","J4SAVE IPARAM and XERSVE message tables","SAVE+DATA","selected XERROR closure; existing native-origin audit","process-global error state","SerializedGlobal"],["PCHIC switch support","R1MACH/D1MACH profile routines","machine constants plus XERROR on invalid access","reviewed GNU MinGW profile closure","profile-global runtime path","SerializedGlobal"],["complete PCHIP closure","COMMON,Fortran I/O,callbacks","none found","source review of all 42 closure files","no direct I/O/callback protocol; lock remains required","SerializedGlobal"]]}),
        ),
        (
            "pchip-concurrency.json",
            json!({"schema_id":"slatec.safe-pchip.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["backend_profile","class","lock_scope","reason"],"records":[["slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","PCHIP DATA/SAVE storage and XERROR are shared"],["external_or_system_backend","BackendDependent","process_global_runtime_lock","provider identity and storage semantics are not qualified"]]}),
        ),
        (
            "pchip-wrapper-index.json",
            json!({"schema_id":"slatec.safe-pchip.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["rust_type","operations","native_roots","precision","runtime_policy"],"records":[["PiecewiseCubicHermite","monotone,controlled,spline,evaluate,evaluate_with_derivative,integrate","PCHIM,PCHIC,PCHSP,PCHFE,PCHFD,PCHIA","f32","SerializedGlobal"],["PiecewiseCubicHermite","monotone,controlled,spline,evaluate,evaluate_with_derivative,integrate","DPCHIM,DPCHIC,DPCHSP,DPCHFE,DPCHFD,DPCHIA","f64","SerializedGlobal"]],"counts":{"native_user_callable_routines":12,"public_curve_types":1,"public_constructors":4,"canonical_evaluation_and_integration_methods":7,"subsidiaries_linkage_only":20}}),
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
        "# Safe PCHIP interpolation\n\n- Snapshot: `{snapshot}`.\n- The selected PCHIP package contains 41 program units. The safe facade exposes the twelve reviewed f32/f64 roots for monotone, controlled-monotone, spline-Hermite construction, evaluation, first-derivative evaluation, and definite integration.\n- Curves own contiguous knot/value/derivative vectors, pass `INCFD=1`, and never sort or merge data. PCHIC scratch is `2*(N-1)`; PCHSP scratch is `2*N`; both use checked arithmetic and fallible allocation.\n- Native endpoint-cubic extrapolation is explicit: safe methods reject it by default, while an allow policy returns a warning report.\n- PCHIP numerical units retain DATA/SAVE constants and every error path reaches process-global XERROR state. Calls remain globally serialized; no PCHIP native parallel-safety claim is made.\n- B-splines, smoothing, multidimensional interpolation, stride support, adapters, and translated algorithms remain deferred.\n"
    );
    fs::write(
        output_dir.join("pchip-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        routine_count: 12,
    })
}

fn role(routine: &str) -> &'static str {
    match routine {
        "PCHIM" | "DPCHIM" => "monotone_derivatives",
        "PCHIC" | "DPCHIC" => "controlled_monotone_derivatives",
        "PCHSP" | "DPCHSP" => "spline_derivatives",
        "PCHFE" | "DPCHFE" => "value_evaluation",
        "PCHFD" | "DPCHFD" => "value_and_derivative_evaluation",
        "PCHIA" | "DPCHIA" => "definite_integration",
        "PCHID" | "DPCHID" => "multi_interval_integral_subsidiary",
        "PCHDF" | "DPCHDF" => "endpoint_difference_subsidiary",
        _ => "package_support_or_deferred_public_candidate",
    }
}

fn public_status(routine: &str) -> &'static str {
    match routine {
        "PCHIM" | "DPCHIM" | "PCHIC" | "DPCHIC" | "PCHSP" | "DPCHSP" | "PCHFE" | "DPCHFE"
        | "PCHFD" | "DPCHFD" | "PCHIA" | "DPCHIA" => "exposed_through_typed_curve",
        _ => "deferred_or_linkage_only",
    }
}

fn state(routine: &str) -> &'static str {
    match routine {
        "PCHST" | "DPCHST" => "SAVE_DATA_constants",
        "PCHIM" | "DPCHIM" | "PCHIC" | "DPCHIC" | "PCHSP" | "DPCHSP" | "PCHIA" | "DPCHIA"
        | "PCHID" | "DPCHID" | "PCHDF" | "DPCHDF" | "PCHCS" | "DPCHCS" | "PCHSW" | "DPCHSW"
        | "PCHCI" | "DPCHCI" | "PCHCE" | "DPCHCE" | "CHFDV" | "DCHFDV" | "CHFIE" | "DCHFIE" => {
            "SAVE_DATA_constants_or_XERROR"
        }
        _ => "audited_package_support",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_deterministic_and_inventories_every_selected_pchip_unit() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        assert_eq!(one.routine_count, 12);
        for name in [
            "pchip-routine-inventory.json",
            "pchip-family-summary.json",
            "pchip-source-closure.json",
            "pchip-storage-contract.json",
            "pchip-endpoint-contract.json",
            "pchip-extrapolation-contract.json",
            "pchip-status-map.json",
            "pchip-native-state.json",
            "pchip-concurrency.json",
            "pchip-wrapper-index.json",
            "pchip-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
        let inventory: Value = serde_json::from_slice(
            &fs::read(first.path().join("pchip-routine-inventory.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(inventory["records"].as_array().unwrap().len(), 41);
    }
}
