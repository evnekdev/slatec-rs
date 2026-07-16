# BLAS Level 1 safe API validation summary

- Safe API semantic version: `1.0.0`
- Raw ABI profile: `ffi-profile-gnu-mingw-x86_64`
- Callable wrapper families: copy, swap, scale, AXPY, dot, norm, absolute sum, index maximum, rotation
- Precisions: `f32`, `f64`
- Wrappers: 18, each with contiguous and checked-strided entry points
- Source-only validation: enabled by `blas-level1-validation`; it does not link native Fortran
- Native validation: `blas-level1-native-tests`, explicitly gated on `x86_64-pc-windows-gnu` and local `SLATEC_NATIVE_LIB_DIR`
- Raw source, static libraries, object files, compiler logs, and test traces remain ignored

This summary contains no copied Fortran source or generated binary data.
