//! Compile-only coverage for canonical raw API paths and compatibility aliases.

#[cfg(feature = "raw-family-roots-scalar")]
#[test]
fn roots_scalar_canonical_and_compatibility_paths_compile() {
    let _ = slatec_sys::roots::scalar::fzero;
    let _ = slatec_sys::roots::scalar::dfzero;
    let _ = slatec_sys::roots::fzero;
    let _ = slatec_sys::roots::dfzero;
}

#[cfg(feature = "raw-family-fishpack-cartesian-2d")]
#[test]
fn fishpack_hwscrt_canonical_and_compatibility_paths_compile() {
    let _ = slatec_sys::pde::fishpack::hwscrt;
    let _ = slatec_sys::fishpack_cartesian_2d::hwscrt;
}

#[cfg(feature = "raw-family-fishpack-pois3d")]
#[test]
fn fishpack_pois3d_canonical_and_compatibility_paths_compile() {
    let _ = slatec_sys::pde::fishpack::pois3d;
    let _ = slatec_sys::fishpack_pois3d::pois3d;
}
