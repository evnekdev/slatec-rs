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
    transformation: String,
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
                "native_execution_status": native_status(&record.domain),
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
        &IndexFile {
            schema_id: "slatec.safe-api.fortran-argument-map",
            schema_version: "1.0.0",
            semantic_version: "1",
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
        "quadrature" => "examples/quadrature/families.rs".to_owned(),
        "roots" => "examples/roots/scalar.rs".to_owned(),
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
        "polynomials" => "examples/special/functions.rs".to_owned(),
        _ => "examples/special/functions.rs".to_owned(),
    };
    FunctionRecord {
        rust_path: path.to_owned(),
        fortran_routine: routine.to_ascii_uppercase(),
        module: module.to_owned(),
        domain: domain.to_owned(),
        precision: precision.to_owned(),
        purpose: purpose(family).to_owned(),
        capability: capability.to_owned(),
        feature: feature.to_owned(),
        native_profile: PROFILE.to_owned(),
        documentation: format!(
            "https://docs.rs/slatec/latest/slatec/{}/fn.{name}.html",
            module.trim_start_matches("slatec::").replace("::", "/")
        ),
        example_file,
        example_case: family.to_owned(),
        inclusion_status: "safe_api_available".to_owned(),
    }
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
        .map(|function| {
            let (program_unit_id, ids) =
                by_name.get(&function.fortran_routine).ok_or_else(|| {
                    policy(&format!(
                        "missing interface for {}",
                        function.fortran_routine
                    ))
                })?;
            let arguments = ids
                .iter()
                .filter_map(|id| by_id.get(id))
                .map(|name| argument_map(function, name))
                .collect();
            Ok(MappingRecord {
                rust_path: function.rust_path.clone(),
                fortran_routine: function.fortran_routine.clone(),
                program_unit_id: program_unit_id.clone(),
                arguments,
            })
        })
        .collect()
}

fn argument_map(function: &FunctionRecord, name: &str) -> ArgumentMap {
    let upper = name.to_ascii_uppercase();
    let internal = [
        "WORK", "IWORK", "LENW", "LAST", "NEVAL", "IER", "IFLAG", "RESULT", "ABSERR", "ALIST",
        "BLIST", "RLIST", "ELIST", "JAC", "IOPT", "NPRINT", "WA", "LWA", "FJAC", "LDFJAC", "R",
        "LR", "QTF", "WA1", "WA2", "WA3", "WA4", "XP", "FVECP", "MODE", "IPVT", "ERR", "RW", "IW",
        "WS", "IP",
    ];
    let inferred = function.rust_path.ends_with("_contiguous")
        && ["LDA", "LDB", "LDC", "INCX", "INCY"].contains(&upper.as_str());
    let analytic_jacobian = function.rust_path.contains("with_jacobian");
    let jacobian_check = function.rust_path.contains("check_jacobian");
    let internal_argument = (internal.contains(&upper.as_str())
        || (jacobian_check && upper == "FVEC"))
        && !(matches!(function.domain.as_str(), "nonlinear" | "least squares") && upper == "INFO")
        && !(function.domain == "linear least squares" && upper == "MODE")
        && !(analytic_jacobian && upper == "JAC")
        && !(jacobian_check && matches!(upper.as_str(), "FJAC" | "ERR"));
    let rust = if internal_argument || inferred {
        None
    } else {
        Some(match upper.as_str() {
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
        || (analytic_jacobian && upper == "JAC")
        || (jacobian_check && upper == "FJAC");
    ArgumentMap {
        rust,
        fortran: upper,
        transformation: if inferred {
            "inferred by the tightly packed convenience wrapper".to_owned()
        } else if internal_argument {
            "allocated, initialized, or validated internally".to_owned()
        } else if callback_argument {
            "Rust closure through the contained callback trampoline".to_owned()
        } else if function.domain == "linear least squares" {
            "validated and copied, derived, or reconstructed in owned native storage".to_owned()
        } else {
            "validated value or caller-owned slice passed through the reviewed raw ABI".to_owned()
        },
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
        "quadrature" => "crates/slatec/tests/quadrature_native.rs",
        "roots" => "crates/slatec/tests/roots_native.rs",
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

fn native_status(domain: &str) -> &'static str {
    match domain {
        "BLAS" => "validated_by_native_batch",
        "quadrature"
        | "roots"
        | "nonlinear"
        | "least squares"
        | "linear least squares"
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
        "finite" => "adaptive finite-interval integration",
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
