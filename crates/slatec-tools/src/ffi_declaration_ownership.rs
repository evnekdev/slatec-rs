//! Deterministic ownership of public raw FFI declarations.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const ALIAS_START: &str = "// ffi-declaration-aliases:start";
const ALIAS_END: &str = "// ffi-declaration-aliases:end";

#[derive(Clone, Debug, Serialize)]
pub struct OwnershipResult {
    pub status: String,
    pub public_routines: usize,
    pub native_symbols_audited: usize,
    pub extern_declarations_before: usize,
    pub extern_declarations_after: usize,
    pub duplicate_symbols_before: usize,
    pub duplicate_symbols_after: usize,
    pub generated_reexports: usize,
    pub abi_conflicts: usize,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
}

#[derive(Clone, Debug)]
struct PublicRecord {
    routine: String,
    native_symbol: String,
    canonical_path: String,
    feature: String,
    generated_feature: String,
    generated_status: String,
    abi_fingerprint: String,
}

#[derive(Clone, Debug)]
struct Declaration {
    symbol: String,
    relative_path: String,
    line: usize,
    signature: String,
}

#[derive(Clone, Debug)]
struct Alias {
    routine: String,
    canonical_path: String,
}

pub fn generate(root: &Path, output_dir: &Path) -> Result<OwnershipResult> {
    let public = public_records(root)?;
    let sys_src = root.join("crates/slatec-sys/src");
    let original = scan_declarations(&sys_src)?;
    let existing_alias_files = scan_generated_aliases(&sys_src.join("generated"))?;
    let by_symbol = declarations_by_symbol(&original);
    let public_by_symbol = public
        .iter()
        .map(|record| (record.native_symbol.clone(), record))
        .collect::<BTreeMap<_, _>>();

    let mut removals = BTreeMap::<String, BTreeSet<String>>::new();
    let mut aliases = BTreeMap::<String, Vec<Alias>>::new();
    let mut conflicts = Vec::new();
    for record in &public {
        let declarations = by_symbol
            .get(&record.native_symbol)
            .cloned()
            .unwrap_or_default();
        let generated = declarations
            .iter()
            .filter(|item| is_abi_shaped_module(&item.relative_path))
            .collect::<Vec<_>>();
        let reviewed = declarations
            .iter()
            .filter(|item| !is_abi_shaped_module(&item.relative_path))
            .collect::<Vec<_>>();
        if reviewed.len() > 1 || (reviewed.is_empty() && generated.len() > 1) {
            conflicts.push(format!(
                "{} ({}) has incompatible declaration ownership: {}",
                record.routine,
                record.native_symbol,
                declarations
                    .iter()
                    .map(|item| format!("{}:{}", item.relative_path, item.line))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            continue;
        }
        if let Some(owner) = reviewed.first() {
            let alias_file = if let Some(generated_owner) = generated.first() {
                removals
                    .entry(generated_owner.relative_path.clone())
                    .or_default()
                    .insert(record.native_symbol.clone());
                Some(generated_owner.relative_path.clone())
            } else {
                existing_alias_files
                    .get(&record.canonical_path)
                    .cloned()
                    .or_else(|| generated_file(&record.generated_feature).map(str::to_owned))
            };
            if let Some(file) = alias_file {
                aliases.entry(file).or_default().push(Alias {
                    routine: record.routine.clone(),
                    canonical_path: record.canonical_path.clone(),
                });
            }
            let _ = owner;
        } else if generated.len() != 1 {
            conflicts.push(format!(
                "{} ({}) has no authoritative declaration",
                record.routine, record.native_symbol
            ));
        }
    }
    if !conflicts.is_empty() {
        return Err(policy(&conflicts.join("\n")));
    }

    let generated_dir = sys_src.join("generated");
    for path in rust_files(&generated_dir)? {
        let relative = slash_path(
            path.strip_prefix(&sys_src)
                .map_err(|_| policy(&format!("{} is outside slatec-sys/src", path.display())))?,
        );
        let content = fs::read_to_string(&path)?;
        let content = strip_alias_section(&content)?;
        let content = remove_declarations(
            &content,
            removals.get(&relative).cloned().unwrap_or_default(),
        )?;
        let content = append_aliases(content, aliases.remove(&relative).unwrap_or_default());
        write_if_changed(&path, content.as_bytes())?;
    }
    if !aliases.is_empty() {
        return Err(policy(&format!(
            "generated alias targets do not exist: {}",
            aliases.keys().cloned().collect::<Vec<_>>().join(", ")
        )));
    }

    let final_declarations = scan_declarations(&sys_src)?;
    let final_by_symbol = declarations_by_symbol(&final_declarations);
    let mut records = Vec::with_capacity(public.len());
    let mut duplicate_after = 0usize;
    let mut abi_conflicts = 0usize;
    let generated_alias_count = public
        .iter()
        .filter(|record| {
            generated_alias_present(&sys_src, &record.routine, &record.canonical_path)
                .unwrap_or(false)
        })
        .count();
    for record in &public {
        let declarations = final_by_symbol
            .get(&record.native_symbol)
            .cloned()
            .unwrap_or_default();
        if declarations.len() != 1 {
            duplicate_after += usize::from(declarations.len() > 1);
        }
        let signatures = declarations
            .iter()
            .map(|item| item.signature.clone())
            .collect::<BTreeSet<_>>();
        if signatures.len() > 1 {
            abi_conflicts += 1;
        }
        let owner = declarations.first();
        records.push(serde_json::json!({
            "routine":record.routine,
            "native_symbol":record.native_symbol,
            "authoritative_declaration_path":owner.map(|item| format!("crates/slatec-sys/src/{}:{}", item.relative_path, item.line)).unwrap_or_else(|| "missing".to_owned()),
            "canonical_public_path":record.canonical_path,
            "extern_declaration_count":declarations.len(),
            "public_path_count":1,
            "private_generated_reexport_present":generated_alias_present(&sys_src, &record.routine, &record.canonical_path)?,
            "abi_fingerprint":record.abi_fingerprint,
            "feature_gates":{"canonical":record.feature,"private_declaration":record.generated_feature},
            "consistency_status":if declarations.len() == 1 && signatures.len() == 1 {"consistent"} else {"invalid"},
        }));
    }
    let invalid = records
        .iter()
        .filter(|record| record["consistency_status"] != "consistent")
        .count();
    if invalid != 0 || duplicate_after != 0 || abi_conflicts != 0 {
        return Err(policy(&format!(
            "unique FFI ownership failed: {invalid} invalid records, {duplicate_after} duplicate symbols, {abi_conflicts} ABI conflicts"
        )));
    }

    let public_symbols = public_by_symbol.keys().cloned().collect::<BTreeSet<_>>();
    let final_public_declarations = final_declarations
        .iter()
        .filter(|item| public_symbols.contains(&item.symbol))
        .count();
    let duplicate_before = public
        .iter()
        .filter(|record| record.generated_status == "generated_abi_validated")
        .count();
    let conceptual_before = final_public_declarations + duplicate_before;
    let report = serde_json::json!({
        "schema_id":"slatec-sys.public-api.ffi-declaration-ownership",
        "schema_version":1,
        "policy":"Every public native symbol has exactly one private authoritative extern declaration and one canonical public mathematical path. ABI-shaped re-exports are private implementation details.",
        "counts":{
            "public_routines":public.len(),
            "native_symbols_audited":public_symbols.len(),
            "extern_declarations_before":conceptual_before,
            "extern_declarations_after":final_public_declarations,
            "symbols_with_duplicate_declarations_before":duplicate_before,
            "symbols_with_duplicate_declarations_after":duplicate_after,
            "declarations_removed_from_abi_shaped_modules":duplicate_before,
            "generated_abi_reexports":generated_alias_count,
            "abi_conflicts":abi_conflicts,
        },
        "records":records,
    });
    fs::create_dir_all(output_dir)?;
    write_json(&output_dir.join("ffi-declaration-ownership.json"), &report)?;
    let summary = format!(
        "# FFI declaration ownership\n\n- Canonical public routines: {}\n- Native symbols audited: {}\n- Extern declarations before ownership consolidation: {}\n- Extern declarations after ownership consolidation: {}\n- Duplicate symbols before: {}\n- Duplicate symbols after: {}\n- Private declaration re-exports: {}\n- ABI conflicts: {}\n- Result: pass\n",
        public.len(),
        public_symbols.len(),
        conceptual_before,
        final_public_declarations,
        duplicate_before,
        duplicate_after,
        generated_alias_count,
        abi_conflicts,
    );
    write_if_changed(
        &output_dir.join("ffi-declaration-ownership-summary.md"),
        summary.as_bytes(),
    )?;
    let semantic_hash = hash::bytes(&serde_json::to_vec(&report)?);
    Ok(OwnershipResult {
        status: "generated".to_owned(),
        public_routines: public.len(),
        native_symbols_audited: public_symbols.len(),
        extern_declarations_before: conceptual_before,
        extern_declarations_after: final_public_declarations,
        duplicate_symbols_before: duplicate_before,
        duplicate_symbols_after: duplicate_after,
        generated_reexports: generated_alias_count,
        abi_conflicts,
        semantic_hash,
        output_dir: output_dir.to_path_buf(),
    })
}

pub fn validate(root: &Path, output_dir: &Path) -> Result<OwnershipResult> {
    let mut result = generate(root, output_dir)?;
    let source = fs::read_to_string(root.join("crates/slatec-sys/src/lib.rs"))?;
    if !source.contains("#[path = \"generated_compat.rs\"]\nmod generated;") {
        return Err(policy(
            "the ABI-shaped generated module must remain private",
        ));
    }
    for path in rust_files(&root.join("crates/slatec-sys/src"))? {
        let content = fs::read_to_string(&path)?;
        if content.contains("cfg(not(") && content.contains("unsafe extern \"C\"") {
            return Err(policy(&format!(
                "{} contains a complementary cfg declaration branch",
                path.display()
            )));
        }
        if content.contains("#[deprecated") {
            return Err(policy(&format!(
                "{} exposes an unreleased deprecation attribute",
                path.display()
            )));
        }
        if content.contains("pub mod numerical") {
            return Err(policy(&format!(
                "{} exposes an unreleased numerical compatibility namespace",
                path.display()
            )));
        }
    }
    let root_source = fs::read_to_string(root.join("crates/slatec-sys/src/lib.rs"))?;
    for module in [
        "generated",
        "families",
        "special_scalar_expanded",
        "fftpack_complex",
        "fishpack_cartesian_2d",
        "fishpack_pois3d",
        "banded",
        "pchip",
        "bspline",
        "piecewise_polynomial",
        "eigen",
    ] {
        if root_source.contains(&format!("pub mod {module}")) {
            return Err(policy(&format!(
                "slatec-sys exposes the unreleased development module `{module}`"
            )));
        }
    }
    result.status = "validated".to_owned();
    Ok(result)
}

fn public_records(root: &Path) -> Result<Vec<PublicRecord>> {
    let disposition: Value = serde_json::from_slice(&fs::read(
        root.join("generated/raw-api/final-disposition.json"),
    )?)?;
    let status: Value = serde_json::from_slice(&fs::read(
        root.join("generated/raw-api/routine-status.json"),
    )?)?;
    let statuses = status["records"]
        .as_array()
        .ok_or_else(|| policy("routine-status records are missing"))?
        .iter()
        .map(|record| (field(record, "routine"), record))
        .collect::<BTreeMap<_, _>>();
    let mut records = Vec::new();
    for record in disposition["records"]
        .as_array()
        .ok_or_else(|| policy("final-disposition records are missing"))?
    {
        if field(record, "final_disposition") != "canonical-public" {
            continue;
        }
        let routine = field(record, "routine");
        let status = statuses
            .get(&routine)
            .ok_or_else(|| policy(&format!("{routine} has no routine-status record")))?;
        records.push(PublicRecord {
            routine,
            native_symbol: field(record, "native_symbol").to_ascii_lowercase(),
            canonical_path: field(record, "canonical_rust_path"),
            feature: field(record, "feature"),
            generated_feature: field(status, "generated_declaration_feature"),
            generated_status: field(status, "generated_declaration_status"),
            abi_fingerprint: status
                .pointer("/documentation_evidence/abi_fingerprint")
                .and_then(Value::as_str)
                .unwrap_or("unavailable")
                .to_owned(),
        });
    }
    records.sort_by(|left, right| left.routine.cmp(&right.routine));
    if records.len() != 812 {
        return Err(policy(&format!(
            "expected 812 public routines, found {}",
            records.len()
        )));
    }
    Ok(records)
}

fn scan_declarations(root: &Path) -> Result<Vec<Declaration>> {
    let mut declarations = Vec::new();
    for path in rust_files(root)? {
        let relative = slash_path(
            path.strip_prefix(root)
                .map_err(|_| policy("Rust source escaped scan root"))?,
        );
        let content = fs::read_to_string(&path)?;
        let lines = content.lines().collect::<Vec<_>>();
        let mut index = 0usize;
        while index < lines.len() {
            if let Some(symbol) = link_name(lines[index]) {
                let start = index;
                while index < lines.len() && !lines[index].contains(';') {
                    index += 1;
                }
                if index == lines.len() {
                    return Err(policy(&format!(
                        "unterminated declaration at {}:{}",
                        relative,
                        start + 1
                    )));
                }
                declarations.push(Declaration {
                    symbol,
                    relative_path: relative.clone(),
                    line: start + 1,
                    signature: lines[start + 1..=index]
                        .join(" ")
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" "),
                });
            }
            index += 1;
        }
    }
    Ok(declarations)
}

fn declarations_by_symbol(declarations: &[Declaration]) -> BTreeMap<String, Vec<&Declaration>> {
    let mut by_symbol = BTreeMap::<String, Vec<&Declaration>>::new();
    for declaration in declarations {
        by_symbol
            .entry(declaration.symbol.to_ascii_lowercase())
            .or_default()
            .push(declaration);
    }
    by_symbol
}

fn remove_declarations(content: &str, symbols: BTreeSet<String>) -> Result<String> {
    if symbols.is_empty() {
        return Ok(content.to_owned());
    }
    let lines = content.split_inclusive('\n').collect::<Vec<_>>();
    let mut output = String::with_capacity(content.len());
    let mut found = BTreeSet::new();
    let mut index = 0usize;
    while index < lines.len() {
        if let Some(symbol) = link_name(lines[index])
            && symbols.contains(&symbol)
        {
            found.insert(symbol);
            while index < lines.len() && !lines[index].contains(';') {
                index += 1;
            }
            if index == lines.len() {
                return Err(policy("unterminated generated declaration"));
            }
            index += 1;
            continue;
        }
        output.push_str(lines[index]);
        index += 1;
    }
    if found != symbols {
        let missing = symbols.difference(&found).cloned().collect::<Vec<_>>();
        return Err(policy(&format!(
            "could not remove generated declarations: {}",
            missing.join(", ")
        )));
    }
    Ok(output)
}

fn append_aliases(mut content: String, mut aliases: Vec<Alias>) -> String {
    if aliases.is_empty() {
        return content;
    }
    aliases.sort_by(|left, right| left.canonical_path.cmp(&right.canonical_path));
    aliases.dedup_by(|left, right| left.routine == right.routine);
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push('\n');
    content.push_str(ALIAS_START);
    content.push('\n');
    for alias in aliases {
        let canonical = alias
            .canonical_path
            .strip_prefix("slatec_sys::")
            .unwrap_or(&alias.canonical_path);
        content.push_str(&format!(
            "#[doc = \"Transitional ABI-shaped alias; use `crate::{canonical}`.\"]\npub use crate::{canonical};\n"
        ));
    }
    content.push_str(ALIAS_END);
    content.push('\n');
    content
}

fn strip_alias_section(content: &str) -> Result<String> {
    let Some(start) = content.find(ALIAS_START) else {
        return Ok(content.to_owned());
    };
    let tail = &content[start..];
    let end = tail
        .find(ALIAS_END)
        .ok_or_else(|| policy("generated alias section is unterminated"))?
        + start
        + ALIAS_END.len();
    let mut output = String::with_capacity(content.len());
    output.push_str(content[..start].trim_end_matches(['\r', '\n']));
    output.push('\n');
    output.push_str(content[end..].trim_start_matches(['\r', '\n']));
    Ok(output)
}

fn generated_alias_present(root: &Path, routine: &str, canonical_path: &str) -> Result<bool> {
    let canonical = canonical_path
        .strip_prefix("slatec_sys::")
        .unwrap_or(canonical_path);
    let needle = format!("pub use crate::{canonical};");
    for path in rust_files(&root.join("generated"))? {
        let content = fs::read_to_string(path)?;
        if content.contains(&needle) {
            return Ok(true);
        }
    }
    let _ = routine;
    Ok(false)
}

fn scan_generated_aliases(root: &Path) -> Result<BTreeMap<String, String>> {
    let mut aliases = BTreeMap::new();
    for path in rust_files(root)? {
        let content = fs::read_to_string(&path)?;
        let Some(start) = content.find(ALIAS_START) else {
            continue;
        };
        let end = content[start..]
            .find(ALIAS_END)
            .ok_or_else(|| policy("generated alias section is unterminated"))?
            + start;
        let relative = format!(
            "generated/{}",
            slash_path(
                path.strip_prefix(root)
                    .map_err(|_| policy("generated alias file escaped generated root"))?
            )
        );
        for line in content[start..end].lines() {
            let Some(path) = line
                .trim()
                .strip_prefix("pub use crate::")
                .and_then(|line| line.strip_suffix(';'))
            else {
                continue;
            };
            aliases.insert(format!("slatec_sys::{path}"), relative.clone());
        }
    }
    Ok(aliases)
}

fn generated_file(feature: &str) -> Option<&'static str> {
    Some(match feature {
        "raw-ffi-numeric-array-subroutines" => "generated/numeric_array_subroutines.rs",
        "raw-ffi-numeric-scalar-subroutines" => "generated/numeric_scalar_subroutines.rs",
        "raw-ffi-scalar-functions" => "generated/scalar_functions.rs",
        "raw-ffi-complex-arguments" => "generated/complex_arguments.rs",
        "raw-ffi-complex-returns" => "generated/complex_returns.rs",
        "raw-ffi-logical" => "generated/logical.rs",
        "raw-ffi-character" => "generated/character.rs",
        "raw-ffi-callbacks" => "generated/callbacks.rs",
        _ => return None,
    })
}

fn is_abi_shaped_module(path: &str) -> bool {
    matches!(
        path,
        "generated/numeric_array_subroutines.rs"
            | "generated/numeric_scalar_subroutines.rs"
            | "generated/scalar_functions.rs"
            | "generated/complex_arguments.rs"
            | "generated/complex_returns.rs"
            | "generated/logical.rs"
            | "generated/character.rs"
            | "generated/callbacks.rs"
    )
}

fn rust_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !root.exists() {
        return Ok(files);
    }
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

fn link_name(line: &str) -> Option<String> {
    let marker = "#[link_name = \"";
    let start = line.find(marker)? + marker.len();
    let end = line[start..].find('"')? + start;
    Some(line[start..end].to_ascii_lowercase())
}

fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn field(record: &Value, name: &str) -> String {
    record[name].as_str().unwrap_or_default().to_owned()
}

fn write_json(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    write_if_changed(path, &bytes)
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
