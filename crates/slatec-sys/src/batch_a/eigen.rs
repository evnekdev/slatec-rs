//! Generated Batch A canonical raw re-exports.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-a --offline`.

/// Batch A canonical `eigen` declarations.
pub mod numerical {
    // raw-api-routine: BAKVEC
    /// Form the eigenvectors of a certain real non-symmetric tridiagonal matrix from a symmetric tridiagonal matrix output from FIGI.
    ///
    /// Original SLATEC routine: `BAKVEC`; source: <https://www.netlib.org/slatec/lin/bakvec.f>. Native symbol: `bakvec_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `T`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::bakvec;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a certain real non-symmetric tridiagonal matrix from a symmetric tridiagonal matrix output from FIGI.
        ///
        /// Original SLATEC routine: `BAKVEC`; source: <https://www.netlib.org/slatec/lin/bakvec.f>. Native symbol: `bakvec_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `T`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "bakvec_"]
        pub fn bakvec(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            t: *mut f32,
            e: *mut f32,
            m: *mut crate::FortranInteger,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: BALANC
    /// Balance a real general matrix and isolate eigenvalues whenever possible.
    ///
    /// Original SLATEC routine: `BALANC`; source: <https://www.netlib.org/slatec/lin/balanc.f>. Native symbol: `balanc_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SCALE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::balanc;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Balance a real general matrix and isolate eigenvalues whenever possible.
        ///
        /// Original SLATEC routine: `BALANC`; source: <https://www.netlib.org/slatec/lin/balanc.f>. Native symbol: `balanc_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SCALE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "balanc_"]
        pub fn balanc(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            scale: *mut f32,
        );
    }

    // raw-api-routine: BALBAK
    /// Form the eigenvectors of a real general matrix from the eigenvectors of matrix output from BALANC.
    ///
    /// Original SLATEC routine: `BALBAK`; source: <https://www.netlib.org/slatec/lin/balbak.f>. Native symbol: `balbak_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SCALE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::balbak;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a real general matrix from the eigenvectors of matrix output from BALANC.
        ///
        /// Original SLATEC routine: `BALBAK`; source: <https://www.netlib.org/slatec/lin/balbak.f>. Native symbol: `balbak_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SCALE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "balbak_"]
        pub fn balbak(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            scale: *mut f32,
            m: *mut crate::FortranInteger,
            z: *mut f32,
        );
    }

    // raw-api-routine: BANDR
    /// Reduce a real symmetric band matrix to symmetric tridiagonal matrix and, optionally, accumulate orthogonal similarity transformations.
    ///
    /// Original SLATEC routine: `BANDR`; source: <https://www.netlib.org/slatec/lin/bandr.f>. Native symbol: `bandr_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::logical::bandr;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a real symmetric band matrix to symmetric tridiagonal matrix and, optionally, accumulate orthogonal similarity transformations.
        ///
        /// Original SLATEC routine: `BANDR`; source: <https://www.netlib.org/slatec/lin/bandr.f>. Native symbol: `bandr_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "bandr_"]
        pub fn bandr(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            mb: *mut crate::FortranInteger,
            a: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            matz: *mut crate::FortranLogical,
            z: *mut f32,
        );
    }

    // raw-api-routine: BANDV
    /// Form the eigenvectors of a real symmetric band matrix associated with a set of ordered approximate eigenvalues by inverse iteration.
    ///
    /// Original SLATEC routine: `BANDV`; source: <https://www.netlib.org/slatec/lin/bandv.f>. Native symbol: `bandv_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MBW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E21`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV6`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::bandv;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a real symmetric band matrix associated with a set of ordered approximate eigenvalues by inverse iteration.
        ///
        /// Original SLATEC routine: `BANDV`; source: <https://www.netlib.org/slatec/lin/bandv.f>. Native symbol: `bandv_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MBW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E21`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV6`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "bandv_"]
        pub fn bandv(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            mbw: *mut crate::FortranInteger,
            a: *mut f32,
            e21: *mut f32,
            m: *mut crate::FortranInteger,
            w: *mut f32,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
            nv: *mut crate::FortranInteger,
            rv: *mut f32,
            rv6: *mut f32,
        );
    }

    // raw-api-routine: BISECT
    /// Compute the eigenvalues of a symmetric tridiagonal matrix in a given interval using Sturm sequencing.
    ///
    /// Original SLATEC routine: `BISECT`; source: <https://www.netlib.org/slatec/lin/bisect.f>. Native symbol: `bisect_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `UB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV4`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV5`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::bisect;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of a symmetric tridiagonal matrix in a given interval using Sturm sequencing.
        ///
        /// Original SLATEC routine: `BISECT`; source: <https://www.netlib.org/slatec/lin/bisect.f>. Native symbol: `bisect_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `UB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV4`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV5`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "bisect_"]
        pub fn bisect(
            n: *mut crate::FortranInteger,
            eps1: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            lb: *mut f32,
            ub: *mut f32,
            mm: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            w: *mut f32,
            ind: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
            rv4: *mut f32,
            rv5: *mut f32,
        );
    }

    // raw-api-routine: BQR
    /// Compute some of the eigenvalues of a real symmetric matrix using the QR method with shifts of origin.
    ///
    /// Original SLATEC routine: `BQR`; source: <https://www.netlib.org/slatec/lin/bqr.f>. Native symbol: `bqr_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `T`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `R`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::bqr;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute some of the eigenvalues of a real symmetric matrix using the QR method with shifts of origin.
        ///
        /// Original SLATEC routine: `BQR`; source: <https://www.netlib.org/slatec/lin/bqr.f>. Native symbol: `bqr_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `T`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `R`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "bqr_"]
        pub fn bqr(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            mb: *mut crate::FortranInteger,
            a: *mut f32,
            t: *mut f32,
            r: *mut f32,
            ierr: *mut crate::FortranInteger,
            nv: *mut crate::FortranInteger,
            rv: *mut f32,
        );
    }

    // raw-api-routine: CBABK2
    /// Form the eigenvectors of a complex general matrix from the eigenvectors of matrix output from CBAL.
    ///
    /// Original SLATEC routine: `CBABK2`; source: <https://www.netlib.org/slatec/lin/cbabk2.f>. Native symbol: `cbabk2_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SCALE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::cbabk2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a complex general matrix from the eigenvectors of matrix output from CBAL.
        ///
        /// Original SLATEC routine: `CBABK2`; source: <https://www.netlib.org/slatec/lin/cbabk2.f>. Native symbol: `cbabk2_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SCALE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "cbabk2_"]
        pub fn cbabk2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            scale: *mut f32,
            m: *mut crate::FortranInteger,
            zr: *mut f32,
            zi: *mut f32,
        );
    }

    // raw-api-routine: CBAL
    /// Balance a complex general matrix and isolate eigenvalues whenever possible.
    ///
    /// Original SLATEC routine: `CBAL`; source: <https://www.netlib.org/slatec/lin/cbal.f>. Native symbol: `cbal_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SCALE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::cbal;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Balance a complex general matrix and isolate eigenvalues whenever possible.
        ///
        /// Original SLATEC routine: `CBAL`; source: <https://www.netlib.org/slatec/lin/cbal.f>. Native symbol: `cbal_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SCALE`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "cbal_"]
        pub fn cbal(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            scale: *mut f32,
        );
    }

    // raw-api-routine: CG
    /// Compute the eigenvalues and, optionally, the eigenvectors of a complex general matrix.
    ///
    /// Original SLATEC routine: `CG`; source: <https://www.netlib.org/slatec/lin/cg.f>. Native symbol: `cg_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV3`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::cg;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a complex general matrix.
        ///
        /// Original SLATEC routine: `CG`; source: <https://www.netlib.org/slatec/lin/cg.f>. Native symbol: `cg_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV3`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "cg_"]
        pub fn cg(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            matz: *mut crate::FortranInteger,
            zr: *mut f32,
            zi: *mut f32,
            fv1: *mut f32,
            fv2: *mut f32,
            fv3: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: CGEEV
    /// Compute the eigenvalues and, optionally, the eigenvectors of a complex general matrix.
    ///
    /// Original SLATEC routine: `CGEEV`; source: <https://www.netlib.org/slatec/src/cgeev.f>. Native symbol: `cgeev_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LDA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `V`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LDV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::cgeev;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a complex general matrix.
        ///
        /// Original SLATEC routine: `CGEEV`; source: <https://www.netlib.org/slatec/src/cgeev.f>. Native symbol: `cgeev_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LDA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `V`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LDV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "cgeev_"]
        pub fn cgeev(
            a: *mut f32,
            lda: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            e: *mut f32,
            v: *mut f32,
            ldv: *mut crate::FortranInteger,
            work: *mut f32,
            job: *mut crate::FortranInteger,
            info: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: CH
    /// Compute the eigenvalues and, optionally, the eigenvectors of a complex Hermitian matrix.
    ///
    /// Original SLATEC routine: `CH`; source: <https://www.netlib.org/slatec/lin/ch.f>. Native symbol: `ch_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FM1`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::ch;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a complex Hermitian matrix.
        ///
        /// Original SLATEC routine: `CH`; source: <https://www.netlib.org/slatec/lin/ch.f>. Native symbol: `ch_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FM1`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "ch_"]
        pub fn ch(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            w: *mut f32,
            matz: *mut crate::FortranInteger,
            zr: *mut f32,
            zi: *mut f32,
            fv1: *mut f32,
            fv2: *mut f32,
            fm1: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: CHIEV
    /// Compute the eigenvalues and, optionally, the eigenvectors of a complex Hermitian matrix.
    ///
    /// Original SLATEC routine: `CHIEV`; source: <https://www.netlib.org/slatec/src/chiev.f>. Native symbol: `chiev_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LDA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `V`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LDV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::chiev;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a complex Hermitian matrix.
        ///
        /// Original SLATEC routine: `CHIEV`; source: <https://www.netlib.org/slatec/src/chiev.f>. Native symbol: `chiev_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LDA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `V`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LDV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "chiev_"]
        pub fn chiev(
            a: *mut f32,
            lda: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            e: *mut f32,
            v: *mut f32,
            ldv: *mut crate::FortranInteger,
            work: *mut f32,
            job: *mut crate::FortranInteger,
            info: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: CINVIT
    /// Compute the eigenvectors of a complex upper Hessenberg associated with specified eigenvalues using inverse iteration.
    ///
    /// Original SLATEC routine: `CINVIT`; source: <https://www.netlib.org/slatec/lin/cinvit.f>. Native symbol: `cinvit_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SELECT`: declared `LOGICAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RM1`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RM2`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::logical::cinvit;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvectors of a complex upper Hessenberg associated with specified eigenvalues using inverse iteration.
        ///
        /// Original SLATEC routine: `CINVIT`; source: <https://www.netlib.org/slatec/lin/cinvit.f>. Native symbol: `cinvit_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SELECT`: declared `LOGICAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RM1`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RM2`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "cinvit_"]
        pub fn cinvit(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            select: *mut crate::FortranLogical,
            mm: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            zr: *mut f32,
            zi: *mut f32,
            ierr: *mut crate::FortranInteger,
            rm1: *mut f32,
            rm2: *mut f32,
            rv1: *mut f32,
            rv2: *mut f32,
        );
    }

    // raw-api-routine: COMBAK
    /// Form the eigenvectors of a complex general matrix from the eigenvectors of a upper Hessenberg matrix output from COMHES.
    ///
    /// Original SLATEC routine: `COMBAK`; source: <https://www.netlib.org/slatec/lin/combak.f>. Native symbol: `combak_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::combak;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a complex general matrix from the eigenvectors of a upper Hessenberg matrix output from COMHES.
        ///
        /// Original SLATEC routine: `COMBAK`; source: <https://www.netlib.org/slatec/lin/combak.f>. Native symbol: `combak_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "combak_"]
        pub fn combak(
            nm: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            int: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            zr: *mut f32,
            zi: *mut f32,
        );
    }

    // raw-api-routine: COMHES
    /// Reduce a complex general matrix to complex upper Hessenberg form using stabilized elementary similarity transformations.
    ///
    /// Original SLATEC routine: `COMHES`; source: <https://www.netlib.org/slatec/lin/comhes.f>. Native symbol: `comhes_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::comhes;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a complex general matrix to complex upper Hessenberg form using stabilized elementary similarity transformations.
        ///
        /// Original SLATEC routine: `COMHES`; source: <https://www.netlib.org/slatec/lin/comhes.f>. Native symbol: `comhes_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "comhes_"]
        pub fn comhes(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            int: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: COMLR
    /// Compute the eigenvalues of a complex upper Hessenberg matrix using the modified LR method.
    ///
    /// Original SLATEC routine: `COMLR`; source: <https://www.netlib.org/slatec/lin/comlr.f>. Native symbol: `comlr_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `HR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `HI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::comlr;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of a complex upper Hessenberg matrix using the modified LR method.
        ///
        /// Original SLATEC routine: `COMLR`; source: <https://www.netlib.org/slatec/lin/comlr.f>. Native symbol: `comlr_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `HR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `HI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "comlr_"]
        pub fn comlr(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            hr: *mut f32,
            hi: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: COMLR2
    /// Compute the eigenvalues and eigenvectors of a complex upper Hessenberg matrix using the modified LR method.
    ///
    /// Original SLATEC routine: `COMLR2`; source: <https://www.netlib.org/slatec/lin/comlr2.f>. Native symbol: `comlr2_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `HR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `HI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::comlr2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and eigenvectors of a complex upper Hessenberg matrix using the modified LR method.
        ///
        /// Original SLATEC routine: `COMLR2`; source: <https://www.netlib.org/slatec/lin/comlr2.f>. Native symbol: `comlr2_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `HR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `HI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "comlr2_"]
        pub fn comlr2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            int: *mut crate::FortranInteger,
            hr: *mut f32,
            hi: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            zr: *mut f32,
            zi: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: COMQR
    /// Compute the eigenvalues of complex upper Hessenberg matrix using the QR method.
    ///
    /// Original SLATEC routine: `COMQR`; source: <https://www.netlib.org/slatec/lin/comqr.f>. Native symbol: `comqr_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `HR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `HI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::comqr;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of complex upper Hessenberg matrix using the QR method.
        ///
        /// Original SLATEC routine: `COMQR`; source: <https://www.netlib.org/slatec/lin/comqr.f>. Native symbol: `comqr_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `HR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `HI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "comqr_"]
        pub fn comqr(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            hr: *mut f32,
            hi: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: COMQR2
    /// Compute the eigenvalues and eigenvectors of a complex upper Hessenberg matrix.
    ///
    /// Original SLATEC routine: `COMQR2`; source: <https://www.netlib.org/slatec/lin/comqr2.f>. Native symbol: `comqr2_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORTR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORTI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `HR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `HI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::comqr2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and eigenvectors of a complex upper Hessenberg matrix.
        ///
        /// Original SLATEC routine: `COMQR2`; source: <https://www.netlib.org/slatec/lin/comqr2.f>. Native symbol: `comqr2_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORTR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORTI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `HR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `HI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "comqr2_"]
        pub fn comqr2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            ortr: *mut f32,
            orti: *mut f32,
            hr: *mut f32,
            hi: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            zr: *mut f32,
            zi: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: CORTB
    /// Form the eigenvectors of a complex general matrix from eigenvectors of upper Hessenberg matrix output from CORTH.
    ///
    /// Original SLATEC routine: `CORTB`; source: <https://www.netlib.org/slatec/lin/cortb.f>. Native symbol: `cortb_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORTR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORTI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::cortb;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a complex general matrix from eigenvectors of upper Hessenberg matrix output from CORTH.
        ///
        /// Original SLATEC routine: `CORTB`; source: <https://www.netlib.org/slatec/lin/cortb.f>. Native symbol: `cortb_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORTR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORTI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "cortb_"]
        pub fn cortb(
            nm: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            ortr: *mut f32,
            orti: *mut f32,
            m: *mut crate::FortranInteger,
            zr: *mut f32,
            zi: *mut f32,
        );
    }

    // raw-api-routine: CORTH
    /// Reduce a complex general matrix to complex upper Hessenberg form using unitary similarity transformations.
    ///
    /// Original SLATEC routine: `CORTH`; source: <https://www.netlib.org/slatec/lin/corth.f>. Native symbol: `corth_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORTR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORTI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::corth;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a complex general matrix to complex upper Hessenberg form using unitary similarity transformations.
        ///
        /// Original SLATEC routine: `CORTH`; source: <https://www.netlib.org/slatec/lin/corth.f>. Native symbol: `corth_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORTR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORTI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "corth_"]
        pub fn corth(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            ortr: *mut f32,
            orti: *mut f32,
        );
    }

    // raw-api-routine: ELMBAK
    /// Form the eigenvectors of a real general matrix from the eigenvectors of the upper Hessenberg matrix output from ELMHES.
    ///
    /// Original SLATEC routine: `ELMBAK`; source: <https://www.netlib.org/slatec/lin/elmbak.f>. Native symbol: `elmbak_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::elmbak;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a real general matrix from the eigenvectors of the upper Hessenberg matrix output from ELMHES.
        ///
        /// Original SLATEC routine: `ELMBAK`; source: <https://www.netlib.org/slatec/lin/elmbak.f>. Native symbol: `elmbak_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "elmbak_"]
        pub fn elmbak(
            nm: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            a: *mut f32,
            int: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            z: *mut f32,
        );
    }

    // raw-api-routine: ELMHES
    /// Reduce a real general matrix to upper Hessenberg form using stabilized elementary similarity transformations.
    ///
    /// Original SLATEC routine: `ELMHES`; source: <https://www.netlib.org/slatec/lin/elmhes.f>. Native symbol: `elmhes_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::elmhes;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a real general matrix to upper Hessenberg form using stabilized elementary similarity transformations.
        ///
        /// Original SLATEC routine: `ELMHES`; source: <https://www.netlib.org/slatec/lin/elmhes.f>. Native symbol: `elmhes_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "elmhes_"]
        pub fn elmhes(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            a: *mut f32,
            int: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: ELTRAN
    /// Accumulates the stabilized elementary similarity transformations used in the reduction of a real general matrix to upper Hessenberg form by ELMHES.
    ///
    /// Original SLATEC routine: `ELTRAN`; source: <https://www.netlib.org/slatec/lin/eltran.f>. Native symbol: `eltran_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::eltran;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Accumulates the stabilized elementary similarity transformations used in the reduction of a real general matrix to upper Hessenberg form by ELMHES.
        ///
        /// Original SLATEC routine: `ELTRAN`; source: <https://www.netlib.org/slatec/lin/eltran.f>. Native symbol: `eltran_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INT`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "eltran_"]
        pub fn eltran(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            a: *mut f32,
            int: *mut crate::FortranInteger,
            z: *mut f32,
        );
    }

    // raw-api-routine: FIGI
    /// Transforms certain real non-symmetric tridiagonal matrix to symmetric tridiagonal matrix.
    ///
    /// Original SLATEC routine: `FIGI`; source: <https://www.netlib.org/slatec/lin/figi.f>. Native symbol: `figi_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `T`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::figi;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Transforms certain real non-symmetric tridiagonal matrix to symmetric tridiagonal matrix.
        ///
        /// Original SLATEC routine: `FIGI`; source: <https://www.netlib.org/slatec/lin/figi.f>. Native symbol: `figi_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `T`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "figi_"]
        pub fn figi(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            t: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: FIGI2
    /// Transforms certain real non-symmetric tridiagonal matrix to symmetric tridiagonal matrix.
    ///
    /// Original SLATEC routine: `FIGI2`; source: <https://www.netlib.org/slatec/lin/figi2.f>. Native symbol: `figi2_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `T`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::figi2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Transforms certain real non-symmetric tridiagonal matrix to symmetric tridiagonal matrix.
        ///
        /// Original SLATEC routine: `FIGI2`; source: <https://www.netlib.org/slatec/lin/figi2.f>. Native symbol: `figi2_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `T`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "figi2_"]
        pub fn figi2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            t: *mut f32,
            d: *mut f32,
            e: *mut f32,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: HQR
    /// Compute the eigenvalues of a real upper Hessenberg matrix using the QR method.
    ///
    /// Original SLATEC routine: `HQR`; source: <https://www.netlib.org/slatec/lin/hqr.f>. Native symbol: `hqr_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `H`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::hqr;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of a real upper Hessenberg matrix using the QR method.
        ///
        /// Original SLATEC routine: `HQR`; source: <https://www.netlib.org/slatec/lin/hqr.f>. Native symbol: `hqr_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `H`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "hqr_"]
        pub fn hqr(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            h: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: HQR2
    /// Compute the eigenvalues and eigenvectors of a real upper Hessenberg matrix using QR method.
    ///
    /// Original SLATEC routine: `HQR2`; source: <https://www.netlib.org/slatec/lin/hqr2.f>. Native symbol: `hqr2_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `H`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::hqr2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and eigenvectors of a real upper Hessenberg matrix using QR method.
        ///
        /// Original SLATEC routine: `HQR2`; source: <https://www.netlib.org/slatec/lin/hqr2.f>. Native symbol: `hqr2_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `H`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "hqr2_"]
        pub fn hqr2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            h: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: HTRIB3
    /// Compute the eigenvectors of a complex Hermitian matrix from the eigenvectors of a real symmetric tridiagonal matrix output from HTRID3.
    ///
    /// Original SLATEC routine: `HTRIB3`; source: <https://www.netlib.org/slatec/lin/htrib3.f>. Native symbol: `htrib3_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `TAU`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::htrib3;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvectors of a complex Hermitian matrix from the eigenvectors of a real symmetric tridiagonal matrix output from HTRID3.
        ///
        /// Original SLATEC routine: `HTRIB3`; source: <https://www.netlib.org/slatec/lin/htrib3.f>. Native symbol: `htrib3_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TAU`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "htrib3_"]
        pub fn htrib3(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            tau: *mut f32,
            m: *mut crate::FortranInteger,
            zr: *mut f32,
            zi: *mut f32,
        );
    }

    // raw-api-routine: HTRIBK
    /// Form the eigenvectors of a complex Hermitian matrix from the eigenvectors of a real symmetric tridiagonal matrix output from HTRIDI.
    ///
    /// Original SLATEC routine: `HTRIBK`; source: <https://www.netlib.org/slatec/lin/htribk.f>. Native symbol: `htribk_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `TAU`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::htribk;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a complex Hermitian matrix from the eigenvectors of a real symmetric tridiagonal matrix output from HTRIDI.
        ///
        /// Original SLATEC routine: `HTRIBK`; source: <https://www.netlib.org/slatec/lin/htribk.f>. Native symbol: `htribk_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TAU`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ZI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "htribk_"]
        pub fn htribk(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            tau: *mut f32,
            m: *mut crate::FortranInteger,
            zr: *mut f32,
            zi: *mut f32,
        );
    }

    // raw-api-routine: HTRID3
    /// Reduce a complex Hermitian (packed) matrix to a real symmetric tridiagonal matrix by unitary similarity transformations.
    ///
    /// Original SLATEC routine: `HTRID3`; source: <https://www.netlib.org/slatec/lin/htrid3.f>. Native symbol: `htrid3_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `TAU`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::htrid3;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a complex Hermitian (packed) matrix to a real symmetric tridiagonal matrix by unitary similarity transformations.
        ///
        /// Original SLATEC routine: `HTRID3`; source: <https://www.netlib.org/slatec/lin/htrid3.f>. Native symbol: `htrid3_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TAU`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "htrid3_"]
        pub fn htrid3(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            tau: *mut f32,
        );
    }

    // raw-api-routine: HTRIDI
    /// Reduce a complex Hermitian matrix to a real symmetric tridiagonal matrix using unitary similarity transformations.
    ///
    /// Original SLATEC routine: `HTRIDI`; source: <https://www.netlib.org/slatec/lin/htridi.f>. Native symbol: `htridi_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `TAU`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::htridi;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a complex Hermitian matrix to a real symmetric tridiagonal matrix using unitary similarity transformations.
        ///
        /// Original SLATEC routine: `HTRIDI`; source: <https://www.netlib.org/slatec/lin/htridi.f>. Native symbol: `htridi_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AR`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `AI`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TAU`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "htridi_"]
        pub fn htridi(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            ar: *mut f32,
            ai: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            tau: *mut f32,
        );
    }

    // raw-api-routine: IMTQL1
    /// Compute the eigenvalues of a symmetric tridiagonal matrix using the implicit QL method.
    ///
    /// Original SLATEC routine: `IMTQL1`; source: <https://www.netlib.org/slatec/lin/imtql1.f>. Native symbol: `imtql1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::imtql1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of a symmetric tridiagonal matrix using the implicit QL method.
        ///
        /// Original SLATEC routine: `IMTQL1`; source: <https://www.netlib.org/slatec/lin/imtql1.f>. Native symbol: `imtql1_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "imtql1_"]
        pub fn imtql1(
            n: *mut crate::FortranInteger,
            d: *mut f32,
            e: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: IMTQL2
    /// Compute the eigenvalues and eigenvectors of a symmetric tridiagonal matrix using the implicit QL method.
    ///
    /// Original SLATEC routine: `IMTQL2`; source: <https://www.netlib.org/slatec/lin/imtql2.f>. Native symbol: `imtql2_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::imtql2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and eigenvectors of a symmetric tridiagonal matrix using the implicit QL method.
        ///
        /// Original SLATEC routine: `IMTQL2`; source: <https://www.netlib.org/slatec/lin/imtql2.f>. Native symbol: `imtql2_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "imtql2_"]
        pub fn imtql2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            d: *mut f32,
            e: *mut f32,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: IMTQLV
    /// Compute the eigenvalues of a symmetric tridiagonal matrix using the implicit QL method. Eigenvectors may be computed later.
    ///
    /// Original SLATEC routine: `IMTQLV`; source: <https://www.netlib.org/slatec/lin/imtqlv.f>. Native symbol: `imtqlv_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::imtqlv;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of a symmetric tridiagonal matrix using the implicit QL method. Eigenvectors may be computed later.
        ///
        /// Original SLATEC routine: `IMTQLV`; source: <https://www.netlib.org/slatec/lin/imtqlv.f>. Native symbol: `imtqlv_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "imtqlv_"]
        pub fn imtqlv(
            n: *mut crate::FortranInteger,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            w: *mut f32,
            ind: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
            rv1: *mut f32,
        );
    }

    // raw-api-routine: INVIT
    /// Compute the eigenvectors of a real upper Hessenberg matrix associated with specified eigenvalues by inverse iteration.
    ///
    /// Original SLATEC routine: `INVIT`; source: <https://www.netlib.org/slatec/lin/invit.f>. Native symbol: `invit_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `SELECT`: declared `LOGICAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RM1`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::logical::invit;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvectors of a real upper Hessenberg matrix associated with specified eigenvalues by inverse iteration.
        ///
        /// Original SLATEC routine: `INVIT`; source: <https://www.netlib.org/slatec/lin/invit.f>. Native symbol: `invit_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `SELECT`: declared `LOGICAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RM1`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "invit_"]
        pub fn invit(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            select: *mut crate::FortranLogical,
            mm: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
            rm1: *mut f32,
            rv1: *mut f32,
            rv2: *mut f32,
        );
    }

    // raw-api-routine: ORTBAK
    /// Form the eigenvectors of a general real matrix from the eigenvectors of the upper Hessenberg matrix output from ORTHES.
    ///
    /// Original SLATEC routine: `ORTBAK`; source: <https://www.netlib.org/slatec/lin/ortbak.f>. Native symbol: `ortbak_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::ortbak;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a general real matrix from the eigenvectors of the upper Hessenberg matrix output from ORTHES.
        ///
        /// Original SLATEC routine: `ORTBAK`; source: <https://www.netlib.org/slatec/lin/ortbak.f>. Native symbol: `ortbak_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "ortbak_"]
        pub fn ortbak(
            nm: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            a: *mut f32,
            ort: *mut f32,
            m: *mut crate::FortranInteger,
            z: *mut f32,
        );
    }

    // raw-api-routine: ORTHES
    /// Reduce a real general matrix to upper Hessenberg form using orthogonal similarity transformations.
    ///
    /// Original SLATEC routine: `ORTHES`; source: <https://www.netlib.org/slatec/lin/orthes.f>. Native symbol: `orthes_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::orthes;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a real general matrix to upper Hessenberg form using orthogonal similarity transformations.
        ///
        /// Original SLATEC routine: `ORTHES`; source: <https://www.netlib.org/slatec/lin/orthes.f>. Native symbol: `orthes_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "orthes_"]
        pub fn orthes(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            a: *mut f32,
            ort: *mut f32,
        );
    }

    // raw-api-routine: ORTRAN
    /// Accumulate orthogonal similarity transformations in the reduction of real general matrix by ORTHES.
    ///
    /// Original SLATEC routine: `ORTRAN`; source: <https://www.netlib.org/slatec/lin/ortran.f>. Native symbol: `ortran_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ORT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::ortran;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Accumulate orthogonal similarity transformations in the reduction of real general matrix by ORTHES.
        ///
        /// Original SLATEC routine: `ORTRAN`; source: <https://www.netlib.org/slatec/lin/ortran.f>. Native symbol: `ortran_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LOW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IGH`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ORT`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "ortran_"]
        pub fn ortran(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            low: *mut crate::FortranInteger,
            igh: *mut crate::FortranInteger,
            a: *mut f32,
            ort: *mut f32,
            z: *mut f32,
        );
    }

    // raw-api-routine: QZHES
    /// The first step of the QZ algorithm for solving generalized matrix eigenproblems. Accepts a pair of real general matrices and reduces one of them to upper Hessenberg and the other to upper triangular form using orthogonal transformations. Usually followed by QZIT, QZVAL, QZVEC.
    ///
    /// Original SLATEC routine: `QZHES`; source: <https://www.netlib.org/slatec/lin/qzhes.f>. Native symbol: `qzhes_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_fortran_logical_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::logical::qzhes;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// The first step of the QZ algorithm for solving generalized matrix eigenproblems. Accepts a pair of real general matrices and reduces one of them to upper Hessenberg and the other to upper triangular form using orthogonal transformations. Usually followed by QZIT, QZVAL, QZVEC.
        ///
        /// Original SLATEC routine: `QZHES`; source: <https://www.netlib.org/slatec/lin/qzhes.f>. Native symbol: `qzhes_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_fortran_logical_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "qzhes_"]
        pub fn qzhes(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            matz: *mut crate::FortranLogical,
            z: *mut f32,
        );
    }

    // raw-api-routine: QZIT
    /// The second step of the QZ algorithm for generalized eigenproblems. Accepts an upper Hessenberg and an upper triangular matrix and reduces the former to quasi-triangular form while preserving the form of the latter. Usually preceded by QZHES and followed by QZVAL and QZVEC.
    ///
    /// Original SLATEC routine: `QZIT`; source: <https://www.netlib.org/slatec/lin/qzit.f>. Native symbol: `qzit_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_fortran_logical_i32,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::logical::qzit;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// The second step of the QZ algorithm for generalized eigenproblems. Accepts an upper Hessenberg and an upper triangular matrix and reduces the former to quasi-triangular form while preserving the form of the latter. Usually preceded by QZHES and followed by QZVAL and QZVEC.
        ///
        /// Original SLATEC routine: `QZIT`; source: <https://www.netlib.org/slatec/lin/qzit.f>. Native symbol: `qzit_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32,mut_fortran_logical_i32,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "qzit_"]
        pub fn qzit(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            eps1: *mut f32,
            matz: *mut crate::FortranLogical,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: QZVAL
    /// The third step of the QZ algorithm for generalized eigenproblems. Accepts a pair of real matrices, one in quasi-triangular form and the other in upper triangular form and computes the eigenvalues of the associated eigenproblem. Usually preceded by QZHES, QZIT, and followed by QZVEC.
    ///
    /// Original SLATEC routine: `QZVAL`; source: <https://www.netlib.org/slatec/lin/qzval.f>. Native symbol: `qzval_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALFR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALFI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BETA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::logical::qzval;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// The third step of the QZ algorithm for generalized eigenproblems. Accepts a pair of real matrices, one in quasi-triangular form and the other in upper triangular form and computes the eigenvalues of the associated eigenproblem. Usually preceded by QZHES, QZIT, and followed by QZVEC.
        ///
        /// Original SLATEC routine: `QZVAL`; source: <https://www.netlib.org/slatec/lin/qzval.f>. Native symbol: `qzval_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALFR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALFI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BETA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "qzval_"]
        pub fn qzval(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            alfr: *mut f32,
            alfi: *mut f32,
            beta: *mut f32,
            matz: *mut crate::FortranLogical,
            z: *mut f32,
        );
    }

    // raw-api-routine: QZVEC
    /// The optional fourth step of the QZ algorithm for generalized eigenproblems. Accepts a matrix in quasi-triangular form and another in upper triangular and computes the eigenvectors of the triangular problem and transforms them back to the original coordinates Usually preceded by QZHES, QZIT, and QZVAL.
    ///
    /// Original SLATEC routine: `QZVEC`; source: <https://www.netlib.org/slatec/lin/qzvec.f>. Native symbol: `qzvec_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALFR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALFI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BETA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::qzvec;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// The optional fourth step of the QZ algorithm for generalized eigenproblems. Accepts a matrix in quasi-triangular form and another in upper triangular and computes the eigenvectors of the triangular problem and transforms them back to the original coordinates Usually preceded by QZHES, QZIT, and QZVAL.
        ///
        /// Original SLATEC routine: `QZVEC`; source: <https://www.netlib.org/slatec/lin/qzvec.f>. Native symbol: `qzvec_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALFR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALFI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BETA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "qzvec_"]
        pub fn qzvec(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            alfr: *mut f32,
            alfi: *mut f32,
            beta: *mut f32,
            z: *mut f32,
        );
    }

    // raw-api-routine: RATQR
    /// Compute the largest or smallest eigenvalues of a symmetric tridiagonal matrix using the rational QR method with Newton correction.
    ///
    /// Original SLATEC routine: `RATQR`; source: <https://www.netlib.org/slatec/lin/ratqr.f>. Native symbol: `ratqr_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BD`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `TYPE`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IDEF`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::logical::ratqr;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the largest or smallest eigenvalues of a symmetric tridiagonal matrix using the rational QR method with Newton correction.
        ///
        /// Original SLATEC routine: `RATQR`; source: <https://www.netlib.org/slatec/lin/ratqr.f>. Native symbol: `ratqr_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_fortran_logical_i32,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BD`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TYPE`: declared `LOGICAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IDEF`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "ratqr_"]
        pub fn ratqr(
            n: *mut crate::FortranInteger,
            eps1: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            m: *mut crate::FortranInteger,
            w: *mut f32,
            ind: *mut crate::FortranInteger,
            bd: *mut f32,
            r#type: *mut crate::FortranLogical,
            idef: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: REBAK
    /// Form the eigenvectors of a generalized symmetric eigensystem from the eigenvectors of derived matrix output from REDUC or REDUC2.
    ///
    /// Original SLATEC routine: `REBAK`; source: <https://www.netlib.org/slatec/lin/rebak.f>. Native symbol: `rebak_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `DL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rebak;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a generalized symmetric eigensystem from the eigenvectors of derived matrix output from REDUC or REDUC2.
        ///
        /// Original SLATEC routine: `REBAK`; source: <https://www.netlib.org/slatec/lin/rebak.f>. Native symbol: `rebak_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `DL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rebak_"]
        pub fn rebak(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            b: *mut f32,
            dl: *mut f32,
            m: *mut crate::FortranInteger,
            z: *mut f32,
        );
    }

    // raw-api-routine: REBAKB
    /// Form the eigenvectors of a generalized symmetric eigensystem from the eigenvectors of derived matrix output from REDUC2.
    ///
    /// Original SLATEC routine: `REBAKB`; source: <https://www.netlib.org/slatec/lin/rebakb.f>. Native symbol: `rebakb_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `DL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rebakb;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a generalized symmetric eigensystem from the eigenvectors of derived matrix output from REDUC2.
        ///
        /// Original SLATEC routine: `REBAKB`; source: <https://www.netlib.org/slatec/lin/rebakb.f>. Native symbol: `rebakb_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `DL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rebakb_"]
        pub fn rebakb(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            b: *mut f32,
            dl: *mut f32,
            m: *mut crate::FortranInteger,
            z: *mut f32,
        );
    }

    // raw-api-routine: REDUC
    /// Reduce a generalized symmetric eigenproblem to a standard symmetric eigenproblem using Cholesky factorization.
    ///
    /// Original SLATEC routine: `REDUC`; source: <https://www.netlib.org/slatec/lin/reduc.f>. Native symbol: `reduc_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `DL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::reduc;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a generalized symmetric eigenproblem to a standard symmetric eigenproblem using Cholesky factorization.
        ///
        /// Original SLATEC routine: `REDUC`; source: <https://www.netlib.org/slatec/lin/reduc.f>. Native symbol: `reduc_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `DL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "reduc_"]
        pub fn reduc(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            dl: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: REDUC2
    /// Reduce a certain generalized symmetric eigenproblem to a standard symmetric eigenproblem using Cholesky factorization.
    ///
    /// Original SLATEC routine: `REDUC2`; source: <https://www.netlib.org/slatec/lin/reduc2.f>. Native symbol: `reduc2_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `DL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::reduc2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a certain generalized symmetric eigenproblem to a standard symmetric eigenproblem using Cholesky factorization.
        ///
        /// Original SLATEC routine: `REDUC2`; source: <https://www.netlib.org/slatec/lin/reduc2.f>. Native symbol: `reduc2_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `DL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "reduc2_"]
        pub fn reduc2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            dl: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RG
    /// Compute the eigenvalues and, optionally, the eigenvectors of a real general matrix.
    ///
    /// Original SLATEC routine: `RG`; source: <https://www.netlib.org/slatec/lin/rg.f>. Native symbol: `rg_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IV1`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rg;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a real general matrix.
        ///
        /// Original SLATEC routine: `RG`; source: <https://www.netlib.org/slatec/lin/rg.f>. Native symbol: `rg_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IV1`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rg_"]
        pub fn rg(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            wr: *mut f32,
            wi: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            iv1: *mut crate::FortranInteger,
            fv1: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RGG
    /// Compute the eigenvalues and eigenvectors for a real generalized eigenproblem.
    ///
    /// Original SLATEC routine: `RGG`; source: <https://www.netlib.org/slatec/lin/rgg.f>. Native symbol: `rgg_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALFR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `ALFI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `BETA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rgg;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and eigenvectors for a real generalized eigenproblem.
        ///
        /// Original SLATEC routine: `RGG`; source: <https://www.netlib.org/slatec/lin/rgg.f>. Native symbol: `rgg_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALFR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ALFI`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `BETA`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rgg_"]
        pub fn rgg(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            alfr: *mut f32,
            alfi: *mut f32,
            beta: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RS
    /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix.
    ///
    /// Original SLATEC routine: `RS`; source: <https://www.netlib.org/slatec/lin/rs.f>. Native symbol: `rs_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rs;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix.
        ///
        /// Original SLATEC routine: `RS`; source: <https://www.netlib.org/slatec/lin/rs.f>. Native symbol: `rs_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rs_"]
        pub fn rs(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            w: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            fv1: *mut f32,
            fv2: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RSB
    /// Compute the eigenvalues and, optionally, the eigenvectors of a symmetric band matrix.
    ///
    /// Original SLATEC routine: `RSB`; source: <https://www.netlib.org/slatec/lin/rsb.f>. Native symbol: `rsb_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rsb;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a symmetric band matrix.
        ///
        /// Original SLATEC routine: `RSB`; source: <https://www.netlib.org/slatec/lin/rsb.f>. Native symbol: `rsb_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rsb_"]
        pub fn rsb(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            mb: *mut crate::FortranInteger,
            a: *mut f32,
            w: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            fv1: *mut f32,
            fv2: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RSG
    /// Compute the eigenvalues and, optionally, the eigenvectors of a symmetric generalized eigenproblem.
    ///
    /// Original SLATEC routine: `RSG`; source: <https://www.netlib.org/slatec/lin/rsg.f>. Native symbol: `rsg_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rsg;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a symmetric generalized eigenproblem.
        ///
        /// Original SLATEC routine: `RSG`; source: <https://www.netlib.org/slatec/lin/rsg.f>. Native symbol: `rsg_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rsg_"]
        pub fn rsg(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            w: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            fv1: *mut f32,
            fv2: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RSGAB
    /// Compute the eigenvalues and, optionally, the eigenvectors of a symmetric generalized eigenproblem.
    ///
    /// Original SLATEC routine: `RSGAB`; source: <https://www.netlib.org/slatec/lin/rsgab.f>. Native symbol: `rsgab_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rsgab;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a symmetric generalized eigenproblem.
        ///
        /// Original SLATEC routine: `RSGAB`; source: <https://www.netlib.org/slatec/lin/rsgab.f>. Native symbol: `rsgab_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rsgab_"]
        pub fn rsgab(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            w: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            fv1: *mut f32,
            fv2: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RSGBA
    /// Compute the eigenvalues and, optionally, the eigenvectors of a symmetric generalized eigenproblem.
    ///
    /// Original SLATEC routine: `RSGBA`; source: <https://www.netlib.org/slatec/lin/rsgba.f>. Native symbol: `rsgba_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rsgba;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a symmetric generalized eigenproblem.
        ///
        /// Original SLATEC routine: `RSGBA`; source: <https://www.netlib.org/slatec/lin/rsgba.f>. Native symbol: `rsgba_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `B`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rsgba_"]
        pub fn rsgba(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            b: *mut f32,
            w: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            fv1: *mut f32,
            fv2: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RSP
    /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix packed into a one dimensional array.
    ///
    /// Original SLATEC routine: `RSP`; source: <https://www.netlib.org/slatec/lin/rsp.f>. Native symbol: `rsp_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rsp;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix packed into a one dimensional array.
        ///
        /// Original SLATEC routine: `RSP`; source: <https://www.netlib.org/slatec/lin/rsp.f>. Native symbol: `rsp_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rsp_"]
        pub fn rsp(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            nv: *mut crate::FortranInteger,
            a: *mut f32,
            w: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            fv1: *mut f32,
            fv2: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RST
    /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric tridiagonal matrix.
    ///
    /// Original SLATEC routine: `RST`; source: <https://www.netlib.org/slatec/lin/rst.f>. Native symbol: `rst_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rst;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric tridiagonal matrix.
        ///
        /// Original SLATEC routine: `RST`; source: <https://www.netlib.org/slatec/lin/rst.f>. Native symbol: `rst_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rst_"]
        pub fn rst(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            w: *mut f32,
            e: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: RT
    /// Compute the eigenvalues and eigenvectors of a special real tridiagonal matrix.
    ///
    /// Original SLATEC routine: `RT`; source: <https://www.netlib.org/slatec/lin/rt.f>. Native symbol: `rt_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::rt;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and eigenvectors of a special real tridiagonal matrix.
        ///
        /// Original SLATEC routine: `RT`; source: <https://www.netlib.org/slatec/lin/rt.f>. Native symbol: `rt_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MATZ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `FV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "rt_"]
        pub fn rt(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            w: *mut f32,
            matz: *mut crate::FortranInteger,
            z: *mut f32,
            fv1: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: SGEEV
    /// Compute the eigenvalues and, optionally, the eigenvectors of a real general matrix.
    ///
    /// Original SLATEC routine: `SGEEV`; source: <https://www.netlib.org/slatec/src/sgeev.f>. Native symbol: `sgeev_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LDA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `V`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LDV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::sgeev;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a real general matrix.
        ///
        /// Original SLATEC routine: `SGEEV`; source: <https://www.netlib.org/slatec/src/sgeev.f>. Native symbol: `sgeev_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LDA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `V`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LDV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "sgeev_"]
        pub fn sgeev(
            a: *mut f32,
            lda: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            e: *mut f32,
            v: *mut f32,
            ldv: *mut crate::FortranInteger,
            work: *mut f32,
            job: *mut crate::FortranInteger,
            info: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: SSIEV
    /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix.
    ///
    /// Original SLATEC routine: `SSIEV`; source: <https://www.netlib.org/slatec/src/ssiev.f>. Native symbol: `ssiev_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LDA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::ssiev;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix.
        ///
        /// Original SLATEC routine: `SSIEV`; source: <https://www.netlib.org/slatec/src/ssiev.f>. Native symbol: `ssiev_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LDA`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "ssiev_"]
        pub fn ssiev(
            a: *mut f32,
            lda: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            e: *mut f32,
            work: *mut f32,
            job: *mut crate::FortranInteger,
            info: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: SSPEV
    /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix stored in packed form.
    ///
    /// Original SLATEC routine: `SSPEV`; source: <https://www.netlib.org/slatec/lin/sspev.f>. Native symbol: `sspev_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `V`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LDV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::sspev;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix stored in packed form.
        ///
        /// Original SLATEC routine: `SSPEV`; source: <https://www.netlib.org/slatec/lin/sspev.f>. Native symbol: `sspev_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `V`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LDV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `WORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `JOB`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INFO`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "sspev_"]
        pub fn sspev(
            a: *mut f32,
            n: *mut crate::FortranInteger,
            e: *mut f32,
            v: *mut f32,
            ldv: *mut crate::FortranInteger,
            work: *mut f32,
            job: *mut crate::FortranInteger,
            info: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: TINVIT
    /// Compute the eigenvectors of symmetric tridiagonal matrix corresponding to specified eigenvalues, using inverse iteration.
    ///
    /// Original SLATEC routine: `TINVIT`; source: <https://www.netlib.org/slatec/lin/tinvit.f>. Native symbol: `tinvit_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV3`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV4`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV6`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tinvit;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvectors of symmetric tridiagonal matrix corresponding to specified eigenvalues, using inverse iteration.
        ///
        /// Original SLATEC routine: `TINVIT`; source: <https://www.netlib.org/slatec/lin/tinvit.f>. Native symbol: `tinvit_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV3`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV4`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV6`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tinvit_"]
        pub fn tinvit(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            m: *mut crate::FortranInteger,
            w: *mut f32,
            ind: *mut crate::FortranInteger,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
            rv1: *mut f32,
            rv2: *mut f32,
            rv3: *mut f32,
            rv4: *mut f32,
            rv6: *mut f32,
        );
    }

    // raw-api-routine: TQL1
    /// Compute the eigenvalues of symmetric tridiagonal matrix by the QL method.
    ///
    /// Original SLATEC routine: `TQL1`; source: <https://www.netlib.org/slatec/lin/tql1.f>. Native symbol: `tql1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tql1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of symmetric tridiagonal matrix by the QL method.
        ///
        /// Original SLATEC routine: `TQL1`; source: <https://www.netlib.org/slatec/lin/tql1.f>. Native symbol: `tql1_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tql1_"]
        pub fn tql1(
            n: *mut crate::FortranInteger,
            d: *mut f32,
            e: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: TQL2
    /// Compute the eigenvalues and eigenvectors of symmetric tridiagonal matrix.
    ///
    /// Original SLATEC routine: `TQL2`; source: <https://www.netlib.org/slatec/lin/tql2.f>. Native symbol: `tql2_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tql2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues and eigenvectors of symmetric tridiagonal matrix.
        ///
        /// Original SLATEC routine: `TQL2`; source: <https://www.netlib.org/slatec/lin/tql2.f>. Native symbol: `tql2_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tql2_"]
        pub fn tql2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            d: *mut f32,
            e: *mut f32,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: TQLRAT
    /// Compute the eigenvalues of symmetric tridiagonal matrix using a rational variant of the QL method.
    ///
    /// Original SLATEC routine: `TQLRAT`; source: <https://www.netlib.org/slatec/lin/tqlrat.f>. Native symbol: `tqlrat_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tqlrat;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of symmetric tridiagonal matrix using a rational variant of the QL method.
        ///
        /// Original SLATEC routine: `TQLRAT`; source: <https://www.netlib.org/slatec/lin/tqlrat.f>. Native symbol: `tqlrat_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tqlrat_"]
        pub fn tqlrat(
            n: *mut crate::FortranInteger,
            d: *mut f32,
            e2: *mut f32,
            ierr: *mut crate::FortranInteger,
        );
    }

    // raw-api-routine: TRBAK1
    /// Form the eigenvectors of real symmetric matrix from the eigenvectors of a symmetric tridiagonal matrix formed by TRED1.
    ///
    /// Original SLATEC routine: `TRBAK1`; source: <https://www.netlib.org/slatec/lin/trbak1.f>. Native symbol: `trbak1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::trbak1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of real symmetric matrix from the eigenvectors of a symmetric tridiagonal matrix formed by TRED1.
        ///
        /// Original SLATEC routine: `TRBAK1`; source: <https://www.netlib.org/slatec/lin/trbak1.f>. Native symbol: `trbak1_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "trbak1_"]
        pub fn trbak1(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            e: *mut f32,
            m: *mut crate::FortranInteger,
            z: *mut f32,
        );
    }

    // raw-api-routine: TRBAK3
    /// Form the eigenvectors of a real symmetric matrix from the eigenvectors of a symmetric tridiagonal matrix formed by TRED3.
    ///
    /// Original SLATEC routine: `TRBAK3`; source: <https://www.netlib.org/slatec/lin/trbak3.f>. Native symbol: `trbak3_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::trbak3;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Form the eigenvectors of a real symmetric matrix from the eigenvectors of a symmetric tridiagonal matrix formed by TRED3.
        ///
        /// Original SLATEC routine: `TRBAK3`; source: <https://www.netlib.org/slatec/lin/trbak3.f>. Native symbol: `trbak3_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "trbak3_"]
        pub fn trbak3(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            nv: *mut crate::FortranInteger,
            a: *mut f32,
            m: *mut crate::FortranInteger,
            z: *mut f32,
        );
    }

    // raw-api-routine: TRED1
    /// Reduce a real symmetric matrix to symmetric tridiagonal matrix using orthogonal similarity transformations.
    ///
    /// Original SLATEC routine: `TRED1`; source: <https://www.netlib.org/slatec/lin/tred1.f>. Native symbol: `tred1_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tred1;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a real symmetric matrix to symmetric tridiagonal matrix using orthogonal similarity transformations.
        ///
        /// Original SLATEC routine: `TRED1`; source: <https://www.netlib.org/slatec/lin/tred1.f>. Native symbol: `tred1_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tred1_"]
        pub fn tred1(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
        );
    }

    // raw-api-routine: TRED2
    /// Reduce a real symmetric matrix to a symmetric tridiagonal matrix using and accumulating orthogonal transformations.
    ///
    /// Original SLATEC routine: `TRED2`; source: <https://www.netlib.org/slatec/lin/tred2.f>. Native symbol: `tred2_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tred2;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a real symmetric matrix to a symmetric tridiagonal matrix using and accumulating orthogonal transformations.
        ///
        /// Original SLATEC routine: `TRED2`; source: <https://www.netlib.org/slatec/lin/tred2.f>. Native symbol: `tred2_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tred2_"]
        pub fn tred2(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            a: *mut f32,
            d: *mut f32,
            e: *mut f32,
            z: *mut f32,
        );
    }

    // raw-api-routine: TRED3
    /// Reduce a real symmetric matrix stored in packed form to symmetric tridiagonal matrix using orthogonal transformations.
    ///
    /// Original SLATEC routine: `TRED3`; source: <https://www.netlib.org/slatec/lin/tred3.f>. Native symbol: `tred3_`.
    /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tred3;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Reduce a real symmetric matrix stored in packed form to symmetric tridiagonal matrix using orthogonal transformations.
        ///
        /// Original SLATEC routine: `TRED3`; source: <https://www.netlib.org/slatec/lin/tred3.f>. Native symbol: `tred3_`.
        /// Batch A ABI class: `numerical_array_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `NV`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tred3_"]
        pub fn tred3(
            n: *mut crate::FortranInteger,
            nv: *mut crate::FortranInteger,
            a: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
        );
    }

    // raw-api-routine: TRIDIB
    /// Compute the eigenvalues of a symmetric tridiagonal matrix in a given interval using Sturm sequencing.
    ///
    /// Original SLATEC routine: `TRIDIB`; source: <https://www.netlib.org/slatec/lin/tridib.f>. Native symbol: `tridib_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `UB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M11`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV4`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV5`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tridib;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Compute the eigenvalues of a symmetric tridiagonal matrix in a given interval using Sturm sequencing.
        ///
        /// Original SLATEC routine: `TRIDIB`; source: <https://www.netlib.org/slatec/lin/tridib.f>. Native symbol: `tridib_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `UB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M11`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IND`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV4`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV5`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tridib_"]
        pub fn tridib(
            n: *mut crate::FortranInteger,
            eps1: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            lb: *mut f32,
            ub: *mut f32,
            m11: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            w: *mut f32,
            ind: *mut crate::FortranInteger,
            ierr: *mut crate::FortranInteger,
            rv4: *mut f32,
            rv5: *mut f32,
        );
    }

    // raw-api-routine: TSTURM
    /// Find those eigenvalues of a symmetric tridiagonal matrix in a given interval and their associated eigenvectors by Sturm sequencing.
    ///
    /// Original SLATEC routine: `TSTURM`; source: <https://www.netlib.org/slatec/lin/tsturm.f>. Native symbol: `tsturm_`.
    /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
    ///
    /// # Arguments
    ///
    /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `LB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `UB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `MM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV3`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV4`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV5`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    /// - `RV6`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
    ///
    /// # Safety
    ///
    /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
    #[cfg(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical"))]
    pub use crate::generated::numeric_array_subroutines::tsturm;

    #[cfg(not(any(feature = "raw-ffi-basic", feature = "raw-ffi-logical")))]
    unsafe extern "C" {
        /// Find those eigenvalues of a symmetric tridiagonal matrix in a given interval and their associated eigenvectors by Sturm sequencing.
        ///
        /// Original SLATEC routine: `TSTURM`; source: <https://www.netlib.org/slatec/lin/tsturm.f>. Native symbol: `tsturm_`.
        /// Batch A ABI class: `workspace_or_matrix_numerical_subroutine`; normalized fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.
        ///
        /// # Arguments
        ///
        /// - `NM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `EPS1`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `D`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `E2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `UB`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `MM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `M`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `W`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Z`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV3`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV4`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV5`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RV6`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified, compiler-profile ABI declaration, not a safe or numerically validated API. Callers must provide valid non-null scalar and array pointers, satisfy every source-declared dimension, leading-dimension, and workspace rule, avoid mutable aliasing, use the supported GNU MinGW Fortran ABI, and serialize access if the native routine reaches legacy global state. The normalized declaration does not establish argument intent or pointer retention; consult the original source prologue before calling.
        #[link_name = "tsturm_"]
        pub fn tsturm(
            nm: *mut crate::FortranInteger,
            n: *mut crate::FortranInteger,
            eps1: *mut f32,
            d: *mut f32,
            e: *mut f32,
            e2: *mut f32,
            lb: *mut f32,
            ub: *mut f32,
            mm: *mut crate::FortranInteger,
            m: *mut crate::FortranInteger,
            w: *mut f32,
            z: *mut f32,
            ierr: *mut crate::FortranInteger,
            rv1: *mut f32,
            rv2: *mut f32,
            rv3: *mut f32,
            rv4: *mut f32,
            rv5: *mut f32,
            rv6: *mut f32,
        );
    }
}
