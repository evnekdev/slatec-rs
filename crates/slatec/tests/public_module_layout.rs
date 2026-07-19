//! Compile-time public-module layout checks.

#[allow(unused_imports)]
use slatec::{
    differential_equations, equations, integration, interpolation, linear_algebra, optimization,
    roadmap, special, transforms,
};

#[test]
fn permanent_structural_domains_are_visible_without_numerical_features() {
    let _ = core::mem::size_of::<usize>();
}

#[cfg(feature = "special-scalar-expanded")]
#[allow(dead_code)]
fn expanded_special_functions_are_reachable_by_mathematical_paths() {
    let _ = slatec::special::integrals::logarithmic::logarithmic_integral
        as fn(f64) -> Result<f64, slatec::special::SpecialFunctionError>;
    let _ = slatec::special::integrals::spence::spence_integral
        as fn(f64) -> Result<f64, slatec::special::SpecialFunctionError>;
    let _ = slatec::special::elliptic::carlson::carlson_rc
        as fn(f64, f64) -> Result<f64, slatec::special::SpecialFunctionError>;
}

#[cfg(feature = "blas-level1")]
#[allow(dead_code)]
fn blas_compatibility_and_grouped_paths_compile() {
    use slatec::blas::level1 as compatibility;
    use slatec::linear_algebra::blas::level1 as grouped;
    let _ =
        compatibility::daxpy as fn(f64, &[f64], &mut [f64]) -> Result<(), slatec::blas::BlasError>;
    let _ = grouped::daxpy as fn(f64, &[f64], &mut [f64]) -> Result<(), slatec::blas::BlasError>;
}
