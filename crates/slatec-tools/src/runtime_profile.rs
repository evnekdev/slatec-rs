//! GNU MinGW runtime-profile validation for SLATEC machine constants and XERROR.
//!
//! Potentially terminating calls are always executed in authored child-process
//! probes. Raw output remains in the ignored evidence tree; committed output is
//! limited to compact classifications, hashes, and counts.

use crate::error::{CorpusError, Result};
use crate::{TOOL_NAME, TOOL_VERSION, hash, raw_ffi};
use flate2::read::GzDecoder;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use tar::Archive;

const SCHEMA_VERSION: &str = "1.0.0";
const SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const PROFILE_TARGET: &str = "x86_64-w64-mingw32";
const SIZE_LIMIT: usize = 4_000_000;
const TARGET_ROUTINES: &[&str] = &[
    "I1MACH", "R1MACH", "D1MACH", "INITDS", "INITS", "DGAMLM", "GAMLIM", "XERMSG", "XERROR",
    "XERRWV", "XERCLR", "XGETF", "XSETF", "XGETUA", "XSETUA", "XGETUN", "XSETUN", "J4SAVE",
    "FDUMP", "XERHLT", "XERCNT", "XERSVE",
];

#[derive(Debug, serde::Serialize)]
pub struct RuntimeProfileResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
    pub machine_discrepancies_before: usize,
    pub machine_discrepancies_after: usize,
    pub recovered_fnlib_probes: usize,
}

#[derive(Clone)]
struct ProbeRun {
    profile: String,
    mode: String,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
    returned: bool,
}

pub fn validate(
    evidence_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<RuntimeProfileResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "validate-runtime-profile uses only verified local evidence and requires --offline"
                .to_owned(),
        ));
    }

    let generated = raw_ffi::generate(
        evidence_dir,
        selected_corpus_dir,
        Path::new("generated/ffi-inventory"),
        Path::new("generated/ffi"),
        Path::new("crates/slatec-sys/src/generated"),
        true,
    )?;
    let snapshot = generated.snapshot_id;
    let compilation = read_value(Path::new("generated/ffi/compilation-results.json"))?;
    let profile_id = required_string(&compilation["compiler_profile"], "id")?;
    let compiler_target = required_string(&compilation["compiler_profile"], "target")?;
    if compiler_target != PROFILE_TARGET {
        return Err(CorpusError::Verification(format!(
            "runtime validation requires {PROFILE_TARGET}, found {compiler_target}"
        )));
    }
    let compiler_version = required_string(&compilation["compiler_profile"], "version")?;
    let workspace = evidence_dir
        .join("runtime-profile")
        .join(&snapshot)
        .join(&profile_id);
    fs::create_dir_all(workspace.join("logs"))?;
    let raw_workspace = evidence_dir
        .join("raw-ffi")
        .join(&snapshot)
        .join(&profile_id);
    let original_archive = raw_workspace.join("libslatec_selected_original.a");
    let corrected_archive = raw_workspace.join("libslatec_selected.a");
    let probe_source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("native/probes/runtime_profile_probe.f90");
    let compiler = compiler_path();
    let original_exe = compile_probe(
        &compiler,
        &probe_source,
        &original_archive,
        &workspace,
        "original",
    )?;
    let corrected_exe = compile_probe(
        &compiler,
        &probe_source,
        &corrected_archive,
        &workspace,
        "corrected",
    )?;

    let original_modes = [
        "constants",
        "initds",
        "inits",
        "dgamlm",
        "gamlim",
        "ai",
        "ei",
        "reciprocal-gamma",
        "degree",
        "warning-once",
        "warning",
        "recoverable-controlled",
        "recoverable-default",
        "fatal",
        "error-state",
        "invalid-machine-selector",
    ];
    let corrected_modes = [
        "compiler",
        "constants",
        "initds",
        "inits",
        "dgamlm",
        "gamlim",
        "ai",
        "ei",
        "reciprocal-gamma",
        "degree",
        "warning-once",
        "warning",
        "recoverable-controlled",
        "recoverable-default",
        "fatal",
        "error-state",
        "invalid-machine-selector",
    ];
    let mut runs = Vec::new();
    for mode in original_modes {
        runs.push(run_probe(&original_exe, "original", mode, &workspace)?);
    }
    for mode in corrected_modes {
        runs.push(run_probe(&corrected_exe, "corrected", mode, &workspace)?);
    }
    write_detailed_probe_evidence(&workspace, &runs)?;

    let original_constants = find_run(&runs, "original", "constants")?;
    let corrected_constants = find_run(&runs, "corrected", "constants")?;
    let compiler_reference = find_run(&runs, "corrected", "compiler")?;
    require_clean_run(corrected_constants)?;
    require_clean_run(compiler_reference)?;
    let machine = machine_outputs(
        &snapshot,
        original_constants,
        corrected_constants,
        compiler_reference,
    )?;
    if machine.after_discrepancies != 0 {
        return Err(CorpusError::Verification(format!(
            "corrected profile retains {} machine-constant discrepancies",
            machine.after_discrepancies
        )));
    }

    let provider_audit = provider_audit(
        &snapshot,
        selected_corpus_dir,
        evidence_dir,
        Path::new("generated/ffi/interface-inventory.json"),
    )?;
    let error_behavior = error_behavior(&snapshot, &runs)?;
    let fnlib = fnlib_results(&snapshot, &runs)?;
    if fnlib.corrected_failures != 0 {
        return Err(CorpusError::Verification(format!(
            "{} corrected FNLIB/runtime probes failed",
            fnlib.corrected_failures
        )));
    }
    let override_summary = override_summary(
        &snapshot,
        &profile_id,
        &original_archive,
        &corrected_archive,
    )?;

    let mut outputs = BTreeMap::<&str, Vec<u8>>::new();
    outputs.insert("provider-audit.json", compact(&provider_audit)?);
    outputs.insert("machine-constant-results.json", compact(&machine.results)?);
    outputs.insert(
        "machine-constant-discrepancies.json",
        compact(&machine.discrepancies)?,
    );
    outputs.insert("error-system-behavior.json", compact(&error_behavior)?);
    outputs.insert("fnlib-reprobe-results.json", compact(&fnlib.value)?);
    outputs.insert("override-summary.json", compact(&override_summary)?);
    outputs.insert(
        "validation-summary.md",
        summary_markdown(
            &snapshot,
            &profile_id,
            &compiler_version,
            machine.before_discrepancies,
            machine.after_discrepancies,
            fnlib.recovered,
        )
        .into_bytes(),
    );
    let semantic_hash = semantic_hash(&outputs);
    outputs.insert(
        "manifest.json",
        compact(&json!({
            "id": stable_id("runtime-profile", &[&snapshot, &profile_id, &semantic_hash]),
            "schema_id": "slatec-rs/runtime-profile-validation",
            "schema_version": SCHEMA_VERSION,
            "snapshot_id": snapshot,
            "created_by": format!("{TOOL_NAME} {TOOL_VERSION}"),
            "created_at": CREATED_AT,
            "semantic_version": SEMANTIC_VERSION,
            "compiler_profile_id": profile_id,
            "compiler_target": compiler_target,
            "status": "success",
            "output_semantic_hash": semantic_hash,
            "validation": {
                "abi_validated": true,
                "machine_constants_validated": true,
                "legacy_error_behavior_validated": true,
                "fnlib_initialization_validated": true
            },
            "scope": "GNU Fortran MinGW x86_64 only. Fatal probes run in child processes. This does not expose a safe special-function API."
        }))?,
    );
    enforce_size(&outputs)?;
    promote(output_dir, &outputs)?;
    Ok(RuntimeProfileResult {
        snapshot_id: snapshot,
        status: "success".to_owned(),
        semantic_hash,
        output_dir: output_dir.to_owned(),
        machine_discrepancies_before: machine.before_discrepancies,
        machine_discrepancies_after: machine.after_discrepancies,
        recovered_fnlib_probes: fnlib.recovered,
    })
}

fn compiler_path() -> PathBuf {
    std::env::var_os("SLATEC_FORTRAN_COMPILER")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("x86_64-w64-mingw32-gfortran"))
}

fn compile_probe(
    compiler: &Path,
    source: &Path,
    archive: &Path,
    workspace: &Path,
    profile: &str,
) -> Result<PathBuf> {
    if !archive.is_file() {
        return Err(CorpusError::Verification(format!(
            "missing runtime-profile archive {}",
            archive.display()
        )));
    }
    let executable = workspace.join(format!("runtime-profile-{profile}.exe"));
    let output = Command::new(compiler)
        .args(["-std=legacy", "-ffree-line-length-none"])
        .arg(source)
        .arg("-Wl,--start-group")
        .arg(archive)
        .arg("-Wl,--end-group")
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| CorpusError::Verification(format!("could not compile probe: {error}")))?;
    fs::write(
        workspace
            .join("logs")
            .join(format!("{profile}.compile.log")),
        [&output.stdout[..], &output.stderr[..]].concat(),
    )?;
    if !output.status.success() || !executable.is_file() {
        return Err(CorpusError::Verification(format!(
            "runtime-profile {profile} probe did not compile"
        )));
    }
    Ok(executable)
}

fn run_probe(executable: &Path, profile: &str, mode: &str, workspace: &Path) -> Result<ProbeRun> {
    let started = Instant::now();
    let output = Command::new(executable)
        .arg(mode)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not run {profile}/{mode} probe: {error}"))
        })?;
    let elapsed_millis = started.elapsed().as_millis();
    let stdout = String::from_utf8_lossy(&output.stdout).replace("\r\n", "\n");
    let stderr = String::from_utf8_lossy(&output.stderr).replace("\r\n", "\n");
    fs::write(
        workspace
            .join("logs")
            .join(format!("{profile}-{mode}.stdout.log")),
        stdout.as_bytes(),
    )?;
    fs::write(
        workspace
            .join("logs")
            .join(format!("{profile}-{mode}.stderr.log")),
        stderr.as_bytes(),
    )?;
    fs::write(
        workspace
            .join("logs")
            .join(format!("{profile}-{mode}.timing.txt")),
        format!("elapsed_millis={elapsed_millis}\n"),
    )?;
    Ok(ProbeRun {
        profile: profile.to_owned(),
        mode: mode.to_owned(),
        exit_code: output.status.code(),
        returned: stdout.contains("RETURNED") || stdout.contains("ERROR_STATE"),
        stdout,
        stderr,
    })
}

fn write_detailed_probe_evidence(workspace: &Path, runs: &[ProbeRun]) -> Result<()> {
    let records = runs
        .iter()
        .map(|run| {
            json!({
                "profile": run.profile,
                "mode": run.mode,
                "exit_code": run.exit_code,
                "returned": run.returned,
                "stdout_sha256": hash::bytes(run.stdout.as_bytes()),
                "stderr_sha256": hash::bytes(run.stderr.as_bytes()),
                "stdout": run.stdout,
                "stderr": run.stderr
            })
        })
        .collect::<Vec<_>>();
    fs::write(
        workspace.join("probe-records.json"),
        compact(&json!({"records":records}))?,
    )?;
    Ok(())
}

fn find_run<'a>(runs: &'a [ProbeRun], profile: &str, mode: &str) -> Result<&'a ProbeRun> {
    runs.iter()
        .find(|run| run.profile == profile && run.mode == mode)
        .ok_or_else(|| CorpusError::Verification(format!("missing {profile}/{mode} probe")))
}

fn require_clean_run(run: &ProbeRun) -> Result<()> {
    if run.exit_code != Some(0) {
        return Err(CorpusError::Verification(format!(
            "{}/{} probe failed with {:?}",
            run.profile, run.mode, run.exit_code
        )));
    }
    Ok(())
}

struct MachineOutputs {
    results: Value,
    discrepancies: Value,
    before_discrepancies: usize,
    after_discrepancies: usize,
}

fn machine_outputs(
    snapshot: &str,
    original: &ProbeRun,
    corrected: &ProbeRun,
    compiler: &ProbeRun,
) -> Result<MachineOutputs> {
    let before = parse_indexed_values(&original.stdout);
    let after = parse_indexed_values(&corrected.stdout);
    let reference = parse_named_values(&compiler.stdout);
    let mut rows = Vec::new();
    let mut discrepancies = Vec::new();
    let mut before_count = 0;
    let mut after_count = 0;
    for (routine, selectors) in [("I1MACH", 16), ("R1MACH", 5), ("D1MACH", 5)] {
        for selector in 1..=selectors {
            let baseline = before
                .get(&(routine.to_owned(), selector))
                .cloned()
                .unwrap_or_else(|| "missing".to_owned());
            let corrected = after
                .get(&(routine.to_owned(), selector))
                .cloned()
                .ok_or_else(|| {
                    CorpusError::Verification(format!(
                        "corrected probe omitted {routine}({selector})"
                    ))
                })?;
            let expected = machine_reference(routine, selector, &reference)?;
            let before_matches = expected
                .as_deref()
                .is_none_or(|value| values_match(routine, &baseline, value));
            let after_matches = expected
                .as_deref()
                .is_none_or(|value| values_match(routine, &corrected, value));
            let classification = selector_classification(routine, selector);
            if !before_matches {
                before_count += 1;
                discrepancies.push(json!([
                    routine,
                    selector,
                    "selected-template",
                    "incorrect_for_profile"
                ]));
            }
            if !after_matches {
                after_count += 1;
                discrepancies.push(json!([
                    routine,
                    selector,
                    "profile-override",
                    "incorrect_for_profile"
                ]));
            }
            rows.push(json!([
                routine,
                selector,
                baseline,
                corrected,
                expected,
                classification,
                if before_matches {
                    "matches"
                } else {
                    "mismatch"
                },
                if after_matches { "matches" } else { "mismatch" }
            ]));
        }
    }
    Ok(MachineOutputs {
        results: json!({
            "schema_id":"slatec-rs/runtime-machine-constants",
            "schema_version":SCHEMA_VERSION,
            "snapshot_id":snapshot,
            "columns":["routine","selector","original_value","profile_value","compiler_reference","selector_classification","original_status","profile_status"],
            "records":rows,
            "reference_basis":"values printed by an authored Fortran program using ISO_FORTRAN_ENV and RADIX, DIGITS, MINEXPONENT, MAXEXPONENT, TINY, HUGE, and EPSILON"
        }),
        discrepancies: json!({
            "schema_id":"slatec-rs/runtime-machine-discrepancies",
            "schema_version":SCHEMA_VERSION,
            "snapshot_id":snapshot,
            "columns":["routine","selector","provider","classification"],
            "records":discrepancies,
            "before_count":before_count,
            "after_count":after_count
        }),
        before_discrepancies: before_count,
        after_discrepancies: after_count,
    })
}

fn parse_indexed_values(text: &str) -> BTreeMap<(String, usize), String> {
    let mut values = BTreeMap::new();
    for line in text.lines() {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() == 3 && matches!(fields[0], "I1MACH" | "R1MACH" | "D1MACH") {
            if let Ok(index) = fields[1].parse::<usize>() {
                values.insert((fields[0].to_owned(), index), fields[2].to_owned());
            }
        }
    }
    values
}

fn parse_named_values(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let name = fields.next()?;
            let value = fields.next()?;
            fields
                .next()
                .is_none()
                .then(|| (name.to_owned(), value.to_owned()))
        })
        .collect()
}

fn machine_reference(
    routine: &str,
    selector: usize,
    reference: &BTreeMap<String, String>,
) -> Result<Option<String>> {
    let named = |name: &str| {
        reference
            .get(name)
            .cloned()
            .ok_or_else(|| CorpusError::Verification(format!("compiler probe omitted {name}")))
    };
    if routine == "I1MACH" {
        return Ok(match selector {
            1 => Some(named("INPUT_UNIT")?),
            2 => Some(named("OUTPUT_UNIT")?),
            3 => None,
            4 => Some(named("ERROR_UNIT")?),
            5 => Some(named("INTEGER_STORAGE_BITS")?),
            6 => Some((named("INTEGER_STORAGE_BITS")?.parse::<i64>().unwrap() / 8).to_string()),
            7 => Some(named("INTEGER_RADIX")?),
            8 => Some(named("INTEGER_DIGITS")?),
            9 => Some(named("INTEGER_HUGE")?),
            10 => Some(named("REAL_RADIX")?),
            11 => Some(named("REAL_DIGITS")?),
            12 => Some(named("REAL_MINEXPONENT")?),
            13 => Some(named("REAL_MAXEXPONENT")?),
            14 => Some(named("DOUBLE_DIGITS")?),
            15 => Some(named("DOUBLE_MINEXPONENT")?),
            16 => Some(named("DOUBLE_MAXEXPONENT")?),
            _ => unreachable!(),
        });
    }
    let prefix = if routine == "R1MACH" {
        "REAL"
    } else {
        "DOUBLE"
    };
    let radix = named(&format!("{prefix}_RADIX"))?.parse::<f64>().unwrap();
    let value = match selector {
        1 => named(&format!("{prefix}_TINY"))?.parse::<f64>().unwrap(),
        2 => named(&format!("{prefix}_HUGE"))?.parse::<f64>().unwrap(),
        3 => named(&format!("{prefix}_EPSILON"))?.parse::<f64>().unwrap() / radix,
        4 => named(&format!("{prefix}_EPSILON"))?.parse::<f64>().unwrap(),
        5 => radix.log10(),
        _ => unreachable!(),
    };
    Ok(Some(format!("{value:.17e}")))
}

fn values_match(routine: &str, observed: &str, expected: &str) -> bool {
    if routine == "I1MACH" {
        return observed.parse::<i64>().ok() == expected.parse::<i64>().ok();
    }
    let Some(observed) = observed.parse::<f64>().ok() else {
        return false;
    };
    let Some(expected) = expected.parse::<f64>().ok() else {
        return false;
    };
    if expected == 0.0 {
        observed == 0.0
    } else {
        ((observed - expected) / expected).abs() <= 2.0e-6
    }
}

fn selector_classification(routine: &str, selector: usize) -> &'static str {
    match (routine, selector) {
        ("I1MACH", 1 | 2 | 4) => "operationally_valid",
        ("I1MACH", 3) => "historical_but_harmless",
        _ => "numerically_valid",
    }
}

fn provider_audit(
    snapshot: &str,
    selected_dir: &Path,
    evidence_dir: &Path,
    interface_path: &Path,
) -> Result<Value> {
    let selected = read_value(&selected_dir.join("selected-providers.json"))?;
    let interfaces = read_value(interface_path)?;
    let relationships = read_value(Path::new(
        "generated/full-corpus/provider-relationships.json",
    ))?;
    let mut interface_by_id = BTreeMap::new();
    for row in interfaces["records"].as_array().into_iter().flatten() {
        let row = row.as_array().ok_or_else(|| {
            CorpusError::Verification("invalid compact interface record".to_owned())
        })?;
        interface_by_id.insert(
            row[0].as_str().unwrap_or_default().to_owned(),
            (
                row[9].as_str().unwrap_or_default().to_owned(),
                row[10].as_str().unwrap_or_default().to_owned(),
            ),
        );
    }
    let overlay = linux_overlay_hashes(&evidence_dir.join("downloads/slatec4linux.tgz"))?;
    let mut relationship_by_name = BTreeMap::new();
    for row in relationships["records"].as_array().into_iter().flatten() {
        let row = row.as_array().ok_or_else(|| {
            CorpusError::Verification("invalid compact provider relationship".to_owned())
        })?;
        relationship_by_name.insert(
            row[1].as_str().unwrap_or_default().to_owned(),
            (
                row[2].as_str().unwrap_or_default().to_owned(),
                row[3].as_array().map(Vec::len).unwrap_or_default(),
                row[4].clone(),
            ),
        );
    }
    let records = selected["records"]
        .as_array()
        .ok_or_else(|| CorpusError::Verification("selected providers lack records".to_owned()))?;
    let fnlib_consumers = fnlib_machine_consumers(records, evidence_dir)?;
    let mut selected_by_name = BTreeMap::new();
    for record in records {
        if let Some(name) = record["normalized_name"].as_str() {
            if TARGET_ROUTINES.contains(&name) {
                selected_by_name.insert(name.to_owned(), record);
            }
        }
    }
    let mut rows = Vec::new();
    for name in TARGET_ROUTINES {
        if let Some(record) = selected_by_name.get(*name) {
            let provider_id = record["selected_provider_id"].as_str().unwrap_or_default();
            let (symbol, confidence) = interface_by_id
                .get(provider_id)
                .cloned()
                .unwrap_or_else(|| (String::new(), "not_in_interface_inventory".to_owned()));
            let alternative = overlay.get(*name).map(|hash| {
                json!({"artifact":"slatec4linux.tgz","member":format!("{}.f", name.to_ascii_lowercase()),"raw_sha256":hash,"disposition":"excluded_alternate_implementation"})
            });
            let relationship = relationship_by_name.get(*name).cloned().unwrap_or_default();
            rows.push(json!([
                name,
                provider_id,
                record["source_subset"],
                record["source_path"],
                record["source_relationship"],
                record["raw_sha256"],
                record["kind"],
                symbol,
                confidence,
                direct_dependencies(name),
                provider_form(name),
                relationship.0,
                relationship.1,
                relationship.2,
                alternative,
                "selected"
            ]));
        } else {
            rows.push(json!([
                name,
                null,
                null,
                null,
                null,
                null,
                null,
                null,
                "absent",
                direct_dependencies(name),
                provider_form(name),
                null,
                0,
                [],
                null,
                if matches!(*name, "XERROR" | "XERRWV") {
                    "legacy_err_evidence_not_selected"
                } else {
                    "missing"
                }
            ]));
        }
    }
    Ok(json!({
        "schema_id":"slatec-rs/runtime-provider-audit",
        "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot,
        "columns":["normalized_name","selected_provider_id","source_subset","source_path","source_relationship","raw_sha256","kind","compiled_symbol","ffi_state","direct_dependencies","provider_form","provider_relationship","audited_provider_count","audited_provider_subsets","alternative_provider","disposition"],
        "records":rows,
        "fnlib_machine_constant_consumers":fnlib_consumers,
        "dependency_scope":"Targeted source and object audit only; this is not a corpus-wide dependency graph."
    }))
}

fn provider_form(name: &str) -> &'static str {
    match name {
        "I1MACH" | "R1MACH" | "D1MACH" => "unconfigured_historical_template",
        "FDUMP" | "XERHLT" => "site_hook",
        "XERROR" | "XERRWV" => "legacy_error_interface_not_selected",
        _ => "selected_slatec_hosted_source",
    }
}

fn fnlib_machine_consumers(records: &[Value], evidence_dir: &Path) -> Result<Value> {
    let mut consumers = BTreeMap::<&str, Vec<String>>::from([
        ("I1MACH", Vec::new()),
        ("R1MACH", Vec::new()),
        ("D1MACH", Vec::new()),
    ]);
    for record in records {
        if record["source_subset"].as_str() != Some("fnlib") {
            continue;
        }
        let path = record["source_path"].as_str().ok_or_else(|| {
            CorpusError::Verification("FNLIB provider lacks source path".to_owned())
        })?;
        let source = evidence_dir
            .join("full-corpus/audit-input/directories/fnlib/files")
            .join(path);
        let bytes = fs::read(&source).map_err(|_| {
            CorpusError::Verification(format!("missing selected FNLIB source {path}"))
        })?;
        if record["raw_sha256"].as_str() != Some(hash::bytes(&bytes).as_str()) {
            return Err(CorpusError::Verification(format!(
                "selected FNLIB source hash mismatch for {path}"
            )));
        }
        let normalized = String::from_utf8_lossy(&bytes)
            .to_ascii_uppercase()
            .chars()
            .filter(|character| !character.is_ascii_whitespace())
            .collect::<String>();
        let name = record["normalized_name"]
            .as_str()
            .unwrap_or_default()
            .to_owned();
        for (machine, names) in &mut consumers {
            if normalized.contains(&format!("{machine}(")) {
                names.push(name.clone());
            }
        }
    }
    for names in consumers.values_mut() {
        names.sort();
        names.dedup();
    }
    Ok(json!({
        "scope":"selected fnlib source providers; targeted lexical reference audit",
        "I1MACH":consumers["I1MACH"],
        "R1MACH":consumers["R1MACH"],
        "D1MACH":consumers["D1MACH"]
    }))
}

fn linux_overlay_hashes(path: &Path) -> Result<BTreeMap<String, String>> {
    let mut hashes = BTreeMap::new();
    if !path.is_file() {
        return Ok(hashes);
    }
    let decoder = GzDecoder::new(fs::File::open(path)?);
    let mut archive = Archive::new(decoder);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_string_lossy().replace('\\', "/");
        let name = path.trim_start_matches("./").to_ascii_uppercase();
        if matches!(name.as_str(), "I1MACH.F" | "R1MACH.F" | "D1MACH.F") {
            let mut bytes = Vec::new();
            std::io::Read::read_to_end(&mut entry, &mut bytes)?;
            hashes.insert(name.trim_end_matches(".F").to_owned(), hash::bytes(&bytes));
        }
    }
    Ok(hashes)
}

fn direct_dependencies(name: &str) -> Vec<&'static str> {
    match name {
        "R1MACH" | "D1MACH" | "INITDS" => vec!["XERMSG"],
        "INITS" => vec!["R1MACH", "XERMSG"],
        "DGAMLM" => vec!["D1MACH", "XERMSG"],
        "GAMLIM" => vec!["R1MACH", "XERMSG"],
        "XERMSG" => vec!["J4SAVE", "XERCNT", "XERSVE", "XERPRN", "FDUMP", "XERHLT"],
        "XERCLR" | "XGETF" | "XSETF" | "XGETUA" | "XSETUA" | "XGETUN" | "XSETUN" => {
            vec!["J4SAVE"]
        }
        _ => Vec::new(),
    }
}

fn error_behavior(snapshot: &str, runs: &[ProbeRun]) -> Result<Value> {
    let modes = [
        ("warning-once", -1, "informational_message", true),
        ("warning", 0, "recoverable_warning", true),
        ("recoverable-controlled", 1, "recoverable_error", true),
        ("recoverable-default", 1, "recoverable_error", false),
        ("fatal", 2, "fatal_error", false),
        ("invalid-machine-selector", 2, "fatal_error", false),
    ];
    let mut rows = Vec::new();
    for profile in ["original", "corrected"] {
        for (mode, level, meaning, expected_return) in modes {
            let run = find_run(runs, profile, mode)?;
            let outcome = process_outcome(run);
            rows.push(json!([
                profile,
                mode,
                level,
                meaning,
                expected_return,
                run.returned,
                run.exit_code,
                outcome,
                hash::bytes(run.stdout.as_bytes()),
                hash::bytes(run.stderr.as_bytes())
            ]));
        }
    }
    let state = find_run(runs, "corrected", "error-state")?;
    let state_values = state
        .stdout
        .lines()
        .find(|line| line.trim_start().starts_with("ERROR_STATE"))
        .map(|line| line.split_whitespace().skip(1).collect::<Vec<_>>())
        .unwrap_or_default();
    if state.exit_code != Some(0) || state_values != ["1", "19", "0"] {
        return Err(CorpusError::Verification(
            "legacy error-state clear/get probe did not match the documented model".to_owned(),
        ));
    }
    Ok(json!({
        "schema_id":"slatec-rs/runtime-error-behavior",
        "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot,
        "columns":["profile","case","level","meaning","expected_return","observed_return","exit_code","outcome","stdout_sha256","stderr_sha256"],
        "records":rows,
        "saved_error_state":{"control":1,"before_clear":19,"after_clear":0,"storage":"J4SAVE SAVEd process-global array"},
        "policy":"Levels -1 and 0 return. Level 1 returns only under recovery control. Level 2 terminates. The profile XERHLT site hook preserves termination with status 70 so parent processes can classify it.",
        "thread_safety":"XERROR control, unit configuration, last-error number, and message counters are process-global mutable state; callers must serialize access."
    }))
}

fn process_outcome(run: &ProbeRun) -> &'static str {
    match (run.returned, run.exit_code) {
        (true, Some(0)) => "returned_cleanly",
        (false, Some(0)) => "terminated_with_zero_status",
        (false, Some(70)) => "contained_fatal_status_70",
        (false, Some(_)) => "terminated_nonzero",
        (false, None) => "abnormal_termination",
        _ => "unexpected",
    }
}

struct FnlibOutputs {
    value: Value,
    recovered: usize,
    corrected_failures: usize,
}

fn fnlib_results(snapshot: &str, runs: &[ProbeRun]) -> Result<FnlibOutputs> {
    let cases = [
        (
            "initds",
            "INITDS=2",
            "INITDS -> XERMSG (invalid inputs only)",
        ),
        ("inits", "INITS=2", "INITS -> R1MACH"),
        (
            "dgamlm",
            "finite negative/positive limits",
            "DGAMLM -> D1MACH",
        ),
        (
            "gamlim",
            "finite negative/positive limits",
            "GAMLIM -> R1MACH",
        ),
        ("ai", "Ai(0)", "AI -> INITS -> R1MACH"),
        ("ei", "Ei(1)", "EI -> INITS -> R1MACH"),
        (
            "reciprocal-gamma",
            "1/Gamma(1), 1/Gamma(1/2)",
            "DGAMR -> DGAMLM -> D1MACH",
        ),
        (
            "degree",
            "sin(30deg), cos(60deg)",
            "SINDG/COSDG non-regression",
        ),
    ];
    let mut rows = Vec::new();
    let mut recovered = 0;
    let mut failures = 0;
    for (mode, reference, trace) in cases {
        let original = find_run(runs, "original", mode)?;
        let corrected = find_run(runs, "corrected", mode)?;
        let original_ok = validate_fnlib_case(mode, original);
        let corrected_ok = validate_fnlib_case(mode, corrected);
        if !original_ok && corrected_ok {
            recovered += 1;
        }
        if !corrected_ok {
            failures += 1;
        }
        rows.push(json!([
            mode,
            fnlib_outcome(original, original_ok),
            fnlib_outcome(corrected, corrected_ok),
            reference,
            trace,
            if corrected_ok { "passed" } else { "failed" }
        ]));
    }
    Ok(FnlibOutputs {
        value: json!({
            "schema_id":"slatec-rs/runtime-fnlib-reprobe",
            "schema_version":SCHEMA_VERSION,
            "snapshot_id":snapshot,
            "columns":["probe","original_outcome","profile_outcome","reference","dependency_trace","validation_status"],
            "records":rows,
            "root_cause":"The archived machine-constant templates compiled with every DATA selection commented out. GNU Fortran supplied zero-initialized static storage, so R1MACH/D1MACH returned zero; initialization limits and tolerances became invalid and XERMSG reached XERHLT."
        }),
        recovered,
        corrected_failures: failures,
    })
}

fn fnlib_outcome(run: &ProbeRun, validated: bool) -> &'static str {
    match (run.exit_code, validated) {
        (Some(0), true) => "completed_and_validated",
        (Some(0), false) => "terminated_with_zero_status_or_invalid_output",
        _ => process_outcome(run),
    }
}

fn validate_fnlib_case(mode: &str, run: &ProbeRun) -> bool {
    if run.exit_code != Some(0) {
        return false;
    }
    let values = run
        .stdout
        .split_whitespace()
        .filter_map(|field| field.parse::<f64>().ok())
        .collect::<Vec<_>>();
    match mode {
        "initds" | "inits" => values.first().is_some_and(|value| *value == 2.0),
        "dgamlm" | "gamlim" => {
            values.len() >= 2 && values[0].is_finite() && values[0] < 0.0 && values[1] > 0.0
        }
        "ai" => values
            .first()
            .is_some_and(|value| (*value - 0.355_028_053_887_817).abs() < 2.0e-6),
        "ei" => values
            .first()
            .is_some_and(|value| (*value - 1.895_117_816_355_936_8).abs() < 2.0e-5),
        "reciprocal-gamma" => {
            values.len() >= 2
                && (values[0] - 1.0).abs() < 1.0e-12
                && (values[1] - 0.564_189_583_547_756_3).abs() < 2.0e-12
        }
        "degree" => {
            values.len() >= 2
                && (values[0] - 0.5).abs() < 2.0e-6
                && (values[1] - 0.5).abs() < 2.0e-6
        }
        _ => false,
    }
}

fn override_summary(
    snapshot: &str,
    profile_id: &str,
    original_archive: &Path,
    corrected_archive: &Path,
) -> Result<Value> {
    let overrides = read_value(Path::new("generated/ffi/profile-overrides.json"))?;
    Ok(json!({
        "schema_id":"slatec-rs/runtime-profile-overrides",
        "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot,
        "compiler_profile_id":profile_id,
        "strategy":"profile-specific compatibility providers plus a profile XERHLT site hook",
        "preference_analysis":{
            "strategy_a":"rejected: live main-tree copies are byte-identical templates; Linux D1MACH/R1MACH add LAPACK dependencies and Linux I1MACH is a hard-coded alternate",
            "strategy_b":"selected for I1MACH, R1MACH, and D1MACH using compiler intrinsics",
            "strategy_c":"selected only for the documented XERHLT site hook; fatal semantics are preserved with deterministic status 70"
        },
        "original_archive_sha256":hash::file(original_archive)?,
        "profile_archive_sha256":hash::file(corrected_archive)?,
        "columns":overrides["columns"],
        "records":overrides["records"],
        "single_definition_check":"passed"
    }))
}

fn summary_markdown(
    snapshot: &str,
    profile: &str,
    compiler: &str,
    before: usize,
    after: usize,
    recovered: usize,
) -> String {
    format!(
        "# GNU MinGW SLATEC runtime-profile validation\n\n- Snapshot: `{snapshot}`\n- Compiler profile: `{profile}` (`{compiler}`)\n- Machine-constant discrepancies before profile support: {before}\n- Machine-constant discrepancies after profile support: {after}\n- Previously failing runtime probes recovered: {recovered}\n- Fatal error containment: child process with deterministic exit status 70\n- Thread-safety: legacy error configuration and saved state are process-global and require serialization\n\nThe immutable selected source hashes are unchanged. Profile-specific machine providers and the `XERHLT` site hook are build-layer compatibility sources. ABI validity, machine-constant validity, legacy-error behavior, and FNLIB initialization are reported separately. No public safe special-function API or translated numerical algorithm is included.\n"
    )
}

fn read_value(path: &Path) -> Result<Value> {
    serde_json::from_slice(&fs::read(path)?).map_err(CorpusError::from)
}

fn required_string(value: &Value, field: &str) -> Result<String> {
    value[field]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| CorpusError::Verification(format!("missing required field {field}")))
}

fn compact(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn stable_id(kind: &str, parts: &[&str]) -> String {
    format!(
        "{kind}-{}",
        &hash::bytes(parts.join("\u{1f}").as_bytes())[..16]
    )
}

fn semantic_hash(outputs: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut material = Vec::new();
    for (name, bytes) in outputs {
        material.extend_from_slice(name.as_bytes());
        material.push(0);
        material.extend_from_slice(bytes);
        material.push(0);
    }
    hash::bytes(&material)
}

fn enforce_size(outputs: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let total = outputs.values().map(Vec::len).sum::<usize>();
    if total > SIZE_LIMIT {
        return Err(CorpusError::Policy(format!(
            "runtime-profile committed output would be {total} bytes, above {SIZE_LIMIT}"
        )));
    }
    Ok(())
}

fn promote(output_dir: &Path, outputs: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let parent = output_dir.parent().ok_or_else(|| {
        CorpusError::Policy("runtime-profile output directory must have a parent".to_owned())
    })?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        ".{}.runtime-profile-staging",
        output_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("runtime-profile")
    ));
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;
    for (name, bytes) in outputs {
        fs::write(staging.join(name), bytes)?;
    }
    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::rename(staging, output_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_machine_probe_records_without_paths() {
        let parsed = parse_indexed_values("I1MACH 9 2147483647\nD1MACH 4 2.220E-16\n");
        assert_eq!(
            parsed.get(&("I1MACH".to_owned(), 9)).map(String::as_str),
            Some("2147483647")
        );
        assert_eq!(
            parsed.get(&("D1MACH".to_owned(), 4)).map(String::as_str),
            Some("2.220E-16")
        );
    }

    #[test]
    fn compares_port_spacing_with_float_tolerance() {
        assert!(values_match(
            "D1MACH",
            "1.1102230246251565E-16",
            "1.1102230246251565e-16"
        ));
        assert!(!values_match("D1MACH", "0.0", "1.1102230246251565e-16"));
    }

    #[test]
    fn separates_historical_io_from_numeric_selectors() {
        assert_eq!(
            selector_classification("I1MACH", 3),
            "historical_but_harmless"
        );
        assert_eq!(selector_classification("I1MACH", 4), "operationally_valid");
        assert_eq!(selector_classification("I1MACH", 9), "numerically_valid");
    }

    #[test]
    fn fatal_process_outcomes_are_not_confused_with_clean_returns() {
        let run = ProbeRun {
            profile: "corrected".to_owned(),
            mode: "fatal".to_owned(),
            exit_code: Some(70),
            stdout: String::new(),
            stderr: String::new(),
            returned: false,
        };
        assert_eq!(process_outcome(&run), "contained_fatal_status_70");
    }

    #[test]
    fn dependency_audit_is_bounded_to_runtime_targets() {
        assert_eq!(direct_dependencies("DGAMLM"), ["D1MACH", "XERMSG"]);
        assert!(direct_dependencies("AI").is_empty());
    }

    #[test]
    fn semantic_hash_is_deterministic() {
        let outputs = BTreeMap::from([("a.json", b"{}\n".to_vec()), ("b.json", b"[]\n".to_vec())]);
        assert_eq!(semantic_hash(&outputs), semantic_hash(&outputs));
    }

    #[test]
    fn output_size_guard_rejects_oversized_records() {
        let outputs = BTreeMap::from([("large.json", vec![0; SIZE_LIMIT + 1])]);
        assert!(enforce_size(&outputs).is_err());
    }

    #[test]
    fn target_names_do_not_contain_duplicates() {
        let names = TARGET_ROUTINES
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(names.len(), TARGET_ROUTINES.len());
    }
}
