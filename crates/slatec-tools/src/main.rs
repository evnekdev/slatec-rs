use slatec_tools::acquire;
use slatec_tools::archive::{inspect_archive, verify_artifact};
use slatec_tools::error::{CorpusError, Result};
use slatec_tools::extract;
use slatec_tools::manifest;
use slatec_tools::policy::Policy;
use slatec_tools::program_units;
use std::path::PathBuf;

#[derive(Debug)]
struct Options {
    command: String,
    artifact_path: PathBuf,
    evidence_dir: PathBuf,
    manifest_dir: PathBuf,
    output_dir: PathBuf,
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
        _ => Err(CorpusError::Policy(format!(
            "unknown command {}; use acquire, verify, inspect, extract, manifest, prepare, or scan-program-units",
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
        output_dir: PathBuf::from("generated/corpus"),
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
            "--output-dir" => {
                options.output_dir = PathBuf::from(required_value(&mut args, "--output-dir")?)
            }
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
    "Usage: slatec-corpus <acquire|verify|inspect|extract|manifest|prepare|scan-program-units> [--artifact-path PATH] [--evidence-dir PATH] [--manifest-dir PATH] [--output-dir PATH] [--offline]"
}
