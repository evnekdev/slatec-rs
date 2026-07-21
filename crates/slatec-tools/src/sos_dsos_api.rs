//! Focused evidence generator for the reviewed SOS and DSOS raw drivers.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

const ROUTINES: [(&str, &str, &str, &str); 2] = [
    (
        "SOS",
        "sos_",
        "slatec_sys::nonlinear::systems::sos",
        "SosEquationF32",
    ),
    (
        "DSOS",
        "dsos_",
        "slatec_sys::nonlinear::systems::dsos",
        "SosEquationF64",
    ),
];
const REQUIRED_SOURCES: [&str; 7] = [
    "src/sos.f",
    "src/dsos.f",
    "src/soseqs.f",
    "src/dsoseq.f",
    "src/sossol.f",
    "src/dsossl.f",
    "src/xermsg.f",
];

/// Compact result of focused SOS/DSOS evidence generation or validation.
pub struct SosDsosResult {
    /// Digest of the two deterministic reports.
    pub semantic_hash: String,
    /// Directory containing the focused reports.
    pub output_dir: PathBuf,
}

/// Generates source-closure and callback-ABI evidence from authoritative records.
pub fn generate(root: &Path, output_dir: &Path) -> Result<SosDsosResult> {
    let status = read_json(&root.join("generated/raw-api/routine-status.json"))?;
    let closure = read_json(&root.join("crates/slatec-src/metadata/family-source-closure.json"))?;
    let records = status["records"]
        .as_array()
        .ok_or_else(|| policy("routine status records are absent"))?;
    let source_catalogue = closure["sources"]
        .as_array()
        .ok_or_else(|| policy("provider source catalogue is absent"))?;
    let source_paths = closure["families"]["nonlinear-systems"]
        .as_array()
        .ok_or_else(|| policy("nonlinear-systems provider closure is absent"))?
        .iter()
        .filter_map(Value::as_str)
        .filter_map(|id| {
            source_catalogue
                .iter()
                .find(|source| source["id"].as_str() == Some(id))
        })
        .filter_map(|source| source["path"].as_str())
        .collect::<Vec<_>>();
    for required in REQUIRED_SOURCES {
        if !source_paths.contains(&required) {
            return Err(policy(&format!(
                "nonlinear-systems provider closure lacks {required}"
            )));
        }
    }

    let mut closure_records = Vec::new();
    let mut callback_records = Vec::new();
    for (routine, symbol, canonical_path, callback_type) in ROUTINES {
        let record = records
            .iter()
            .find(|record| record["routine"].as_str() == Some(routine))
            .ok_or_else(|| policy(&format!("routine status lacks {routine}")))?;
        for (field, expected) in [
            ("raw_api_state", "reviewed_public_driver"),
            ("canonical_rust_path", canonical_path),
            ("native_symbol", symbol),
            ("feature", "nonlinear-systems"),
            ("provider_feature", "nonlinear-systems"),
            ("link_test_status", "passed"),
            ("runtime_test_status", "passed"),
        ] {
            if record[field].as_str() != Some(expected) {
                return Err(policy(&format!("{routine} has invalid {field}")));
            }
        }
        let (direct, subsidiary, machine) = if routine == "SOS" {
            (
                vec!["SOSEQS", "XERMSG"],
                vec!["SOSEQS", "SOSSOL"],
                vec!["I1MACH", "R1MACH"],
            )
        } else {
            (
                vec!["DSOSEQ", "XERMSG"],
                vec!["DSOSEQ", "DSOSSL"],
                vec!["I1MACH", "D1MACH"],
            )
        };
        closure_records.push(json!({
            "routine":routine,
            "native_symbol":symbol,
            "canonical_rust_path":canonical_path,
            "source_hash":record["source_hash"],
            "provider_feature":"nonlinear-systems",
            "direct_callees":direct,
            "reviewed_subsidiaries":subsidiary,
            "profile_override_dependencies":machine,
            "provider_source_paths":source_paths,
            "closure_status":"source_closure_complete"
        }));
        callback_records.push(json!({
            "routine":routine,
            "callback_name":"FNC",
            "rust_callback_type":callback_type,
            "fortran_kind":"scalar real function",
            "return_abi":"direct scalar return; no hidden result argument",
            "arguments":[
                {"name":"X","fortran_type":if routine == "SOS" {"REAL(*)"} else {"DOUBLE PRECISION(*)"},"direction":"read-only callback view","extent":"NEQ elements supplied by externally managed problem state; NEQ is not a callback argument"},
                {"name":"K","fortran_type":"INTEGER","direction":"input","range":"1..=NEQ (one-based equation index)"}
            ],
            "executable_call_evidence":if routine == "SOS" {"SOSEQS: F = FNC(X,K); FP = FNC(X,K)"} else {"DSOSEQ: F = FNC(X,K); FP = FNC(X,K)"},
            "context_abi":"no user-data or context pointer is present; stateful Rust use requires a future scoped context mechanism",
            "callback_status":"reviewed_source_and_runtime_tested"
        }));
    }
    closure_records.sort_by_key(|record| record["routine"].as_str().unwrap_or_default().to_owned());
    callback_records
        .sort_by_key(|record| record["routine"].as_str().unwrap_or_default().to_owned());
    let closure_json = json!({
        "schema_id":"slatec-sys.sos-dsos-call-closure",
        "schema_version":"1.0.0",
        "policy":"Only SOS and DSOS are public drivers; SOSEQS, DSOSEQ, SOSSOL, and DSOSSL remain reviewed internal subsidiaries.",
        "records":closure_records
    });
    let callback_json = json!({
        "schema_id":"slatec-sys.sos-dsos-callback-abi",
        "schema_version":"1.0.0",
        "supported_abi_profile":"ffi-profile-gnu-mingw-x86_64",
        "records":callback_records
    });
    fs::create_dir_all(output_dir)?;
    write_json(
        &output_dir.join("sos-dsos-call-closure.json"),
        &closure_json,
    )?;
    write_json(
        &output_dir.join("sos-dsos-callback-abi.json"),
        &callback_json,
    )?;
    write_if_changed(
        &output_dir.join("sos-dsos-call-closure.md"),
        format!(
            "# SOS/DSOS native call closure\n\nThe `nonlinear-systems` provider closure contains the two reviewed drivers, their four internal subsidiaries, `XERMSG`, and the selected machine-profile leaves. `SOS` calls `SOSEQS`; `DSOS` calls `DSOSEQ`. The subsidiary solvers are not public raw paths.\n\n- Source closure entries: {}\n- Driver symbols: `sos_`, `dsos_`\n- Closure status: `source_closure_complete`\n",
            source_paths.len()
        )
        .as_bytes(),
    )?;
    write_if_changed(
        &output_dir.join("sos-dsos-callback-abi.md"),
        b"# SOS/DSOS callback ABI\n\n`FNC` is a synchronous scalar function. `SOS` uses `unsafe extern \"C\" fn(*const f32, *const FortranInteger) -> f32`; `DSOS` uses the corresponding `f64` type. The callback receives only `X` and `K`: `NEQ` is not passed through this ABI, so the callback must know the readable `X` extent from externally managed problem state. `K` is a one-based equation index. There is no user-data/context pointer or hidden result argument; a future stateful Rust wrapper therefore needs a scoped context mechanism. The focused native tests exercise both callbacks and verify that both equation indices are observed.\n",
    )?;
    let semantic_hash =
        hash::bytes(&[json_bytes(&closure_json)?, json_bytes(&callback_json)?].concat());
    Ok(SosDsosResult {
        semantic_hash,
        output_dir: output_dir.to_owned(),
    })
}

/// Regenerates the reports and asserts the focused public-API invariants.
pub fn validate(root: &Path, output_dir: &Path) -> Result<SosDsosResult> {
    let result = generate(root, output_dir)?;
    let source = fs::read_to_string(root.join("crates/slatec-sys/src/nonlinear.rs"))?;
    for symbol in ["sos_", "dsos_"] {
        if source
            .matches(&format!("#[link_name = \"{symbol}\"]"))
            .count()
            != 1
        {
            return Err(policy(&format!(
                "{symbol} does not have exactly one extern declaration"
            )));
        }
    }
    for required in [
        "pub fn sos(",
        "pub fn dsos(",
        "pub mod systems",
        "pub use super::{dsos, sos, SosEquationF32, SosEquationF64};",
        "SosEquationF32",
        "SosEquationF64",
    ] {
        if !source.contains(required) {
            return Err(policy(&format!("nonlinear declaration lacks {required}")));
        }
    }
    Ok(result)
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    write_if_changed(path, &json_bytes(value)?)
}

fn json_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<()> {
    if fs::read(path).ok().as_deref() != Some(bytes) {
        fs::write(path, bytes)?;
    }
    Ok(())
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::REQUIRED_SOURCES;

    #[test]
    fn focused_closure_keeps_both_precision_siblings_and_subsidiaries() {
        assert!(REQUIRED_SOURCES.contains(&"src/sos.f"));
        assert!(REQUIRED_SOURCES.contains(&"src/dsos.f"));
        assert!(REQUIRED_SOURCES.contains(&"src/soseqs.f"));
        assert!(REQUIRED_SOURCES.contains(&"src/dsoseq.f"));
    }
}
