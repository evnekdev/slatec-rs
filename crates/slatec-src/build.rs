use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const TARGET: &str = "x86_64-pc-windows-gnu";
const ACQUIRE_COMMAND: &str = "cargo run -p slatec-tools --bin slatec-corpus -- acquire-provider-sources --output-dir <SLATEC_SOURCE_CACHE>";
const FAMILY_FEATURES: &[(&str, &str)] = &[
    ("BLAS_LEVEL1", "blas-level1"),
    ("BLAS_LEVEL2", "blas-level2"),
    ("BLAS_LEVEL3", "blas-level3"),
    ("SPECIAL_ELEMENTARY", "special-elementary"),
    ("SPECIAL_GAMMA", "special-gamma"),
    ("SPECIAL_BETA", "special-beta"),
    ("SPECIAL_ERROR", "special-error"),
    ("SPECIAL_AIRY", "special-airy"),
    ("SPECIAL_BESSEL", "special-bessel"),
    ("SPECIAL_INTEGRALS", "special-integrals"),
    ("SPECIAL_POLYNOMIALS", "special-polynomials"),
    ("SPECIAL_SCALAR_EXPANDED", "special-scalar-expanded"),
    ("QUADRATURE_BASIC", "quadrature-basic"),
    ("QUADRATURE_BREAKPOINTS", "quadrature-breakpoints"),
    ("QUADRATURE_WEIGHTED", "quadrature-weighted"),
    ("QUADRATURE_OSCILLATORY", "quadrature-oscillatory"),
    ("QUADRATURE_FOURIER", "quadrature-fourier"),
    ("QUADRATURE_NONADAPTIVE", "quadrature-nonadaptive"),
    ("QUADRATURE_CALLBACKS", "quadrature-callbacks"),
    ("ROOTS_SCALAR", "roots-scalar"),
    ("ROOTS_POLYNOMIAL", "roots-polynomial"),
    ("NONLINEAR_EASY", "nonlinear-easy"),
    ("NONLINEAR_EXPERT", "nonlinear-expert"),
    ("NONLINEAR_JACOBIAN_CHECK", "nonlinear-jacobian-check"),
    ("NONLINEAR_SYSTEMS", "nonlinear-systems"),
    ("ODE_SDRIVE_EXPERT", "ode-sdrive-expert"),
    ("DASSL", "dassl"),
    (
        "OPTIMIZATION_LINEAR_PROGRAMMING_IN_MEMORY",
        "optimization-linear-programming-in-memory",
    ),
    ("FFTPACK_REAL", "fftpack-real"),
    ("FFTPACK_COMPLEX", "fftpack-complex"),
    ("FISHPACK_CARTESIAN_2D", "fishpack-cartesian-2d"),
    ("FISHPACK_POIS3D", "fishpack-pois3d"),
    ("FISHPACK_CYLINDRICAL_POLAR", "fishpack-cylindrical-polar"),
    ("FISHPACK_SPHERICAL", "fishpack-spherical"),
    ("BANDED_LINEAR_SYSTEMS", "banded-linear-systems"),
    ("PCHIP", "pchip"),
    ("BSPLINE", "bspline"),
    ("PIECEWISE_POLYNOMIAL", "piecewise-polynomial"),
    ("LINEAR_ALGEBRA_REAL", "linear-algebra-real"),
    ("LINEAR_ALGEBRA_EIGEN", "linear-algebra-eigen"),
    ("SPECIAL_REAL", "special-real"),
    ("QUADRATURE_DIRECT", "quadrature-direct"),
    ("NONLINEAR_JACOBIAN", "nonlinear-jacobian"),
    ("ODE_INTEGRATION", "ode-integration"),
    ("ODE_CALLBACKS", "ode-callbacks"),
    ("FFTPACK_EXTENDED_REAL", "fftpack-extended-real"),
    ("FISHPACK_GENERAL", "fishpack-general"),
    ("INTERPOLATION_GENERAL", "interpolation-general"),
    ("APPROXIMATION_CORE", "approximation-core"),
    ("STATISTICS_CORE", "statistics-core"),
    ("BLAS_COMPLEX", "blas-complex"),
    ("LINEAR_ALGEBRA_COMPLEX", "linear-algebra-complex"),
    ("SPECIAL_COMPLEX", "special-complex"),
    ("NONLINEAR_COMPLEX", "nonlinear-complex"),
    ("FISHPACK_COMPLEX", "fishpack-complex"),
    (
        "LEAST_SQUARES_NONLINEAR_EASY",
        "least-squares-nonlinear-easy",
    ),
    (
        "LEAST_SQUARES_NONLINEAR_EXPERT",
        "least-squares-nonlinear-expert",
    ),
    ("LEAST_SQUARES_COVARIANCE", "least-squares-covariance"),
    (
        "LEAST_SQUARES_LINEAR_NONNEGATIVE",
        "least-squares-linear-nonnegative",
    ),
    (
        "LEAST_SQUARES_LINEAR_BOUNDED",
        "least-squares-linear-bounded",
    ),
    (
        "LEAST_SQUARES_LINEAR_CONSTRAINED",
        "least-squares-linear-constrained",
    ),
    (
        "LEAST_SQUARES_LINEAR_BOUNDED_CONSTRAINED",
        "least-squares-linear-bounded-constrained",
    ),
];

#[derive(Deserialize)]
struct Manifest {
    sources: Vec<Source>,
    families: BTreeMap<String, Vec<String>>,
    profile_override_families: BTreeSet<String>,
}

#[derive(Clone, Deserialize)]
struct Source {
    id: String,
    subset: String,
    path: String,
    sha256: String,
}

#[derive(Deserialize)]
struct FamilyOverlay {
    family: String,
    source_ids: Vec<String>,
    sources: Vec<Source>,
    profile_override: bool,
}

fn main() {
    for name in [
        "SLATEC_SOURCE_CACHE",
        "SLATEC_SYSTEM_LIB_DIR",
        "SLATEC_SYSTEM_LIB_NAME",
        "SLATEC_SYSTEM_RUNTIME_LIB_DIR",
        "SLATEC_GFORTRAN",
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }
    println!("cargo:rerun-if-changed=metadata/family-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/ode-sdrive-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/ode-callbacks-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/dassl-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/lp-in-memory-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/fftpack-real-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/fftpack-complex-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/fishpack-cartesian-2d-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/fishpack-pois3d-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/fishpack-cylindrical-polar-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/fishpack-spherical-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/banded-linear-systems-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/pchip-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/bspline-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/piecewise-polynomial-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/special-scalar-expanded-source-closure.json");
    println!("cargo:rerun-if-changed=metadata/bundled-source-eligibility.json");
    println!("cargo:rerun-if-changed=native/gnu-mingw-x86_64");

    let families = enabled_families();
    if families.is_empty() {
        return;
    }
    let backends = ["BUNDLED", "SOURCE_BUILD", "SYSTEM", "EXTERNAL_BACKEND"]
        .into_iter()
        .filter(|name| feature(name))
        .collect::<Vec<_>>();
    if backends.len() != 1 {
        panic!(
            "native SLATEC families require exactly one backend feature: bundled, source-build, system, or external-backend; enabled {backends:?}. Backend selection should be made by the top-level application"
        );
    }
    match backends[0] {
        "BUNDLED" => link_bundled(&families),
        "SOURCE_BUILD" => build_sources(&families),
        "SYSTEM" => link_system(),
        "EXTERNAL_BACKEND" => (),
        _ => unreachable!(),
    }
}

#[derive(Deserialize)]
struct BundledEligibility {
    families: Vec<BundledFamily>,
}

#[derive(Deserialize)]
struct BundledFamily {
    feature: String,
    bundle_available: bool,
}

fn link_bundled(families: &BTreeSet<String>) {
    let target = env::var("TARGET").unwrap_or_default();
    if target != TARGET {
        panic!(
            "The bundled SLATEC provider does not support {target}.\nSupported bundled targets:\n- {TARGET}\n\nSelect source-build, system, or external-backend explicitly, or build for a supported target."
        );
    }
    let eligibility_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("metadata/bundled-source-eligibility.json");
    let eligibility: BundledEligibility = serde_json::from_slice(
        &fs::read(&eligibility_path).expect("read bundled-source eligibility metadata"),
    )
    .expect("parse bundled-source eligibility metadata");
    let unavailable = families
        .iter()
        .filter(|family| {
            eligibility
                .families
                .iter()
                .find(|record| record.feature == **family)
                .is_none_or(|record| !record.bundle_available)
        })
        .cloned()
        .collect::<Vec<_>>();
    if !unavailable.is_empty() {
        let details = unavailable.join(", ");
        panic!(
            "the bundled SLATEC provider has no redistributable archive for {details}. The generated source-level provenance audit blocks this provider before any compiler, source cache, system archive, or network path is considered. Select source-build, system, or external-backend explicitly."
        );
    }
    let mut linked_archives = BTreeSet::new();
    for family in families {
        let key = family.replace('-', "_").to_ascii_uppercase();
        let prefix = "DEP_SLATEC_BUNDLED_X86_64_PC_WINDOWS_GNU";
        let available = env::var(format!("{prefix}_AVAILABLE_{key}"))
            .ok()
            .as_deref()
            == Some("true");
        let archive = env::var_os(format!("{prefix}_ARCHIVE_{key}")).map(PathBuf::from);
        if !available || archive.as_ref().is_none_or(|path| !path.is_file()) {
            panic!(
                "the bundled SLATEC carrier has no verified archive for {family}. Select source-build, system, or external-backend explicitly."
            );
        }
        let archive = archive.expect("checked archive");
        if linked_archives.insert(archive.clone()) {
            println!(
                "cargo:rustc-link-search=native={}",
                archive.parent().expect("bundled archive parent").display()
            );
            println!(
                "cargo:rustc-link-lib=static={}",
                archive_link_name(&archive)
            );
        }
    }
    let prefix = "DEP_SLATEC_BUNDLED_X86_64_PC_WINDOWS_GNU";
    for component in ["LIBGFORTRAN", "LIBQUADMATH"] {
        let runtime = env::var_os(format!("{prefix}_RUNTIME_{component}")).map(PathBuf::from);
        let runtime_available = env::var(format!("{prefix}_RUNTIME_AVAILABLE_{component}"))
            .ok()
            .as_deref()
            == Some("true");
        if runtime_available {
            let runtime = runtime.expect("bundled runtime path");
            if !runtime.is_file() {
                panic!("the bundled {component} runtime is missing from the verified carrier");
            }
            println!(
                "cargo:rustc-link-search=native={}",
                runtime.parent().expect("bundled runtime parent").display()
            );
            println!(
                "cargo:rustc-link-lib=static={}",
                archive_link_name(&runtime)
            );
        }
    }
}

fn archive_link_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_prefix("lib"))
        .unwrap_or_else(|| {
            panic!(
                "bundled archive path has no Rust linker name: {}",
                path.display()
            )
        })
        .to_owned()
}

fn feature(name: &str) -> bool {
    env::var_os(format!("CARGO_FEATURE_{name}")).is_some()
}

fn enabled_families() -> BTreeSet<String> {
    FAMILY_FEATURES
        .iter()
        .filter(|(env_name, _)| feature(env_name))
        .map(|(_, cargo_name)| (*cargo_name).to_owned())
        .collect()
}

fn link_system() {
    let dir = env::var_os("SLATEC_SYSTEM_LIB_DIR").map(PathBuf::from).unwrap_or_else(|| {
        panic!("system backend requires SLATEC_SYSTEM_LIB_DIR containing the validated static SLATEC archive")
    });
    let name = env::var("SLATEC_SYSTEM_LIB_NAME").unwrap_or_else(|_| "slatec".to_owned());
    let candidates = [
        dir.join(format!("lib{name}.a")),
        dir.join(format!("{name}.lib")),
    ];
    if !candidates.iter().any(|path| path.is_file()) {
        panic!(
            "system backend did not find lib{name}.a or {name}.lib in {}",
            dir.display()
        );
    }
    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=static={name}");
    let runtime = env::var_os("SLATEC_SYSTEM_RUNTIME_LIB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            panic!(
                "system backend requires SLATEC_SYSTEM_RUNTIME_LIB_DIR containing static libgfortran.a and libquadmath.a"
            )
        });
    for (file, name) in [("libgfortran.a", "gfortran"), ("libquadmath.a", "quadmath")] {
        if !runtime.join(file).is_file() {
            panic!(
                "system backend did not find {file} in {}",
                runtime.display()
            );
        }
        println!("cargo:rustc-link-search=native={}", runtime.display());
        println!("cargo:rustc-link-lib=static={name}");
    }
}

fn build_sources(families: &BTreeSet<String>) {
    let target = env::var("TARGET").unwrap_or_default();
    if target != TARGET {
        panic!(
            "source-build currently supports only {TARGET}; found {target}. Select system or external-backend for another target"
        );
    }
    let cache = env::var_os("SLATEC_SOURCE_CACHE").map(PathBuf::from).unwrap_or_else(|| {
        panic!(
            "source-build is offline-only and requires SLATEC_SOURCE_CACHE. Populate it explicitly with: {ACQUIRE_COMMAND}"
        )
    });
    let manifest_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("metadata/family-source-closure.json");
    let mut manifest: Manifest = serde_json::from_slice(
        &fs::read(&manifest_path).expect("read family source closure manifest"),
    )
    .expect("parse family source closure manifest");
    for (family, file) in [
        ("ode-sdrive-expert", "ode-sdrive-source-closure.json"),
        ("ode-callbacks", "ode-callbacks-source-closure.json"),
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
            "fishpack-cylindrical-polar",
            "fishpack-cylindrical-polar-source-closure.json",
        ),
        (
            "fishpack-spherical",
            "fishpack-spherical-source-closure.json",
        ),
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
    ] {
        // The safe DASSL facade scopes `XGETF`/`XSETF` around recoverable
        // native statuses. Its focused closure is the authoritative source
        // selection because the compiler-derived base closure cannot observe
        // those Rust-side runtime calls. Other historical overlays remain
        // additive only when the base manifest has no family entry.
        if families.contains(family)
            && (!manifest.families.contains_key(family) || family == "dassl")
        {
            apply_family_overlay(&mut manifest, family, file, family == "dassl");
        }
    }
    let by_id = manifest
        .sources
        .into_iter()
        .map(|source| (source.id.clone(), source))
        .collect::<BTreeMap<_, _>>();
    let mut selected = BTreeSet::new();
    for family in families {
        selected.extend(
            manifest
                .families
                .get(family)
                .unwrap_or_else(|| panic!("family {family} has no reviewed source closure"))
                .iter()
                .cloned(),
        );
    }
    if selected.is_empty() {
        return;
    }

    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let objects = out.join("objects");
    fs::create_dir_all(&objects).expect("create SLATEC object directory");
    let verified_sources = selected
        .iter()
        .map(|id| {
            let source = by_id
                .get(id)
                .unwrap_or_else(|| panic!("unknown source id {id}"));
            (source.id.clone(), verified_cached_source(&cache, source))
        })
        .collect::<Vec<_>>();
    let compiler = compiler();
    verify_compiler(&compiler);
    let mut object_paths = Vec::new();
    for (id, path) in verified_sources {
        let object = objects.join(format!("{id}.o"));
        compile_one(&compiler, &path, &object);
        object_paths.push(object);
    }
    if families
        .iter()
        .any(|family| manifest.profile_override_families.contains(family))
    {
        for name in ["i1mach", "r1mach", "d1mach"] {
            let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join(format!("native/gnu-mingw-x86_64/{name}.f"));
            let object = objects.join(format!("profile-{name}.o"));
            compile_one(&compiler, &source, &object);
            object_paths.push(object);
        }
    }
    object_paths.sort();
    let archive = out.join("libslatec_family.a");
    archive_objects(&compiler, &archive, &object_paths);
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=slatec_family");
    emit_runtime_links(&compiler);
}

fn apply_family_overlay(
    manifest: &mut Manifest,
    family: &str,
    file: &str,
    replace_existing_family: bool,
) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("metadata")
        .join(file);
    let overlay: FamilyOverlay =
        serde_json::from_slice(&fs::read(&path).expect("read family source closure overlay"))
            .expect("parse family source closure overlay");
    if overlay.family != family {
        panic!("source closure overlay has an unexpected family name");
    }
    for source in overlay.sources {
        if let Some(existing) = manifest
            .sources
            .iter()
            .find(|existing| existing.id == source.id)
        {
            if existing.subset != source.subset
                || existing.path != source.path
                || existing.sha256 != source.sha256
            {
                panic!(
                    "source closure overlays disagree about source {}",
                    source.id
                );
            }
        } else {
            manifest.sources.push(source);
        }
    }
    if manifest
        .families
        .insert(overlay.family.clone(), overlay.source_ids)
        .is_some()
        && !replace_existing_family
    {
        panic!("base source closure unexpectedly already defines {family}");
    }
    if overlay.profile_override {
        manifest.profile_override_families.insert(overlay.family);
    }
}

fn compiler() -> PathBuf {
    env::var_os("SLATEC_GFORTRAN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("gfortran"))
}

fn verify_compiler(compiler: &Path) {
    let target = output(compiler, &["-dumpmachine"]);
    if target.trim() != "x86_64-w64-mingw32" {
        panic!(
            "source-build requires GNU Fortran target x86_64-w64-mingw32; found {}",
            target.trim()
        );
    }
}

fn verified_cached_source(cache: &Path, source: &Source) -> PathBuf {
    let relative = if source.subset == "main-src" {
        source.path.clone()
    } else {
        format!("{}/{}", source.subset, source.path)
    };
    let path = cache.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    if !path.is_file() {
        panic!(
            "source-build cache is missing {} at {}. Populate the cache with: {ACQUIRE_COMMAND}",
            source.id,
            path.display()
        );
    }
    let actual = file_sha256(&path);
    if actual != source.sha256 {
        panic!(
            "source-build cache hash mismatch for {}: expected {}, found {actual}. Reacquire with: {ACQUIRE_COMMAND}",
            source.id, source.sha256
        );
    }
    path
}

fn compile_one(compiler: &Path, source: &Path, object: &Path) {
    let output = Command::new(compiler)
        .args(["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"])
        .arg(source)
        .arg("-o")
        .arg(object)
        .output()
        .unwrap_or_else(|error| panic!("start {}: {error}", compiler.display()));
    if !output.status.success() {
        panic!(
            "GNU Fortran failed for {} with {}\nstdout:\n{}\nstderr:\n{}",
            source.display(),
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}

fn archive_objects(compiler: &Path, archive: &Path, objects: &[PathBuf]) {
    let ar = sibling_tool(compiler, "ar");
    let _ = fs::remove_file(archive);
    for (index, chunk) in objects.chunks(40).enumerate() {
        let status = Command::new(&ar)
            .arg(if index == 0 { "cr" } else { "q" })
            .arg(archive)
            .args(chunk)
            .status()
            .expect("start GNU ar");
        if !status.success() {
            panic!("GNU ar failed creating {}", archive.display());
        }
    }
    if !Command::new(ar)
        .arg("s")
        .arg(archive)
        .status()
        .expect("index GNU archive")
        .success()
    {
        panic!("GNU ar failed indexing {}", archive.display());
    }
}

fn emit_runtime_links(compiler: &Path) {
    for (file, name) in [("libgfortran.a", "gfortran"), ("libquadmath.a", "quadmath")] {
        let path = PathBuf::from(output(compiler, &[&format!("-print-file-name={file}")]).trim());
        if !path.is_file() {
            panic!(
                "source-build requires the static GNU runtime {file}; compiler reported {}",
                path.display()
            );
        }
        println!(
            "cargo:rustc-link-search=native={}",
            path.parent().expect("runtime parent").display()
        );
        println!("cargo:rustc-link-lib=static={name}");
    }
}

fn sibling_tool(compiler: &Path, name: &str) -> PathBuf {
    compiler
        .parent()
        .map(|parent| parent.join(format!("{name}.exe")))
        .filter(|path| path.is_file())
        .unwrap_or_else(|| PathBuf::from(name))
}

fn output(program: &Path, args: &[&str]) -> String {
    let value = Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("start {}: {error}", program.display()));
    if !value.status.success() {
        panic!("{} {args:?} failed", program.display());
    }
    String::from_utf8(value.stdout).expect("tool output is UTF-8")
}

fn file_sha256(path: &Path) -> String {
    format!(
        "{:x}",
        Sha256::digest(fs::read(path).expect("read cached source"))
    )
}
