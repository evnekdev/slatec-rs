//! Generated Batch A canonical raw re-exports.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-a --offline`.

/// Batch A canonical `fftpack` declarations.
pub mod numerical {
    // raw-api-routine: CFFTB1
    /// Compute the unnormalized inverse of CFFTF1.
    ///
    /// Original SLATEC routine: `CFFTB1`; source: <https://www.netlib.org/slatec/fishfft/cfftb1.f>. Native symbol: `cfftb1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `C`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CH`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack_complex::cfftb1;

    // raw-api-routine: CFFTF1
    /// Compute the forward transform of a complex, periodic sequence.
    ///
    /// Original SLATEC routine: `CFFTF1`; source: <https://www.netlib.org/slatec/fishfft/cfftf1.f>. Native symbol: `cfftf1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `C`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CH`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack_complex::cfftf1;

    // raw-api-routine: CFFTI1
    /// Initialize a real and an integer work array for CFFTF1 and CFFTB1.
    ///
    /// Original SLATEC routine: `CFFTI1`; source: <https://www.netlib.org/slatec/fishfft/cffti1.f>. Native symbol: `cffti1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack_complex::cffti1;

    // raw-api-routine: COSQB
    /// Compute the unnormalized inverse cosine transform.
    ///
    /// Original SLATEC routine: `COSQB`; source: <https://www.netlib.org/slatec/fishfft/cosqb.f>. Native symbol: `cosqb_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::cosqb;

    // raw-api-routine: COSQF
    /// Compute the forward cosine transform with odd wave numbers.
    ///
    /// Original SLATEC routine: `COSQF`; source: <https://www.netlib.org/slatec/fishfft/cosqf.f>. Native symbol: `cosqf_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::cosqf;

    // raw-api-routine: COSQI
    /// Initialize a work array for COSQF and COSQB.
    ///
    /// Original SLATEC routine: `COSQI`; source: <https://www.netlib.org/slatec/fishfft/cosqi.f>. Native symbol: `cosqi_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::cosqi;

    // raw-api-routine: COST
    /// Compute the cosine transform of a real, even sequence.
    ///
    /// Original SLATEC routine: `COST`; source: <https://www.netlib.org/slatec/fishfft/cost.f>. Native symbol: `cost_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::cost;

    // raw-api-routine: COSTI
    /// Initialize a work array for COST.
    ///
    /// Original SLATEC routine: `COSTI`; source: <https://www.netlib.org/slatec/fishfft/costi.f>. Native symbol: `costi_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::costi;

    // raw-api-routine: EZFFTB
    /// A simplified real, periodic, backward fast Fourier transform.
    ///
    /// Original SLATEC routine: `EZFFTB`; source: <https://www.netlib.org/slatec/fishfft/ezfftb.f>. Native symbol: `ezfftb_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `R`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AZERO`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::ezfftb;

    // raw-api-routine: EZFFTF
    /// Compute a simplified real, periodic, fast Fourier forward transform.
    ///
    /// Original SLATEC routine: `EZFFTF`; source: <https://www.netlib.org/slatec/fishfft/ezfftf.f>. Native symbol: `ezfftf_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `R`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AZERO`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::ezfftf;

    // raw-api-routine: EZFFTI
    /// Initialize a work array for EZFFTF and EZFFTB.
    ///
    /// Original SLATEC routine: `EZFFTI`; source: <https://www.netlib.org/slatec/fishfft/ezffti.f>. Native symbol: `ezffti_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::ezffti;

    // raw-api-routine: RFFTB1
    /// Compute the backward fast Fourier transform of a real coefficient array.
    ///
    /// Original SLATEC routine: `RFFTB1`; source: <https://www.netlib.org/slatec/fishfft/rfftb1.f>. Native symbol: `rfftb1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `C`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CH`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rfftb1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the backward fast Fourier transform of a real coefficient array.
        ///
        /// Original SLATEC routine: `RFFTB1`; source: <https://www.netlib.org/slatec/fishfft/rfftb1.f>. Native symbol: `rfftb1_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `C`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CH`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rfftb1_"]
        pub fn rfftb1(
            n: *mut crate::FortranInteger,
            c: *mut f32,
            ch: *mut f32,
            wa: *mut f32,
            ifac: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RFFTF1
    /// Compute the forward transform of a real, periodic sequence.
    ///
    /// Original SLATEC routine: `RFFTF1`; source: <https://www.netlib.org/slatec/fishfft/rfftf1.f>. Native symbol: `rfftf1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `C`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `CH`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rfftf1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the forward transform of a real, periodic sequence.
        ///
        /// Original SLATEC routine: `RFFTF1`; source: <https://www.netlib.org/slatec/fishfft/rfftf1.f>. Native symbol: `rfftf1_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `C`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `CH`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rfftf1_"]
        pub fn rfftf1(
            n: *mut crate::FortranInteger,
            c: *mut f32,
            ch: *mut f32,
            wa: *mut f32,
            ifac: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RFFTI1
    /// Initialize a real and an integer work array for RFFTF1 and RFFTB1.
    ///
    /// Original SLATEC routine: `RFFTI1`; source: <https://www.netlib.org/slatec/fishfft/rffti1.f>. Native symbol: `rffti1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rffti1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Initialize a real and an integer work array for RFFTF1 and RFFTB1.
        ///
        /// Original SLATEC routine: `RFFTI1`; source: <https://www.netlib.org/slatec/fishfft/rffti1.f>. Native symbol: `rffti1_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IFAC`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rffti1_"]
        pub fn rffti1(
            n: *mut crate::FortranInteger,
            wa: *mut f32,
            ifac: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: SINQB
    /// Compute the unnormalized inverse of SINQF.
    ///
    /// Original SLATEC routine: `SINQB`; source: <https://www.netlib.org/slatec/fishfft/sinqb.f>. Native symbol: `sinqb_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::sinqb;

    // raw-api-routine: SINQF
    /// Compute the forward sine transform with odd wave numbers.
    ///
    /// Original SLATEC routine: `SINQF`; source: <https://www.netlib.org/slatec/fishfft/sinqf.f>. Native symbol: `sinqf_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::sinqf;

    // raw-api-routine: SINQI
    /// Initialize a work array for SINQF and SINQB.
    ///
    /// Original SLATEC routine: `SINQI`; source: <https://www.netlib.org/slatec/fishfft/sinqi.f>. Native symbol: `sinqi_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::sinqi;

    // raw-api-routine: SINT
    /// Compute the sine transform of a real, odd sequence.
    ///
    /// Original SLATEC routine: `SINT`; source: <https://www.netlib.org/slatec/fishfft/sint.f>. Native symbol: `sint_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::sint;

    // raw-api-routine: SINTI
    /// Initialize a work array for SINT.
    ///
    /// Original SLATEC routine: `SINTI`; source: <https://www.netlib.org/slatec/fishfft/sinti.f>. Native symbol: `sinti_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WSAVE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    pub use crate::fftpack::sinti;
}
