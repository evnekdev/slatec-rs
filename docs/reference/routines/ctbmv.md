# CTBMV

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Multiply a complex vector by a complex triangular band matrix.

## Description

CTBMV performs one of the matrix-vector operations

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
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

- Canonical provider: `lin/ctbmv.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ctbmv.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ctbmv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CTBMV](https://www.netlib.org/slatec/lin/ctbmv.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `UPLO` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, UPLO specifies whether the matrix is an upper or lower triangular matrix as follows: 'U' or 'u' A is an upper triangular matrix. 'L' or 'l' A is a lower triangular matrix. Unchanged on exit. |
| 2 | `TRANS` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, TRANS specifies the operation to be performed as follows: = A*x. 'T' or 't' x := A'*x. 'C' or 'c' x := conjg( A' )*x. Unchanged on exit. |
| 3 | `DIAG` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, DIAG specifies whether or not A is unit triangular as follows: 'U' or 'u' A is assumed to be unit triangular. 'N' or 'n' A is not assumed to be unit Unchanged on exit. |
| 4 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | = A*x. INTEGER. On entry, N specifies the order of the matrix A. must be at least zero. Unchanged on exit. |
| 5 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry with UPLO = 'U' or 'u', K specifies the number of super-diagonals of the matrix A. On entry with UPLO = 'L' or 'l', K specifies the number of sub-diagonals of the matrix A. must satisfy 0. le. K. |
| 6 | `A` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDA, *) | COMPLEX array of DIMENSION ( LDA, n ). Before entry with UPLO = 'U' or 'u', the leading ( k + 1 ) by n part of the array A must contain the upper triangular band part of the matrix of coefficients, supplied column by column, with the leading diagonal of the matrix in row ( k + 1 ) of the array, the first super-diagonal starting at position 2 in row k, and so on. The top left k by k triangle of the array A is not referenced. The following program segment will transfer an upper triangular band matrix from conventional full matrix storage to band storage: DO 20, J = 1, N M = K + 1 - J DO 10, I = MAX( 1, J - K ), J matrix( I, J ) 10 CONTINUE 20 CONTINUE Before entry with UPLO = 'L' or 'l', the leading ( k + 1 ) by n part of the array A must contain the lower triangular column, with the leading diagonal of the matrix in row 1 of the array, the first sub-diagonal starting at position 1 in row 2, and so on. The bottom right k by k triangle of the The following program segment will transfer a lower M = 1 - J DO 10, I = J, MIN( N, J + K ) Note that when DIAG = 'U' or 'u' the elements of the array A corresponding to the diagonal elements of the matrix are not referenced, but are assumed to be unity. Unchanged on exit. |
| 7 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least ( k + 1 ). Unchanged on exit. |
| 8 | `X` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | = A*x, or x := A'*x, or x := conjg( A')*x, where x is an n element vector and A is an n by n unit, or non-unit, upper or lower triangular band matrix, with ( k + 1 ) diagonals. = A*x. COMPLEX array of dimension at least ( 1 + ( n - 1 )*abs( INCX ) ). Before entry, the incremented array X must contain the n element vector x. On exit, X is overwritten with the transformed vector x. |
| 9 | `INCX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level2::ctbmv`. Native symbol: `ctbmv_`. Declaration feature: `blas-level2`. Provider feature: `blas-level2`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level2::ctbmv`
- Public declaration feature: `blas-level2`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
