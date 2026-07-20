# DGBMV

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Perform one of the matrix-vector operations.

## Description

DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. Parameters TRANS - CHARACTER*1. On entry, TRANS specifies the operation to be performed as follows: TRANS = 'N' or 'n' y := alpha*A*x + beta*y. TRANS = 'T' or 't' y := alpha*A'*x + beta*y. TRANS = 'C' or 'c' y := alpha*A'*x + beta*y. Unchanged on exit. M - INTEGER. On entry, M specifies the number of rows of the matrix A. M must be at least zero. Unchanged on exit. N - INTEGER. On entry, N specifies the number of columns of the matrix A. N must be at least zero. Unchanged on exit. KL - INTEGER. On entry, KL specifies the number of sub-diagonals of the matrix A. KL must satisfy 0 .le. KL. Unchanged on exit. KU - INTEGER. On entry, KU specifies the number of super-diagonals of the matrix A. KU must satisfy 0 .le. KU. Unchanged on exit. ALPHA - DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit. A - DOUBLE PRECISION array of DIMENSION ( LDA, n ). Before entry, the leading ( kl + ku + 1 ) by n part of the array A must contain the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row ( ku + 1 ) of the array, the first super-diagonal starting at position 2 in row ku, the first sub-diagonal starting at position 1 in row ( ku + 2 ), and so on. Elements in the array A that do not correspond to elements in the band matrix (such as the top left ku by ku triangle) are not referenced. The following program segment will transfer a band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N K = KU + 1 - J DO 10, I = MAX( 1, J - KU ), MIN( M, J + KL ) A( K + I, J ) = matrix( I, J ) 10 CONTINUE 20 CONTINUE Unchanged on exit. LDA - INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( kl + ku + 1 ). Unchanged on exit. X - DOUBLE PRECISION array of DIMENSION at least ( 1 + ( n - 1 )*abs( INCX ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( m - 1 )*abs( INCX ) ) otherwise. Before entry, the incremented array X must contain the vector x. Unchanged on exit. INCX - INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit. BETA - DOUBLE PRECISION. On entry, BETA specifies the scalar beta. When BETA is supplied as zero then Y need not be set on input. Unchanged on exit. Y - DOUBLE PRECISION array of DIMENSION at least ( 1 + ( m - 1 )*abs( INCY ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( n - 1 )*abs( INCY ) ) otherwise. Before entry, the incremented array Y must contain the vector y. On exit, Y is overwritten by the updated vector y. INCY - INTEGER. On entry, INCY specifies the increment for the elements of Y. INCY must not be zero. Unchanged on exit.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1B4`
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

- Canonical provider: `lin/dgbmv.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dgbmv.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dgbmv.f) — `verified_cached`
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
| `TRANS` | input | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | Parameters TRANS - CHARACTER*1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KL` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KU` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALPHA` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (LDA, *) | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - DOUBLE PRECISION array of DIMENSION ( LDA, n ). | A - DOUBLE PRECISION array of DIMENSION ( LDA, n ). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INCX` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | X - DOUBLE PRECISION array of DIMENSION at least ( 1 + ( n - 1 )*abs( INCX ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( m - 1 )*abs( INCX ) ) otherwise. | X - DOUBLE PRECISION array of DIMENSION at least ( 1 + ( n - 1 )*abs( INCX ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( m - 1 )*abs( INCX ) ) otherwise. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BETA` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | DGBMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n band matrix, with kl sub-diagonals and ku super-diagonals. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INCY` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Y - DOUBLE PRECISION array of DIMENSION at least ( 1 + ( m - 1 )*abs( INCY ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( n - 1 )*abs( INCY ) ) otherwise. | Y - DOUBLE PRECISION array of DIMENSION at least ( 1 + ( m - 1 )*abs( INCY ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( n - 1 )*abs( INCY ) ) otherwise. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::blas::level2::dgbmv`. Native symbol: `dgbmv_`. Feature: `blas-level2`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level2::dgbmv`
- Compatibility aliases: `slatec_sys::families::blas_level2::dgbmv`
- Public declaration feature: `blas-level2`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
