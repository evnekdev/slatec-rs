//! Generated native link coverage for every reviewed real Airy symbol.
//! Run on the supported GNU MinGW target with `special-airy`.

#![cfg(all(
    feature = "special-airy",
    target_arch = "x86_64",
    target_env = "gnu",
    target_os = "windows"
))]

#[test]
fn every_reviewed_real_airy_symbol_links_from_the_airy_provider_closure() {
    slatec_src::ensure_linked();
    std::hint::black_box(slatec_sys::special::airy::ai as *const () as usize);
    std::hint::black_box(slatec_sys::special::airy::aie as *const () as usize);
    std::hint::black_box(slatec_sys::special::airy::bi as *const () as usize);
    std::hint::black_box(slatec_sys::special::airy::bie as *const () as usize);
    std::hint::black_box(slatec_sys::special::airy::dai as *const () as usize);
    std::hint::black_box(slatec_sys::special::airy::daie as *const () as usize);
    std::hint::black_box(slatec_sys::special::airy::dbi as *const () as usize);
    std::hint::black_box(slatec_sys::special::airy::dbie as *const () as usize);
}
