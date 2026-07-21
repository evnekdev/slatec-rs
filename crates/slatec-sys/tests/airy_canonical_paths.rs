//! Generated canonical import coverage for reviewed real Airy drivers.
//! Regenerate with `slatec-corpus generate-raw-api-inventory --offline`.

#[cfg(feature = "special-airy")]
#[test]
fn real_airy_canonical_paths_compile() {
    let _ = slatec_sys::special::airy::ai;
    let _ = slatec_sys::special::airy::aie;
    let _ = slatec_sys::special::airy::bi;
    let _ = slatec_sys::special::airy::bie;
    let _ = slatec_sys::special::airy::dai;
    let _ = slatec_sys::special::airy::daie;
    let _ = slatec_sys::special::airy::dbi;
    let _ = slatec_sys::special::airy::dbie;
}
