# STRSM

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a real triangular system of equations with multiple right-hand sides.

## Description

STRSM solves one of the matrix equations op( A )*X = alpha*B, or X*op( A ) = alpha*B, where alpha is a scalar, X and B are m by n matrices, A is a unit, or non-unit, upper or lower triangular matrix and op( A ) is one of op( A ) = A or op( A ) = A'. The matrix X is overwritten on B.

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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level3::strsm`

## Providers

- Canonical provider: `lin/strsm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/strsm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/strsm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [STRSM](https://www.netlib.org/slatec/lin/strsm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `SIDE` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, SIDE specifies whether op( A ) appears on the left or right of X as follows: 'L' or 'l' op( A )*X = alpha*B. 'R' or 'r' X*op( A ) = alpha*B. Unchanged on exit. |
| 2 | `UPLO` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, UPLO specifies whether the matrix A is an upper or lower triangular matrix as follows: 'U' or 'u' A is an upper triangular matrix. 'L' or 'l' A is a lower triangular matrix. Unchanged on exit. |
| 3 | `TRANSA` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: 'N' or 'n' op( A ) = A. 'T' or 't' op( A ) = A'. 'C' or 'c' op( A ) = A'. Unchanged on exit. |
| 4 | `DIAG` | `input` | `scalar` | `CHARACTER` | `*mut core::ffi::c_char` | scalar | CHARACTER*1. On entry, DIAG specifies whether or not A is unit triangular as follows: 'U' or 'u' A is assumed to be unit triangular. 'N' or 'n' A is not assumed to be unit Unchanged on exit. |
| 5 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, M specifies the number of rows of B. M must be at least zero. Unchanged on exit. |
| 6 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, N specifies the number of columns of B. N must be at least zero. Unchanged on exit. |
| 7 | `ALPHA` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | REAL. On entry, ALPHA specifies the scalar alpha. When alpha is zero then A is not referenced and B need not be set before entry. Unchanged on exit. |
| 8 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDA, *) | REAL array of DIMENSION ( LDA, k ), where k is m when SIDE = 'L' or 'l' and is n when SIDE = 'R' or 'r'. Before entry with UPLO = 'U' or 'u', the leading k by k upper triangular part of the array A must contain the upper triangular matrix and the strictly lower triangular part of is not referenced. Before entry with UPLO = 'L' or 'l', the leading k by k lower triangular part of the array A must contain the lower triangular matrix and the strictly upper triangular part of Note that when DIAG = 'U' or 'u', the diagonal elements of are not referenced either, but are assumed to be unity. Unchanged on exit. |
| 9 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When SIDE = 'L' or 'l' then must be at least max( 1, m ), when SIDE = 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. |
| 10 | `B` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (LDB, *) | REAL array of DIMENSION ( LDB, n ). Before entry, the leading m by n part of the array B must contain the right-hand side matrix B, and on exit is overwritten by the solution matrix X. |
| 11 | `LDB` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. LDB must be at least max( 1, m ). Unchanged on exit. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level3::strsm`. Native symbol: `strsm_`. Declaration feature: `blas-level3`. Provider feature: `blas-level3`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level3::strsm`
- Public declaration feature: `blas-level3`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::blas::level3::strsm`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
