//! Generated Batch B callback-bearing canonical raw declarations.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-b --offline`.

/// Batch B canonical callback-bearing `ode` declarations.
pub mod callbacks {
    /// GNU Fortran callback `DF` for `DDEABM`.
    pub type DDEABMDF =
        unsafe extern "C" fn(*mut f64, *mut f64, *mut f64, *mut f64, *mut crate::FortranInteger);

    // raw-api-routine: DDEABM
    unsafe extern "C" {
        /// Solve an initial value problem in ordinary differential equations using an Adams-Bashforth method.
        ///
        /// Original SLATEC routine: `DDEABM`; source: <https://www.netlib.org/slatec/src/ddeabm.f>. Native symbol: `ddeabm_`.
        /// Batch B callback-bearing ABI fingerprint: `subroutine:void(sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32),mut_i32,mut_f64,mut_f64_ptr_rank1,mut_f64,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
        /// Callback `DF` uses `DDEABMDF` with fingerprint `sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`; evidence: `DDEABM -> DDES:DF(A,Y,YP,RPAR,IPAR); DDEABM -> DDES:DF(TOUT,Y,YPOUT,RPAR,IPAR); DDEABM -> DDES -> DSTEPS:DF(X,P,YP,RPAR,IPAR); DDEABM -> DDES -> DSTEPS:DF(X,Y,YP,RPAR,IPAR); DDEABM -> DDES -> DSTEPS -> DHSTRT:DF(A+DA,Y,SF,RPAR,IPAR); DDEABM -> DDES -> DSTEPS -> DHSTRT:DF(A,PV,YP,RPAR,IPAR); DDEABM -> DDES -> DSTEPS -> DHSTRT:DF(A+DA,PV,YP,RPAR,IPAR)`.
        ///
        /// # Arguments
        ///
        /// - `DF`: non-null callback pointer with ABI `sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
        /// - `NEQ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `T`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TOUT`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INFO`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RTOL`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ATOL`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IDID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LRW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LIW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RPAR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IPAR`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
        #[link_name = "ddeabm_"]
        pub fn ddeabm(
            df: DDEABMDF,
            neq: *mut crate::FortranInteger,
            t: *mut f64,
            y: *mut f64,
            tout: *mut f64,
            info: *mut crate::FortranInteger,
            rtol: *mut f64,
            atol: *mut f64,
            idid: *mut crate::FortranInteger,
            rwork: *mut f64,
            lrw: *mut crate::FortranInteger,
            iwork: *mut crate::FortranInteger,
            liw: *mut crate::FortranInteger,
            rpar: *mut f64,
            ipar: *mut crate::FortranInteger,
        );
    }

    /// GNU Fortran callback `DF` for `DDERKF`.
    pub type DDERKFDF =
        unsafe extern "C" fn(*mut f64, *mut f64, *mut f64, *mut f64, *mut crate::FortranInteger);

    // raw-api-routine: DDERKF
    unsafe extern "C" {
        /// Solve an initial value problem in ordinary differential equations using a Runge-Kutta-Fehlberg scheme.
        ///
        /// Original SLATEC routine: `DDERKF`; source: <https://www.netlib.org/slatec/src/dderkf.f>. Native symbol: `dderkf_`.
        /// Batch B callback-bearing ABI fingerprint: `subroutine:void(sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32),mut_i32,mut_f64,mut_f64_ptr_rank1,mut_f64,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
        /// Callback `DF` uses `DDERKFDF` with fingerprint `sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`; evidence: `DDERKF -> DRKFS:DF(A,Y,YP,RPAR,IPAR); DDERKF -> DRKFS:DF(A,Y,YP,RPAR,IPAR); DDERKF -> DRKFS:DF(A,Y,YP,RPAR,IPAR); DDERKF -> DRKFS -> DHSTRT:DF(A+DA,Y,SF,RPAR,IPAR); DDERKF -> DRKFS -> DHSTRT:DF(A,PV,YP,RPAR,IPAR); DDERKF -> DRKFS -> DHSTRT:DF(A+DA,PV,YP,RPAR,IPAR); DDERKF -> DRKFS -> DFEHL:DF(T+CH,YS,F1,RPAR,IPAR); DDERKF -> DRKFS -> DFEHL:DF(T+3.0D0*H/8.0D0,YS,F2,RPAR,IPAR); DDERKF -> DRKFS -> DFEHL:DF(T+12.0D0*H/13.0D0,YS,F3,RPAR,IPAR); DDERKF -> DRKFS -> DFEHL:DF(T+H,YS,F4,RPAR,IPAR); DDERKF -> DRKFS -> DFEHL:DF(T+H/2.0D0,YS,F5,RPAR,IPAR)`.
        ///
        /// # Arguments
        ///
        /// - `DF`: non-null callback pointer with ABI `sub:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
        /// - `NEQ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `T`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TOUT`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INFO`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RTOL`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ATOL`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IDID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LRW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LIW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RPAR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IPAR`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
        #[link_name = "dderkf_"]
        pub fn dderkf(
            df: DDERKFDF,
            neq: *mut crate::FortranInteger,
            t: *mut f64,
            y: *mut f64,
            tout: *mut f64,
            info: *mut crate::FortranInteger,
            rtol: *mut f64,
            atol: *mut f64,
            idid: *mut crate::FortranInteger,
            rwork: *mut f64,
            lrw: *mut crate::FortranInteger,
            iwork: *mut crate::FortranInteger,
            liw: *mut crate::FortranInteger,
            rpar: *mut f64,
            ipar: *mut crate::FortranInteger,
        );
    }

    /// GNU Fortran callback `F` for `DERKF`.
    pub type DERKFF =
        unsafe extern "C" fn(*mut f32, *mut f32, *mut f32, *mut f32, *mut crate::FortranInteger);

    // raw-api-routine: DERKF
    unsafe extern "C" {
        /// Solve an initial value problem in ordinary differential equations using a Runge-Kutta-Fehlberg scheme.
        ///
        /// Original SLATEC routine: `DERKF`; source: <https://www.netlib.org/slatec/src/derkf.f>. Native symbol: `derkf_`.
        /// Batch B callback-bearing ABI fingerprint: `subroutine:void(sub:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_i32),mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
        /// Callback `F` uses `DERKFF` with fingerprint `sub:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`; evidence: `DERKF -> DERKFS:F(A,Y,YP,RPAR,IPAR); DERKF -> DERKFS:F(A,Y,YP,RPAR,IPAR); DERKF -> DERKFS:F(A,Y,YP,RPAR,IPAR); DERKF -> DERKFS -> HSTART:F(A+DA,Y,SF,RPAR,IPAR); DERKF -> DERKFS -> HSTART:F(A,PV,YP,RPAR,IPAR); DERKF -> DERKFS -> HSTART:F(A+DA,PV,YP,RPAR,IPAR); DERKF -> DERKFS -> DEFEHL:F(T+CH,YS,F1,RPAR,IPAR); DERKF -> DERKFS -> DEFEHL:F(T+3.*H/8.,YS,F2,RPAR,IPAR); DERKF -> DERKFS -> DEFEHL:F(T+12.*H/13.,YS,F3,RPAR,IPAR); DERKF -> DERKFS -> DEFEHL:F(T+H,YS,F4,RPAR,IPAR); DERKF -> DERKFS -> DEFEHL:F(T+H/2.,YS,F5,RPAR,IPAR)`.
        ///
        /// # Arguments
        ///
        /// - `F`: non-null callback pointer with ABI `sub:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
        /// - `NEQ`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `T`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `Y`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `TOUT`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `INFO`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RTOL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `ATOL`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IDID`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RWORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LRW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `LIW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `RPAR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        /// - `IPAR`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
        ///
        /// # Safety
        ///
        /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
        #[link_name = "derkf_"]
        pub fn derkf(
            f: DERKFF,
            neq: *mut crate::FortranInteger,
            t: *mut f32,
            y: *mut f32,
            tout: *mut f32,
            info: *mut crate::FortranInteger,
            rtol: *mut f32,
            atol: *mut f32,
            idid: *mut crate::FortranInteger,
            rwork: *mut f32,
            lrw: *mut crate::FortranInteger,
            iwork: *mut crate::FortranInteger,
            liw: *mut crate::FortranInteger,
            rpar: *mut f32,
            ipar: *mut crate::FortranInteger,
        );
    }
}
