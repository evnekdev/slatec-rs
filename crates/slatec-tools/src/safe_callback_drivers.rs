//! Deterministic implementation plan for safe callback-driver expansion.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

/// Summary returned by the callback-driver plan generator.
#[derive(Debug)]
pub struct ResultSummary {
    /// Stable hash of the generated plan files.
    pub semantic_hash: String,
}

const DRIVER_FAMILIES: &[&str] = &[
    "quadrature-callbacks",
    "nonlinear-systems",
    "ode-sdrive-expert",
];

const DRIVER_PROJECTIONS: &[(&str, &str, &str, &str, &str, &str)] = &[
    (
        "slatec::quadrature::integrate_piecewise_polynomial",
        "DPFQAD",
        "quadrature-piecewise-polynomial",
        "quadrature-callbacks",
        "src/dpfqad.f",
        "thread-local scalar callback context",
    ),
    (
        "slatec::nonlinear::solve_scalar_equations",
        "DSOS",
        "nonlinear-systems",
        "nonlinear-systems",
        "src/dsos.f",
        "thread-local indexed-equation callback context",
    ),
    (
        "slatec::nonlinear::solve_scalar_equations_f32",
        "SOS",
        "nonlinear-systems",
        "nonlinear-systems",
        "src/sos.f",
        "thread-local indexed-equation callback context",
    ),
    (
        "slatec::ode::Driv1Session::<f64>::integrate_to",
        "DDRIV1",
        "ode-sdrive-expert",
        "ode-sdrive-expert",
        "src/ddriv1.f",
        "thread-local ODE RHS callback context",
    ),
    (
        "slatec::ode::Driv1Session::<f32>::integrate_to",
        "SDRIV1",
        "ode-sdrive-expert",
        "ode-sdrive-expert",
        "src/sdriv1.f",
        "thread-local ODE RHS callback context",
    ),
    (
        "slatec::ode::Driv2Session::<f64>::integrate_to_with_events",
        "DDRIV2",
        "ode-sdrive-expert",
        "ode-sdrive-expert",
        "src/ddriv2.f",
        "thread-local ODE RHS and indexed-root callback contexts",
    ),
    (
        "slatec::ode::Driv2Session::<f32>::integrate_to_with_events",
        "SDRIV2",
        "ode-sdrive-expert",
        "ode-sdrive-expert",
        "src/sdriv2.f",
        "thread-local ODE RHS and indexed-root callback contexts",
    ),
    (
        "slatec::ode::ComplexDriv1Session::integrate_to",
        "CDRIV1",
        "ode-sdrive-expert",
        "ode-sdrive-expert",
        "src/cdriv1.f",
        "thread-local complex ODE RHS callback context",
    ),
    (
        "slatec::ode::ComplexDriv2Session::integrate_to_with_events",
        "CDRIV2",
        "ode-sdrive-expert",
        "ode-sdrive-expert",
        "src/cdriv2.f",
        "thread-local complex ODE RHS and indexed-root callback contexts",
    ),
];

/// Returns the hash-guarded provider records in the three focused callback
/// families. The caller retains the conservative serialized classification
/// because every closure can reach XERROR and invokes a scoped Rust callback.
pub(crate) fn focused_native_sources() -> Result<Vec<Value>> {
    let closure = family_closure()?;
    let source_ids = DRIVER_FAMILIES
        .iter()
        .flat_map(|family| closure["families"][family].as_array().into_iter().flatten())
        .filter_map(Value::as_str)
        .collect::<BTreeSet<_>>();
    let mut sources = BTreeMap::new();
    for source in closure["sources"].as_array().ok_or_else(|| {
        CorpusError::Verification("family source closure lacks sources".to_owned())
    })? {
        let id = source["id"].as_str().ok_or_else(|| {
            CorpusError::Verification("family source closure source lacks id".to_owned())
        })?;
        if source_ids.contains(id) {
            sources.insert(id.to_owned(), source.clone());
        }
    }
    if sources.len() != source_ids.len() {
        return Err(CorpusError::Verification(
            "callback-driver source closure references an unknown source".to_owned(),
        ));
    }
    Ok(sources.into_values().collect())
}

/// Returns focused native-state projections for every new safe callback
/// operation. This avoids treating a prior broad archive audit as coverage for
/// subsequently reviewed provider closures.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let sources = focused_native_sources()?;
    let by_path = sources
        .iter()
        .map(|source| {
            Ok((
                source["path"]
                    .as_str()
                    .ok_or_else(|| {
                        CorpusError::Verification("callback-driver source lacks path".to_owned())
                    })?
                    .to_owned(),
                source["id"]
                    .as_str()
                    .ok_or_else(|| {
                        CorpusError::Verification("callback-driver source lacks id".to_owned())
                    })?
                    .to_owned(),
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    DRIVER_PROJECTIONS
        .iter()
        .map(|(safe_function, routine, feature, family, entry_path, callback_state)| {
            let entry_id = by_path.get(*entry_path).ok_or_else(|| {
                CorpusError::Verification(format!(
                    "callback-driver closure lacks source for {routine}"
                ))
            })?;
            let closure = family_source_paths(&sources, family)?;
            Ok(json!({
                "safe_function":safe_function,
                "native_entry_points":[routine],
                "feature":feature,
                "effective_native_families":[family],
                "entry_object":format!("{entry_id}.o"),
                "object_closure":closure.0,
                "source_closure":closure.1,
                "saved_mutable_locals":["focused source closure remains conservatively serialized"],
                "common_blocks":[],
                "xerror_state":["XERROR J4SAVE/XERSVE process-global state"],
                "fortran_io":[],
                "callback_state":[callback_state],
                "writable_symbols":["focused source-build and link validation; no parallel-native claim"],
                "source_object_unresolved":[],
                "external_undefined_symbols":["GNU Fortran runtime and BLAS provider contract where selected by the closure"],
                "feature_closure_mismatch":false,
                "current_class":"SerializedGlobal",
                "best_possible_class_from_slatec_source":"SerializedFamily",
                "native_routine_reentrancy":"callback and XERROR state require the process-global native lock",
                "rust_api_concurrency":"owned continuation sessions prevent concurrent same-session use; all native calls remain serialized",
                "provider_runtime_thread_safety":"reviewed, external, and system providers remain serialized",
                "provider_unknowns":["Fortran runtime and provider contracts are not qualified for concurrent native entry"],
                "remaining_blockers":["process-global XERROR", "callback dispatch", "provider/runtime qualification"]
            }))
        })
        .collect()
}

fn family_closure() -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/family-source-closure.json"),
    )?)?)
}

fn family_source_paths(sources: &[Value], family: &str) -> Result<(Vec<String>, Vec<String>)> {
    let closure = family_closure()?;
    let source_ids = closure["families"][family]
        .as_array()
        .ok_or_else(|| {
            CorpusError::Verification(format!("missing callback-driver family {family}"))
        })?
        .iter()
        .filter_map(Value::as_str)
        .collect::<BTreeSet<_>>();
    let selected = sources
        .iter()
        .filter(|source| {
            source["id"]
                .as_str()
                .is_some_and(|id| source_ids.contains(id))
        })
        .map(|source| {
            Ok((
                source["id"]
                    .as_str()
                    .ok_or_else(|| CorpusError::Verification("source lacks id".to_owned()))?
                    .to_owned(),
                source["path"]
                    .as_str()
                    .ok_or_else(|| CorpusError::Verification("source lacks path".to_owned()))?
                    .to_owned(),
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    if selected.len() != source_ids.len() {
        return Err(CorpusError::Verification(format!(
            "focused callback-driver closure is incomplete for {family}"
        )));
    }
    Ok((
        selected.iter().map(|(id, _)| format!("{id}.o")).collect(),
        selected.into_iter().map(|(_, path)| path).collect(),
    ))
}

/// Generates the concise, repository-specific plan for the reviewed callback
/// driver wrappers. The plan is intentionally implementation evidence, not a
/// second whole-project audit.
pub fn generate(output_dir: &Path) -> Result<ResultSummary> {
    let manifest: serde_json::Value = serde_json::from_slice(&fs::read(repo_path(
        "generated/selected-corpus/manifest.json",
    ))?)?;
    let snapshot = manifest["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;

    let plan = json!({
        "schema_id": "slatec.safe-api.callback-driver-extension-plan",
        "schema_version": "1.0.0",
        "snapshot_id": snapshot,
        "scope": ["DPFQAD", "SOS", "DSOS", "SDRIV1", "DDRIV1", "SDRIV2", "DDRIV2", "CDRIV1", "CDRIV2"],
        "out_of_scope": ["CDRIV3", "DBVSUP", "DFQAD", "DVSUP"],
        "existing_architecture": {
            "callback_storage": "crates/slatec/src/callback_runtime.rs owns lexical thread-local slots for scalar, vector, nonlinear, and least-squares callbacks; crates/slatec/src/ode.rs owns the existing SDRIV3/DDRIV3 RHS slot.",
            "runtime_serialization": "Every hosted callback entry holds crate::runtime::lock_native for the complete native call; XERROR recovery is scoped with permit_recoverable_native_statuses where a reviewed native status is recoverable.",
            "panic_capture": "Each trampoline uses catch_unwind(AssertUnwindSafe(...)); callback failures are retained in scoped state and translated after native return, so panics never cross Fortran.",
            "nested_calls": "callback_runtime rejects any active callback slot. The expansion extends that guard to cover ODE callback contexts so unrelated callback families cannot re-enter each other.",
            "error_propagation": "Quadrature maps scalar callback failures to IntegrationError; nonlinear maps typed callback failures to NonlinearError; OdeError preserves callback error, panic, non-finite derivative, native status, and continuation state.",
            "workspace_ownership": "Safe wrappers allocate Vec workspaces, validate checked usize-to-FortranInteger conversion, and retain opaque work arrays only in owned continuation sessions.",
            "options_and_results": "Existing public names are IntegrationResult/IntegrationError, NonlinearOptions/NonlinearError, and OdeOptions/OdeError/OdeStepResult. New public types extend these families rather than creating a parallel API vocabulary.",
            "precision_naming": "The existing convention uses an unqualified f64 API and an explicit _f32 companion where an f32 sibling exists; existing ODE generic public types remain available.",
            "features": "Family aggregates are quadrature, nonlinear, and ode. Subfeatures select exact slatec-sys declarations and slatec-src provider closures; no milestone-named public feature is introduced."
        },
        "implementation_policy": {
            "shared_callback_support": "Extend callback_runtime with scalar indexed-equation and ODE RHS/root adapters plus one cross-family active-context guard. Keep all trampoline state private and scoped.",
            "dpfqad": "Use the checked PiecewisePolynomial<f64> representation and the existing quadrature module. The new wrapper validates finite interval/tolerance and derivative order, then returns a typed native completion status.",
            "sos_dsos": "Expose checked f32/f64 nonlinear-system functions. They allocate RW=1+6*N+N*(N+1)/2 and IW=3+N, translate K from one-based to zero-based, and retain IFLAG distinctions in SystemReport.",
            "driv": "Add owned continuation sessions for the reviewed DRIV1/2 and CDRIV1/2 surfaces. DRIV1 uses the source formula N*N+11*N+300; DRIV2/ CDRIV2 use their source-reviewed method/root-dependent formulas. State owns time, solution, and opaque native workspaces; a call with callback panic/non-finite output becomes terminal and cannot be continued.",
            "events": "DRIV2/ CDRIV2 use one indexed Rust root closure and a checked root count. A native root index is translated from IWORK(6) to zero-based Rust indexing.",
            "complex": "Use the existing public num_complex::Complex32 representation, verify its size and alignment against slatec_sys::Complex32, and convert explicitly at the internal boundary; no public transmutation or GNU Fortran layout type is exposed."
        },
        "validation": [
            "focused callback-runtime unit tests for mutable state, panic containment, cleanup, sequential replacement, and nested calls",
            "native DPFQAD polynomial integral; SOS/DSOS nonlinear system; SDRIV1/DDRIV1 exponential; SDRIV2/DDRIV2 root; CDRIV1/CDRIV2 complex exponential/root",
            "safe metadata, public-module, function-index, and rustdoc regeneration",
            "affected existing quadrature, roots, nonlinear, least-squares, and SDRIV3 callback tests because the shared runtime is extended"
        ]
    });
    let plan_json = serde_json::to_vec(&plan)?;
    let plan_md = format!(
        "# Safe callback-driver extension plan\n\n- Snapshot: `{snapshot}`.\n- Scope: `DPFQAD`, `SOS`/`DSOS`, `SDRIV1`/`DDRIV1`, `SDRIV2`/`DDRIV2`, and `CDRIV1`/`CDRIV2`.\n- Existing callback storage: private lexical TLS in `callback_runtime` plus the current SDRIV3/DDRIV3 ODE context. The implementation extends the shared guard rather than adding a public trampoline framework.\n- Runtime: all native callback calls remain process-serialized; `catch_unwind` records callback panic in scoped state and no panic crosses Fortran.\n- Nesting: active callback scopes reject another callback-bearing SLATEC operation, including across ODE and non-ODE families.\n- Workspaces: safe APIs allocate them; DRIV continuation sessions own and preserve opaque state across same-direction calls.\n- Public vocabulary: preserve `Integration*`, `Nonlinear*`, and `Ode*` naming and add family-local report/options only where the raw driver has distinct controls.\n- Features: extend `quadrature`, `nonlinear`, and `ode` subfeatures with exact declaration/provider closures; no milestone feature is exposed.\n- Out of scope: `CDRIV3`, `DBVSUP`, `DFQAD`, and `DVSUP`.\n"
    );
    let wrappers = json!({
        "schema_id":"slatec.safe-api.callback-driver-wrapper-index",
        "schema_version":"1.0.0",
        "snapshot_id":snapshot,
        "columns":["safe_path","raw_routine","precision","domain","callback_shape","workspace_ownership","status_mapping","runtime_class","panic_containment","continuation","feature","test_evidence"],
        "records":[
            ["slatec::quadrature::integrate_piecewise_polynomial","DPFQAD","f64","quadrature","FnMut(f64)->f64","checked PiecewisePolynomial; no caller workspace","PiecewiseQuadratureStatus","SerializedGlobal","shared scalar catch_unwind","one_shot","quadrature-piecewise-polynomial","DPFQAD polynomial integral"],
            ["slatec::nonlinear::solve_scalar_equations","DSOS","f64","nonlinear","FnMut(&[f64],usize)->f64","owned RW and IW","SystemTermination","SerializedGlobal","indexed catch_unwind; K is zero-based","one_shot","nonlinear-systems","DSOS circle system"],
            ["slatec::nonlinear::solve_scalar_equations_f32","SOS","f32","nonlinear","FnMut(&[f32],usize)->f32","owned RW and IW","SystemTermination","SerializedGlobal","indexed catch_unwind; K is zero-based","one_shot","nonlinear-systems","SOS circle system"],
            ["slatec::ode::Driv1Session::<f64>::integrate_to","DDRIV1","f64","ordinary differential equations","FnMut(f64,&[f64],&mut[f64])","owned persistent WORK","DrivStatus","SerializedGlobal","shared ODE TLS and catch_unwind","owned same-direction continuation","ode-sdrive-expert","DDRIV1 exponential"],
            ["slatec::ode::Driv1Session::<f32>::integrate_to","SDRIV1","f32","ordinary differential equations","FnMut(f32,&[f32],&mut[f32])","owned persistent WORK","DrivStatus","SerializedGlobal","shared ODE TLS and catch_unwind","owned same-direction continuation","ode-sdrive-expert","SDRIV1 exponential"],
            ["slatec::ode::Driv2Session::<f64>::integrate_to_with_events","DDRIV2","f64","ordinary differential equations","RHS plus indexed root closure","owned persistent WORK and IWORK","DrivStatus::RootFound zero-based","SerializedGlobal","shared ODE TLS and catch_unwind","owned same-direction continuation","ode-sdrive-expert","DDRIV2 root event"],
            ["slatec::ode::Driv2Session::<f32>::integrate_to_with_events","SDRIV2","f32","ordinary differential equations","RHS plus indexed root closure","owned persistent WORK and IWORK","DrivStatus::RootFound zero-based","SerializedGlobal","shared ODE TLS and catch_unwind","owned same-direction continuation","ode-sdrive-expert","SDRIV2 root event"],
            ["slatec::ode::ComplexDriv1Session::integrate_to","CDRIV1","Complex32/f32 time","ordinary differential equations","complex RHS closure","owned persistent complex WORK","DrivStatus","SerializedGlobal","shared ODE TLS and catch_unwind","owned same-direction continuation","ode-sdrive-expert","CDRIV1 rotation"],
            ["slatec::ode::ComplexDriv2Session::integrate_to_with_events","CDRIV2","Complex32/f32 time","ordinary differential equations","complex RHS plus indexed real root closure","owned persistent complex WORK and IWORK","DrivStatus::RootFound zero-based","SerializedGlobal","shared ODE TLS and catch_unwind","owned same-direction continuation","ode-sdrive-expert","CDRIV2 complex root event"]
        ]
    });
    let status = json!({"schema_id":"slatec.safe-api.callback-driver-status-map","schema_version":"1.0.0","snapshot_id":snapshot,"records":[["DPFQAD","1=Converged; 2=ToleranceNotMet"],["SOS/DSOS","1=iterate; 2=residual; 3=both; 4=slow/precision; 5=limit; 6=nonconverging limit; 7=diverging; 8=singular; 9=native invalid input"],["SDRIV1/DDRIV1/CDRIV1","2=target; 3=excess work; 4=tolerance adjusted; 5=RHS controlled stop; 6=interpolated return; 7=recoverable native failure"],["SDRIV2/DDRIV2/CDRIV2","2=target; 3=excess work; 4=tolerance adjusted; 5=root; 6=RHS controlled stop; 7=root controlled stop; 8=interpolated return; 9=recoverable native failure"]]});
    let deferred = json!({"schema_id":"slatec.safe-api.callback-driver-deferred","schema_version":"1.0.0","snapshot_id":snapshot,"records":[["CDRIV3","raw callback ABI unresolved"],["DBVSUP","outside scope"],["DFQAD","missing retained raw identity"],["DVSUP","missing retained raw identity"],["DRIV3 events/Jacobians/mass matrices","existing RHS-only session remains restricted"]]});
    fs::create_dir_all(output_dir)?;
    fs::write(
        output_dir.join("callback-driver-extension-plan.json"),
        &plan_json,
    )?;
    fs::write(
        output_dir.join("callback-driver-extension-plan.md"),
        plan_md.as_bytes(),
    )?;
    let wrapper_json = serde_json::to_vec(&wrappers)?;
    let status_json = serde_json::to_vec(&status)?;
    let deferred_json = serde_json::to_vec(&deferred)?;
    fs::write(
        output_dir.join("callback-driver-wrapper-index.json"),
        &wrapper_json,
    )?;
    fs::write(
        output_dir.join("callback-driver-status-map.json"),
        &status_json,
    )?;
    fs::write(
        output_dir.join("callback-driver-deferred.json"),
        &deferred_json,
    )?;
    let mut bytes = plan_json;
    bytes.extend_from_slice(plan_md.as_bytes());
    bytes.extend_from_slice(&wrapper_json);
    bytes.extend_from_slice(&status_json);
    bytes.extend_from_slice(&deferred_json);
    Ok(ResultSummary {
        semantic_hash: hash::bytes(&bytes),
    })
}

fn repo_path(relative: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_generation_is_deterministic() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        assert_eq!(
            generate(first.path()).unwrap().semantic_hash,
            generate(second.path()).unwrap().semantic_hash
        );
        for name in [
            "callback-driver-extension-plan.json",
            "callback-driver-extension-plan.md",
            "callback-driver-wrapper-index.json",
            "callback-driver-status-map.json",
            "callback-driver-deferred.json",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }

    #[test]
    fn focused_native_projections_cover_every_wrapper() {
        let projections = native_state_projections().unwrap();
        assert_eq!(projections.len(), DRIVER_PROJECTIONS.len());
        assert!(projections.iter().all(|projection| {
            projection["current_class"].as_str() == Some("SerializedGlobal")
                && projection["callback_state"]
                    .as_array()
                    .is_some_and(|state| !state.is_empty())
        }));
    }
}
