# DGLSS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linear least squares problems by performing a QR factorization of the input matrix using Householder transformations. Emphasis is put on detecting possible rank deficiency.

## Description

DGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. If M.GE.N, the least squares solution is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factorization). If the matrix A is determined to be rank deficient, that is the rank of A is less than MIN(M,N), then the minimal length least squares solution is computed. DGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes DLLSIA and DULSIA are recommended. DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. ****************************************************************** * * * WARNING - All input arrays are changed on exit. * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. M is the row dimension (no. of EQUATIONS of the problem) and N the col dimension (no. of UNKNOWNS). B(,) Right hand side(s), with MDB the actual first MDB,NB dimension of B in the calling program. NB is the number of M by 1 right hand sides. Must have MDB.GE.MAX(M,N). If NB = 0, B is never accessed. RNORM() Vector of length at least NB. On input the contents of RNORM are unused. WORK() A real work array dimensioned 5*MIN(M,N). LW Actual dimension of WORK. IWORK() Integer work array dimensioned at least N+M. LIW Actual dimension of IWORK. INFO A flag which provides for the efficient solution of subsequent problems involving the same A but different B. If INFO = 0 original call INFO = 1 subsequent calls On subsequent calls, the user must supply A, INFO, LW, IWORK, LIW, and the first 2*MIN(M,N) locations of WORK as output by the original call to DGLSS. Output..All TYPE REAL variables are DOUBLE PRECISION A(,) Contains the triangular part of the reduced matrix and the transformation information. It together with the first 2*MIN(M,N) elements of WORK (see below) completely specify the factorization of A. B(,) Contains the N by NB solution matrix X. RNORM() Contains the Euclidean length of the NB residual vectors B(I)-AX(I), I=1,NB. WORK() The first 2*MIN(M,N) locations of WORK contain value necessary to reproduce the factorization of A. IWORK() The first M+N locations contain the order in which the rows and columns of A were used. If M.GE.N columns then rows. If M.LT.N rows then columns. INFO Flag to indicate status of computation on completion -1 Parameter error(s) 0 - Full rank N.GT.0 - Reduced rank rank=MIN(M,N)-INFO

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D9`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dglss.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dglss.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dglss.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dglss.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `A` | input | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 2; dimensions (MDA, *) | DGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDA` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. | DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 2; dimensions (MDB, *) | DGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDB` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. | DGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NB` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORM` | input | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 1; dimensions (*) | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | input | `DOUBLE PRECISION` (`implicit_rule`) | `*mut f64` | rank 1; dimensions (*) | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LW` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIW` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. | * * * ****************************************************************** SUBROUTINE DGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) Input..All TYPE REAL variables are DOUBLE PRECISION A(,) Linear coefficient matrix of AX=B, with MDA the MDA,M,N actual first dimension of A in the calling program. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::dglss`. Native symbol: `dglss_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f64_ptr_rank2,mut_i32,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::dglss`
- Compatibility aliases: `none`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
