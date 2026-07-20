//! Generated Batch B callback-bearing canonical raw declarations.
//!
//! Do not edit. Regenerate with `slatec-corpus generate-raw-batch-b --offline`.

/// Batch B canonical callback-bearing `linear_algebra` declarations.
pub mod sparse {
    pub mod callbacks {
        /// GNU Fortran callback `MATVEC` for `DBCG`.
        pub type DBCGMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `DBCG`.
        pub type DBCGMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MTSOLV` for `DBCG`.
        pub type DBCGMTSOLV = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MTTVEC` for `DBCG`.
        pub type DBCGMTTVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: DBCG
        unsafe extern "C" {
            /// Preconditioned BiConjugate Gradient Sparse Ax = b Solver. Routine to solve a Non-Symmetric linear system Ax = b using the Preconditioned BiConjugate Gradient method.
            ///
            /// Original SLATEC routine: `DBCG`; source: <https://www.netlib.org/slatec/lin/dbcg.f>. Native symbol: `dbcg_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `DBCGMATVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DBCG:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); DBCG:MATVEC(N,P,Z,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `DBCGMSOLVE` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`; evidence: `DBCG:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); DBCG:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            /// Callback `MTSOLV` uses `DBCGMTSOLV` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`; evidence: `DBCG:MTSOLV(N,RR,ZZ,NELT,IA,JA,A,ISYM,RWORK,IWORK); DBCG:MTSOLV(N,RR,ZZ,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            /// Callback `MTTVEC` uses `DBCGMTTVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DBCG:MTTVEC(N,PP,ZZ,NELT,IA,JA,A,ISYM)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MTTVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MTSOLV`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RR`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ZZ`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `PP`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "dbcg_"]
            pub fn dbcg(
                n: *mut crate::FortranInteger,
                b: *mut f64,
                x: *mut f64,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f64,
                isym: *mut crate::FortranInteger,
                matvec: DBCGMATVEC,
                mttvec: DBCGMTTVEC,
                msolve: DBCGMSOLVE,
                mtsolv: DBCGMTSOLV,
                itol: *mut crate::FortranInteger,
                tol: *mut f64,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f64,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f64,
                z: *mut f64,
                p: *mut f64,
                rr: *mut f64,
                zz: *mut f64,
                pp: *mut f64,
                dz: *mut f64,
                rwork: *mut f64,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `DCG`.
        pub type DCGMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `DCG`.
        pub type DCGMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: DCG
        unsafe extern "C" {
            /// Preconditioned Conjugate Gradient Sparse Ax=b Solver. Routine to solve a symmetric positive definite linear system Ax = b using the Preconditioned Conjugate Gradient method.
            ///
            /// Original SLATEC routine: `DCG`; source: <https://www.netlib.org/slatec/lin/dcg.f>. Native symbol: `dcg_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `DCGMATVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DCG:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); DCG:MATVEC(N,P,Z,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `DCGMSOLVE` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`; evidence: `DCG:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); DCG:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "dcg_"]
            pub fn dcg(
                n: *mut crate::FortranInteger,
                b: *mut f64,
                x: *mut f64,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f64,
                isym: *mut crate::FortranInteger,
                matvec: DCGMATVEC,
                msolve: DCGMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f64,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f64,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f64,
                z: *mut f64,
                p: *mut f64,
                dz: *mut f64,
                rwork: *mut f64,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `DCGN`.
        pub type DCGNMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `DCGN`.
        pub type DCGNMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MTTVEC` for `DCGN`.
        pub type DCGNMTTVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: DCGN
        unsafe extern "C" {
            /// Preconditioned CG Sparse Ax=b Solver for Normal Equations. Routine to solve a general linear system Ax = b using the Preconditioned Conjugate Gradient method applied to the normal equations AA'y = b, x=A'y.
            ///
            /// Original SLATEC routine: `DCGN`; source: <https://www.netlib.org/slatec/lin/dcgn.f>. Native symbol: `dcgn_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `DCGNMATVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DCGN:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); DCGN:MATVEC(N,ATP,Z,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `DCGNMSOLVE` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`; evidence: `DCGN:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); DCGN:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            /// Callback `MTTVEC` uses `DCGNMTTVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DCGN:MTTVEC(N,Z,ATZ,NELT,IA,JA,A,ISYM); DCGN:MTTVEC(N,Z,ATZ,NELT,IA,JA,A,ISYM)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MTTVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ATP`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ATZ`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ATDZ`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "dcgn_"]
            pub fn dcgn(
                n: *mut crate::FortranInteger,
                b: *mut f64,
                x: *mut f64,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f64,
                isym: *mut crate::FortranInteger,
                matvec: DCGNMATVEC,
                mttvec: DCGNMTTVEC,
                msolve: DCGNMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f64,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f64,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f64,
                z: *mut f64,
                p: *mut f64,
                atp: *mut f64,
                atz: *mut f64,
                dz: *mut f64,
                atdz: *mut f64,
                rwork: *mut f64,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `DCGS`.
        pub type DCGSMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `DCGS`.
        pub type DCGSMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: DCGS
        unsafe extern "C" {
            /// Preconditioned BiConjugate Gradient Squared Ax=b Solver. Routine to solve a Non-Symmetric linear system Ax = b using the Preconditioned BiConjugate Gradient Squared method.
            ///
            /// Original SLATEC routine: `DCGS`; source: <https://www.netlib.org/slatec/lin/dcgs.f>. Native symbol: `dcgs_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `DCGSMATVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DCGS:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); DCGS:MATVEC(N,P,V2,NELT,IA,JA,A,ISYM); DCGS:MATVEC(N,V1,V2,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `DCGSMSOLVE` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`; evidence: `DCGS:MSOLVE(N,V1,R,NELT,IA,JA,A,ISYM,RWORK,IWORK); DCGS:MSOLVE(N,V2,V1,NELT,IA,JA,A,ISYM,RWORK,IWORK); DCGS:MSOLVE(N,V2,V1,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R0`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Q`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `U`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `V1`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `V2`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "dcgs_"]
            pub fn dcgs(
                n: *mut crate::FortranInteger,
                b: *mut f64,
                x: *mut f64,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f64,
                isym: *mut crate::FortranInteger,
                matvec: DCGSMATVEC,
                msolve: DCGSMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f64,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f64,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f64,
                r0: *mut f64,
                p: *mut f64,
                q: *mut f64,
                u: *mut f64,
                v1: *mut f64,
                v2: *mut f64,
                rwork: *mut f64,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `DGMRES`.
        pub type DGMRESMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `DGMRES`.
        pub type DGMRESMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: DGMRES
        unsafe extern "C" {
            /// Preconditioned GMRES iterative sparse Ax=b solver. This routine uses the generalized minimum residual (GMRES) method with preconditioning to solve non-symmetric linear systems of the form: Ax = b.
            ///
            /// Original SLATEC routine: `DGMRES`; source: <https://www.netlib.org/slatec/lin/dgmres.f>. Native symbol: `dgmres_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `DGMRESMATVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DGMRES:MATVEC(N,X,RGWK(LR),NELT,IA,JA,A,ISYM); DGMRES -> DPIGMR:MATVEC(N,Z,V(1,LL+1),NELT,IA,JA,A,ISYM); DGMRES -> DPIGMR:MATVEC(N,WK,V(1,LL+1),NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `DGMRESMSOLVE` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`; evidence: `DGMRES:MSOLVE(N,B,RGWK(LR),NELT,IA,JA,A,ISYM,RWORK,IWORK); DGMRES -> DPIGMR:MSOLVE(N,WK,R0,NELT,IA,JA,A,ISYM,RPAR,IPAR); DGMRES -> DPIGMR:MSOLVE(N,WK,Z,NELT,IA,JA,A,ISYM,RPAR,IPAR); DGMRES -> DPIGMR:MSOLVE(N,WK,V(1,LL+1),NELT,IA,JA,A,ISYM,RPAR,IPAR); DGMRES -> DPIGMR:MSOLVE(N,WK,Z,NELT,IA,JA,A,ISYM,RPAR,IPAR)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `SB`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `SX`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RGWK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `LRGW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IGWK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `LIGW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "dgmres_"]
            pub fn dgmres(
                n: *mut crate::FortranInteger,
                b: *mut f64,
                x: *mut f64,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f64,
                isym: *mut crate::FortranInteger,
                matvec: DGMRESMATVEC,
                msolve: DGMRESMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f64,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f64,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                sb: *mut f64,
                sx: *mut f64,
                rgwk: *mut f64,
                lrgw: *mut crate::FortranInteger,
                igwk: *mut crate::FortranInteger,
                ligw: *mut crate::FortranInteger,
                rwork: *mut f64,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `DIR`.
        pub type DIRMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `DIR`.
        pub type DIRMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: DIR
        unsafe extern "C" {
            /// Preconditioned Iterative Refinement Sparse Ax = b Solver. Routine to solve a general linear system Ax = b using iterative refinement with a matrix splitting.
            ///
            /// Original SLATEC routine: `DIR`; source: <https://www.netlib.org/slatec/lin/dir.f>. Native symbol: `dir_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `DIRMATVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DIR:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); DIR:MATVEC(N,X,R,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `DIRMSOLVE` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`; evidence: `DIR:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); DIR:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "dir_"]
            pub fn dir(
                n: *mut crate::FortranInteger,
                b: *mut f64,
                x: *mut f64,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f64,
                isym: *mut crate::FortranInteger,
                matvec: DIRMATVEC,
                msolve: DIRMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f64,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f64,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f64,
                z: *mut f64,
                dz: *mut f64,
                rwork: *mut f64,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `DOMN`.
        pub type DOMNMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `DOMN`.
        pub type DOMNMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f64,
            *mut f64,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
            *mut f64,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: DOMN
        unsafe extern "C" {
            /// Preconditioned Orthomin Sparse Iterative Ax=b Solver. Routine to solve a general linear system Ax = b using the Preconditioned Orthomin method.
            ///
            /// Original SLATEC routine: `DOMN`; source: <https://www.netlib.org/slatec/lin/domn.f>. Native symbol: `domn_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f64_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32),sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32),mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `DOMNMATVEC` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`; evidence: `DOMN:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); DOMN:MATVEC(N,P(1,IP),AP(1,IP),NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `DOMNMSOLVE` with fingerprint `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`; evidence: `DOMN:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); DOMN:MSOLVE(N,AP(1,IP),EMAP(1,IP),NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f64,mut_f64,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `NSAVE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `DOUBLE PRECISION` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `DOUBLE PRECISION` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `AP`: declared `DOUBLE PRECISION` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `EMAP`: declared `DOUBLE PRECISION` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `CSAV`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `DOUBLE PRECISION` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "domn_"]
            pub fn domn(
                n: *mut crate::FortranInteger,
                b: *mut f64,
                x: *mut f64,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f64,
                isym: *mut crate::FortranInteger,
                matvec: DOMNMATVEC,
                msolve: DOMNMSOLVE,
                nsave: *mut crate::FortranInteger,
                itol: *mut crate::FortranInteger,
                tol: *mut f64,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f64,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f64,
                z: *mut f64,
                p: *mut f64,
                ap: *mut f64,
                emap: *mut f64,
                dz: *mut f64,
                csav: *mut f64,
                rwork: *mut f64,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `SBCG`.
        pub type SBCGMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `SBCG`.
        pub type SBCGMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MTSOLV` for `SBCG`.
        pub type SBCGMTSOLV = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MTTVEC` for `SBCG`.
        pub type SBCGMTTVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: SBCG
        unsafe extern "C" {
            /// Preconditioned BiConjugate Gradient Sparse Ax = b Solver. Routine to solve a Non-Symmetric linear system Ax = b using the Preconditioned BiConjugate Gradient method.
            ///
            /// Original SLATEC routine: `SBCG`; source: <https://www.netlib.org/slatec/lin/sbcg.f>. Native symbol: `sbcg_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `SBCGMATVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SBCG:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); SBCG:MATVEC(N,P,Z,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `SBCGMSOLVE` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`; evidence: `SBCG:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); SBCG:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            /// Callback `MTSOLV` uses `SBCGMTSOLV` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`; evidence: `SBCG:MTSOLV(N,RR,ZZ,NELT,IA,JA,A,ISYM,RWORK,IWORK); SBCG:MTSOLV(N,RR,ZZ,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            /// Callback `MTTVEC` uses `SBCGMTTVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SBCG:MTTVEC(N,PP,ZZ,NELT,IA,JA,A,ISYM)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MTTVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MTSOLV`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RR`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ZZ`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `PP`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "sbcg_"]
            pub fn sbcg(
                n: *mut crate::FortranInteger,
                b: *mut f32,
                x: *mut f32,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f32,
                isym: *mut crate::FortranInteger,
                matvec: SBCGMATVEC,
                mttvec: SBCGMTTVEC,
                msolve: SBCGMSOLVE,
                mtsolv: SBCGMTSOLV,
                itol: *mut crate::FortranInteger,
                tol: *mut f32,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f32,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f32,
                z: *mut f32,
                p: *mut f32,
                rr: *mut f32,
                zz: *mut f32,
                pp: *mut f32,
                dz: *mut f32,
                rwork: *mut f32,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `SCG`.
        pub type SCGMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `SCG`.
        pub type SCGMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: SCG
        unsafe extern "C" {
            /// Preconditioned Conjugate Gradient Sparse Ax=b Solver. Routine to solve a symmetric positive definite linear system Ax = b using the Preconditioned Conjugate Gradient method.
            ///
            /// Original SLATEC routine: `SCG`; source: <https://www.netlib.org/slatec/lin/scg.f>. Native symbol: `scg_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `SCGMATVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SCG:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); SCG:MATVEC(N,P,Z,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `SCGMSOLVE` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`; evidence: `SCG:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); SCG:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "scg_"]
            pub fn scg(
                n: *mut crate::FortranInteger,
                b: *mut f32,
                x: *mut f32,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f32,
                isym: *mut crate::FortranInteger,
                matvec: SCGMATVEC,
                msolve: SCGMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f32,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f32,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f32,
                z: *mut f32,
                p: *mut f32,
                dz: *mut f32,
                rwork: *mut f32,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `SCGN`.
        pub type SCGNMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `SCGN`.
        pub type SCGNMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MTTVEC` for `SCGN`.
        pub type SCGNMTTVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: SCGN
        unsafe extern "C" {
            /// Preconditioned CG Sparse Ax=b Solver for Normal Equations. Routine to solve a general linear system Ax = b using the Preconditioned Conjugate Gradient method applied to the normal equations AA'y = b, x=A'y.
            ///
            /// Original SLATEC routine: `SCGN`; source: <https://www.netlib.org/slatec/lin/scgn.f>. Native symbol: `scgn_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `SCGNMATVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SCGN:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); SCGN:MATVEC(N,ATP,Z,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `SCGNMSOLVE` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`; evidence: `SCGN:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); SCGN:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            /// Callback `MTTVEC` uses `SCGNMTTVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SCGN:MTTVEC(N,Z,ATZ,NELT,IA,JA,A,ISYM); SCGN:MTTVEC(N,Z,ATZ,NELT,IA,JA,A,ISYM)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MTTVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ATP`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ATZ`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ATDZ`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "scgn_"]
            pub fn scgn(
                n: *mut crate::FortranInteger,
                b: *mut f32,
                x: *mut f32,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f32,
                isym: *mut crate::FortranInteger,
                matvec: SCGNMATVEC,
                mttvec: SCGNMTTVEC,
                msolve: SCGNMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f32,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f32,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f32,
                z: *mut f32,
                p: *mut f32,
                atp: *mut f32,
                atz: *mut f32,
                dz: *mut f32,
                atdz: *mut f32,
                rwork: *mut f32,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `SCGS`.
        pub type SCGSMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `SCGS`.
        pub type SCGSMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: SCGS
        unsafe extern "C" {
            /// Preconditioned BiConjugate Gradient Squared Ax=b Solver. Routine to solve a Non-Symmetric linear system Ax = b using the Preconditioned BiConjugate Gradient Squared method.
            ///
            /// Original SLATEC routine: `SCGS`; source: <https://www.netlib.org/slatec/lin/scgs.f>. Native symbol: `scgs_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `SCGSMATVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SCGS:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); SCGS:MATVEC(N,P,V2,NELT,IA,JA,A,ISYM); SCGS:MATVEC(N,V1,V2,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `SCGSMSOLVE` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`; evidence: `SCGS:MSOLVE(N,V1,R,NELT,IA,JA,A,ISYM,RWORK,IWORK); SCGS:MSOLVE(N,V2,V1,NELT,IA,JA,A,ISYM,RWORK,IWORK); SCGS:MSOLVE(N,V2,V1,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R0`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Q`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `U`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `V1`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `V2`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "scgs_"]
            pub fn scgs(
                n: *mut crate::FortranInteger,
                b: *mut f32,
                x: *mut f32,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f32,
                isym: *mut crate::FortranInteger,
                matvec: SCGSMATVEC,
                msolve: SCGSMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f32,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f32,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f32,
                r0: *mut f32,
                p: *mut f32,
                q: *mut f32,
                u: *mut f32,
                v1: *mut f32,
                v2: *mut f32,
                rwork: *mut f32,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `SGMRES`.
        pub type SGMRESMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `SGMRES`.
        pub type SGMRESMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: SGMRES
        unsafe extern "C" {
            /// Preconditioned GMRES Iterative Sparse Ax=b Solver. This routine uses the generalized minimum residual (GMRES) method with preconditioning to solve non-symmetric linear systems of the form: Ax = b.
            ///
            /// Original SLATEC routine: `SGMRES`; source: <https://www.netlib.org/slatec/lin/sgmres.f>. Native symbol: `sgmres_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `SGMRESMATVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SGMRES:MATVEC(N,X,RGWK(LR),NELT,IA,JA,A,ISYM); SGMRES -> SPIGMR:MATVEC(N,Z,V(1,LL+1),NELT,IA,JA,A,ISYM); SGMRES -> SPIGMR:MATVEC(N,WK,V(1,LL+1),NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `SGMRESMSOLVE` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`; evidence: `SGMRES:MSOLVE(N,B,RGWK(LR),NELT,IA,JA,A,ISYM,RWORK,IWORK); SGMRES -> SPIGMR:MSOLVE(N,WK,R0,NELT,IA,JA,A,ISYM,RPAR,IPAR); SGMRES -> SPIGMR:MSOLVE(N,WK,Z,NELT,IA,JA,A,ISYM,RPAR,IPAR); SGMRES -> SPIGMR:MSOLVE(N,WK,V(1,LL+1),NELT,IA,JA,A,ISYM,RPAR,IPAR); SGMRES -> SPIGMR:MSOLVE(N,WK,Z,NELT,IA,JA,A,ISYM,RPAR,IPAR)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `SB`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `SX`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RGWK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `LRGW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IGWK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `LIGW`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "sgmres_"]
            pub fn sgmres(
                n: *mut crate::FortranInteger,
                b: *mut f32,
                x: *mut f32,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f32,
                isym: *mut crate::FortranInteger,
                matvec: SGMRESMATVEC,
                msolve: SGMRESMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f32,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f32,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                sb: *mut f32,
                sx: *mut f32,
                rgwk: *mut f32,
                lrgw: *mut crate::FortranInteger,
                igwk: *mut crate::FortranInteger,
                ligw: *mut crate::FortranInteger,
                rwork: *mut f32,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `SIR`.
        pub type SIRMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `SIR`.
        pub type SIRMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: SIR
        unsafe extern "C" {
            /// Preconditioned Iterative Refinement Sparse Ax = b Solver. Routine to solve a general linear system Ax = b using iterative refinement with a matrix splitting.
            ///
            /// Original SLATEC routine: `SIR`; source: <https://www.netlib.org/slatec/lin/sir.f>. Native symbol: `sir_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `SIRMATVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SIR:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); SIR:MATVEC(N,X,R,NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `SIRMSOLVE` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`; evidence: `SIR:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); SIR:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "sir_"]
            pub fn sir(
                n: *mut crate::FortranInteger,
                b: *mut f32,
                x: *mut f32,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f32,
                isym: *mut crate::FortranInteger,
                matvec: SIRMATVEC,
                msolve: SIRMSOLVE,
                itol: *mut crate::FortranInteger,
                tol: *mut f32,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f32,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f32,
                z: *mut f32,
                dz: *mut f32,
                rwork: *mut f32,
                iwork: *mut crate::FortranInteger,
            );
        }

        /// GNU Fortran callback `MATVEC` for `SOMN`.
        pub type SOMNMATVEC = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        /// GNU Fortran callback `MSOLVE` for `SOMN`.
        pub type SOMNMSOLVE = unsafe extern "C" fn(
            *mut crate::FortranInteger,
            *mut f32,
            *mut f32,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
            *mut f32,
            *mut crate::FortranInteger,
        );

        // raw-api-routine: SOMN
        unsafe extern "C" {
            /// Preconditioned Orthomin Sparse Iterative Ax=b Solver. Routine to solve a general linear system Ax = b using the Preconditioned Orthomin method.
            ///
            /// Original SLATEC routine: `SOMN`; source: <https://www.netlib.org/slatec/lin/somn.f>. Native symbol: `somn_`.
            /// Batch B callback-bearing ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32),sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32),mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.
            /// Callback `MATVEC` uses `SOMNMATVEC` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`; evidence: `SOMN:MATVEC(N,X,R,NELT,IA,JA,A,ISYM); SOMN:MATVEC(N,P(1,IP),AP(1,IP),NELT,IA,JA,A,ISYM)`.
            /// Callback `MSOLVE` uses `SOMNMSOLVE` with fingerprint `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`; evidence: `SOMN:MSOLVE(N,R,Z,NELT,IA,JA,A,ISYM,RWORK,IWORK); SOMN:MSOLVE(N,AP(1,IP),EMAP(1,IP),NELT,IA,JA,A,ISYM,RWORK,IWORK)`.
            ///
            /// # Arguments
            ///
            /// - `N`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `B`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `X`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `NELT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `JA`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `A`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ISYM`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `MATVEC`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `MSOLVE`: non-null callback pointer with ABI `sub:void(mut_i32,mut_f32,mut_f32,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32,mut_i32)`. It is invoked synchronously by SLATEC or a source-hash-verified subsidiary and is not retained after the outer native call returns.
            /// - `NSAVE`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITOL`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `TOL`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITMAX`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ITER`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `ERR`: declared `REAL` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IERR`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IUNIT`: declared `INTEGER` scalar. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `R`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `Z`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `P`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `AP`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `EMAP`: declared `REAL` array of rank 2. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `DZ`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `CSAV`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `RWORK`: declared `REAL` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            /// - `IWORK`: declared `INTEGER` array of rank 1. Intent, exact extent expressions, aliasing permission, and pointer retention are unavailable from the normalized declaration; it must be non-null, aligned, and valid for every native access.
            ///
            /// # Safety
            ///
            /// This is a source-verified callback-bearing raw declaration, not a safe or numerically validated API. Callback functions must use the exact GNU Fortran calling convention, remain callable for the full native invocation, write only through documented mutable outputs, and must not unwind into Fortran. Scalar and array pointers must be non-null, aligned, valid for the native access, and must not violate Rust aliasing rules. No user-data pointer or closure trampoline is invented by this raw API; callers must arrange any state through the original Fortran arguments and serialize legacy runtime access where required.
            #[link_name = "somn_"]
            pub fn somn(
                n: *mut crate::FortranInteger,
                b: *mut f32,
                x: *mut f32,
                nelt: *mut crate::FortranInteger,
                ia: *mut crate::FortranInteger,
                ja: *mut crate::FortranInteger,
                a: *mut f32,
                isym: *mut crate::FortranInteger,
                matvec: SOMNMATVEC,
                msolve: SOMNMSOLVE,
                nsave: *mut crate::FortranInteger,
                itol: *mut crate::FortranInteger,
                tol: *mut f32,
                itmax: *mut crate::FortranInteger,
                iter: *mut crate::FortranInteger,
                err: *mut f32,
                ierr: *mut crate::FortranInteger,
                iunit: *mut crate::FortranInteger,
                r: *mut f32,
                z: *mut f32,
                p: *mut f32,
                ap: *mut f32,
                emap: *mut f32,
                dz: *mut f32,
                csav: *mut f32,
                rwork: *mut f32,
                iwork: *mut crate::FortranInteger,
            );
        }
    }
}
