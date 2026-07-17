use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const TARGET: &str = "x86_64-pc-windows-gnu";
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
    ("QUADRATURE_BASIC", "quadrature-basic"),
    ("QUADRATURE_BREAKPOINTS", "quadrature-breakpoints"),
    ("QUADRATURE_WEIGHTED", "quadrature-weighted"),
    ("QUADRATURE_OSCILLATORY", "quadrature-oscillatory"),
    ("QUADRATURE_FOURIER", "quadrature-fourier"),
    ("QUADRATURE_NONADAPTIVE", "quadrature-nonadaptive"),
    ("ROOTS_SCALAR", "roots-scalar"),
    ("ROOTS_POLYNOMIAL", "roots-polynomial"),
];

#[derive(Deserialize)]
struct Manifest {
    snapshot_id: String,
    sources: Vec<Source>,
    families: BTreeMap<String, Vec<String>>,
}

#[derive(Clone, Deserialize)]
struct Source {
    id: String,
    subset: String,
    path: String,
    sha256: String,
    url: String,
}

fn main() {
    println!("cargo:rerun-if-env-changed=SLATEC_SOURCE_CACHE");
    println!("cargo:rerun-if-env-changed=SLATEC_SYSTEM_LIB_DIR");
    println!("cargo:rerun-if-env-changed=SLATEC_SYSTEM_LIB_NAME");
    println!("cargo:rerun-if-env-changed=SLATEC_GFORTRAN");
    println!("cargo:rerun-if-changed=metadata/family-source-closure.json");
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
            "native SLATEC families require exactly one backend feature: bundled, source-build, system, or external-backend; enabled {backends:?}"
        );
    }
    match backends[0] {
        "EXTERNAL_BACKEND" => (),
        "SYSTEM" => link_system(),
        "BUNDLED" => build_sources(&families, true),
        "SOURCE_BUILD" => build_sources(&families, false),
        _ => unreachable!(),
    }
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
    if let Some(dir) = env::var_os("SLATEC_SYSTEM_LIB_DIR") {
        println!(
            "cargo:rustc-link-search=native={}",
            PathBuf::from(dir).display()
        );
    }
    let name = env::var("SLATEC_SYSTEM_LIB_NAME").unwrap_or_else(|_| "slatec".to_owned());
    println!("cargo:rustc-link-lib=static={name}");
}

fn build_sources(families: &BTreeSet<String>, acquire: bool) {
    let target = env::var("TARGET").unwrap_or_default();
    if target != TARGET {
        panic!(
            "automatic SLATEC source builds currently support only {TARGET}; found {target}. Select system or external-backend for another target"
        );
    }
    let manifest_path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("metadata/family-source-closure.json");
    let manifest: Manifest = serde_json::from_slice(
        &fs::read(&manifest_path).expect("read family source closure manifest"),
    )
    .expect("parse family source closure manifest");
    let _snapshot = &manifest.snapshot_id;
    let by_id = manifest
        .sources
        .into_iter()
        .map(|s| (s.id.clone(), s))
        .collect::<BTreeMap<_, _>>();
    let mut selected = BTreeSet::new();
    for family in families {
        let ids = manifest
            .families
            .get(family)
            .unwrap_or_else(|| panic!("family {family} has no reviewed source closure"));
        selected.extend(ids.iter().cloned());
    }
    if selected.is_empty() {
        return;
    }
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let cache = env::var_os("SLATEC_SOURCE_CACHE")
        .map(PathBuf::from)
        .unwrap_or_else(|| out.join("source-cache"));
    let objects = out.join("objects");
    fs::create_dir_all(&cache).expect("create SLATEC source cache");
    fs::create_dir_all(&objects).expect("create SLATEC object directory");
    let compiler = compiler();
    verify_compiler(&compiler);
    let mut object_paths = Vec::new();
    for id in selected {
        let source = by_id
            .get(&id)
            .unwrap_or_else(|| panic!("unknown source id {id}"));
        let path = cached_source(&cache, source, acquire);
        let object = objects.join(format!("{}.o", source.id));
        compile_one(&compiler, &path, &object);
        object_paths.push(object);
    }
    for name in ["i1mach", "r1mach", "d1mach"] {
        let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("native/gnu-mingw-x86_64/{name}.f"));
        if source.is_file() {
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

fn compiler() -> PathBuf {
    env::var_os("SLATEC_GFORTRAN")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("gfortran"))
}

fn verify_compiler(compiler: &Path) {
    let target = output(compiler, &["-dumpmachine"]);
    if target.trim() != "x86_64-w64-mingw32" {
        panic!(
            "bundled profile requires GNU Fortran target x86_64-w64-mingw32; found {}",
            target.trim()
        );
    }
}

fn cached_source(cache: &Path, source: &Source, acquire: bool) -> PathBuf {
    let relative = if source.subset == "main-src" {
        source.path.clone()
    } else {
        format!("{}/{}", source.subset, source.path)
    };
    let path = cache.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    if path.is_file() && file_sha256(&path) == source.sha256 {
        return path;
    }
    if !acquire {
        panic!(
            "source-build requires verified source {} in SLATEC_SOURCE_CACHE ({})",
            source.id,
            path.display()
        );
    }
    let bytes = download_with_retries(&source.url);
    let actual = hex_sha256(&bytes);
    if actual != source.sha256 {
        panic!(
            "checksum mismatch for {}: expected {}, found {actual}",
            source.url, source.sha256
        );
    }
    fs::create_dir_all(path.parent().expect("source parent")).expect("create source parent");
    fs::write(&path, bytes).expect("write verified source");
    path
}

fn compile_one(compiler: &Path, source: &Path, object: &Path) {
    let status = Command::new(compiler)
        .args(["-x", "f77", "-std=legacy", "-ffixed-line-length-none", "-c"])
        .arg(source)
        .arg("-o")
        .arg(object)
        .status()
        .unwrap_or_else(|e| panic!("start {}: {e}", compiler.display()));
    if !status.success() {
        panic!("GNU Fortran failed for {}", source.display());
    }
}

fn archive_objects(compiler: &Path, archive: &Path, objects: &[PathBuf]) {
    let ar = sibling_tool(compiler, "ar");
    let _ = fs::remove_file(archive);
    for (index, chunk) in objects.chunks(40).enumerate() {
        let mut command = Command::new(&ar);
        command
            .arg(if index == 0 { "cr" } else { "q" })
            .arg(archive)
            .args(chunk);
        let status = command.status().expect("start GNU ar");
        if !status.success() {
            panic!("GNU ar failed creating {}", archive.display());
        }
    }
    let status = Command::new(ar)
        .arg("s")
        .arg(archive)
        .status()
        .expect("index GNU archive");
    if !status.success() {
        panic!("GNU ar failed indexing {}", archive.display());
    }
}

fn download_with_retries(url: &str) -> Vec<u8> {
    let mut last_error = String::new();
    for attempt in 1..=3 {
        match ureq::get(url).call() {
            Ok(response) => {
                let mut body = response.into_body();
                return body
                    .read_to_vec()
                    .unwrap_or_else(|error| panic!("read {url}: {error}"));
            }
            Err(error) => {
                last_error = error.to_string();
                if attempt < 3 {
                    std::thread::sleep(std::time::Duration::from_millis(250 * attempt));
                }
            }
        }
    }
    panic!("download {url} after three attempts: {last_error}")
}

fn emit_runtime_links(compiler: &Path) {
    for library in ["libgfortran.a", "libquadmath.a"] {
        let path =
            PathBuf::from(output(compiler, &[&format!("-print-file-name={library}")]).trim());
        if let Some(parent) = path.parent() {
            println!("cargo:rustc-link-search=native={}", parent.display());
        }
    }
    println!("cargo:rustc-link-lib=static=gfortran");
    println!("cargo:rustc-link-lib=static=quadmath");
}

fn sibling_tool(compiler: &Path, name: &str) -> PathBuf {
    compiler
        .parent()
        .map(|p| p.join(format!("{name}.exe")))
        .filter(|p| p.is_file())
        .unwrap_or_else(|| PathBuf::from(name))
}

fn output(program: &Path, args: &[&str]) -> String {
    let value = Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("start {}: {e}", program.display()));
    if !value.status.success() {
        panic!("{} {:?} failed", program.display(), args);
    }
    String::from_utf8(value.stdout).expect("tool output is UTF-8")
}

fn file_sha256(path: &Path) -> String {
    hex_sha256(&fs::read(path).expect("read cached source"))
}
fn hex_sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
