//! Generated Batch A canonical raw re-exports.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-a --offline`.

/// Batch A canonical `ode` declarations.
pub mod numerical {
    // raw-api-routine: DINTP
    /// Approximate the solution at XOUT by evaluating the polynomial computed in DSTEPS at XOUT. Must be used in conjunction with DSTEPS.
    ///
    /// Original SLATEC routine: `DINTP`; source: <https://www.netlib.org/slatec/src/dintp.f>. Native symbol: `dintp_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `XOUT`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `YOUT`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `YPOUT`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NEQN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KOLD`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `PHI`: declared `DOUBLE PRECISION` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IVC`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IV`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KGI`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `GI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALPHA`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `OG`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `OW`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `OX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `OY`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::dintp;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Approximate the solution at XOUT by evaluating the polynomial computed in DSTEPS at XOUT. Must be used in conjunction with DSTEPS.
        ///
        /// Original SLATEC routine: `DINTP`; source: <https://www.netlib.org/slatec/src/dintp.f>. Native symbol: `dintp_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f64,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64,mut_f64_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `XOUT`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `YOUT`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `YPOUT`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NEQN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KOLD`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `PHI`: declared `DOUBLE PRECISION` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IVC`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IV`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KGI`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `GI`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALPHA`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `OG`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `OW`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `OX`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `OY`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "dintp_"]
        pub fn dintp(
            x: *mut f64,
            y: *mut f64,
            xout: *mut f64,
            yout: *mut f64,
            ypout: *mut f64,
            neqn: *mut crate::FortranInteger,
            kold: *mut crate::FortranInteger,
            phi: *mut f64,
            ivc: *mut crate::FortranInteger,
            iv: *mut crate::FortranInteger,
            kgi: *mut crate::FortranInteger,
            gi: *mut f64,
            alpha: *mut f64,
            og: *mut f64,
            ow: *mut f64,
            ox: *mut f64,
            oy: *mut f64,
        );
    }

    // raw-api-routine: SINTRP
    /// Approximate the solution at XOUT by evaluating the polynomial computed in STEPS at XOUT. Must be used in conjunction with STEPS.
    ///
    /// Original SLATEC routine: `SINTRP`; source: <https://www.netlib.org/slatec/src/sintrp.f>. Native symbol: `sintrp_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `XOUT`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `YOUT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `YPOUT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NEQN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KOLD`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `PHI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IVC`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IV`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `KGI`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `GI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALPHA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `OG`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `OW`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `OX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `OY`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::sintrp;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Approximate the solution at XOUT by evaluating the polynomial computed in STEPS at XOUT. Must be used in conjunction with STEPS.
        ///
        /// Original SLATEC routine: `SINTRP`; source: <https://www.netlib.org/slatec/src/sintrp.f>. Native symbol: `sintrp_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `X`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `XOUT`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `YOUT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `YPOUT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NEQN`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KOLD`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `PHI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IVC`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IV`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `KGI`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `GI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALPHA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `OG`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `OW`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `OX`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `OY`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "sintrp_"]
        pub fn sintrp(
            x: *mut f32,
            y: *mut f32,
            xout: *mut f32,
            yout: *mut f32,
            ypout: *mut f32,
            neqn: *mut crate::FortranInteger,
            kold: *mut crate::FortranInteger,
            phi: *mut f32,
            ivc: *mut crate::FortranInteger,
            iv: *mut crate::FortranInteger,
            kgi: *mut crate::FortranInteger,
            gi: *mut f32,
            alpha: *mut f32,
            og: *mut f32,
            ow: *mut f32,
            ox: *mut f32,
            oy: *mut f32,
        );
    }
}
