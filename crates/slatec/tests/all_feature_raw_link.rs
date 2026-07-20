//! Aggregate declaration/provider composition probe for `slatec-sys/all`.
//!
//! This retains a representative canonical symbol from every public family so
//! the supported source provider must resolve the aggregate feature closure.

#![cfg(all(
    feature = "raw-all-link-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

#[test]
fn all_feature_representative_symbols_link_from_the_full_provider() {
    slatec_src::ensure_linked();
    std::hint::black_box(slatec_sys::blas::level1::daxpy as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::dlnrel as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dgamma as *const () as usize);
    std::hint::black_box(slatec_sys::special::beta::dbeta as *const () as usize);
    std::hint::black_box(slatec_sys::special::error::derf as *const () as usize);
    std::hint::black_box(slatec_sys::quadrature::dqag as *const () as usize);
    std::hint::black_box(slatec_sys::roots::dfzero as *const () as usize);
    std::hint::black_box(slatec_sys::nonlinear::dnsqe as *const () as usize);
    std::hint::black_box(slatec_sys::least_squares::dnls1 as *const () as usize);
    std::hint::black_box(slatec_sys::linear_least_squares::dwnnls as *const () as usize);
    std::hint::black_box(slatec_sys::ode::ddriv3 as *const () as usize);
    std::hint::black_box(slatec_sys::dassl::ddassl as *const () as usize);
    std::hint::black_box(slatec_sys::linear_programming::dsplp as *const () as usize);
    std::hint::black_box(slatec_sys::fftpack::rffti as *const () as usize);
    std::hint::black_box(slatec_sys::fftpack_complex::cffti1 as *const () as usize);
    std::hint::black_box(slatec_sys::pde::fishpack::hwscrt as *const () as usize);
    std::hint::black_box(slatec_sys::pde::fishpack::pois3d as *const () as usize);
    std::hint::black_box(slatec_sys::banded::dgbfa as *const () as usize);
    std::hint::black_box(slatec_sys::pchip::pchim as *const () as usize);
    std::hint::black_box(slatec_sys::bspline::dbvalu as *const () as usize);
    std::hint::black_box(slatec_sys::piecewise_polynomial::dppval as *const () as usize);
}
