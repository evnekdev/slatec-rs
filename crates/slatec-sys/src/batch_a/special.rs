//! Generated Batch A canonical raw re-exports.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-a --offline`.

/// Batch A canonical `special` declarations.
pub mod numerical {
    // raw-api-routine: ACOSH
    /// Compute the arc hyperbolic cosine.
    ///
    /// Original SLATEC routine: `ACOSH`; source: <https://www.netlib.org/slatec/fnlib/acosh.f>. Native symbol: `acosh_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::acosh;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the arc hyperbolic cosine.
        ///
        /// Original SLATEC routine: `ACOSH`; source: <https://www.netlib.org/slatec/fnlib/acosh.f>. Native symbol: `acosh_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "acosh_"]
        pub fn acosh(x: *mut f32) -> f32;
    }

    // raw-api-routine: ALGAMS
    /// Compute the logarithm of the absolute value of the Gamma function.
    ///
    /// Original SLATEC routine: `ALGAMS`; source: <https://www.netlib.org/slatec/fnlib/algams.f>. Native symbol: `algams_`.
    /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALGAM`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SGNGAM`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_scalar_subroutines::algams;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the logarithm of the absolute value of the Gamma function.
        ///
        /// Original SLATEC routine: `ALGAMS`; source: <https://www.netlib.org/slatec/fnlib/algams.f>. Native symbol: `algams_`.
        /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALGAM`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SGNGAM`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "algams_"]
        pub fn algams(x: *mut f32, algam: *mut f32, sgngam: *mut f32);
    }

    // raw-api-routine: ALI
    /// Compute the logarithmic integral.
    ///
    /// Original SLATEC routine: `ALI`; source: <https://www.netlib.org/slatec/fnlib/ali.f>. Native symbol: `ali_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::ali;

    // raw-api-routine: ASINH
    /// Compute the arc hyperbolic sine.
    ///
    /// Original SLATEC routine: `ASINH`; source: <https://www.netlib.org/slatec/fnlib/asinh.f>. Native symbol: `asinh_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::asinh;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the arc hyperbolic sine.
        ///
        /// Original SLATEC routine: `ASINH`; source: <https://www.netlib.org/slatec/fnlib/asinh.f>. Native symbol: `asinh_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "asinh_"]
        pub fn asinh(x: *mut f32) -> f32;
    }

    // raw-api-routine: ATANH
    /// Compute the arc hyperbolic tangent.
    ///
    /// Original SLATEC routine: `ATANH`; source: <https://www.netlib.org/slatec/fnlib/atanh.f>. Native symbol: `atanh_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::atanh;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the arc hyperbolic tangent.
        ///
        /// Original SLATEC routine: `ATANH`; source: <https://www.netlib.org/slatec/fnlib/atanh.f>. Native symbol: `atanh_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "atanh_"]
        pub fn atanh(x: *mut f32) -> f32;
    }

    // raw-api-routine: BESI
    /// Compute an N member sequence of I Bessel functions I/SUB(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.
    ///
    /// Original SLATEC routine: `BESI`; source: <https://www.netlib.org/slatec/src/besi.f>. Native symbol: `besi_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALPHA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::besi;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute an N member sequence of I Bessel functions I/SUB(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.
        ///
        /// Original SLATEC routine: `BESI`; source: <https://www.netlib.org/slatec/src/besi.f>. Native symbol: `besi_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALPHA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "besi_"]
        pub fn besi(
            x: *mut f32,
            alpha: *mut f32,
            kode: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            y: *mut f32,
            nz: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: BESI0
    /// Compute the hyperbolic Bessel function of the first kind of order zero.
    ///
    /// Original SLATEC routine: `BESI0`; source: <https://www.netlib.org/slatec/fnlib/besi0.f>. Native symbol: `besi0_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besi0;

    // raw-api-routine: BESI0E
    /// Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero.
    ///
    /// Original SLATEC routine: `BESI0E`; source: <https://www.netlib.org/slatec/fnlib/besi0e.f>. Native symbol: `besi0e_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besi0e;

    // raw-api-routine: BESI1
    /// Compute the modified (hyperbolic) Bessel function of the first kind of order one.
    ///
    /// Original SLATEC routine: `BESI1`; source: <https://www.netlib.org/slatec/fnlib/besi1.f>. Native symbol: `besi1_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besi1;

    // raw-api-routine: BESI1E
    /// Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one.
    ///
    /// Original SLATEC routine: `BESI1E`; source: <https://www.netlib.org/slatec/fnlib/besi1e.f>. Native symbol: `besi1e_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besi1e;

    // raw-api-routine: BESJ
    /// Compute an N member sequence of J Bessel functions J/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.
    ///
    /// Original SLATEC routine: `BESJ`; source: <https://www.netlib.org/slatec/src/besj.f>. Native symbol: `besj_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALPHA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::besj;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute an N member sequence of J Bessel functions J/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.
        ///
        /// Original SLATEC routine: `BESJ`; source: <https://www.netlib.org/slatec/src/besj.f>. Native symbol: `besj_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALPHA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "besj_"]
        pub fn besj(
            x: *mut f32,
            alpha: *mut f32,
            n: *mut crate::FortranInteger,
            y: *mut f32,
            nz: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: BESJ0
    /// Compute the Bessel function of the first kind of order zero.
    ///
    /// Original SLATEC routine: `BESJ0`; source: <https://www.netlib.org/slatec/fnlib/besj0.f>. Native symbol: `besj0_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besj0;

    // raw-api-routine: BESJ1
    /// Compute the Bessel function of the first kind of order one.
    ///
    /// Original SLATEC routine: `BESJ1`; source: <https://www.netlib.org/slatec/fnlib/besj1.f>. Native symbol: `besj1_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besj1;

    // raw-api-routine: BESK
    /// Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/SUB(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.
    ///
    /// Original SLATEC routine: `BESK`; source: <https://www.netlib.org/slatec/src/besk.f>. Native symbol: `besk_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::besk;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/SUB(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.
        ///
        /// Original SLATEC routine: `BESK`; source: <https://www.netlib.org/slatec/src/besk.f>. Native symbol: `besk_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "besk_"]
        pub fn besk(
            x: *mut f32,
            fnu: *mut f32,
            kode: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            y: *mut f32,
            nz: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: BESK0
    /// Compute the modified (hyperbolic) Bessel function of the third kind of order zero.
    ///
    /// Original SLATEC routine: `BESK0`; source: <https://www.netlib.org/slatec/fnlib/besk0.f>. Native symbol: `besk0_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besk0;

    // raw-api-routine: BESK0E
    /// Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order zero.
    ///
    /// Original SLATEC routine: `BESK0E`; source: <https://www.netlib.org/slatec/fnlib/besk0e.f>. Native symbol: `besk0e_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besk0e;

    // raw-api-routine: BESK1
    /// Compute the modified (hyperbolic) Bessel function of the third kind of order one.
    ///
    /// Original SLATEC routine: `BESK1`; source: <https://www.netlib.org/slatec/fnlib/besk1.f>. Native symbol: `besk1_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besk1;

    // raw-api-routine: BESK1E
    /// Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order one.
    ///
    /// Original SLATEC routine: `BESK1E`; source: <https://www.netlib.org/slatec/fnlib/besk1e.f>. Native symbol: `besk1e_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besk1e;

    // raw-api-routine: BESKES
    /// Compute a sequence of exponentially scaled modified Bessel functions of the third kind of fractional order.
    ///
    /// Original SLATEC routine: `BESKES`; source: <https://www.netlib.org/slatec/fnlib/beskes.f>. Native symbol: `beskes_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `XNU`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NIN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BKE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::beskes;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of exponentially scaled modified Bessel functions of the third kind of fractional order.
        ///
        /// Original SLATEC routine: `BESKES`; source: <https://www.netlib.org/slatec/fnlib/beskes.f>. Native symbol: `beskes_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `XNU`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NIN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BKE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "beskes_"]
        pub fn beskes(xnu: *mut f32, x: *mut f32, nin: *mut crate::FortranInteger, bke: *mut f32);
    }

    // raw-api-routine: BESKS
    /// Compute a sequence of modified Bessel functions of the third kind of fractional order.
    ///
    /// Original SLATEC routine: `BESKS`; source: <https://www.netlib.org/slatec/fnlib/besks.f>. Native symbol: `besks_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `XNU`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NIN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::besks;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of modified Bessel functions of the third kind of fractional order.
        ///
        /// Original SLATEC routine: `BESKS`; source: <https://www.netlib.org/slatec/fnlib/besks.f>. Native symbol: `besks_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `XNU`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NIN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "besks_"]
        pub fn besks(xnu: *mut f32, x: *mut f32, nin: *mut crate::FortranInteger, bk: *mut f32);
    }

    // raw-api-routine: BESY
    /// Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions Y/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.
    ///
    /// Original SLATEC routine: `BESY`; source: <https://www.netlib.org/slatec/src/besy.f>. Native symbol: `besy_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::besy;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions Y/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.
        ///
        /// Original SLATEC routine: `BESY`; source: <https://www.netlib.org/slatec/src/besy.f>. Native symbol: `besy_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "besy_"]
        pub fn besy(x: *mut f32, fnu: *mut f32, n: *mut crate::FortranInteger, y: *mut f32);
    }

    // raw-api-routine: BESY0
    /// Compute the Bessel function of the second kind of order zero.
    ///
    /// Original SLATEC routine: `BESY0`; source: <https://www.netlib.org/slatec/fnlib/besy0.f>. Native symbol: `besy0_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besy0;

    // raw-api-routine: BESY1
    /// Compute the Bessel function of the second kind of order one.
    ///
    /// Original SLATEC routine: `BESY1`; source: <https://www.netlib.org/slatec/fnlib/besy1.f>. Native symbol: `besy1_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::besy1;

    // raw-api-routine: BSKIN
    /// Compute repeated integrals of the K-zero Bessel function.
    ///
    /// Original SLATEC routine: `BSKIN`; source: <https://www.netlib.org/slatec/src/bskin.f>. Native symbol: `bskin_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::bskin;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute repeated integrals of the K-zero Bessel function.
        ///
        /// Original SLATEC routine: `BSKIN`; source: <https://www.netlib.org/slatec/src/bskin.f>. Native symbol: `bskin_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "bskin_"]
        pub fn bskin(
            x: *mut f32,
            n: *mut crate::FortranInteger,
            kode: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            y: *mut f32,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: CHU
    /// Compute the logarithmic confluent hypergeometric function.
    ///
    /// Original SLATEC routine: `CHU`; source: <https://www.netlib.org/slatec/fnlib/chu.f>. Native symbol: `chu_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32,mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::chu;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the logarithmic confluent hypergeometric function.
        ///
        /// Original SLATEC routine: `CHU`; source: <https://www.netlib.org/slatec/fnlib/chu.f>. Native symbol: `chu_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32,mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "chu_"]
        pub fn chu(a: *mut f32, b: *mut f32, x: *mut f32) -> f32;
    }

    // raw-api-routine: COT
    /// Compute the cotangent.
    ///
    /// Original SLATEC routine: `COT`; source: <https://www.netlib.org/slatec/fnlib/cot.f>. Native symbol: `cot_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::cot;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the cotangent.
        ///
        /// Original SLATEC routine: `COT`; source: <https://www.netlib.org/slatec/fnlib/cot.f>. Native symbol: `cot_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "cot_"]
        pub fn cot(x: *mut f32) -> f32;
    }

    // raw-api-routine: CSEVL
    /// Evaluate a Chebyshev series.
    ///
    /// Original SLATEC routine: `CSEVL`; source: <https://www.netlib.org/slatec/fnlib/csevl.f>. Native symbol: `csevl_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CS`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_polynomials::csevl;

    // raw-api-routine: DACOSH
    /// Compute the arc hyperbolic cosine.
    ///
    /// Original SLATEC routine: `DACOSH`; source: <https://www.netlib.org/slatec/fnlib/dacosh.f>. Native symbol: `dacosh_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::dacosh;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the arc hyperbolic cosine.
        ///
        /// Original SLATEC routine: `DACOSH`; source: <https://www.netlib.org/slatec/fnlib/dacosh.f>. Native symbol: `dacosh_`.
        /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `DOUBLE PRECISION` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dacosh_"]
        pub fn dacosh(x: *mut f64) -> f64;
    }

    // raw-api-routine: DASINH
    /// Compute the arc hyperbolic sine.
    ///
    /// Original SLATEC routine: `DASINH`; source: <https://www.netlib.org/slatec/fnlib/dasinh.f>. Native symbol: `dasinh_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::dasinh;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the arc hyperbolic sine.
        ///
        /// Original SLATEC routine: `DASINH`; source: <https://www.netlib.org/slatec/fnlib/dasinh.f>. Native symbol: `dasinh_`.
        /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `DOUBLE PRECISION` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dasinh_"]
        pub fn dasinh(x: *mut f64) -> f64;
    }

    // raw-api-routine: DATANH
    /// Compute the arc hyperbolic tangent.
    ///
    /// Original SLATEC routine: `DATANH`; source: <https://www.netlib.org/slatec/fnlib/datanh.f>. Native symbol: `datanh_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::datanh;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the arc hyperbolic tangent.
        ///
        /// Original SLATEC routine: `DATANH`; source: <https://www.netlib.org/slatec/fnlib/datanh.f>. Native symbol: `datanh_`.
        /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `DOUBLE PRECISION` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "datanh_"]
        pub fn datanh(x: *mut f64) -> f64;
    }

    // raw-api-routine: DBESI
    /// Compute an N member sequence of I Bessel functions I/SUB(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/SUB(ALPHA+K-1)/(X), K=1,...,N for nonnegative ALPHA and X.
    ///
    /// Original SLATEC routine: `DBESI`; source: <https://www.netlib.org/slatec/src/dbesi.f>. Native symbol: `dbesi_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALPHA`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dbesi;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute an N member sequence of I Bessel functions I/SUB(ALPHA+K-1)/(X), K=1,...,N or scaled Bessel functions EXP(-X)*I/SUB(ALPHA+K-1)/(X), K=1,...,N for nonnegative ALPHA and X.
        ///
        /// Original SLATEC routine: `DBESI`; source: <https://www.netlib.org/slatec/src/dbesi.f>. Native symbol: `dbesi_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALPHA`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dbesi_"]
        pub fn dbesi(
            x: *mut f64,
            alpha: *mut f64,
            kode: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            y: *mut f64,
            nz: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DBESI0
    /// Compute the hyperbolic Bessel function of the first kind of order zero.
    ///
    /// Original SLATEC routine: `DBESI0`; source: <https://www.netlib.org/slatec/fnlib/dbesi0.f>. Native symbol: `dbesi0_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbesi0;

    // raw-api-routine: DBESI1
    /// Compute the modified (hyperbolic) Bessel function of the first kind of order one.
    ///
    /// Original SLATEC routine: `DBESI1`; source: <https://www.netlib.org/slatec/fnlib/dbesi1.f>. Native symbol: `dbesi1_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbesi1;

    // raw-api-routine: DBESJ
    /// Compute an N member sequence of J Bessel functions J/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.
    ///
    /// Original SLATEC routine: `DBESJ`; source: <https://www.netlib.org/slatec/src/dbesj.f>. Native symbol: `dbesj_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALPHA`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dbesj;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute an N member sequence of J Bessel functions J/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.
        ///
        /// Original SLATEC routine: `DBESJ`; source: <https://www.netlib.org/slatec/src/dbesj.f>. Native symbol: `dbesj_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALPHA`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dbesj_"]
        pub fn dbesj(
            x: *mut f64,
            alpha: *mut f64,
            n: *mut crate::FortranInteger,
            y: *mut f64,
            nz: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DBESJ0
    /// Compute the Bessel function of the first kind of order zero.
    ///
    /// Original SLATEC routine: `DBESJ0`; source: <https://www.netlib.org/slatec/fnlib/dbesj0.f>. Native symbol: `dbesj0_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbesj0;

    // raw-api-routine: DBESJ1
    /// Compute the Bessel function of the first kind of order one.
    ///
    /// Original SLATEC routine: `DBESJ1`; source: <https://www.netlib.org/slatec/fnlib/dbesj1.f>. Native symbol: `dbesj1_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbesj1;

    // raw-api-routine: DBESK
    /// Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/SUB(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.
    ///
    /// Original SLATEC routine: `DBESK`; source: <https://www.netlib.org/slatec/src/dbesk.f>. Native symbol: `dbesk_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dbesk;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions K/SUB(FNU+I-1)/(X), or scaled Bessel functions EXP(X)*K/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.
        ///
        /// Original SLATEC routine: `DBESK`; source: <https://www.netlib.org/slatec/src/dbesk.f>. Native symbol: `dbesk_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dbesk_"]
        pub fn dbesk(
            x: *mut f64,
            fnu: *mut f64,
            kode: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            y: *mut f64,
            nz: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DBESK0
    /// Compute the modified (hyperbolic) Bessel function of the third kind of order zero.
    ///
    /// Original SLATEC routine: `DBESK0`; source: <https://www.netlib.org/slatec/fnlib/dbesk0.f>. Native symbol: `dbesk0_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbesk0;

    // raw-api-routine: DBESK1
    /// Compute the modified (hyperbolic) Bessel function of the third kind of order one.
    ///
    /// Original SLATEC routine: `DBESK1`; source: <https://www.netlib.org/slatec/fnlib/dbesk1.f>. Native symbol: `dbesk1_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbesk1;

    // raw-api-routine: DBESKS
    /// Compute a sequence of modified Bessel functions of the third kind of fractional order.
    ///
    /// Original SLATEC routine: `DBESKS`; source: <https://www.netlib.org/slatec/fnlib/dbesks.f>. Native symbol: `dbesks_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `XNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NIN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dbesks;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of modified Bessel functions of the third kind of fractional order.
        ///
        /// Original SLATEC routine: `DBESKS`; source: <https://www.netlib.org/slatec/fnlib/dbesks.f>. Native symbol: `dbesks_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `XNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NIN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dbesks_"]
        pub fn dbesks(xnu: *mut f64, x: *mut f64, nin: *mut crate::FortranInteger, bk: *mut f64);
    }

    // raw-api-routine: DBESY
    /// Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions Y/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.
    ///
    /// Original SLATEC routine: `DBESY`; source: <https://www.netlib.org/slatec/src/dbesy.f>. Native symbol: `dbesy_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dbesy;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Implement forward recursion on the three term recursion relation for a sequence of non-negative order Bessel functions Y/SUB(FNU+I-1)/(X), I=1,...,N for real, positive X and non-negative orders FNU.
        ///
        /// Original SLATEC routine: `DBESY`; source: <https://www.netlib.org/slatec/src/dbesy.f>. Native symbol: `dbesy_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dbesy_"]
        pub fn dbesy(x: *mut f64, fnu: *mut f64, n: *mut crate::FortranInteger, y: *mut f64);
    }

    // raw-api-routine: DBESY0
    /// Compute the Bessel function of the second kind of order zero.
    ///
    /// Original SLATEC routine: `DBESY0`; source: <https://www.netlib.org/slatec/fnlib/dbesy0.f>. Native symbol: `dbesy0_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbesy0;

    // raw-api-routine: DBESY1
    /// Compute the Bessel function of the second kind of order one.
    ///
    /// Original SLATEC routine: `DBESY1`; source: <https://www.netlib.org/slatec/fnlib/dbesy1.f>. Native symbol: `dbesy1_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbesy1;

    // raw-api-routine: DBSI0E
    /// Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero.
    ///
    /// Original SLATEC routine: `DBSI0E`; source: <https://www.netlib.org/slatec/fnlib/dbsi0e.f>. Native symbol: `dbsi0e_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbsi0e;

    // raw-api-routine: DBSI1E
    /// Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order one.
    ///
    /// Original SLATEC routine: `DBSI1E`; source: <https://www.netlib.org/slatec/fnlib/dbsi1e.f>. Native symbol: `dbsi1e_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbsi1e;

    // raw-api-routine: DBSK0E
    /// Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order zero.
    ///
    /// Original SLATEC routine: `DBSK0E`; source: <https://www.netlib.org/slatec/fnlib/dbsk0e.f>. Native symbol: `dbsk0e_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbsk0e;

    // raw-api-routine: DBSK1E
    /// Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order one.
    ///
    /// Original SLATEC routine: `DBSK1E`; source: <https://www.netlib.org/slatec/fnlib/dbsk1e.f>. Native symbol: `dbsk1e_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_bessel::dbsk1e;

    // raw-api-routine: DBSKES
    /// Compute a sequence of exponentially scaled modified Bessel functions of the third kind of fractional order.
    ///
    /// Original SLATEC routine: `DBSKES`; source: <https://www.netlib.org/slatec/fnlib/dbskes.f>. Native symbol: `dbskes_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `XNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NIN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BKE`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dbskes;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of exponentially scaled modified Bessel functions of the third kind of fractional order.
        ///
        /// Original SLATEC routine: `DBSKES`; source: <https://www.netlib.org/slatec/fnlib/dbskes.f>. Native symbol: `dbskes_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `XNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NIN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BKE`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dbskes_"]
        pub fn dbskes(xnu: *mut f64, x: *mut f64, nin: *mut crate::FortranInteger, bke: *mut f64);
    }

    // raw-api-routine: DBSKIN
    /// Compute repeated integrals of the K-zero Bessel function.
    ///
    /// Original SLATEC routine: `DBSKIN`; source: <https://www.netlib.org/slatec/src/dbskin.f>. Native symbol: `dbskin_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dbskin;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute repeated integrals of the K-zero Bessel function.
        ///
        /// Original SLATEC routine: `DBSKIN`; source: <https://www.netlib.org/slatec/src/dbskin.f>. Native symbol: `dbskin_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dbskin_"]
        pub fn dbskin(
            x: *mut f64,
            n: *mut crate::FortranInteger,
            kode: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            y: *mut f64,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DCHU
    /// Compute the logarithmic confluent hypergeometric function.
    ///
    /// Original SLATEC routine: `DCHU`; source: <https://www.netlib.org/slatec/fnlib/dchu.f>. Native symbol: `dchu_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64,mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::dchu;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the logarithmic confluent hypergeometric function.
        ///
        /// Original SLATEC routine: `DCHU`; source: <https://www.netlib.org/slatec/fnlib/dchu.f>. Native symbol: `dchu_`.
        /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64,mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `DOUBLE PRECISION` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dchu_"]
        pub fn dchu(a: *mut f64, b: *mut f64, x: *mut f64) -> f64;
    }

    // raw-api-routine: DCOT
    /// Compute the cotangent.
    ///
    /// Original SLATEC routine: `DCOT`; source: <https://www.netlib.org/slatec/fnlib/dcot.f>. Native symbol: `dcot_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::dcot;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the cotangent.
        ///
        /// Original SLATEC routine: `DCOT`; source: <https://www.netlib.org/slatec/fnlib/dcot.f>. Native symbol: `dcot_`.
        /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `DOUBLE PRECISION` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dcot_"]
        pub fn dcot(x: *mut f64) -> f64;
    }

    // raw-api-routine: DCSEVL
    /// Evaluate a Chebyshev series.
    ///
    /// Original SLATEC routine: `DCSEVL`; source: <https://www.netlib.org/slatec/fnlib/dcsevl.f>. Native symbol: `dcsevl_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CS`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_polynomials::dcsevl;

    // raw-api-routine: DE1
    /// Compute the exponential integral E1(X).
    ///
    /// Original SLATEC routine: `DE1`; source: <https://www.netlib.org/slatec/fnlib/de1.f>. Native symbol: `de1_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_integrals::de1;

    // raw-api-routine: DEI
    /// Compute the exponential integral Ei(X).
    ///
    /// Original SLATEC routine: `DEI`; source: <https://www.netlib.org/slatec/fnlib/dei.f>. Native symbol: `dei_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_integrals::dei;

    // raw-api-routine: DEXINT
    /// Compute an M member sequence of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0.
    ///
    /// Original SLATEC routine: `DEXINT`; source: <https://www.netlib.org/slatec/src/dexint.f>. Native symbol: `dexint_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `EN`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dexint;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute an M member sequence of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0.
        ///
        /// Original SLATEC routine: `DEXINT`; source: <https://www.netlib.org/slatec/src/dexint.f>. Native symbol: `dexint_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `EN`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dexint_"]
        pub fn dexint(
            x: *mut f64,
            n: *mut crate::FortranInteger,
            kode: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            tol: *mut f64,
            en: *mut f64,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DGAMLM
    /// Compute the minimum and maximum bounds for the argument in the Gamma function.
    ///
    /// Original SLATEC routine: `DGAMLM`; source: <https://www.netlib.org/slatec/fnlib/dgamlm.f>. Native symbol: `dgamlm_`.
    /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `XMIN`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `XMAX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_scalar_subroutines::dgamlm;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the minimum and maximum bounds for the argument in the Gamma function.
        ///
        /// Original SLATEC routine: `DGAMLM`; source: <https://www.netlib.org/slatec/fnlib/dgamlm.f>. Native symbol: `dgamlm_`.
        /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `XMIN`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `XMAX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dgamlm_"]
        pub fn dgamlm(xmin: *mut f64, xmax: *mut f64);
    }

    // raw-api-routine: DLGAMS
    /// Compute the logarithm of the absolute value of the Gamma function.
    ///
    /// Original SLATEC routine: `DLGAMS`; source: <https://www.netlib.org/slatec/fnlib/dlgams.f>. Native symbol: `dlgams_`.
    /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `DLGAM`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SGNGAM`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_scalar_subroutines::dlgams;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the logarithm of the absolute value of the Gamma function.
        ///
        /// Original SLATEC routine: `DLGAMS`; source: <https://www.netlib.org/slatec/fnlib/dlgams.f>. Native symbol: `dlgams_`.
        /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `DLGAM`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SGNGAM`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dlgams_"]
        pub fn dlgams(x: *mut f64, dlgam: *mut f64, sgngam: *mut f64);
    }

    // raw-api-routine: DLI
    /// Compute the logarithmic integral.
    ///
    /// Original SLATEC routine: `DLI`; source: <https://www.netlib.org/slatec/fnlib/dli.f>. Native symbol: `dli_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::dli;

    // raw-api-routine: DPOCH
    /// Evaluate a generalization of Pochhammer's symbol.
    ///
    /// Original SLATEC routine: `DPOCH`; source: <https://www.netlib.org/slatec/fnlib/dpoch.f>. Native symbol: `dpoch_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::dpoch;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate a generalization of Pochhammer's symbol.
        ///
        /// Original SLATEC routine: `DPOCH`; source: <https://www.netlib.org/slatec/fnlib/dpoch.f>. Native symbol: `dpoch_`.
        /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `DOUBLE PRECISION` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dpoch_"]
        pub fn dpoch(a: *mut f64, x: *mut f64) -> f64;
    }

    // raw-api-routine: DPOCH1
    /// Calculate a generalization of Pochhammer's symbol starting from first order.
    ///
    /// Original SLATEC routine: `DPOCH1`; source: <https://www.netlib.org/slatec/fnlib/dpoch1.f>. Native symbol: `dpoch1_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::dpoch1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Calculate a generalization of Pochhammer's symbol starting from first order.
        ///
        /// Original SLATEC routine: `DPOCH1`; source: <https://www.netlib.org/slatec/fnlib/dpoch1.f>. Native symbol: `dpoch1_`.
        /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `DOUBLE PRECISION` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dpoch1_"]
        pub fn dpoch1(a: *mut f64, x: *mut f64) -> f64;
    }

    // raw-api-routine: DPSIFN
    /// Compute derivatives of the Psi function.
    ///
    /// Original SLATEC routine: `DPSIFN`; source: <https://www.netlib.org/slatec/src/dpsifn.f>. Native symbol: `dpsifn_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ANS`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dpsifn;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute derivatives of the Psi function.
        ///
        /// Original SLATEC routine: `DPSIFN`; source: <https://www.netlib.org/slatec/src/dpsifn.f>. Native symbol: `dpsifn_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ANS`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dpsifn_"]
        pub fn dpsifn(
            x: *mut f64,
            n: *mut crate::FortranInteger,
            kode: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            ans: *mut f64,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DRC
    /// Calculate a double precision approximation to DRC(X,Y) = Integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive.
    ///
    /// Original SLATEC routine: `DRC`; source: <https://www.netlib.org/slatec/src/drc.f>. Native symbol: `drc_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::drc;

    // raw-api-routine: DRC3JJ
    /// Evaluate the 3j symbol f(L1) = ( L1 L2 L3) (-M2-M3 M2 M3) for all allowed values of L1, the other parameters being held fixed.
    ///
    /// Original SLATEC routine: `DRC3JJ`; source: <https://www.netlib.org/slatec/src/drc3jj.f>. Native symbol: `drc3jj_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `L2`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L3`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M2`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M3`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L1MIN`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L1MAX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `THRCOF`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::drc3jj;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate the 3j symbol f(L1) = ( L1 L2 L3) (-M2-M3 M2 M3) for all allowed values of L1, the other parameters being held fixed.
        ///
        /// Original SLATEC routine: `DRC3JJ`; source: <https://www.netlib.org/slatec/src/drc3jj.f>. Native symbol: `drc3jj_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `L2`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L3`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M2`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M3`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L1MIN`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L1MAX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `THRCOF`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "drc3jj_"]
        pub fn drc3jj(
            l2: *mut f64,
            l3: *mut f64,
            m2: *mut f64,
            m3: *mut f64,
            l1min: *mut f64,
            l1max: *mut f64,
            thrcof: *mut f64,
            ndim: *mut crate::FortranInteger,
            ier: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DRC3JM
    /// Evaluate the 3j symbol g(M2) = (L1 L2 L3 ) (M1 M2 -M1-M2) for all allowed values of M2, the other parameters being held fixed.
    ///
    /// Original SLATEC routine: `DRC3JM`; source: <https://www.netlib.org/slatec/src/drc3jm.f>. Native symbol: `drc3jm_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `L1`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L2`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L3`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M1`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M2MIN`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M2MAX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `THRCOF`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::drc3jm;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate the 3j symbol g(M2) = (L1 L2 L3 ) (M1 M2 -M1-M2) for all allowed values of M2, the other parameters being held fixed.
        ///
        /// Original SLATEC routine: `DRC3JM`; source: <https://www.netlib.org/slatec/src/drc3jm.f>. Native symbol: `drc3jm_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `L1`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L2`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L3`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M1`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M2MIN`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M2MAX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `THRCOF`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "drc3jm_"]
        pub fn drc3jm(
            l1: *mut f64,
            l2: *mut f64,
            l3: *mut f64,
            m1: *mut f64,
            m2min: *mut f64,
            m2max: *mut f64,
            thrcof: *mut f64,
            ndim: *mut crate::FortranInteger,
            ier: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DRC6J
    /// Evaluate the 6j symbol h(L1) = {L1 L2 L3} {L4 L5 L6} for all allowed values of L1, the other parameters being held fixed.
    ///
    /// Original SLATEC routine: `DRC6J`; source: <https://www.netlib.org/slatec/src/drc6j.f>. Native symbol: `drc6j_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `L2`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L3`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L4`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L5`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L6`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L1MIN`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L1MAX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SIXCOF`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::drc6j;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate the 6j symbol h(L1) = {L1 L2 L3} {L4 L5 L6} for all allowed values of L1, the other parameters being held fixed.
        ///
        /// Original SLATEC routine: `DRC6J`; source: <https://www.netlib.org/slatec/src/drc6j.f>. Native symbol: `drc6j_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `L2`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L3`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L4`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L5`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L6`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L1MIN`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L1MAX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SIXCOF`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "drc6j_"]
        pub fn drc6j(
            l2: *mut f64,
            l3: *mut f64,
            l4: *mut f64,
            l5: *mut f64,
            l6: *mut f64,
            l1min: *mut f64,
            l1max: *mut f64,
            sixcof: *mut f64,
            ndim: *mut crate::FortranInteger,
            ier: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DRD
    /// Compute the incomplete or complete elliptic integral of the 2nd kind. For X and Y nonnegative, X+Y and Z positive, DRD(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -3/2 (3/2)(t+X) (t+Y) (t+Z) dt. If X or Y is zero, the integral is complete.
    ///
    /// Original SLATEC routine: `DRD`; source: <https://www.netlib.org/slatec/src/drd.f>. Native symbol: `drd_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::drd;

    // raw-api-routine: DRF
    /// Compute the incomplete or complete elliptic integral of the 1st kind. For X, Y, and Z non-negative and at most one of them zero, RF(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -1/2 (1/2)(t+X) (t+Y) (t+Z) dt. If X, Y or Z is zero, the integral is complete.
    ///
    /// Original SLATEC routine: `DRF`; source: <https://www.netlib.org/slatec/src/drf.f>. Native symbol: `drf_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64,mut_f64,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::drf;

    // raw-api-routine: DRJ
    /// Compute the incomplete or complete (X or Y or Z is zero) elliptic integral of the 3rd kind. For X, Y, and Z nonnegative, at most one of them zero, and P positive, RJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt.
    ///
    /// Original SLATEC routine: `DRJ`; source: <https://www.netlib.org/slatec/src/drj.f>. Native symbol: `drj_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `P`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::drj;

    // raw-api-routine: DSPENC
    /// Compute a form of Spence's integral due to K. Mitchell.
    ///
    /// Original SLATEC routine: `DSPENC`; source: <https://www.netlib.org/slatec/fnlib/dspenc.f>. Native symbol: `dspenc_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::dspenc;

    // raw-api-routine: DXLEGF
    /// Compute normalized Legendre polynomials and associated Legendre functions.
    ///
    /// Original SLATEC routine: `DXLEGF`; source: <https://www.netlib.org/slatec/src/dxlegf.f>. Native symbol: `dxlegf_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `DNU1`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NUDIFF`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MU1`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MU2`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `THETA`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `PQA`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IPQA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERROR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dxlegf;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute normalized Legendre polynomials and associated Legendre functions.
        ///
        /// Original SLATEC routine: `DXLEGF`; source: <https://www.netlib.org/slatec/src/dxlegf.f>. Native symbol: `dxlegf_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `DNU1`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NUDIFF`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MU1`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MU2`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `THETA`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `PQA`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IPQA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERROR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dxlegf_"]
        pub fn dxlegf(
            dnu1: *mut f64,
            nudiff: *mut crate::FortranInteger,
            mu1: *mut crate::FortranInteger,
            mu2: *mut crate::FortranInteger,
            theta: *mut f64,
            id: *mut crate::FortranInteger,
            pqa: *mut f64,
            ipqa: *mut crate::FortranInteger,
            ierror: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: DXNRMP
    /// Compute normalized Legendre polynomials.
    ///
    /// Original SLATEC routine: `DXNRMP`; source: <https://www.netlib.org/slatec/src/dxnrmp.f>. Native symbol: `dxnrmp_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NU`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MU1`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MU2`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `DARG`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `DPN`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IPN`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ISIG`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERROR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dxnrmp;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute normalized Legendre polynomials.
        ///
        /// Original SLATEC routine: `DXNRMP`; source: <https://www.netlib.org/slatec/src/dxnrmp.f>. Native symbol: `dxnrmp_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NU`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MU1`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MU2`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `DARG`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `DPN`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IPN`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ISIG`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERROR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dxnrmp_"]
        pub fn dxnrmp(
            nu: *mut crate::FortranInteger,
            mu1: *mut crate::FortranInteger,
            mu2: *mut crate::FortranInteger,
            darg: *mut f64,
            mode: *mut crate::FortranInteger,
            dpn: *mut f64,
            ipn: *mut crate::FortranInteger,
            isig: *mut crate::FortranInteger,
            ierror: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: E1
    /// Compute the exponential integral E1(X).
    ///
    /// Original SLATEC routine: `E1`; source: <https://www.netlib.org/slatec/fnlib/e1.f>. Native symbol: `e1_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_integrals::e1;

    // raw-api-routine: EI
    /// Compute the exponential integral Ei(X).
    ///
    /// Original SLATEC routine: `EI`; source: <https://www.netlib.org/slatec/fnlib/ei.f>. Native symbol: `ei_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::families::special_integrals::ei;

    // raw-api-routine: EXINT
    /// Compute an M member sequence of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0.
    ///
    /// Original SLATEC routine: `EXINT`; source: <https://www.netlib.org/slatec/src/exint.f>. Native symbol: `exint_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `EN`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::exint;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute an M member sequence of exponential integrals E(N+K,X), K=0,1,...,M-1 for N .GE. 1 and X .GE. 0.
        ///
        /// Original SLATEC routine: `EXINT`; source: <https://www.netlib.org/slatec/src/exint.f>. Native symbol: `exint_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `EN`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "exint_"]
        pub fn exint(
            x: *mut f32,
            n: *mut crate::FortranInteger,
            kode: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            tol: *mut f32,
            en: *mut f32,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: GAMLIM
    /// Compute the minimum and maximum bounds for the argument in the Gamma function.
    ///
    /// Original SLATEC routine: `GAMLIM`; source: <https://www.netlib.org/slatec/fnlib/gamlim.f>. Native symbol: `gamlim_`.
    /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `XMIN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `XMAX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_scalar_subroutines::gamlim;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the minimum and maximum bounds for the argument in the Gamma function.
        ///
        /// Original SLATEC routine: `GAMLIM`; source: <https://www.netlib.org/slatec/fnlib/gamlim.f>. Native symbol: `gamlim_`.
        /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `XMIN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `XMAX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "gamlim_"]
        pub fn gamlim(xmin: *mut f32, xmax: *mut f32);
    }

    // raw-api-routine: INITDS
    /// Determine the number of terms needed in an orthogonal polynomial series so that it meets a specified accuracy.
    ///
    /// Original SLATEC routine: `INITDS`; source: <https://www.netlib.org/slatec/fnlib/initds.f>. Native symbol: `initds_`.
    /// Batch A ABI class: `integer_scalar_function`; normalized fingerprint: `function:i32(mut_f64_ptr_rank1,mut_i32,mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `OS`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NOS`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ETA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `INTEGER` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::initds;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Determine the number of terms needed in an orthogonal polynomial series so that it meets a specified accuracy.
        ///
        /// Original SLATEC routine: `INITDS`; source: <https://www.netlib.org/slatec/fnlib/initds.f>. Native symbol: `initds_`.
        /// Batch A ABI class: `integer_scalar_function`; normalized fingerprint: `function:i32(mut_f64_ptr_rank1,mut_i32,mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `OS`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NOS`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ETA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `INTEGER` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "initds_"]
        pub fn initds(
            os: *mut f64,
            nos: *mut crate::FortranInteger,
            eta: *mut f32,
        ) -> crate::FortranInteger;
    }

    // raw-api-routine: INITS
    /// Determine the number of terms needed in an orthogonal polynomial series so that it meets a specified accuracy.
    ///
    /// Original SLATEC routine: `INITS`; source: <https://www.netlib.org/slatec/fnlib/inits.f>. Native symbol: `inits_`.
    /// Batch A ABI class: `integer_scalar_function`; normalized fingerprint: `function:i32(mut_f32_ptr_rank1,mut_i32,mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `OS`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NOS`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ETA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `INTEGER` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::inits;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Determine the number of terms needed in an orthogonal polynomial series so that it meets a specified accuracy.
        ///
        /// Original SLATEC routine: `INITS`; source: <https://www.netlib.org/slatec/fnlib/inits.f>. Native symbol: `inits_`.
        /// Batch A ABI class: `integer_scalar_function`; normalized fingerprint: `function:i32(mut_f32_ptr_rank1,mut_i32,mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `OS`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NOS`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ETA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `INTEGER` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "inits_"]
        pub fn inits(
            os: *mut f32,
            nos: *mut crate::FortranInteger,
            eta: *mut f32,
        ) -> crate::FortranInteger;
    }

    // raw-api-routine: POCH
    /// Evaluate a generalization of Pochhammer's symbol.
    ///
    /// Original SLATEC routine: `POCH`; source: <https://www.netlib.org/slatec/fnlib/poch.f>. Native symbol: `poch_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::poch;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate a generalization of Pochhammer's symbol.
        ///
        /// Original SLATEC routine: `POCH`; source: <https://www.netlib.org/slatec/fnlib/poch.f>. Native symbol: `poch_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "poch_"]
        pub fn poch(a: *mut f32, x: *mut f32) -> f32;
    }

    // raw-api-routine: POCH1
    /// Calculate a generalization of Pochhammer's symbol starting from first order.
    ///
    /// Original SLATEC routine: `POCH1`; source: <https://www.netlib.org/slatec/fnlib/poch1.f>. Native symbol: `poch1_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::poch1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Calculate a generalization of Pochhammer's symbol starting from first order.
        ///
        /// Original SLATEC routine: `POCH1`; source: <https://www.netlib.org/slatec/fnlib/poch1.f>. Native symbol: `poch1_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "poch1_"]
        pub fn poch1(a: *mut f32, x: *mut f32) -> f32;
    }

    // raw-api-routine: PSIFN
    /// Compute derivatives of the Psi function.
    ///
    /// Original SLATEC routine: `PSIFN`; source: <https://www.netlib.org/slatec/src/psifn.f>. Native symbol: `psifn_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ANS`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::psifn;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute derivatives of the Psi function.
        ///
        /// Original SLATEC routine: `PSIFN`; source: <https://www.netlib.org/slatec/src/psifn.f>. Native symbol: `psifn_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ANS`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "psifn_"]
        pub fn psifn(
            x: *mut f32,
            n: *mut crate::FortranInteger,
            kode: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            ans: *mut f32,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RC
    /// Calculate an approximation to RC(X,Y) = Integral from zero to infinity of -1/2 -1 (1/2)(t+X) (t+Y) dt, where X is nonnegative and Y is positive.
    ///
    /// Original SLATEC routine: `RC`; source: <https://www.netlib.org/slatec/src/rc.f>. Native symbol: `rc_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::rc;

    // raw-api-routine: RC3JJ
    /// Evaluate the 3j symbol f(L1) = ( L1 L2 L3) (-M2-M3 M2 M3) for all allowed values of L1, the other parameters being held fixed.
    ///
    /// Original SLATEC routine: `RC3JJ`; source: <https://www.netlib.org/slatec/src/rc3jj.f>. Native symbol: `rc3jj_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `L2`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L3`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M2`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M3`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L1MIN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L1MAX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `THRCOF`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rc3jj;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate the 3j symbol f(L1) = ( L1 L2 L3) (-M2-M3 M2 M3) for all allowed values of L1, the other parameters being held fixed.
        ///
        /// Original SLATEC routine: `RC3JJ`; source: <https://www.netlib.org/slatec/src/rc3jj.f>. Native symbol: `rc3jj_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `L2`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L3`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M2`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M3`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L1MIN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L1MAX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `THRCOF`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rc3jj_"]
        pub fn rc3jj(
            l2: *mut f32,
            l3: *mut f32,
            m2: *mut f32,
            m3: *mut f32,
            l1min: *mut f32,
            l1max: *mut f32,
            thrcof: *mut f32,
            ndim: *mut crate::FortranInteger,
            ier: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RC3JM
    /// Evaluate the 3j symbol g(M2) = (L1 L2 L3 ) (M1 M2 -M1-M2) for all allowed values of M2, the other parameters being held fixed.
    ///
    /// Original SLATEC routine: `RC3JM`; source: <https://www.netlib.org/slatec/src/rc3jm.f>. Native symbol: `rc3jm_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `L1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L2`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L3`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M2MIN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M2MAX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `THRCOF`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rc3jm;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate the 3j symbol g(M2) = (L1 L2 L3 ) (M1 M2 -M1-M2) for all allowed values of M2, the other parameters being held fixed.
        ///
        /// Original SLATEC routine: `RC3JM`; source: <https://www.netlib.org/slatec/src/rc3jm.f>. Native symbol: `rc3jm_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `L1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L2`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L3`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M2MIN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M2MAX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `THRCOF`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rc3jm_"]
        pub fn rc3jm(
            l1: *mut f32,
            l2: *mut f32,
            l3: *mut f32,
            m1: *mut f32,
            m2min: *mut f32,
            m2max: *mut f32,
            thrcof: *mut f32,
            ndim: *mut crate::FortranInteger,
            ier: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RC6J
    /// Evaluate the 6j symbol h(L1) = {L1 L2 L3} {L4 L5 L6} for all allowed values of L1, the other parameters being held fixed.
    ///
    /// Original SLATEC routine: `RC6J`; source: <https://www.netlib.org/slatec/src/rc6j.f>. Native symbol: `rc6j_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `L2`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L3`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L4`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L5`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L6`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L1MIN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `L1MAX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SIXCOF`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rc6j;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate the 6j symbol h(L1) = {L1 L2 L3} {L4 L5 L6} for all allowed values of L1, the other parameters being held fixed.
        ///
        /// Original SLATEC routine: `RC6J`; source: <https://www.netlib.org/slatec/src/rc6j.f>. Native symbol: `rc6j_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `L2`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L3`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L4`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L5`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L6`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L1MIN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `L1MAX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SIXCOF`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NDIM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rc6j_"]
        pub fn rc6j(
            l2: *mut f32,
            l3: *mut f32,
            l4: *mut f32,
            l5: *mut f32,
            l6: *mut f32,
            l1min: *mut f32,
            l1max: *mut f32,
            sixcof: *mut f32,
            ndim: *mut crate::FortranInteger,
            ier: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RD
    /// Compute the incomplete or complete elliptic integral of the 2nd kind. For X and Y nonnegative, X+Y and Z positive, RD(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -3/2 (3/2)(t+X) (t+Y) (t+Z) dt. If X or Y is zero, the integral is complete.
    ///
    /// Original SLATEC routine: `RD`; source: <https://www.netlib.org/slatec/src/rd.f>. Native symbol: `rd_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32,mut_f32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::rd;

    // raw-api-routine: RF
    /// Compute the incomplete or complete elliptic integral of the 1st kind. For X, Y, and Z non-negative and at most one of them zero, RF(X,Y,Z) = Integral from zero to infinity of -1/2 -1/2 -1/2 (1/2)(t+X) (t+Y) (t+Z) dt. If X, Y or Z is zero, the integral is complete.
    ///
    /// Original SLATEC routine: `RF`; source: <https://www.netlib.org/slatec/src/rf.f>. Native symbol: `rf_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32,mut_f32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::rf;

    // raw-api-routine: RJ
    /// Compute the incomplete or complete (X or Y or Z is zero) elliptic integral of the 3rd kind. For X, Y, and Z nonnegative, at most one of them zero, and P positive, RJ(X,Y,Z,P) = Integral from zero to infinity of -1/2 -1/2 -1/2 -1 (3/2)(t+X) (t+Y) (t+Z) (t+P) dt.
    ///
    /// Original SLATEC routine: `RJ`; source: <https://www.netlib.org/slatec/src/rj.f>. Native symbol: `rj_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `P`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::rj;

    // raw-api-routine: SPENC
    /// Compute a form of Spence's integral due to K. Mitchell.
    ///
    /// Original SLATEC routine: `SPENC`; source: <https://www.netlib.org/slatec/fnlib/spenc.f>. Native symbol: `spenc_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::special_scalar_expanded::spenc;

    // raw-api-routine: XLEGF
    /// Compute normalized Legendre polynomials and associated Legendre functions.
    ///
    /// Original SLATEC routine: `XLEGF`; source: <https://www.netlib.org/slatec/src/xlegf.f>. Native symbol: `xlegf_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `DNU1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NUDIFF`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MU1`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MU2`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `THETA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `PQA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IPQA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERROR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::xlegf;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute normalized Legendre polynomials and associated Legendre functions.
        ///
        /// Original SLATEC routine: `XLEGF`; source: <https://www.netlib.org/slatec/src/xlegf.f>. Native symbol: `xlegf_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `DNU1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NUDIFF`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MU1`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MU2`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `THETA`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `PQA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IPQA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERROR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "xlegf_"]
        pub fn xlegf(
            dnu1: *mut f32,
            nudiff: *mut crate::FortranInteger,
            mu1: *mut crate::FortranInteger,
            mu2: *mut crate::FortranInteger,
            theta: *mut f32,
            id: *mut crate::FortranInteger,
            pqa: *mut f32,
            ipqa: *mut crate::FortranInteger,
            ierror: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: XNRMP
    /// Compute normalized Legendre polynomials.
    ///
    /// Original SLATEC routine: `XNRMP`; source: <https://www.netlib.org/slatec/src/xnrmp.f>. Native symbol: `xnrmp_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NU`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MU1`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MU2`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SARG`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SPN`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IPN`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ISIG`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERROR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::xnrmp;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute normalized Legendre polynomials.
        ///
        /// Original SLATEC routine: `XNRMP`; source: <https://www.netlib.org/slatec/src/xnrmp.f>. Native symbol: `xnrmp_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NU`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MU1`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MU2`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SARG`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SPN`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IPN`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ISIG`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERROR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "xnrmp_"]
        pub fn xnrmp(
            nu: *mut crate::FortranInteger,
            mu1: *mut crate::FortranInteger,
            mu2: *mut crate::FortranInteger,
            sarg: *mut f32,
            mode: *mut crate::FortranInteger,
            spn: *mut f32,
            ipn: *mut crate::FortranInteger,
            isig: *mut crate::FortranInteger,
            ierror: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: ZAIRY
    /// Compute the Airy function Ai(z) or its derivative dAi/dz for complex argument z. A scaling option is available to help avoid underflow and overflow.
    ///
    /// Original SLATEC routine: `ZAIRY`; source: <https://www.netlib.org/slatec/src/zairy.f>. Native symbol: `zairy_`.
    /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AIR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AII`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_scalar_subroutines::zairy;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the Airy function Ai(z) or its derivative dAi/dz for complex argument z. A scaling option is available to help avoid underflow and overflow.
        ///
        /// Original SLATEC routine: `ZAIRY`; source: <https://www.netlib.org/slatec/src/zairy.f>. Native symbol: `zairy_`.
        /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AIR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AII`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "zairy_"]
        pub fn zairy(
            zr: *mut f64,
            zi: *mut f64,
            id: *mut crate::FortranInteger,
            kode: *mut crate::FortranInteger,
            air: *mut f64,
            aii: *mut f64,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: ZBESH
    /// Compute a sequence of the Hankel functions H(m,a,z) for superscript m=1 or 2, real nonnegative orders a=b, b+1,... where b>0, and nonzero complex argument z. A scaling option is available to help avoid overflow.
    ///
    /// Original SLATEC routine: `ZBESH`; source: <https://www.netlib.org/slatec/src/zbesh.f>. Native symbol: `zbesh_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::zbesh;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of the Hankel functions H(m,a,z) for superscript m=1 or 2, real nonnegative orders a=b, b+1,... where b>0, and nonzero complex argument z. A scaling option is available to help avoid overflow.
        ///
        /// Original SLATEC routine: `ZBESH`; source: <https://www.netlib.org/slatec/src/zbesh.f>. Native symbol: `zbesh_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "zbesh_"]
        pub fn zbesh(
            zr: *mut f64,
            zi: *mut f64,
            fnu: *mut f64,
            kode: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            cyr: *mut f64,
            cyi: *mut f64,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: ZBESI
    /// Compute a sequence of the Bessel functions I(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.
    ///
    /// Original SLATEC routine: `ZBESI`; source: <https://www.netlib.org/slatec/src/zbesi.f>. Native symbol: `zbesi_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::zbesi;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of the Bessel functions I(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.
        ///
        /// Original SLATEC routine: `ZBESI`; source: <https://www.netlib.org/slatec/src/zbesi.f>. Native symbol: `zbesi_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "zbesi_"]
        pub fn zbesi(
            zr: *mut f64,
            zi: *mut f64,
            fnu: *mut f64,
            kode: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            cyr: *mut f64,
            cyi: *mut f64,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: ZBESJ
    /// Compute a sequence of the Bessel functions J(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.
    ///
    /// Original SLATEC routine: `ZBESJ`; source: <https://www.netlib.org/slatec/src/zbesj.f>. Native symbol: `zbesj_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::zbesj;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of the Bessel functions J(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.
        ///
        /// Original SLATEC routine: `ZBESJ`; source: <https://www.netlib.org/slatec/src/zbesj.f>. Native symbol: `zbesj_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "zbesj_"]
        pub fn zbesj(
            zr: *mut f64,
            zi: *mut f64,
            fnu: *mut f64,
            kode: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            cyr: *mut f64,
            cyi: *mut f64,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: ZBESK
    /// Compute a sequence of the Bessel functions K(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.
    ///
    /// Original SLATEC routine: `ZBESK`; source: <https://www.netlib.org/slatec/src/zbesk.f>. Native symbol: `zbesk_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::zbesk;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of the Bessel functions K(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.
        ///
        /// Original SLATEC routine: `ZBESK`; source: <https://www.netlib.org/slatec/src/zbesk.f>. Native symbol: `zbesk_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "zbesk_"]
        pub fn zbesk(
            zr: *mut f64,
            zi: *mut f64,
            fnu: *mut f64,
            kode: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            cyr: *mut f64,
            cyi: *mut f64,
            nz: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: ZBESY
    /// Compute a sequence of the Bessel functions Y(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.
    ///
    /// Original SLATEC routine: `ZBESY`; source: <https://www.netlib.org/slatec/src/zbesy.f>. Native symbol: `zbesy_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CWRKR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CWRKI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::zbesy;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute a sequence of the Bessel functions Y(a,z) for complex argument z and real nonnegative orders a=b,b+1, b+2,... where b>0. A scaling option is available to help avoid overflow.
        ///
        /// Original SLATEC routine: `ZBESY`; source: <https://www.netlib.org/slatec/src/zbesy.f>. Native symbol: `zbesy_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FNU`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CYI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CWRKR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CWRKI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "zbesy_"]
        pub fn zbesy(
            zr: *mut f64,
            zi: *mut f64,
            fnu: *mut f64,
            kode: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            cyr: *mut f64,
            cyi: *mut f64,
            nz: *mut crate::FortranInteger,
            cwrkr: *mut f64,
            cwrki: *mut f64,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: ZBIRY
    /// Compute the Airy function Bi(z) or its derivative dBi/dz for complex argument z. A scaling option is available to help avoid overflow.
    ///
    /// Original SLATEC routine: `ZBIRY`; source: <https://www.netlib.org/slatec/src/zbiry.f>. Native symbol: `zbiry_`.
    /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BIR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BII`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_scalar_subroutines::zbiry;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the Airy function Bi(z) or its derivative dBi/dz for complex argument z. A scaling option is available to help avoid overflow.
        ///
        /// Original SLATEC routine: `ZBIRY`; source: <https://www.netlib.org/slatec/src/zbiry.f>. Native symbol: `zbiry_`.
        /// Batch A ABI class: `simple_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64,mut_i32,mut_i32,mut_f64,mut_f64,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `ZR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KODE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BIR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BII`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "zbiry_"]
        pub fn zbiry(
            zr: *mut f64,
            zi: *mut f64,
            id: *mut crate::FortranInteger,
            kode: *mut crate::FortranInteger,
            bir: *mut f64,
            bii: *mut f64,
            ierr: *mut crate::FortranInteger,
        );
    }
}
