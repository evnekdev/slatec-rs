use slatec_tools::acquire;
use slatec_tools::archive::{inspect_archive, verify_artifact};
use slatec_tools::blas1_concurrency;
use slatec_tools::complete_corpus;
use slatec_tools::error::{CorpusError, Result};
use slatec_tools::extract;
use slatec_tools::ffi_inventory;
use slatec_tools::ffi_validation;
use slatec_tools::full_corpus;
use slatec_tools::linkage;
use slatec_tools::manifest;
use slatec_tools::native_origin_audit;
use slatec_tools::native_probe;
use slatec_tools::ode_audit;
use slatec_tools::optimization_audit;
use slatec_tools::policy::Policy;
use slatec_tools::program_units;
use slatec_tools::prologues;
use slatec_tools::provider;
use slatec_tools::public_module_roadmap;
use slatec_tools::raw_ffi;
use slatec_tools::runtime_profile;
use slatec_tools::runtime_storage_policy;
use slatec_tools::safe_api_docs;
use slatec_tools::safe_banded;
use slatec_tools::safe_bounded_constrained_linear_least_squares;
use slatec_tools::safe_bounded_linear_least_squares;
use slatec_tools::safe_bspline;
use slatec_tools::safe_constrained_linear_least_squares;
use slatec_tools::safe_dassl;
use slatec_tools::safe_fftpack;
use slatec_tools::safe_fftpack_complex;
use slatec_tools::safe_least_squares;
use slatec_tools::safe_linear_least_squares;
use slatec_tools::safe_linear_programming_deferred;
use slatec_tools::safe_lp_in_memory;
use slatec_tools::safe_nonlinear;
use slatec_tools::safe_nonlinear_expert;
use slatec_tools::safe_ode_sdrive;
use slatec_tools::safe_pchip;
use slatec_tools::safe_piecewise_polynomial;
use slatec_tools::safe_quadrature;
use slatec_tools::safe_roots;
use slatec_tools::safe_special;
use std::path::PathBuf;

#[derive(Debug)]
struct Options {
    command: String,
    artifact_path: PathBuf,
    evidence_dir: PathBuf,
    manifest_dir: PathBuf,
    program_unit_dir: PathBuf,
    full_corpus_dir: PathBuf,
    selected_corpus_dir: PathBuf,
    ffi_inventory_dir: PathBuf,
    bindings_dir: PathBuf,
    output_dir: PathBuf,
    batches: Vec<String>,
    offline: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("slatec-corpus: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut options = parse_options()?;
    if options.command == "scan-program-units"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/program-units");
    }
    if options.command == "scan-prologues"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/prologues");
    }
    if options.command == "analyze-prologues"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/prologues-analysis");
    }
    if options.command == "audit-full-corpus"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/full-corpus");
    }
    if options.command == "select-full-corpus"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/selected-corpus");
    }
    if options.command == "scan-ffi-inventory"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/ffi-inventory");
    }
    if options.command == "probe-native-ffi"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/native-feasibility");
    }
    if options.command == "generate-raw-ffi"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/ffi");
    }
    if matches!(
        options.command.as_str(),
        "build-native-ffi" | "validate-raw-ffi"
    ) && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/ffi-validation");
    }
    if options.command == "validate-runtime-profile"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/runtime-profile");
    }
    if options.command == "generate-safe-special-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-roots-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-nonlinear-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-nonlinear-expert-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-least-squares-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-linear-least-squares-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-linear-programming-deferred-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-ode-sdrive-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-dassl-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-lp-in-memory-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-fftpack-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-fftpack-complex-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-bspline-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-piecewise-polynomial-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-banded-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-optimization-audit"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-ode-audit"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-bounded-linear-least-squares-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-bounded-constrained-linear-least-squares-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-constrained-linear-least-squares-api"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-safe-api-docs"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-public-module-roadmap"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-runtime-storage-policy"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-blas1-concurrency-audit"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-native-origin-audit"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/safe-api");
    }
    if options.command == "generate-linkage-metadata"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("generated/linkage");
    }
    if options.command == "acquire-provider-sources"
        && options.output_dir == std::path::Path::new("generated/corpus")
    {
        options.output_dir = PathBuf::from("evidence/provider-sources");
    }
    let policy = Policy::load(&PathBuf::from("metadata/canonical-corpus.toml"))?;
    match options.command.as_str() {
        "acquire" => acquire::acquire(
            &options.artifact_path,
            &options.evidence_dir,
            &policy,
            options.offline,
        ),
        "verify" => {
            verify_artifact(&options.artifact_path, &policy)?;
            println!("success: verified {}", options.artifact_path.display());
            Ok(())
        }
        "inspect" => {
            let inventory = inspect_archive(&options.artifact_path, &policy)?;
            println!("{}", serde_json::to_string_pretty(&inventory)?);
            Ok(())
        }
        "extract" => {
            let inventory = inspect_archive(&options.artifact_path, &policy)?;
            let snapshot_id = manifest::snapshot_id(&policy);
            let root = extract::extract_raw(
                &options.artifact_path,
                &options.evidence_dir,
                &snapshot_id,
                &policy,
                &inventory,
            )?;
            println!("success: extracted raw evidence to {}", root.display());
            Ok(())
        }
        "manifest" => {
            let inventory = inspect_archive(&options.artifact_path, &policy)?;
            let result = manifest::generate(&options.output_dir, &policy, &inventory)?;
            println!(
                "{:?}: {} ({})",
                result.status,
                result.output_dir.display(),
                result.semantic_hash
            );
            Ok(())
        }
        "prepare" => {
            acquire::acquire(
                &options.artifact_path,
                &options.evidence_dir,
                &policy,
                options.offline,
            )?;
            let inventory = inspect_archive(&options.artifact_path, &policy)?;
            let snapshot_id = manifest::snapshot_id(&policy);
            extract::extract_raw(
                &options.artifact_path,
                &options.evidence_dir,
                &snapshot_id,
                &policy,
                &inventory,
            )?;
            let result = manifest::generate(&options.output_dir, &policy, &inventory)?;
            println!(
                "{:?}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "scan-program-units" => {
            let result = program_units::scan(
                &options.evidence_dir,
                &options.manifest_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            if result.status == "failed" {
                return Err(CorpusError::Verification(
                    "program-unit provider validation failed; see duplicate-providers.json"
                        .to_owned(),
                ));
            }
            Ok(())
        }
        "scan-prologues" => {
            let result = prologues::scan(
                &options.evidence_dir,
                &options.manifest_dir,
                &options.program_unit_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            if result.status == "failed" {
                return Err(CorpusError::Verification(
                    "prologue extraction failed; see generated/prologues/diagnostics.json"
                        .to_owned(),
                ));
            }
            Ok(())
        }
        "analyze-prologues" => {
            let result = prologues::analyze_baseline(
                &options.evidence_dir,
                &PathBuf::from("generated/prologues"),
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: baseline prologue analysis for snapshot {} ({})",
                result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "audit-full-corpus" => {
            let result = full_corpus::audit(
                &options.evidence_dir,
                &options.manifest_dir,
                &options.program_unit_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "select-full-corpus" => {
            let result = complete_corpus::select(
                &options.evidence_dir,
                &options.manifest_dir,
                &options.full_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            if result.status == "failed" {
                return Err(CorpusError::Verification(
                    "complete-corpus provider selection failed; see generated/selected-corpus/unresolved-providers.json"
                        .to_owned(),
                ));
            }
            Ok(())
        }
        "scan-ffi-inventory" => {
            let result = ffi_inventory::scan(
                &options.evidence_dir,
                &options.selected_corpus_dir,
                &options.program_unit_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "probe-native-ffi" => {
            let result = native_probe::probe(
                &options.evidence_dir,
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "generate-raw-ffi" => {
            let result = raw_ffi::generate(
                &options.evidence_dir,
                &options.selected_corpus_dir,
                &options.ffi_inventory_dir,
                &options.output_dir,
                &options.bindings_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "build-native-ffi" => {
            let result = ffi_validation::build_native(
                &options.evidence_dir,
                &options.selected_corpus_dir,
                &options.ffi_inventory_dir,
                &PathBuf::from("generated/ffi"),
                &options.bindings_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "validate-raw-ffi" => {
            let result = ffi_validation::validate(
                ffi_validation::ValidationPaths {
                    evidence_dir: &options.evidence_dir,
                    selected_corpus_dir: &options.selected_corpus_dir,
                    inventory_dir: &options.ffi_inventory_dir,
                    ffi_dir: &PathBuf::from("generated/ffi"),
                    bindings_dir: &options.bindings_dir,
                    output_dir: &options.output_dir,
                },
                &options.batches,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({})",
                result.status, result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "validate-runtime-profile" => {
            let result = runtime_profile::validate(
                &options.evidence_dir,
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({}); machine discrepancies {} -> {}; recovered probes {}",
                result.status,
                result.snapshot_id,
                result.semantic_hash,
                result.machine_discrepancies_before,
                result.machine_discrepancies_after,
                result.recovered_fnlib_probes
            );
            Ok(())
        }
        "generate-safe-special-api" => {
            let result = safe_special::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "{}: snapshot {} ({}); candidates {}; wrappers {}; deferred {}",
                result.status,
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
                result.deferred_count,
            );
            Ok(())
        }
        "generate-safe-quadrature-api" => {
            let result = safe_quadrature::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success_with_review_items: snapshot {} ({}); wrappers {}; deferred {}",
                result.snapshot_id, result.semantic_hash, result.wrappers, result.deferred
            );
            Ok(())
        }
        "generate-safe-roots-api" => {
            let result = safe_roots::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success_with_review_items: snapshot {} ({}); candidates {}; wrappers {}; deferred {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
                result.deferred_count,
            );
            Ok(())
        }
        "generate-safe-nonlinear-api" => {
            let result = safe_nonlinear::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success_with_review_items: snapshot {} ({}); candidates {}; wrappers {}; deferred {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
                result.deferred_count,
            );
            Ok(())
        }
        "generate-safe-nonlinear-expert-api" => {
            let result = safe_nonlinear_expert::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success_with_review_items: snapshot {} ({}); candidates {}; wrappers {}; deferred {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
                result.deferred_count
            );
            Ok(())
        }
        "generate-safe-least-squares-api" => {
            let result = safe_least_squares::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success_with_review_items: snapshot {} ({}); candidates {}; wrappers {}; deferred {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
                result.deferred_count
            );
            Ok(())
        }
        "generate-safe-linear-least-squares-api" => {
            let result = safe_linear_least_squares::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); candidates {}; wrappers {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
            );
            Ok(())
        }
        "generate-safe-linear-programming-deferred-metadata" => {
            let result = safe_linear_programming_deferred::generate(
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); reviewed deferred candidates {}",
                result.snapshot_id, result.semantic_hash, result.candidate_count,
            );
            Ok(())
        }
        "generate-safe-ode-sdrive-metadata" => {
            let result = safe_ode_sdrive::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); wrappers {}",
                result.snapshot_id, result.semantic_hash, result.wrapper_count,
            );
            Ok(())
        }
        "generate-safe-dassl-metadata" => {
            let result = safe_dassl::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: generated {} DASSL wrappers ({})",
                result.wrapper_count, result.semantic_hash
            );
            Ok(())
        }
        "generate-safe-lp-in-memory-metadata" => {
            let result = safe_lp_in_memory::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); wrappers {}",
                result.snapshot_id, result.semantic_hash, result.wrapper_count,
            );
            Ok(())
        }
        "generate-safe-fftpack-metadata" => {
            let result = safe_fftpack::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); reviewed routines {}",
                result.snapshot_id, result.semantic_hash, result.routine_count,
            );
            Ok(())
        }
        "generate-safe-fftpack-complex-metadata" => {
            let result = safe_fftpack_complex::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); reviewed routines {}",
                result.snapshot_id, result.semantic_hash, result.routine_count,
            );
            Ok(())
        }
        "generate-safe-pchip-metadata" => {
            let result = safe_pchip::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); reviewed routines {}",
                result.snapshot_id, result.semantic_hash, result.routine_count,
            );
            Ok(())
        }
        "generate-safe-bspline-metadata" => {
            let result = safe_bspline::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); reviewed routines {}",
                result.snapshot_id, result.semantic_hash, result.routine_count,
            );
            Ok(())
        }
        "generate-safe-piecewise-polynomial-metadata" => {
            let result = safe_piecewise_polynomial::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); reviewed routines {}",
                result.snapshot_id, result.semantic_hash, result.routine_count,
            );
            Ok(())
        }
        "generate-safe-banded-metadata" => {
            let result = safe_banded::generate(
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "generated safe banded metadata snapshot={} hash={}",
                result.snapshot_id, result.semantic_hash
            );
            Ok(())
        }
        "generate-optimization-audit" => {
            let result = optimization_audit::generate(
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); candidates {}; recommendation {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.recommendation,
            );
            Ok(())
        }
        "generate-ode-audit" => {
            let result = ode_audit::generate(
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); candidates {}",
                result.snapshot_id, result.semantic_hash, result.candidate_count,
            );
            Ok(())
        }
        "generate-safe-bounded-linear-least-squares-api" => {
            let result = safe_bounded_linear_least_squares::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); candidates {}; wrappers {}; deferred {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
                result.deferred_count,
            );
            Ok(())
        }
        "generate-safe-bounded-constrained-linear-least-squares-api" => {
            let result = safe_bounded_constrained_linear_least_squares::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); candidates {}; wrappers {}; deferred {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
                result.deferred_count,
            );
            Ok(())
        }
        "generate-safe-constrained-linear-least-squares-api" => {
            let result = safe_constrained_linear_least_squares::generate(
                &PathBuf::from("generated/runtime-profile"),
                &PathBuf::from("generated/ffi"),
                &options.selected_corpus_dir,
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: snapshot {} ({}); candidates {}; wrappers {}; deferred {}",
                result.snapshot_id,
                result.semantic_hash,
                result.candidate_count,
                result.wrapper_count,
                result.deferred_count,
            );
            Ok(())
        }
        "generate-safe-api-docs" => {
            let result = safe_api_docs::generate(&options.output_dir)?;
            println!(
                "success: indexed {} safe functions ({})",
                result.function_count, result.semantic_hash
            );
            Ok(())
        }
        "generate-public-module-roadmap" => {
            let result = public_module_roadmap::generate(&options.output_dir)?;
            println!(
                "success: documented {} safe functions ({})",
                result.safe_function_count, result.semantic_hash
            );
            Ok(())
        }
        "generate-runtime-storage-policy" => {
            let result = runtime_storage_policy::generate(&options.output_dir)?;
            println!(
                "success: classified {} safe functions ({})",
                result.function_count, result.semantic_hash
            );
            Ok(())
        }
        "generate-blas1-concurrency-audit" => {
            let result = blas1_concurrency::generate(&options.output_dir)?;
            println!(
                "success: qualified {} BLAS Level 1 candidates across {} backend records ({})",
                result.candidate_count, result.backend_record_count, result.semantic_hash
            );
            Ok(())
        }
        "generate-native-origin-audit" => {
            let result = native_origin_audit::generate(&options.output_dir)?;
            println!(
                "success: native-origin audit scanned {} sources and {} objects ({})",
                result.source_count, result.object_count, result.semantic_hash
            );
            Ok(())
        }
        "generate-linkage-metadata" => {
            let result = linkage::generate(
                &PathBuf::from("."),
                &options.output_dir,
                &PathBuf::from("crates/slatec-src/metadata/family-source-closure.json"),
            )?;
            println!(
                "success: snapshot {} families {} ({})",
                result.snapshot_id, result.family_count, result.semantic_hash
            );
            Ok(())
        }
        "acquire-provider-sources" => {
            let result = provider::acquire(
                &PathBuf::from("crates/slatec-src/metadata/family-source-closure.json"),
                &options.output_dir,
                options.offline,
            )?;
            println!(
                "success: provider cache {} sources (downloaded {}, existing {})",
                result.total, result.downloaded, result.verified_existing
            );
            Ok(())
        }
        "generate-provider-metadata" => {
            let semantic_hash = provider::generate_metadata(&PathBuf::from("."))?;
            println!("success: provider metadata ({semantic_hash})");
            Ok(())
        }
        _ => Err(CorpusError::Policy(format!(
            "unknown command {}; use acquire, verify, inspect, extract, manifest, prepare, scan-program-units, scan-prologues, analyze-prologues, audit-full-corpus, select-full-corpus, scan-ffi-inventory, probe-native-ffi, generate-raw-ffi, build-native-ffi, validate-raw-ffi, validate-runtime-profile, generate-safe-special-api, generate-safe-quadrature-api, generate-safe-roots-api, generate-safe-nonlinear-api, generate-safe-nonlinear-expert-api, generate-safe-least-squares-api, generate-safe-linear-least-squares-api, generate-safe-linear-programming-deferred-metadata, generate-safe-lp-in-memory-metadata, generate-safe-fftpack-metadata, generate-safe-fftpack-complex-metadata, generate-safe-pchip-metadata, generate-safe-bspline-metadata, generate-safe-piecewise-polynomial-metadata, generate-safe-ode-sdrive-metadata, generate-safe-dassl-metadata, generate-optimization-audit, generate-ode-audit, generate-safe-bounded-linear-least-squares-api, generate-safe-bounded-constrained-linear-least-squares-api, generate-safe-bounded-constrained-linear-least-squares-api, generate-safe-constrained-linear-least-squares-api, generate-safe-api-docs, generate-runtime-storage-policy, generate-blas1-concurrency-audit, generate-native-origin-audit, generate-linkage-metadata, acquire-provider-sources, or generate-provider-metadata",
            options.command
        ))),
    }
}

fn parse_options() -> Result<Options> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .ok_or_else(|| CorpusError::Policy(usage().to_owned()))?;
    if command == "--help" || command == "help" {
        println!("{}", usage());
        std::process::exit(0);
    }
    let mut options = Options {
        command,
        artifact_path: PathBuf::from("evidence/downloads/slatec_src.tgz"),
        evidence_dir: PathBuf::from("evidence"),
        manifest_dir: PathBuf::from("generated/corpus"),
        program_unit_dir: PathBuf::from("generated/program-units"),
        full_corpus_dir: PathBuf::from("generated/full-corpus"),
        selected_corpus_dir: PathBuf::from("generated/selected-corpus"),
        ffi_inventory_dir: PathBuf::from("generated/ffi-inventory"),
        bindings_dir: PathBuf::from("crates/slatec-sys/src/generated"),
        output_dir: PathBuf::from("generated/corpus"),
        batches: Vec::new(),
        offline: false,
    };
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--artifact-path" => {
                options.artifact_path = PathBuf::from(required_value(&mut args, "--artifact-path")?)
            }
            "--evidence-dir" => {
                options.evidence_dir = PathBuf::from(required_value(&mut args, "--evidence-dir")?)
            }
            "--manifest-dir" => {
                options.manifest_dir = PathBuf::from(required_value(&mut args, "--manifest-dir")?)
            }
            "--program-unit-dir" => {
                options.program_unit_dir =
                    PathBuf::from(required_value(&mut args, "--program-unit-dir")?)
            }
            "--full-corpus-dir" => {
                options.full_corpus_dir =
                    PathBuf::from(required_value(&mut args, "--full-corpus-dir")?)
            }
            "--selected-corpus-dir" => {
                options.selected_corpus_dir =
                    PathBuf::from(required_value(&mut args, "--selected-corpus-dir")?)
            }
            "--ffi-inventory-dir" => {
                options.ffi_inventory_dir =
                    PathBuf::from(required_value(&mut args, "--ffi-inventory-dir")?)
            }
            "--bindings-dir" => {
                options.bindings_dir = PathBuf::from(required_value(&mut args, "--bindings-dir")?)
            }
            "--output-dir" => {
                options.output_dir = PathBuf::from(required_value(&mut args, "--output-dir")?)
            }
            "--batch" => options.batches.push(required_value(&mut args, "--batch")?),
            "--offline" => options.offline = true,
            "--help" => return Err(CorpusError::Policy(usage().to_owned())),
            _ => {
                return Err(CorpusError::Policy(format!(
                    "unknown option {argument}\n{}",
                    usage()
                )));
            }
        }
    }
    Ok(options)
}

fn required_value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String> {
    args.next()
        .ok_or_else(|| CorpusError::Policy(format!("{flag} requires a path")))
}

fn usage() -> &'static str {
    "Usage: slatec-corpus <acquire|verify|inspect|extract|manifest|prepare|scan-program-units|scan-prologues|analyze-prologues|audit-full-corpus|select-full-corpus|scan-ffi-inventory|probe-native-ffi|generate-raw-ffi|build-native-ffi|validate-raw-ffi|validate-runtime-profile|generate-safe-special-api|generate-safe-quadrature-api|generate-safe-roots-api|generate-safe-nonlinear-api|generate-safe-nonlinear-expert-api|generate-safe-least-squares-api|generate-safe-linear-least-squares-api|generate-safe-linear-programming-deferred-metadata|generate-safe-lp-in-memory-metadata|generate-safe-fftpack-metadata|generate-safe-pchip-metadata|generate-safe-bspline-metadata|generate-safe-piecewise-polynomial-metadata|generate-safe-banded-metadata|generate-safe-ode-sdrive-metadata|generate-safe-dassl-metadata|generate-optimization-audit|generate-ode-audit|generate-safe-bounded-linear-least-squares-api|generate-safe-bounded-constrained-linear-least-squares-api|generate-safe-bounded-constrained-linear-least-squares-api|generate-safe-constrained-linear-least-squares-api|generate-safe-api-docs|generate-runtime-storage-policy|generate-blas1-concurrency-audit|generate-native-origin-audit|generate-linkage-metadata|acquire-provider-sources|generate-provider-metadata> [--artifact-path PATH] [--evidence-dir PATH] [--manifest-dir PATH] [--program-unit-dir PATH] [--full-corpus-dir PATH] [--selected-corpus-dir PATH] [--ffi-inventory-dir PATH] [--bindings-dir PATH] [--output-dir PATH] [--batch NAME] [--offline]"
}
