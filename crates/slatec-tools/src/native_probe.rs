//! Isolated native feasibility probe.
//!
//! This module compiles a deliberately small, selected-provider pilot and
//! records compiler and symbol observations.  It is not a native component
//! builder and does not generate production FFI bindings.

use crate::error::{CorpusError, Result};
use crate::hash;
use crate::{TOOL_NAME, TOOL_VERSION};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA_VERSION: &str = "1.0.0";
const SEMANTIC_VERSION: &str = "1";
const CREATED_AT: &str = "1970-01-01T00:00:00Z";
const COMMITTED_SIZE_LIMIT: u64 = 1_000_000;

#[derive(Clone, Deserialize)]
struct Provider {
    snapshot_id: String,
    program_unit_id: String,
    normalized_name: String,
    kind: String,
    source_subset: String,
    source_path: String,
    raw_sha256: String,
    normalized_sha256: String,
}

#[derive(Deserialize)]
struct ProviderRecords {
    snapshot_id: String,
    records: Vec<Provider>,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct NativeProbeResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
}

#[derive(Clone)]
struct Pilot<'a> {
    role: &'a str,
    name: &'a str,
}

const PILOT_SET: &[Pilot<'static>] = &[
    Pilot {
        role: "scalar_function",
        name: "DASUM",
    },
    Pilot {
        role: "machine_constant",
        name: "D1MACH",
    },
    Pilot {
        role: "simple_array_subroutine",
        name: "DAXPY",
    },
    Pilot {
        role: "character_argument_and_error_handling",
        name: "XERMSG",
    },
    Pilot {
        role: "complex_valued_function",
        name: "CABS",
    },
    Pilot {
        role: "external_callback",
        name: "DFZERO",
    },
];

/// Compile only the representative, selected Fortran pilot under one explicit
/// compiler profile.  Failure is recorded as review evidence; source/hash or
/// provider mismatches remain fatal.
pub fn probe(
    evidence_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<NativeProbeResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "probe-native-ffi only uses verified local evidence and requires --offline".to_owned(),
        ));
    }
    let providers: ProviderRecords =
        read_json(&selected_corpus_dir.join("selected-providers.json"))?;
    let snapshot = providers.snapshot_id.clone();
    let selected_manifest: Value = read_json(&selected_corpus_dir.join("manifest.json"))?;
    let main_src_snapshot = selected_manifest["main_src_snapshot_id"]
        .as_str()
        .ok_or_else(|| {
            CorpusError::Verification(
                "selected-corpus manifest lacks the immutable main-src snapshot ID".to_owned(),
            )
        })?
        .to_owned();
    let selection_hash = hash::file(&selected_corpus_dir.join("selected-providers.json"))?;
    let selected = provider_map(providers.records, &snapshot)?;
    let compiler = compiler_path();
    let compiler_version = command_output(&compiler, &["--version"])?;
    let compiler_target = command_output(&compiler, &["-dumpmachine"])?;
    let workspace = evidence_dir
        .join("native-probe")
        .join(&snapshot)
        .join("mingw-gfortran-pilot");
    fs::create_dir_all(&workspace)?;
    let mut source_rows = Vec::new();
    let mut result_rows = Vec::new();
    let mut symbols = Vec::new();
    let mut seen_source = BTreeSet::new();
    let mut dasum_object = None;
    for pilot in PILOT_SET {
        let provider = selected.get(pilot.name).ok_or_else(|| {
            CorpusError::Verification(format!(
                "selected corpus lacks required native pilot {}",
                pilot.name
            ))
        })?;
        let source = source_path(evidence_dir, &main_src_snapshot, provider)?;
        let bytes = fs::read(&source).map_err(|_| {
            CorpusError::Verification(format!(
                "missing selected native-pilot evidence {}",
                source.display()
            ))
        })?;
        if hash::bytes(&bytes) != provider.raw_sha256 {
            return Err(CorpusError::Verification(format!(
                "native-pilot source hash mismatch for {}",
                provider.normalized_name
            )));
        }
        if seen_source.insert((
            provider.source_subset.clone(),
            provider.source_path.clone(),
            provider.raw_sha256.clone(),
        )) {
            source_rows.push(json!([
                provider.source_subset,
                provider.source_path,
                provider.raw_sha256,
                provider.normalized_sha256
            ]));
        }
        let object = workspace.join(format!(
            "{}.o",
            provider.normalized_name.to_ascii_lowercase()
        ));
        let log = workspace.join(format!(
            "{}.compile.log",
            provider.normalized_name.to_ascii_lowercase()
        ));
        let result = Command::new(&compiler)
            .args(["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"])
            .arg(&source)
            .arg("-o")
            .arg(&object)
            .output()
            .map_err(|error| {
                CorpusError::Verification(format!("could not start native compiler: {error}"))
            })?;
        fs::write(&log, [&result.stdout[..], &result.stderr[..]].concat())?;
        let object_hash = if result.status.success() && object.is_file() {
            Some(hash::file(&object)?)
        } else {
            None
        };
        if pilot.name == "DASUM" && object_hash.is_some() {
            dasum_object = Some(object.clone());
        }
        let log_hash = hash::file(&log)?;
        let symbol_rows = if object_hash.is_some() {
            inspect_symbols(&object, provider, &workspace, &compiler)?
        } else {
            Vec::new()
        };
        for row in &symbol_rows {
            symbols.push(row.clone());
        }
        result_rows.push(json!([
            pilot.role,
            provider.program_unit_id,
            provider.normalized_name,
            provider.kind,
            provider.source_subset,
            provider.source_path,
            result.status.code(),
            if result.status.success() {
                "compiled"
            } else {
                "failed"
            },
            if result.status.success() {
                Value::Null
            } else {
                Value::String("compiler_exit_nonzero".to_owned())
            },
            object_hash,
            log_hash,
            symbol_rows.len()
        ]));
    }
    source_rows.sort_by(|left, right| {
        left[0]
            .as_str()
            .cmp(&right[0].as_str())
            .then(left[1].as_str().cmp(&right[1].as_str()))
    });
    result_rows.sort_by(|left, right| left[0].as_str().cmp(&right[0].as_str()));
    symbols.sort_by(|left, right| {
        left[0]
            .as_str()
            .cmp(&right[0].as_str())
            .then(left[2].as_str().cmp(&right[2].as_str()))
    });
    let all_compiled = result_rows
        .iter()
        .all(|row| row[6].as_i64() == Some(0) && row[9].as_str().is_some());
    let rust_probe = match dasum_object {
        Some(object) => raw_rust_probe(&workspace, &compiler, &object)?,
        None => RawRustProbe::unavailable("DASUM did not compile"),
    };
    write_outputs(
        output_dir,
        &snapshot,
        &selection_hash,
        &compiler,
        &compiler_version,
        &compiler_target,
        source_rows,
        result_rows,
        symbols,
        all_compiled,
        rust_probe,
    )
}

#[derive(Clone)]
struct RawRustProbe {
    status: String,
    rust_target: String,
    raw_symbol: String,
    compile_log_sha256: Option<String>,
    run_log_sha256: Option<String>,
}

impl RawRustProbe {
    fn unavailable(reason: &str) -> Self {
        Self {
            status: format!("unavailable:{reason}"),
            rust_target: "x86_64-pc-windows-gnu".to_owned(),
            raw_symbol: "dasum_".to_owned(),
            compile_log_sha256: None,
            run_log_sha256: None,
        }
    }
}

fn raw_rust_probe(workspace: &Path, compiler: &Path, object: &Path) -> Result<RawRustProbe> {
    let source = workspace.join("dasum_raw_ffi_probe.rs");
    let executable = workspace.join("dasum_raw_ffi_probe.exe");
    let compile_log = workspace.join("dasum_raw_ffi_probe.compile.log");
    let run_log = workspace.join("dasum_raw_ffi_probe.run.log");
    let source_text = r#"#[link(name = "gfortran")]
unsafe extern "C" {
    #[link_name = "dasum_"]
    fn dasum(length: *const i32, values: *const f64, increment: *const i32) -> f64;
}

fn main() {
    let length = 3_i32;
    let increment = 1_i32;
    let values = [1.0_f64, -2.0, 3.0];
    let value = unsafe { dasum(&length, values.as_ptr(), &increment) };
    if value != 6.0 {
        std::process::exit(2);
    }
}
"#;
    fs::write(&source, source_text)?;
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let compile = Command::new(rustc)
        .arg("--edition=2024")
        .arg("--target")
        .arg("x86_64-pc-windows-gnu")
        .arg(&source)
        .arg("-C")
        .arg(format!("linker={}", compiler.display()))
        .arg("-C")
        .arg(format!("link-arg={}", object.display()))
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not start Rust ABI probe: {error}"))
        })?;
    fs::write(
        &compile_log,
        [&compile.stdout[..], &compile.stderr[..]].concat(),
    )?;
    let compile_hash = hash::file(&compile_log)?;
    if !compile.status.success() || !executable.is_file() {
        return Ok(RawRustProbe {
            status: "compile_failed".to_owned(),
            rust_target: "x86_64-pc-windows-gnu".to_owned(),
            raw_symbol: "dasum_".to_owned(),
            compile_log_sha256: Some(compile_hash),
            run_log_sha256: None,
        });
    }
    let run = Command::new(&executable).output().map_err(|error| {
        CorpusError::Verification(format!("could not start compiled Rust ABI probe: {error}"))
    })?;
    fs::write(&run_log, [&run.stdout[..], &run.stderr[..]].concat())?;
    Ok(RawRustProbe {
        status: if run.status.success() {
            "passed"
        } else {
            "run_failed"
        }
        .to_owned(),
        rust_target: "x86_64-pc-windows-gnu".to_owned(),
        raw_symbol: "dasum_".to_owned(),
        compile_log_sha256: Some(compile_hash),
        run_log_sha256: Some(hash::file(&run_log)?),
    })
}

fn provider_map(providers: Vec<Provider>, snapshot: &str) -> Result<BTreeMap<String, Provider>> {
    if providers.len() != 1_476 {
        return Err(CorpusError::Verification(
            "native feasibility must use the exact 1476-provider selected manifest".to_owned(),
        ));
    }
    let mut map = BTreeMap::new();
    for provider in providers {
        if provider.snapshot_id != snapshot
            || map
                .insert(provider.normalized_name.clone(), provider)
                .is_some()
        {
            return Err(CorpusError::Verification(
                "selected-provider identities are not unique for native feasibility".to_owned(),
            ));
        }
    }
    Ok(map)
}

fn compiler_path() -> PathBuf {
    std::env::var_os("SLATEC_FORTRAN_COMPILER")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("x86_64-w64-mingw32-gfortran"))
}

fn source_path(
    evidence_dir: &Path,
    main_src_snapshot: &str,
    provider: &Provider,
) -> Result<PathBuf> {
    match provider.source_subset.as_str() {
        "main-src" => Ok(evidence_dir
            .join("extracted")
            .join(main_src_snapshot)
            .join("slatec-source-archive")
            .join(&provider.source_path)),
        "lin" | "fishfft" | "fnlib" | "pchip" => Ok(evidence_dir
            .join("full-corpus/audit-input/directories")
            .join(&provider.source_subset)
            .join("files")
            .join(&provider.source_path)),
        "spfun" => Ok(evidence_dir
            .join("full-corpus/audit-input/supplemental")
            .join("spfun")),
        _ => Err(CorpusError::Verification(format!(
            "no native-pilot evidence rule for selected subset {}",
            provider.source_subset
        ))),
    }
}

fn command_output(program: &Path, arguments: &[&str]) -> Result<String> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not run {}: {error}", program.display()))
        })?;
    if !output.status.success() {
        return Err(CorpusError::Verification(format!(
            "{} {:?} failed",
            program.display(),
            arguments
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or_default()
        .trim()
        .to_owned())
}

fn inspect_symbols(
    object: &Path,
    provider: &Provider,
    workspace: &Path,
    compiler: &Path,
) -> Result<Vec<Value>> {
    let nm = compiler_tool(compiler, "nm")?;
    let output = Command::new(&nm)
        .args(["-g", "--defined-only"])
        .arg(object)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not inspect native symbols: {error}"))
        })?;
    let log = workspace.join(format!(
        "{}.nm.log",
        provider.normalized_name.to_ascii_lowercase()
    ));
    fs::write(log, [&output.stdout[..], &output.stderr[..]].concat())?;
    if !output.status.success() {
        return Err(CorpusError::Verification(format!(
            "symbol inspection failed for {}",
            provider.normalized_name
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let _address = fields.next()?;
            let kind = fields.next()?;
            let raw = fields.next()?;
            Some(json!([
                provider.program_unit_id,
                provider.normalized_name,
                raw,
                kind,
                normalize_symbol(raw)
            ]))
        })
        .collect())
}

fn compiler_tool(compiler: &Path, tool: &str) -> Result<PathBuf> {
    let output = Command::new(compiler)
        .arg(format!("-print-prog-name={tool}"))
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not locate compiler tool {tool}: {error}"))
        })?;
    if !output.status.success() {
        return Err(CorpusError::Verification(format!(
            "compiler could not locate tool {tool}"
        )));
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if path.is_empty() {
        return Err(CorpusError::Verification(format!(
            "compiler returned an empty path for tool {tool}"
        )));
    }
    Ok(PathBuf::from(path))
}

fn normalize_symbol(raw: &str) -> String {
    raw.trim_start_matches('_')
        .trim_end_matches('_')
        .to_ascii_uppercase()
}

#[allow(clippy::too_many_arguments)]
fn write_outputs(
    output_dir: &Path,
    snapshot: &str,
    selection_hash: &str,
    compiler: &Path,
    compiler_version: &str,
    compiler_target: &str,
    source_rows: Vec<Value>,
    result_rows: Vec<Value>,
    symbols: Vec<Value>,
    all_compiled: bool,
    rust_probe: RawRustProbe,
) -> Result<NativeProbeResult> {
    let status = "success_with_review_items";
    let mut outputs = BTreeMap::<&str, Vec<u8>>::new();
    outputs.insert("selected-source-files.json", compact(&json!({"schema_id":"slatec-rs/native-pilot-source-list","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"columns":["source_subset","source_path","raw_sha256","normalized_sha256"],"records":source_rows}))?);
    outputs.insert("compilation-results.json", compact(&json!({"schema_id":"slatec-rs/native-pilot-compilation","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"compiler_profile":{"compiler":compiler,"version":compiler_version,"target":compiler_target,"flags":["-x","f77","-std=legacy","-ffixed-line-length-none","-c"]},"columns":["role","program_unit_id","normalized_name","kind","source_subset","source_path","exit_code","compilation_status","failure_rule","object_sha256","diagnostic_log_sha256","defined_symbol_count"],"records":result_rows}))?);
    outputs.insert("symbol-inventory.json", compact(&json!({"schema_id":"slatec-rs/native-pilot-symbol","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"normalization_rule":"remove one optional leading and trailing underscore; preserve raw spelling as authoritative","columns":["program_unit_id","normalized_name","raw_symbol","nm_kind","profile_normalized_symbol"],"records":symbols}))?);
    outputs.insert("raw-rust-probe.json", compact(&json!({"schema_id":"slatec-rs/native-raw-rust-probe","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"routine":"DASUM","raw_symbol":rust_probe.raw_symbol,"rust_target":rust_probe.rust_target,"status":rust_probe.status,"compile_log_sha256":rust_probe.compile_log_sha256,"run_log_sha256":rust_probe.run_log_sha256,"scope":"one scalar DOUBLE PRECISION function call with default INTEGER scalar pointers and a contiguous DOUBLE PRECISION array"}))?);
    let semantic_hash = semantic_hash(&outputs);
    outputs.insert("manifest.json", compact(&json!({"id":stable_id("native-feasibility", &[snapshot,&semantic_hash]),"schema_id":"slatec-rs/native-feasibility","schema_version":SCHEMA_VERSION,"snapshot_id":snapshot,"created_by":format!("{TOOL_NAME} {TOOL_VERSION}"),"created_at":CREATED_AT,"semantic_version":SEMANTIC_VERSION,"selected_provider_manifest_sha256":selection_hash,"output_semantic_hash":semantic_hash,"status":status,"all_pilot_sources_compiled":all_compiled,"raw_rust_ffi_probe_status":rust_probe.status,"abi_assumptions":{"symbol_spelling":"observed in object inventory","fortran_default_integer_width":"not established by this probe","character_hidden_length":"not established","complex_return":"not established","callback_abi":"not established"}}))?);
    outputs.insert("feasibility-summary.md", format!("# Native Fortran feasibility pilot\n\n- Snapshot: `{snapshot}`\n- Compiler: `{}`\n- Compiler target: `{compiler_target}`\n- Profile flags: `-x f77 -std=legacy -ffixed-line-length-none -c`\n- Representative pilot sources compiled: {}\n- All pilot sources compiled: `{all_compiled}`\n- Isolated `DASUM` raw Rust FFI smoke probe: `{}`\n\nThis is an isolated compiler, object-symbol, and scalar raw-call observation. It does not prove dependency closure, callback ABI, character hidden-length ABI, complex-return ABI, safe API calling, component linking, or a production raw FFI contract.\n", compiler.display(), PILOT_SET.len(), rust_probe.status).into_bytes());
    let total = outputs
        .values()
        .map(|value| value.len() as u64)
        .sum::<u64>();
    if total > COMMITTED_SIZE_LIMIT {
        return Err(CorpusError::Verification(format!(
            "native feasibility output would be {total} bytes, above the compact-output limit"
        )));
    }
    promote(output_dir, snapshot, &outputs)?;
    Ok(NativeProbeResult {
        snapshot_id: snapshot.to_owned(),
        status: status.to_owned(),
        semantic_hash,
        output_dir: output_dir.to_owned(),
    })
}

fn read_json<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(path)?).map_err(CorpusError::from)
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
fn promote(output_dir: &Path, snapshot: &str, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let parent = output_dir.parent().ok_or_else(|| {
        CorpusError::Policy("native feasibility output directory must have a parent".to_owned())
    })?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        "{}.staging-{snapshot}",
        output_dir.file_name().unwrap_or_default().to_string_lossy()
    ));
    if staging.exists() {
        return Err(CorpusError::Verification(format!(
            "native feasibility staging directory exists: {}",
            staging.display()
        )));
    }
    fs::create_dir(&staging)?;
    for (name, bytes) in files {
        fs::write(staging.join(name), bytes)?;
    }
    if output_dir.exists() {
        let identical = files.iter().all(|(name, bytes)| {
            fs::read(output_dir.join(name)).ok().as_deref() == Some(bytes.as_slice())
        });
        if identical {
            fs::remove_dir_all(staging)?;
            return Ok(());
        }
        let backup = parent.join(format!(
            "{}.previous-{snapshot}",
            output_dir.file_name().unwrap_or_default().to_string_lossy()
        ));
        if backup.exists() {
            return Err(CorpusError::Verification(format!(
                "native feasibility backup directory exists: {}",
                backup.display()
            )));
        }
        fs::rename(output_dir, &backup)?;
        if let Err(error) = fs::rename(&staging, output_dir) {
            let _ = fs::rename(&backup, output_dir);
            return Err(CorpusError::from(error));
        }
        fs::remove_dir_all(backup)?;
    } else {
        fs::rename(staging, output_dir)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalizes_only_documented_underscore_decoration() {
        assert_eq!(normalize_symbol("d1mach_"), "D1MACH");
        assert_eq!(normalize_symbol("_d1mach_"), "D1MACH");
        assert_eq!(normalize_symbol("__d1mach"), "D1MACH");
    }

    #[test]
    fn pilot_set_covers_each_targeted_interface_risk_without_a_corpus_download() {
        let roles = PILOT_SET
            .iter()
            .map(|pilot| pilot.role)
            .collect::<BTreeSet<_>>();
        for role in [
            "scalar_function",
            "machine_constant",
            "simple_array_subroutine",
            "character_argument_and_error_handling",
            "complex_valued_function",
            "external_callback",
        ] {
            assert!(roles.contains(role));
        }
        assert_eq!(RawRustProbe::unavailable("test").raw_symbol, "dasum_");
    }
}
