//! Compile-only samples for every public mathematical family selected by `all`.

#![cfg(feature = "all")]

#[test]
fn all_reaches_each_public_mathematical_family() {
    let _ = slatec_sys::blas::level1::daxpy;
    let _ = slatec_sys::special::elementary::dlnrel;
    let _ = slatec_sys::special::gamma::dgamma;
    let _ = slatec_sys::special::beta::dbeta;
    let _ = slatec_sys::special::error::derf;
    let _ = slatec_sys::quadrature::dqag;
    let _ = slatec_sys::roots::dfzero;
    let _ = slatec_sys::nonlinear::dnsqe;
    let _ = slatec_sys::least_squares::dnls1;
    let _ = slatec_sys::linear_least_squares::dwnnls;
    let _ = slatec_sys::ode::ddriv3;
    let _ = slatec_sys::dassl::ddassl;
    let _ = slatec_sys::linear_programming::dsplp;
    let _ = slatec_sys::fftpack::rffti;
    let _ = slatec_sys::fftpack_complex::cffti1;
    let _ = slatec_sys::pde::fishpack::hwscrt;
    let _ = slatec_sys::pde::fishpack::pois3d;
    let _ = slatec_sys::banded::dgbfa;
    let _ = slatec_sys::pchip::dpchim;
    let _ = slatec_sys::bspline::dbvalu;
    let _ = slatec_sys::piecewise_polynomial::dppval;
}
