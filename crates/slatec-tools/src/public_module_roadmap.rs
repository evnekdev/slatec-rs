//! Deterministic safe-API module-roadmap metadata.

use crate::error::{CorpusError, Result};
use crate::hash;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: &str = "1.0.0";
const SAFE_FUNCTION_TARGET: usize = 238;
const CURRENT_FAMILY_FEATURES: &[&str] = &[
    "blas-level1",
    "blas-level2",
    "blas-level3",
    "special-elementary",
    "special-gamma",
    "special-beta",
    "special-error",
    "special-airy",
    "special-bessel",
    "special-integrals",
    "special-polynomials",
    "special-scalar-expanded",
    "quadrature-basic",
    "quadrature-breakpoints",
    "quadrature-weighted",
    "quadrature-oscillatory",
    "quadrature-fourier",
    "quadrature-nonadaptive",
    "roots-scalar",
    "roots-polynomial",
    "nonlinear-easy",
    "nonlinear-expert",
    "nonlinear-jacobian-check",
    "least-squares-nonlinear-easy",
    "least-squares-nonlinear-expert",
    "least-squares-covariance",
    "least-squares-linear-nonnegative",
    "least-squares-linear-bounded",
    "least-squares-linear-constrained",
    "least-squares-linear-bounded-constrained",
    "ode-sdrive-expert",
    "dassl",
    "optimization-linear-programming-in-memory",
    "fftpack-real",
    "fftpack-complex",
    "pchip",
    "bspline",
    "piecewise-polynomial",
];
const FROZEN_HIGH_LEVEL_PATHS: &[&str] = &[
    "roadmap",
    "linear_algebra",
    "special",
    "integration",
    "equations",
    "least_squares",
    "differential_equations",
    "optimization",
    "transforms",
    "interpolation",
];

#[derive(Clone, Debug)]
struct LeafSpec {
    path: &'static str,
    module_file: &'static str,
    stability: &'static str,
    status: &'static str,
    feature: Option<&'static str>,
    implementation_module: Option<&'static str>,
    compatibility_path: Option<&'static str>,
    native_families: &'static [&'static str],
    precision: &'static str,
    runtime: &'static str,
    concurrency: &'static str,
    rationale: &'static str,
    blocker: &'static str,
    next_milestone: &'static str,
    evidence_source: &'static str,
    item_prefixes: &'static [&'static str],
}

#[derive(Clone, Debug, Serialize)]
struct LeafRecord {
    canonical_grouped_module_path: &'static str,
    module_file: &'static str,
    stability: &'static str,
    status: &'static str,
    cargo_feature: Option<&'static str>,
    current_implementation_module: Option<&'static str>,
    compatibility_path: Option<&'static str>,
    safe_native_families: Vec<&'static str>,
    precision_coverage: &'static str,
    runtime_level: &'static str,
    concurrency_classification: &'static str,
    status_rationale: &'static str,
    blocker: &'static str,
    likely_next_milestone: &'static str,
    evidence_source: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct FeatureRecord {
    cargo_feature: &'static str,
    grouped_paths: &'static [&'static str],
    evidence_source: &'static str,
}

#[derive(Debug, Serialize)]
struct MetadataFile<T> {
    schema_id: &'static str,
    schema_version: &'static str,
    records: T,
}

#[derive(Debug, Serialize)]
struct RoadmapFile {
    schema_id: &'static str,
    schema_version: &'static str,
    safe_function_count: usize,
    capability_counts: BTreeMap<String, usize>,
    status_totals: BTreeMap<String, usize>,
    stability_totals: BTreeMap<String, usize>,
    frozen_high_level_paths: &'static [&'static str],
    records: Vec<LeafRecord>,
}

#[derive(Debug)]
pub struct GenerationResult {
    /// Current checked safe-function count.
    pub safe_function_count: usize,
    /// Semantic hash of the canonical roadmap record set.
    pub semantic_hash: String,
}

macro_rules! implemented {
    ($path:literal, $file:literal, $feature:literal, $implementation:literal, $compatibility:literal, $family:literal, $precision:literal, $runtime:literal, $concurrency:literal, $items:expr) => {
        LeafSpec {
            path: $path,
            module_file: $file,
            stability: "Reserved",
            status: "Implemented",
            feature: Some($feature),
            implementation_module: Some($implementation),
            compatibility_path: Some($compatibility),
            native_families: &[$family],
            precision: $precision,
            runtime: $runtime,
            concurrency: $concurrency,
            rationale: "The reviewed initial safe scope is public.",
            blocker: "None for the documented initial scope.",
            next_milestone: "Expand only after a focused source, ABI, workspace, and state audit.",
            evidence_source: "generated/safe-api/function-index.json",
            item_prefixes: $items,
        }
    };
}

macro_rules! partial {
    ($path:literal, $file:literal, $feature:literal, $implementation:literal, $compatibility:literal, $family:literal, $precision:literal, $runtime:literal, $concurrency:literal, $blocker:literal, $next:literal) => {
        LeafSpec {
            path: $path,
            module_file: $file,
            stability: "Reserved",
            status: "Partial",
            feature: Some($feature),
            implementation_module: Some($implementation),
            compatibility_path: Some($compatibility),
            native_families: &[$family],
            precision: $precision,
            runtime: $runtime,
            concurrency: $concurrency,
            rationale: "Useful safe public functionality exists, but material reviewed scope remains.",
            blocker: $blocker,
            next_milestone: $next,
            evidence_source: "generated/safe-api/function-index.json",
            item_prefixes: &[],
        }
    };
}

macro_rules! planned {
    ($path:literal, $file:literal, $stability:literal, $blocker:literal, $next:literal) => {
        LeafSpec {
            path: $path,
            module_file: $file,
            stability: $stability,
            status: "Planned",
            feature: None,
            implementation_module: None,
            compatibility_path: None,
            native_families: &[],
            precision: "unreviewed",
            runtime: "unreviewed",
            concurrency: "unreviewed",
            rationale: "A coherent future family is intended, but no safe API is public.",
            blocker: $blocker,
            next_milestone: $next,
            evidence_source: "selected-snapshot inventories and domain roadmap",
            item_prefixes: &[],
        }
    };
}

macro_rules! deferred {
    ($path:literal, $file:literal, $blocker:literal) => {
        LeafSpec {
            path: $path,
            module_file: $file,
            stability: "Reserved",
            status: "Deferred",
            feature: None,
            implementation_module: None,
            compatibility_path: None,
            native_families: &[],
            precision: "unreviewed",
            runtime: "unreviewed",
            concurrency: "unreviewed",
            rationale: "The family is deliberately postponed pending a documented blocker.",
            blocker: $blocker,
            next_milestone: "Resolve the blocker in a dedicated audited milestone.",
            evidence_source: "generated safe-family deferred metadata",
            item_prefixes: &[],
        }
    };
}

fn leaves() -> Vec<LeafSpec> {
    vec![
        implemented!(
            "linear_algebra::blas::level1",
            "crates/slatec/src/linear_algebra/blas/level1.rs",
            "blas-level1",
            "crate::blas::level1",
            "slatec::blas::level1",
            "blas-level1",
            "f32,f64",
            "core",
            "BackendDependent",
            &["slatec::blas::level1::"]
        ),
        implemented!(
            "linear_algebra::blas::level2",
            "crates/slatec/src/linear_algebra/blas/level2.rs",
            "blas-level2",
            "crate::blas::level2",
            "slatec::blas::level2",
            "blas-level2",
            "f32,f64",
            "core",
            "SerializedGlobal",
            &["slatec::blas::level2::"]
        ),
        implemented!(
            "linear_algebra::blas::level3",
            "crates/slatec/src/linear_algebra/blas/level3.rs",
            "blas-level3",
            "crate::blas::level3",
            "slatec::blas::level3",
            "blas-level3",
            "f32,f64",
            "core",
            "SerializedGlobal",
            &["slatec::blas::level3::"]
        ),
        planned!(
            "linear_algebra::dense",
            "crates/slatec/src/linear_algebra/dense.rs",
            "Reserved",
            "Dense overwrite and factorization contracts are unreviewed.",
            "Audit one narrow dense solver family."
        ),
        planned!(
            "linear_algebra::banded",
            "crates/slatec/src/linear_algebra/banded.rs",
            "Reserved",
            "Banded leading-dimension and compact-layout contracts are unreviewed.",
            "Audit a narrow banded family."
        ),
        planned!(
            "linear_algebra::packed",
            "crates/slatec/src/linear_algebra/packed.rs",
            "Reserved",
            "Packed storage representation requires a dedicated safe view policy.",
            "Audit packed storage and mutation semantics."
        ),
        planned!(
            "linear_algebra::eigen",
            "crates/slatec/src/linear_algebra/eigen.rs",
            "Reserved",
            "No safe EISPACK/LINPACK family is selected.",
            "Inventory eigenvalue drivers."
        ),
        planned!(
            "linear_algebra::sparse::iterative",
            "crates/slatec/src/linear_algebra/sparse/iterative.rs",
            "Reserved",
            "Sparse ownership and callback contracts are unreviewed.",
            "Audit an iterative sparse family."
        ),
        planned!(
            "linear_algebra::sparse::direct",
            "crates/slatec/src/linear_algebra/sparse/direct.rs",
            "Provisional",
            "Source-family selection and sparse storage ownership remain uncertain.",
            "Complete sparse direct-family inventory."
        ),
        planned!(
            "linear_algebra::sparse::operators",
            "crates/slatec/src/linear_algebra/sparse/operators.rs",
            "Reserved",
            "No checked sparse operator representation exists.",
            "Define storage-neutral checked views after audit."
        ),
        partial!(
            "special::elementary",
            "crates/slatec/src/special/elementary.rs",
            "special-elementary",
            "crate::special::elementary",
            "slatec::special::elementary",
            "special-elementary",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Remaining scalar candidates need domain and scaling review.",
            "Expand only reviewed scalar contracts."
        ),
        partial!(
            "special::gamma",
            "crates/slatec/src/special/gamma.rs",
            "special-gamma",
            "crate::special::gamma",
            "slatec::special::gamma",
            "special-gamma",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Several related contracts remain deferred.",
            "Audit additional real scalar gamma functions."
        ),
        partial!(
            "special::beta",
            "crates/slatec/src/special/beta.rs",
            "special-beta",
            "crate::special::gamma",
            "slatec::special::gamma",
            "special-beta",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "The historical implementation shares the gamma module and remains incomplete.",
            "Audit additional beta-family scalar contracts."
        ),
        partial!(
            "special::error",
            "crates/slatec/src/special/error_family.rs",
            "special-error",
            "crate::special::error_functions",
            "slatec::special::error_functions",
            "special-error",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Only reviewed scalar error functions are public.",
            "Audit additional real scalar error/probability contracts."
        ),
        partial!(
            "special::airy",
            "crates/slatec/src/special/airy.rs",
            "special-airy",
            "crate::special::airy",
            "slatec::special::airy",
            "special-airy",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Sequence and further Airy contracts remain deferred.",
            "Audit further scalar Airy contracts."
        ),
        partial!(
            "special::bessel",
            "crates/slatec/src/special/bessel.rs",
            "special-bessel",
            "crate::special::bessel",
            "slatec::special::bessel",
            "special-bessel",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Complex and sequence Bessel interfaces remain deferred.",
            "Audit a distinct real scalar Bessel subset."
        ),
        partial!(
            "special::integrals",
            "crates/slatec/src/special/integrals.rs",
            "special-integrals",
            "crate::special::integrals",
            "slatec::special::integrals",
            "special-integrals",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Only audited real scalar integral contracts are public.",
            "Audit another explicitly bounded integral family."
        ),
        partial!(
            "special::integrals::exponential",
            "crates/slatec/src/special/integrals/exponential.rs",
            "special-integrals",
            "crate::special::integrals",
            "slatec::special::integrals",
            "special-integrals",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "The current E1/Ei domain is deliberately conservative.",
            "Expand only after branch and overflow audit."
        ),
        implemented!(
            "special::integrals::logarithmic",
            "crates/slatec/src/special/integrals/logarithmic.rs",
            "special-scalar-expanded",
            "crate::special::scalar_expanded",
            "slatec::special::scalar_expanded",
            "special-scalar-expanded",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::special::scalar_expanded::logarithmic_integral"]
        ),
        implemented!(
            "special::integrals::spence",
            "crates/slatec/src/special/integrals/spence.rs",
            "special-scalar-expanded",
            "crate::special::scalar_expanded",
            "slatec::special::scalar_expanded",
            "special-scalar-expanded",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::special::scalar_expanded::spence_integral"]
        ),
        planned!(
            "special::integrals::trigonometric",
            "crates/slatec/src/special/integrals/trigonometric.rs",
            "Reserved",
            "Real-domain and branch contracts are not yet selected.",
            "Audit trigonometric scalar integrals."
        ),
        partial!(
            "special::elliptic",
            "crates/slatec/src/special/elliptic/mod.rs",
            "special-scalar-expanded",
            "crate::special::scalar_expanded",
            "slatec::special::scalar_expanded",
            "special-scalar-expanded",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Carlson forms are public; other elliptic conventions are unreviewed.",
            "Audit a separately named elliptic family."
        ),
        implemented!(
            "special::elliptic::carlson",
            "crates/slatec/src/special/elliptic/carlson.rs",
            "special-scalar-expanded",
            "crate::special::scalar_expanded",
            "slatec::special::scalar_expanded",
            "special-scalar-expanded",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::special::scalar_expanded::carlson_"]
        ),
        partial!(
            "special::probability",
            "crates/slatec/src/special/probability.rs",
            "special-beta",
            "crate::special::gamma",
            "slatec::special::gamma",
            "special-beta",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Only regularized beta is currently grouped here; no distribution framework exists.",
            "Audit additional scalar probability functions."
        ),
        partial!(
            "special::polynomials",
            "crates/slatec/src/special/polynomials.rs",
            "special-polynomials",
            "crate::polynomials",
            "slatec::polynomials",
            "special-polynomials",
            "f32,f64",
            "core",
            "SerializedGlobal",
            "Only selected scalar orthogonal-polynomial helpers are public.",
            "Audit coherent scalar polynomial evaluators."
        ),
        implemented!(
            "integration::quadrature::basic",
            "crates/slatec/src/integration/quadrature/basic.rs",
            "quadrature-basic",
            "crate::quadrature",
            "slatec::quadrature",
            "quadrature-basic",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::quadrature::integrate"]
        ),
        implemented!(
            "integration::quadrature::breakpoints",
            "crates/slatec/src/integration/quadrature/breakpoints.rs",
            "quadrature-breakpoints",
            "crate::quadrature",
            "slatec::quadrature",
            "quadrature-breakpoints",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::quadrature::integrate_with_breakpoints"]
        ),
        implemented!(
            "integration::quadrature::weighted",
            "crates/slatec/src/integration/quadrature/weighted.rs",
            "quadrature-weighted",
            "crate::quadrature",
            "slatec::quadrature",
            "quadrature-weighted",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::quadrature::integrate_weighted_endpoints"]
        ),
        implemented!(
            "integration::quadrature::oscillatory",
            "crates/slatec/src/integration/quadrature/oscillatory.rs",
            "quadrature-oscillatory",
            "crate::quadrature",
            "slatec::quadrature",
            "quadrature-oscillatory",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::quadrature::integrate_oscillatory"]
        ),
        implemented!(
            "integration::quadrature::fourier",
            "crates/slatec/src/integration/quadrature/fourier.rs",
            "quadrature-fourier",
            "crate::quadrature",
            "slatec::quadrature",
            "quadrature-fourier",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::quadrature::integrate_fourier_tail"]
        ),
        implemented!(
            "integration::quadrature::nonadaptive",
            "crates/slatec/src/integration/quadrature/nonadaptive.rs",
            "quadrature-nonadaptive",
            "crate::quadrature",
            "slatec::quadrature",
            "quadrature-nonadaptive",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::quadrature::integrate_non_adaptive"]
        ),
        planned!(
            "integration::integral_equations",
            "crates/slatec/src/integration/integral_equations.rs",
            "Reserved",
            "No reviewed safe integral-equation family is selected.",
            "Inventory candidate source families."
        ),
        implemented!(
            "equations::roots::scalar",
            "crates/slatec/src/equations/roots/scalar.rs",
            "roots-scalar",
            "crate::roots",
            "slatec::roots",
            "roots-scalar",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::roots::find_root"]
        ),
        partial!(
            "equations::roots::polynomial",
            "crates/slatec/src/equations/roots/polynomial.rs",
            "roots-polynomial",
            "none",
            "none",
            "roots-polynomial",
            "unreviewed",
            "alloc",
            "unreviewed",
            "Selected source support has no safe public facade.",
            "Audit polynomial output, mutation, and multiplicity contracts."
        ),
        implemented!(
            "equations::nonlinear::easy",
            "crates/slatec/src/equations/nonlinear/easy.rs",
            "nonlinear-easy",
            "crate::nonlinear",
            "slatec::nonlinear",
            "nonlinear-easy",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::nonlinear::solve_system"]
        ),
        implemented!(
            "equations::nonlinear::expert",
            "crates/slatec/src/equations/nonlinear/expert.rs",
            "nonlinear-expert",
            "crate::nonlinear",
            "slatec::nonlinear",
            "nonlinear-expert",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::nonlinear::solve_system_expert"]
        ),
        implemented!(
            "equations::nonlinear::jacobian_check",
            "crates/slatec/src/equations/nonlinear/jacobian_check.rs",
            "nonlinear-jacobian-check",
            "crate::nonlinear",
            "slatec::nonlinear",
            "nonlinear-jacobian-check",
            "f32,f64",
            "alloc",
            "SerializedGlobal",
            &["slatec::nonlinear::check_jacobian"]
        ),
        implemented!(
            "least_squares::nonlinear::easy",
            "crates/slatec/src/least_squares/nonlinear/easy.rs",
            "least-squares-nonlinear-easy",
            "crate::least_squares",
            "slatec::least_squares",
            "least-squares-nonlinear-easy",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::least_squares::least_squares"]
        ),
        implemented!(
            "least_squares::nonlinear::expert",
            "crates/slatec/src/least_squares/nonlinear/expert.rs",
            "least-squares-nonlinear-expert",
            "crate::least_squares",
            "slatec::least_squares",
            "least-squares-nonlinear-expert",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::least_squares::least_squares_expert"]
        ),
        implemented!(
            "least_squares::covariance",
            "crates/slatec/src/least_squares/covariance.rs",
            "least-squares-covariance",
            "crate::least_squares::covariance",
            "slatec::least_squares",
            "least-squares-covariance",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::least_squares::estimate_covariance"]
        ),
        implemented!(
            "least_squares::linear::nonnegative",
            "crates/slatec/src/least_squares/linear/nonnegative.rs",
            "least-squares-linear-nonnegative",
            "crate::linear_least_squares",
            "slatec::linear_least_squares",
            "least-squares-linear-nonnegative",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::linear_least_squares::solve_nonnegative"]
        ),
        implemented!(
            "least_squares::linear::bounded",
            "crates/slatec/src/least_squares/linear/bounded.rs",
            "least-squares-linear-bounded",
            "crate::bounded_least_squares",
            "slatec::bounded_least_squares",
            "least-squares-linear-bounded",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::bounded_least_squares::solve_bounded"]
        ),
        implemented!(
            "least_squares::linear::constrained",
            "crates/slatec/src/least_squares/linear/constrained.rs",
            "least-squares-linear-constrained",
            "crate::constrained_least_squares",
            "slatec::constrained_least_squares",
            "least-squares-linear-constrained",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::constrained_least_squares::solve_constrained"]
        ),
        implemented!(
            "least_squares::linear::bounded_constrained",
            "crates/slatec/src/least_squares/linear/bounded_constrained.rs",
            "least-squares-linear-bounded-constrained",
            "crate::bounded_constrained_least_squares",
            "slatec::bounded_constrained_least_squares",
            "least-squares-linear-bounded-constrained",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::bounded_constrained_least_squares::solve_bounded"]
        ),
        implemented!(
            "differential_equations::ode::sdrive",
            "crates/slatec/src/differential_equations/ode/sdrive.rs",
            "ode-sdrive-expert",
            "crate::ode",
            "slatec::ode",
            "ode-sdrive-expert",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::ode::"]
        ),
        planned!(
            "differential_equations::ode::runge_kutta",
            "crates/slatec/src/differential_equations/ode/runge_kutta.rs",
            "Reserved",
            "No reviewed safe Runge--Kutta source family is selected.",
            "Audit a bounded ODE family."
        ),
        planned!(
            "differential_equations::ode::adams",
            "crates/slatec/src/differential_equations/ode/adams.rs",
            "Reserved",
            "Continuation and workspace semantics are unreviewed.",
            "Audit a bounded Adams family."
        ),
        implemented!(
            "differential_equations::dae::dassl::residual",
            "crates/slatec/src/differential_equations/dae/dassl/residual.rs",
            "dassl",
            "crate::dassl",
            "slatec::dassl",
            "dassl",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::dassl::"]
        ),
        deferred!(
            "differential_equations::dae::dassl::user_jacobian",
            "crates/slatec/src/differential_equations/dae/dassl/user_jacobian.rs",
            "User-Jacobian callback aliasing and lifecycle contracts are not yet reviewed."
        ),
        deferred!(
            "differential_equations::dae::dassl::banded",
            "crates/slatec/src/differential_equations/dae/dassl/banded.rs",
            "Banded Jacobian layout and callback contracts are outside the residual-only scope."
        ),
        planned!(
            "differential_equations::boundary_value",
            "crates/slatec/src/differential_equations/boundary_value.rs",
            "Reserved",
            "No reviewed safe boundary-value family is selected.",
            "Inventory boundary-value candidates."
        ),
        planned!(
            "differential_equations::pde",
            "crates/slatec/src/differential_equations/pde.rs",
            "Reserved",
            "PDE/FISHPACK contracts are outside current ODE/DAE scopes.",
            "Audit a distinct PDE family."
        ),
        implemented!(
            "optimization::linear_programming",
            "crates/slatec/src/optimization/linear_programming.rs",
            "optimization-linear-programming-in-memory",
            "crate::linear_programming",
            "slatec::linear_programming",
            "optimization-linear-programming-in-memory",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::linear_programming::"]
        ),
        planned!(
            "optimization::unconstrained",
            "crates/slatec/src/optimization/unconstrained.rs",
            "Reserved",
            "No selected safe unconstrained-minimization family exists.",
            "Use the optimization inventory to select one family."
        ),
        planned!(
            "optimization::constrained",
            "crates/slatec/src/optimization/constrained.rs",
            "Reserved",
            "No selected safe constrained-optimization family exists.",
            "Use the optimization inventory to select one family."
        ),
        planned!(
            "optimization::nonlinear",
            "crates/slatec/src/optimization/nonlinear.rs",
            "Provisional",
            "Callback, state, and source-family selection remain uncertain.",
            "Complete nonlinear-optimization inventory."
        ),
        implemented!(
            "transforms::fft::real",
            "crates/slatec/src/transforms/fft/real.rs",
            "fftpack-real",
            "crate::fftpack",
            "slatec::fftpack",
            "fftpack-real",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::fftpack::"]
        ),
        partial!(
            "transforms::fft::complex",
            "crates/slatec/src/transforms/fft/complex.rs",
            "fftpack-complex",
            "crate::transforms::fft::complex",
            "none",
            "fftpack-complex",
            "f32 (Complex32); Complex64 native roots absent",
            "std",
            "SerializedGlobal",
            "Only the selected single-precision standard real-array complex FFTPACK interface exists; multidimensional work remains separate.",
            "Audit a native double-precision complex family only if one is added to the selected snapshot."
        ),
        planned!(
            "transforms::fft::multidimensional",
            "crates/slatec/src/transforms/fft/multidimensional.rs",
            "Reserved",
            "No reviewed safe multidimensional transform family is selected.",
            "Audit a bounded multidimensional transform family."
        ),
        implemented!(
            "interpolation::pchip",
            "crates/slatec/src/interpolation/pchip.rs",
            "pchip",
            "crate::pchip",
            "slatec::pchip",
            "pchip",
            "f32,f64",
            "std",
            "SerializedGlobal",
            &["slatec::pchip::"]
        ),
        partial!(
            "interpolation::bspline",
            "crates/slatec/src/interpolation/bspline.rs",
            "bspline",
            "crate::interpolation::bspline",
            "none",
            "bspline",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "Interpolation construction, basis vectors, weighted callbacks, tensor products, and smoothing remain deferred.",
            "Audit one constructor family without broadening the representation API."
        ),
        partial!(
            "interpolation::piecewise_polynomial",
            "crates/slatec/src/interpolation/piecewise_polynomial.rs",
            "piecewise-polynomial",
            "crate::interpolation::piecewise_polynomial",
            "none",
            "piecewise-polynomial",
            "f32,f64",
            "std",
            "SerializedGlobal",
            "PP-to-B-spline conversion, PCHIP conversion, multidimensional PP, fitting, and arbitrary-stride storage remain deferred.",
            "Audit one additional representation conversion only after its native contract and storage semantics are complete."
        ),
        planned!(
            "interpolation::divided_differences",
            "crates/slatec/src/interpolation/divided_differences.rs",
            "Reserved",
            "No selected safe divided-difference family exists.",
            "Audit a coherent family."
        ),
        planned!(
            "interpolation::chebyshev",
            "crates/slatec/src/interpolation/chebyshev.rs",
            "Reserved",
            "Interpolation uses need a distinct contract from scalar polynomial evaluation.",
            "Audit Chebyshev interpolation."
        ),
        planned!(
            "interpolation::approximation",
            "crates/slatec/src/interpolation/approximation.rs",
            "Provisional",
            "The intended source-family boundary remains uncertain.",
            "Complete approximation-family inventory."
        ),
    ]
}

fn features() -> Vec<FeatureRecord> {
    vec![
        FeatureRecord {
            cargo_feature: "blas-level1",
            grouped_paths: &["linear_algebra::blas::level1"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "blas-level2",
            grouped_paths: &["linear_algebra::blas::level2"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "blas-level3",
            grouped_paths: &["linear_algebra::blas::level3"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-elementary",
            grouped_paths: &["special::elementary"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-gamma",
            grouped_paths: &["special::gamma"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-beta",
            grouped_paths: &["special::beta", "special::probability"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-error",
            grouped_paths: &["special::error"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-airy",
            grouped_paths: &["special::airy"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-bessel",
            grouped_paths: &["special::bessel"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-integrals",
            grouped_paths: &["special::integrals", "special::integrals::exponential"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-polynomials",
            grouped_paths: &["special::polynomials"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "special-scalar-expanded",
            grouped_paths: &[
                "special::integrals::logarithmic",
                "special::integrals::spence",
                "special::elliptic::carlson",
            ],
            evidence_source: "generated/safe-api/special-wrapper-index.json",
        },
        FeatureRecord {
            cargo_feature: "quadrature-basic",
            grouped_paths: &["integration::quadrature::basic"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "quadrature-breakpoints",
            grouped_paths: &["integration::quadrature::breakpoints"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "quadrature-weighted",
            grouped_paths: &["integration::quadrature::weighted"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "quadrature-oscillatory",
            grouped_paths: &["integration::quadrature::oscillatory"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "quadrature-fourier",
            grouped_paths: &["integration::quadrature::fourier"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "quadrature-nonadaptive",
            grouped_paths: &["integration::quadrature::nonadaptive"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "roots-scalar",
            grouped_paths: &["equations::roots::scalar"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "roots-polynomial",
            grouped_paths: &["equations::roots::polynomial"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "nonlinear-easy",
            grouped_paths: &["equations::nonlinear::easy"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "nonlinear-expert",
            grouped_paths: &["equations::nonlinear::expert"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "nonlinear-jacobian-check",
            grouped_paths: &["equations::nonlinear::jacobian_check"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "least-squares-nonlinear-easy",
            grouped_paths: &["least_squares::nonlinear::easy"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "least-squares-nonlinear-expert",
            grouped_paths: &["least_squares::nonlinear::expert"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "least-squares-covariance",
            grouped_paths: &["least_squares::covariance"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "least-squares-linear-nonnegative",
            grouped_paths: &["least_squares::linear::nonnegative"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "least-squares-linear-bounded",
            grouped_paths: &["least_squares::linear::bounded"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "least-squares-linear-constrained",
            grouped_paths: &["least_squares::linear::constrained"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "least-squares-linear-bounded-constrained",
            grouped_paths: &["least_squares::linear::bounded_constrained"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "ode-sdrive-expert",
            grouped_paths: &["differential_equations::ode::sdrive"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "dassl",
            grouped_paths: &["differential_equations::dae::dassl::residual"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "optimization-linear-programming-in-memory",
            grouped_paths: &["optimization::linear_programming"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "fftpack-real",
            grouped_paths: &["transforms::fft::real"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "fftpack-complex",
            grouped_paths: &["transforms::fft::complex"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "pchip",
            grouped_paths: &["interpolation::pchip"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "bspline",
            grouped_paths: &["interpolation::bspline"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
        FeatureRecord {
            cargo_feature: "piecewise-polynomial",
            grouped_paths: &["interpolation::piecewise_polynomial"],
            evidence_source: "crates/slatec/Cargo.toml",
        },
    ]
}

/// Generates deterministic permanent-module roadmap metadata.
pub fn generate(output_dir: &Path) -> Result<GenerationResult> {
    let functions = read_json("generated/safe-api/function-index.json")?;
    let capability = read_json("generated/safe-api/capability-summary.json")?;
    let special = read_json("generated/safe-api/special-wrapper-index.json")?;
    let function_paths = functions["records"]
        .as_array()
        .ok_or_else(|| policy("function-index records"))?
        .iter()
        .filter_map(|record| record.get("rust_path").and_then(Value::as_str))
        .collect::<Vec<_>>();
    let safe_function_count = functions["records"]
        .as_array()
        .ok_or_else(|| policy("function-index records"))?
        .len();
    if safe_function_count != SAFE_FUNCTION_TARGET {
        return Err(policy(
            "safe-function count changed during documentation-only milestone",
        ));
    }
    let capability_counts = capability["capabilities"]
        .as_object()
        .ok_or_else(|| policy("capability counts"))?
        .iter()
        .map(|(key, value)| {
            Ok((
                key.clone(),
                value.as_u64().ok_or_else(|| policy(key))? as usize,
            ))
        })
        .collect::<Result<BTreeMap<_, _>>>()?;
    if capability_counts
        != BTreeMap::from([
            ("alloc".to_owned(), 2),
            ("core".to_owned(), 58),
            ("std".to_owned(), 178),
        ])
    {
        return Err(policy(
            "capability counts changed during documentation-only milestone",
        ));
    }

    let leaves = leaves();
    validate_leaves(&leaves, &function_paths)?;
    let records = leaves.iter().map(record).collect::<Vec<_>>();
    let status_totals = totals(records.iter().map(|record| record.status));
    let mut stability_totals = totals(records.iter().map(|record| record.stability));
    stability_totals.insert("Frozen".to_owned(), FROZEN_HIGH_LEVEL_PATHS.len());
    let feature_records = features();
    validate_features(&feature_records, &leaves)?;
    validate_special_mappings(&special)?;

    fs::create_dir_all(output_dir)?;
    let roadmap = RoadmapFile {
        schema_id: "slatec.safe-api.public-module-roadmap",
        schema_version: SCHEMA_VERSION,
        safe_function_count,
        capability_counts: capability_counts.clone(),
        status_totals: status_totals.clone(),
        stability_totals: stability_totals.clone(),
        frozen_high_level_paths: FROZEN_HIGH_LEVEL_PATHS,
        records: records.clone(),
    };
    let canonical = serde_json::to_vec(&roadmap)?;
    let semantic_hash = hash::bytes(&canonical);
    write_json(&output_dir.join("public-module-roadmap.json"), &roadmap)?;
    write_json(
        &output_dir.join("public-module-status.json"),
        &MetadataFile {
            schema_id: "slatec.safe-api.public-module-status",
            schema_version: SCHEMA_VERSION,
            records: records.clone(),
        },
    )?;
    write_json(
        &output_dir.join("public-module-compatibility.json"),
        &MetadataFile {
            schema_id: "slatec.safe-api.public-module-compatibility",
            schema_version: SCHEMA_VERSION,
            records: records
                .iter()
                .filter(|record| record.compatibility_path.is_some())
                .cloned()
                .collect::<Vec<_>>(),
        },
    )?;
    write_json(
        &output_dir.join("public-module-feature-map.json"),
        &MetadataFile {
            schema_id: "slatec.safe-api.public-module-feature-map",
            schema_version: SCHEMA_VERSION,
            records: feature_records,
        },
    )?;
    fs::write(
        output_dir.join("public-module-tree.md"),
        render_tree(&records, &status_totals, &stability_totals),
    )?;
    write_json(
        &output_dir.join("public-module-summary.json"),
        &serde_json::json!({
            "schema_id": "slatec.safe-api.public-module-summary",
            "schema_version": SCHEMA_VERSION,
            "safe_function_count": safe_function_count,
            "capability_counts": capability_counts,
            "status_totals": status_totals,
            "stability_totals": stability_totals,
            "frozen_high_level_paths": FROZEN_HIGH_LEVEL_PATHS,
            "semantic_hash": semantic_hash,
        }),
    )?;
    Ok(GenerationResult {
        safe_function_count,
        semantic_hash,
    })
}

fn record(spec: &LeafSpec) -> LeafRecord {
    LeafRecord {
        canonical_grouped_module_path: spec.path,
        module_file: spec.module_file,
        stability: spec.stability,
        status: spec.status,
        cargo_feature: spec.feature,
        current_implementation_module: spec.implementation_module,
        compatibility_path: spec.compatibility_path,
        safe_native_families: spec.native_families.to_vec(),
        precision_coverage: spec.precision,
        runtime_level: spec.runtime,
        concurrency_classification: spec.concurrency,
        status_rationale: spec.rationale,
        blocker: spec.blocker,
        likely_next_milestone: spec.next_milestone,
        evidence_source: spec.evidence_source,
    }
}

fn totals<'a>(values: impl Iterator<Item = &'a str>) -> BTreeMap<String, usize> {
    let mut totals = BTreeMap::new();
    for value in values {
        *totals.entry(value.to_owned()).or_insert(0) += 1;
    }
    totals
}

fn validate_leaves(leaves: &[LeafSpec], function_paths: &[&str]) -> Result<()> {
    let mut paths = std::collections::BTreeSet::new();
    for leaf in leaves {
        if !paths.insert(leaf.path) {
            return Err(policy("duplicate canonical grouped module path"));
        }
        if !workspace_path(leaf.module_file).is_file() {
            return Err(policy("roadmap module file is absent"));
        }
        if leaf.status == "Implemented"
            && !leaf
                .item_prefixes
                .iter()
                .all(|prefix| function_paths.iter().any(|path| path.starts_with(prefix)))
        {
            return Err(policy(
                "implemented roadmap leaf lacks a current public item",
            ));
        }
    }
    Ok(())
}

fn validate_features(features: &[FeatureRecord], leaves: &[LeafSpec]) -> Result<()> {
    for mapping in features {
        if mapping.grouped_paths.is_empty()
            || !mapping
                .grouped_paths
                .iter()
                .all(|path| leaves.iter().any(|leaf| leaf.path == *path))
        {
            return Err(policy("feature mapping has no roadmap path"));
        }
    }
    for feature in CURRENT_FAMILY_FEATURES {
        if !features
            .iter()
            .any(|mapping| mapping.cargo_feature == *feature)
        {
            return Err(policy("current family feature lacks a roadmap mapping"));
        }
    }
    let scalar = features
        .iter()
        .find(|mapping| mapping.cargo_feature == "special-scalar-expanded")
        .ok_or_else(|| policy("special-scalar-expanded mapping"))?;
    if scalar
        .grouped_paths
        .iter()
        .any(|path| path.contains("scalar_expanded"))
    {
        return Err(policy(
            "implementation feature leaked into grouped namespace",
        ));
    }
    Ok(())
}

fn validate_special_mappings(special: &Value) -> Result<()> {
    let rendered = serde_json::to_string(special)?;
    for path in [
        "slatec::special::scalar_expanded::carlson_rc",
        "slatec::special::scalar_expanded::carlson_rf",
        "slatec::special::scalar_expanded::carlson_rd",
        "slatec::special::scalar_expanded::carlson_rj",
        "slatec::special::scalar_expanded::logarithmic_integral",
        "slatec::special::scalar_expanded::spence_integral",
    ] {
        if !rendered.contains(path) {
            return Err(policy(
                "reviewed special-expanded wrapper is absent from metadata",
            ));
        }
    }
    Ok(())
}

fn render_tree(
    records: &[LeafRecord],
    status_totals: &BTreeMap<String, usize>,
    stability_totals: &BTreeMap<String, usize>,
) -> String {
    let mut text = String::from(
        "# Permanent safe-API module tree\n\nThis generated roadmap distinguishes documentation placeholders from callable safe APIs. Existing paths remain supported compatibility paths.\n\n## Totals\n\n",
    );
    text.push_str("| Status | Leaves |\n| --- | ---: |\n");
    for status in [
        "Implemented",
        "Partial",
        "Planned",
        "Deferred",
        "Unavailable",
    ] {
        text.push_str(&format!(
            "| {status} | {} |\n",
            status_totals.get(status).copied().unwrap_or(0)
        ));
    }
    text.push_str("\n| Stability | Leaves |\n| --- | ---: |\n");
    for stability in ["Frozen", "Reserved", "Provisional"] {
        text.push_str(&format!(
            "| {stability} | {} |\n",
            stability_totals.get(stability).copied().unwrap_or(0)
        ));
    }
    text.push_str("\n## Leaf status\n\n| Domain | Module | Stability | Status | Safe API | Cargo feature | Precision | Runtime | Concurrency | Compatibility path | Blocker | Next milestone |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for record in records {
        let domain = record
            .canonical_grouped_module_path
            .split("::")
            .next()
            .unwrap_or(record.canonical_grouped_module_path);
        text.push_str(&format!(
            "| {domain} | `{}` | {} | {} | {} | `{}` | {} | {} | {} | `{}` | {} | {} |\n",
            record.canonical_grouped_module_path,
            record.stability,
            record.status,
            if record.status == "Implemented" || record.status == "Partial" {
                "yes"
            } else {
                "no"
            },
            record.cargo_feature.unwrap_or(""),
            record.precision_coverage,
            record.runtime_level,
            record.concurrency_classification,
            record.compatibility_path.unwrap_or(""),
            record.blocker,
            record.likely_next_milestone,
        ));
    }
    text
}

fn read_json(path: &str) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(workspace_path(path))?)?)
}

fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}

fn policy(message: &str) -> CorpusError {
    CorpusError::Policy(format!("public module roadmap: {message}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic_and_preserves_current_surface() {
        let first = tempfile::tempdir().unwrap();
        let second = tempfile::tempdir().unwrap();
        let a = generate(first.path()).unwrap();
        let b = generate(second.path()).unwrap();
        assert_eq!(a.safe_function_count, SAFE_FUNCTION_TARGET);
        assert_eq!(a.semantic_hash, b.semantic_hash);
        for file in [
            "public-module-roadmap.json",
            "public-module-status.json",
            "public-module-compatibility.json",
            "public-module-feature-map.json",
            "public-module-tree.md",
            "public-module-summary.json",
        ] {
            assert_eq!(
                fs::read(first.path().join(file)).unwrap(),
                fs::read(second.path().join(file)).unwrap()
            );
        }
    }

    #[test]
    fn special_expanded_paths_are_mathematical() {
        let mappings = features();
        let scalar = mappings
            .iter()
            .find(|mapping| mapping.cargo_feature == "special-scalar-expanded")
            .unwrap();
        assert!(scalar.grouped_paths.contains(&"special::elliptic::carlson"));
        assert!(
            scalar
                .grouped_paths
                .contains(&"special::integrals::logarithmic")
        );
        assert!(scalar.grouped_paths.contains(&"special::integrals::spence"));
        assert!(
            !scalar
                .grouped_paths
                .iter()
                .any(|path| path.contains("scalar_expanded"))
        );
    }
}
