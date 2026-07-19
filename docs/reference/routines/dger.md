# DGER

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Perform the rank 1 operation.

## Description

DGER performs the rank 1 operation A := alpha*x*y' + A, where alpha is a scalar, x is an m element vector, y is an n element vector and A is an m by n matrix. Parameters M - INTEGER. On entry, M specifies the number of rows of the matrix A. M must be at least zero. Unchanged on exit. N - INTEGER. On entry, N specifies the number of columns of the matrix A. N must be at least zero. Unchanged on exit. ALPHA - DOUBLE PRECISION. On entry, ALPHA specifies the scalar alpha. Unchanged on exit. X - DOUBLE PRECISION array of dimension at least ( 1 + ( m - 1)*abs( INCX)). Before entry, the incremented array X must contain the m element vector x. Unchanged on exit. INCX - INTEGER. On entry, INCX specifies the increment for the elements of X. INCX must not be zero. Unchanged on exit. Y - DOUBLE PRECISION array of dimension at least ( 1 + ( n - 1 )*abs( INCY ) ). Before entry, the incremented array Y must contain the n element vector y. Unchanged on exit. INCY - INTEGER. On entry, INCY specifies the increment for the elements of Y. INCY must not be zero. Unchanged on exit. A - DOUBLE PRECISION array of DIMENSION ( LDA, n ). Before entry, the leading m by n part of the array A must contain the matrix of coefficients. On exit, A is overwritten by the updated matrix. LDA - INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. LDA must be at least max( 1, m ). Unchanged on exit.

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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level2::dger`

## Providers

- Canonical provider: `lin/dger.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dger.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dger.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::blas::level2::dger`
- Current legacy Rust paths: `slatec_sys::families::blas_level2::dger`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::blas::level2::dger`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
