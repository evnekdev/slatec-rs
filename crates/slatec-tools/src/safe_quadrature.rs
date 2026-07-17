//! Deterministic compact metadata for the reviewed safe quadrature surface.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const MAX_TOTAL_BYTES: usize = 64 * 1024;

#[derive(Clone, Copy)]
struct Wrapper {
    source: &'static str,
    safe_path: &'static str,
    precision: &'static str,
    interval: &'static str,
}

const WRAPPERS: &[Wrapper] = &[
    Wrapper {
        source: "DQAG",
        safe_path: "slatec::quadrature::integrate",
        precision: "f64",
        interval: "finite",
    },
    Wrapper {
        source: "QAG",
        safe_path: "slatec::quadrature::integrate_f32",
        precision: "f32",
        interval: "finite",
    },
    Wrapper {
        source: "DQAGS",
        safe_path: "slatec::quadrature::integrate_singular",
        precision: "f64",
        interval: "finite_endpoint_singularity",
    },
    Wrapper {
        source: "QAGS",
        safe_path: "slatec::quadrature::integrate_singular_f32",
        precision: "f32",
        interval: "finite_endpoint_singularity",
    },
    Wrapper {
        source: "DQAGI",
        safe_path: "slatec::quadrature::integrate_infinite",
        precision: "f64",
        interval: "infinite",
    },
    Wrapper {
        source: "QAGI",
        safe_path: "slatec::quadrature::integrate_infinite_f32",
        precision: "f32",
        interval: "infinite",
    },
    Wrapper {
        source: "DQAWC",
        safe_path: "slatec::quadrature::integrate_principal_value",
        precision: "f64",
        interval: "cauchy_principal_value",
    },
    Wrapper {
        source: "QAWC",
        safe_path: "slatec::quadrature::integrate_principal_value_f32",
        precision: "f32",
        interval: "cauchy_principal_value",
    },
];

#[derive(Debug)]
pub struct SafeQuadratureResult {
    pub snapshot_id: String,
    pub semantic_hash: String,
    pub wrappers: usize,
    pub deferred: usize,
    pub output_dir: PathBuf,
}

pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<SafeQuadratureResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-quadrature-api requires --offline".to_owned(),
        ));
    }
    let runtime = read(&runtime_profile_dir.join("manifest.json"))?;
    require_runtime_gate(&runtime)?;
    let inventory = read(&ffi_dir.join("interface-inventory.json"))?;
    let selected = read(&selected_corpus_dir.join("manifest.json"))?;
    let snapshot = string(&runtime, "snapshot_id")?;
    if string(&inventory, "snapshot_id")? != snapshot
        || string(&selected, "snapshot_id")? != snapshot
    {
        return Err(CorpusError::Verification(
            "quadrature runtime, FFI, and selected-corpus snapshots differ".to_owned(),
        ));
    }

    let columns = inventory["columns"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("raw FFI inventory lacks columns".to_owned()))?;
    let records = inventory["records"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("raw FFI inventory lacks records".to_owned()))?;
    let index = |name: &str| -> Result<usize> {
        columns
            .iter()
            .position(|value| value.as_str() == Some(name))
            .ok_or_else(|| CorpusError::Verification(format!("raw FFI inventory lacks {name}")))
    };
    let (id_ix, name_ix, symbol_ix, batch_ix) = (
        index("program_unit_id")?,
        index("normalized_name")?,
        index("observed_raw_symbol")?,
        index("binding_batch")?,
    );
    let specs = WRAPPERS
        .iter()
        .map(|entry| (entry.source, *entry))
        .collect::<BTreeMap<_, _>>();
    let mut found = BTreeSet::new();
    let mut wrappers = Vec::new();
    let mut deferred = Vec::new();
    for record in records {
        let row = record.as_array().ok_or_else(|| {
            CorpusError::Verification("raw FFI record is not positional".to_owned())
        })?;
        if row.get(batch_ix).and_then(Value::as_str) != Some("batch_callbacks") {
            continue;
        }
        let source = row.get(name_ix).and_then(Value::as_str).unwrap_or_default();
        if !is_quadrature(source) {
            continue;
        }
        let id = row.get(id_ix).and_then(Value::as_str).unwrap_or_default();
        let symbol = row
            .get(symbol_ix)
            .and_then(Value::as_str)
            .unwrap_or_default();
        if let Some(spec) = specs.get(source) {
            if id.is_empty() || symbol.is_empty() {
                return Err(CorpusError::Verification(format!(
                    "selected quadrature routine {source} lacks identity or observed symbol"
                )));
            }
            found.insert(source);
            wrappers.push(json!([
                spec.safe_path,
                source,
                symbol,
                id,
                spec.precision,
                spec.interval,
                "thread_local_scoped_trampoline",
                "internal_limit_and_4x_work_arrays",
                "serialized_process_native_lock",
                "native_reference_passed",
            ]));
        } else {
            deferred.push(json!([
                source,
                symbol,
                deferred_reason(source),
                "manual_review_required"
            ]));
        }
    }
    let missing = specs
        .keys()
        .filter(|name| !found.contains(**name))
        .copied()
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return Err(CorpusError::Verification(format!(
            "reviewed quadrature bindings absent from raw inventory: {}",
            missing.join(", ")
        )));
    }
    wrappers.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    deferred.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));

    let mut outputs = BTreeMap::new();
    outputs.insert("quadrature-wrapper-index.json", compact(&json!({
        "schema_id":"slatec.safe-quadrature.wrapper-index", "schema_version":"1.0.0",
        "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["safe_path","raw_routine","raw_symbol","program_unit_id","precision","interval_type","callback_policy","workspace_policy","concurrency_policy","native_test_status"],
        "records":wrappers
    }))?);
    outputs.insert(
        "quadrature-status-map.json",
        compact(&json!({
            "schema_id":"slatec.safe-quadrature.status-map", "schema_version":"1.0.0",
            "snapshot_id":snapshot,
            "columns":["native_status","rust_error","meaning"],
            "records":[
                [0,"success","requested accuracy reached"],
                [1,"MaximumSubdivisions","subdivision limit reached"],
                [2,"RoundoffDetected","roundoff prevents requested accuracy"],
                [3,"BadIntegrandBehavior","bad local integrand behavior"],
                [4,"NonConvergence","extrapolation did not converge"],
                [5,"DivergentOrSlowlyConvergent","probable divergence or slow convergence"],
                [6,"NativeContractViolation","invalid native input; prevented by safe validation"]
            ]
        }))?,
    );
    outputs.insert(
        "quadrature-deferred.json",
        compact(&json!({
            "schema_id":"slatec.safe-quadrature.deferred", "schema_version":"1.0.0",
            "snapshot_id":snapshot, "columns":["raw_routine","raw_symbol","reason","review_state"],
            "records":deferred
        }))?,
    );
    let semantic_hash = semantic_hash(&outputs);
    outputs.insert(
        "quadrature-validation-summary.md",
        summary(&snapshot, wrappers.len(), deferred.len(), &semantic_hash).into_bytes(),
    );
    let total = outputs.values().map(Vec::len).sum::<usize>();
    if total > MAX_TOTAL_BYTES {
        return Err(CorpusError::Policy(format!(
            "safe quadrature metadata is {total} bytes, above {MAX_TOTAL_BYTES}"
        )));
    }
    promote(output_dir, &outputs)?;
    Ok(SafeQuadratureResult {
        snapshot_id: snapshot,
        semantic_hash,
        wrappers: wrappers.len(),
        deferred: deferred.len(),
        output_dir: output_dir.to_owned(),
    })
}

fn is_quadrature(name: &str) -> bool {
    let name = name.strip_prefix('D').unwrap_or(name);
    name.starts_with("QAG")
        || name.starts_with("QAW")
        || name.starts_with("QC25")
        || name.starts_with("QK")
        || matches!(name, "QNC79" | "QNG")
}

fn deferred_reason(name: &str) -> &'static str {
    let name = name.strip_prefix('D').unwrap_or(name);
    if matches!(name, "QAGP" | "QAGPE") {
        "caller_supplied_breakpoint_array"
    } else if matches!(name, "QAWF" | "QAWFE" | "QAWO" | "QAWOE") {
        "oscillatory_weighted_workspace_interface"
    } else if matches!(name, "QAWS" | "QAWSE") {
        "specialized_weighted_endpoint_interface"
    } else if name.ends_with('E') || name.starts_with("QC25") || name.starts_with("QK") {
        "expert_or_internal_workspace_interface"
    } else {
        "outside_focused_adaptive_family"
    }
}

fn require_runtime_gate(runtime: &Value) -> Result<()> {
    let validation = runtime["validation"].as_object().ok_or_else(|| {
        CorpusError::Verification("runtime profile lacks validation gates".to_owned())
    })?;
    for gate in [
        "abi_validated",
        "machine_constants_validated",
        "legacy_error_behavior_validated",
        "fnlib_initialization_validated",
    ] {
        if validation.get(gate).and_then(Value::as_bool) != Some(true) {
            return Err(CorpusError::Verification(format!(
                "runtime-profile gate {gate} is not true"
            )));
        }
    }
    Ok(())
}

fn read(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path)?).map_err(CorpusError::from)
}

fn string(value: &Value, field: &str) -> Result<String> {
    value[field]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("missing {field}")))
}

fn compact(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn semantic_hash(outputs: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut bytes = Vec::new();
    for (name, content) in outputs {
        bytes.extend_from_slice(name.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(content);
        bytes.push(0);
    }
    hash::bytes(&bytes)
}

fn summary(snapshot: &str, wrappers: usize, deferred: usize, hash: &str) -> String {
    format!(
        "# Safe adaptive quadrature validation\n\n- Snapshot: `{snapshot}`\n- Profile: `{PROFILE}`\n- Reviewed safe wrappers: {wrappers}\n- Deferred quadrature interfaces: {deferred}\n- Callback policy: scoped thread-local trampoline; panics and non-finite values are contained\n- Concurrency policy: native calls serialize; nested callback integration is rejected\n- Workspace policy: safe API allocates `LIMIT` integers and `4 * LIMIT` numeric values\n- Semantic hash: `{hash}`\n\nThe original SLATEC Fortran routines remain the numerical implementation. Native execution evidence is profile-specific; detailed binaries and logs remain ignored.\n"
    )
}

fn promote(output_dir: &Path, outputs: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    for (name, bytes) in outputs {
        let temporary = output_dir.join(format!(".{name}.tmp"));
        fs::write(&temporary, bytes)?;
        let destination = output_dir.join(name);
        if destination.exists() {
            fs::remove_file(&destination)?;
        }
        fs::rename(temporary, destination)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_only_the_focused_roots_as_wrappers() {
        assert!(is_quadrature("DQAG"));
        assert!(is_quadrature("QAWSE"));
        assert!(!is_quadrature("DGAMMA"));
        assert_eq!(
            deferred_reason("DQAGPE"),
            "caller_supplied_breakpoint_array"
        );
        assert_eq!(
            deferred_reason("QAWO"),
            "oscillatory_weighted_workspace_interface"
        );
    }

    #[test]
    fn runtime_gate_requires_every_validation_layer() {
        let valid = json!({"validation":{
            "abi_validated":true, "machine_constants_validated":true,
            "legacy_error_behavior_validated":true, "fnlib_initialization_validated":true
        }});
        assert!(require_runtime_gate(&valid).is_ok());
        let mut invalid = valid;
        invalid["validation"]["abi_validated"] = json!(false);
        assert!(require_runtime_gate(&invalid).is_err());
    }

    #[test]
    fn committed_inputs_regenerate_identically_in_separate_roots() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let temporary = tempfile::tempdir().unwrap();
        let first = temporary.path().join("first/safe-api");
        let second = temporary.path().join("second/safe-api");
        let regenerate = |output: &Path| {
            generate(
                &root.join("generated/runtime-profile"),
                &root.join("generated/ffi"),
                &root.join("generated/selected-corpus"),
                output,
                true,
            )
            .unwrap()
        };
        let first_result = regenerate(&first);
        let second_result = regenerate(&second);
        assert_eq!(first_result.semantic_hash, second_result.semantic_hash);
        for name in [
            "quadrature-wrapper-index.json",
            "quadrature-status-map.json",
            "quadrature-deferred.json",
            "quadrature-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.join(name)).unwrap(),
                fs::read(second.join(name)).unwrap()
            );
        }
    }
}
