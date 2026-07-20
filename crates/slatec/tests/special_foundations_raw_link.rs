//! Generated native link retention coverage for every reviewed R2B special symbol.
//! Run on the supported GNU MinGW target with `special-raw-link-tests`.

#![cfg(all(
    feature = "special-raw-link-tests",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

#[test]
fn every_reviewed_special_foundation_symbol_links_from_its_provider_closure() {
    slatec_src::ensure_linked();
    // elementary
    std::hint::black_box(slatec_sys::special::elementary::alnrel as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::cbrt as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::cosdg as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::daws as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::dcbrt as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::dcosdg as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::ddaws as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::dexprl as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::dlnrel as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::dsindg as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::exprel as *const () as usize);
    std::hint::black_box(slatec_sys::special::elementary::sindg as *const () as usize);
    // gamma
    std::hint::black_box(slatec_sys::special::gamma::alngam as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::binom as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dbinom as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dfac as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dgami as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dgamic as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dgamit as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dgamma as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dgamr as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dlngam as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::dpsi as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::fac as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::gami as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::gamic as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::gamit as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::gamma as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::gamr as *const () as usize);
    std::hint::black_box(slatec_sys::special::gamma::psi as *const () as usize);
    // beta
    std::hint::black_box(slatec_sys::special::beta::albeta as *const () as usize);
    std::hint::black_box(slatec_sys::special::beta::beta as *const () as usize);
    std::hint::black_box(slatec_sys::special::beta::betai as *const () as usize);
    std::hint::black_box(slatec_sys::special::beta::dbeta as *const () as usize);
    std::hint::black_box(slatec_sys::special::beta::dbetai as *const () as usize);
    std::hint::black_box(slatec_sys::special::beta::dlbeta as *const () as usize);
    // error
    std::hint::black_box(slatec_sys::special::error::derf as *const () as usize);
    std::hint::black_box(slatec_sys::special::error::derfc as *const () as usize);
    std::hint::black_box(slatec_sys::special::error::erf as *const () as usize);
    std::hint::black_box(slatec_sys::special::error::erfc as *const () as usize);
}
