//! Generated canonical import coverage for reviewed special foundations.
//! Regenerate with `slatec-corpus generate-raw-api-inventory --offline`.

#[cfg(feature = "special-elementary")]
#[test]
fn elementary_canonical_paths_compile() {
    let _ = slatec_sys::special::elementary::alnrel;
    let _ = slatec_sys::special::elementary::cbrt;
    let _ = slatec_sys::special::elementary::cosdg;
    let _ = slatec_sys::special::elementary::daws;
    let _ = slatec_sys::special::elementary::dcbrt;
    let _ = slatec_sys::special::elementary::dcosdg;
    let _ = slatec_sys::special::elementary::ddaws;
    let _ = slatec_sys::special::elementary::dexprl;
    let _ = slatec_sys::special::elementary::dlnrel;
    let _ = slatec_sys::special::elementary::dsindg;
    let _ = slatec_sys::special::elementary::exprel;
    let _ = slatec_sys::special::elementary::sindg;
}

#[cfg(feature = "special-gamma")]
#[test]
fn gamma_canonical_paths_compile() {
    let _ = slatec_sys::special::gamma::alngam;
    let _ = slatec_sys::special::gamma::binom;
    let _ = slatec_sys::special::gamma::dbinom;
    let _ = slatec_sys::special::gamma::dfac;
    let _ = slatec_sys::special::gamma::dgami;
    let _ = slatec_sys::special::gamma::dgamic;
    let _ = slatec_sys::special::gamma::dgamit;
    let _ = slatec_sys::special::gamma::dgamma;
    let _ = slatec_sys::special::gamma::dgamr;
    let _ = slatec_sys::special::gamma::dlngam;
    let _ = slatec_sys::special::gamma::dpsi;
    let _ = slatec_sys::special::gamma::fac;
    let _ = slatec_sys::special::gamma::gami;
    let _ = slatec_sys::special::gamma::gamic;
    let _ = slatec_sys::special::gamma::gamit;
    let _ = slatec_sys::special::gamma::gamma;
    let _ = slatec_sys::special::gamma::gamr;
    let _ = slatec_sys::special::gamma::psi;
}

#[cfg(feature = "special-beta")]
#[test]
fn beta_canonical_paths_compile() {
    let _ = slatec_sys::special::beta::albeta;
    let _ = slatec_sys::special::beta::beta;
    let _ = slatec_sys::special::beta::betai;
    let _ = slatec_sys::special::beta::dbeta;
    let _ = slatec_sys::special::beta::dbetai;
    let _ = slatec_sys::special::beta::dlbeta;
}

#[cfg(feature = "special-error")]
#[test]
fn error_canonical_paths_compile() {
    let _ = slatec_sys::special::error::derf;
    let _ = slatec_sys::special::error::derfc;
    let _ = slatec_sys::special::error::erf;
    let _ = slatec_sys::special::error::erfc;
}
