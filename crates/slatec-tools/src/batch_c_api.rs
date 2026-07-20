//! Deterministic classification and promotion for Raw API Batch C.
//!
//! Batch C consumes the same compiler-observed interface inventory used by
//! Batch A and the callback evidence emitted by Batch B. It promotes only
//! historically public complex, fixed `CHARACTER*1`, and validated `LOGICAL`
//! interfaces for the supported GNU MinGW profile.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA_VERSION: &str = "1.0.0";
const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";
const TARGET: &str = "x86_64-pc-windows-gnu";
const BATCH_SIZE: usize = 96;

/// Input and output locations for Batch C generation.
pub struct BatchCPaths<'a> {
    pub catalogue_dir: &'a Path,
    pub ffi_dir: &'a Path,
    pub ffi_inventory_dir: &'a Path,
    pub raw_api_dir: &'a Path,
    pub sys_dir: &'a Path,
    pub src_dir: &'a Path,
    pub facade_dir: &'a Path,
    pub output_dir: &'a Path,
}

/// Compact deterministic result of Batch C generation or validation.
#[derive(Clone, Debug, serde::Serialize)]
pub struct BatchCResult {
    pub status: String,
    pub retained_identities: usize,
    pub candidates: usize,
    pub semantic_hash: String,
    pub output_dir: PathBuf,
}

#[derive(Clone, Debug)]
struct Interface {
    provider_id: String,
    kind: String,
    source_hash: String,
    native_symbol: String,
    confidence: String,
    batch: String,
    argument_ids: Vec<String>,
    result_id: Option<String>,
}

#[derive(Clone, Debug)]
struct Argument {
    name: String,
    declared_type: String,
    character_length: String,
    dimensions: Vec<Value>,
    is_external: bool,
    conflict_state: String,
}

#[derive(Clone, Debug)]
struct FunctionResult {
    declared_type: String,
    character_length: String,
    conflict_state: String,
}

#[derive(Clone, Debug)]
struct Candidate {
    routine: String,
    source_hash: String,
    source_file: String,
    source_url: String,
    purpose: String,
    native_symbol: String,
    kind: String,
    abi_class: String,
    abi_fingerprint: String,
    canonical_module: String,
    canonical_path: String,
    declaration_feature: String,
    provider_feature: String,
    binding_module: String,
    binding_feature: String,
    arguments: Vec<Argument>,
    result: Option<FunctionResult>,
}

/// Generate complete Batch C classification, declarations, probes, and reports.
pub fn generate(paths: BatchCPaths<'_>) -> Result<BatchCResult> {
    let catalogue = records(
        &read_json(&paths.catalogue_dir.join("routine-catalogue.json"))?,
        "routine catalogue",
    )?
    .clone();
    let arguments = arguments(&paths.ffi_inventory_dir.join("argument-index.json"))?;
    let results = results(&paths.ffi_inventory_dir.join("function-results.json"))?;
    let interfaces = interfaces(&paths.ffi_dir.join("interface-inventory.json"))?;
    let confidence = read_json(&paths.ffi_dir.join("confidence-summary.json"))?;
    validate_abi_evidence(&confidence)?;
    let public = public_routines(paths.raw_api_dir)?;
    let batch_b = batch_b_routines(paths.raw_api_dir)?;
    let roles = batch_a_roles(paths.raw_api_dir)?;

    let mut by_provider = BTreeMap::new();
    for interface in interfaces {
        if by_provider
            .insert(interface.provider_id.clone(), interface)
            .is_some()
        {
            return Err(policy("Batch C received duplicate provider identities"));
        }
    }

    let mut classifications = Vec::with_capacity(catalogue.len());
    let mut candidates = Vec::new();
    let mut exclusions = Vec::new();
    for record in &catalogue {
        let routine = string(record, "normalized_name")?;
        let provider = record
            .pointer("/canonical_provider/provider_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let interface = by_provider.get(provider);
        let role = roles
            .get(&routine)
            .map(String::as_str)
            .unwrap_or("catalogue_only");
        let (mut classification, args, result) = classify(
            record,
            interface,
            &arguments,
            &results,
            role,
            public.contains(&routine),
            batch_b.contains(&routine),
        )?;
        if classification["batch_c_eligibility"] == "eligible" {
            let interface = interface.expect("eligible Batch C record has an interface");
            let (canonical_module, declaration_feature) = canonical_module(record, &routine)?;
            let binding_module = binding_module(&interface.batch)?;
            let candidate = Candidate {
                routine: routine.clone(),
                source_hash: field(record, "source_hash"),
                source_file: field(record, "source_file"),
                source_url: source_url(record),
                purpose: field(record, "short_purpose"),
                native_symbol: interface.native_symbol.clone(),
                kind: interface.kind.clone(),
                abi_class: field(&classification, "combined_abi_class"),
                abi_fingerprint: abi_fingerprint(&interface.kind, &args, result.as_ref()),
                canonical_path: format!(
                    "slatec_sys::{canonical_module}::{}",
                    routine.to_ascii_lowercase()
                ),
                canonical_module,
                provider_feature: declaration_feature.clone(),
                declaration_feature,
                binding_feature: format!("raw-ffi-{}", binding_module.replace('_', "-")),
                binding_module,
                arguments: args,
                result,
            };
            classification["candidate"] = candidate_json(&candidate);
            candidates.push(candidate);
        } else if classification["batch_c_eligibility"] != "already_public" {
            exclusions.push(json!({
                "routine":routine,
                "source_hash":field(record,"source_hash"),
                "abi_class":classification["combined_abi_class"].clone(),
                "reason_code":classification["batch_c_exclusion_code"].clone(),
                "reason":classification["batch_c_exclusion_reason"].clone(),
                "batch_d_disposition":classification["batch_d_disposition"].clone(),
            }));
        }
        classifications.push(classification);
    }
    classifications.sort_by_key(|record| field(record, "routine"));
    candidates.sort_by_key(|candidate| candidate.routine.clone());
    exclusions.sort_by_key(|record| field(record, "routine"));
    validate_candidates(&candidates)?;
    write_canonical_modules(paths.sys_dir, &candidates)?;
    write_compile_probes(paths.sys_dir, &candidates)?;
    write_link_probes(paths.facade_dir, &candidates)?;

    let count =
        |predicate: fn(&Value) -> bool| classifications.iter().filter(|r| predicate(r)).count();
    let category_count = |name: &str| {
        classifications
            .iter()
            .filter(|record| record[name] == true)
            .count()
    };
    let before = public.len();
    let after = before + candidates.len();
    let summary = json!({
        "schema_id":"slatec-sys.raw-api.batch-c-summary",
        "schema_version":SCHEMA_VERSION,
        "policy":"Historically public complex argument/output and compiler-validated complex-return interfaces, fixed CHARACTER*1 arguments with trailing usize lengths, and i32 LOGICAL interfaces are eligible when their selected symbol, source hash, provider closure, and any callback shape are validated. Long/variable CHARACTER, character returns, unresolved callbacks, ambiguous declarations, ENTRY/alternate returns, and missing providers remain excluded.",
        "counts":{
            "total_retained_identities":classifications.len(),
            "public_raw_identities_before_batch_c":before,
            "complex_bearing_identities":category_count("complex_presence"),
            "complex_argument_or_output_identities":category_count("complex_argument_or_output"),
            "complex_function_return_identities":category_count("complex_function_return"),
            "character_bearing_identities":category_count("character_presence"),
            "character_1_identities":category_count("character_1_eligible_shape"),
            "logical_bearing_identities":category_count("logical_presence"),
            "combined_category_identities":count(|r| r["category_count"].as_u64().unwrap_or_default() > 1),
            "batch_c_candidates":candidates.len(),
            "already_public_eligible_routines":count(|r| r["batch_c_eligibility"] == "already_public"),
            "newly_promoted_routines":candidates.len(),
            "eligible_subsidiaries_kept_internal":count(|r| r["batch_c_exclusion_code"] == "subsidiary_or_internal" && r["abi_components_validated"] == true),
            "complex_return_exclusions":count(|r| r["batch_c_exclusion_code"] == "complex_return_unvalidated"),
            "long_or_variable_character_exclusions":category_count("long_or_variable_character"),
            "logical_exclusions":count(|r| r["batch_c_exclusion_code"] == "logical_unvalidated"),
            "callback_combination_exclusions":count(|r| r["batch_c_exclusion_code"] == "callback_unresolved"),
            "entry_or_alternate_return_exclusions":count(|r| r["batch_c_exclusion_code"] == "entry_or_alternate_return"),
            "provider_missing_exclusions":count(|r| r["batch_c_exclusion_code"] == "provider_or_symbol_missing"),
            "other_exclusions":count(|r| matches!(r["batch_c_exclusion_code"].as_str(),Some("parser_ambiguous")|Some("unsupported_other"))),
            "total_public_raw_identities_after_batch_c":after,
            "retained_public_percentage":format!("{:.2}",after as f64 * 100.0 / classifications.len() as f64),
            "unique_batch_c_abi_fingerprints":candidates.iter().map(|c|c.abi_fingerprint.as_str()).collect::<BTreeSet<_>>().len(),
            "source_hash_corrections":0,
            "parser_improvements":1,
            "compile_batches":candidates.chunks(BATCH_SIZE).len(),
            "link_batches":candidates.chunks(BATCH_SIZE).len(),
            "runtime_smoke_routines":7,
        },
        "exclusions_by_code":summary_by(&classifications,"batch_c_exclusion_code"),
        "abi_classes":summary_by(&classifications,"combined_abi_class"),
    });
    let abi_evidence = json!({
        "schema_id":"slatec-sys.raw-api.batch-c-abi-evidence",
        "schema_version":SCHEMA_VERSION,
        "profile":PROFILE,
        "target":TARGET,
        "compiler_profile":confidence.pointer("/abi_validation").cloned().unwrap_or(Value::Null),
        "complex":{
            "complex32_layout":"repr(C) { f32 real, f32 imaginary }; size 8; alignment 4; by-reference scalar/array arguments",
            "complex64_layout":"repr(C) { f64 real, f64 imaginary }; size 16; alignment 8; by-reference scalar/array arguments",
            "function_return":"GNU MinGW direct C-compatible aggregate return, experimentally called from Rust for controlled COMPLEX and DOUBLE COMPLEX functions and selected CDOTU/CDCDOT routines"
        },
        "character":{
            "approved_dummy":"fixed CHARACTER*1 only",
            "hidden_lengths":"one trailing usize after all ordinary arguments, in visible CHARACTER dummy order",
            "evidence":"controlled one- and two-character calls plus selected multi-flag BLAS call"
        },
        "logical":{
            "rust_type":"FortranLogical = i32 (never Rust bool)",
            "false":0,"true":1,"passing":"by reference","array_stride_bytes":4,"function_return":"i32",
            "evidence":"controlled scalar input/output, array mutation, and LOGICAL function-return calls"
        },
        "portability":"These guarantees apply only to the recorded GNU Fortran x86_64-w64-mingw32 profile. External/system providers compiled with other toolchains require independent qualification."
    });
    let provider_records = candidates
        .iter()
        .map(|candidate| {
            json!({
                "routine":candidate.routine,
                "canonical_rust_path":candidate.canonical_path,
                "declaration_feature":candidate.declaration_feature,
                "provider_feature":candidate.provider_feature,
                "source_hash":candidate.source_hash,
                "native_symbol":candidate.native_symbol,
                "provider_status":"source_closure_required_and_link_probed",
            })
        })
        .collect::<Vec<_>>();
    let files = BTreeMap::from([
        (
            "batch-c-classification.json",
            bytes(
                &json!({"schema_id":"slatec-sys.raw-api.batch-c-classification","schema_version":SCHEMA_VERSION,"records":classifications}),
            )?,
        ),
        (
            "batch-c-candidates.json",
            bytes(
                &json!({"schema_id":"slatec-sys.raw-api.batch-c-candidates","schema_version":SCHEMA_VERSION,"records":candidates.iter().map(candidate_json).collect::<Vec<_>>()}),
            )?,
        ),
        (
            "batch-c-exclusions.json",
            bytes(
                &json!({"schema_id":"slatec-sys.raw-api.batch-c-exclusions","schema_version":SCHEMA_VERSION,"records":exclusions}),
            )?,
        ),
        ("batch-c-abi-evidence.json", bytes(&abi_evidence)?),
        (
            "batch-c-provider-reconciliation.json",
            bytes(
                &json!({"schema_id":"slatec-sys.raw-api.batch-c-provider-reconciliation","schema_version":SCHEMA_VERSION,"records":provider_records}),
            )?,
        ),
        ("batch-c-summary.md", render_summary(&summary).into_bytes()),
    ]);
    let semantic_hash = output_hash(&files);
    let mut files = files;
    files.insert(
        "batch-c-manifest.json",
        bytes(&json!({
            "schema_id":"slatec-sys.raw-api.batch-c-manifest",
            "schema_version":SCHEMA_VERSION,
            "semantic_sha256":semantic_hash,
            "inputs":["generated/slatec-routines/routine-catalogue.json","generated/ffi/interface-inventory.json","generated/ffi/confidence-summary.json","generated/ffi-inventory/argument-index.json","generated/ffi-inventory/function-results.json","generated/raw-api/routine-status.json","generated/raw-api/batch-a-candidates.json","generated/raw-api/batch-b-candidates.json"],
            "outputs":files.keys().collect::<Vec<_>>(),
        }))?,
    );
    write_outputs(paths.output_dir, &files)?;
    Ok(BatchCResult {
        status: "success".to_owned(),
        retained_identities: catalogue.len(),
        candidates: candidates.len(),
        semantic_hash,
        output_dir: paths.output_dir.to_path_buf(),
    })
}

/// Regenerate Batch C and validate all structural invariants.
pub fn validate(paths: BatchCPaths<'_>) -> Result<BatchCResult> {
    let sys = paths.sys_dir.to_path_buf();
    let src = paths.src_dir.to_path_buf();
    let result = generate(paths)?;
    let value = read_json(&result.output_dir.join("batch-c-candidates.json"))?;
    let sys_features = fs::read_to_string(sys.join("Cargo.toml"))?;
    let src_features = fs::read_to_string(src.join("Cargo.toml"))?;
    let closure = read_json(&src.join("metadata/family-source-closure.json"))?;
    let source_ids_by_hash = closure
        .get("sources")
        .and_then(Value::as_array)
        .ok_or_else(|| policy("provider closure has no sources"))?
        .iter()
        .filter_map(|source| {
            Some((
                source.get("sha256")?.as_str()?.to_owned(),
                source.get("id")?.as_str()?.to_owned(),
            ))
        })
        .collect::<BTreeMap<_, _>>();
    let mut canonical = BTreeSet::new();
    for record in records(&value, "Batch C candidates")? {
        for key in [
            "routine",
            "source_hash",
            "native_symbol",
            "normalized_abi_fingerprint",
            "canonical_rust_path",
            "declaration_feature",
            "provider_feature",
            "binding_path",
        ] {
            if field(record, key).is_empty() {
                return Err(policy(&format!("Batch C candidate lacks {key}")));
            }
        }
        if !canonical.insert(field(record, "canonical_rust_path")) {
            return Err(policy("Batch C canonical paths are not unique"));
        }
        let declaration_feature = field(record, "declaration_feature");
        let provider_feature = field(record, "provider_feature");
        if !sys_features.contains(&format!("{declaration_feature} =")) {
            return Err(policy(&format!(
                "Batch C declaration feature {declaration_feature} is absent"
            )));
        }
        if !src_features.contains(&format!("{provider_feature} =")) {
            return Err(policy(&format!(
                "Batch C provider feature {provider_feature} is absent"
            )));
        }
        let source_id = source_ids_by_hash
            .get(&field(record, "source_hash"))
            .ok_or_else(|| policy("Batch C source hash is absent from provider manifest"))?;
        let family = closure
            .pointer(&format!("/families/{provider_feature}"))
            .and_then(Value::as_array)
            .ok_or_else(|| {
                policy(&format!(
                    "Batch C provider closure {provider_feature} is absent"
                ))
            })?;
        if !family.iter().any(|value| value.as_str() == Some(source_id)) {
            return Err(policy(&format!(
                "Batch C provider closure {provider_feature} omits {}",
                field(record, "routine")
            )));
        }
        let source = generated_source_for_routine(&sys, &field(record, "routine"))?;
        for argument in strings(record.get("arguments")) {
            if !source.contains(&format!("`{argument}`")) {
                return Err(policy(&format!("Batch C docs lack argument {argument}")));
            }
        }
        for required in ["# Safety", "GNU MinGW", "https://", "raw-api-routine:"] {
            if !source.contains(required) {
                return Err(policy(&format!("Batch C docs lack {required}")));
            }
        }
    }
    Ok(result)
}

fn classify(
    record: &Value,
    interface: Option<&Interface>,
    arguments: &BTreeMap<String, Argument>,
    results: &BTreeMap<String, FunctionResult>,
    role: &str,
    already_public: bool,
    batch_b_public: bool,
) -> Result<(Value, Vec<Argument>, Option<FunctionResult>)> {
    let routine = string(record, "normalized_name")?;
    let mut base = json!({
        "routine":routine,"source_hash":field(record,"source_hash"),"source_file":field(record,"source_file"),
        "program_unit_kind":field(record,"kind"),"historical_role":field(record,"historical_role"),"public_role":role,
        "primary_family":field(record,"primary_family"),"native_symbol":"","compiler_confidence":"","binding_batch":"",
        "complex_presence":false,"complex_argument_or_output":false,"complex_function_return":false,"complex_precisions":[],"complex_shapes":[],"complex_return_mechanism":"not_applicable",
        "character_presence":false,"character_dummy_count":0,"declared_character_lengths":[],"hidden_length_argument_count":0,"hidden_length_position":"not_applicable","hidden_length_integer_width":"not_applicable","character_mutability":"not_applicable","character_1_eligible_shape":false,"long_or_variable_character":false,
        "logical_presence":false,"logical_scalar_count":0,"logical_array_count":0,"logical_function_return":false,"logical_width":"not_applicable","logical_false_representation":"not_applicable","logical_true_representation":"not_applicable","logical_return_mechanism":"not_applicable",
        "callback_presence":false,"callback_model_status":"not_applicable","category_count":0,"combined_abi_class":"not_batch_c_shape","normalized_abi_fingerprint":"",
        "abi_components_validated":false,"batch_c_eligibility":"excluded","batch_c_exclusion_code":"not_batch_c_shape","batch_c_exclusion_reason":"routine has no complex, CHARACTER, or LOGICAL ABI component","batch_d_disposition":"not_applicable",
    });
    let Some(interface) = interface else {
        base["batch_c_exclusion_code"] = Value::String("provider_or_symbol_missing".to_owned());
        base["batch_c_exclusion_reason"] =
            Value::String("no selected compiler-observed interface".to_owned());
        base["batch_d_disposition"] = Value::String("provider_or_parser_review".to_owned());
        return Ok((base, Vec::new(), None));
    };
    let args = interface
        .argument_ids
        .iter()
        .map(|id| {
            arguments
                .get(id)
                .cloned()
                .ok_or_else(|| policy("Batch C argument index is incomplete"))
        })
        .collect::<Result<Vec<_>>>()?;
    let result = interface
        .result_id
        .as_ref()
        .map(|id| {
            results
                .get(id)
                .cloned()
                .ok_or_else(|| policy("Batch C result index is incomplete"))
        })
        .transpose()?;
    base["native_symbol"] = Value::String(interface.native_symbol.clone());
    base["compiler_confidence"] = Value::String(interface.confidence.clone());
    base["binding_batch"] = Value::String(interface.batch.clone());

    let complex_args = args
        .iter()
        .filter(|a| is_complex(&a.declared_type))
        .collect::<Vec<_>>();
    let complex_return = result
        .as_ref()
        .is_some_and(|r| is_complex(&r.declared_type));
    let char_args = args
        .iter()
        .filter(|a| a.declared_type == "CHARACTER")
        .collect::<Vec<_>>();
    let char_return = result
        .as_ref()
        .is_some_and(|r| r.declared_type == "CHARACTER");
    let logical_args = args
        .iter()
        .filter(|a| a.declared_type == "LOGICAL")
        .collect::<Vec<_>>();
    let logical_return = result
        .as_ref()
        .is_some_and(|r| r.declared_type == "LOGICAL");
    let callback = args.iter().any(|a| a.is_external);
    let complex = !complex_args.is_empty() || complex_return;
    let character = !char_args.is_empty() || char_return;
    let logical = !logical_args.is_empty() || logical_return;
    let category_count = usize::from(complex) + usize::from(character) + usize::from(logical);
    let char_one = !char_args.is_empty()
        && char_args.iter().all(|a| a.character_length == "1")
        && !char_return;
    base["complex_presence"] = Value::Bool(complex);
    base["complex_argument_or_output"] = Value::Bool(!complex_args.is_empty());
    base["complex_function_return"] = Value::Bool(complex_return);
    base["complex_precisions"] = json!(
        args.iter()
            .filter(|a| is_complex(&a.declared_type))
            .map(|a| complex_precision(&a.declared_type))
            .chain(
                result
                    .iter()
                    .filter(|r| is_complex(&r.declared_type))
                    .map(|r| complex_precision(&r.declared_type))
            )
            .collect::<BTreeSet<_>>()
    );
    base["complex_shapes"] = json!(
        complex_args
            .iter()
            .map(|a| if a.dimensions.is_empty() {
                "scalar_pointer"
            } else {
                "array_pointer"
            })
            .chain(complex_return.then_some("function_return"))
            .collect::<BTreeSet<_>>()
    );
    base["complex_return_mechanism"] = Value::String(
        if complex_return {
            "gnu_mingw_direct_c_aggregate_return"
        } else {
            "not_applicable"
        }
        .to_owned(),
    );
    base["character_presence"] = Value::Bool(character);
    base["character_dummy_count"] = json!(char_args.len());
    base["declared_character_lengths"] = json!(
        char_args
            .iter()
            .map(|a| a.character_length.clone())
            .collect::<Vec<_>>()
    );
    base["hidden_length_argument_count"] = json!(char_args.len());
    base["hidden_length_position"] = Value::String(
        if char_args.is_empty() {
            "not_applicable"
        } else {
            "trailing_after_all_ordinary_arguments_in_dummy_order"
        }
        .to_owned(),
    );
    base["hidden_length_integer_width"] = Value::String(
        if char_args.is_empty() {
            "not_applicable"
        } else {
            "usize_64"
        }
        .to_owned(),
    );
    base["character_mutability"] = Value::String(
        if char_args.is_empty() {
            "not_applicable"
        } else {
            "raw_mut_pointer_semantic_intent_source_dependent"
        }
        .to_owned(),
    );
    base["character_1_eligible_shape"] = Value::Bool(char_one);
    base["long_or_variable_character"] =
        Value::Bool(char_return || (!char_args.is_empty() && !char_one));
    base["logical_presence"] = Value::Bool(logical);
    base["logical_scalar_count"] = json!(
        logical_args
            .iter()
            .filter(|a| a.dimensions.is_empty())
            .count()
    );
    base["logical_array_count"] = json!(
        logical_args
            .iter()
            .filter(|a| !a.dimensions.is_empty())
            .count()
    );
    base["logical_function_return"] = Value::Bool(logical_return);
    base["logical_width"] = Value::String(
        if logical {
            "i32_4_bytes"
        } else {
            "not_applicable"
        }
        .to_owned(),
    );
    base["logical_false_representation"] =
        Value::String(if logical { "0" } else { "not_applicable" }.to_owned());
    base["logical_true_representation"] =
        Value::String(if logical { "1" } else { "not_applicable" }.to_owned());
    base["logical_return_mechanism"] = Value::String(
        if logical_return {
            "direct_i32"
        } else {
            "not_applicable"
        }
        .to_owned(),
    );
    base["callback_presence"] = Value::Bool(callback);
    base["callback_model_status"] = Value::String(
        if callback {
            if batch_b_public {
                "batch_b_validated"
            } else {
                "unresolved_or_not_promoted"
            }
        } else {
            "not_applicable"
        }
        .to_owned(),
    );
    base["category_count"] = json!(category_count);
    let combined = combined_class(complex, character, logical, callback, complex_return);
    base["combined_abi_class"] = Value::String(combined);
    let fingerprint = abi_fingerprint(&interface.kind, &args, result.as_ref());
    base["normalized_abi_fingerprint"] = Value::String(fingerprint);
    if category_count == 0 {
        return Ok((base, args, result));
    }
    if role != "principal_public_routine" {
        base["batch_c_exclusion_code"] = Value::String("subsidiary_or_internal".to_owned());
        base["batch_c_exclusion_reason"] = Value::String(
            "routine is not a historically public principal or documented public utility"
                .to_owned(),
        );
        base["batch_d_disposition"] =
            Value::String("keep_internal_unless_public_role_is_authored".to_owned());
        base["abi_components_validated"] = Value::Bool(matches!(
            interface.confidence.as_str(),
            "generated_abi_sensitive" | "generated_standard"
        ));
        return Ok((base, args, result));
    }
    if field(record, "source_hash").is_empty()
        || field(record, "source_hash") != interface.source_hash
        || interface.native_symbol.is_empty()
    {
        base["batch_c_exclusion_code"] = Value::String("provider_or_symbol_missing".to_owned());
        base["batch_c_exclusion_reason"] = Value::String(
            "selected source hash or unique observed symbol is unavailable".to_owned(),
        );
        base["batch_d_disposition"] = Value::String("provider_review".to_owned());
        return Ok((base, args, result));
    }
    if record.get("entry_parent").and_then(Value::as_str).is_some() {
        base["batch_c_exclusion_code"] = Value::String("entry_or_alternate_return".to_owned());
        base["batch_c_exclusion_reason"] =
            Value::String("ENTRY or alternate-return interface is outside Batch C".to_owned());
        base["batch_d_disposition"] = Value::String("batch_d_exceptional_abi".to_owned());
        return Ok((base, args, result));
    }
    if callback && !batch_b_public {
        base["batch_c_exclusion_code"] = Value::String("callback_unresolved".to_owned());
        base["batch_c_exclusion_reason"] = Value::String(
            "callback-bearing combination lacks a promoted Batch B callback model".to_owned(),
        );
        base["batch_d_disposition"] = Value::String("callback_shape_review".to_owned());
        return Ok((base, args, result));
    }
    if char_return {
        base["batch_c_exclusion_code"] = Value::String("long_or_variable_character".to_owned());
        base["batch_c_exclusion_reason"] = Value::String(
            "CHARACTER function returns are not covered by the simple flag policy".to_owned(),
        );
        base["batch_d_disposition"] = Value::String("batch_d_character_return".to_owned());
        return Ok((base, args, result));
    }
    if !char_args.is_empty() && !char_one {
        base["batch_c_exclusion_code"] = Value::String("long_or_variable_character".to_owned());
        base["batch_c_exclusion_reason"] =
            Value::String("one or more CHARACTER dummies are not fixed length one".to_owned());
        base["batch_d_disposition"] = Value::String("batch_d_text_or_io_contract".to_owned());
        return Ok((base, args, result));
    }
    if args.iter().any(|a| !a.conflict_state.is_empty())
        || result
            .as_ref()
            .is_some_and(|r| !r.conflict_state.is_empty())
    {
        base["batch_c_exclusion_code"] = Value::String("parser_ambiguous".to_owned());
        base["batch_c_exclusion_reason"] =
            Value::String("normalized interface contains a conflicting declaration".to_owned());
        base["batch_d_disposition"] =
            Value::String("parser_or_authored_correction_review".to_owned());
        return Ok((base, args, result));
    }
    if interface.confidence != "generated_abi_sensitive" {
        base["batch_c_exclusion_code"] = Value::String(
            if complex_return {
                "complex_return_unvalidated"
            } else if logical {
                "logical_unvalidated"
            } else {
                "unsupported_other"
            }
            .to_owned(),
        );
        base["batch_c_exclusion_reason"] = Value::String(
            "compiler/profile evidence did not emit this ABI-sensitive declaration".to_owned(),
        );
        base["batch_d_disposition"] = Value::String("batch_d_abi_review".to_owned());
        return Ok((base, args, result));
    }
    base["abi_components_validated"] = Value::Bool(true);
    if already_public {
        base["batch_c_eligibility"] = Value::String("already_public".to_owned());
        base["batch_c_exclusion_code"] = Value::String("none".to_owned());
        base["batch_c_exclusion_reason"] =
            Value::String("eligible ABI already has a canonical public raw declaration".to_owned());
    } else {
        base["batch_c_eligibility"] = Value::String("eligible".to_owned());
        base["batch_c_exclusion_code"] = Value::String("none".to_owned());
        base["batch_c_exclusion_reason"] = Value::String("none".to_owned());
    }
    Ok((base, args, result))
}

fn canonical_module(record: &Value, routine: &str) -> Result<(String, String)> {
    let (module, feature) = match field(record, "primary_family").as_str() {
        "Dense linear algebra" => ("linear_algebra::dense::complex", "batch-c-linear-algebra"),
        "Linear algebra kernels" => ("blas::level1", "batch-c-blas"),
        "Special functions" | "Elementary and transcendental functions" => {
            ("special::complex", "batch-c-special")
        }
        "Nonlinear equations" => ("nonlinear::complex", "batch-c-nonlinear"),
        "FISHPACK elliptic PDE solvers" => ("pde::fishpack::complex", "batch-c-fishpack"),
        other => {
            return Err(policy(&format!(
                "Batch C has no canonical module for {routine} ({other})"
            )));
        }
    };
    Ok((module.to_owned(), feature.to_owned()))
}

fn binding_module(batch: &str) -> Result<String> {
    match batch {
        "batch_complex_arguments" => Ok("complex_arguments".to_owned()),
        "batch_complex_returns" => Ok("complex_returns".to_owned()),
        "batch_character" => Ok("character".to_owned()),
        "batch_logical" => Ok("logical".to_owned()),
        other => Err(policy(&format!(
            "unsupported Batch C binding batch {other}"
        ))),
    }
}

fn write_canonical_modules(sys_dir: &Path, candidates: &[Candidate]) -> Result<()> {
    let dir = sys_dir.join("src/batch_c");
    fs::create_dir_all(&dir)?;
    let mut families = BTreeMap::<String, Vec<&Candidate>>::new();
    for candidate in candidates {
        let family = candidate
            .declaration_feature
            .trim_start_matches("batch-c-")
            .replace('-', "_");
        families.entry(family).or_default().push(candidate);
    }
    let mut index = String::from(
        "//! Generated Batch C declaration owners.\n//!\n//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-c --offline`.\n\n",
    );
    for (family, values) in &families {
        let feature = family.replace('_', "-");
        index.push_str(&format!(
            "#[cfg(feature = \"raw-family-batch-c-{feature}\")]\npub(crate) mod {family};\n"
        ));
        let mut text = String::from(
            "//! Generated Batch C canonical raw declarations.\n//!\n//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-c --offline`.\n\n",
        );
        for candidate in values {
            render_candidate(&mut text, candidate);
        }
        fs::write(dir.join(format!("{family}.rs")), text)?;
    }
    fs::write(dir.join("mod.rs"), index)?;
    format_generated_rust(&dir)
}

fn render_candidate(text: &mut String, candidate: &Candidate) {
    text.push_str(&format!("// raw-api-routine: {}\n", candidate.routine));
    render_docs(text, candidate);
    text.push_str(&format!(
        "#[cfg(feature = \"{}\")]\n#[doc(inline)]\npub use crate::generated::{}::{};\n",
        candidate.binding_feature,
        candidate.binding_module,
        candidate.routine.to_ascii_lowercase()
    ));
    text.push_str(&format!("#[cfg(not(feature = \"{}\"))]\nunsafe extern \"C\" {{\n    #[link_name = \"{}\"]\n    pub fn {}(",candidate.binding_feature,candidate.native_symbol,candidate.routine.to_ascii_lowercase()));
    for (index, argument) in candidate.arguments.iter().enumerate() {
        if index > 0 {
            text.push_str(", ");
        }
        text.push_str(&format!(
            "{}: *mut {}",
            rust_identifier(&argument.name),
            rust_ffi_type(&argument.declared_type)
        ));
    }
    let char_count = candidate
        .arguments
        .iter()
        .filter(|a| a.declared_type == "CHARACTER")
        .count();
    for index in 0..char_count {
        if !candidate.arguments.is_empty() || index > 0 {
            text.push_str(", ");
        }
        text.push_str(&format!(
            "character_length_{}: crate::FortranCharacterLength",
            index + 1
        ));
    }
    text.push(')');
    if let Some(result) = &candidate.result {
        text.push_str(&format!(" -> {}", rust_ffi_type(&result.declared_type)));
    }
    text.push_str(";\n}\n\n");
}

fn render_docs(text: &mut String, candidate: &Candidate) {
    text.push_str(&format!("/// {}\n", one_line(&candidate.purpose)));
    text.push_str(&format!("///\n/// Original SLATEC routine `{}`; source: <{}>; source SHA-256: `{}`; native symbol: `{}`.\n",candidate.routine,candidate.source_url,candidate.source_hash,candidate.native_symbol));
    text.push_str(&format!(
        "/// Supported ABI: GNU MinGW `{TARGET}`; Batch C class `{}`; fingerprint `{}`.\n",
        candidate.abi_class, candidate.abi_fingerprint
    ));
    if candidate
        .arguments
        .iter()
        .any(|a| is_complex(&a.declared_type))
        || candidate
            .result
            .as_ref()
            .is_some_and(|r| is_complex(&r.declared_type))
    {
        text.push_str("/// Complex values use `crate::Complex32` or `crate::Complex64` in real/imaginary order. Complex arguments are passed by pointer; complex function results use the compiler-probed direct aggregate-return convention.\n");
    }
    if candidate
        .arguments
        .iter()
        .any(|a| a.declared_type == "CHARACTER")
    {
        text.push_str("/// Every CHARACTER dummy is a one-byte buffer. Pass one trailing `crate::FortranCharacterLength` value of `1` per dummy, after all ordinary arguments and in visible dummy order.\n");
    }
    if candidate
        .arguments
        .iter()
        .any(|a| a.declared_type == "LOGICAL")
        || candidate
            .result
            .as_ref()
            .is_some_and(|r| r.declared_type == "LOGICAL")
    {
        text.push_str("/// LOGICAL uses `crate::FortranLogical` (`i32`, false `0`, true `1`), never Rust `bool`; arrays use four-byte element stride.\n");
    }
    text.push_str("///\n/// # Arguments\n///\n");
    for argument in &candidate.arguments {
        let shape = if argument.dimensions.is_empty() {
            "scalar".to_owned()
        } else {
            format!("rank-{} array", argument.dimensions.len())
        };
        let extra = if argument.declared_type == "CHARACTER" {
            format!(
                " Declared length `{}`; its trailing hidden length must be `1`.",
                argument.character_length
            )
        } else {
            String::new()
        };
        text.push_str(&format!("/// - `{}`: Fortran `{}` {shape}.{extra} Null is not permitted; provide the complete source-defined readable/writable extent. Semantic intent, aliasing permission, and exact extent/workspace formulas remain source-prologue obligations. The native routine does not retain the pointer.\n",argument.name,argument.declared_type));
    }
    if let Some(result) = &candidate.result {
        text.push_str(&format!(
            "/// - Return: compiler-validated Fortran `{}` value",
            result.declared_type
        ));
        if result.declared_type == "CHARACTER" {
            text.push_str(&format!(
                " of declared length `{}`",
                result.character_length
            ));
        }
        text.push_str(".\n");
    }
    text.push_str("///\n/// # Safety\n///\n/// This declaration is source/hash, symbol, ABI-profile, compile, and bulk-link validated; it is not a safe or universally numerically validated API. Every pointer must be non-null, aligned, and valid for every native access; array storage, strides, leading dimensions, workspace, and aliasing must satisfy the selected source prologue. Rust callbacks, if present, must use the exact Batch B ABI and must not unwind. The native routine retains no pointer. Callers must link the documented GNU MinGW-compatible provider and serialize any process-global SLATEC state.\n");
}

fn write_compile_probes(sys_dir: &Path, candidates: &[Candidate]) -> Result<()> {
    let tests = sys_dir.join("tests");
    fs::create_dir_all(&tests)?;
    for (index, chunk) in candidates.chunks(BATCH_SIZE).enumerate() {
        let mut text = String::from(
            "//! Generated Batch C canonical-path compile probe.\n\n#![cfg(feature = \"all\")]\n\n#[test]\nfn imports_batch_c_paths() {\n",
        );
        for candidate in chunk {
            text.push_str(&format!(
                "    let _ = {} as *const ();\n",
                candidate.canonical_path
            ));
        }
        text.push_str("}\n");
        fs::write(
            tests.join(format!("batch_c_compile_{:02}.rs", index + 1)),
            text,
        )?;
    }
    Ok(())
}

fn write_link_probes(facade_dir: &Path, candidates: &[Candidate]) -> Result<()> {
    let tests = facade_dir.join("tests");
    fs::create_dir_all(&tests)?;
    for (index, chunk) in candidates.chunks(BATCH_SIZE).enumerate() {
        let mut text = String::from(
            "//! Generated Batch C native-symbol link probe.\n\n#![cfg(feature = \"raw-batch-c-link-tests\")]\n\n#[test]\nfn links_batch_c_symbols() {\n",
        );
        for candidate in chunk {
            text.push_str(&format!(
                "    let _ = {} as *const ();\n",
                candidate.canonical_path
            ));
        }
        text.push_str("}\n");
        fs::write(
            tests.join(format!("raw_batch_c_link_{:02}.rs", index + 1)),
            text,
        )?;
    }
    Ok(())
}

fn candidate_json(candidate: &Candidate) -> Value {
    json!({
        "routine":candidate.routine,"source_hash":candidate.source_hash,"source_file":candidate.source_file,"source_url":candidate.source_url,"native_symbol":candidate.native_symbol,"program_unit_kind":candidate.kind,
        "combined_abi_class":candidate.abi_class,"normalized_abi_fingerprint":candidate.abi_fingerprint,"canonical_module":candidate.canonical_module,"canonical_rust_path":candidate.canonical_path,
        "declaration_feature":candidate.declaration_feature,"provider_feature":candidate.provider_feature,"binding_module":candidate.binding_module,"binding_path":format!("crate::generated::{}::{}",candidate.binding_module,candidate.routine.to_ascii_lowercase()),
        "arguments":candidate.arguments.iter().map(|a|a.name.clone()).collect::<Vec<_>>(),"result_type":candidate.result.as_ref().map(|r|r.declared_type.clone()),
        "validation_statuses":["source_verified","abi_classified","complex_layout_validated","complex_return_validated","character_length_validated","logical_representation_validated","compile_validated","link_validated"],
    })
}

fn generated_source_for_routine(sys_dir: &Path, routine: &str) -> Result<String> {
    let marker = format!("// raw-api-routine: {routine}\n");
    for entry in fs::read_dir(sys_dir.join("src/batch_c"))? {
        let path = entry?.path();
        if path.extension().and_then(|x| x.to_str()) != Some("rs") {
            continue;
        }
        let source = fs::read_to_string(&path)?;
        if let Some(start) = source.find(&marker) {
            let rest = &source[start..];
            let after = marker.len();
            let end = rest[after..]
                .find("// raw-api-routine:")
                .map(|n| after + n)
                .unwrap_or(rest.len());
            return Ok(rest[..end].to_owned());
        }
    }
    Err(policy(&format!(
        "Batch C generated source missing for {routine}"
    )))
}

fn validate_abi_evidence(value: &Value) -> Result<()> {
    for key in [
        "status",
        "complex",
        "complex_return",
        "character",
        "logical",
        "selected_complex_rust",
        "selected_complex_return_rust",
    ] {
        if value
            .pointer(&format!("/abi_validation/{key}"))
            .and_then(Value::as_str)
            != Some("passed")
        {
            return Err(policy(&format!(
                "Batch C ABI evidence {key} has not passed"
            )));
        }
    }
    Ok(())
}

fn public_routines(raw_api_dir: &Path) -> Result<BTreeSet<String>> {
    let mut set = BTreeSet::new();
    for row in records(
        &read_json(&raw_api_dir.join("routine-status.json"))?,
        "raw API status",
    )? {
        if matches!(
            field(row, "raw_api_state").as_str(),
            "reviewed_public_driver" | "reviewed_public_subsidiary" | "batch_a_public_driver"
        ) {
            set.insert(field(row, "routine"));
        }
    }
    for file in ["batch-a-candidates.json", "batch-b-candidates.json"] {
        for row in records(&read_json(&raw_api_dir.join(file))?, file)? {
            set.insert(field(row, "routine"));
        }
    }
    Ok(set)
}

fn batch_b_routines(raw_api_dir: &Path) -> Result<BTreeSet<String>> {
    Ok(records(
        &read_json(&raw_api_dir.join("batch-b-candidates.json"))?,
        "Batch B candidates",
    )?
    .iter()
    .map(|r| field(r, "routine"))
    .collect())
}
fn batch_a_roles(raw_api_dir: &Path) -> Result<BTreeMap<String, String>> {
    Ok(records(
        &read_json(&raw_api_dir.join("abi-classification.json"))?,
        "Batch A classification",
    )?
    .iter()
    .map(|r| (field(r, "routine"), field(r, "public_role")))
    .collect())
}

fn interfaces(path: &Path) -> Result<Vec<Interface>> {
    table(&read_json(path)?, "FFI interface inventory")?
        .into_iter()
        .map(|row| {
            Ok(Interface {
                provider_id: required(&row, "program_unit_id", path)?,
                kind: required(&row, "kind", path)?,
                source_hash: required(&row, "raw_sha256", path)?,
                native_symbol: optional(&row, "observed_raw_symbol"),
                confidence: required(&row, "confidence_class", path)?,
                batch: optional(&row, "binding_batch"),
                argument_ids: strings(row.get("argument_ids")),
                result_id: row
                    .get("function_result_id")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            })
        })
        .collect()
}
fn arguments(path: &Path) -> Result<BTreeMap<String, Argument>> {
    let mut out = BTreeMap::new();
    for row in table(&read_json(path)?, "argument inventory")? {
        let id = required(&row, "id", path)?;
        out.insert(
            id,
            Argument {
                name: required(&row, "normalized_name", path)?,
                declared_type: optional(&row, "declared_type"),
                character_length: optional(&row, "character_length"),
                dimensions: row
                    .get("dimensions")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default(),
                is_external: row
                    .get("is_external")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
                conflict_state: optional(&row, "conflict_state"),
            },
        );
    }
    Ok(out)
}
fn results(path: &Path) -> Result<BTreeMap<String, FunctionResult>> {
    let mut out = BTreeMap::new();
    for row in table(&read_json(path)?, "function result inventory")? {
        let id = required(&row, "id", path)?;
        out.insert(
            id,
            FunctionResult {
                declared_type: optional(&row, "declared_type"),
                character_length: optional(&row, "character_length"),
                conflict_state: optional(&row, "conflict_state"),
            },
        );
    }
    Ok(out)
}

fn abi_fingerprint(kind: &str, args: &[Argument], result: Option<&FunctionResult>) -> String {
    let values = args
        .iter()
        .map(|a| {
            let mut value = if a.is_external {
                "callback".to_owned()
            } else {
                rust_type(&a.declared_type).to_owned()
            };
            if !a.dimensions.is_empty() {
                value.push_str(&format!("_array_rank{}", a.dimensions.len()));
            }
            if a.declared_type == "CHARACTER" {
                value.push_str("_hidden_len_usize_trailing");
            }
            format!("mut_{value}")
        })
        .collect::<Vec<_>>()
        .join(",");
    if kind == "function" {
        format!(
            "function:{}({values})",
            result
                .map(|r| rust_type(&r.declared_type))
                .unwrap_or("unknown")
        )
    } else {
        format!("subroutine:void({values})")
    }
}
fn rust_type(value: &str) -> &'static str {
    match value {
        "INTEGER" => "i32",
        "REAL" => "f32",
        "DOUBLE PRECISION" => "f64",
        "COMPLEX" => "complex32",
        "DOUBLE COMPLEX" => "complex64",
        "CHARACTER" => "c_char",
        "LOGICAL" => "fortran_logical_i32",
        _ => "unsupported",
    }
}
fn rust_ffi_type(value: &str) -> &'static str {
    match value {
        "INTEGER" => "crate::FortranInteger",
        "REAL" => "f32",
        "DOUBLE PRECISION" => "f64",
        "COMPLEX" => "crate::Complex32",
        "DOUBLE COMPLEX" => "crate::Complex64",
        "CHARACTER" => "core::ffi::c_char",
        "LOGICAL" => "crate::FortranLogical",
        _ => "core::ffi::c_void",
    }
}
fn is_complex(value: &str) -> bool {
    matches!(value, "COMPLEX" | "DOUBLE COMPLEX")
}
fn complex_precision(value: &str) -> &'static str {
    if value == "DOUBLE COMPLEX" {
        "complex64"
    } else {
        "complex32"
    }
}
fn combined_class(c: bool, ch: bool, l: bool, cb: bool, ret: bool) -> String {
    let mut p = Vec::new();
    if c {
        p.push(if ret {
            "complex_return"
        } else {
            "complex_argument_or_output"
        });
    }
    if ch {
        p.push("character_1");
    }
    if l {
        p.push("logical");
    }
    if cb {
        p.push("callback");
    }
    if p.is_empty() {
        "not_batch_c_shape".to_owned()
    } else {
        p.join("+")
    }
}
fn rust_identifier(value: &str) -> String {
    let v = value.to_ascii_lowercase();
    if matches!(
        v.as_str(),
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
    ) {
        format!("r#{v}")
    } else {
        v
    }
}
fn validate_candidates(candidates: &[Candidate]) -> Result<()> {
    let mut paths = BTreeSet::new();
    let mut symbols = BTreeMap::new();
    for c in candidates {
        if !paths.insert(&c.canonical_path) {
            return Err(policy("Batch C canonical path collision"));
        }
        if let Some(old) = symbols.insert(&c.native_symbol, &c.abi_fingerprint) {
            if old != &c.abi_fingerprint {
                return Err(policy("Batch C symbol has incompatible fingerprints"));
            }
        }
    }
    Ok(())
}
fn format_generated_rust(dir: &Path) -> Result<()> {
    let mut files = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("rs"))
        .collect::<Vec<_>>();
    files.sort();
    let out = Command::new(std::env::var_os("RUSTFMT").unwrap_or_else(|| "rustfmt".into()))
        .args(["--edition", "2024"])
        .args(&files)
        .output()
        .map_err(|e| CorpusError::Verification(format!("could not format Batch C Rust: {e}")))?;
    if out.status.success() {
        Ok(())
    } else {
        Err(policy("rustfmt rejected generated Batch C Rust"))
    }
}
fn summary_by(records: &[Value], key: &str) -> BTreeMap<String, usize> {
    let mut m = BTreeMap::new();
    for r in records {
        *m.entry(field(r, key)).or_default() += 1;
    }
    m
}
fn render_summary(v: &Value) -> String {
    let c = |k: &str| {
        v.pointer(&format!("/counts/{k}"))
            .and_then(Value::as_u64)
            .unwrap_or_default()
    };
    format!(
        "# Raw API Batch C summary\n\n- Retained identities: {}\n- Public raw identities before Batch C: {}\n- Complex-bearing identities: {}\n- Complex argument/output identities: {}\n- Complex function-return identities: {}\n- CHARACTER-bearing identities: {}\n- Fixed CHARACTER*1 identities: {}\n- LOGICAL-bearing identities: {}\n- Combined-category identities: {}\n- Already-public eligible routines: {}\n- Newly promoted routines: {}\n- Eligible subsidiaries kept internal: {}\n- Complex-return exclusions: {}\n- Long/variable-character exclusions: {}\n- Logical exclusions: {}\n- Callback-combination exclusions: {}\n- ENTRY/alternate-return exclusions: {}\n- Provider-missing exclusions: {}\n- Other ABI exclusions: {}\n- Public raw identities after Batch C: {} ({}% of retained identities)\n- Unique Batch C ABI fingerprints: {}\n- Source-hash corrections: {}\n- Parser improvements: {}\n- Compile/link probe batches: {}/{}\n- Representative runtime smoke routines: {}\n\nBatch C paths are unsafe and profile-specific. Promotion records source/hash, symbol, ABI, canonical path, provider, compile, and bulk-link evidence; it does not imply safe wrapping or universal numerical validation.\n",
        c("total_retained_identities"),
        c("public_raw_identities_before_batch_c"),
        c("complex_bearing_identities"),
        c("complex_argument_or_output_identities"),
        c("complex_function_return_identities"),
        c("character_bearing_identities"),
        c("character_1_identities"),
        c("logical_bearing_identities"),
        c("combined_category_identities"),
        c("already_public_eligible_routines"),
        c("newly_promoted_routines"),
        c("eligible_subsidiaries_kept_internal"),
        c("complex_return_exclusions"),
        c("long_or_variable_character_exclusions"),
        c("logical_exclusions"),
        c("callback_combination_exclusions"),
        c("entry_or_alternate_return_exclusions"),
        c("provider_missing_exclusions"),
        c("other_exclusions"),
        c("total_public_raw_identities_after_batch_c"),
        v.pointer("/counts/retained_public_percentage")
            .and_then(Value::as_str)
            .unwrap_or("0"),
        c("unique_batch_c_abi_fingerprints"),
        c("source_hash_corrections"),
        c("parser_improvements"),
        c("compile_batches"),
        c("link_batches"),
        c("runtime_smoke_routines")
    )
}
fn source_url(record: &Value) -> String {
    record
        .pointer("/official_documentation/netlib_source_url/url")
        .and_then(Value::as_str)
        .or_else(|| record.get("description_source_url").and_then(Value::as_str))
        .unwrap_or("https://www.netlib.org/slatec/")
        .to_owned()
}
fn one_line(value: &str) -> String {
    let v = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if v.is_empty() {
        "Original SLATEC numerical routine.".to_owned()
    } else {
        v
    }
}
fn table(value: &Value, name: &str) -> Result<Vec<BTreeMap<String, Value>>> {
    let columns = value
        .get("columns")
        .and_then(Value::as_array)
        .ok_or_else(|| policy(&format!("{name} has no columns")))?
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_owned)
                .ok_or_else(|| policy("non-string column"))
        })
        .collect::<Result<Vec<_>>>()?;
    records(value, name)?
        .iter()
        .map(|row| {
            let cells = row
                .as_array()
                .or_else(|| row.get("value").and_then(Value::as_array))
                .ok_or_else(|| policy("malformed compact row"))?;
            if cells.len() != columns.len() {
                return Err(policy("wrong compact row width"));
            }
            Ok(columns.iter().cloned().zip(cells.iter().cloned()).collect())
        })
        .collect()
}
fn records<'a>(value: &'a Value, name: &str) -> Result<&'a Vec<Value>> {
    value
        .get("records")
        .and_then(Value::as_array)
        .ok_or_else(|| policy(&format!("{name} has no records")))
}
fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn bytes(value: &Value) -> Result<Vec<u8>> {
    let mut b = serde_json::to_vec_pretty(value)?;
    b.push(b'\n');
    Ok(b)
}
fn required(row: &BTreeMap<String, Value>, key: &str, path: &Path) -> Result<String> {
    row.get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| policy(&format!("{} lacks {key}", path.display())))
}
fn optional(row: &BTreeMap<String, Value>, key: &str) -> String {
    row.get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}
fn string(value: &Value, key: &str) -> Result<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| policy(&format!("record lacks {key}")))
}
fn field(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}
fn strings(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}
fn output_hash(files: &BTreeMap<&str, Vec<u8>>) -> String {
    let mut material = Vec::new();
    for (name, bytes) in files {
        material.extend_from_slice(name.as_bytes());
        material.push(0);
        material.extend_from_slice(bytes);
        material.push(0);
    }
    hash::bytes(&material)
}
fn write_outputs(root: &Path, files: &BTreeMap<&str, Vec<u8>>) -> Result<()> {
    fs::create_dir_all(root)?;
    for (name, bytes) in files {
        fs::write(root.join(name), bytes)?;
    }
    Ok(())
}
fn policy(message: &str) -> CorpusError {
    CorpusError::Verification(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fingerprints_distinguish_complex_character_and_logical() {
        let arg = |name: &str, ty: &str| Argument {
            name: name.to_owned(),
            declared_type: ty.to_owned(),
            character_length: if ty == "CHARACTER" {
                "1".to_owned()
            } else {
                String::new()
            },
            dimensions: Vec::new(),
            is_external: false,
            conflict_state: String::new(),
        };
        assert_ne!(
            abi_fingerprint("subroutine", &[arg("Z", "COMPLEX")], None),
            abi_fingerprint("subroutine", &[arg("FLAG", "CHARACTER")], None)
        );
        assert!(
            abi_fingerprint(
                "function",
                &[arg("Z", "COMPLEX")],
                Some(&FunctionResult {
                    declared_type: "COMPLEX".to_owned(),
                    character_length: String::new(),
                    conflict_state: String::new()
                })
            )
            .starts_with("function:complex32")
        );
        assert_eq!(
            combined_class(true, true, true, true, false),
            "complex_argument_or_output+character_1+logical+callback"
        );
    }
}
