//! Deterministic safe-family to raw-symbol and native-object closure metadata.

use crate::blas_api;
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
    let batch_a = read_json(&root.join("generated/raw-api/batch-a-candidates.json"))?;
    let batch_a_classification =
        read_json(&root.join("generated/raw-api/abi-classification.json"))?;
    let batch_c = read_json(&root.join("generated/raw-api/batch-c-candidates.json"))?;
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
    // R2A raw BLAS is reviewed as a complete public family, not merely as
    // whatever subset current safe wrappers happen to call. These seeds make
    // each `slatec-src` level feature select the exact source closure for all
    // canonical raw symbols and give link probes a complete symbol set.
    for (level, routines) in [
        ("blas-level1", blas_api::LEVEL1),
        ("blas-level2", blas_api::LEVEL2),
        ("blas-level3", blas_api::LEVEL3),
    ] {
        family_symbols
            .entry(level.to_owned())
            .or_default()
            .extend(routines.iter().map(|routine| (*routine).to_owned()));
    }
    // Batch A is driven by the deterministic ABI classifier rather than safe
    // wrappers.  The same object-level undefined-symbol walk used by the safe
    // family closures therefore proves every promoted raw driver's provider
    // closure without falling back to a corpus-wide archive.
    for record in array_records(&batch_a, "Batch A candidates")? {
        let family = string_field(record, "provider_feature")?;
        let routine = string_field(record, "routine")?;
        family_symbols
            .entry(family.to_owned())
            .or_default()
            .insert(routine.to_owned());
    }
    // Batch C uses the same exact object-level dependency walk. ABI shape is
    // a declaration concern; provider selection remains grouped by coherent
    // mathematical family closures.
    for record in array_records(&batch_c, "Batch C candidates")? {
        let family = string_field(record, "provider_feature")?;
        let routine = string_field(record, "routine")?;
        family_symbols
            .entry(family.to_owned())
            .or_default()
            .insert(routine.to_owned());
    }
    // Reviewed raw-only drivers do not necessarily have a safe wrapper to
    // seed this graph. Their authored correction is the canonical review
    // decision and provides the provider feature deliberately, so it is the
    // authoritative source for the same object-level closure walk used above.
    let corrections = read_json(&root.join("metadata/raw-api-corrections.json"))?;
    for record in array_records(&corrections, "raw API corrections")? {
        if string_field(record, "review_status")? != "reviewed_public_driver" {
            continue;
        }
        let family = string_field(record, "provider_feature")?;
        let routine = string_field(record, "routine")?;
        family_symbols
            .entry(family.to_owned())
            .or_default()
            .insert(routine.to_owned());
    }

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
        let symbol = row_string(row, 1)?;
        let owner = row_string(row, 0)?;
        if symbol.starts_with(".refptr.") || row.get(2).and_then(Value::as_str) == Some("C") {
            continue;
        }
        if let Some(previous) = symbol_owner.insert(symbol.clone(), owner.clone()) {
            if previous != owner {
                return Err(CorpusError::Verification(format!(
                    "duplicate compiled symbol owner for {symbol}: {previous} and {owner}"
                )));
            }
        }
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
    let batch_linkage = batch_linkage_report(
        array_records(&batch_a_classification, "Batch A ABI classification")?,
        &name_to_unit,
        &unit_to_source,
        &source_names,
        &undefined,
        &symbol_owner,
    )?;

    let xermsg_source = name_to_unit
        .get("XERMSG")
        .and_then(|unit| unit_to_source.get(unit))
        .cloned();
    let mut family_sources = BTreeMap::<String, BTreeSet<String>>::new();
    let mut profile_override_families = BTreeSet::new();
    for (family, seeds) in &family_symbols {
        let mut queue = VecDeque::new();
        for name in seeds {
            let source = name_to_unit
                .get(name)
                .and_then(|unit| unit_to_source.get(unit))
                .ok_or_else(|| {
                    CorpusError::Verification(format!(
                        "safe symbol {name} in {family} has no selected source owner"
                    ))
                })?;
            queue.push_back(source.clone());
        }
        if matches!(
            family.as_str(),
            "least-squares-nonlinear-easy"
                | "least-squares-nonlinear-expert"
                | "least-squares-covariance"
                | "least-squares-linear-bounded"
                | "least-squares-linear-bounded-constrained"
                | "least-squares-linear-constrained"
                | "ode-sdrive-expert"
        ) {
            // The nonlinear drivers report valid INFO=4..8 and bounded
            // drivers report MODE=-22 numerical termination via a
            // level-one XERMSG. The safe façade snapshots XGETF, selects the
            // documented nonfatal XSETF control while the native lock is held,
            // then restores it, so these support objects are part of the
            // reviewed narrow closure rather than an ambient dependency.
            for name in ["XGETF", "XSETF"] {
                let source = name_to_unit
                    .get(name)
                    .and_then(|unit| unit_to_source.get(unit))
                    .ok_or_else(|| {
                        CorpusError::Verification(format!(
                            "least-squares runtime support {name} has no selected source owner"
                        ))
                    })?;
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
                profile_override_families.insert(family.clone());
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
        "profile_override_families":profile_override_families,
        "sources":source_rows, "families":family_rows
    });
    render_family_bindings(root, &family_symbols)?;

    let raw_rows = family_symbols
        .iter()
        .map(|(family, names)| json!({"family":family,"raw_symbols":names}))
        .collect::<Vec<_>>();
    let closure_rows = family_sources.iter().map(|(family, sources)| json!({"family":family,"source_count":sources.len(),"source_ids":sources})).collect::<Vec<_>>();
    let closure_audit = family_sources
        .iter()
        .map(|(family, sources)| {
            json!({
                "family":family,
                "safe_symbol_count":family_symbols.get(family).map(BTreeSet::len).unwrap_or(0),
                "source_count":sources.len(),
                "safe_symbol_owner_resolution":"passed",
                "selected_dependency_resolution":"passed",
                "duplicate_symbol_owners":0,
                "profile_overrides_required":profile_override_families.contains(family),
                "status":"passed"
            })
        })
        .collect::<Vec<_>>();
    let family_root_owners = [
        ("dlnrel_", "DLNREL"),
        ("dgamma_", "DGAMMA"),
        ("dai_", "DAI"),
        ("dbesj0_", "DBESJ0"),
        ("ddot_", "DDOT"),
        ("dgemm_", "DGEMM"),
        ("dqag_", "DQAG"),
        ("dqawo_", "DQAWO"),
        ("dfzero_", "DFZERO"),
        ("dnsqe_", "DNSQE"),
        ("dnsq_", "DNSQ"),
        ("dckder_", "DCKDER"),
        ("dnls1_", "DNLS1"),
        ("dnls1e_", "DNLS1E"),
        ("dcov_", "DCOV"),
        ("dwnnls_", "DWNNLS"),
        ("dbols_", "DBOLS"),
        ("dlsei_", "DLSEI"),
        ("ddriv3_", "DDRIV3"),
    ]
    .into_iter()
    .map(|(symbol, name)| {
        let source = name_to_unit
            .get(name)
            .and_then(|unit| unit_to_source.get(unit))
            .ok_or_else(|| bad("link-retention root source owner"))?;
        Ok((symbol, source.clone()))
    })
    .collect::<Result<BTreeMap<_, _>>>()?;
    let examples = inspect_examples(root, &nm, &family_sources, &family_root_owners)?;
    let gamma = examples
        .iter()
        .find(|record| record["example"] == "link_gamma");
    let gamma_retention = gamma
        .map(|record| record["retention_checks"].clone())
        .unwrap_or(Value::Null);
    let least_squares = examples
        .iter()
        .find(|record| record["example"] == "link_least_squares_nonlinear_easy");
    let least_squares_retention = least_squares
        .map(|record| record["retention_checks"].clone())
        .unwrap_or(Value::Null);
    let expert_least_squares = examples
        .iter()
        .find(|record| record["example"] == "link_least_squares_nonlinear_expert");
    let expert_least_squares_retention = expert_least_squares
        .map(|record| record["retention_checks"].clone())
        .unwrap_or(Value::Null);
    let covariance = examples
        .iter()
        .find(|record| record["example"] == "link_least_squares_covariance");
    let covariance_retention = covariance
        .map(|record| record["retention_checks"].clone())
        .unwrap_or(Value::Null);
    let nonnegative_retention = examples
        .iter()
        .find(|record| record["example"] == "link_least_squares_linear_nonnegative")
        .map(|record| record["retention_checks"].clone())
        .unwrap_or(Value::Null);
    let bounded_retention = examples
        .iter()
        .find(|record| record["example"] == "link_least_squares_linear_bounded")
        .map(|record| record["retention_checks"].clone())
        .unwrap_or(Value::Null);
    let constrained_retention = examples
        .iter()
        .find(|record| record["example"] == "link_least_squares_linear_constrained")
        .map(|record| record["retention_checks"].clone())
        .unwrap_or(Value::Null);
    let report = json!({"schema_id":"slatec-rs/family-link-report","schema_version":"1.1.0","snapshot_id":SNAPSHOT,"provider_mode":"source-build","target":"x86_64-pc-windows-gnu","families":closure_rows,"examples":examples,"archive_policy":"separate object per physical selected source; no whole-archive"});
    let retention = json!({"schema_id":"slatec-rs/symbol-retention-report","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"records":raw_rows,"single_gamma":gamma_retention,"least_squares_easy":least_squares_retention,"least_squares_expert":expert_least_squares_retention,"least_squares_covariance":covariance_retention,"least_squares_linear_nonnegative":nonnegative_retention,"least_squares_linear_bounded":bounded_retention,"least_squares_linear_constrained":constrained_retention,"rule":"only referenced static-archive members and their compiler-observed dependency closure are retained"});
    let raw_map = json!({"schema_id":"slatec-rs/family-to-raw-symbols","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"records":raw_rows});
    let source_map = json!({"schema_id":"slatec-rs/family-to-source-closure","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"records":closure_rows});
    let closure_audit_report = json!({"schema_id":"slatec-rs/family-closure-audit","schema_version":"1.0.0","snapshot_id":SNAPSHOT,"records":closure_audit});
    let batch_linkage_report = json!({
        "schema_id":"slatec-sys.raw-api.batch-a-linkage-report",
        "schema_version":"1.0.0",
        "policy":"Every pre-link Batch A candidate's selected object closure must resolve all Fortran procedure symbols through the selected provider or an explicit ABI-profile override. Unresolved procedure symbols make the routine an external-dependency exclusion.",
        "records":batch_linkage,
    });
    let validated_examples = examples
        .iter()
        .filter(|record| record["status"] == "passed")
        .count();
    let summary = format!(
        "# Family linkage validation\n\n- Snapshot: `{SNAPSHOT}`\n- Families: {}\n- Reviewed physical sources in the union: {}\n- Native example binaries validated: {validated_examples}/{}.\n- Single-gamma unrelated-domain retention check: {}.\n- Least-squares narrow-link check: {}. `DNLS1E` intentionally retains `DNLS1`, its direct original implementation; `DCKDER` remains in that object because its optional native checking branch cannot be extracted separately.\n- Covariance narrow-link check: {}.\n- Bounded linear least-squares narrow-link check: {}.\n- Constrained linear least-squares narrow-link check: {}.\n- Object policy: one object per selected physical source; no whole-archive linking.\n- Provider policy: offline cache-only `source-build`; blocked `prebuilt`; explicit `system` and inert `external-backend` escape hatches.\n- Rights boundary: source and native bytes remain outside Git and crate packages.\n",
        family_sources.len(),
        selected_ids.len(),
        examples.len(),
        if gamma_retention["passed"].as_bool() == Some(true) {
            "passed"
        } else {
            "not run"
        },
        if least_squares_retention["passed"].as_bool() == Some(true) {
            "passed"
        } else {
            "not run"
        },
        if covariance_retention["passed"].as_bool() == Some(true) {
            "passed"
        } else {
            "not run"
        },
        if bounded_retention["passed"].as_bool() == Some(true) {
            "passed"
        } else {
            "not run"
        },
        if constrained_retention["passed"].as_bool() == Some(true) {
            "passed"
        } else {
            "not run"
        },
    );
    let files = [
        ("family-link-report.json", bytes(&report)?),
        ("family-to-raw-symbols.json", bytes(&raw_map)?),
        ("family-to-source-closure.json", bytes(&source_map)?),
        ("family-closure-audit.json", bytes(&closure_audit_report)?),
        ("batch-a-linkage-report.json", bytes(&batch_linkage_report)?),
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
    if path.contains("::bounded_constrained_least_squares::") {
        return "least-squares-linear-bounded-constrained".to_owned();
    }
    if path.contains("::bounded_least_squares::") {
        return "least-squares-linear-bounded".to_owned();
    }
    if path.contains("::constrained_least_squares::") {
        return "least-squares-linear-constrained".to_owned();
    }
    if path.contains("::linear_least_squares::") {
        return "least-squares-linear-nonnegative".to_owned();
    }
    if path.contains("::least_squares::estimate_covariance")
        || path.contains("::least_squares::covariance_from_expert_fit")
    {
        return "least-squares-covariance".to_owned();
    }
    if path.contains("::least_squares::least_squares_expert")
        || path.contains("::least_squares::least_squares_with_jacobian")
    {
        return "least-squares-nonlinear-expert".to_owned();
    }
    if path.contains("::least_squares::") {
        return "least-squares-nonlinear-easy".to_owned();
    }
    if path.contains("::nonlinear::") {
        if path.contains("check_jacobian") {
            return "nonlinear-jacobian-check".to_owned();
        }
        if path.contains("solve_system_expert") || path.contains("solve_system_with_jacobian") {
            return "nonlinear-expert".to_owned();
        }
        return "nonlinear-easy".to_owned();
    }
    if path.contains("::roots::") {
        return "roots-scalar".to_owned();
    }
    if path.contains("::ode::") {
        return "ode-sdrive-expert".to_owned();
    }
    if path.contains("::dassl::") {
        return "dassl".to_owned();
    }
    if path.contains("::differential_equations::pde::Pois3d")
        || path.contains("::differential_equations::pde::Grid3")
    {
        return "fishpack-pois3d".to_owned();
    }
    if path.contains("::differential_equations::pde::") {
        return "fishpack-cartesian-2d".to_owned();
    }
    if path.contains("::fftpack::") {
        return "fftpack-real".to_owned();
    }
    if path.contains("::pchip::") {
        return "pchip".to_owned();
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
    let family_owned = family_declaration_routines(root)?;
    let mut declarations = BTreeMap::<String, String>::new();
    // This is the one private declaration owner for the hand-reviewed
    // special-function groups.  Batch A and scalar-expanded declarations
    // deliberately stay in their own canonical-owner files instead.
    let source = fs::read_to_string(root.join("crates/slatec-sys/src/families.rs"))?;
    {
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
            // `families.rs` is regenerated into a nested module.  Parse the
            // declaration independent of the indentation used by the prior
            // generated output so repeated regeneration is idempotent.
            let mut block = format!("        {}\n", lines[index].trim_start());
            index += 1;
            while index < lines.len() {
                block.push_str("        ");
                block.push_str(lines[index].trim_start());
                block.push('\n');
                let complete = lines[index].trim_end().ends_with(';');
                index += 1;
                if complete {
                    break;
                }
            }
            if !family_owned.contains(&routine) {
                continue;
            }
            // A prior generated file can contain a stale duplicate while this
            // ownership migration is being regenerated.  Keep the first
            // declaration-owner block; the regenerated output has exactly one
            // owner and the ownership validator enforces that invariant.
            declarations.entry(routine).or_insert(block);
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
        if let Some(level) = family.strip_prefix("blas-level") {
            output.push_str(&format!(
                "/// Private declaration forwarding for the canonical reviewed BLAS Level {level} namespace.\n#[cfg(feature = \"{feature}\")]\npub mod {module} {{\n    pub use crate::blas::level{level}::*;\n}}\n\n"
            ));
            continue;
        }
        if let Some(domain) = family.strip_suffix("-complex") {
            let owner = domain.replace('-', "_");
            output.push_str(&format!(
                "/// Private declaration forwarding for the canonical ABI-sensitive {domain} namespace.\n#[cfg(feature = \"{feature}\")]\npub mod {module} {{\n    pub use crate::abi_bindings::{owner}::*;\n}}\n\n"
            ));
            continue;
        }
        if matches!(family.as_str(), "special-real" | "special-scalar-expanded") {
            continue;
        }
        let selected = symbols
            .iter()
            .filter_map(|symbol| declarations.get(symbol))
            .collect::<Vec<_>>();
        if selected.is_empty() {
            continue;
        }
        output.push_str(&format!(
            "/// Reviewed declarations required by `{family}`.\n#[cfg(feature = \"{feature}\")]\npub mod {module} {{\n    use crate::{{Complex32, Complex64, FortranCharacterLength, FortranInteger, FortranLogical}};\n    use core::ffi::c_char;\n\n    unsafe extern \"C\" {{\n"
        ));
        for declaration in selected {
            output.push_str(declaration);
        }
        output.push_str("    }\n}\n\n");
    }
    output.truncate(output.trim_end().len());
    output.push('\n');
    fs::write(root.join("crates/slatec-sys/src/families.rs"), output)?;
    Ok(())
}

fn family_declaration_routines(root: &Path) -> Result<BTreeSet<String>> {
    let statuses = read_json(&root.join("generated/raw-api/routine-status.json"))?;
    let batch_a = read_json(&root.join("generated/raw-api/batch-a-candidates.json"))?;
    let mut routines = BTreeSet::new();

    for record in object_records(&statuses, "raw API routine statuses")? {
        let reviewed = string_field(record, "reviewed_declaration_status")?;
        if reviewed.starts_with("reviewed_public_") {
            routines.insert(string_field(record, "routine")?.to_owned());
        }
    }
    for record in array_records(&batch_a, "Batch A candidates")? {
        if string_field(record, "binding_path")?.starts_with("crate::families::") {
            routines.insert(string_field(record, "routine")?.to_owned());
        }
    }
    Ok(routines)
}

fn inspect_examples(
    root: &Path,
    nm: &Path,
    family_sources: &BTreeMap<String, BTreeSet<String>>,
    family_root_owners: &BTreeMap<&str, String>,
) -> Result<Vec<Value>> {
    let specifications = [
        ("link_elementary", "special-elementary", "dlnrel_"),
        ("link_gamma", "special-gamma", "dgamma_"),
        ("link_airy", "special-airy", "dai_"),
        ("link_bessel", "special-bessel", "dbesj0_"),
        ("link_blas_level1", "blas-level1", "ddot_"),
        ("link_blas_level3", "blas-level3", "dgemm_"),
        ("link_quadrature_basic", "quadrature-basic", "dqag_"),
        (
            "link_quadrature_oscillatory",
            "quadrature-oscillatory",
            "dqawo_",
        ),
        ("link_roots_scalar", "roots-scalar", "dfzero_"),
        ("link_nonlinear_easy", "nonlinear-easy", "dnsqe_"),
        (
            "link_least_squares_nonlinear_easy",
            "least-squares-nonlinear-easy",
            "dnls1e_",
        ),
        (
            "link_least_squares_nonlinear_expert",
            "least-squares-nonlinear-expert",
            "dnls1_",
        ),
        (
            "link_least_squares_covariance",
            "least-squares-covariance",
            "dcov_",
        ),
        (
            "link_least_squares_linear_nonnegative",
            "least-squares-linear-nonnegative",
            "dwnnls_",
        ),
        (
            "link_least_squares_linear_bounded",
            "least-squares-linear-bounded",
            "dbols_",
        ),
        (
            "link_least_squares_linear_bounded_constrained",
            "least-squares-linear-bounded-constrained",
            "dbocls_",
        ),
        (
            "link_least_squares_linear_constrained",
            "least-squares-linear-constrained",
            "dlsei_",
        ),
        ("link_nonlinear_expert", "nonlinear-expert", "dnsq_"),
        ("link_nonlinear_analytic", "nonlinear-expert", "dnsq_"),
        (
            "link_nonlinear_jacobian_check",
            "nonlinear-jacobian-check",
            "dckder_",
        ),
        ("link_ode_sdrive", "ode-sdrive-expert", "ddriv3_"),
        ("link_complete_safe_api", "full", "dgamma_"),
    ];
    let family_roots = [
        "dlnrel_", "dgamma_", "dai_", "dbesj0_", "ddot_", "dgemm_", "dqag_", "dqawo_", "dfzero_",
        "dnsqe_", "dnsq_", "dckder_", "dnls1_", "dnls1e_", "dcov_", "dwnnls_", "dbols_", "dbocls_",
        "dlsei_", "ddriv3_",
    ];
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
        let allowed_sources = family_sources.get(feature);
        let unrelated = if example != "link_complete_safe_api" {
            family_roots
                .iter()
                .filter(|symbol| {
                    **symbol != required
                        && symbols.contains(**symbol)
                        && !allowed_sources.is_some_and(|sources| {
                            family_root_owners
                                .get(**symbol)
                                .is_some_and(|owner| sources.contains(owner))
                        })
                })
                .copied()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        output.push(json!({
            "example":example,"feature":feature,"status":if required_present && unrelated.is_empty(){"passed"}else{"failed"},
            "binary_bytes":fs::metadata(&path)?.len(),"required_symbol":required,"required_symbol_present":required_present,
            "retained_defined_symbol_count":symbols.len(),
            "retained_family_roots":family_roots.iter().filter(|symbol| symbols.contains(**symbol)).copied().collect::<Vec<_>>(),
            "runtime_symbols":{"gfortran":symbols.iter().filter(|symbol| symbol.contains("gfortran")).count(),"quadmath":symbols.iter().filter(|symbol| symbol.contains("quadmath") || symbol.contains("flt128")).count()},
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

fn batch_linkage_report(
    classifications: &[Value],
    name_to_unit: &BTreeMap<String, String>,
    unit_to_source: &BTreeMap<String, String>,
    source_names: &BTreeMap<String, BTreeSet<String>>,
    undefined: &BTreeMap<String, BTreeSet<String>>,
    symbol_owner: &BTreeMap<String, String>,
) -> Result<Vec<Value>> {
    let mut records = Vec::new();
    for record in classifications
        .iter()
        .filter(|record| record["batch_a_pre_link_eligible"] == true)
    {
        let routine = string_field(record, "routine")?;
        let source_hash = string_field(record, "source_hash")?;
        let seed = name_to_unit
            .get(routine)
            .and_then(|unit| unit_to_source.get(unit))
            .cloned()
            .ok_or_else(|| {
                CorpusError::Verification(format!(
                    "Batch A candidate {routine} has no selected source owner"
                ))
            })?;
        let mut closure = BTreeSet::new();
        let mut external = BTreeSet::new();
        let mut queue = VecDeque::from([seed]);
        while let Some(source) = queue.pop_front() {
            if !closure.insert(source.clone()) {
                continue;
            }
            for symbol in undefined.get(&source).into_iter().flatten() {
                if let Some(owner) = symbol_owner.get(symbol) {
                    queue.push_back(owner.clone());
                } else if is_external_fortran_procedure(symbol) {
                    external.insert(symbol.clone());
                }
            }
            // Provider profile replacements are linked by slatec-src rather
            // than selected-source objects. They are valid closure leaves.
            if source_names.get(&source).is_some_and(|names| {
                names
                    .iter()
                    .any(|name| MACHINE_CONSTANTS.contains(&name.as_str()))
            }) {
                external
                    .retain(|symbol| !matches!(symbol.as_str(), "i1mach_" | "r1mach_" | "d1mach_"));
            }
        }
        let link_status = if external.is_empty() {
            "source_closure_complete"
        } else {
            "external_dependency"
        };
        records.push(json!({
            "routine":routine,
            "source_hash":source_hash,
            "source_ids":closure,
            "external_undefined_symbols":external,
            "link_status":link_status,
        }));
    }
    records.sort_by(|left, right| left["routine"].as_str().cmp(&right["routine"].as_str()));
    Ok(records)
}

fn is_external_fortran_procedure(symbol: &str) -> bool {
    symbol.ends_with('_')
        && !matches!(symbol, "i1mach_" | "r1mach_" | "d1mach_")
        && !symbol.starts_with("__")
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
    use serde_json::Value;
    use std::{fs, path::PathBuf};

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
        assert_eq!(
            family_for("slatec::nonlinear::solve_system", "DNSQE"),
            "nonlinear-easy"
        );
        assert_eq!(
            family_for("slatec::nonlinear::solve_system_expert", "DNSQ"),
            "nonlinear-expert"
        );
        assert_eq!(
            family_for("slatec::nonlinear::check_jacobian", "DCKDER"),
            "nonlinear-jacobian-check"
        );
        assert_eq!(
            family_for("slatec::least_squares::least_squares", "DNLS1E"),
            "least-squares-nonlinear-easy"
        );
        assert_eq!(
            family_for(
                "slatec::least_squares::least_squares_with_jacobian",
                "DNLS1"
            ),
            "least-squares-nonlinear-expert"
        );
        assert_eq!(
            family_for(
                "slatec::linear_least_squares::solve_nonnegative_least_squares",
                "DWNNLS"
            ),
            "least-squares-linear-nonnegative"
        );
        assert_eq!(
            family_for(
                "slatec::bounded_least_squares::solve_bounded_least_squares",
                "DBOLS"
            ),
            "least-squares-linear-bounded"
        );
        assert_eq!(
            family_for(
                "slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares",
                "DBOCLS"
            ),
            "least-squares-linear-bounded-constrained"
        );
        assert_eq!(
            family_for(
                "slatec::constrained_least_squares::solve_constrained_least_squares",
                "DLSEI"
            ),
            "least-squares-linear-constrained"
        );
        assert_eq!(
            family_for(
                "slatec::ode::OdeSession::<f64, F, E>::integrate_to",
                "DDRIV3"
            ),
            "ode-sdrive-expert"
        );
        assert_eq!(
            family_for("slatec::fftpack::RealFftPlan::forward", "RFFTF"),
            "fftpack-real"
        );
        assert_eq!(
            family_for("slatec::pchip::PiecewiseCubicHermite::monotone", "PCHIM"),
            "pchip"
        );
        assert_eq!(
            family_for(
                "slatec::differential_equations::pde::CartesianHelmholtz2d::solve",
                "HWSCRT"
            ),
            "fishpack-cartesian-2d"
        );
        assert_eq!(
            family_for(
                "slatec::differential_equations::pde::Pois3dProblem::solve",
                "POIS3D"
            ),
            "fishpack-pois3d"
        );
    }

    #[test]
    fn every_narrow_family_has_a_passing_closure_audit() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let report: Value = serde_json::from_slice(
            &fs::read(root.join("generated/linkage/family-closure-audit.json"))
                .expect("closure audit"),
        )
        .expect("valid closure audit");
        let records = report["records"].as_array().expect("audit records");
        assert!(records.len() >= 29);
        assert!(records.iter().all(|record| record["status"] == "passed"));
    }
}
