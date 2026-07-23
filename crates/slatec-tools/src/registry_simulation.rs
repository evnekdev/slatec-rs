//! Registry-only downstream simulation using packaged crates and a local directory source.
//!
//! The portable CI jobs run the consumer for their own supported bundled
//! target.  `SLATEC_REGISTRY_TARGET=both` is deliberately a local Windows
//! convenience mode: it runs the Windows GNU consumer on the host and the
//! Linux GNU consumer through WSL.  CI must not depend on WSL being present.

use crate::error::{CorpusError, Result};
use crate::hash;
use flate2::read::GzDecoder;
use serde_json::json;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;

const WINDOWS_TARGET: &str = "x86_64-pc-windows-gnu";
const LINUX_TARGET: &str = "x86_64-unknown-linux-gnu";
const WINDOWS_CARRIER: &str = "slatec-bundled-x86_64-pc-windows-gnu";
const LINUX_CARRIER: &str = "slatec-bundled-x86_64-unknown-linux-gnu";

const ALL_BUNDLED_SOURCE: &str = r#"
fn main() {
    let mut total = 0.0_f64;
    total += slatec::roots::find_root(
        |x| x - 1.0,
        slatec::roots::RootBracket { lower: 0.0, upper: 2.0 },
        slatec::roots::RootOptions::default(),
    ).expect("FZERO").estimate;
    total += slatec::special::airy::airy_ai(0.0).expect("Airy");
    total += slatec::special::bessel::bessel_j0(1.0).expect("Bessel");
    total += slatec::special::gamma::beta(0.5, 0.5).expect("beta");
    total += slatec::special::elementary::log1p(0.5).expect("elementary");
    total += slatec::special::error_functions::erf(0.5).expect("error");
    total += slatec::special::gamma::gamma(0.5).expect("gamma");
    total += slatec::special::integrals::exponential_integral_e1(1.0).expect("integral");
    total += slatec::polynomials::chebyshev::chebyshev_series(0.0, &[1.0]).expect("polynomial");
    total += slatec::special::scalar_expanded::spence_integral(0.5).expect("scalar");
    println!("{total}");
}
"#;

#[derive(Clone, Debug, serde::Serialize)]
pub struct RegistrySimulationResult {
    pub status: String,
    pub local_packages: usize,
    pub downstream_configurations: usize,
    pub semantic_hash: String,
    pub output_path: PathBuf,
}

#[derive(Clone, Copy)]
enum RequestedTarget {
    Windows,
    Linux,
    Both,
}

impl RequestedTarget {
    fn parse() -> Result<Self> {
        match env::var("SLATEC_REGISTRY_TARGET")
            .unwrap_or_else(|_| "host".to_owned())
            .as_str()
        {
            "host" => {
                if cfg!(windows) {
                    Ok(Self::Windows)
                } else if cfg!(target_os = "linux") {
                    Ok(Self::Linux)
                } else {
                    Err(policy(
                        "registry simulation host mode supports Windows or Linux only; set SLATEC_REGISTRY_TARGET explicitly",
                    ))
                }
            }
            "windows" => Ok(Self::Windows),
            "linux" => Ok(Self::Linux),
            "both" => Ok(Self::Both),
            value => Err(policy(&format!(
                "unknown SLATEC_REGISTRY_TARGET `{value}`; expected host, windows, linux, or both"
            ))),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::Both => "both",
        }
    }
}

#[derive(Clone, Copy)]
enum ConsumerExecutor {
    Host,
    Wsl,
}

impl ConsumerExecutor {
    fn name(self) -> &'static str {
        match self {
            Self::Host => "host",
            Self::Wsl => "wsl",
        }
    }
}

#[derive(Clone, Copy)]
struct Consumer {
    name: &'static str,
    dependency: &'static str,
    source: &'static str,
    target: Option<&'static str>,
    run: bool,
    executor: ConsumerExecutor,
}

pub fn validate(root: &Path, output_path: &Path) -> Result<RegistrySimulationResult> {
    let target = RequestedTarget::parse()?;
    if matches!(target, RequestedTarget::Windows) && !cfg!(windows) {
        return Err(policy("Windows registry simulation must run on Windows"));
    }
    if matches!(target, RequestedTarget::Linux) && !cfg!(target_os = "linux") {
        return Err(policy("Linux registry simulation must run on Linux"));
    }
    if matches!(target, RequestedTarget::Both) && !cfg!(windows) {
        return Err(policy(
            "the cross-target `both` registry simulation is a Windows+WSL local mode; run target-specific checks in CI",
        ));
    }

    let canonical_root = root.canonicalize()?;
    let root = canonical_root.as_path();
    let simulation_parent = root.join("target/release-readiness-registry");
    let target_root = root.join("target");
    if !simulation_parent.starts_with(&target_root) {
        return Err(policy(
            "registry simulation escaped the workspace target directory",
        ));
    }
    let simulation = create_simulation_directory(&simulation_parent)?;
    let vendor = simulation.join("vendor");
    fs::create_dir_all(&vendor)?;
    run(
        root,
        Command::new("cargo")
            .args(["vendor", "--versioned-dirs"])
            .arg(&vendor)
            .env("CARGO_NET_OFFLINE", "true"),
        "cargo vendor",
    )?;
    let local_source_config = source_config(&vendor);
    let cargo_home = simulation.join("cargo-home");
    fs::create_dir_all(&cargo_home)?;
    fs::write(cargo_home.join("config.toml"), &local_source_config)?;

    let packages = [
        WINDOWS_CARRIER,
        LINUX_CARRIER,
        "slatec-sys",
        "slatec-core",
        "slatec-src",
        "slatec",
    ];
    for package in packages {
        run(
            root,
            Command::new("cargo")
                .args([
                    "package",
                    "-p",
                    package,
                    "--allow-dirty",
                    "--no-verify",
                    "--offline",
                ])
                .env("CARGO_HOME", &cargo_home),
            &format!("cargo package {package}"),
        )?;
        let archive_path = root
            .join("target/package")
            .join(format!("{package}-0.1.0.crate"));
        let bytes = fs::read(&archive_path)?;
        let archive_hash = hash::bytes(&bytes);
        Archive::new(GzDecoder::new(bytes.as_slice())).unpack(&vendor)?;
        let unpacked = vendor.join(format!("{package}-0.1.0"));
        write_directory_checksum(&unpacked, &archive_hash)?;
    }

    let cargo_config = simulation.join(".cargo/config.toml");
    fs::create_dir_all(cargo_config.parent().expect("config parent"))?;
    fs::write(&cargo_config, &local_source_config)?;

    let consumer_root = simulation.join("consumers");
    let mut records = generic_consumers(&consumer_root)?;
    match target {
        RequestedTarget::Windows => run_target_consumers(
            &consumer_root,
            WINDOWS_TARGET,
            WINDOWS_CARRIER,
            ConsumerExecutor::Host,
            &mut records,
        )?,
        RequestedTarget::Linux => run_target_consumers(
            &consumer_root,
            LINUX_TARGET,
            LINUX_CARRIER,
            ConsumerExecutor::Host,
            &mut records,
        )?,
        RequestedTarget::Both => {
            run_target_consumers(
                &consumer_root,
                WINDOWS_TARGET,
                WINDOWS_CARRIER,
                ConsumerExecutor::Host,
                &mut records,
            )?;
            let wsl_cargo_home = simulation.join("wsl-cargo-home");
            fs::create_dir_all(&wsl_cargo_home)?;
            let wsl_source_config = source_config(&PathBuf::from(wsl_path(&vendor)?));
            fs::write(wsl_cargo_home.join("config.toml"), &wsl_source_config)?;
            run_target_consumers_with_wsl(
                &consumer_root,
                LINUX_CARRIER,
                &wsl_cargo_home,
                &wsl_source_config,
                &mut records,
            )?;
        }
    }
    let report = json!({
        "schema_id":"slatec-rs.registry-only-downstream-simulation",
        "schema_version":"2.0.0",
        "method":"cargo package archives installed into a Cargo directory source together with vendored registry dependencies; bundled consumers set SLATEC_GFORTRAN to an invalid executable and clear source/system configuration",
        "execution_target":target.name(),
        "local_packages":packages,
        "downstream_configurations":records,
        "workspace_path_dependencies":0,
        "result":"pass",
    });
    let mut bytes = serde_json::to_vec_pretty(&report)?;
    bytes.push(b'\n');
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, &bytes)?;
    Ok(RegistrySimulationResult {
        status: "validated".to_owned(),
        local_packages: packages.len(),
        downstream_configurations: report["downstream_configurations"]
            .as_array()
            .map_or(0, Vec::len),
        semantic_hash: hash::bytes(&bytes),
        output_path: output_path.to_path_buf(),
    })
}

/// Creates a private, ignored working directory for one registry rehearsal.
///
/// Do not remove the previous shared directory here.  Windows file browsers,
/// virus scanners, and interrupted Cargo commands can retain a handle inside
/// it briefly; deleting it made the next validation fail with `os error 32`.
/// A process-unique directory keeps every invocation isolated and leaves only
/// disposable files below the already ignored `target/` tree.
fn create_simulation_directory(parent: &Path) -> Result<PathBuf> {
    fs::create_dir_all(parent)?;
    let process = std::process::id();
    for attempt in 0..1024_u32 {
        let candidate = parent.join(format!("run-{process}-{attempt}"));
        match fs::create_dir(&candidate) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error.into()),
        }
    }
    Err(policy(
        "could not allocate an isolated registry-simulation directory after 1,024 attempts",
    ))
}

fn source_config(vendor: &Path) -> String {
    format!(
        "[source.crates-io]\nreplace-with = \"local-release-vendor\"\n\n[source.local-release-vendor]\ndirectory = {:?}\n\n[net]\noffline = true\n",
        vendor.to_string_lossy()
    )
}

fn generic_consumers(consumer_root: &Path) -> Result<Vec<serde_json::Value>> {
    let configurations = [
        Consumer {
            name: "raw-no-default",
            dependency: "slatec-sys = { version = \"=0.1.0\", default-features = false }",
            source: "fn main() {}\n",
            target: None,
            run: false,
            executor: ConsumerExecutor::Host,
        },
        Consumer {
            name: "raw-all-declarations",
            dependency: "slatec-sys = { version = \"=0.1.0\", default-features = false, features = [\"all\"] }",
            source: "fn main() { let _ = slatec_sys::special::airy::ai as *const (); }\n",
            target: None,
            run: false,
            executor: ConsumerExecutor::Host,
        },
        Consumer {
            name: "external-provider-narrow",
            dependency: "slatec-src = { version = \"=0.1.0\", default-features = false, features = [\"external-backend\", \"special-gamma\"] }",
            source: "fn main() { slatec_src::ensure_linked(); }\n",
            target: None,
            run: false,
            executor: ConsumerExecutor::Host,
        },
        Consumer {
            name: "safe-no-default",
            dependency: "slatec = { version = \"=0.1.0\", default-features = false }",
            source: "fn main() {}\n",
            target: None,
            run: false,
            executor: ConsumerExecutor::Host,
        },
        Consumer {
            name: "safe-narrow-external",
            dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"external-backend\", \"special-gamma\"] }",
            source: "fn main() { let _ = slatec::special::gamma::gamma(1.0_f64); }\n",
            target: None,
            run: false,
            executor: ConsumerExecutor::Host,
        },
    ];
    let mut records = Vec::new();
    for consumer in configurations {
        execute_consumer(consumer_root, consumer, None)?;
        records.push(record(consumer, "pass"));
    }
    Ok(records)
}

fn run_target_consumers(
    consumer_root: &Path,
    target: &'static str,
    carrier: &'static str,
    executor: ConsumerExecutor,
    records: &mut Vec<serde_json::Value>,
) -> Result<()> {
    let prefix = if target == WINDOWS_TARGET {
        "windows"
    } else {
        "linux"
    };
    let single = Consumer {
        name: if target == WINDOWS_TARGET {
            "safe-bundled-elementary-windows"
        } else {
            "safe-bundled-elementary-linux"
        },
        dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"bundled\", \"special-elementary\"] }",
        source: "fn main() { println!(\"{}\", slatec::special::elementary::log1p(0.5_f64).expect(\"finite input\")); }\n",
        target: Some(target),
        run: true,
        executor,
    };
    let all = Consumer {
        name: if target == WINDOWS_TARGET {
            "safe-bundled-all-families-windows"
        } else {
            "safe-bundled-all-families-linux"
        },
        dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"bundled\", \"roots-scalar\", \"special\"] }",
        source: ALL_BUNDLED_SOURCE,
        target: Some(target),
        run: true,
        executor,
    };
    for consumer in [single, all] {
        execute_consumer(consumer_root, consumer, None)?;
        records.push(record(consumer, "pass"));
    }
    let system = Consumer {
        name: if target == WINDOWS_TARGET {
            "system-provider-compile-windows"
        } else {
            "system-provider-compile-linux"
        },
        dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"system\", \"special-elementary\"] }",
        source: "fn main() { let _ = slatec::special::elementary::log1p(0.5_f64); }\n",
        target: Some(target),
        run: false,
        executor,
    };
    let native = consumer_root
        .ancestors()
        .nth(1)
        .expect("simulation root")
        .join("vendor")
        .join(format!("{carrier}-0.1.0/native"));
    let system_runtime = consumer_root
        .ancestors()
        .nth(1)
        .expect("simulation root")
        .join(format!("system-runtime-{prefix}"));
    fs::create_dir_all(&system_runtime)?;
    for (source, destination) in [
        ("libslatec_bundle_gfortran.a", "libgfortran.a"),
        ("libslatec_bundle_quadmath.a", "libquadmath.a"),
    ] {
        fs::copy(native.join(source), system_runtime.join(destination))?;
    }
    let system_environment = [
        ("SLATEC_SYSTEM_LIB_DIR", native.clone()),
        ("SLATEC_SYSTEM_RUNTIME_LIB_DIR", system_runtime),
    ];
    execute_consumer(consumer_root, system, Some(&system_environment))?;
    records.push(record(system, "pass"));
    let unavailable = Consumer {
        name: if target == WINDOWS_TARGET {
            "unsupported-bundled-family-windows"
        } else {
            "unsupported-bundled-family-linux"
        },
        dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"bundled\", \"quadrature-basic\"] }",
        source: "fn main() {}\n",
        target: Some(target),
        run: false,
        executor,
    };
    execute_expected_failure(
        consumer_root,
        unavailable,
        "bundled SLATEC provider has no redistributable archive",
    )?;
    records.push(json!({
        "configuration":unavailable.name,
        "target":target,
        "executor":executor.name(),
        "workspace_path_dependencies":0,
        "expected_diagnostic":"bundled SLATEC provider has no redistributable archive",
        "result":"expected-failure-observed",
    }));
    if target == WINDOWS_TARGET && matches!(executor, ConsumerExecutor::Host) {
        let unsupported_target = Consumer {
            name: "unsupported-bundled-target-windows-msvc",
            dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"bundled\", \"special-elementary\"] }",
            source: "fn main() {}\n",
            target: Some("x86_64-pc-windows-msvc"),
            run: false,
            executor: ConsumerExecutor::Host,
        };
        execute_expected_failure(
            consumer_root,
            unsupported_target,
            "The bundled SLATEC provider does not support",
        )?;
        records.push(json!({
            "configuration":unsupported_target.name,
            "target":"x86_64-pc-windows-msvc",
            "executor":executor.name(),
            "workspace_path_dependencies":0,
            "expected_diagnostic":"The bundled SLATEC provider does not support",
            "result":"expected-failure-observed",
        }));
    } else {
        records.push(json!({
            "configuration":format!("unsupported-target-diagnostic-{prefix}"),
            "target":"x86_64-pc-windows-msvc",
            "expected_diagnostic":"The bundled SLATEC provider does not support",
            "executor":executor.name(),
            "workspace_path_dependencies":0,
            "result":"manual-target-toolchain-gate",
        }));
    }
    records.push(json!({
        "configuration":format!("source-build-{prefix}"),
        "target":target,
        "executor":executor.name(),
        "workspace_path_dependencies":0,
        "result":"manual-release-gate",
        "reason":"requires an explicitly acquired checksum-verified source cache and a matching GNU Fortran toolchain; it is not silently substituted by bundled consumers",
    }));
    Ok(())
}

fn run_target_consumers_with_wsl(
    consumer_root: &Path,
    carrier: &'static str,
    cargo_home: &Path,
    source_config: &str,
    records: &mut Vec<serde_json::Value>,
) -> Result<()> {
    let consumers = [
        Consumer {
            name: "safe-bundled-elementary-linux",
            dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"bundled\", \"special-elementary\"] }",
            source: "fn main() { println!(\"{}\", slatec::special::elementary::log1p(0.5_f64).expect(\"finite input\")); }\n",
            target: Some(LINUX_TARGET),
            run: true,
            executor: ConsumerExecutor::Wsl,
        },
        Consumer {
            name: "safe-bundled-all-families-linux",
            dependency: "slatec = { version = \"=0.1.0\", default-features = false, features = [\"std\", \"bundled\", \"roots-scalar\", \"special\"] }",
            source: ALL_BUNDLED_SOURCE,
            target: Some(LINUX_TARGET),
            run: true,
            executor: ConsumerExecutor::Wsl,
        },
    ];
    for consumer in consumers {
        write_consumer(consumer_root, consumer)?;
        run_wsl_consumer(
            &consumer_root.join(consumer.name),
            cargo_home,
            source_config,
            consumer.name,
        )?;
        records.push(record(consumer, "pass"));
    }
    records.push(json!({
        "configuration":"system-provider-compile-linux",
        "target":LINUX_TARGET,
        "executor":"wsl",
        "workspace_path_dependencies":0,
        "result":"manual-release-gate",
        "reason":format!("the cross-target helper does not pass Windows paths to the packaged Linux system provider; run SLATEC_REGISTRY_TARGET=linux on a Linux host"),
        "carrier":carrier,
    }));
    Ok(())
}

fn execute_consumer(
    consumer_root: &Path,
    consumer: Consumer,
    extra_environment: Option<&[(&str, PathBuf)]>,
) -> Result<()> {
    let dir = write_consumer(consumer_root, consumer)?;
    match consumer.executor {
        ConsumerExecutor::Host => {
            let mut command = Command::new("cargo");
            command
                .arg(if consumer.run { "run" } else { "check" })
                .arg("--offline")
                .env("CARGO_NET_OFFLINE", "true")
                .env(
                    "SLATEC_GFORTRAN",
                    "__slatec_no_gfortran_at_consumer_build_time__",
                )
                .env_remove("SLATEC_SOURCE_CACHE");
            if let Some(target) = consumer.target {
                command.args(["--target", target]);
            }
            if let Some(environment) = extra_environment {
                for (key, value) in environment {
                    command.env(key, value);
                }
                command.env("SLATEC_SYSTEM_LIB_NAME", "slatec_bundle_accepted");
            } else {
                command
                    .env_remove("SLATEC_SYSTEM_LIB_DIR")
                    .env_remove("SLATEC_SYSTEM_RUNTIME_LIB_DIR")
                    .env_remove("SLATEC_SYSTEM_LIB_NAME");
            }
            run(&dir, &mut command, consumer.name)
        }
        ConsumerExecutor::Wsl => Err(policy(
            "WSL consumers must be dispatched by run_target_consumers_with_wsl",
        )),
    }
}

fn execute_expected_failure(
    consumer_root: &Path,
    consumer: Consumer,
    expected_diagnostic: &str,
) -> Result<()> {
    if !matches!(consumer.executor, ConsumerExecutor::Host) {
        return Err(policy(
            "WSL expected-failure consumers must be dispatched through a host target validation",
        ));
    }
    let dir = write_consumer(consumer_root, consumer)?;
    let mut command = Command::new("cargo");
    command
        .args(["check", "--offline"])
        .env("CARGO_NET_OFFLINE", "true")
        .env(
            "SLATEC_GFORTRAN",
            "__slatec_no_gfortran_at_consumer_build_time__",
        )
        .env_remove("SLATEC_SOURCE_CACHE")
        .env_remove("SLATEC_SYSTEM_LIB_DIR")
        .env_remove("SLATEC_SYSTEM_RUNTIME_LIB_DIR")
        .env_remove("SLATEC_SYSTEM_LIB_NAME");
    if let Some(target) = consumer.target {
        command.args(["--target", target]);
    }
    let output = command.current_dir(&dir).output()?;
    if output.status.success() {
        return Err(policy(&format!(
            "{} unexpectedly succeeded; expected `{expected_diagnostic}`",
            consumer.name
        )));
    }
    let detail = String::from_utf8_lossy(&output.stderr);
    if detail.contains(expected_diagnostic) {
        Ok(())
    } else {
        Err(policy(&format!(
            "{} failed with the wrong diagnostic; expected `{expected_diagnostic}`\nstderr:\n{}",
            consumer.name,
            detail.trim()
        )))
    }
}

fn write_consumer(consumer_root: &Path, consumer: Consumer) -> Result<PathBuf> {
    let dir = consumer_root.join(consumer.name);
    fs::create_dir_all(dir.join("src"))?;
    fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{}\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n\n[dependencies]\n{}\n",
            consumer.name, consumer.dependency
        ),
    )?;
    fs::write(dir.join("src/main.rs"), consumer.source)?;
    Ok(dir)
}

fn record(consumer: Consumer, result: &str) -> serde_json::Value {
    json!({
        "configuration":consumer.name,
        "dependency":consumer.dependency,
        "target":consumer.target,
        "executor":consumer.executor.name(),
        "workspace_path_dependencies":0,
        "result":result,
    })
}

fn write_directory_checksum(directory: &Path, package_hash: &str) -> Result<()> {
    let mut files = BTreeMap::new();
    let mut pending = vec![directory.to_path_buf()];
    while let Some(current) = pending.pop() {
        for entry in fs::read_dir(current)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.file_name().and_then(|name| name.to_str())
                != Some(".cargo-checksum.json")
            {
                let relative = path
                    .strip_prefix(directory)
                    .map_err(|_| policy("packaged file escaped its directory"))?
                    .to_string_lossy()
                    .replace('\\', "/");
                files.insert(relative, hash::bytes(&fs::read(path)?));
            }
        }
    }
    let value = json!({"files":files,"package":package_hash});
    let mut bytes = serde_json::to_vec(&value)?;
    bytes.push(b'\n');
    fs::write(directory.join(".cargo-checksum.json"), bytes)?;
    Ok(())
}

fn run(root: &Path, command: &mut Command, label: &str) -> Result<()> {
    let output = command.current_dir(root).output()?;
    if !output.status.success() {
        return Err(policy(&format!(
            "{label} failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(())
}

fn run_wsl_consumer(
    directory: &Path,
    cargo_home: &Path,
    source_config: &str,
    label: &str,
) -> Result<()> {
    let config = directory.join(".cargo/config.toml");
    fs::create_dir_all(config.parent().expect("consumer Cargo config parent"))?;
    fs::write(config, source_config)?;
    let directory = wsl_path(directory)?;
    let cargo_home = wsl_path(cargo_home)?;
    let script = format!(
        "env -u SLATEC_SOURCE_CACHE -u SLATEC_SYSTEM_LIB_DIR -u SLATEC_SYSTEM_RUNTIME_LIB_DIR SLATEC_GFORTRAN=__slatec_no_gfortran_at_consumer_build_time__ CARGO_HOME={} CARGO_NET_OFFLINE=true cargo run --offline --target {LINUX_TARGET}",
        bash_quote(&cargo_home),
    );
    let output = Command::new("wsl.exe")
        .args(["--cd", &directory, "bash", "-lc", &script])
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(policy(&format!(
            "{label} failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stderr).trim()
        )))
    }
}

fn wsl_path(path: &Path) -> Result<String> {
    let path = path.to_string_lossy().replace('\\', "/");
    let path = path.strip_prefix("//?/").unwrap_or(&path);
    let bytes = path.as_bytes();
    if bytes.len() < 3 || bytes[1] != b':' || bytes[2] != b'/' {
        return Err(policy(&format!(
            "WSL registry simulation requires an absolute Windows drive path; found {path}"
        )));
    }
    Ok(format!(
        "/mnt/{}/{}",
        (bytes[0] as char).to_ascii_lowercase(),
        &path[3..]
    ))
}

fn bash_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\\"'\\\"'"))
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(message.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_runs_receive_private_target_directories() {
        let directory = tempfile::tempdir().expect("temporary directory");
        let parent = directory.path().join("target/release-readiness-registry");
        let first = create_simulation_directory(&parent).expect("first directory");
        let second = create_simulation_directory(&parent).expect("second directory");

        assert!(first.starts_with(&parent));
        assert!(second.starts_with(&parent));
        assert_ne!(first, second);
        assert!(first.is_dir());
        assert!(second.is_dir());
    }
}
