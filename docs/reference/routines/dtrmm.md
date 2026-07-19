# DTRMM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Perform one of the matrix-matrix operations.

## Description

DTRMM performs one of the matrix-matrix operations B := alpha*op( A )*B, or B := alpha*B*op( A ), where alpha is a scalar, B is an m by n matrix, A is a unit, or non-unit, upper or lower triangular matrix and op( A ) is one of op( A ) = A or op( A ) = A'. Parameters SIDE - CHARACTER*1. On entry, SIDE specifies whether op( A ) multiplies B from the left or right as follows: SIDE = 'L' or 'l' B := alpha*op( A )*B. SIDE = 'R' or 'r' B := alpha*B*op( A ). Unchanged on exit. UPLO - CHARACTER*1. On entry, UPLO specifies whether the matrix A is an upper or lower triangular matrix as follows: UPLO = 'U' or 'u' A is an upper triangular matrix. UPLO = 'L' or 'l' A is a lower triangular matrix. Unchanged on exit. TRANSA - CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: TRANSA = 'N' or 'n' op( A ) = A. TRANSA = 'T' or 't' op( A ) = A'. TRANSA = 'C' or 'c' op( A ) = A'. Unchanged on exit. DIAG - CHARACTER*1. On entry, DIAG specifies whether or not A is unit triangular as follows: DIAG = 'U' or 'u' A is assumed to be unit triangular. DIAG = 'N' or 'n' A is not assumed to be unit triangular. Unchanged on exit. M - INTEGER. On entry, M specifies the number of rows of B. M must be at least zero. Unchanged on exit. N - INTEGER. On entry, N specifies the number of columns of B. N must be at least zero. Unchanged on exit. ALPHA - DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. When alpha is zero then A is not referenced and B need not be set before entry. Unchanged on exit. A - DOUBLE PRECISION array of DIMENSION ( LDA, k ), where k is m when SIDE = 'L' or 'l' and is n when SIDE = 'R' or 'r'. Before entry with UPLO = 'U' or 'u', the leading k by k upper triangular part of the array A must contain the upper triangular matrix and the strictly lower triangular part of A is not referenced. Before entry with UPLO = 'L' or 'l', the leading k by k lower triangular part of the array A must contain the lower triangular matrix and the strictly upper triangular part of A is not referenced. Note that when DIAG = 'U' or 'u', the diagonal elements of A are not referenced either, but are assumed to be unity. Unchanged on exit. LDA - INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When SIDE = 'L' or 'l' then LDA must be at least max( 1, m ), when SIDE = 'R' or 'r' then LDA must be at least max( 1, n ). Unchanged on exit. B - DOUBLE PRECISION array of DIMENSION ( LDB, n ). Before entry, the leading m by n part of the array B must contain the matrix B, and on exit is overwritten by the transformed matrix. LDB - INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. LDB must be at least max( 1, m ). Unchanged on exit.

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
- GAMS classifications: `D1B6`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level3::dtrmm`

## Providers

- Canonical provider: `lin/dtrmm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dtrmm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dtrmm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::blas::level3::dtrmm`
- Current legacy Rust paths: `slatec_sys::families::blas_level3::dtrmm`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::blas::level3::dtrmm`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
