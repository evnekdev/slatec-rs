//! Deterministic, compact safe-special-function candidate metadata.
//!
//! This generator deliberately refuses to run unless the committed runtime
//! profile validates the machine constants, legacy errors, and FNLIB startup
//! for the exact selected-corpus/raw-FFI snapshot.  It emits structural
//! inventories only: no copied source, native archive, or raw diagnostic text.

use crate::error::{CorpusError, Result};
use crate::{TOOL_NAME, TOOL_VERSION, hash};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: &str = "1.0.0";
const SEMANTIC_VERSION: &str = "1";
const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const MAX_FILE_BYTES: usize = 64 * 1024;
const MAX_TOTAL_BYTES: usize = 256 * 1024;

#[derive(Clone, Copy)]
struct WrapperSpec {
    source: &'static str,
    safe_path: &'static str,
    family: &'static str,
    precision: &'static str,
    domain_policy: &'static str,
    runtime_dependency: &'static str,
    state_policy: &'static str,
}

macro_rules! spec {
    ($source:literal, $path:literal, $family:literal, $precision:literal, $domain:literal) => {
        WrapperSpec {
            source: $source,
            safe_path: $path,
            family: $family,
            precision: $precision,
            domain_policy: $domain,
            runtime_dependency: "fnlib_initialized_saved_state",
            state_policy: "serialized_process_global",
        }
    };
}

macro_rules! spec_with_runtime {
    ($source:literal, $path:literal, $family:literal, $precision:literal, $domain:literal, $runtime:literal, $state:literal) => {
        WrapperSpec {
            source: $source,
            safe_path: $path,
            family: $family,
            precision: $precision,
            domain_policy: $domain,
            runtime_dependency: $runtime,
            state_policy: $state,
        }
    };
}

const WRAPPERS: &[WrapperSpec] = &[
    spec!(
        "DLNREL",
        "slatec::special::elementary::log1p",
        "elementary",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DEXPRL",
        "slatec::special::elementary::exprel",
        "elementary",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DCBRT",
        "slatec::special::elementary::cbrt",
        "elementary",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DSINDG",
        "slatec::special::elementary::sin_degrees",
        "elementary",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DCOSDG",
        "slatec::special::elementary::cos_degrees",
        "elementary",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DDAWS",
        "slatec::special::elementary::dawson",
        "elementary",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "ALNREL",
        "slatec::special::elementary::log1p_f32",
        "elementary",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "EXPREL",
        "slatec::special::elementary::exprel_f32",
        "elementary",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "CBRT",
        "slatec::special::elementary::cbrt_f32",
        "elementary",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "SINDG",
        "slatec::special::elementary::sin_degrees_f32",
        "elementary",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "COSDG",
        "slatec::special::elementary::cos_degrees_f32",
        "elementary",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "DAWS",
        "slatec::special::elementary::dawson_f32",
        "elementary",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "DAI",
        "slatec::special::airy::airy_ai",
        "airy",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DAIE",
        "slatec::special::airy::airy_ai_scaled",
        "airy",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBI",
        "slatec::special::airy::airy_bi",
        "airy",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBIE",
        "slatec::special::airy::airy_bi_scaled",
        "airy",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "AI",
        "slatec::special::airy::airy_ai_f32",
        "airy",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "AIE",
        "slatec::special::airy::airy_ai_scaled_f32",
        "airy",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BI",
        "slatec::special::airy::airy_bi_f32",
        "airy",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BIE",
        "slatec::special::airy::airy_bi_scaled_f32",
        "airy",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "DERF",
        "slatec::special::error_functions::erf",
        "error_functions",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DERFC",
        "slatec::special::error_functions::erfc",
        "error_functions",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "ERF",
        "slatec::special::error_functions::erf_f32",
        "error_functions",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "ERFC",
        "slatec::special::error_functions::erfc_f32",
        "error_functions",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "DE1",
        "slatec::special::integrals::exponential_integral_e1",
        "integrals",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DEI",
        "slatec::special::integrals::exponential_integral_ei",
        "integrals",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "E1",
        "slatec::special::integrals::exponential_integral_e1_f32",
        "integrals",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "EI",
        "slatec::special::integrals::exponential_integral_ei_f32",
        "integrals",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "DGAMMA",
        "slatec::special::gamma::gamma",
        "gamma_beta",
        "f64",
        "fully_prechecked"
    ),
    spec_with_runtime!(
        "DGAMR",
        "slatec::special::gamma::reciprocal_gamma",
        "gamma_beta",
        "f64",
        "legacy_error_prevented_by_checks",
        "legacy_error_state",
        "serialized_process_global"
    ),
    spec!(
        "DLNGAM",
        "slatec::special::gamma::log_gamma",
        "gamma_beta",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBETA",
        "slatec::special::gamma::beta",
        "gamma_beta",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DLBETA",
        "slatec::special::gamma::log_beta",
        "gamma_beta",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBETAI",
        "slatec::special::gamma::regularized_beta",
        "probability",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DGAMI",
        "slatec::special::gamma::incomplete_gamma_lower",
        "gamma_beta",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DGAMIC",
        "slatec::special::gamma::incomplete_gamma_upper",
        "gamma_beta",
        "f64",
        "legacy_error_prevented_by_checks"
    ),
    spec!(
        "DGAMIT",
        "slatec::special::gamma::tricomi_incomplete_gamma",
        "gamma_beta",
        "f64",
        "legacy_error_prevented_by_checks"
    ),
    spec!(
        "DPSI",
        "slatec::special::gamma::digamma",
        "gamma_beta",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DFAC",
        "slatec::special::gamma::factorial",
        "elementary",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBINOM",
        "slatec::special::gamma::binomial_coefficient",
        "elementary",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "GAMMA",
        "slatec::special::gamma::gamma_f32",
        "gamma_beta",
        "f32",
        "fully_prechecked"
    ),
    spec_with_runtime!(
        "GAMR",
        "slatec::special::gamma::reciprocal_gamma_f32",
        "gamma_beta",
        "f32",
        "legacy_error_prevented_by_checks",
        "legacy_error_state",
        "serialized_process_global"
    ),
    spec!(
        "ALNGAM",
        "slatec::special::gamma::log_gamma_f32",
        "gamma_beta",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BETA",
        "slatec::special::gamma::beta_f32",
        "gamma_beta",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "ALBETA",
        "slatec::special::gamma::log_beta_f32",
        "gamma_beta",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BETAI",
        "slatec::special::gamma::regularized_beta_f32",
        "probability",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "GAMI",
        "slatec::special::gamma::incomplete_gamma_lower_f32",
        "gamma_beta",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "GAMIC",
        "slatec::special::gamma::incomplete_gamma_upper_f32",
        "gamma_beta",
        "f32",
        "legacy_error_prevented_by_checks"
    ),
    spec!(
        "GAMIT",
        "slatec::special::gamma::tricomi_incomplete_gamma_f32",
        "gamma_beta",
        "f32",
        "legacy_error_prevented_by_checks"
    ),
    spec!(
        "PSI",
        "slatec::special::gamma::digamma_f32",
        "gamma_beta",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "FAC",
        "slatec::special::gamma::factorial_f32",
        "elementary",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BINOM",
        "slatec::special::gamma::binomial_coefficient_f32",
        "elementary",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "DCSEVL",
        "slatec::polynomials::chebyshev::chebyshev_series",
        "polynomials",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "CSEVL",
        "slatec::polynomials::chebyshev::chebyshev_series_f32",
        "polynomials",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "DBESJ0",
        "slatec::special::bessel::bessel_j0",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBESJ1",
        "slatec::special::bessel::bessel_j1",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBESY0",
        "slatec::special::bessel::bessel_y0",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBESY1",
        "slatec::special::bessel::bessel_y1",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBESI0",
        "slatec::special::bessel::bessel_i0",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBESI1",
        "slatec::special::bessel::bessel_i1",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBSI0E",
        "slatec::special::bessel::bessel_i0_scaled",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBSI1E",
        "slatec::special::bessel::bessel_i1_scaled",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBESK0",
        "slatec::special::bessel::bessel_k0",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBESK1",
        "slatec::special::bessel::bessel_k1",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBSK0E",
        "slatec::special::bessel::bessel_k0_scaled",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "DBSK1E",
        "slatec::special::bessel::bessel_k1_scaled",
        "bessel",
        "f64",
        "fully_prechecked"
    ),
    spec!(
        "BESJ0",
        "slatec::special::bessel::bessel_j0_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESJ1",
        "slatec::special::bessel::bessel_j1_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESY0",
        "slatec::special::bessel::bessel_y0_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESY1",
        "slatec::special::bessel::bessel_y1_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESI0",
        "slatec::special::bessel::bessel_i0_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESI1",
        "slatec::special::bessel::bessel_i1_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESI0E",
        "slatec::special::bessel::bessel_i0_scaled_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESI1E",
        "slatec::special::bessel::bessel_i1_scaled_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESK0",
        "slatec::special::bessel::bessel_k0_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESK1",
        "slatec::special::bessel::bessel_k1_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESK0E",
        "slatec::special::bessel::bessel_k0_scaled_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
    spec!(
        "BESK1E",
        "slatec::special::bessel::bessel_k1_scaled_f32",
        "bessel",
        "f32",
        "fully_prechecked"
    ),
];

#[derive(Debug)]
pub struct SafeSpecialResult {
    pub snapshot_id: String,
    pub status: String,
    pub semantic_hash: String,
    pub candidate_count: usize,
    pub wrapper_count: usize,
    pub deferred_count: usize,
    pub output_dir: PathBuf,
}

pub fn generate(
    runtime_profile_dir: &Path,
    ffi_dir: &Path,
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<SafeSpecialResult> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-special-api requires --offline".to_owned(),
        ));
    }
    let runtime = read_value(&runtime_profile_dir.join("manifest.json"))?;
    require_runtime_gate(&runtime)?;
    let ffi = read_value(&ffi_dir.join("interface-inventory.json"))?;
    let selected = read_value(&selected_corpus_dir.join("manifest.json"))?;
    let snapshot = required_string(&runtime, "snapshot_id")?;
    if required_string(&ffi, "snapshot_id")? != snapshot
        || required_string(&selected, "snapshot_id")? != snapshot
    {
        return Err(CorpusError::Verification(
            "runtime-profile snapshot does not match selected corpus and raw FFI".to_owned(),
        ));
    }
    let mut specs = BTreeMap::new();
    for spec in WRAPPERS {
        if specs.insert(spec.source, spec).is_some() {
            return Err(CorpusError::Policy(format!(
                "duplicate safe-wrapper source {}",
                spec.source
            )));
        }
    }

    let columns = required_array(&ffi, "columns")?;
    let records = required_array(&ffi, "records")?;
    let ix = |name: &str| -> Result<usize> {
        columns
            .iter()
            .position(|value| value.as_str() == Some(name))
            .ok_or_else(|| {
                CorpusError::Verification(format!("raw FFI interface inventory lacks {name}"))
            })
    };
    let (id_ix, name_ix, kind_ix, subset_ix, symbol_ix, confidence_ix, batch_ix) = (
        ix("program_unit_id")?,
        ix("normalized_name")?,
        ix("kind")?,
        ix("source_subset")?,
        ix("observed_raw_symbol")?,
        ix("confidence_class")?,
        ix("binding_batch")?,
    );
    let mut candidates = Vec::new();
    let mut wrappers = Vec::new();
    let mut deferred = Vec::new();
    let mut seen_sources = BTreeSet::new();
    for record in records {
        let row = record.as_array().ok_or_else(|| {
            CorpusError::Verification("raw FFI record is not positional".to_owned())
        })?;
        let batch = row
            .get(batch_ix)
            .and_then(Value::as_str)
            .unwrap_or_default();
        let kind = row.get(kind_ix).and_then(Value::as_str).unwrap_or_default();
        if batch != "batch_scalar_functions" || kind != "function" {
            continue;
        }
        let source = row_string(row, name_ix)?;
        let program_unit_id = row_string(row, id_ix)?;
        let raw_symbol = row_string(row, symbol_ix)?;
        let confidence = row_string(row, confidence_ix)?;
        let subset = row_string(row, subset_ix)?;
        let (disposition, reason, spec) = if let Some(spec) = specs.get(source.as_str()) {
            if confidence != "generated_abi_sensitive" {
                return Err(CorpusError::Verification(format!(
                    "{} is not a profile-validated scalar raw binding",
                    source
                )));
            }
            seen_sources.insert(source.clone());
            ("included", "validated_scalar_contract", Some(*spec))
        } else {
            ("deferred", deferred_reason(&source), None)
        };
        let shape = if spec
            .map(|entry| entry.source == "CSEVL" || entry.source == "DCSEVL")
            .unwrap_or(false)
        {
            "coefficient_slice_scalar_function"
        } else {
            "scalar_function"
        };
        candidates.push(json!([
            program_unit_id,
            source,
            raw_symbol,
            subset,
            confidence,
            shape,
            disposition,
            reason,
            spec.map(|entry| entry.safe_path).unwrap_or(""),
            spec.map(|entry| entry.family).unwrap_or(""),
            spec.map(|entry| entry.precision).unwrap_or(""),
            spec.map(|entry| entry.runtime_dependency)
                .unwrap_or("unverified_runtime_dependency"),
            spec.map(|entry| entry.state_policy)
                .unwrap_or("not_applicable"),
            spec.map(|entry| entry.domain_policy)
                .unwrap_or("deferred_uncontained"),
        ]));
        if let Some(spec) = spec {
            wrappers.push(json!([
                program_unit_id,
                source,
                raw_symbol,
                spec.safe_path,
                spec.family,
                spec.precision,
                shape,
                spec.runtime_dependency,
                spec.state_policy,
                spec.domain_policy,
                "native_execution_required",
                "reference_required",
                "identity_required",
                "contained_invalid_input_required",
                "approved",
            ]));
        } else {
            deferred.push(json!([
                program_unit_id,
                source,
                raw_symbol,
                reason,
                "manual_review_required"
            ]));
        }
    }
    if seen_sources.len() != WRAPPERS.len() {
        let missing = WRAPPERS
            .iter()
            .filter(|entry| !seen_sources.contains(entry.source))
            .map(|entry| entry.source)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(CorpusError::Verification(format!(
            "configured safe wrappers are absent from the validated scalar inventory: {missing}"
        )));
    }
    sort_rows(&mut candidates, 1);
    sort_rows(&mut wrappers, 3);
    sort_rows(&mut deferred, 1);
    let family_summary = family_summary(&wrappers)?;
    let runtime_state = wrappers
        .iter()
        .map(|row| {
            let row = row.as_array().expect("constructed row");
            json!([
                row[0].clone(),
                row[1].clone(),
                row[3].clone(),
                row[7].clone(),
                row[8].clone()
            ])
        })
        .collect::<Vec<_>>();
    let mut outputs = BTreeMap::new();
    outputs.insert("special-function-candidates.json", compact(&json!({
        "schema_id":"slatec.safe-special.candidates", "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["program_unit_id","source_unit","raw_symbol","source_subset","raw_confidence_class","interface_shape","disposition","reason","safe_path","family","precision","runtime_dependency","state_policy","domain_policy"],
        "records":candidates
    }))?);
    outputs.insert("special-function-wrapper-index.json", compact(&json!({
        "schema_id":"slatec.safe-special.wrapper-index", "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot, "raw_ffi_profile":PROFILE,
        "columns":["program_unit_id","source_unit","raw_symbol","safe_path","family","precision","interface_shape","runtime_dependency","state_policy","domain_policy","native_execution","reference_test","identity_test","invalid_input_containment","review_state"],
        "records":wrappers
    }))?);
    outputs.insert("special-function-family-summary.json", compact(&json!({
        "schema_id":"slatec.safe-special.family-summary", "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot, "columns":["family","precision","wrapper_count"], "records":family_summary
    }))?);
    outputs.insert("special-function-runtime-state.json", compact(&json!({
        "schema_id":"slatec.safe-special.runtime-state", "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot, "columns":["program_unit_id","source_unit","safe_path","runtime_dependency","state_policy"], "records":runtime_state
    }))?);
    outputs.insert("special-function-deferred.json", compact(&json!({
        "schema_id":"slatec.safe-special.deferred", "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot, "columns":["program_unit_id","source_unit","raw_symbol","reason","review_state"], "records":deferred
    }))?);
    let semantic_hash = semantic_hash(&outputs);
    outputs.insert(
        "special-function-validation-summary.md",
        summary(&snapshot, candidates.len(), wrappers.len(), deferred.len()).into_bytes(),
    );
    outputs.insert("special-function-manifest.json", compact(&json!({
        "id": stable_id("safe-special", &[&snapshot, &semantic_hash]),
        "schema_id":"slatec.safe-special.manifest", "schema_version":SCHEMA_VERSION,
        "snapshot_id":snapshot, "created_by":format!("{TOOL_NAME} {TOOL_VERSION}"),
        "semantic_version":SEMANTIC_VERSION, "raw_ffi_profile":PROFILE,
        "runtime_profile_validation": {"abi_validated":true,"machine_constants_validated":true,"legacy_error_behavior_validated":true,"fnlib_initialization_validated":true},
        "candidate_count":candidates.len(), "wrapper_count":wrappers.len(), "deferred_count":deferred.len(),
        "output_semantic_hash":semantic_hash, "status":"success_with_review_items",
        "scope":"Compact safe special-function inventory only; detailed native evidence remains ignored."
    }))?);
    enforce_size(&outputs)?;
    promote(output_dir, &outputs)?;
    Ok(SafeSpecialResult {
        snapshot_id: snapshot,
        status: "success_with_review_items".to_owned(),
        semantic_hash,
        candidate_count: candidates.len(),
        wrapper_count: wrappers.len(),
        deferred_count: deferred.len(),
        output_dir: output_dir.to_owned(),
    })
}

fn require_runtime_gate(runtime: &Value) -> Result<()> {
    let validation = runtime["validation"].as_object().ok_or_else(|| {
        CorpusError::Verification("runtime-profile manifest lacks validation state".to_owned())
    })?;
    for field in [
        "abi_validated",
        "machine_constants_validated",
        "legacy_error_behavior_validated",
        "fnlib_initialization_validated",
    ] {
        if validation.get(field).and_then(Value::as_bool) != Some(true) {
            return Err(CorpusError::Verification(format!(
                "runtime-profile gate {field} is not true"
            )));
        }
    }
    Ok(())
}

fn deferred_reason(source: &str) -> &'static str {
    if matches!(source, "RAND" | "RGAUSS" | "RUNIF") {
        "mutable_global_state"
    } else if source.starts_with("D9") || source.starts_with("R9") {
        "internal_approximant"
    } else if matches!(
        source,
        "ACOS"
            | "ALOG"
            | "ALOG10"
            | "ASIN"
            | "ATAN"
            | "ATAN2"
            | "COS"
            | "COSH"
            | "DACOS"
            | "DASIN"
            | "DATAN"
            | "DATAN2"
            | "DCOS"
            | "DCOSH"
            | "DEXP"
            | "DLOG"
            | "DLOG10"
            | "DSIN"
            | "DSINH"
            | "DSQRT"
            | "DTAN"
            | "DTANH"
            | "EXP"
            | "SIN"
            | "SINH"
            | "SQRT"
            | "TAN"
            | "TANH"
    ) {
        "portable_intrinsic_duplicate"
    } else if matches!(
        source,
        "BVALU" | "DBVALU" | "CV" | "DCV" | "PPVAL" | "DPPVAL"
    ) {
        "dynamic_workspace_or_array"
    } else {
        "unverified_public_contract"
    }
}

fn family_summary(wrappers: &[Value]) -> Result<Vec<Value>> {
    let mut counts = BTreeMap::<(String, String), usize>::new();
    for wrapper in wrappers {
        let row = wrapper.as_array().ok_or_else(|| {
            CorpusError::Verification("constructed wrapper row is invalid".to_owned())
        })?;
        let family = row[4].as_str().unwrap_or_default().to_owned();
        let precision = row[5].as_str().unwrap_or_default().to_owned();
        *counts.entry((family, precision)).or_default() += 1;
    }
    Ok(counts
        .into_iter()
        .map(|((family, precision), count)| json!([family, precision, count]))
        .collect())
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
fn required_array<'a>(value: &'a Value, field: &str) -> Result<&'a Vec<Value>> {
    value[field]
        .as_array()
        .ok_or_else(|| CorpusError::Verification(format!("missing array {field}")))
}
fn row_string(row: &[Value], index: usize) -> Result<String> {
    row.get(index)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| {
            CorpusError::Verification(format!(
                "malformed raw FFI record field {index}: {}",
                row.get(index).unwrap_or(&Value::Null)
            ))
        })
}
fn sort_rows(rows: &mut [Value], index: usize) {
    rows.sort_by(|left, right| left[index].as_str().cmp(&right[index].as_str()));
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
    if total > MAX_TOTAL_BYTES {
        return Err(CorpusError::Policy(format!(
            "safe-special metadata is {total} bytes, above {MAX_TOTAL_BYTES}"
        )));
    }
    for (name, bytes) in outputs {
        if bytes.len() > MAX_FILE_BYTES {
            return Err(CorpusError::Policy(format!(
                "{name} is {} bytes, above {MAX_FILE_BYTES}",
                bytes.len()
            )));
        }
    }
    Ok(())
}
fn promote(output_dir: &Path, outputs: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    let parent = output_dir.parent().ok_or_else(|| {
        CorpusError::Policy("safe-special output needs a parent directory".to_owned())
    })?;
    fs::create_dir_all(parent)?;
    let staging = parent.join(format!(
        ".{}.safe-special-staging",
        output_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("safe-api")
    ));
    if staging.exists() {
        fs::remove_dir_all(&staging)?;
    }
    fs::create_dir_all(&staging)?;
    for (name, bytes) in outputs {
        fs::write(staging.join(name), bytes)?;
    }
    fs::create_dir_all(output_dir)?;
    // This stage extends the existing safe-API metadata tree, which also
    // contains independent BLAS inventories.  Promote only the deterministic
    // special-function files instead of deleting unrelated generated outputs.
    for name in outputs.keys() {
        let destination = output_dir.join(name);
        if destination.exists() {
            fs::remove_file(&destination)?;
        }
        fs::rename(staging.join(name), destination)?;
    }
    fs::remove_dir(staging)?;
    Ok(())
}

fn summary(snapshot: &str, candidates: usize, wrappers: usize, deferred: usize) -> String {
    format!(
        "# Runtime-validated SLATEC special-function API\n\n- Snapshot: `{snapshot}`\n- Raw ABI profile: `{PROFILE}`\n- Scalar raw-function candidates classified: {candidates}\n- Safe wrappers: {wrappers}\n- Deferred candidates: {deferred}\n- Runtime policy: FNLIB saved/error state is process-global and serialized only for this safe special-function surface\n- Domain policy: each exposed routine applies a conservative pre-call domain check; unsupported contracts remain deferred\n\nThe original Fortran implementation remains authoritative. This compact inventory contains no source text, archive, object, or native log.\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_gate_requires_every_validation_flag() {
        let valid = json!({"validation":{"abi_validated":true,"machine_constants_validated":true,"legacy_error_behavior_validated":true,"fnlib_initialization_validated":true}});
        assert!(require_runtime_gate(&valid).is_ok());
        let invalid = json!({"validation":{"abi_validated":true,"machine_constants_validated":false,"legacy_error_behavior_validated":true,"fnlib_initialization_validated":true}});
        assert!(require_runtime_gate(&invalid).is_err());
    }

    #[test]
    fn configured_wrapper_sources_are_unique() {
        let names = WRAPPERS
            .iter()
            .map(|entry| entry.source)
            .collect::<BTreeSet<_>>();
        assert_eq!(names.len(), WRAPPERS.len());
        assert!(WRAPPERS.len() >= 50);
    }

    #[test]
    fn committed_inputs_regenerate_identically_in_separate_roots() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let temp = tempfile::tempdir().unwrap();
        let first = temp.path().join("first/safe-api");
        let second = temp.path().join("second/safe-api");
        let paths = |output: &Path| {
            generate(
                &root.join("generated/runtime-profile"),
                &root.join("generated/ffi"),
                &root.join("generated/selected-corpus"),
                output,
                true,
            )
            .unwrap()
        };
        let first_result = paths(&first);
        let second_result = paths(&second);
        assert_eq!(first_result.semantic_hash, second_result.semantic_hash);
        for name in [
            "special-function-candidates.json",
            "special-function-wrapper-index.json",
            "special-function-family-summary.json",
            "special-function-runtime-state.json",
            "special-function-deferred.json",
            "special-function-validation-summary.md",
            "special-function-manifest.json",
        ] {
            assert_eq!(
                fs::read(first.join(name)).unwrap(),
                fs::read(second.join(name)).unwrap()
            );
        }
    }
}
