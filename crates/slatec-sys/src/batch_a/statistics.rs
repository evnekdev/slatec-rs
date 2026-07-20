//! Generated Batch A canonical raw re-exports.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-a --offline`.

/// Batch A canonical `statistics` declarations.
pub mod numerical {
    // raw-api-routine: CV
    /// Evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram FC.
    ///
    /// Original SLATEC routine: `CV`; source: <https://www.netlib.org/slatec/src/cv.f>. Native symbol: `cv_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `XVAL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NDATA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NCONST`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NORD`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NBKPT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BKPT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::cv;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram FC.
        ///
        /// Original SLATEC routine: `CV`; source: <https://www.netlib.org/slatec/src/cv.f>. Native symbol: `cv_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `XVAL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NDATA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NCONST`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NORD`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NBKPT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BKPT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "cv_"]
        pub fn cv(
            xval: *mut f32,
            ndata: *mut crate::FortranInteger,
            nconst: *mut crate::FortranInteger,
            nord: *mut crate::FortranInteger,
            nbkpt: *mut crate::FortranInteger,
            bkpt: *mut f32,
            w: *mut f32,
        ) -> f32;
    }

    // raw-api-routine: DCV
    /// Evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram DFC.
    ///
    /// Original SLATEC routine: `DCV`; source: <https://www.netlib.org/slatec/src/dcv.f>. Native symbol: `dcv_`.
    /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `XVAL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NDATA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NCONST`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NORD`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NBKPT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BKPT`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `DOUBLE PRECISION` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::dcv;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Evaluate the variance function of the curve obtained by the constrained B-spline fitting subprogram DFC.
        ///
        /// Original SLATEC routine: `DCV`; source: <https://www.netlib.org/slatec/src/dcv.f>. Native symbol: `dcv_`.
        /// Batch A ABI class: `double_scalar_function`; normalized fingerprint: `function:f64(mut_f64,mut_i32,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `XVAL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NDATA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NCONST`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NORD`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NBKPT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BKPT`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `DOUBLE PRECISION` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dcv_"]
        pub fn dcv(
            xval: *mut f64,
            ndata: *mut crate::FortranInteger,
            nconst: *mut crate::FortranInteger,
            nord: *mut crate::FortranInteger,
            nbkpt: *mut crate::FortranInteger,
            bkpt: *mut f64,
            w: *mut f64,
        ) -> f64;
    }

    // raw-api-routine: RAND
    /// Generate a uniformly distributed random number.
    ///
    /// Original SLATEC routine: `RAND`; source: <https://www.netlib.org/slatec/fnlib/rand.f>. Native symbol: `rand_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `R`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::rand;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Generate a uniformly distributed random number.
        ///
        /// Original SLATEC routine: `RAND`; source: <https://www.netlib.org/slatec/fnlib/rand.f>. Native symbol: `rand_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `R`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rand_"]
        pub fn rand(r: *mut f32) -> f32;
    }

    // raw-api-routine: RGAUSS
    /// Generate a normally distributed (Gaussian) random number.
    ///
    /// Original SLATEC routine: `RGAUSS`; source: <https://www.netlib.org/slatec/fnlib/rgauss.f>. Native symbol: `rgauss_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32)`.
    ///
    /// # Arguments
    ///
    /// - `XMEAN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SD`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::rgauss;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Generate a normally distributed (Gaussian) random number.
        ///
        /// Original SLATEC routine: `RGAUSS`; source: <https://www.netlib.org/slatec/fnlib/rgauss.f>. Native symbol: `rgauss_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32,mut_f32)`.
        ///
        /// # Arguments
        ///
        /// - `XMEAN`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SD`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rgauss_"]
        pub fn rgauss(xmean: *mut f32, sd: *mut f32) -> f32;
    }

    // raw-api-routine: RUNIF
    /// Generate a uniformly distributed random number.
    ///
    /// Original SLATEC routine: `RUNIF`; source: <https://www.netlib.org/slatec/fnlib/runif.f>. Native symbol: `runif_`.
    /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `T`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - Return: Fortran `REAL` scalar result.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::scalar_functions::runif;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Generate a uniformly distributed random number.
        ///
        /// Original SLATEC routine: `RUNIF`; source: <https://www.netlib.org/slatec/fnlib/runif.f>. Native symbol: `runif_`.
        /// Batch A ABI class: `real_scalar_function`; normalized fingerprint: `function:f32(mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `T`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - Return: Fortran `REAL` scalar result.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "runif_"]
        pub fn runif(t: *mut f32, n: *mut crate::FortranInteger) -> f32;
    }
}
