//! Deterministic metadata for the reviewed standard complex FFTPACK facade.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use crate::error::{CorpusError, Result};
use crate::hash;

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

/// Returns focused native-state projections for the complex FFT plan methods.
pub(crate) fn native_state_projections() -> Result<Vec<Value>> {
    let closure = closure()?;
    let objects = closure["source_ids"]
        .as_array()
        .ok_or_else(|| {
            CorpusError::Verification("complex FFT closure lacks source ids".to_owned())
        })?
        .iter()
        .filter_map(Value::as_str)
        .map(|id| format!("{id}.o"))
        .collect::<Vec<_>>();
    Ok(["new", "forward", "backward"]
        .into_iter()
        .map(|method| {
            json!({
                "safe_function":format!("slatec::transforms::fft::complex::ComplexFftPlan32::{method}"),
                "native_entry_points": if method == "new" { vec!["CFFTI1"] } else if method == "forward" { vec!["CFFTF1"] } else { vec!["CFFTB1"] },
                "feature":"fftpack-complex",
                "effective_native_families":["fftpack-complex"],
                "entry_object":if method == "new" { "selected-source-43ec770eeca689dc.o" } else if method == "forward" { "selected-source-c5c747d24cd866bb.o" } else { "selected-source-3325208a9dffff3a.o" },
                "object_closure":objects,
                "source_closure":["CFFTI1/CFFTF1/CFFTB1","PASSF/PASSB radix subsidiaries"],
                "saved_mutable_locals":["CFFTI1:NTRYH (SAVE+DATA, source-read-only factor trial table)"],
                "common_blocks":[],
                "xerror_state":[],
                "fortran_io":[],
                "callback_state":["none"],
                "writable_symbols":["CFFTI1 local NTRYH emitted in .data; source has no writer"],
                "source_object_unresolved":[],
                "external_undefined_symbols":["sinf/cosf from GNU MinGW C math runtime"],
                "feature_closure_mismatch":false,
                "current_class":"SerializedGlobal",
                "best_possible_class_from_slatec_source":"SerializedGlobal",
                "native_routine_reentrancy":"SerializedGlobal",
                "rust_api_concurrency":"owned plans are movable; &mut self prevents same-plan concurrent transforms",
                "provider_runtime_thread_safety":"reviewed source and external/system profiles remain serialized",
                "provider_unknowns":["external_or_system_Fortran_runtime_and_provider_contract_not_qualified"],
                "remaining_blockers":["CFFTI1 SAVE+DATA storage","provider/runtime qualification","existing hosted runtime policy"]
            })
        })
        .collect())
}

/// Concise result from complex FFTPACK metadata generation.
#[derive(Debug)]
pub struct ResultSummary {
    /// The selected-corpus snapshot identifier.
    pub snapshot_id: String,
    /// Stable hash over every emitted file.
    pub semantic_hash: String,
    /// Count of reviewed native standard entry points.
    pub routine_count: usize,
}

/// Generates explicit complex-FFTPACK audit and wrapper metadata.
pub fn generate(
    selected_corpus_dir: &Path,
    output_dir: &Path,
    offline: bool,
) -> Result<ResultSummary> {
    if !offline {
        return Err(CorpusError::Policy(
            "generate-safe-fftpack-complex-metadata requires --offline".to_owned(),
        ));
    }
    let manifest: Value =
        serde_json::from_slice(&fs::read(selected_corpus_dir.join("manifest.json"))?)?;
    let snapshot = manifest["snapshot_id"]
        .as_str()
        .ok_or_else(|| CorpusError::Verification("selected corpus lacks snapshot_id".to_owned()))?;
    let closure = closure()?;
    let routines = json!([
        [
            "CFFTI1",
            "cffti1_",
            "f32",
            "standard_initializer",
            "N,WA,IFAC",
            "WA=2*N real; IFAC=15 integer",
            "user_callable",
            "included",
            "N >= 2"
        ],
        [
            "CFFTF1",
            "cfftf1_",
            "f32",
            "forward",
            "N,C,CH,WA,IFAC",
            "C=2*N interleaved real; CH=2*N real; WA=2*N real; IFAC=15 integer",
            "user_callable",
            "included",
            "N >= 2"
        ],
        [
            "CFFTB1",
            "cfftb1_",
            "f32",
            "backward",
            "N,C,CH,WA,IFAC",
            "C=2*N interleaved real; CH=2*N real; WA=2*N real; IFAC=15 integer",
            "user_callable",
            "included",
            "N >= 2"
        ],
        [
            "CFFTI/CFFTF/CFFTB",
            "f77_nonstandard_wrappers",
            "f32",
            "deprecated_complex_dummy_wrapper",
            "N,COMPLEX C,WSAVE",
            "WSAVE=4*N+15 real",
            "subsidiary",
            "deferred_in_favor_of_standard_star1_interface",
            "legacy wrapper handles N=1 identity path"
        ],
        [
            "double_precision_complex_FFTPACK",
            "none",
            "f64",
            "precision_pair",
            "not present in selected snapshot",
            "not applicable",
            "absent",
            "deferred",
            "not present"
        ]
    ]);
    let wrappers = json!([
        [
            "slatec::transforms::fft::complex::ComplexFftPlan32::new",
            "CFFTI1",
            "Complex32",
            "initialize reusable plan",
            "WA=2*N; IFAC=15",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::transforms::fft::complex::ComplexFftPlan32::forward",
            "CFFTF1",
            "Complex32",
            "negative exponent in-place DFT",
            "CH=2*N; WA=2*N; IFAC=15",
            "SerializedGlobal",
            "reviewed"
        ],
        [
            "slatec::transforms::fft::complex::ComplexFftPlan32::backward",
            "CFFTB1",
            "Complex32",
            "positive exponent in-place DFT",
            "CH=2*N; WA=2*N; IFAC=15",
            "SerializedGlobal",
            "reviewed"
        ]
    ]);
    let files = [
        (
            "fftpack-complex-routine-inventory.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.routine-inventory","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["routine","symbol","precision","role","arguments","workspace","catalogue_role","safe_status","valid_length"],"records":routines,"source_records":closure["sources"],"audited_deferred_sources":[["CFFTI","https://www.netlib.org/slatec/fishfft/cffti.f","96ace3e65d97da04c1ee492190a6acccfa368df16588539ebc925f6be5695180","legacy wrapper returns immediately for N=1"],["CFFTF","https://www.netlib.org/slatec/fishfft/cfftf.f","af08ea8288cbd02b2d55314d4ff114fbe2c371673f90362cc1308dd342e0f65","legacy wrapper returns immediately for N=1"],["CFFTB","https://www.netlib.org/slatec/fishfft/cfftb.f","a2c929aa91c6cb449551790f5330a2ddbd7905306a3fce3938870929aec35e53","legacy wrapper returns immediately for N=1"]]}),
        ),
        (
            "fftpack-complex-candidate-classification.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.candidate-classification","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["candidate","status","reason"],"records":[["CFFTI1/CFFTF1/CFFTB1","included","standard real-array interface is fully audited and avoids passing a Fortran COMPLEX dummy; N must be at least 2 because the standard initializer lacks the legacy identity return"],["CFFTI/CFFTF/CFFTB","deferred","SLATEC marks the wrappers subsidiary because their calls use non-standard argument association, although they alone contain the N=1 identity return"],["Complex64 plan","deferred","no selected-snapshot double-precision complex roots"],["multidimensional complex FFTPACK","deferred","outside the one-dimensional plan milestone"]]}),
        ),
        (
            "fftpack-complex-abi-contract.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.abi-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["boundary","contract","evidence","policy"],"records":[["public_complex","num-complex 0.4.6 Complex32 is repr(C), fields re then im, and layout-compatible with [f32;2]","num-complex crate documentation and layout tests","public slices use Complex32"],["native_C_argument","CFFTF1/CFFTB1 declare DIMENSION C(*) without COMPLEX","reviewed cfftf1.f/cfftb1.f declarations","raw FFI takes *mut f32 interleaved words"],["Fortran_complex_ABI","not crossed by selected standard interface","standard *1 source declarations","no Complex32 pointer is declared in raw FFI"],["profile","GNU MinGW x86_64 Fortran uses pointer arguments for these F77 arrays","native source-build probe","layout evidence is profile-scoped"]]}),
        ),
        (
            "fftpack-complex-layout-contract.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.layout-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["property","required_value","validation"],"records":[["Complex32 size","2 * size_of(f32)","const assertion and test"],["Complex32 alignment","align_of(f32)","const assertion and test"],["component order","real then imaginary","num-complex repr(C) fields and word-view test"],["slice stride","two contiguous f32 words per element","checked 2*N conversion before raw pointer view"],["raw boundary","*mut f32","CFFTF1/CFFTB1 real DIMENSION C(*) contract"]]}),
        ),
        (
            "fftpack-complex-normalization-contract.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.normalization-contract","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["operation","equation","normalization"],"records":[["CFFTF1","Xhat[j]=sum_k X[k]*exp(-i*2*pi*j*k/N)","none"],["CFFTB1","X[j]=sum_k Xhat[k]*exp(+i*2*pi*j*k/N)","none"],["composition","backward(forward(X))=N*X","native scale preserved"]]}),
        ),
        (
            "fftpack-complex-source-closure.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.source-closure","schema_version":"1.0.0","snapshot_id":snapshot,"manifest":"crates/slatec-src/metadata/fftpack-complex-source-closure.json","roots":["CFFTI1","CFFTF1","CFFTB1"],"source_ids":closure["source_ids"],"sources":closure["sources"],"direct_subsidiaries":["PASSF","PASSF2","PASSF3","PASSF4","PASSF5","PASSB","PASSB2","PASSB3","PASSB4","PASSB5"],"excluded":["deprecated CFFTI/CFFTF/CFFTB wrappers","real FFTPACK","multidimensional FFTPACK","FISHPACK","XERROR","Fortran I/O","callbacks"]}),
        ),
        (
            "fftpack-complex-native-state.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.native-state","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["closure_part","state","origin","effect","policy"],"records":[["CFFTI1","NTRYH(4)","SAVE+DATA","source-read-only factor trial table; emitted writable object storage","SerializedGlobal"],["CFFTF1/CFFTB1 and PASS*","none","focused source audit","caller buffers and plan workspace carry numerical state","SerializedGlobal under existing policy"],["XERROR, I/O, callbacks, COMMON","not reachable","complete selected closure audit","none","no relaxation claimed"]]}),
        ),
        (
            "fftpack-complex-concurrency.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.concurrency","schema_version":"1.0.0","snapshot_id":snapshot,"columns":["safe_function","backend_profile","class","lock_scope","reason"],"records":[["ComplexFftPlan32::new/forward/backward","slatec-src-gnu-mingw","SerializedGlobal","process_global_runtime_lock","CFFTI1 SAVE+DATA factor table and existing conservative hosted policy"],["ComplexFftPlan32::new/forward/backward","external_or_system_backend","BackendDependent","process_global_runtime_lock","provider and Fortran runtime thread-safety are not qualified"]]}),
        ),
        (
            "fftpack-complex-wrapper-index.json",
            json!({"schema_id":"slatec.safe-fftpack-complex.wrapper-index","schema_version":"1.0.0","snapshot_id":snapshot,"raw_ffi_profile":PROFILE,"columns":["safe_path","raw_routine","public_scalar","operation","workspace_formula","runtime_policy","review_state"],"records":wrappers,"counts":{"native_user_callable_routines":3,"public_plan_types":1,"canonical_native_operations":3,"public_callable_operations":3,"precision_pairs":0,"subsidiaries_linkage_only":10}}),
        ),
    ];
    fs::create_dir_all(output_dir)?;
    let mut bytes = Vec::new();
    for (name, value) in files {
        let encoded = serde_json::to_vec(&value)?;
        fs::write(output_dir.join(name), &encoded)?;
        bytes.extend_from_slice(&encoded);
    }
    let summary = format!(
        "# Safe complex FFTPACK plans\n\n- Snapshot: `{snapshot}`.\n- The reviewed standard CFFTI1/CFFTF1/CFFTB1 interface exposes `ComplexFftPlan32` over `num_complex::Complex32`; the selected snapshot contains no double-precision complex roots.\n- The standard interface is valid for `N >= 2`. Legacy CFFTI/CFFTF/CFFTB wrappers have an `N = 1` identity early return, but are deferred because they use non-standard complex-dummy argument association.\n- `Complex32` is passed as its documented real/imaginary `f32` words to the native real-array interface, so no Fortran COMPLEX value ABI crosses FFI.\n- Forward uses the negative exponent, backward uses the positive exponent, and compositions scale by `N`; no automatic normalization occurs.\n- Plans own exact `CH=2*N`, `WA=2*N`, and `IFAC=15` workspace. CFFTI1's source-read-only SAVE+DATA NTRYH table and the existing hosted policy keep calls `SerializedGlobal`.\n- Deprecated non-standard CFFTI/CFFTF/CFFTB wrappers, Complex64, multidimensional transforms, adapters, and translated algorithms remain deferred.\n"
    );
    fs::write(
        output_dir.join("fftpack-complex-validation-summary.md"),
        summary.as_bytes(),
    )?;
    bytes.extend_from_slice(summary.as_bytes());
    Ok(ResultSummary {
        snapshot_id: snapshot.to_owned(),
        semantic_hash: hash::bytes(&bytes),
        routine_count: 3,
    })
}

fn closure() -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../slatec-src/metadata/fftpack-complex-source-closure.json"),
    )?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_deterministic_and_covers_standard_roots() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let selected = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("generated/selected-corpus");
        let one = generate(&selected, first.path(), true).unwrap();
        let two = generate(&selected, second.path(), true).unwrap();
        assert_eq!(one.semantic_hash, two.semantic_hash);
        assert_eq!(one.routine_count, 3);
        for name in [
            "fftpack-complex-routine-inventory.json",
            "fftpack-complex-candidate-classification.json",
            "fftpack-complex-abi-contract.json",
            "fftpack-complex-layout-contract.json",
            "fftpack-complex-normalization-contract.json",
            "fftpack-complex-source-closure.json",
            "fftpack-complex-native-state.json",
            "fftpack-complex-concurrency.json",
            "fftpack-complex-wrapper-index.json",
            "fftpack-complex-validation-summary.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(name)).unwrap(),
                fs::read(second.path().join(name)).unwrap()
            );
        }
    }
}
