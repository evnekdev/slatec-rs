//! Compile-only coverage for canonical raw API paths.

#[cfg(feature = "raw-family-roots-scalar")]
#[test]
fn roots_scalar_canonical_paths_compile() {
    let _ = slatec_sys::roots::scalar::fzero;
    let _ = slatec_sys::roots::scalar::dfzero;
}

#[cfg(feature = "raw-family-fishpack-cartesian-2d")]
#[test]
fn fishpack_hwscrt_canonical_path_compiles() {
    let _ = slatec_sys::pde::fishpack::hwscrt;
}

#[cfg(feature = "raw-family-fishpack-pois3d")]
#[test]
fn fishpack_pois3d_canonical_path_compiles() {
    let _ = slatec_sys::pde::fishpack::pois3d;
}
