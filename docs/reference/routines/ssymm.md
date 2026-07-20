# SSYMM

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Multiply a real general matrix by a real symmetric matrix.

## Description

SSYMM performs one of the matrix-matrix operations C := alpha*A*B + beta*C, or C := alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and C are m by n matrices. Parameters SIDE - CHARACTER*1. On entry, SIDE specifies whether the symmetric matrix A appears on the left or right in the operation as follows: SIDE = 'L' or 'l' C := alpha*A*B + beta*C, SIDE = 'R' or 'r' C := alpha*B*A + beta*C, Unchanged on exit. UPLO - CHARACTER*1. On entry, UPLO specifies whether the upper or lower triangular part of the symmetric matrix A is to be referenced as follows: UPLO = 'U' or 'u' Only the upper triangular part of the symmetric matrix is to be referenced. UPLO = 'L' or 'l' Only the lower triangular part of the symmetric matrix is to be referenced. Unchanged on exit. M - INTEGER. On entry, M specifies the number of rows of the matrix C. M must be at least zero. Unchanged on exit. N - INTEGER. On entry, N specifies the number of columns of the matrix C. N must be at least zero. Unchanged on exit. ALPHA - REAL . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. A - REAL array of DIMENSION ( LDA, ka ), where ka is m when SIDE = 'L' or 'l' and is n otherwise. Before entry with SIDE = 'L' or 'l', the m by m part of the array A must contain the symmetric matrix, such that when UPLO = 'U' or 'u', the leading m by m upper triangular part of the array A must contain the upper triangular part of the symmetric matrix and the strictly lower triangular part of A is not referenced, and when UPLO = 'L' or 'l', the leading m by m lower triangular part of the array A must contain the lower triangular part of the symmetric matrix and the strictly upper triangular part of A is not referenced. Before entry with SIDE = 'R' or 'r', the n by n part of the array A must contain the symmetric matrix, such that when UPLO = 'U' or 'u', the leading n by n upper triangular part of the array A must contain the upper triangular part of the symmetric matrix and the strictly lower triangular part of A is not referenced, and when UPLO = 'L' or 'l', the leading n by n lower triangular part of the array A must contain the lower triangular part of the symmetric matrix and the strictly upper triangular part of A is not referenced. Unchanged on exit. LDA - INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When SIDE = 'L' or 'l' then LDA must be at least max( 1, m ), otherwise LDA must be at least max( 1, n ). Unchanged on exit. B - REAL array of DIMENSION ( LDB, n ). Before entry, the leading m by n part of the array B must contain the matrix B. Unchanged on exit. LDB - INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. LDB must be at least max( 1, m ). Unchanged on exit. BETA - REAL . On entry, BETA specifies the scalar beta. When BETA is supplied as zero then C need not be set on input. Unchanged on exit. C - REAL array of DIMENSION ( LDC, n ). Before entry, the leading m by n part of the array C must contain the matrix C, except when beta is zero, in which case C need not be set on entry. On exit, the array C is overwritten by the m by n updated matrix. LDC - INTEGER. On entry, LDC specifies the first dimension of C as declared in the calling (sub) program. LDC must be at least max( 1, m ). Unchanged on exit.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1B6`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/ssymm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ssymm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ssymm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Linear algebra kernels](../families/linear-algebra-kernels.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `SIDE` | input | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | Parameters SIDE - CHARACTER*1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `UPLO` | input | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | UPLO - CHARACTER*1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SSYMM performs one of the matrix-matrix operations C := alpha*A*B + beta*C, or C := alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and C are m by n matrices. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | SSYMM performs one of the matrix-matrix operations C := alpha*A*B + beta*C, or C := alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and C are m by n matrices. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALPHA` | input | `REAL` (`explicit`) | `*mut f32` | scalar | SSYMM performs one of the matrix-matrix operations C := alpha*A*B + beta*C, or C := alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and C are m by n matrices. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDA, *) | SSYMM performs one of the matrix-matrix operations C := alpha*A*B + beta*C, or C := alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and C are m by n matrices. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - REAL array of DIMENSION ( LDA, ka ), where ka is m when SIDE = 'L' or 'l' and is n otherwise. | A - REAL array of DIMENSION ( LDA, ka ), where ka is m when SIDE = 'L' or 'l' and is n otherwise. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDB, *) | SSYMM performs one of the matrix-matrix operations C := alpha*A*B + beta*C, or C := alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and C are m by n matrices. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDB` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | B - REAL array of DIMENSION ( LDB, n ). | B - REAL array of DIMENSION ( LDB, n ). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BETA` | input | `REAL` (`explicit`) | `*mut f32` | scalar | SSYMM performs one of the matrix-matrix operations C := alpha*A*B + beta*C, or C := alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and C are m by n matrices. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | input | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDC, *) | SSYMM performs one of the matrix-matrix operations C := alpha*A*B + beta*C, or C := alpha*B*A + beta*C, where alpha and beta are scalars, A is a symmetric matrix and B and C are m by n matrices. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDC` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | C - REAL array of DIMENSION ( LDC, n ). | C - REAL array of DIMENSION ( LDC, n ). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::blas::level3::ssymm`. Native symbol: `ssymm_`. Feature: `blas-level3`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level3::ssymm`
- Compatibility aliases: `slatec_sys::families::blas_level3::ssymm`
- Public declaration feature: `blas-level3`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
