# SGEMV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Multiply a real vector by a real general matrix.

## Description

SGEMV performs one of the matrix-vector operations y := alpha*A*x + beta*y, or y := alpha*A'*x + beta*y, where alpha and beta are scalars, x and y are vectors and A is an m by n matrix. Parameters TRANS - CHARACTER*1. On entry, TRANS specifies the operation to be performed as follows: TRANS = 'N' or 'n' y := alpha*A*x + beta*y. TRANS = 'T' or 't' y := alpha*A'*x + beta*y. TRANS = 'C' or 'c' y := alpha*A'*x + beta*y. Unchanged on exit. M - INTEGER. On entry, M specifies the number of rows of the matrix A. M must be at least zero. Unchanged on exit. N - INTEGER. On entry, N specifies the number of columns of the matrix A. N must be at least zero. Unchanged on exit. ALPHA - REAL . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. A - REAL array of DIMENSION ( LDA, n ). Before entry, the leading m by n part of the array A must contain the matrix of coefficients. Unchanged on exit. LDA - INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, m ). Unchanged on exit. X - REAL array of DIMENSION at least ( 1 + ( n - 1 )*abs( INCX ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( m - 1 )*abs( INCX ) ) otherwise. Before entry, the incremented array X must contain the vector x. Unchanged on exit. INCX - INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit. BETA - REAL . On entry, BETA specifies the scalar beta. When BETA is supplied as zero then Y need not be set on input. Unchanged on exit. Y - REAL array of DIMENSION at least ( 1 + ( m - 1 )*abs( INCY ) ) when TRANS = 'N' or 'n' and at least ( 1 + ( n - 1 )*abs( INCY ) ) otherwise. Before entry with BETA non-zero, the incremented array Y must contain the vector y. On exit, Y is overwritten by the updated vector y. INCY - INTEGER. On entry, INCY specifies the increment for the elements of Y. INCY must not be zero. Unchanged on exit.

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
- GAMS classifications: `D1B4`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level2::sgemv, slatec::blas::level2::sgemv_contiguous`

## Providers

- Canonical provider: `lin/sgemv.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sgemv.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sgemv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::blas::level2::sgemv`
- Current legacy Rust paths: `slatec_sys::families::blas_level2::sgemv`
- Public declaration feature: `blas-level2`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::blas::level2::sgemv`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
