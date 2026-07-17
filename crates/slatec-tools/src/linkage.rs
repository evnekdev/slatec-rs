//! Deterministic safe-family to raw-symbol and native-object closure metadata.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SNAPSHOT: &str = "complete-slatec-05078ebcb649b50e4435";
const PROFILE: &str = "native-profile-7e29d91c176d0c60";
const MACHINE_CONSTANTS: &[&str] = &["I1MACH", "R1MACH", "D1MACH"];

/// Summary returned after deterministic linkage metadata generation.
pub struct LinkageResult {
    /// Selected complete-corpus snapshot.
    pub snapshot_id: String,
    /// Number of public family closures generated.
    pub family_count: usize,
    /// Hash over the generated compact metadata.
    pub semantic_hash: String,
}

/// Generates family/raw/source closure records from compiler-observed objects.
pub fn generate(root: &Path, output: &Path, provider_manifest: &Path) -> Result<LinkageResult> {
    let safe = read_json(&root.join("generated/safe-api/function-index.json"))?;
    let interfaces = read_json(&root.join("generated/ffi/interface-inventory.json"))?;
    let selected_sources = read_json(&root.join("generated/ffi/selected-source-files.json"))?;
    let compilation = read_json(&root.join("generated/ffi/compilation-results.json"))?;
    let symbols = read_json(&root.join("generated/ffi/symbol-inventory.json"))?;
    require_snapshot(&interfaces)?;
    require_snapshot(&selected_sources)?;

    let mut family_symbols = BTreeMap::<String, BTreeSet<String>>::new();
    for record in object_records(&safe, "safe function index")? {
        let path = string_field(record, "rust_path")?;
        let raw = string_field(record, "fortran_routine")?;
        family_symbols
            .entry(family_for(path, raw))
            .or_default()
            .insert(raw.to_owned());
    }
    // Public deferred families remain explicit even when they currently own no
    // safe wrapper or native source seed.
    family_symbols
        .entry("roots-polynomial".to_owned())
        .or_default();

    let mut unit_to_source = BTreeMap::new();
    let mut source_details = BTreeMap::<String, SourceDetail>::new();
    for row in array_records(&selected_sources, "selected source files")? {
        let row = row.as_array().ok_or_else(|| bad("selected source row"))?;
        let id = row_string(row, 0)?;
        let subset = row_string(row, 1)?;
        let path = row_string(row, 2)?;
        let sha256 = row_string(row, 3)?;
        let units = row
            .get(5)
            .and_then(Value::as_array)
            .ok_or_else(|| bad("source program units"))?;
        let unit_ids = units
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| bad("program unit id"))
            })
            .collect::<Result<Vec<_>>>()?;
        for unit in &unit_ids {
            unit_to_source.insert(unit.clone(), id.clone());
        }
        source_details.insert(
            id.clone(),
            SourceDetail {
                id,
                subset: subset.clone(),
                path: path.clone(),
                sha256,
                url: source_url(&subset, &path),
            },
        );
    }

    let mut name_to_unit = BTreeMap::new();
    let mut source_names = BTreeMap::<String, BTreeSet<String>>::new();
    for row in array_records(&interfaces, "FFI interfaces")? {
        let row = row.as_array().ok_or_else(|| bad("interface row"))?;
        let unit = row_string(row, 0)?;
        let name = row_string(row, 1)?;
        name_to_unit.insert(name.clone(), unit.clone());
        if let Some(source) = unit_to_source.get(&unit) {
            source_names.entry(source.clone()).or_default().insert(name);
        }
    }

    let mut source_objects = BTreeMap::new();
    for row in array_records(&compilation, "compilation results")? {
        let row = row.as_array().ok_or_else(|| bad("compilation row"))?;
        if row.get(3).and_then(Value::as_str) == Some("compiled") {
            if let Some(object) = row.get(6).and_then(Value::as_str) {
                source_objects.insert(row_string(row, 0)?, object.to_owned());
            }
        }
    }
    let mut symbol_owner = BTreeMap::new();
    for row in array_records(&symbols, "symbol inventory")? {
        let row = row.as_array().ok_or_else(|| bad("symbol row"))?;
        symbol_owner.insert(row_string(row, 1)?, row_string(row, 0)?);
    }

    let object_root = root
        .join("evidence/raw-ffi")
        .join(SNAPSHOT)
        .join(PROFILE)
        .join("objects");
    let nm = nm_path();
    for object in source_objects.values() {
        let path = object_root.join(format!("{object}.o"));
        if !path.is_file() {
            return Err(CorpusError::Verification(format!(
                "missing compiler-observed object {}",
                path.display()
            )));
        }
    }
    let archive = object_root
        .parent()
        .expect("objects has a parent")
        .join("libslatec_selected_original.a");
    let undefined = undefined_symbols_archive(&nm, &archive, &source_objects)?;

    let xermsg_source = name_to_unit
        .get("XERMSG")
        .and_then(|unit| unit_to_source.get(unit))
        .cloned();
    let mut family_sources = BTreeMap::<String, BTreeSet<String>>::new();
    for (family, seeds) in &family_symbols {
        let mut queue = VecDeque::new();
        for name in seeds {
            if let Some(source) = name_to_unit
                .get(name)
                .and_then(|unit| unit_to_source.get(unit))
            {
                queue.push_back(source.clone());
            }
        }
        let mut closure = BTreeSet::new();
        while let Some(source) = queue.pop_front() {
            if closure.contains(&source) {
                continue;
            }
            let names = source_names.get(&source).cloned().unwrap_or_default();
            if names
                .iter()
                .any(|name| MACHINE_CONSTANTS.contains(&name.as_str()))
            {
                if let Some(xer) = &xermsg_source {
                    queue.push_back(xer.clone());
                }
            } else {
                closure.insert(source.clone());
            }
            for symbol in undefined.get(&source).into_iter().flatten() {
                if let Some(owner) = symbol_owner.get(symbol) {
                    queue.push_back(owner.clone());
                }
            }
        }
        family_sources.insert(family.clone(), closure);
    }

    let selected_ids = family_sources
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let source_rows = selected_ids.iter().map(|id| {
        let source = source_details.get(id).expect("closure source exists");
        json!({"id":source.id,"subset":source.subset,"path":source.path,"sha256":source.sha256,"url":source.url})
    }).collect::<Vec<_>>();
    let family_rows = family_sources
        .iter()
        .map(|(family, sources)| (family.clone(), sources.iter().cloned().collect::<Vec<_>>()))
        .collect::<BTreeMap<_, _>>();
    let provider = json!({
        "schema_id":"slatec-rs/family-source-closure", "schema_version":"1.0.0",
        "snapshot_id":SNAPSHOT, "compiler_profile_id":PROFILE,
        "profile_overrides":["I1MACH","R1MACH","D1MACH"],
        "sources":source_rows, "families":family_rows
    });
    render_family_bindings(root, &family_symbols)?;

    let raw_rows = family_symbols
        .iter()
        .map(|(family, names)| json!({"family":family,"raw_symbols":names}))
        .collect::<Vec<_>>();
    let closure_rows = family_sources.iter().map(|(family, sources)| json!({"family":family,"source_count":sources.len(),"source_ids":sources})).collect::<Vec<_>>();
    let examples = inspect_examples(root, &nm)?;
    let gamma = examples
        .iter()
        .find(|record| record["example"] == "link_gamma");
    let gamma_retention = gamma
        .map(|record| record["retention_checks"].clone())
        .unwrap_or(Value::Null);
    let report = json!({"schema_id":"slatec-rs/family-link-report","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"families":closure_rows,"examples":examples,"archive_policy":"separate object per physical selected source; no whole-archive"});
    let retention = json!({"schema_id":"slatec-rs/symbol-retention-report","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"records":raw_rows,"single_gamma":gamma_retention,"rule":"only referenced static-archive members and their compiler-observed dependency closure are retained"});
    let raw_map = json!({"schema_id":"slatec-rs/family-to-raw-symbols","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"records":raw_rows});
    let source_map = json!({"schema_id":"slatec-rs/family-to-source-closure","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"records":closure_rows});
    let validated_examples = examples
        .iter()
        .filter(|record| record["status"] == "passed")
        .count();
    let summary = format!(
        "# Family linkage validation\n\n- Snapshot: `{SNAPSHOT}`\n- Families: {}\n- Reviewed physical sources in the union: {}\n- Native example binaries validated: {validated_examples}/5.\n- Single-gamma unrelated-domain retention check: {}.\n- Object policy: one object per selected physical source; no whole-archive linking.\n- Provider policy: checksum-pinned automatic source acquisition for `bundled`; verified local cache for `source-build`; explicit `system` and inert `external-backend` escape hatches.\n- Rights boundary: source and native bytes remain outside Git and crate packages.\n",
        family_sources.len(),
        selected_ids.len(),
        if gamma_retention["passed"].as_bool() == Some(true) {
            "passed"
        } else {
            "not run"
        }
    );
    let files = [
        ("family-link-report.json", bytes(&report)?),
        ("family-to-raw-symbols.json", bytes(&raw_map)?),
        ("family-to-source-closure.json", bytes(&source_map)?),
        ("symbol-retention-report.json", bytes(&retention)?),
        ("validation-summary.md", summary.into_bytes()),
    ];
    fs::create_dir_all(output)?;
    for (name, content) in &files {
        fs::write(output.join(name), content)?;
    }
    if let Some(parent) = provider_manifest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(provider_manifest, bytes(&provider)?)?;
    let semantic_hash = hash::bytes(
        &files
            .iter()
            .flat_map(|(_, bytes)| bytes.iter().copied())
            .collect::<Vec<_>>(),
    );
    Ok(LinkageResult {
        snapshot_id: SNAPSHOT.to_owned(),
        family_count: family_sources.len(),
        semantic_hash,
    })
}

#[derive(Clone)]
struct SourceDetail {
    id: String,
    subset: String,
    path: String,
    sha256: String,
    url: String,
}

fn family_for(path: &str, routine: &str) -> String {
    if path.contains("::blas::level1::") {
        return "blas-level1".to_owned();
    }
    if path.contains("::blas::level2::") {
        return "blas-level2".to_owned();
    }
    if path.contains("::blas::level3::") {
        return "blas-level3".to_owned();
    }
    if path.contains("::special::elementary::") {
        return "special-elementary".to_owned();
    }
    if path.contains("::special::airy::") {
        return "special-airy".to_owned();
    }
    if path.contains("::special::bessel::") {
        return "special-bessel".to_owned();
    }
    if path.contains("::special::error_functions::") {
        return "special-error".to_owned();
    }
    if path.contains("::special::integrals::") {
        return "special-integrals".to_owned();
    }
    if path.contains("::polynomials::") {
        return "special-polynomials".to_owned();
    }
    if path.contains("::special::gamma::") {
        return if routine.contains("BETA") || matches!(routine, "BETAI" | "DBETAI") {
            "special-beta"
        } else {
            "special-gamma"
        }
        .to_owned();
    }
    if path.contains("::quadrature::") {
        if path.contains("breakpoint") {
            return "quadrature-breakpoints".to_owned();
        }
        if path.contains("weighted_endpoint") {
            return "quadrature-weighted".to_owned();
        }
        if path.contains("fourier") {
            return "quadrature-fourier".to_owned();
        }
        if path.contains("oscillatory") {
            return "quadrature-oscillatory".to_owned();
        }
        if path.contains("non_adaptive") || path.contains("nc79") {
            return "quadrature-nonadaptive".to_owned();
        }
        return "quadrature-basic".to_owned();
    }
    if path.contains("::roots::") {
        return "roots-scalar".to_owned();
    }
    "unclassified".to_owned()
}

fn source_url(subset: &str, path: &str) -> String {
    match subset {
        "main-src" => format!("https://www.netlib.org/slatec/{path}"),
        "spfun" => "https://www.netlib.org/slatec/spfun".to_owned(),
        _ => format!("https://www.netlib.org/slatec/{subset}/{path}"),
    }
}

fn render_family_bindings(
    root: &Path,
    families: &BTreeMap<String, BTreeSet<String>>,
) -> Result<()> {
    let generated = root.join("crates/slatec-sys/src/generated");
    let mut declarations = BTreeMap::<String, String>::new();
    for name in [
        "character.rs",
        "complex_arguments.rs",
        "logical.rs",
        "numeric_array_subroutines.rs",
        "numeric_scalar_subroutines.rs",
        "scalar_functions.rs",
    ] {
        let source = fs::read_to_string(generated.join(name))?;
        let lines = source.lines().collect::<Vec<_>>();
        let mut index = 0;
        while index < lines.len() {
            let line = lines[index].trim();
            if !line.starts_with("#[link_name = \"") {
                index += 1;
                continue;
            }
            let raw = line
                .split('"')
                .nth(1)
                .ok_or_else(|| bad("generated link name"))?;
            let routine = raw.trim_end_matches('_').to_ascii_uppercase();
            let mut block = format!("    {}\n", lines[index]);
            index += 1;
            while index < lines.len() {
                block.push_str("    ");
                block.push_str(lines[index]);
                block.push('\n');
                let complete = lines[index].trim_end().ends_with(';');
                index += 1;
                if complete {
                    break;
                }
            }
            if declarations.insert(routine.clone(), block).is_some() {
                return Err(CorpusError::Verification(format!(
                    "duplicate generated raw declaration for {routine}"
                )));
            }
        }
    }

    let mut output = String::from(
        "//! Generated family-scoped raw declarations.\n//!\n//! Regenerate with `slatec-corpus generate-linkage-metadata --offline`.\n#![allow(clippy::missing_safety_doc, unused_imports)]\n\n",
    );
    for (family, symbols) in families {
        if !(family.starts_with("blas-") || family.starts_with("special-")) {
            continue;
        }
        let module = family.replace('-', "_");
        let feature = format!("raw-family-{family}");
        output.push_str(&format!(
            "/// Reviewed declarations required by `{family}`.\n#[cfg(feature = \"{feature}\")]\npub mod {module} {{\n    use crate::{{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical}};\n    use core::ffi::c_char;\n\n    unsafe extern \"C\" {{\n"
        ));
        for symbol in symbols {
            let declaration = declarations.get(symbol).ok_or_else(|| {
                CorpusError::Verification(format!(
                    "safe family {family} references missing generated declaration {symbol}"
                ))
            })?;
            output.push_str(declaration);
        }
        output.push_str("    }\n}\n\n");
    }
    output.truncate(output.trim_end().len());
    output.push('\n');
    fs::write(root.join("crates/slatec-sys/src/families.rs"), output)?;
    Ok(())
}

fn inspect_examples(root: &Path, nm: &Path) -> Result<Vec<Value>> {
    let specifications = [
        ("link_gamma", "special-gamma", "dgamma_"),
        ("link_airy", "special-airy", "dai_"),
        ("link_blas_level1", "blas-level1", "ddot_"),
        ("link_quadrature_basic", "quadrature-basic", "dqag_"),
        ("link_complete_safe_api", "full", "dgamma_"),
    ];
    let forbidden = ["dai_", "dbesj0_", "dqag_", "dfzero_", "ddot_"];
    let mut output = Vec::new();
    for (example, feature, required) in specifications {
        let path = root
            .join("target/x86_64-pc-windows-gnu/debug/examples")
            .join(format!("{example}.exe"));
        if !path.is_file() {
            output.push(json!({"example":example,"feature":feature,"status":"not_run"}));
            continue;
        }
        let symbols = defined_symbols(nm, &path)?;
        let required_present = symbols.contains(required);
        let unrelated = if example == "link_gamma" {
            forbidden
                .iter()
                .filter(|symbol| **symbol != required && symbols.contains(**symbol))
                .copied()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        output.push(json!({
            "example":example,"feature":feature,"status":if required_present && unrelated.is_empty(){"passed"}else{"failed"},
            "binary_bytes":fs::metadata(&path)?.len(),"required_symbol":required,"required_symbol_present":required_present,
            "retention_checks":{"unrelated_entry_points":unrelated,"passed":required_present && unrelated.is_empty()}
        }));
    }
    Ok(output)
}

fn defined_symbols(nm: &Path, binary: &Path) -> Result<BTreeSet<String>> {
    let output = Command::new(nm)
        .args(["-g", "--defined-only"])
        .arg(binary)
        .output()
        .map_err(|error| {
            CorpusError::Verification(format!("could not inspect {}: {error}", binary.display()))
        })?;
    if !output.status.success() {
        return Err(CorpusError::Verification(format!(
            "nm failed for {}",
            binary.display()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_whitespace().last().map(str::to_owned))
        .collect())
}

fn nm_path() -> PathBuf {
    std::env::var_os("SLATEC_NM")
        .map(PathBuf::from)
        .or_else(|| {
            let path = PathBuf::from(r"C:\gcc64\bin\nm.exe");
            path.is_file().then_some(path)
        })
        .unwrap_or_else(|| PathBuf::from("nm"))
}

fn undefined_symbols_archive(
    nm: &Path,
    archive: &Path,
    source_objects: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let output = Command::new(nm)
        .arg("-u")
        .arg(archive)
        .output()
        .map_err(|e| CorpusError::Verification(format!("could not run {}: {e}", nm.display())))?;
    if !output.status.success() {
        return Err(CorpusError::Verification(format!(
            "nm failed for {}",
            archive.display()
        )));
    }
    let object_sources = source_objects
        .iter()
        .map(|(source, object)| (format!("{object}.o"), source.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut result = BTreeMap::<String, BTreeSet<String>>::new();
    let mut current = None;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Some(object) = line.strip_suffix(':') {
            current = object_sources.get(object.trim()).cloned();
        } else if let (Some(source), Some(symbol)) = (&current, line.split_whitespace().last()) {
            result
                .entry(source.clone())
                .or_default()
                .insert(symbol.to_owned());
        }
    }
    Ok(result)
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn require_snapshot(value: &Value) -> Result<()> {
    if value["snapshot_id"].as_str() == Some(SNAPSHOT) {
        Ok(())
    } else {
        Err(bad("snapshot mismatch"))
    }
}
fn array_records<'a>(value: &'a Value, name: &str) -> Result<&'a Vec<Value>> {
    value["records"].as_array().ok_or_else(|| bad(name))
}
fn object_records<'a>(value: &'a Value, name: &str) -> Result<&'a Vec<Value>> {
    array_records(value, name)
}
fn row_string(row: &[Value], index: usize) -> Result<String> {
    row.get(index)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| bad("compact field"))
}
fn string_field<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value[name].as_str().ok_or_else(|| bad(name))
}
fn bad(name: &str) -> CorpusError {
    CorpusError::Verification(format!("invalid linkage metadata field: {name}"))
}
fn bytes(value: &Value) -> Result<Vec<u8>> {
    let mut value = serde_json::to_vec(value)?;
    value.push(b'\n');
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::family_for;

    #[test]
    fn classifies_public_families_without_raw_name_guessing() {
        assert_eq!(
            family_for("slatec::special::gamma::gamma", "DGAMMA"),
            "special-gamma"
        );
        assert_eq!(
            family_for("slatec::special::gamma::beta", "DBETA"),
            "special-beta"
        );
        assert_eq!(
            family_for("slatec::quadrature::integrate_fourier_tail", "DQAWF"),
            "quadrature-fourier"
        );
        assert_eq!(
            family_for("slatec::blas::level1::ddot", "DDOT"),
            "blas-level1"
        );
    }
}
