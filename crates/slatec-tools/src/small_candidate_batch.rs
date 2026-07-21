//! Focused evidence generator for the small quadrature and ODE raw-API batch.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};

const PUBLIC_ROUTINES: [&str; 9] = [
    "DPFQAD", "SDRIV1", "SDRIV2", "SDRIV3", "DDRIV1", "DDRIV2", "DDRIV3", "CDRIV1", "CDRIV2",
];
const NEW_PUBLIC_ROUTINES: [&str; 7] = [
    "DPFQAD", "SDRIV1", "SDRIV2", "DDRIV1", "DDRIV2", "CDRIV1", "CDRIV2",
];
const ODE_CLOSURE_SOURCES: [&str; 9] = [
    "src/sdriv1.f",
    "src/sdriv2.f",
    "src/sdriv3.f",
    "src/ddriv1.f",
    "src/ddriv2.f",
    "src/ddriv3.f",
    "src/cdriv1.f",
    "src/cdriv2.f",
    "src/cdriv3.f",
];

/// Result of generating or validating the focused candidate evidence.
pub struct SmallCandidateBatchResult {
    /// Digest of the deterministic JSON reports.
    pub semantic_hash: String,
    /// Directory containing the reports.
    pub output_dir: PathBuf,
}

struct Candidate {
    routine: &'static str,
    source_path: &'static str,
    historical_role: &'static str,
    direct_callers: &'static [&'static str],
    direct_callees: &'static [&'static str],
    callback_count: u8,
    abi_status: &'static str,
    final_disposition: &'static str,
    reason: &'static str,
}

const CANDIDATES: [Candidate; 13] = [
    Candidate {
        routine: "DFQAD",
        source_path: "src/dfqad.f",
        historical_role: "no retained SLATEC program unit",
        direct_callers: &[],
        direct_callees: &[],
        callback_count: 0,
        abi_status: "not-applicable",
        final_disposition: "missing-symbol",
        reason: "The exact Netlib src/dfqad.f URL returns HTTP 404; no retained catalogue identity, selected provider, or native symbol exists.",
    },
    Candidate {
        routine: "DPFQAD",
        source_path: "src/dpfqad.f",
        historical_role: "independently callable double-precision piecewise-polynomial quadrature driver",
        direct_callers: &[],
        direct_callees: &["D1MACH", "DINTRV", "DPPGQ8", "XERMSG"],
        callback_count: 1,
        abi_status: "validated scalar f64 callback; direct scalar return",
        final_disposition: "canonical-public",
        reason: "The source prologue and DPPGQ8 call site establish an independent integration contract and `FUN(X)` callback ABI.",
    },
    Candidate {
        routine: "SDRIV1",
        source_path: "src/sdriv1.f",
        historical_role: "ordinary single-precision initial-value ODE driver",
        direct_callers: &[],
        direct_callees: &["SDRIV3"],
        callback_count: 1,
        abi_status: "validated real f32 derivative subroutine callback",
        final_disposition: "canonical-public",
        reason: "The wrapper has its own documented call contract and supplies default stiff-method controls to SDRIV3.",
    },
    Candidate {
        routine: "SDRIV2",
        source_path: "src/sdriv2.f",
        historical_role: "advanced single-precision ODE driver with method and root controls",
        direct_callers: &[],
        direct_callees: &["SDRIV3"],
        callback_count: 2,
        abi_status: "validated real f32 derivative and root-function callbacks",
        final_disposition: "canonical-public",
        reason: "The source documents independent method, event, workspace, and continuation controls before forwarding to SDRIV3.",
    },
    Candidate {
        routine: "SDRIV3",
        source_path: "src/sdriv3.f",
        historical_role: "expert single-precision ODE driver",
        direct_callers: &["SDRIV1", "SDRIV2"],
        direct_callees: &["SDSTP", "SDNTL", "SROOTS"],
        callback_count: 5,
        abi_status: "previously reviewed expert callback ABI",
        final_disposition: "canonical-public",
        reason: "Already public as the expert driver; this batch preserves its canonical path and provider closure.",
    },
    Candidate {
        routine: "DDRIV1",
        source_path: "src/ddriv1.f",
        historical_role: "ordinary double-precision initial-value ODE driver",
        direct_callers: &[],
        direct_callees: &["DDRIV3"],
        callback_count: 1,
        abi_status: "validated real f64 derivative subroutine callback",
        final_disposition: "canonical-public",
        reason: "The wrapper has its own documented call contract and supplies default stiff-method controls to DDRIV3.",
    },
    Candidate {
        routine: "DDRIV2",
        source_path: "src/ddriv2.f",
        historical_role: "advanced double-precision ODE driver with method and root controls",
        direct_callers: &[],
        direct_callees: &["DDRIV3"],
        callback_count: 2,
        abi_status: "validated real f64 derivative and root-function callbacks",
        final_disposition: "canonical-public",
        reason: "The source documents independent method, event, workspace, and continuation controls before forwarding to DDRIV3.",
    },
    Candidate {
        routine: "DDRIV3",
        source_path: "src/ddriv3.f",
        historical_role: "expert double-precision ODE driver",
        direct_callers: &["DDRIV1", "DDRIV2"],
        direct_callees: &["DDSTP", "DDNTL", "DROOTS"],
        callback_count: 5,
        abi_status: "previously reviewed expert callback ABI",
        final_disposition: "canonical-public",
        reason: "Already public as the expert driver; this batch preserves its canonical path and provider closure.",
    },
    Candidate {
        routine: "CDRIV1",
        source_path: "src/cdriv1.f",
        historical_role: "ordinary complex single-precision initial-value ODE driver",
        direct_callers: &[],
        direct_callees: &["CDRIV3"],
        callback_count: 1,
        abi_status: "validated complex32 argument callback through focused native integration",
        final_disposition: "canonical-public",
        reason: "The independent convenience-driver contract and complex derivative callback both have focused source, link, and runtime evidence.",
    },
    Candidate {
        routine: "CDRIV2",
        source_path: "src/cdriv2.f",
        historical_role: "advanced complex single-precision ODE driver with root controls",
        direct_callers: &[],
        direct_callees: &["CDRIV3"],
        callback_count: 2,
        abi_status: "validated complex32 derivative and real root-function callbacks through focused native integration",
        final_disposition: "canonical-public",
        reason: "The independent advanced-driver contract, complex callback layout, root callback, and workspace formula have focused evidence.",
    },
    Candidate {
        routine: "CDRIV3",
        source_path: "src/cdriv3.f",
        historical_role: "expert complex ODE driver",
        direct_callers: &["CDRIV1", "CDRIV2"],
        direct_callees: &["CDSTP", "CDNTL", "CROOTS"],
        callback_count: 5,
        abi_status: "unsupported advanced complex callback ABI",
        final_disposition: "unsupported-abi",
        reason: "The independent JACOBN, FA, and USERS complex callback contracts are not yet compiler- and runtime-probed on the supported profile; CDRIV1/2 do not require exposing them.",
    },
    Candidate {
        routine: "DVSUP",
        source_path: "src/dvsup.f",
        historical_role: "no retained SLATEC program unit",
        direct_callers: &[],
        direct_callees: &[],
        callback_count: 0,
        abi_status: "not-applicable",
        final_disposition: "missing-symbol",
        reason: "The exact Netlib src/dvsup.f URL returns HTTP 404; no retained catalogue identity, selected provider, or native symbol exists.",
    },
    Candidate {
        routine: "DBVSUP",
        source_path: "src/dbvsup.f",
        historical_role: "boundary-value support routine with implicit external dependencies",
        direct_callers: &[],
        direct_callees: &["DFMAT", "DGVEC", "DUIVP", "DUVEC"],
        callback_count: 4,
        abi_status: "unsupported implicit external-procedure ABI",
        final_disposition: "unsupported-abi",
        reason: "Its four external-procedure dependencies have no direct Rust callback declaration or user-data/context contract on the selected profile, so an ABI-correct independent public path cannot be claimed.",
    },
];

/// Generates the four concise, source-hash-grounded focused reports.
pub fn generate(root: &Path, output_dir: &Path) -> Result<SmallCandidateBatchResult> {
    let status = read_json(&root.join("generated/raw-api/routine-status.json"))?;
    let final_disposition = read_json(&root.join("generated/raw-api/final-disposition.json"))?;
    let closure = read_json(&root.join("crates/slatec-src/metadata/family-source-closure.json"))?;
    let status_records = records(&status, "routine status")?;
    let final_records = records(&final_disposition, "final disposition")?;
    let quadrature_paths = closure_paths(&closure, "quadrature-callbacks")?;
    let ode_paths = closure_paths(&closure, "ode-sdrive-expert")?;

    let mut report_records = CANDIDATES
        .iter()
        .map(|candidate| record(candidate, status_records, final_records))
        .collect::<Result<Vec<_>>>()?;
    report_records.sort_by_key(|record| record["routine"].as_str().unwrap_or_default().to_owned());

    let report = json!({
        "schema_id":"slatec-sys.small-raw-api-candidate-batch-review",
        "schema_version":"1.0.0",
        "supported_abi_profile":"ffi-profile-gnu-mingw-x86_64",
        "scope":"DFQAD/DPFQAD, the DRIV1/2/3 families, and DVSUP/DBVSUP",
        "provider_closures":{
            "quadrature-callbacks":quadrature_paths,
            "ode-sdrive-expert":ode_paths
        },
        "summary":{
            "named_candidates":CANDIDATES.len(),
            "canonical_public":PUBLIC_ROUTINES.len(),
            "new_canonical_public":NEW_PUBLIC_ROUTINES.len(),
            "unsupported_abi":2,
            "missing_symbol":2
        },
        "records":report_records
    });
    let dfqad = focused_abi_report(&report, &["DFQAD", "DPFQAD"], "slatec-sys.dfqad-dpfqad-abi")?;
    let driv = focused_abi_report(
        &report,
        &[
            "SDRIV1", "SDRIV2", "SDRIV3", "DDRIV1", "DDRIV2", "DDRIV3", "CDRIV1", "CDRIV2",
            "CDRIV3",
        ],
        "slatec-sys.driv-family-abi",
    )?;
    let bvsup = focused_abi_report(&report, &["DVSUP", "DBVSUP"], "slatec-sys.bvsup-family-abi")?;

    fs::create_dir_all(output_dir)?;
    write_json(
        &output_dir.join("small-candidate-batch-review.json"),
        &report,
    )?;
    write_json(&output_dir.join("dfqad-dpfqad-abi.json"), &dfqad)?;
    write_json(&output_dir.join("driv-family-abi.json"), &driv)?;
    write_json(&output_dir.join("bvsup-family-abi.json"), &bvsup)?;
    write_if_changed(
        &output_dir.join("small-candidate-batch-review.md"),
        markdown(&report)?.as_bytes(),
    )?;

    let semantic_hash = hash::bytes(
        &[
            json_bytes(&report)?,
            json_bytes(&dfqad)?,
            json_bytes(&driv)?,
            json_bytes(&bvsup)?,
        ]
        .concat(),
    );
    Ok(SmallCandidateBatchResult {
        semantic_hash,
        output_dir: output_dir.to_owned(),
    })
}

/// Regenerates the reports and enforces local promotion and provider invariants.
pub fn validate(root: &Path, output_dir: &Path) -> Result<SmallCandidateBatchResult> {
    let result = generate(root, output_dir)?;
    let status = read_json(&root.join("generated/raw-api/routine-status.json"))?;
    let final_disposition = read_json(&root.join("generated/raw-api/final-disposition.json"))?;
    let status_records = records(&status, "routine status")?;
    let final_records = records(&final_disposition, "final disposition")?;
    for routine in PUBLIC_ROUTINES {
        let record = find(status_records, routine, "routine status")?;
        if record["canonical_rust_path"].as_str() == Some("not_promoted") {
            return Err(policy(&format!("{routine} lacks a canonical public path")));
        }
        if record["provider_feature"]
            .as_str()
            .is_none_or(|feature| feature == "not_assigned")
        {
            return Err(policy(&format!("{routine} lacks a provider feature")));
        }
    }
    for routine in NEW_PUBLIC_ROUTINES {
        let record = find(status_records, routine, "routine status")?;
        for (field, expected) in [
            ("raw_api_state", "reviewed_public_driver"),
            ("link_test_status", "passed"),
            ("runtime_test_status", "passed"),
        ] {
            if record[field].as_str() != Some(expected) {
                return Err(policy(&format!("{routine} has invalid {field}")));
            }
        }
        if find(final_records, routine, "final disposition")?["final_disposition"].as_str()
            != Some("canonical-public")
        {
            return Err(policy(&format!("{routine} is not canonical-public")));
        }
    }
    for routine in ["CDRIV3", "DBVSUP"] {
        if find(final_records, routine, "final disposition")?["final_disposition"].as_str()
            != Some("unsupported-callback-abi")
        {
            return Err(policy(&format!(
                "{routine} must retain an explicit unsupported ABI disposition"
            )));
        }
    }
    let ode = fs::read_to_string(root.join("crates/slatec-sys/src/ode.rs"))?;
    for symbol in [
        "sdriv1_", "ddriv1_", "sdriv2_", "ddriv2_", "cdriv1_", "cdriv2_",
    ] {
        if ode.matches(&format!("#[link_name = \"{symbol}\"]")).count() != 1 {
            return Err(policy(&format!(
                "{symbol} does not have exactly one extern declaration"
            )));
        }
    }
    let quadrature = fs::read_to_string(root.join("crates/slatec-sys/src/quadrature.rs"))?;
    if quadrature.matches("#[link_name = \"dpfqad_\"]").count() != 1 {
        return Err(policy(
            "dpfqad_ does not have exactly one extern declaration",
        ));
    }
    Ok(result)
}

fn record(candidate: &Candidate, status: &[Value], final_records: &[Value]) -> Result<Value> {
    let netlib_url = format!("https://www.netlib.org/slatec/{}", candidate.source_path);
    let selected = find_optional(status, candidate.routine);
    let final_record = find_optional(final_records, candidate.routine);
    let final_disposition = final_record
        .and_then(|record| record["final_disposition"].as_str())
        .unwrap_or(candidate.final_disposition);
    let expected_inventory_disposition = match candidate.final_disposition {
        "unsupported-abi" => "unsupported-callback-abi",
        disposition => disposition,
    };
    if final_disposition != expected_inventory_disposition {
        return Err(policy(&format!(
            "{} has final inventory disposition {final_disposition}, expected {expected_inventory_disposition}",
            candidate.routine
        )));
    }
    let canonical_path = selected
        .and_then(|record| record["canonical_rust_path"].as_str())
        .filter(|path| *path != "not_promoted");
    let inventory_link_status = selected.and_then(|record| record["link_test_status"].as_str());
    let inventory_runtime_status =
        selected.and_then(|record| record["runtime_test_status"].as_str());
    let public_raw_test_status = if candidate.final_disposition == "canonical-public" {
        None
    } else {
        Some("not-applicable for a non-public raw ABI")
    };
    let provider_status = match selected.and_then(|record| record["provider_feature"].as_str()) {
        Some(feature) if feature != "not_assigned" => {
            format!("selected provider feature `{feature}`")
        }
        Some(_) => "selected source exists, but no public provider feature is assigned".to_owned(),
        None => "no retained selected-provider source or symbol".to_owned(),
    };
    Ok(json!({
        "routine":candidate.routine,
        "source_file":candidate.source_path,
        "exact_netlib_url":netlib_url,
        "source_hash":selected.and_then(|record| record["source_hash"].as_str()),
        "historical_role":candidate.historical_role,
        "program_unit_kind":selected.and_then(|record| record["program_unit_kind"].as_str()).unwrap_or("not-found"),
        "native_symbol":selected.and_then(|record| record["native_symbol"].as_str()),
        "provider_status":provider_status,
        "direct_callers":candidate.direct_callers,
        "direct_callees":candidate.direct_callees,
        "callback_count":candidate.callback_count,
        "abi_status":candidate.abi_status,
        "independent_call_contract":candidate.final_disposition == "canonical-public",
        "recommended_disposition":candidate.final_disposition,
        "final_disposition":candidate.final_disposition,
        "canonical_rust_path":canonical_path,
        "feature":selected.and_then(|record| record["feature"].as_str()),
        "provider_feature":selected.and_then(|record| record["provider_feature"].as_str()),
        "link_test_status":public_raw_test_status.or(inventory_link_status),
        "runtime_test_status":public_raw_test_status.or(inventory_runtime_status),
        "inventory_link_test_status":inventory_link_status,
        "inventory_runtime_test_status":inventory_runtime_status,
        "reason":candidate.reason
    }))
}

fn focused_abi_report(report: &Value, names: &[&str], schema_id: &str) -> Result<Value> {
    let records = records(report, "small candidate report")?
        .iter()
        .filter(|record| names.contains(&record["routine"].as_str().unwrap_or_default()))
        .map(|record| {
            json!({
                "routine":record["routine"],
                "source_hash":record["source_hash"],
                "native_symbol":record["native_symbol"],
                "callback_count":record["callback_count"],
                "abi_status":record["abi_status"],
                "final_disposition":record["final_disposition"],
                "reason":record["reason"]
            })
        })
        .collect::<Vec<_>>();
    if records.len() != names.len() {
        return Err(policy(&format!("{schema_id} omitted a named routine")));
    }
    Ok(json!({
        "schema_id":schema_id,
        "schema_version":"1.0.0",
        "supported_abi_profile":"ffi-profile-gnu-mingw-x86_64",
        "records":records
    }))
}

fn closure_paths(closure: &Value, family: &str) -> Result<Vec<String>> {
    let sources = closure["sources"]
        .as_array()
        .ok_or_else(|| policy("provider source catalogue is absent"))?;
    let identifiers = closure["families"][family]
        .as_array()
        .ok_or_else(|| policy(&format!("{family} provider closure is absent")))?;
    let paths = identifiers
        .iter()
        .filter_map(Value::as_str)
        .filter_map(|identifier| {
            sources
                .iter()
                .find(|source| source["id"].as_str() == Some(identifier))
        })
        .filter_map(|source| source["path"].as_str())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    if family == "quadrature-callbacks" && !paths.iter().any(|path| path == "src/dpfqad.f") {
        return Err(policy(
            "quadrature-callbacks provider closure lacks src/dpfqad.f",
        ));
    }
    if family == "ode-sdrive-expert" {
        for required in ODE_CLOSURE_SOURCES {
            if !paths.iter().any(|path| path == required) {
                return Err(policy(&format!(
                    "ode-sdrive-expert provider closure lacks {required}"
                )));
            }
        }
    }
    Ok(paths)
}

fn markdown(report: &Value) -> Result<String> {
    let records = records(report, "small candidate report")?;
    let mut output = String::from(
        "# Small raw-API candidate batch review\n\nThis focused evidence resolves every requested candidate. `DFQAD` and `DVSUP` are exact-name misses, not withheld precision counterparts. `CDRIV3` and `DBVSUP` remain non-public because their independent callback contracts are not ABI-proven.\n\n| Routine | Historical role | Final disposition | Canonical path | ABI evidence |\n| --- | --- | --- | --- | --- |\n",
    );
    for record in records {
        output.push_str(&format!(
            "| `{}` | {} | `{}` | {} | {} |\n",
            record["routine"].as_str().unwrap_or_default(),
            record["historical_role"].as_str().unwrap_or_default(),
            record["final_disposition"].as_str().unwrap_or_default(),
            record["canonical_rust_path"].as_str().unwrap_or("—"),
            record["abi_status"].as_str().unwrap_or_default(),
        ));
    }
    output.push_str("\nThe JSON companion records source hashes, exact Netlib URLs, native symbols, provider features, direct callers/callees, callback counts, link/runtime statuses, and the precise reason for each non-public disposition.\n");
    Ok(output)
}

fn records<'a>(value: &'a Value, label: &str) -> Result<&'a [Value]> {
    value["records"]
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| policy(&format!("{label} records are absent")))
}

fn find<'a>(records: &'a [Value], routine: &str, label: &str) -> Result<&'a Value> {
    find_optional(records, routine).ok_or_else(|| policy(&format!("{label} lacks {routine}")))
}

fn find_optional<'a>(records: &'a [Value], routine: &str) -> Option<&'a Value> {
    records
        .iter()
        .find(|record| record["routine"].as_str() == Some(routine))
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
    use super::{CANDIDATES, NEW_PUBLIC_ROUTINES, ODE_CLOSURE_SOURCES, PUBLIC_ROUTINES};

    #[test]
    fn tracks_each_named_candidate_once() {
        assert_eq!(CANDIDATES.len(), 13);
        assert_eq!(PUBLIC_ROUTINES.len(), 9);
        assert_eq!(NEW_PUBLIC_ROUTINES.len(), 7);
        assert_eq!(ODE_CLOSURE_SOURCES.len(), 9);
    }
}
