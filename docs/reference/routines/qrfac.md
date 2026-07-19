# QRFAC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SNLS1, SNLS1E, SNSQ and SNSQE

## Description

This subroutine uses Householder transformations with column pivoting (optional) to compute a QR factorization of the M by N matrix A. That is, QRFAC determines an orthogonal matrix Q, a permutation matrix P, and an upper trapezoidal matrix R with diagonal elements of nonincreasing magnitude, such that A*P = Q*R. The Householder transformation for column K, K = 1,2,...,MIN(M,N), is of the form T I - (1/U(K))*U*U where U has zeros in the first K-1 positions. The form of this transformation and the method of pivoting first appeared in the corresponding LINPACK subroutine. The subroutine statement is SUBROUTINE QRFAC(M,N,A,LDA,PIVOT,IPVT,LIPVT,SIGMA,ACNORM,WA) where M is a positive integer input variable set to the number of rows of A. N is a positive integer input variable set to the number of columns of A. A is an M by N array. On input A contains the matrix for which the QR factorization is to be computed. On output the strict upper trapezoidal part of A contains the strict upper trapezoidal part of R, and the lower trapezoidal part of A contains a factored form of Q (the non-trivial elements of the U vectors described above). LDA is a positive integer input variable not less than M which specifies the leading dimension of the array A. PIVOT is a logical input variable. If pivot is set .TRUE., then column pivoting is enforced. If pivot is set .FALSE., then no column pivoting is done. IPVT is an integer output array of length LIPVT. IPVT defines the permutation matrix P such that A*P = Q*R. Column J of P is column IPVT(J) of the identity matrix. If pivot is .FALSE., IPVT is not referenced. LIPVT is a positive integer input variable. If PIVOT is .FALSE., then LIPVT may be as small as 1. If PIVOT is .TRUE., then LIPVT must be at least N. SIGMA is an output array of length N which contains the diagonal elements of R. ACNORM is an output array of length N which contains the norms of the corresponding columns of the input matrix A. If this information is not needed, then ACNORM can coincide with SIGMA. WA is a work array of length N. If pivot is .FALSE., then WA can coincide with SIGMA.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Shared numerical utilities`
- Mathematical domain: `data-utilities`
- Package provenance: `multiple-parent-families`
- Secondary families: `Approximation, Nonlinear equations`
- Family evidence: `parent_inheritance` (`medium`)
- Parent-family evidence: `SNLS1, SNLS1E, SNSQ, SNSQE`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qrfac.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qrfac.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qrfac.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qrfac.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
