# CGEMM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Multiply a complex general matrix by a complex general matrix.

## Description

CGEMM performs one of the matrix-matrix operations C := alpha*op( A )*op( B ) + beta*C, where op( X ) is one of op( X ) = X or op( X ) = X' or op( X ) = conjg( X' ), alpha and beta are scalars, and A, B and C are matrices, with op( A ) an m by k matrix, op( B ) a k by n matrix and C an m by n matrix. Parameters TRANSA - CHARACTER*1. On entry, TRANSA specifies the form of op( A ) to be used in the matrix multiplication as follows: TRANSA = 'N' or 'n', op( A ) = A. TRANSA = 'T' or 't', op( A ) = A'. TRANSA = 'C' or 'c', op( A ) = conjg( A' ). Unchanged on exit. TRANSB - CHARACTER*1. On entry, TRANSB specifies the form of op( B ) to be used in the matrix multiplication as follows: TRANSB = 'N' or 'n', op( B ) = B. TRANSB = 'T' or 't', op( B ) = B'. TRANSB = 'C' or 'c', op( B ) = conjg( B' ). Unchanged on exit. M - INTEGER. On entry, M specifies the number of rows of the matrix op( A ) and of the matrix C. M must be at least zero. Unchanged on exit. N - INTEGER. On entry, N specifies the number of columns of the matrix op( B ) and the number of columns of the matrix C. N must be at least zero. Unchanged on exit. K - INTEGER. On entry, K specifies the number of columns of the matrix op( A ) and the number of rows of the matrix op( B ). K must be at least zero. Unchanged on exit. ALPHA - COMPLEX . On entry, ALPHA specifies the scalar alpha. Unchanged on exit. A - COMPLEX array of DIMENSION ( LDA, ka ), where ka is k when TRANSA = 'N' or 'n', and is m otherwise. Before entry with TRANSA = 'N' or 'n', the leading m by k part of the array A must contain the matrix A, otherwise the leading k by m part of the array A must contain the matrix A. Unchanged on exit. LDA - INTEGER. On entry, LDA specifies the first dimension of A as declared in the calling (sub) program. When TRANSA = 'N' or 'n' then LDA must be at least max( 1, m ), otherwise LDA must be at least max( 1, k ). Unchanged on exit. B - COMPLEX array of DIMENSION ( LDB, kb ), where kb is n when TRANSB = 'N' or 'n', and is k otherwise. Before entry with TRANSB = 'N' or 'n', the leading k by n part of the array B must contain the matrix B, otherwise the leading n by k part of the array B must contain the matrix B. Unchanged on exit. LDB - INTEGER. On entry, LDB specifies the first dimension of B as declared in the calling (sub) program. When TRANSB = 'N' or 'n' then LDB must be at least max( 1, k ), otherwise LDB must be at least max( 1, n ). Unchanged on exit. BETA - COMPLEX . On entry, BETA specifies the scalar beta. When BETA is supplied as zero then C need not be set on input. Unchanged on exit. C - COMPLEX array of DIMENSION ( LDC, n ). Before entry, the leading m by n part of the array C must contain the matrix C, except when beta is zero, in which case C need not be set on entry. On exit, the array C is overwritten by the m by n matrix ( alpha*op( A )*op( B ) + beta*C ). LDC - INTEGER. On entry, LDC specifies the first dimension of C as declared in the calling (sub) program. LDC must be at least max( 1, m ). Unchanged on exit.

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

- Canonical provider: `lin/cgemm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cgemm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cgemm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::blas::level3::cgemm`
- Current legacy Rust paths: `slatec_sys::families::blas_level3::cgemm`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
