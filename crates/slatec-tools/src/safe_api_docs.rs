//! Deterministic documentation indexes for the reviewed safe API inventories.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const PROFILE: &str = "ffi-profile-gnu-mingw-x86_64";

#[derive(Clone, Debug, Serialize)]
struct FunctionRecord {
    rust_path: String,
    fortran_routine: String,
    module: String,
    domain: String,
    precision: String,
    purpose: String,
    capability: String,
    feature: String,
    native_profile: String,
    documentation: String,
    example_file: String,
    example_case: String,
    inclusion_status: String,
}

#[derive(Clone, Debug, Serialize)]
struct ArgumentMap {
    rust: Option<String>,
    fortran: String,
    /// A key in the top-level `transformation_codes` legend.
    transformation: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct MappingRecord {
    rust_path: String,
    fortran_routine: String,
    program_unit_id: String,
    arguments: Vec<ArgumentMap>,
}

#[derive(Debug, Serialize)]
struct IndexFile<T> {
    schema_id: &'static str,
    schema_version: &'static str,
    semantic_version: &'static str,
    records: Vec<T>,
}

#[derive(Debug, Serialize)]
struct FortranArgumentIndex {
    schema_id: &'static str,
    schema_version: &'static str,
    semantic_version: &'static str,
    /// Stable, compact codes used by every argument record.
    transformation_codes: BTreeMap<&'static str, &'static str>,
    records: Vec<MappingRecord>,
}

#[derive(Debug)]
pub struct GenerationResult {
    pub function_count: usize,
    pub semantic_hash: String,
}

pub fn generate(output_dir: &Path) -> Result<GenerationResult> {
    let mut functions = collect_functions()?;
    functions.sort_by(|left, right| left.rust_path.cmp(&right.rust_path));
    reject_duplicates(&functions)?;
    validate_examples(&functions)?;

    let mappings = build_argument_maps(&functions)?;
    let coverage = functions
        .iter()
        .map(|record| {
            serde_json::json!({
                "rust_path": record.rust_path,
                "fortran_routine": record.fortran_routine,
                "doctest_present": source_has_doctest(&record.rust_path),
                "example_file": record.example_file,
                "example_case": record.example_case,
                "validation_file": validation_path_for(record),
                "validation_case": record.rust_path.rsplit("::").next().unwrap_or(&record.rust_path),
                "native_execution_status": native_status(record),
            })
        })
        .collect::<Vec<_>>();

    fs::create_dir_all(output_dir)?;
    let function_bytes = write_json(
        &output_dir.join("function-index.json"),
        &IndexFile {
            schema_id: "slatec.safe-api.function-index",
            schema_version: "1.0.0",
            semantic_version: "1",
            records: functions.clone(),
        },
    )?;
    let mapping_bytes = write_json(
        &output_dir.join("fortran-argument-map.json"),
        &FortranArgumentIndex {
            schema_id: "slatec.safe-api.fortran-argument-map",
            schema_version: "1.1.0",
            semantic_version: "1",
            transformation_codes: BTreeMap::from([
                (
                    "callback",
                    "Rust closure invoked through a contained callback trampoline",
                ),
                (
                    "inferred",
                    "inferred by a tightly packed convenience wrapper",
                ),
                (
                    "internal",
                    "allocated, initialized, or validated inside the safe wrapper",
                ),
                (
                    "owned",
                    "validated and copied, derived, or reconstructed in owned native storage",
                ),
                (
                    "pass",
                    "validated value or caller-owned slice passed through the reviewed raw ABI",
                ),
            ]),
            records: mappings,
        },
    )?;
    let coverage_bytes = write_json(
        &output_dir.join("example-coverage.json"),
        &IndexFile {
            schema_id: "slatec.safe-api.example-coverage",
            schema_version: "1.0.0",
            semantic_version: "1",
            records: coverage,
        },
    )?;

    let mut capabilities = BTreeMap::<String, usize>::new();
    for record in &functions {
        *capabilities.entry(record.capability.clone()).or_default() += 1;
    }
    let capability_bytes = write_json(
        &output_dir.join("capability-summary.json"),
        &serde_json::json!({
            "schema_id": "slatec.safe-api.capability-summary",
            "schema_version": "1.0.0",
            "semantic_version": "1",
            "function_count": functions.len(),
            "capabilities": capabilities,
            "api_no_std_compatible": true,
            "alloc_free_api_available": true,
            "native_backend_available": true,
            "native_runtime_validated": true,
            "bare_metal_execution_validated": false,
            "native_profile": PROFILE,
        }),
    )?;

    let markdown = render_markdown(&functions);
    fs::write(output_dir.join("function-index.md"), markdown.as_bytes())?;
    if output_dir == Path::new("generated/safe-api") {
        fs::create_dir_all("docs/api")?;
        fs::write("docs/api/function-index.md", markdown.as_bytes())?;
    }

    let mut semantic = Vec::new();
    semantic.extend_from_slice(&function_bytes);
    semantic.extend_from_slice(&mapping_bytes);
    semantic.extend_from_slice(&coverage_bytes);
    semantic.extend_from_slice(&capability_bytes);
    semantic.extend_from_slice(markdown.as_bytes());
    Ok(GenerationResult {
        function_count: functions.len(),
        semantic_hash: hash::bytes(&semantic),
    })
}

fn collect_functions() -> Result<Vec<FunctionRecord>> {
    let mut output = Vec::new();
    let level1 = read_json("generated/safe-api/wrapper-index.json")?;
    for wrapper in array(&level1, "wrappers")? {
        let path = string(wrapper, "safe_path")?;
        let routine = raw_routine(string(wrapper, "raw_symbol")?);
        let precision = string(wrapper, "precision")?;
        let family = string(wrapper, "family")?;
        output.push(record(
            path,
            &routine,
            "BLAS",
            precision,
            family,
            "core",
            "blas-level1",
        ));
        output.push(record(
            &format!("{path}_strided"),
            &routine,
            "BLAS",
            precision,
            family,
            "core",
            "blas-level1",
        ));
    }

    let matrix = read_json("generated/safe-api/matrix-wrapper-index.json")?;
    for wrapper in array(&matrix, "wrappers")? {
        let relative = string(wrapper, "safe_path")?;
        let path = format!("slatec::blas::{relative}");
        let routine = raw_routine(string(wrapper, "raw_symbol")?);
        let precision = string(wrapper, "precision")?;
        let family = string(wrapper, "family")?;
        let level = wrapper["level"]
            .as_u64()
            .ok_or_else(|| policy("matrix level"))?;
        let feature = format!("blas-level{level}");
        output.push(record(
            &path, &routine, "BLAS", precision, family, "core", &feature,
        ));
        if wrapper["contiguous_convenience"].as_bool() == Some(true) {
            output.push(record(
                &format!("{path}_contiguous"),
                &routine,
                "BLAS",
                precision,
                family,
                "core",
                &feature,
            ));
        }
    }

    collect_columnar(
        "generated/safe-api/special-function-wrapper-index.json",
        &mut output,
        |row, columns| {
            let family = column(row, columns, "family")?;
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_symbol")?
                    .trim_end_matches('_')
                    .to_ascii_uppercase()
                    .as_str(),
                if family == "polynomials" {
                    "polynomials"
                } else {
                    "special functions"
                },
                column(row, columns, "precision")?,
                family,
                "std",
                special_feature(family),
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/quadrature-wrapper-index.json",
        &mut output,
        |row, columns| {
            let path = column(row, columns, "safe_path")?;
            Ok(record(
                path,
                column(row, columns, "raw_routine")?,
                "quadrature",
                column(row, columns, "precision")?,
                column(row, columns, "quadrature_family")?,
                "std",
                quadrature_feature(path),
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/root-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "roots",
                column(row, columns, "precision")?,
                "bracketed scalar root",
                "std",
                "roots-scalar",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/nonlinear-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "nonlinear",
                column(row, columns, "precision")?,
                "finite-difference nonlinear system",
                "std",
                "nonlinear-easy",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/nonlinear-expert-wrapper-index.json",
        &mut output,
        |row, columns| {
            let mode = column(row, columns, "jacobian_policy")?;
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "nonlinear",
                column(row, columns, "precision")?,
                if mode == "finite_difference" {
                    "expert finite-difference nonlinear system"
                } else {
                    "expert analytic-Jacobian nonlinear system"
                },
                "std",
                "nonlinear-expert",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/nonlinear-jacobian-check-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "nonlinear",
                column(row, columns, "precision")?,
                "Jacobian consistency checking",
                "alloc",
                "nonlinear-jacobian-check",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/least-squares-easy-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "least squares",
                column(row, columns, "precision")?,
                "finite-difference nonlinear least squares",
                "std",
                "least-squares-nonlinear-easy",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/least-squares-expert-wrapper-index.json",
        &mut output,
        |row, columns| {
            let mode = column(row, columns, "jacobian_policy")?;
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "least squares",
                column(row, columns, "precision")?,
                if mode == "IOPT=1_forward_difference" {
                    "expert finite-difference nonlinear least squares"
                } else {
                    "expert analytic-Jacobian nonlinear least squares"
                },
                "std",
                "least-squares-nonlinear-expert",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/covariance-wrapper-index.json",
        &mut output,
        |row, columns| {
            let path = column(row, columns, "safe_path")?;
            let feature = if path.contains("from_expert_fit") {
                "least-squares-covariance + least-squares-nonlinear-expert"
            } else {
                "least-squares-covariance"
            };
            Ok(record(
                path,
                column(row, columns, "raw_routine")?,
                "least squares",
                column(row, columns, "precision")?,
                "nonlinear least-squares covariance estimation",
                "std",
                feature,
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/nonnegative-least-squares-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "linear least squares",
                column(row, columns, "precision")?,
                "equality-constrained nonnegative linear least squares",
                "std",
                "least-squares-linear-nonnegative",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/bounded-least-squares-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "linear least squares",
                column(row, columns, "precision")?,
                "bounded linear least squares",
                "std",
                "least-squares-linear-bounded",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/bounded-constrained-least-squares-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "linear least squares",
                column(row, columns, "precision")?,
                "bounded linear least squares with bounded constraint expressions",
                "std",
                "least-squares-linear-bounded-constrained",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/constrained-least-squares-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "linear least squares",
                column(row, columns, "precision")?,
                "equality and inequality constrained linear least squares",
                "std",
                "least-squares-linear-constrained",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/ode-sdrive-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "ordinary differential equations",
                column(row, columns, "precision")?,
                "explicit real initial-value problem y'=f(t,y)",
                "std",
                "ode-sdrive-expert",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/callback-driver-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                column(row, columns, "domain")?,
                column(row, columns, "precision")?,
                column(row, columns, "callback_shape")?,
                "std",
                column(row, columns, "feature")?,
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/dassl-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "differential-algebraic equations",
                column(row, columns, "precision")?,
                "real implicit index-1 initial-value problem G(t,y,y_prime)=0",
                "std",
                "dassl",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/lp-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "linear programming",
                column(row, columns, "precision")?,
                "sparse in-memory linear programming",
                "std",
                "optimization-linear-programming-in-memory",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/bspline-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "B-spline interpolation",
                column(row, columns, "precision")?,
                "exact B-spline interpolation construction, evaluation, derivatives, and definite integration",
                "std",
                "bspline",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/piecewise-polynomial-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "piecewise-polynomial interpolation",
                column(row, columns, "precision")?,
                column(row, columns, "mathematical_model")?,
                "std",
                "piecewise-polynomial",
            ))
        },
    )?;
    collect_columnar(
        "generated/safe-api/tabulated-data-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "tabulated data",
                column(row, columns, "precision")?,
                column(row, columns, "mathematical_operation")?,
                "std",
                "tabulated-data",
            ))
        },
    )?;
    for (path, routine, family) in [
        (
            "slatec::fftpack::RealFftPlan::new",
            "RFFTI",
            "periodic real FFT plan initialization",
        ),
        (
            "slatec::fftpack::RealFftPlan::forward",
            "RFFTF",
            "periodic real FFT forward transform",
        ),
        (
            "slatec::fftpack::RealFftPlan::backward",
            "RFFTB",
            "periodic real FFT backward transform",
        ),
        (
            "slatec::fftpack::EasyRealFftPlan::new",
            "EZFFTI",
            "easy real Fourier plan initialization",
        ),
        (
            "slatec::fftpack::EasyRealFftPlan::forward",
            "EZFFTF",
            "easy real Fourier forward transform",
        ),
        (
            "slatec::fftpack::EasyRealFftPlan::backward",
            "EZFFTB",
            "easy real Fourier synthesis",
        ),
        (
            "slatec::fftpack::SineTransformPlan::new",
            "SINTI",
            "full sine transform plan initialization",
        ),
        (
            "slatec::fftpack::SineTransformPlan::transform",
            "SINT",
            "full sine transform",
        ),
        (
            "slatec::fftpack::CosineTransformPlan::new",
            "COSTI",
            "full cosine transform plan initialization",
        ),
        (
            "slatec::fftpack::CosineTransformPlan::transform",
            "COST",
            "full cosine transform",
        ),
        (
            "slatec::fftpack::QuarterWaveSinePlan::new",
            "SINQI",
            "quarter-wave sine plan initialization",
        ),
        (
            "slatec::fftpack::QuarterWaveSinePlan::forward",
            "SINQF",
            "quarter-wave sine forward transform",
        ),
        (
            "slatec::fftpack::QuarterWaveSinePlan::backward",
            "SINQB",
            "quarter-wave sine backward transform",
        ),
        (
            "slatec::fftpack::QuarterWaveCosinePlan::new",
            "COSQI",
            "quarter-wave cosine plan initialization",
        ),
        (
            "slatec::fftpack::QuarterWaveCosinePlan::forward",
            "COSQF",
            "quarter-wave cosine forward transform",
        ),
        (
            "slatec::fftpack::QuarterWaveCosinePlan::backward",
            "COSQB",
            "quarter-wave cosine backward transform",
        ),
    ] {
        output.push(record(
            path,
            routine,
            "real FFTPACK",
            "f32",
            family,
            "std",
            "fftpack-real",
        ));
    }
    collect_columnar(
        "generated/safe-api/fftpack-complex-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(record(
                column(row, columns, "safe_path")?,
                column(row, columns, "raw_routine")?,
                "complex FFTPACK",
                column(row, columns, "public_scalar")?,
                column(row, columns, "operation")?,
                "std",
                "fftpack-complex",
            ))
        },
    )?;
    for (path, routine, precision, purpose) in [
        (
            "slatec::linear_algebra::banded::BandMatrixRef::factorize_with_condition_estimate",
            "SGBCO",
            "f32/f64",
            "general-band LU factorization with reciprocal 1-norm condition estimate",
        ),
        (
            "slatec::linear_algebra::banded::BandLu32::scaled_determinant",
            "SGBDI",
            "f32",
            "scaled base-ten determinant metadata from general-band LU factors",
        ),
        (
            "slatec::linear_algebra::banded::BandLu64::scaled_determinant",
            "DGBDI",
            "f64",
            "scaled base-ten determinant metadata from general-band LU factors",
        ),
    ] {
        output.push(record(
            path,
            routine,
            "banded linear systems",
            precision,
            purpose,
            "std",
            "banded-linear-systems",
        ));
    }
    for (path, routine, family) in [
        (
            "slatec::pchip::PiecewiseCubicHermite::monotone",
            "PCHIM",
            "monotonicity-preserving Hermite derivative construction",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::monotone_with_conditions",
            "PCHIC",
            "controlled monotone Hermite derivative construction",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::spline",
            "PCHSP",
            "cubic-spline Hermite derivative construction",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::evaluate_into",
            "PCHFE",
            "piecewise-cubic Hermite value evaluation",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::evaluate_with_derivatives_into",
            "PCHFD",
            "piecewise-cubic Hermite value and derivative evaluation",
        ),
        (
            "slatec::pchip::PiecewiseCubicHermite::integrate",
            "PCHIA",
            "piecewise-cubic Hermite definite integration",
        ),
    ] {
        output.push(record(
            path,
            routine,
            "piecewise cubic Hermite interpolation",
            "f32/f64",
            family,
            "std",
            "pchip",
        ));
    }
    collect_columnar(
        "generated/safe-api/fishpack-cartesian-2d-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(FunctionRecord {
                rust_path: column(row, columns, "safe_path")?.to_owned(),
                fortran_routine: column(row, columns, "raw_routine")?.to_owned(),
                module: "slatec::differential_equations::pde".to_owned(),
                domain: "Cartesian FISHPACK PDE".to_owned(),
                precision: column(row, columns, "precision")?.to_owned(),
                purpose: "checked owned Cartesian Poisson/Helmholtz finite-difference solve"
                    .to_owned(),
                capability: "std".to_owned(),
                feature: "fishpack-cartesian-2d".to_owned(),
                native_profile: PROFILE.to_owned(),
                documentation: "https://docs.rs/slatec/latest/slatec/differential_equations/pde/struct.CartesianHelmholtz2d.html#method.solve".to_owned(),
                example_file: "examples/fishpack_cartesian_2d.rs".to_owned(),
                example_case: "mixed Cartesian Poisson problem".to_owned(),
                inclusion_status: "safe_api_available".to_owned(),
            })
        },
    )?;
    collect_columnar(
        "generated/safe-api/fishpack-pois3d-wrapper-index.json",
        &mut output,
        |row, columns| {
            Ok(FunctionRecord {
                rust_path: column(row, columns, "safe_path")?.to_owned(),
                fortran_routine: column(row, columns, "raw_routine")?.to_owned(),
                module: "slatec::differential_equations::pde".to_owned(),
                domain: "Structured FISHPACK PDE".to_owned(),
                precision: column(row, columns, "precision")?.to_owned(),
                purpose: "checked owned structured three-dimensional block-tridiagonal solve"
                    .to_owned(),
                capability: "std".to_owned(),
                feature: "fishpack-pois3d".to_owned(),
                native_profile: PROFILE.to_owned(),
                documentation: "https://docs.rs/slatec/latest/slatec/differential_equations/pde/struct.Pois3dProblem.html#method.solve".to_owned(),
                example_file: "examples/fishpack_pois3d.rs".to_owned(),
                example_case: "manufactured structured cyclic POIS3D system".to_owned(),
                inclusion_status: "safe_api_available".to_owned(),
            })
        },
    )?;
    Ok(output)
}

fn special_feature(family: &str) -> &'static str {
    match family {
        "elementary" => "special-elementary",
        "gamma" => "special-gamma",
        "beta" => "special-beta",
        "error_functions" => "special-error",
        "airy" => "special-airy",
        "bessel" => "special-bessel",
        "integrals" => "special-integrals",
        "scalar-expanded-integrals" | "elliptic" => "special-scalar-expanded",
        "polynomials" => "special-polynomials",
        _ => "special",
    }
}

fn quadrature_feature(path: &str) -> &'static str {
    if path.contains("breakpoint") {
        "quadrature-breakpoints"
    } else if path.contains("weighted_endpoint") {
        "quadrature-weighted"
    } else if path.contains("oscillatory") {
        "quadrature-oscillatory"
    } else if path.contains("fourier") {
        "quadrature-fourier"
    } else if path.contains("non_adaptive") || path.contains("nc79") {
        "quadrature-nonadaptive"
    } else {
        "quadrature-basic"
    }
}

fn collect_columnar(
    path: &str,
    output: &mut Vec<FunctionRecord>,
    make: impl Fn(&[Value], &[String]) -> Result<FunctionRecord>,
) -> Result<()> {
    let value = read_json(path)?;
    let columns = value["columns"]
        .as_array()
        .ok_or_else(|| policy("columnar columns"))?
        .iter()
        .map(|value| value.as_str().unwrap_or_default().to_owned())
        .collect::<Vec<_>>();
    for row in value["records"]
        .as_array()
        .ok_or_else(|| policy("columnar records"))?
    {
        output.push(make(
            row.as_array().ok_or_else(|| policy("columnar row"))?,
            &columns,
        )?);
    }
    Ok(())
}

fn record(
    path: &str,
    routine: &str,
    domain: &str,
    precision: &str,
    family: &str,
    capability: &str,
    feature: &str,
) -> FunctionRecord {
    let module = path.rsplit_once("::").map_or(path, |(module, _)| module);
    let name = path.rsplit("::").next().unwrap_or(path);
    let example_file = match domain {
        "BLAS" => format!(
            "examples/blas/level{}.rs",
            if path.contains("level1") {
                "1"
            } else if path.contains("level2") {
                "2"
            } else {
                "3"
            }
        ),
        "quadrature" if path.contains("integrate_piecewise_polynomial") => {
            "examples/quadrature/piecewise_polynomial.rs".to_owned()
        }
        "quadrature" if path.contains("integrate_tabulated") => {
            "examples/interpolation/tabulated_data.rs".to_owned()
        }
        "quadrature" => "examples/quadrature/families.rs".to_owned(),
        "roots" => "examples/roots/scalar.rs".to_owned(),
        "nonlinear" if path.contains("solve_scalar_equations") => {
            "examples/nonlinear/scalar_equations.rs".to_owned()
        }
        "nonlinear" if path.contains("check_jacobian") => {
            "examples/nonlinear/check_jacobian.rs".to_owned()
        }
        "nonlinear" if path.contains("with_jacobian") => {
            "examples/nonlinear/solve_system_with_jacobian.rs".to_owned()
        }
        "nonlinear" if path.contains("solve_system_expert") => {
            "examples/nonlinear/solve_system_expert.rs".to_owned()
        }
        "nonlinear" if precision == "f32" => "examples/nonlinear/solve_system_f32.rs".to_owned(),
        "nonlinear" => "examples/nonlinear/solve_system.rs".to_owned(),
        "linear least squares"
            if path.contains("bounded_constrained_least_squares") && precision == "f32" =>
        {
            "examples/least_squares/active_bound_and_constraint.rs".to_owned()
        }
        "linear least squares" if path.contains("bounded_constrained_least_squares") => {
            "examples/least_squares/bounded_constrained_fit.rs".to_owned()
        }
        "linear least squares" if path.contains("bounded_least_squares") && precision == "f32" => {
            "examples/least_squares/mixed_bounds.rs".to_owned()
        }
        "linear least squares" if path.contains("bounded_least_squares") => {
            "examples/least_squares/bounded_fit.rs".to_owned()
        }
        "linear least squares"
            if path.contains("constrained_least_squares") && precision == "f32" =>
        {
            "examples/least_squares/inequality_constrained_fit.rs".to_owned()
        }
        "linear least squares" if path.contains("constrained_least_squares") => {
            "examples/least_squares/equality_constrained_fit.rs".to_owned()
        }
        "linear least squares" if precision == "f32" => {
            "examples/least_squares/mixed_nonnegative_fit.rs".to_owned()
        }
        "linear least squares" => "examples/least_squares/nonnegative_fit.rs".to_owned(),
        "least squares" if path.contains("covariance") && path.contains("rank") => {
            "examples/least_squares/covariance_rank_deficient.rs".to_owned()
        }
        "least squares" if path.contains("covariance") && path.contains("finite_difference") => {
            "examples/least_squares/covariance_linear_fit.rs".to_owned()
        }
        "least squares" if path.contains("covariance") => {
            "examples/least_squares/covariance_nonlinear_fit.rs".to_owned()
        }
        "least squares" if path.contains("with_jacobian") && precision == "f32" => {
            "examples/least_squares/expert_analytic_jacobian_f32.rs".to_owned()
        }
        "least squares" if path.contains("with_jacobian") => {
            "examples/least_squares/expert_analytic_jacobian.rs".to_owned()
        }
        "least squares" if path.contains("expert") => {
            "examples/least_squares/expert_finite_difference.rs".to_owned()
        }
        "least squares" if precision == "f32" => {
            "examples/least_squares/linear_fit_f32.rs".to_owned()
        }
        "least squares" if path.ends_with("least_squares") => {
            "examples/least_squares/linear_fit.rs".to_owned()
        }
        "least squares" => "examples/least_squares/exponential_fit.rs".to_owned(),
        "ordinary differential equations" if path.contains("Driv") => {
            "examples/ode/callback_drivers.rs".to_owned()
        }
        "ordinary differential equations" if precision == "f32" => {
            "examples/ode/harmonic_oscillator.rs".to_owned()
        }
        "ordinary differential equations" => "examples/ode/exponential_decay.rs".to_owned(),
        "differential-algebraic equations" if path.contains("f32") => {
            "examples/dassl/algebraic_variable.rs".to_owned()
        }
        "differential-algebraic equations" => "examples/dassl/index1_constraint.rs".to_owned(),
        "linear programming" => "examples/linear_programming/basic.rs".to_owned(),
        "real FFTPACK" if path.contains("SineTransform") => {
            "examples/fftpack/sine_transform.rs".to_owned()
        }
        "real FFTPACK" if path.contains("CosineTransform") => {
            "examples/fftpack/cosine_transform.rs".to_owned()
        }
        "real FFTPACK" if path.contains("QuarterWave") => {
            "examples/fftpack/quarter_wave.rs".to_owned()
        }
        "real FFTPACK" => "examples/fftpack/real_fft.rs".to_owned(),
        "complex FFTPACK" if path.contains("forward") => {
            "examples/fftpack/complex_spectrum.rs".to_owned()
        }
        "complex FFTPACK" => "examples/fftpack/complex_round_trip.rs".to_owned(),
        "banded linear systems" => "examples/banded/factor_and_solve.rs".to_owned(),
        "piecewise cubic Hermite interpolation" if path.contains("integrate") => {
            "examples/pchip/integrate.rs".to_owned()
        }
        "piecewise cubic Hermite interpolation" if path.contains("derivative") => {
            "examples/pchip/evaluate_derivative.rs".to_owned()
        }
        "piecewise cubic Hermite interpolation" if path.contains("spline") => {
            "examples/pchip/custom_derivatives.rs".to_owned()
        }
        "piecewise cubic Hermite interpolation" => "examples/pchip/monotone.rs".to_owned(),
        "B-spline interpolation" if path.contains("interpolate_with_knots") => {
            "examples/interpolation/bspline_interpolate.rs".to_owned()
        }
        "B-spline interpolation" if path.contains("integrate") => {
            "examples/bspline/integrate.rs".to_owned()
        }
        "B-spline interpolation" if path.contains("derivative") => {
            "examples/bspline/derivatives.rs".to_owned()
        }
        "B-spline interpolation" => "examples/bspline/from_parts.rs".to_owned(),
        "piecewise-polynomial interpolation" if path.contains("to_piecewise_polynomial") => {
            "examples/piecewise_polynomial/from_bspline.rs".to_owned()
        }
        "piecewise-polynomial interpolation" if path.contains("integrate") => {
            "examples/piecewise_polynomial/integrate.rs".to_owned()
        }
        "piecewise-polynomial interpolation"
            if path.contains("derivative") || path.contains("into") =>
        {
            "examples/piecewise_polynomial/evaluate_derivatives.rs".to_owned()
        }
        "piecewise-polynomial interpolation" => {
            "examples/piecewise_polynomial/from_pieces.rs".to_owned()
        }
        "tabulated data" => "examples/interpolation/tabulated_data.rs".to_owned(),
        "special functions" if path.contains("scalar_expanded") && path.contains("carlson_") => {
            "examples/special/elliptic.rs".to_owned()
        }
        "special functions" if path.contains("scalar_expanded") => {
            "examples/special/integrals.rs".to_owned()
        }
        "polynomials" => "examples/special/functions.rs".to_owned(),
        _ => "examples/special/functions.rs".to_owned(),
    };
    FunctionRecord {
        rust_path: path.to_owned(),
        fortran_routine: routine.to_ascii_uppercase(),
        module: module.to_owned(),
        domain: domain.to_owned(),
        precision: precision.to_owned(),
        purpose: if domain == "ordinary differential equations" {
            family.to_owned()
        } else {
            purpose(family).to_owned()
        },
        capability: capability.to_owned(),
        feature: feature.to_owned(),
        native_profile: PROFILE.to_owned(),
        documentation: documentation_url(module, name),
        example_file,
        example_case: family.to_owned(),
        inclusion_status: "safe_api_available".to_owned(),
    }
}

fn documentation_url(module: &str, name: &str) -> String {
    let module = module.split("::<").next().unwrap_or(module);
    if let Some((parent, type_name)) = module.rsplit_once("::") {
        if type_name
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_uppercase())
        {
            return format!(
                "https://docs.rs/slatec/latest/slatec/{}/struct.{type_name}.html#method.{name}",
                parent.trim_start_matches("slatec::").replace("::", "/")
            );
        }
    }
    format!(
        "https://docs.rs/slatec/latest/slatec/{}/fn.{name}.html",
        module.trim_start_matches("slatec::").replace("::", "/")
    )
}

fn build_argument_maps(functions: &[FunctionRecord]) -> Result<Vec<MappingRecord>> {
    let interfaces = read_json("generated/ffi/interface-inventory.json")?;
    let columns = string_columns(&interfaces)?;
    let mut by_name = BTreeMap::<String, (String, Vec<String>)>::new();
    for row in rows(&interfaces)? {
        let name = column(row, &columns, "normalized_name")?.to_owned();
        let unit = column(row, &columns, "program_unit_id")?.to_owned();
        let ids = row[index(&columns, "argument_ids")?]
            .as_array()
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default();
        by_name.insert(name, (unit, ids));
    }
    let arguments = read_json("generated/ffi-inventory/argument-index.json")?;
    let argument_columns = string_columns(&arguments)?;
    let mut by_id = BTreeMap::<String, String>::new();
    for row in rows(&arguments)? {
        by_id.insert(
            column(row, &argument_columns, "id")?.to_owned(),
            column(row, &argument_columns, "normalized_name")?.to_owned(),
        );
    }
    functions
        .iter()
        .flat_map(|function| {
            function
                .fortran_routine
                .split('/')
                .map(move |routine| (function, routine))
        })
        .map(|(function, routine)| {
            let (program_unit_id, ids) = by_name
                .get(routine)
                .ok_or_else(|| policy(&format!("missing interface for {routine}")))?;
            let arguments = ids
                .iter()
                .filter_map(|id| by_id.get(id))
                .map(|name| argument_map(function, name))
                .collect();
            Ok(MappingRecord {
                rust_path: function.rust_path.clone(),
                fortran_routine: routine.to_owned(),
                program_unit_id: program_unit_id.clone(),
                arguments,
            })
        })
        .collect()
}

fn argument_map(function: &FunctionRecord, name: &str) -> ArgumentMap {
    let upper = name.to_ascii_uppercase();
    let fishpack = function.domain == "Cartesian FISHPACK PDE";
    let pois3d = function.domain == "Structured FISHPACK PDE";
    let internal = [
        "WORK", "IWORK", "LENW", "LAST", "NEVAL", "IER", "IFLAG", "RESULT", "ABSERR", "ALIST",
        "BLIST", "RLIST", "ELIST", "JAC", "IOPT", "NPRINT", "WA", "LWA", "FJAC", "LDFJAC", "R",
        "LR", "QTF", "WA1", "WA2", "WA3", "WA4", "XP", "FVECP", "MODE", "IPVT", "ERR", "RW", "IW",
        "WS", "WSAVE", "IP", "LW", "LIW", "DATTRV", "DUALS", "IBASIS",
    ];
    let inferred = function.rust_path.ends_with("_contiguous")
        && ["LDA", "LDB", "LDC", "INCX", "INCY"].contains(&upper.as_str());
    let analytic_jacobian = function.rust_path.contains("with_jacobian");
    let jacobian_check = function.rust_path.contains("check_jacobian");
    let bspline_constructor = function.domain == "B-spline interpolation"
        && function.rust_path.contains("interpolate_with_knots");
    let tabulated = function.domain == "tabulated data";
    let internal_argument = (internal.contains(&upper.as_str())
        || (fishpack
            && matches!(
                upper.as_str(),
                "MBDCND" | "NBDCND" | "IDIMF" | "IERROR" | "W"
            ))
        || (pois3d
            && matches!(
                upper.as_str(),
                "LPEROD" | "MPEROD" | "NPEROD" | "LDIMF" | "MDIMF" | "IERROR" | "W"
            ))
        || (bspline_constructor && matches!(upper.as_str(), "N" | "BCOEF" | "Q"))
        || (function.domain == "complex FFTPACK"
            && matches!(upper.as_str(), "CH" | "WA" | "IFAC"))
        || (tabulated && matches!(upper.as_str(), "C" | "D" | "YFIT" | "YP" | "ANS" | "IERR"))
        || (jacobian_check && upper == "FVEC"))
        && !(matches!(function.domain.as_str(), "nonlinear" | "least squares") && upper == "INFO")
        && !(function.domain == "linear least squares" && upper == "MODE")
        && !(analytic_jacobian && upper == "JAC")
        && !(jacobian_check && matches!(upper.as_str(), "FJAC" | "ERR"))
        && !(function.domain == "linear programming"
            && matches!(upper.as_str(), "INFO" | "DUALS" | "IBASIS" | "IWORK"));
    let rust = if internal_argument || inferred {
        None
    } else {
        Some(match upper.as_str() {
            "X" if tabulated => {
                "checked owned sample abscissas or the interpolant's retained abscissas"
                    .to_owned()
            }
            "Y" if tabulated => "checked owned sample values".to_owned(),
            "N" if tabulated => "data.sample_count".to_owned(),
            "NDER" if tabulated => "derivative_count".to_owned(),
            "XX" if tabulated => "finite evaluation or Taylor-expansion point".to_owned(),
            "XLO" if tabulated => "interval start".to_owned(),
            "XUP" if tabulated => "interval end".to_owned(),
            "L" if pois3d => "problem.rhs.nx".to_owned(),
            "M" if pois3d => "problem.rhs.ny".to_owned(),
            "N" if pois3d => "problem.rhs.nz".to_owned(),
            "C1" if pois3d => "problem.c1".to_owned(),
            "C2" if pois3d => "problem.c2".to_owned(),
            "A" | "B" | "C" if pois3d => {
                "private owned buffers derived from ThirdAxisOperator".to_owned()
            }
            "F" if pois3d => "owned Grid3 RHS overwritten with solution".to_owned(),
            "A" if fishpack => "problem.x.lower".to_owned(),
            "B" if fishpack => "problem.x.upper".to_owned(),
            "M" if fishpack => "problem.x.intervals".to_owned(),
            "C" if fishpack => "problem.y.lower".to_owned(),
            "D" if fishpack => "problem.y.upper".to_owned(),
            "N" if fishpack => "problem.y.intervals".to_owned(),
            "BDA" | "BDB" if fishpack => {
                "private x-edge derivative buffer from AxisBoundary".to_owned()
            }
            "BDC" | "BDD" if fishpack => {
                "private y-edge derivative buffer from AxisBoundary".to_owned()
            }
            "ELMBDA" if fishpack => "problem.coefficient".to_owned(),
            "F" if fishpack => "owned Grid2 RHS overwritten with solution".to_owned(),
            "PERTRB" if fishpack => "result.perturbation".to_owned(),
            "USRMAT" | "DUSRMT" if function.domain == "linear programming" => {
                "problem.matrix delivered by an internal one-based sparse-column trampoline"
                    .to_owned()
            }
            "MRELAS" if function.domain == "linear programming" => {
                "problem.matrix.rows".to_owned()
            }
            "NVARS" if function.domain == "linear programming" => {
                "problem.matrix.columns".to_owned()
            }
            "COSTS" if function.domain == "linear programming" => {
                "problem.objective".to_owned()
            }
            "PRGOPT" if function.domain == "linear programming" => {
                "internal reviewed option list: printing, continuation, save/restore, dense callback, and key 54 disabled; exact LAMAT/LBM plus typed iteration/tolerance/pricing controls"
                    .to_owned()
            }
            "BL" | "BU" | "IND" if function.domain == "linear programming" => {
                "problem.variable_bounds followed by problem.row_bounds in native typed bound encoding"
                    .to_owned()
            }
            "INFO" if function.domain == "linear programming" => "result.status".to_owned(),
            "PRIMAL" if function.domain == "linear programming" => {
                "optimal result.solution variables plus independently recomputed row activities; labelled progress only for INFO=-25"
                    .to_owned()
            }
            "DUALS" if function.domain == "linear programming" => {
                "optimal result.solution.dual: row multipliers y then reduced costs c-A^T*y"
                    .to_owned()
            }
            "IBASIS" if function.domain == "linear programming" => {
                "optimal result.solution.basis.basic, validated one-based native identifiers"
                    .to_owned()
            }
            "IWORK" if function.domain == "linear programming" => {
                "private checked workspace; optimal IBB segment decodes typed basis positions"
                    .to_owned()
            }
            "ABD" if function.domain == "banded linear systems" => {
                "compact input copied into private expanded LINPACK band storage, or immutable private LU factors".to_owned()
            }
            "LDA" if function.domain == "banded linear systems" => {
                "private expanded leading dimension 2*lower+upper+1".to_owned()
            }
            "N" if function.domain == "banded linear systems" => "matrix dimension".to_owned(),
            "ML" if function.domain == "banded linear systems" => {
                "matrix.lower_bandwidth".to_owned()
            }
            "MU" if function.domain == "banded linear systems" => {
                "matrix.upper_bandwidth".to_owned()
            }
            "RCOND" if function.domain == "banded linear systems" => {
                "ReciprocalCondition::value".to_owned()
            }
            "DET" if function.domain == "banded linear systems" => {
                "ScaledDeterminant mantissa and checked decimal exponent".to_owned()
            }
            "X" if bspline_constructor => "nodes".to_owned(),
            "Y" if bspline_constructor => "values".to_owned(),
            "T" if bspline_constructor => {
                "complete knot sequence copied into owned native storage".to_owned()
            }
            "K" if bspline_constructor => "order".to_owned(),
            "N" if function.domain == "real FFTPACK" => "plan.length".to_owned(),
            "N" if function.domain == "complex FFTPACK" => "plan.length".to_owned(),
            "R" | "X" if function.domain == "real FFTPACK" => {
                "caller contiguous transform buffer".to_owned()
            }
            "C" if function.domain == "complex FFTPACK" => {
                "caller contiguous Complex32 transform buffer, viewed as checked re/im f32 words"
                    .to_owned()
            }
            "AZERO" if function.domain == "real FFTPACK" => "EasyRealSpectrum::mean".to_owned(),
            "A" if function.domain == "real FFTPACK" => "EasyRealSpectrum::cosine".to_owned(),
            "B" if function.domain == "real FFTPACK" => "EasyRealSpectrum::sine".to_owned(),
            "F" | "FCN" => "function".to_owned(),
            "M" if jacobian_check => "point.len".to_owned(),
            "N" if jacobian_check => "point.len".to_owned(),
            "M" if function.domain == "least squares" => "residual_count".to_owned(),
            "N" if matches!(function.domain.as_str(), "nonlinear" | "least squares") => {
                "initial.len".to_owned()
            }
            "X" if jacobian_check => "point".to_owned(),
            "X" if matches!(function.domain.as_str(), "nonlinear" | "least squares") => {
                "initial".to_owned()
            }
            "FVEC" if function.domain == "nonlinear" => "result.residual".to_owned(),
            "FVEC" if function.domain == "least squares" => "result.residuals".to_owned(),
            "JAC" if analytic_jacobian => "jacobian".to_owned(),
            "FJAC" if jacobian_check => "jacobian".to_owned(),
            "ERR" if jacobian_check => "result.scores".to_owned(),
            "TOL" if matches!(function.domain.as_str(), "nonlinear" | "least squares") => {
                "options.tolerance".to_owned()
            }
            "FTOL" if function.domain == "least squares" => "options.function_tolerance".to_owned(),
            "XTOL" if function.domain == "least squares" => {
                "options.parameter_tolerance".to_owned()
            }
            "GTOL" if function.domain == "least squares" => "options.gradient_tolerance".to_owned(),
            "MAXFEV" if function.domain == "least squares" => {
                "options.maximum_function_evaluations".to_owned()
            }
            "EPSFCN" if function.domain == "least squares" => {
                "options.finite_difference_step".to_owned()
            }
            "DIAG" if function.domain == "least squares" => "options.scaling".to_owned(),
            "FACTOR" if function.domain == "least squares" => {
                "options.step_bound_factor".to_owned()
            }
            "NFEV" if function.domain == "least squares" => {
                "result.function_evaluations".to_owned()
            }
            "NJEV" if function.domain == "least squares" => {
                "result.jacobian_evaluations".to_owned()
            }
            "XTOL" if function.domain == "nonlinear" => "options.tolerance".to_owned(),
            "MAXFEV" if function.domain == "nonlinear" => {
                "options.maximum_function_evaluations".to_owned()
            }
            "ML" if function.domain == "nonlinear" => {
                "options.jacobian_structure.lower_bandwidth".to_owned()
            }
            "MU" if function.domain == "nonlinear" => {
                "options.jacobian_structure.upper_bandwidth".to_owned()
            }
            "EPSFCN" if function.domain == "nonlinear" => {
                "options.finite_difference_step".to_owned()
            }
            "DIAG" if function.domain == "nonlinear" => "options.scaling".to_owned(),
            "FACTOR" if function.domain == "nonlinear" => "options.step_bound_factor".to_owned(),
            "NFEV" if function.domain == "nonlinear" => "result.function_evaluations".to_owned(),
            "NJEV" if function.domain == "nonlinear" => "result.jacobian_evaluations".to_owned(),
            "INFO" if function.domain == "nonlinear" => "result.status".to_owned(),
            "INFO" if function.domain == "least squares" => "result.status".to_owned(),
            "W" if function.domain == "linear least squares" => {
                "problem matrices and right-hand sides copied into owned augmented storage"
                    .to_owned()
            }
            "MDW" if function.domain == "linear least squares" => {
                "internal augmented leading dimension".to_owned()
            }
            "MCON" if function.rust_path.contains("bounded_constrained_least_squares") => {
                "problem.constraints.matrix.rows".to_owned()
            }
            "MROWS" if function.rust_path.contains("bounded_constrained_least_squares") => {
                "problem.objective_matrix.rows".to_owned()
            }
            "NCOLS" if function.rust_path.contains("bounded_constrained_least_squares") => {
                "problem.variable_bounds.len".to_owned()
            }
            "BL" | "BU" | "IND"
                if function.rust_path.contains("bounded_constrained_least_squares") =>
            {
                "problem.variable_bounds and problem.constraints.bounds (typed closed-bound encoding)".to_owned()
            }
            "IOPT" if function.rust_path.contains("bounded_constrained_least_squares") => {
                "BoundedConstrainedLeastSquaresOptions encoded as the reviewed default native filter".to_owned()
            }
            "RNORMC" if function.rust_path.contains("bounded_constrained_least_squares") => {
                "result.constraint_residual_norm and result.constraint_feasibility".to_owned()
            }
            "RNORM" if function.rust_path.contains("bounded_constrained_least_squares") => {
                "result.objective_residual_norm".to_owned()
            }
            "MROWS" if function.rust_path.contains("bounded_least_squares") => {
                "problem.matrix.rows".to_owned()
            }
            "NCOLS" if function.rust_path.contains("bounded_least_squares") => {
                "problem.bounds.len".to_owned()
            }
            "BL" | "BU" | "IND" if function.rust_path.contains("bounded_least_squares") => {
                "problem.bounds (typed closed-bound encoding)".to_owned()
            }
            "ME" if function.domain == "linear least squares" => {
                if function.rust_path.contains("constrained_least_squares") {
                    "problem.equalities matrix row count".to_owned()
                } else {
                    "problem.equality_matrix.rows".to_owned()
                }
            }
            "MA" if function.domain == "linear least squares" => {
                if function.rust_path.contains("constrained_least_squares") {
                    "problem.objective_matrix.rows".to_owned()
                } else {
                    "problem.least_squares_matrix.rows".to_owned()
                }
            }
            "MG" if function.rust_path.contains("constrained_least_squares") => {
                "problem.inequalities row count".to_owned()
            }
            "N" if function.domain == "linear least squares" => {
                if function.rust_path.contains("constrained_least_squares") {
                    "problem.objective_matrix.cols".to_owned()
                } else {
                    "problem.variable_constraints.len".to_owned()
                }
            }
            "L" if function.domain == "linear least squares" => {
                "internally permuted free-variable count".to_owned()
            }
            "PRGOPT" if function.domain == "linear least squares" => {
                if function.rust_path.contains("constrained_least_squares") {
                    "options.rank_tolerance encoded in internal PRGOPT list".to_owned()
                } else {
                    "internal nominal option sentinel".to_owned()
                }
            }
            "X" if function.domain == "linear least squares" => "result.solution".to_owned(),
            "RNORM" if function.domain == "linear least squares" => {
                "result.residual_norm".to_owned()
            }
            "RNORME" if function.rust_path.contains("constrained_least_squares") => {
                "result.equality_residual_norm".to_owned()
            }
            "RNORML" if function.rust_path.contains("constrained_least_squares") => {
                "result.objective_residual_norm".to_owned()
            }
            "MODE" if function.domain == "linear least squares" => "result.status".to_owned(),
            "EPSABS" => "options.absolute_tolerance".to_owned(),
            "EPSREL" | "RE" => "options.relative_tolerance".to_owned(),
            "AE" => "options.absolute_tolerance".to_owned(),
            "LIMIT" => "options.limit".to_owned(),
            "B" if function.domain == "roots" => "bracket.lower".to_owned(),
            "C" if function.domain == "roots" => "bracket.upper".to_owned(),
            "R" if function.domain == "roots" => "options.initial_guess".to_owned(),
            "CS" => "coefficients".to_owned(),
            _ => name.to_ascii_lowercase(),
        })
    };
    let callback_argument = upper == "F"
        || (matches!(function.domain.as_str(), "nonlinear" | "least squares") && upper == "FCN")
        || (function.domain == "linear programming"
            && matches!(upper.as_str(), "USRMAT" | "DUSRMT"))
        || (analytic_jacobian && upper == "JAC")
        || (jacobian_check && upper == "FJAC");
    let transformation = if inferred {
        "inferred"
    } else if internal_argument {
        "internal"
    } else if bspline_constructor && upper == "T" {
        "owned"
    } else if callback_argument {
        "callback"
    } else if matches!(
        function.domain.as_str(),
        "linear least squares"
            | "linear programming"
            | "real FFTPACK"
            | "complex FFTPACK"
            | "Cartesian FISHPACK PDE"
            | "banded linear systems"
            | "piecewise cubic Hermite interpolation"
            | "piecewise-polynomial interpolation"
    ) {
        "owned"
    } else {
        "pass"
    };
    ArgumentMap {
        rust,
        fortran: upper,
        transformation,
    }
}

fn render_markdown(functions: &[FunctionRecord]) -> String {
    let mut text = String::from(
        "# Safe function index\n\nThis index is generated from the reviewed safe-API inventories. The Rust surface is `no_std`; individual functions are classified as core-only or hosted `std`. The only validated native backend is GNU Fortran `x86_64-w64-mingw32`; this is not a bare-metal support claim.\n\n## Alphabetical Rust API\n\n| Rust API | Original Fortran | Domain | Precision | Purpose | Requirement | Feature | Example |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for item in functions {
        text.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} | `{}` | `{}` | [{}](../../{}) |\n",
            item.rust_path,
            item.fortran_routine,
            item.domain,
            item.precision,
            item.purpose,
            item.capability,
            item.feature,
            item.example_case,
            item.example_file
        ));
    }
    text.push_str("\n## Original Fortran routine index\n\n");
    let mut fortran = functions.to_vec();
    fortran.sort_by(|a, b| {
        a.fortran_routine
            .cmp(&b.fortran_routine)
            .then(a.rust_path.cmp(&b.rust_path))
    });
    for item in fortran {
        text.push_str(&format!(
            "- `{}` -> `{}`\n",
            item.fortran_routine, item.rust_path
        ));
    }
    text.push_str("\n## Domain index\n");
    for domain in [
        "BLAS",
        "special functions",
        "polynomials",
        "quadrature",
        "roots",
        "nonlinear",
        "least squares",
        "linear least squares",
        "ordinary differential equations",
        "linear programming",
        "real FFTPACK",
        "banded linear systems",
        "piecewise cubic Hermite interpolation",
        "piecewise-polynomial interpolation",
        "tabulated data",
        "Cartesian FISHPACK PDE",
    ] {
        text.push_str(&format!("\n### {domain}\n\n"));
        for item in functions.iter().filter(|item| item.domain == domain) {
            text.push_str(&format!("- `{}` — {}\n", item.rust_path, item.purpose));
        }
    }
    text.push_str("\n## Capability index\n\n### Core only\n\n");
    for item in functions.iter().filter(|item| item.capability == "core") {
        text.push_str(&format!("- `{}`\n", item.rust_path));
    }
    text.push_str("\n### Requires `alloc`\n\n");
    for item in functions.iter().filter(|item| item.capability == "alloc") {
        text.push_str(&format!("- `{}`\n", item.rust_path));
    }
    text.push_str("\n### Requires `std`\n\n");
    for item in functions.iter().filter(|item| item.capability == "std") {
        text.push_str(&format!("- `{}`\n", item.rust_path));
    }
    text
}

fn validate_examples(functions: &[FunctionRecord]) -> Result<()> {
    for path in functions
        .iter()
        .map(|item| &item.example_file)
        .collect::<BTreeSet<_>>()
    {
        let contents = fs::read_to_string(workspace_path(path))?;
        if !contents.contains("slatec-safe-example") {
            return Err(policy(&format!("example {path} lacks the coverage marker")));
        }
    }
    for function in functions {
        let example = fs::read_to_string(workspace_path(&function.example_file))?;
        let validation_path = validation_path_for(function);
        if validation_path.is_empty() {
            return Err(policy("unknown example domain"));
        }
        let validation = fs::read_to_string(workspace_path(validation_path))?;
        let name = function
            .rust_path
            .rsplit("::")
            .next()
            .unwrap_or(&function.rust_path);
        let call_pattern = format!("{name}(");
        if !example.contains(&call_pattern) && !validation.contains(&call_pattern) {
            return Err(policy(&format!(
                "{} has no executable example or native case",
                function.rust_path
            )));
        }
    }
    Ok(())
}

fn validation_path_for(function: &FunctionRecord) -> &'static str {
    match function.domain.as_str() {
        "BLAS" if function.rust_path.contains("level1") => {
            "crates/slatec/tests/blas_level1_native.rs"
        }
        "BLAS" => "crates/slatec/tests/blas_level23_native.rs",
        "quadrature"
            if function
                .rust_path
                .contains("integrate_piecewise_polynomial") =>
        {
            "crates/slatec/tests/callback_driver_expansion.rs"
        }
        "quadrature" => "crates/slatec/tests/quadrature_native.rs",
        "roots" => "crates/slatec/tests/roots_native.rs",
        "nonlinear" if function.rust_path.contains("solve_scalar_equations") => {
            "crates/slatec/tests/callback_driver_expansion.rs"
        }
        "nonlinear" if function.feature != "nonlinear-easy" => {
            "crates/slatec/tests/nonlinear_expert_native.rs"
        }
        "nonlinear" => "crates/slatec/tests/nonlinear_native.rs",
        "least squares" => "crates/slatec/tests/least_squares_native.rs",
        "linear least squares"
            if function
                .rust_path
                .contains("bounded_constrained_least_squares") =>
        {
            "crates/slatec/tests/bounded_constrained_least_squares_native.rs"
        }
        "linear least squares" if function.rust_path.contains("bounded_least_squares") => {
            "crates/slatec/tests/bounded_least_squares_native.rs"
        }
        "linear least squares" if function.rust_path.contains("constrained_least_squares") => {
            "crates/slatec/tests/constrained_least_squares_native.rs"
        }
        "linear least squares" => "crates/slatec/tests/nonnegative_least_squares_native.rs",
        "ordinary differential equations" if function.rust_path.contains("Driv") => {
            "crates/slatec/tests/callback_driver_expansion.rs"
        }
        "ordinary differential equations" => "crates/slatec/tests/ode_sdrive_native.rs",
        "differential-algebraic equations" => "crates/slatec/tests/dassl_native.rs",
        "linear programming" => "crates/slatec/tests/linear_programming_native.rs",
        "real FFTPACK" => "crates/slatec/tests/fftpack_native.rs",
        "complex FFTPACK" => "crates/slatec/tests/fftpack_complex_native.rs",
        "banded linear systems" => "crates/slatec/tests/banded_native.rs",
        "piecewise cubic Hermite interpolation" => "crates/slatec/tests/pchip_native.rs",
        "B-spline interpolation" => "crates/slatec/tests/bspline_native.rs",
        "piecewise-polynomial interpolation" => {
            "crates/slatec/tests/piecewise_polynomial_native.rs"
        }
        "tabulated data" => "crates/slatec/tests/tabulated_data_native.rs",
        "Cartesian FISHPACK PDE" => "crates/slatec/tests/fishpack_cartesian_2d_native.rs",
        "Structured FISHPACK PDE" => "crates/slatec/tests/fishpack_pois3d_native.rs",
        "special functions" | "polynomials" => "crates/slatec/tests/special_functions_native.rs",
        _ => "",
    }
}

fn source_has_doctest(path: &str) -> bool {
    matches!(
        path,
        "slatec::blas::level1::daxpy"
            | "slatec::nonlinear::solve_system"
            | "slatec::nonlinear::solve_system_f32"
            | "slatec::nonlinear::solve_system_expert"
            | "slatec::nonlinear::solve_system_expert_f32"
            | "slatec::nonlinear::solve_system_with_jacobian"
            | "slatec::nonlinear::solve_system_with_jacobian_f32"
            | "slatec::nonlinear::solve_scalar_equations"
            | "slatec::quadrature::integrate_piecewise_polynomial"
            | "slatec::nonlinear::check_jacobian"
            | "slatec::nonlinear::check_jacobian_f32"
            | "slatec::least_squares::least_squares"
            | "slatec::least_squares::least_squares_f32"
            | "slatec::least_squares::least_squares_expert"
            | "slatec::least_squares::least_squares_expert_f32"
            | "slatec::least_squares::least_squares_with_jacobian"
            | "slatec::least_squares::least_squares_with_jacobian_f32"
            | "slatec::least_squares::estimate_covariance"
            | "slatec::least_squares::estimate_covariance_f32"
            | "slatec::least_squares::estimate_covariance_finite_difference"
            | "slatec::least_squares::estimate_covariance_finite_difference_f32"
            | "slatec::least_squares::covariance_from_expert_fit"
            | "slatec::least_squares::covariance_from_expert_fit_f32"
            | "slatec::linear_least_squares::solve_nonnegative_least_squares"
            | "slatec::linear_least_squares::solve_nonnegative_least_squares_f32"
            | "slatec::bounded_least_squares::solve_bounded_least_squares"
            | "slatec::bounded_least_squares::solve_bounded_least_squares_f32"
            | "slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares"
            | "slatec::bounded_constrained_least_squares::solve_bounded_constrained_least_squares_f32"
            | "slatec::constrained_least_squares::solve_constrained_least_squares"
            | "slatec::constrained_least_squares::solve_constrained_least_squares_f32"
    )
}

fn native_status(record: &FunctionRecord) -> &'static str {
    match record.domain.as_str() {
        "BLAS" => "validated_by_native_batch",
        "ordinary differential equations"
            if matches!(
                record.fortran_routine.as_str(),
                "SDRIV1" | "DDRIV1" | "SDRIV2" | "DDRIV2" | "CDRIV1" | "CDRIV2"
            ) =>
        {
            "native_execution_passed"
        }
        "quadrature"
        | "roots"
        | "nonlinear"
        | "least squares"
        | "linear least squares"
        | "linear programming"
        | "real FFTPACK"
        | "complex FFTPACK"
        | "piecewise cubic Hermite interpolation"
        | "piecewise-polynomial interpolation"
        | "tabulated data"
        | "Cartesian FISHPACK PDE"
        | "special functions"
        | "polynomials" => "native_execution_passed",
        _ => "unknown",
    }
}

fn purpose(family: &str) -> &'static str {
    match family {
        "copy" => "copy vector elements",
        "swap" => "exchange vector elements",
        "scale" => "scale a vector",
        "axpy" => "scaled vector addition",
        "dot" => "dot product",
        "norm" => "Euclidean norm",
        "absolute_sum" => "sum of absolute values",
        "index_max" => "index of maximum magnitude",
        "rotation" => "plane rotation",
        "gemv" => "general matrix-vector product",
        "gemm" => "general matrix-matrix product",
        "ger" => "rank-one matrix update",
        "symv" => "symmetric matrix-vector product",
        "trmv" => "triangular matrix-vector product",
        "trsv" => "triangular solve",
        "trmm" => "triangular matrix-matrix product",
        "trsm" => "triangular matrix solve",
        "syrk" => "symmetric rank-k update",
        "bracketed scalar root" => "bracketed scalar root finding",
        "finite-difference nonlinear system" => "finite-difference nonlinear-system solving",
        "FnMut(&[f64],usize)->f64" | "FnMut(&[f32],usize)->f32" => {
            "scalar-equation nonlinear-system solving"
        }
        "expert finite-difference nonlinear system" => {
            "expert finite-difference nonlinear-system solving"
        }
        "expert analytic-Jacobian nonlinear system" => {
            "expert analytic-Jacobian nonlinear-system solving"
        }
        "Jacobian consistency checking" => "componentwise Jacobian consistency checking",
        "finite-difference nonlinear least squares" => {
            "finite-difference nonlinear least-squares fitting"
        }
        "expert finite-difference nonlinear least squares" => {
            "expert finite-difference nonlinear least-squares fitting"
        }
        "expert analytic-Jacobian nonlinear least squares" => {
            "expert analytic-Jacobian nonlinear least-squares fitting"
        }
        "nonlinear least-squares covariance estimation" => {
            "nonlinear least-squares covariance estimation"
        }
        "equality-constrained nonnegative linear least squares" => {
            "equality-constrained nonnegative linear least-squares fitting"
        }
        "bounded linear least squares" => "dense bounded linear least-squares fitting",
        "bounded linear least squares with bounded constraint expressions" => {
            "bounded constrained linear least-squares fitting"
        }
        "equality and inequality constrained linear least squares" => {
            "dense equality and inequality constrained linear least-squares fitting"
        }
        "sparse in-memory linear programming" => {
            "sparse in-memory linear programming with variable and row-activity bounds"
        }
        "periodic real FFT plan initialization" => "initialize a periodic real FFTPACK plan",
        "periodic real FFT forward transform" => "compute a packed periodic real Fourier transform",
        "periodic real FFT backward transform" => {
            "compute a packed periodic real Fourier synthesis"
        }
        "easy real Fourier plan initialization" => "initialize an easy real Fourier plan",
        "easy real Fourier forward transform" => {
            "compute separate real Fourier-series coefficients"
        }
        "easy real Fourier synthesis" => {
            "synthesize a real sequence from Fourier-series coefficients"
        }
        "full sine transform plan initialization" => "initialize a full sine-transform plan",
        "full sine transform" => "compute the full FFTPACK sine transform",
        "full cosine transform plan initialization" => "initialize a full cosine-transform plan",
        "full cosine transform" => "compute the full FFTPACK cosine transform",
        "quarter-wave sine plan initialization" => "initialize a quarter-wave sine-transform plan",
        "quarter-wave sine forward transform" => "compute a quarter-wave sine forward transform",
        "quarter-wave sine backward transform" => "compute a quarter-wave sine backward transform",
        "quarter-wave cosine plan initialization" => {
            "initialize a quarter-wave cosine-transform plan"
        }
        "quarter-wave cosine forward transform" => {
            "compute a quarter-wave cosine forward transform"
        }
        "quarter-wave cosine backward transform" => {
            "compute a quarter-wave cosine backward transform"
        }
        "monotonicity-preserving Hermite derivative construction" => {
            "construct monotonicity-preserving piecewise-cubic Hermite derivatives"
        }
        "controlled monotone Hermite derivative construction" => {
            "construct controlled monotone piecewise-cubic Hermite derivatives"
        }
        "cubic-spline Hermite derivative construction" => {
            "construct PCHSP cubic-spline Hermite derivatives"
        }
        "piecewise-cubic Hermite value evaluation" => "evaluate a piecewise-cubic Hermite curve",
        "piecewise-cubic Hermite value and derivative evaluation" => {
            "evaluate a piecewise-cubic Hermite curve and first derivative"
        }
        "piecewise-cubic Hermite definite integration" => {
            "integrate a piecewise-cubic Hermite curve"
        }
        "PP Taylor evaluation" => "evaluate a right-Taylor piecewise polynomial",
        "PP Taylor derivative evaluation" => {
            "evaluate a right-Taylor piecewise-polynomial derivative"
        }
        "PP batch evaluation" => "batch-evaluate a right-Taylor piecewise polynomial",
        "exact PP definite integration" => "integrate a piecewise polynomial exactly",
        "exact B-spline to PP conversion" => {
            "convert a B-spline exactly to piecewise-polynomial form"
        }
        "construct global Newton interpolation polynomial" => {
            "construct a global Newton interpolation polynomial from checked samples"
        }
        "evaluate global Newton interpolation polynomial" => {
            "evaluate a global Newton interpolation polynomial"
        }
        "evaluate global Newton interpolation polynomial and first derivatives" => {
            "evaluate a global Newton interpolation polynomial and its first derivatives"
        }
        "derive Taylor coefficients about a finite centre" => {
            "derive global polynomial Taylor coefficients about a finite centre"
        }
        "integrate arbitrarily spaced tabulated values" => {
            "integrate arbitrarily spaced tabulated values by overlapping parabolas"
        }
        "finite" => "adaptive finite-interval integration",
        "FnMut(f64)->f64" => "piecewise-polynomial weighted quadrature",
        "infinite" => "adaptive infinite-interval integration",
        "breakpoints" => "breakpoint-aware integration",
        "weighted_endpoints" => "endpoint-weighted integration",
        "finite_oscillatory" => "finite oscillatory integration",
        "fourier_tail" => "infinite Fourier-tail integration",
        "non_adaptive" => "non-adaptive nested-rule integration",
        "nc79" => "Newton-Cotes integration",
        "principal_value" => "Cauchy principal-value integration",
        "singular" => "endpoint-singularity integration",
        _ => "validated scalar numerical function",
    }
}

fn raw_routine(symbol: &str) -> String {
    symbol.trim_end_matches('_').to_ascii_uppercase()
}
fn read_json(path: &str) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(workspace_path(path))?)?)
}
fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn array<'a>(value: &'a Value, field: &str) -> Result<&'a Vec<Value>> {
    value[field].as_array().ok_or_else(|| policy(field))
}
fn string<'a>(value: &'a Value, field: &str) -> Result<&'a str> {
    value[field].as_str().ok_or_else(|| policy(field))
}
fn string_columns(value: &Value) -> Result<Vec<String>> {
    Ok(array(value, "columns")?
        .iter()
        .map(|v| v.as_str().unwrap_or_default().to_owned())
        .collect())
}
fn rows(value: &Value) -> Result<Vec<&[Value]>> {
    array(value, "records")?
        .iter()
        .map(|v| v.as_array().map(Vec::as_slice).ok_or_else(|| policy("row")))
        .collect::<Result<Vec<_>>>()
}
fn index(columns: &[String], name: &str) -> Result<usize> {
    columns
        .iter()
        .position(|column| column == name)
        .ok_or_else(|| policy(name))
}
fn column<'a>(row: &'a [Value], columns: &[String], name: &str) -> Result<&'a str> {
    row[index(columns, name)?]
        .as_str()
        .ok_or_else(|| policy(name))
}
fn policy(detail: &str) -> CorpusError {
    CorpusError::Policy(format!("safe API documentation inventory: {detail}"))
}
fn write_json(path: &Path, value: &impl Serialize) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, &bytes)?;
    Ok(bytes)
}

fn reject_duplicates(functions: &[FunctionRecord]) -> Result<()> {
    let mut paths = BTreeSet::new();
    for item in functions {
        if !paths.insert(&item.rust_path) {
            return Err(policy(&format!("duplicate Rust path {}", item.rust_path)));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic_and_complete() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let a = generate(first.path()).unwrap();
        let b = generate(second.path()).unwrap();
        assert_eq!(a.function_count, b.function_count);
        assert_eq!(a.semantic_hash, b.semantic_hash);
        assert!(a.function_count >= 150);
        for file in [
            "function-index.json",
            "fortran-argument-map.json",
            "example-coverage.json",
            "capability-summary.json",
            "function-index.md",
        ] {
            assert_eq!(
                fs::read(first.path().join(file)).unwrap(),
                fs::read(second.path().join(file)).unwrap()
            );
        }
    }
}
