//! Compile with `cargo check -p slatec --example raw_all_features_compile --features raw-all-compile`.
//!
//! `slatec-sys/all` enables declarations for every public mathematical family.
//! It intentionally selects no source-build, system, or external provider.

fn main() {
    let _ = slatec_sys::blas::level1::daxpy;
    let _ = slatec_sys::special::gamma::dgamma;
    let _ = slatec_sys::quadrature::dqag;
    let _ = slatec_sys::pde::fishpack::pois3d;
}
