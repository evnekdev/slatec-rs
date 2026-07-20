//! Reproducible audit of native archive granularity and final-link closure.
//!
//! This is deliberately a development-only measurement tool.  It rebuilds the
//! same source-to-object-to-archive shape as `slatec-src` in `target/`, then
//! commits only compact, deterministic metadata under `generated/native-link`.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const TARGET: &str = "x86_64-pc-windows-gnu";
const SOURCE_FLAGS: &[&str] = &["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"];
const OVERLAYS: &[(&str, &str)] = &[
    ("ode-sdrive-expert", "ode-sdrive-source-closure.json"),
    ("dassl", "dassl-source-closure.json"),
    (
        "optimization-linear-programming-in-memory",
        "lp-in-memory-source-closure.json",
    ),
    ("fftpack-real", "fftpack-real-source-closure.json"),
    ("fftpack-complex", "fftpack-complex-source-closure.json"),
    (
        "fishpack-cartesian-2d",
        "fishpack-cartesian-2d-source-closure.json",
    ),
    ("fishpack-pois3d", "fishpack-pois3d-source-closure.json"),
    (
        "banded-linear-systems",
        "banded-linear-systems-source-closure.json",
    ),
    ("pchip", "pchip-source-closure.json"),
    ("bspline", "bspline-source-closure.json"),
    (
        "piecewise-polynomial",
        "piecewise-polynomial-source-closure.json",
    ),
    (
        "special-scalar-expanded",
        "special-scalar-expanded-source-closure.json",
    ),
];

#[derive(Clone, Deserialize)]
struct Source {
    id: String,
    subset: String,
    path: String,
    sha256: String,
}

#[derive(Deserialize)]
struct Manifest {
    snapshot_id: String,
    sources: Vec<Source>,
    families: BTreeMap<String, Vec<String>>,
    profile_override_families: BTreeSet<String>,
}

#[derive(Deserialize)]
struct Overlay {
    family: String,
    source_ids: Vec<String>,
    sources: Vec<Source>,
    profile_override: bool,
}

#[derive(Clone)]
struct Member {
    id: String,
    source: PathBuf,
    source_display: String,
    source_hash: String,
    families: BTreeSet<String>,
    object: PathBuf,
}

/// Generate the archive inventory and independently linked release probes.
///
/// The local source cache and GNU MinGW compiler are intentionally explicit:
/// this command never downloads source or silently substitutes another ABI.
pub fn generate(output_dir: &Path) -> Result<()> {
    let root = PathBuf::from(".");
    let cache = env_path("SLATEC_SOURCE_CACHE")?;
    let compiler = env::var_os("SLATEC_GFORTRAN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("gfortran"));
    check_output(&compiler, &["-dumpmachine"], "GNU Fortran target")?;
    let target = output_text(&compiler, &["-dumpmachine"])?;
    if target.trim() != "x86_64-w64-mingw32" {
        return Err(CorpusError::Verification(format!(
            "native-link audit requires GNU Fortran target x86_64-w64-mingw32; found {}",
            target.trim()
        )));
    }

    let mut manifest = read_manifest(&root)?;
    let members = selected_members(&root, &cache, &mut manifest)?;
    let work = root.join("target/native-link");
    let objects = work.join("objects");
    fs::create_dir_all(&objects)?;
    let ar = sibling_tool(&compiler, "ar");
    let nm = sibling_tool(&compiler, "nm");
    let size = sibling_tool(&compiler, "size");
    let objdump = sibling_tool(&compiler, "objdump");

    for member in &members {
        compile(&compiler, &member.source, &member.object)?;
    }
    let archive = work.join("libslatec_native_link.a");
    archive_members(&ar, &archive, &members)?;
    let archived = archive_list(&ar, &archive)?;
    if archived.len() != members.len() {
        return Err(CorpusError::Verification(format!(
            "archive member count {} did not match compiled object count {}",
            archived.len(),
            members.len()
        )));
    }

    let mut inventory = Vec::new();
    let mut definitions = BTreeMap::<String, Vec<String>>::new();
    let mut dependencies = BTreeMap::<String, Vec<String>>::new();
    let mut all_slatec_symbols = BTreeSet::new();
    let archive_defined = archive_symbols(&nm, &archive, true)?;
    let archive_undefined = archive_symbols(&nm, &archive, false)?;
    let archive_sizes = archive_sections(&size, &archive)?;
    for member in &members {
        let name = member.object_name();
        let defined = archive_defined.get(&name).cloned().unwrap_or_default();
        let undefined = archive_undefined.get(&name).cloned().unwrap_or_default();
        let (code, data, bss) = archive_sizes.get(&name).copied().unwrap_or_default();
        let units = scan_program_units(&member.source)?;
        let classification = classify_member(&units, &member.source_display);
        for symbol in &defined {
            let normalized = normalize_symbol(symbol);
            definitions
                .entry(normalized.clone())
                .or_default()
                .push(name.clone());
            all_slatec_symbols.insert(normalized);
        }
        dependencies.insert(name.clone(), undefined.clone());
        inventory.push(json!({
            "archive_path": "target/native-link/libslatec_native_link.a",
            "archive_member": name,
            "originating_source_file": member.source_display,
            "source_sha256": member.source_hash,
            "program_units_defined": units,
            "classification": classification,
            "global_symbols_defined": defined,
            "undefined_symbols_referenced": undefined,
            "code_size": code,
            "data_size": data,
            "bss_size": bss,
            "families": member.families,
            "feature_closure": member.families,
        }));
    }
    inventory.sort_by(|a, b| {
        a["archive_member"]
            .as_str()
            .cmp(&b["archive_member"].as_str())
    });
    for values in definitions.values_mut() {
        values.sort();
        values.dedup();
    }

    let graph = symbol_graph(&dependencies, &definitions);
    let probe = build_probes(
        &root,
        &work,
        &archive,
        &compiler,
        &nm,
        &size,
        &objdump,
        &all_slatec_symbols,
        &definitions,
    )?;
    let pipeline = json!({
        "source_selection": "family-source-closure.json plus reviewed family overlay files",
        "compiler_command": ["gfortran", "-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c", "SOURCE", "-o", "OBJECT"],
        "archive_commands": [[tool_name(&ar), "cr", "ARCHIVE", "OBJECT..."], [tool_name(&ar), "q", "ARCHIVE", "OBJECT..."], [tool_name(&ar), "s", "ARCHIVE"]],
        "rust_link_directives": ["cargo:rustc-link-search=native=OUT_DIR", "cargo:rustc-link-lib=static=slatec_family", "cargo:rustc-link-lib=static=gfortran", "cargo:rustc-link-lib=static=quadmath"],
        "partial_linking": false,
        "whole_archive": false,
        "generated_amalgamation": false,
        "object_count": members.len(),
        "archive_member_count": archived.len(),
        "archive_bytes": fs::metadata(&archive)?.len(),
    });
    let toolchain = json!({
        "compiler": "gfortran",
        "compiler_version": first_line(&output_text(&compiler, &["--version"])?),
        "compiler_target": target.trim(),
        "archiver": tool_name(&ar),
        "archiver_version": first_line(&output_text(&ar, &["--version"])?),
        "nm": tool_name(&nm),
        "size": size.display().to_string(),
        "objdump": tool_name(&objdump),
    });
    write_reports(
        output_dir,
        &manifest.snapshot_id,
        &pipeline,
        &toolchain,
        inventory,
        graph,
        probe,
    )?;
    Ok(())
}

/// Check the generated audit without compiling or linking native code.
pub fn validate(output_dir: &Path) -> Result<()> {
    let members: Value = read_json(&output_dir.join("archive-members.json"))?;
    let manifest: Value = read_json(&output_dir.join("manifest.json"))?;
    let probe: Value = read_json(&output_dir.join("probe-results.json"))?;
    let rows = members["records"]
        .as_array()
        .ok_or_else(|| bad("archive-members records"))?;
    let object_count = manifest["pipeline"]["object_count"]
        .as_u64()
        .ok_or_else(|| bad("object count"))?;
    let archive_count = manifest["pipeline"]["archive_member_count"]
        .as_u64()
        .ok_or_else(|| bad("archive count"))?;
    if rows.len() as u64 != object_count || object_count != archive_count {
        return Err(CorpusError::Verification(
            "native archive is no longer one member per compiled object".to_owned(),
        ));
    }
    if manifest["pipeline"]["whole_archive"] != Value::Bool(false)
        || manifest["pipeline"]["partial_linking"] != Value::Bool(false)
    {
        return Err(CorpusError::Verification(
            "native-link pipeline uses a forbidden coalescing mode".to_owned(),
        ));
    }
    for name in [
        "raw_all_no_call",
        "safe_all_no_call",
        "raw_saxpy_only",
        "safe_saxpy_only",
        "raw_ddot_only",
        "safe_ddot_only",
        "raw_dgemv_only",
        "safe_dgemv_only",
        "raw_dgemm_only",
        "safe_dgemm_only",
        "raw_dgamma_only",
        "safe_special_only",
        "safe_roots_only",
        "safe_fishpack_hwscrt_only",
    ] {
        let row = probe["records"]
            .as_array()
            .and_then(|rows| rows.iter().find(|row| row["name"] == name))
            .ok_or_else(|| CorpusError::Verification(format!("missing {name} probe")))?;
        if row["assertions_passed"] != Value::Bool(true) {
            return Err(CorpusError::Verification(format!(
                "native-link assertions failed for {name}"
            )));
        }
    }
    Ok(())
}

impl Member {
    fn object_name(&self) -> String {
        self.object
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    }
}

fn read_manifest(root: &Path) -> Result<Manifest> {
    let path = root.join("crates/slatec-src/metadata/family-source-closure.json");
    let mut manifest: Manifest = read_json(&path)?;
    for (family, file) in OVERLAYS {
        if manifest.families.contains_key(*family) {
            continue;
        }
        let overlay: Overlay = read_json(&root.join("crates/slatec-src/metadata").join(file))?;
        if overlay.family != *family {
            return Err(CorpusError::Verification(format!(
                "unexpected family in {file}"
            )));
        }
        for source in overlay.sources {
            if let Some(existing) = manifest.sources.iter().find(|item| item.id == source.id) {
                if existing.path != source.path
                    || existing.subset != source.subset
                    || existing.sha256 != source.sha256
                {
                    return Err(CorpusError::Verification(format!(
                        "source metadata conflict for {}",
                        source.id
                    )));
                }
            } else {
                manifest.sources.push(source);
            }
        }
        manifest
            .families
            .insert(overlay.family.clone(), overlay.source_ids);
        if overlay.profile_override {
            manifest.profile_override_families.insert(overlay.family);
        }
    }
    Ok(manifest)
}

fn selected_members(root: &Path, cache: &Path, manifest: &mut Manifest) -> Result<Vec<Member>> {
    let by_id = manifest
        .sources
        .iter()
        .cloned()
        .map(|source| (source.id.clone(), source))
        .collect::<BTreeMap<_, _>>();
    let mut source_families = BTreeMap::<String, BTreeSet<String>>::new();
    for (family, ids) in &manifest.families {
        for id in ids {
            source_families
                .entry(id.clone())
                .or_default()
                .insert(family.clone());
        }
    }
    let mut members = Vec::new();
    for (id, families) in source_families {
        let source = by_id
            .get(&id)
            .ok_or_else(|| CorpusError::Verification(format!("missing source {id}")))?;
        let relative = if source.subset == "main-src" {
            source.path.clone()
        } else {
            format!("{}/{}", source.subset, source.path)
        };
        let path = cache.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
        verify_source(&path, &source.sha256, &id)?;
        members.push(Member {
            id: id.clone(),
            source: path,
            source_display: relative,
            source_hash: source.sha256.clone(),
            families,
            object: root
                .join("target/native-link/objects")
                .join(format!("{id}.o")),
        });
    }
    if !manifest.profile_override_families.is_empty() {
        for name in ["i1mach", "r1mach", "d1mach"] {
            let source = root
                .join("crates/slatec-src/native/gnu-mingw-x86_64")
                .join(format!("{name}.f"));
            let hash = hash::file(&source)?;
            members.push(Member {
                id: format!("profile-{name}"),
                source,
                source_display: format!("crates/slatec-src/native/gnu-mingw-x86_64/{name}.f"),
                source_hash: hash,
                families: manifest.profile_override_families.clone(),
                object: root
                    .join("target/native-link/objects")
                    .join(format!("profile-{name}.o")),
            });
        }
    }
    members.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(members)
}

fn verify_source(path: &Path, expected: &str, id: &str) -> Result<()> {
    if !path.is_file() {
        return Err(CorpusError::Verification(format!(
            "source cache lacks {id}: {}",
            path.display()
        )));
    }
    let actual = hash::file(path)?;
    if actual != expected {
        return Err(CorpusError::Verification(format!(
            "source cache hash mismatch for {id}"
        )));
    }
    Ok(())
}

fn compile(compiler: &Path, source: &Path, object: &Path) -> Result<()> {
    // Source hashes are verified before this point. Keeping an already-built
    // object makes same-root regeneration practical; a clean target directory
    // still performs the complete one-source-per-object build.
    if object.is_file() {
        return Ok(());
    }
    let output = Command::new(compiler)
        .args(SOURCE_FLAGS)
        .arg(source)
        .arg("-o")
        .arg(object)
        .output()?;
    if !output.status.success() {
        return Err(command_error("GNU Fortran compilation", output));
    }
    Ok(())
}

fn archive_members(ar: &Path, archive: &Path, members: &[Member]) -> Result<()> {
    if archive.exists() {
        fs::remove_file(archive)?;
    }
    for (index, chunk) in members.chunks(40).enumerate() {
        let output = Command::new(ar)
            .arg(if index == 0 { "cr" } else { "q" })
            .arg(archive)
            .args(chunk.iter().map(|member| &member.object))
            .output()?;
        if !output.status.success() {
            return Err(command_error("GNU ar archive", output));
        }
    }
    let output = Command::new(ar).arg("s").arg(archive).output()?;
    if !output.status.success() {
        return Err(command_error("GNU ar indexing", output));
    }
    Ok(())
}

fn archive_list(ar: &Path, archive: &Path) -> Result<Vec<String>> {
    let text = output_text(ar, &["t", &archive.display().to_string()])?;
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect())
}

fn archive_symbols(
    nm: &Path,
    archive: &Path,
    defined: bool,
) -> Result<BTreeMap<String, Vec<String>>> {
    let mut command = Command::new(nm);
    if defined {
        command.args(["-A", "-g", "--defined-only"]);
    } else {
        command.args(["-A", "-u"]);
    }
    let output = command.arg(archive).output()?;
    if !output.status.success() {
        return Err(command_error("GNU nm archive inspection", output));
    }
    let prefix = format!("{}:", archive.display());
    let mut result = BTreeMap::<String, Vec<String>>::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Some(rest) = line.strip_prefix(&prefix) else {
            continue;
        };
        let Some((member, fields)) = rest.split_once(':') else {
            continue;
        };
        let Some(symbol) = fields.split_whitespace().last() else {
            continue;
        };
        result
            .entry(member.to_owned())
            .or_default()
            .push(symbol.to_owned());
    }
    for symbols in result.values_mut() {
        symbols.sort();
        symbols.dedup();
    }
    Ok(result)
}

fn archive_sections(size: &Path, archive: &Path) -> Result<BTreeMap<String, (u64, u64, u64)>> {
    let output = Command::new(size).args(["-A"]).arg(archive).output()?;
    if !output.status.success() {
        return Err(command_error("GNU size archive inspection", output));
    }
    let mut result = BTreeMap::<String, (u64, u64, u64)>::new();
    let mut current = None::<String>;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Some((member, _)) = line.split_once(" (ex ") {
            current = member.split_whitespace().next().map(str::to_owned);
            continue;
        }
        let Some(member) = current.as_ref() else {
            continue;
        };
        let mut words = line.split_whitespace();
        let Some(section) = words.next() else {
            continue;
        };
        let Some(bytes) = words.next().and_then(|value| value.parse::<u64>().ok()) else {
            continue;
        };
        let entry = result.entry(member.clone()).or_default();
        if section.starts_with(".text") {
            entry.0 += bytes;
        } else if section.starts_with(".bss") {
            entry.2 += bytes;
        } else if section.starts_with(".data") || section.starts_with(".rdata") {
            entry.1 += bytes;
        }
    }
    Ok(result)
}

fn symbols(nm: &Path, object: &Path, defined: bool) -> Result<Vec<String>> {
    let mut command = Command::new(nm);
    if defined {
        command.args(["-g", "--defined-only"]);
    } else {
        command.args(["-u"]);
    }
    let output = command.arg(object).output()?;
    if !output.status.success() {
        return Err(command_error("GNU nm", output));
    }
    let mut values = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().last())
        .filter(|symbol| !symbol.ends_with(':'))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    Ok(values)
}

/// Returns compact demangled Rust symbols retained in an executable.  This is
/// deliberately a classification aid rather than a brittle full-symbol
/// snapshot: compiler-generated hashes and standard-library symbols are
/// excluded from committed evidence.
fn demangled_rust_symbols(nm: &Path, object: &Path) -> Result<Vec<String>> {
    let output = Command::new(nm)
        .args(["-C", "-g", "--defined-only"])
        .arg(object)
        .output()?;
    if !output.status.success() {
        return Err(command_error("GNU nm Rust symbol inspection", output));
    }
    let mut values = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_once(" T ").map(|(_, symbol)| symbol.trim()))
        .filter(|symbol| symbol.contains("slatec::") || symbol.contains("slatec_sys::"))
        .map(str::to_owned)
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    Ok(values)
}

fn rust_symbol_groups(symbols: &[String]) -> Value {
    let mut safe_wrappers = Vec::new();
    let mut raw_bindings = Vec::new();
    let mut shared_helpers = Vec::new();
    let mut unrelated = Vec::new();
    for symbol in symbols {
        if symbol.contains("slatec_sys::") {
            raw_bindings.push(symbol.clone());
        } else if symbol.contains("::validation::") || symbol.contains("::runtime::") {
            shared_helpers.push(symbol.clone());
        } else if symbol.contains("::blas::")
            || symbol.contains("::special::")
            || symbol.contains("::roots::")
            || symbol.contains("::differential_equations::")
        {
            safe_wrappers.push(symbol.clone());
        } else {
            unrelated.push(symbol.clone());
        }
    }
    json!({
        "safe_wrappers": safe_wrappers,
        "raw_binding_references": raw_bindings,
        "shared_safety_or_runtime_helpers": shared_helpers,
        "unclassified_slatec_symbols": unrelated,
    })
}

fn sections(size: &Path, object: &Path) -> Result<(u64, u64, u64)> {
    let output = Command::new(size).args(["-A"]).arg(object).output()?;
    if !output.status.success() {
        return Err(command_error("GNU size", output));
    }
    let mut code = 0;
    let mut data = 0;
    let mut bss = 0;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let mut words = line.split_whitespace();
        let Some(section) = words.next() else {
            continue;
        };
        let Some(bytes) = words.next().and_then(|value| value.parse::<u64>().ok()) else {
            continue;
        };
        if section.starts_with(".text") {
            code += bytes;
        } else if section.starts_with(".bss") {
            bss += bytes;
        } else if section.starts_with(".data") || section.starts_with(".rdata") {
            data += bytes;
        }
    }
    Ok((code, data, bss))
}

fn scan_program_units(path: &Path) -> Result<Vec<Value>> {
    let text = fs::read_to_string(path)?;
    let mut units = Vec::new();
    for raw in text.lines() {
        if raw
            .as_bytes()
            .first()
            .is_some_and(|byte| matches!(*byte, b'c' | b'C' | b'*' | b'!'))
        {
            continue;
        }
        let line = raw
            .get(6..)
            .unwrap_or(raw)
            .trim_start()
            .to_ascii_uppercase();
        let words = line.split_whitespace().collect::<Vec<_>>();
        let (kind, name) = if let ["BLOCK", "DATA", name, ..] = words.as_slice() {
            ("block-data", *name)
        } else if let Some(index) = words.iter().position(|word| *word == "SUBROUTINE") {
            let Some(name) = words.get(index + 1) else {
                continue;
            };
            ("subroutine", *name)
        } else if let Some(index) = words.iter().position(|word| *word == "FUNCTION") {
            let Some(name) = words.get(index + 1) else {
                continue;
            };
            ("function", *name)
        } else if let ["PROGRAM", name, ..] = words.as_slice() {
            ("program", *name)
        } else {
            continue;
        };
        let clean = name.split('(').next().unwrap_or(name).trim_end_matches(',');
        if !clean.is_empty() {
            units.push(json!({"name": clean, "kind": kind}));
        }
    }
    units.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    units.dedup();
    Ok(units)
}

fn classify_member(units: &[Value], source: &str) -> &'static str {
    if units.iter().any(|unit| unit["kind"] == "block-data") {
        return "block-data";
    }
    if units.len() == 1 {
        return "single-routine";
    }
    let stem = Path::new(source)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_ascii_uppercase();
    if units
        .iter()
        .any(|unit| unit["name"].as_str() == Some(stem.as_str()))
    {
        "principal-routine-plus-subsidiaries"
    } else if units.len() > 1 {
        "multiple-related-routines"
    } else {
        "unknown"
    }
}

fn symbol_graph(
    dependencies: &BTreeMap<String, Vec<String>>,
    definitions: &BTreeMap<String, Vec<String>>,
) -> Value {
    let records = dependencies.iter().map(|(member, undefined)| {
        let edges = undefined.iter().map(|raw| { let normalized = normalize_symbol(raw); json!({"symbol": raw, "providers": definitions.get(&normalized).cloned().unwrap_or_default()}) }).collect::<Vec<_>>();
        json!({"archive_member": member, "undefined_edges": edges})
    }).collect::<Vec<_>>();
    json!({"schema_id":"slatec-rs/native-link-symbol-graph","normalization_rule":"remove optional leading and trailing underscore and uppercase","records":records})
}

#[allow(clippy::too_many_arguments)]
fn build_probes(
    root: &Path,
    work: &Path,
    archive: &Path,
    compiler: &Path,
    nm: &Path,
    size: &Path,
    objdump: &Path,
    all_slatec: &BTreeSet<String>,
    definitions: &BTreeMap<String, Vec<String>>,
) -> Result<Value> {
    let runtime_dirs = runtime_dirs(compiler)?;
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let specs = probe_specs();
    let probe_dir = work.join("probes");
    fs::create_dir_all(&probe_dir)?;
    let mut rows = Vec::new();
    let mut symbol_rows = Vec::new();
    let mut size_rows = Vec::new();
    for spec in specs {
        let source = probe_dir.join(format!("{}.rs", spec.name));
        let executable = probe_dir.join(format!("{}.exe", spec.name));
        let map = probe_dir.join(format!("{}.map", spec.name));
        fs::write(&source, spec.source)?;
        let mut command = Command::new(&rustc);
        command
            .args([
                "--edition=2021",
                "--target",
                TARGET,
                "-O",
                "-C",
                &format!("linker={}", sibling_tool(compiler, "gcc").display()),
                "-C",
                "link-arg=-Wl,--gc-sections",
                "-C",
                &format!("link-arg=-Wl,-Map,{}", map.display()),
            ])
            .arg(&source)
            .arg("-o")
            .arg(&executable);
        if spec.link_slatec {
            command
                .arg("-L")
                .arg(format!("native={}", archive.parent().unwrap().display()))
                .arg("-l")
                .arg("static=slatec_native_link");
            for directory in &runtime_dirs {
                command
                    .arg("-L")
                    .arg(format!("native={}", directory.display()));
            }
            command
                .arg("-l")
                .arg("static=gfortran")
                .arg("-l")
                .arg("static=quadmath");
        }
        let output = command.output()?;
        if !output.status.success() {
            return Err(command_error(
                &format!("release probe {}", spec.name),
                output,
            ));
        }
        let defined = symbols(nm, &executable, true)?;
        let normalized = defined
            .iter()
            .map(|symbol| normalize_symbol(symbol))
            .collect::<BTreeSet<_>>();
        let slatec = normalized
            .intersection(all_slatec)
            .cloned()
            .collect::<BTreeSet<_>>();
        let selected_members = selected_from_map(&map, definitions);
        let (text, data, bss) = sections(size, &executable)?;
        let imported_dlls = dlls(objdump, &executable)?;
        let assertions = probe_assertions(spec.name, &slatec, all_slatec);
        let passed = assertions
            .iter()
            .all(|entry| entry["passed"] == Value::Bool(true));
        symbol_rows.push(json!({"name":spec.name,"defined_slatec_symbols":slatec,"archive_members_selected":selected_members,"imported_dlls":imported_dlls}));
        size_rows.push(json!({"name":spec.name,"text_size":text,"data_size":data,"bss_size":bss}));
        rows.push(json!({"name":spec.name,"kind":spec.kind,"requested_symbols":spec.requested,"link_slatec_archive":spec.link_slatec,"assertions":assertions,"assertions_passed":passed}));
    }
    for (row, symbols, sizes) in cargo_probes(
        root,
        work,
        compiler,
        nm,
        size,
        objdump,
        all_slatec,
        definitions,
    )? {
        rows.push(row);
        symbol_rows.push(symbols);
        size_rows.push(sizes);
    }
    rows.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    symbol_rows.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    size_rows.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    Ok(
        json!({"records":rows,"symbols":symbol_rows,"sizes":size_rows,"runtime_link_policy":"static libgfortran and libquadmath, matching slatec-src source-build"}),
    )
}

struct Probe {
    name: &'static str,
    kind: &'static str,
    requested: &'static [&'static str],
    link_slatec: bool,
    source: &'static str,
}
fn probe_specs() -> Vec<Probe> {
    vec![
        Probe {
            name: "baseline_no_slatec",
            kind: "baseline",
            requested: &[],
            link_slatec: false,
            source: "fn main(){ println!(\"{}\", std::hint::black_box(1_u64)); }",
        },
        Probe {
            name: "raw_saxpy_only",
            kind: "raw",
            requested: &["SAXPY"],
            link_slatec: true,
            source: RAW_SAXPY,
        },
        Probe {
            name: "raw_ddot_only",
            kind: "raw",
            requested: &["DDOT"],
            link_slatec: true,
            source: RAW_DDOT,
        },
        Probe {
            name: "raw_saxpy_ddot",
            kind: "raw",
            requested: &["SAXPY", "DDOT"],
            link_slatec: true,
            source: RAW_SAXPY_DDOT,
        },
        Probe {
            name: "raw_dgemv_only",
            kind: "raw",
            requested: &["DGEMV"],
            link_slatec: true,
            source: RAW_DGEMV,
        },
        Probe {
            name: "raw_dgemm_only",
            kind: "raw",
            requested: &["DGEMM"],
            link_slatec: true,
            source: RAW_DGEMM,
        },
        Probe {
            name: "raw_dgamma_only",
            kind: "raw",
            requested: &["DGAMMA"],
            link_slatec: true,
            source: RAW_DGAMMA,
        },
        Probe {
            name: "raw_fzero_only",
            kind: "raw-address",
            requested: &["FZERO"],
            link_slatec: true,
            source: RAW_FZERO,
        },
        Probe {
            name: "raw_hwscrt_only",
            kind: "raw-address",
            requested: &["HWSCRT"],
            link_slatec: true,
            source: RAW_HWSCRT,
        },
        Probe {
            name: "raw_pois3d_only",
            kind: "raw-address",
            requested: &["POIS3D"],
            link_slatec: true,
            source: RAW_POIS3D,
        },
        Probe {
            name: "all_saxpy_only",
            kind: "complete-source-archive",
            requested: &["SAXPY"],
            link_slatec: true,
            source: RAW_SAXPY,
        },
    ]
}
const RAW_SAXPY: &str = "unsafe extern \"C\" { fn saxpy_(n:*const i32,a:*const f32,x:*const f32,ix:*const i32,y:*mut f32,iy:*const i32); } fn main(){let n=2;let a=2.0;let x=[1.0,2.0];let mut y=[3.0,4.0];let one=1;unsafe{saxpy_(&n,&a,x.as_ptr(),&one,y.as_mut_ptr(),&one)};println!(\"{}\",std::hint::black_box(y[0]+y[1]));}";
const RAW_DDOT: &str = "unsafe extern \"C\" { fn ddot_(n:*const i32,x:*const f64,ix:*const i32,y:*const f64,iy:*const i32)->f64; } fn main(){let n=2;let one=1;let x=[1.0,2.0];let y=[3.0,4.0];println!(\"{}\",std::hint::black_box(unsafe{ddot_(&n,x.as_ptr(),&one,y.as_ptr(),&one)}));}";
const RAW_SAXPY_DDOT: &str = "unsafe extern \"C\" { fn saxpy_(n:*const i32,a:*const f32,x:*const f32,ix:*const i32,y:*mut f32,iy:*const i32); fn ddot_(n:*const i32,x:*const f64,ix:*const i32,y:*const f64,iy:*const i32)->f64; } fn main(){let n=1;let one=1;let a=2.0;let x=[1.0];let mut y=[3.0];unsafe{saxpy_(&n,&a,x.as_ptr(),&one,y.as_mut_ptr(),&one)};let dx=[1.0];let dy=[2.0];println!(\"{}\",std::hint::black_box(y[0] as f64+unsafe{ddot_(&n,dx.as_ptr(),&one,dy.as_ptr(),&one)}));}";
const RAW_DGEMV: &str = "use core::ffi::c_char; unsafe extern \"C\" { fn dgemv_(t:*const c_char,m:*const i32,n:*const i32,a:*const f64,x:*const f64,ld:*const i32,v:*const f64,ix:*const i32,b:*const f64,y:*mut f64,iy:*const i32,l:usize); } fn main(){let t=b'N' as c_char;let m=1;let n=1;let one=1;let a=1.0;let b=0.0;let x=[2.0];let v=[3.0];let mut y=[0.0];unsafe{dgemv_(&t,&m,&n,&a,x.as_ptr(),&one,v.as_ptr(),&one,&b,y.as_mut_ptr(),&one,1)};println!(\"{}\",std::hint::black_box(y[0]));}";
const RAW_DGEMM: &str = "use core::ffi::c_char; unsafe extern \"C\" { fn dgemm_(ta:*const c_char,tb:*const c_char,m:*const i32,n:*const i32,k:*const i32,a:*const f64,x:*const f64,lda:*const i32,y:*const f64,ldb:*const i32,b:*const f64,z:*mut f64,ldc:*const i32,la:usize,lb:usize); } fn main(){let t=b'N' as c_char;let one=1;let a=1.0;let b=0.0;let x=[2.0];let y=[3.0];let mut z=[0.0];unsafe{dgemm_(&t,&t,&one,&one,&one,&a,x.as_ptr(),&one,y.as_ptr(),&one,&b,z.as_mut_ptr(),&one,1,1)};println!(\"{}\",std::hint::black_box(z[0]));}";
const RAW_DGAMMA: &str = "unsafe extern \"C\" { fn dgamma_(x:*const f64)->f64; } fn main(){let x=0.5;println!(\"{}\",std::hint::black_box(unsafe{dgamma_(&x)}));}";
const RAW_FZERO: &str = "unsafe extern \"C\" { fn fzero_(); } fn main(){println!(\"{}\",std::hint::black_box(fzero_ as usize));}";
const RAW_HWSCRT: &str = "unsafe extern \"C\" { fn hwscrt_(); } fn main(){println!(\"{}\",std::hint::black_box(hwscrt_ as usize));}";
const RAW_POIS3D: &str = "unsafe extern \"C\" { fn pois3d_(); } fn main(){println!(\"{}\",std::hint::black_box(pois3d_ as usize));}";

#[allow(clippy::too_many_arguments)]
fn cargo_probes(
    root: &Path,
    work: &Path,
    compiler: &Path,
    nm: &Path,
    size: &Path,
    objdump: &Path,
    all_slatec: &BTreeSet<String>,
    definitions: &BTreeMap<String, Vec<String>>,
) -> Result<Vec<(Value, Value, Value)>> {
    let specs = [
        (
            "safe_saxpy_only",
            "link_blas_level1_saxpy",
            "source-build,blas-level1",
            "safe",
        ),
        (
            "safe_ddot_only",
            "link_blas_level1",
            "source-build,blas-level1",
            "safe",
        ),
        (
            "safe_dgemv_only",
            "link_blas_level2_gemv",
            "source-build,blas-level2",
            "safe",
        ),
        (
            "safe_dgemm_only",
            "link_blas_level3",
            "source-build,blas-level3",
            "safe",
        ),
        (
            "safe_special_only",
            "link_gamma",
            "source-build,special-gamma",
            "safe",
        ),
        (
            "safe_roots_only",
            "link_roots_scalar",
            "source-build,roots-scalar",
            "safe",
        ),
        (
            "safe_fishpack_hwscrt_only",
            "fishpack_cartesian_2d",
            "source-build,fishpack-cartesian-2d",
            "safe",
        ),
        (
            "raw_all_no_call",
            "raw_all_features_compile",
            "source-build,raw-all-link-tests",
            "all-no-call",
        ),
        (
            "safe_all_no_call",
            "link_safe_all_no_call",
            "source-build,full",
            "all-no-call",
        ),
    ];
    let cache = env_path("SLATEC_SOURCE_CACHE")?;
    let mut rows = Vec::new();
    for (name, example, features, kind) in specs {
        let target_dir = work.join("cargo-probes").join(name);
        let map = work.join("probes").join(format!("{name}.map"));
        let output = Command::new("cargo")
            .current_dir(root)
            .args([
                "build",
                "-p",
                "slatec",
                "--example",
                example,
                "--release",
                "--no-default-features",
                "--features",
                features,
                "--target",
                TARGET,
                "--target-dir",
            ])
            .arg(&target_dir)
            .env("SLATEC_SOURCE_CACHE", &cache)
            .env("SLATEC_GFORTRAN", compiler)
            .env(
                "RUSTFLAGS",
                format!("-C link-arg=-Wl,-Map,{}", map.display()),
            )
            .env("CARGO_TERM_COLOR", "never")
            .output()?;
        if !output.status.success() {
            return Err(command_error(
                &format!("Cargo release probe {name}"),
                output,
            ));
        }
        let executable = target_dir
            .join(TARGET)
            .join("release/examples")
            .join(format!("{example}.exe"));
        if !executable.is_file() {
            return Err(CorpusError::Verification(format!(
                "Cargo release probe {name} did not produce {}",
                executable.display()
            )));
        }
        let defined = symbols(nm, &executable, true)?;
        let normalized = defined
            .iter()
            .map(|symbol| normalize_symbol(symbol))
            .collect::<BTreeSet<_>>();
        let slatec = normalized
            .intersection(all_slatec)
            .cloned()
            .collect::<BTreeSet<_>>();
        let rust = demangled_rust_symbols(nm, &executable)?;
        let rust_groups = rust_symbol_groups(&rust);
        let (text, data, bss) = sections(size, &executable)?;
        let assertions = probe_assertions(name, &slatec, all_slatec);
        let passed = assertions
            .iter()
            .all(|entry| entry["passed"] == Value::Bool(true));
        rows.push((
            json!({"name":name,"kind":kind,"requested_symbols":requested_for_cargo_probe(name),"link_slatec_archive":true,"assertions":assertions,"assertions_passed":passed}),
            json!({"name":name,"defined_slatec_symbols":slatec,"archive_members_selected":selected_from_map(&map,definitions),"defined_rust_symbols":rust,"rust_symbol_groups":rust_groups,"imported_dlls":dlls(objdump,&executable)?}),
            json!({"name":name,"text_size":text,"data_size":data,"bss_size":bss}),
        ));
    }
    Ok(rows)
}

fn requested_for_cargo_probe(name: &str) -> &'static [&'static str] {
    match name {
        "safe_saxpy_only" => &["SAXPY"],
        "safe_ddot_only" => &["DDOT"],
        "safe_dgemv_only" => &["DGEMV"],
        "safe_dgemm_only" => &["DGEMM"],
        "safe_special_only" => &["DGAMMA"],
        "safe_roots_only" => &["FZERO"],
        "safe_fishpack_hwscrt_only" => &["HWSCRT"],
        _ => &[],
    }
}

fn selected_from_map(map: &Path, definitions: &BTreeMap<String, Vec<String>>) -> BTreeSet<String> {
    let Ok(text) = fs::read_to_string(map) else {
        return BTreeSet::new();
    };
    definitions
        .values()
        .flatten()
        .filter(|member| text.contains(member.as_str()))
        .cloned()
        .collect()
}
fn dlls(objdump: &Path, executable: &Path) -> Result<Vec<String>> {
    let text = output_text(objdump, &["-p", &executable.display().to_string()])?;
    let mut rows = text
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("DLL Name:")
                .map(str::trim)
                .map(str::to_owned)
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows.dedup();
    Ok(rows)
}
fn probe_assertions(name: &str, slatec: &BTreeSet<String>, all: &BTreeSet<String>) -> Vec<Value> {
    let contains = |s: &str| slatec.contains(s);
    let excludes = |xs: &[&str]| xs.iter().all(|s| !contains(s));
    match name {
        "raw_all_no_call" | "safe_all_no_call" => vec![
            json!({"rule":"all no-call links no SLATEC implementation symbol","passed":slatec.is_empty(),"observed":slatec}),
        ],
        "raw_saxpy_only" | "all_saxpy_only" | "safe_saxpy_only" => vec![
            json!({"rule":"contains SAXPY","passed":contains("SAXPY")}),
            json!({"rule":"excludes unrelated BLAS, special, roots, and FISHPACK drivers","passed":excludes(&["SDOT","SNRM2","SGEMM","DGAMMA","FZERO","HWSCRT","POIS3D"]),"observed":slatec}),
        ],
        "raw_dgamma_only" => vec![
            json!({"rule":"contains DGAMMA","passed":contains("DGAMMA")}),
            json!({"rule":"excludes unrelated BLAS, roots, and FISHPACK drivers","passed":excludes(&["SAXPY","DDOT","SGEMM","FZERO","HWSCRT","POIS3D"]),"observed":slatec}),
        ],
        "safe_ddot_only" => vec![
            json!({"rule":"contains DDOT","passed":contains("DDOT")} ),
            json!({"rule":"excludes unrelated BLAS and other family drivers","passed":excludes(&["SAXPY","SDOT","SNRM2","SSCAL","SGEMM","DGAMMA","FZERO","HWSCRT","POIS3D"]),"observed":slatec}),
        ],
        "raw_dgemv_only" | "safe_dgemv_only" => vec![
            json!({"rule":"contains DGEMV","passed":contains("DGEMV")} ),
            json!({"rule":"excludes unrelated Level 2/3 and other family drivers","passed":excludes(&["SGEMV","DTRSV","SGEMM","DGEMM","SAXPY","DGAMMA","FZERO","HWSCRT"]),"observed":slatec}),
        ],
        "raw_dgemm_only" | "safe_dgemm_only" => vec![
            json!({"rule":"contains DGEMM","passed":contains("DGEMM")} ),
            json!({"rule":"excludes unrelated Level 2/3 and other family drivers","passed":excludes(&["SGEMM","DTRMM","DTRSM","DGEMV","SAXPY","DGAMMA","FZERO","HWSCRT"]),"observed":slatec}),
        ],
        "safe_special_only" => vec![
            json!({"rule":"contains DGAMMA","passed":contains("DGAMMA")} ),
            json!({"rule":"excludes unrelated BLAS, roots, and FISHPACK drivers","passed":excludes(&["SAXPY","DDOT","DGEMM","FZERO","HWSCRT","POIS3D"]),"observed":slatec}),
        ],
        "safe_roots_only" => vec![
            json!({"rule":"contains FZERO","passed":contains("FZERO")} ),
            json!({"rule":"excludes unrelated BLAS, special, and FISHPACK drivers","passed":excludes(&["SAXPY","DDOT","DGEMM","DGAMMA","HWSCRT","POIS3D"]),"observed":slatec}),
        ],
        "safe_fishpack_hwscrt_only" => vec![
            json!({"rule":"contains HWSCRT","passed":contains("HWSCRT")} ),
            json!({"rule":"excludes unrelated BLAS, special, roots, and POIS3D drivers","passed":excludes(&["SAXPY","DDOT","DGEMM","DGAMMA","FZERO","POIS3D"]),"observed":slatec}),
        ],
        _ => {
            let requested = match name {
                "raw_ddot_only" => "DDOT",
                "raw_saxpy_ddot" => "SAXPY",
                "raw_fzero_only" => "FZERO",
                "raw_hwscrt_only" => "HWSCRT",
                "raw_pois3d_only" => "POIS3D",
                _ => "",
            };
            vec![
                json!({"rule":"requested symbol is retained","passed":requested.is_empty() || contains(requested),"observed":slatec,"known_archive_symbols":all.len()}),
            ]
        }
    }
}

fn runtime_dirs(compiler: &Path) -> Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    for name in ["libgfortran.a", "libquadmath.a"] {
        let path =
            PathBuf::from(output_text(compiler, &[&format!("-print-file-name={name}")])?.trim());
        if !path.is_file() {
            return Err(CorpusError::Verification(format!("compiler lacks {name}")));
        }
        dirs.push(path.parent().unwrap().to_path_buf());
    }
    dirs.sort();
    dirs.dedup();
    Ok(dirs)
}
fn write_reports(
    output: &Path,
    snapshot: &str,
    pipeline: &Value,
    toolchain: &Value,
    members: Vec<Value>,
    graph: Value,
    probe: Value,
) -> Result<()> {
    fs::create_dir_all(output)?;
    let records = probe["records"].clone();
    let symbols = probe["symbols"].clone();
    let sizes = probe["sizes"].clone();
    write_json(
        &output.join("archive-members.json"),
        &json!({"schema_id":"slatec-rs/native-link-archive-members","snapshot_id":snapshot,"records":members}),
    )?;
    write_json(&output.join("archive-symbol-graph.json"), &graph)?;
    write_json(
        &output.join("probe-results.json"),
        &json!({"schema_id":"slatec-rs/native-link-probe-results","snapshot_id":snapshot,"records":records}),
    )?;
    write_json(
        &output.join("probe-symbols.json"),
        &json!({"schema_id":"slatec-rs/native-link-probe-symbols","snapshot_id":snapshot,"records":symbols}),
    )?;
    write_json(
        &output.join("probe-sizes.json"),
        &json!({"schema_id":"slatec-rs/native-link-probe-sizes","snapshot_id":snapshot,"records":sizes}),
    )?;
    write_json(
        &output.join("manifest.json"),
        &json!({"schema_id":"slatec-rs/native-link","schema_version":"1.0.0","snapshot_id":snapshot,"pipeline":pipeline,"toolchain":toolchain,"determinism":"committed reports exclude timestamps, paths outside the workspace, logs, binaries, maps, objects, archives, and elapsed-time measurements"}),
    )?;
    let member_count = pipeline["archive_member_count"].as_u64().unwrap_or(0);
    fs::write(
        output.join("archive-summary.md"),
        format!(
            "# Native archive summary\n\n- Archive members: `{member_count}`\n- Compiled objects: `{}`\n- Archive bytes: `{}`\n- Partial linking (`ld -r`): `false`\n- Whole-archive linking: `false`\n- Generated amalgamated source: `false`\n- Compiler command: `gfortran -x f77 -std=legacy -ffixed-line-length-none -c SOURCE -o OBJECT`\n- Archive construction: `ar cr`, chunked `ar q`, then `ar s`\n\n`slatec-sys` itself emits no native link directive. `slatec-src` provides the ordinary static archive and static GNU Fortran runtime libraries. Archive extraction, rather than the number of Rust declarations or enabled API features, determines which native members reach a final executable.\n",
            pipeline["object_count"], pipeline["archive_bytes"]
        ),
    )?;
    let comparison = probe["sizes"].as_array().unwrap();
    let mut text = String::from(
        "# Native link probe comparison\n\nSizes are release-probe section measurements. They are diagnostic measurements, not exact-byte regression thresholds. Executable file bytes are deliberately omitted because PE linker layout can vary by a few bytes between clean roots.\n\n| Probe | .text | .data/.rdata | .bss |\n|---|---:|---:|---:|\n",
    );
    for row in comparison {
        text.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            row["name"].as_str().unwrap_or("?"),
            row["text_size"],
            row["data_size"],
            row["bss_size"]
        ));
    }
    text.push_str("\nThe static GNU runtime is selected only when a referenced SLATEC member needs it; archive and runtime on-disk sizes are not final-executable contributions. Link maps in `target/native-link/probes` provide the per-probe selected-member evidence.\n\nDirect raw probes are the source-archive granularity regression baseline. Safe-facade probes are reported separately: a broad safe Rust compilation unit can retain a broader raw symbol set before the archive extractor runs, even though the Fortran archive itself remains one source per member. The symbol report makes that distinction explicit rather than treating safe-wrapper size as evidence of archive coalescing.\n");
    fs::write(output.join("probe-comparison.md"), text)?;
    write_safe_facade_comparison(output, &probe)?;
    Ok(())
}

fn write_safe_facade_comparison(output: &Path, probe: &Value) -> Result<()> {
    let records = probe["records"]
        .as_array()
        .ok_or_else(|| bad("probe records"))?;
    let symbols = probe["symbols"]
        .as_array()
        .ok_or_else(|| bad("probe symbols"))?;
    let sizes = probe["sizes"]
        .as_array()
        .ok_or_else(|| bad("probe sizes"))?;
    let pairs = [
        ("raw_saxpy_only", "safe_saxpy_only"),
        ("raw_ddot_only", "safe_ddot_only"),
        ("raw_dgemv_only", "safe_dgemv_only"),
        ("raw_dgemm_only", "safe_dgemm_only"),
        ("raw_dgamma_only", "safe_special_only"),
        ("raw_fzero_only", "safe_roots_only"),
        ("raw_hwscrt_only", "safe_fishpack_hwscrt_only"),
    ];
    let mut comparisons = Vec::new();
    for (raw_name, safe_name) in pairs {
        let Some(raw_symbols) = find_named(symbols, raw_name) else {
            continue;
        };
        let Some(safe_symbols) = find_named(symbols, safe_name) else {
            continue;
        };
        let raw_set = raw_symbols["defined_slatec_symbols"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        let safe_set = safe_symbols["defined_slatec_symbols"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        let safe_only = safe_set.difference(&raw_set).cloned().collect::<Vec<_>>();
        comparisons.push(json!({
            "raw_probe": raw_name,
            "safe_probe": safe_name,
            "raw_native_symbols": raw_set,
            "safe_native_symbols": safe_set,
            "safe_only_native_symbols": safe_only,
            "safe_rust_symbols": safe_symbols["defined_rust_symbols"].clone(),
            "safe_rust_symbol_groups": safe_symbols["rust_symbol_groups"].clone(),
            "raw_selected_archive_members": raw_symbols["archive_members_selected"].clone(),
            "safe_selected_archive_members": safe_symbols["archive_members_selected"].clone(),
            "raw_size": find_named(sizes, raw_name).cloned().unwrap_or(Value::Null),
            "safe_size": find_named(sizes, safe_name).cloned().unwrap_or(Value::Null),
            "raw_assertions_passed": find_named(records, raw_name).map(|value| value["assertions_passed"].clone()).unwrap_or(Value::Null),
            "safe_assertions_passed": find_named(records, safe_name).map(|value| value["assertions_passed"].clone()).unwrap_or(Value::Null),
        }));
    }
    comparisons.sort_by(|left, right| {
        left["safe_probe"]
            .as_str()
            .cmp(&right["safe_probe"].as_str())
    });
    write_json(
        &output.join("safe-facade-comparison.json"),
        &json!({
            "schema_id": "slatec-rs/safe-facade-link-comparison",
            "policy": "A safe call may retain its wrapper, checked-safety helpers, provider anchor, and genuine native closure, but not unrelated numerical operations merely because wrappers share a Rust compilation unit.",
            "records": comparisons,
        }),
    )?;
    let mut markdown = String::from(
        "# Safe-facade versus raw link comparison\n\nThe raw row is the direct source-archive baseline. `safe-only` may contain checked wrapper/runtime support and genuine native dependencies; the native-symbol assertions reject unrelated numerical drivers. Link maps remain under ignored `target/native-link/probes`.\n\n| Safe probe | Raw probe | Raw native symbols | Safe native symbols | Safe-only native symbols | Assertions |\n|---|---|---:|---:|---:|---|\n",
    );
    for comparison in &comparisons {
        let count = |field: &str| comparison[field].as_array().map_or(0, Vec::len);
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            comparison["safe_probe"].as_str().unwrap_or("?"),
            comparison["raw_probe"].as_str().unwrap_or("?"),
            count("raw_native_symbols"),
            count("safe_native_symbols"),
            count("safe_only_native_symbols"),
            comparison["safe_assertions_passed"],
        ));
    }
    fs::write(output.join("safe-facade-comparison.md"), markdown)?;
    Ok(())
}

fn find_named<'a>(rows: &'a [Value], name: &str) -> Option<&'a Value> {
    rows.iter()
        .find(|value| value["name"].as_str() == Some(name))
}
fn write_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}
fn env_path(name: &str) -> Result<PathBuf> {
    env::var_os(name).map(PathBuf::from).ok_or_else(|| {
        CorpusError::Policy(format!(
            "{name} is required; source acquisition remains explicit and offline"
        ))
    })
}

fn tool_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("tool")
        .to_owned()
}

fn sibling_tool(compiler: &Path, name: &str) -> PathBuf {
    compiler
        .parent()
        .map(|parent| parent.join(format!("{name}.exe")))
        .filter(|path| path.is_file())
        .unwrap_or_else(|| PathBuf::from(name))
}
fn output_text(program: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new(program).args(args).output()?;
    if !output.status.success() {
        return Err(command_error(
            &format!("{} {:?}", program.display(), args),
            output,
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
fn check_output(program: &Path, args: &[&str], what: &str) -> Result<()> {
    output_text(program, args)
        .map(|_| ())
        .map_err(|_| CorpusError::Verification(format!("could not determine {what}")))
}
fn command_error(context: &str, output: Output) -> CorpusError {
    CorpusError::Verification(format!(
        "{context} failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}
fn normalize_symbol(raw: &str) -> String {
    raw.trim_start_matches('_')
        .trim_end_matches('_')
        .to_ascii_uppercase()
}
fn first_line(value: &str) -> String {
    value.lines().next().unwrap_or_default().trim().to_owned()
}
fn bad(field: &str) -> CorpusError {
    CorpusError::Verification(format!("native-link report lacks {field}"))
}
fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalizes_fortran_symbols() {
        assert_eq!(normalize_symbol("_saxpy_"), "SAXPY");
    }
    #[test]
    fn distinguishes_single_routine_members() {
        let unit = vec![json!({"name":"SAXPY","kind":"subroutine"})];
        assert_eq!(classify_member(&unit, "lin/saxpy.f"), "single-routine");
    }
}
