# DOGLEG

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SNSQ and SNSQE

## Description

Given an M by N matrix A, an N by N nonsingular DIAGONAL matrix D, an M-vector B, and a positive number DELTA, the problem is to determine the convex combination X of the Gauss-Newton and scaled gradient directions that minimizes (A*X - B) in the least squares sense, subject to the restriction that the Euclidean norm of D*X be at most DELTA. This subroutine completes the solution of the problem if it is provided with the necessary information from the QR factorization of A. That is, if A = Q*R, where Q has orthogonal columns and R is an upper triangular matrix, then DOGLEG expects the full upper triangle of R and the first N components of (Q TRANSPOSE)*B. The subroutine statement is SUBROUTINE DOGLEG(N,R,LR,DIAG,QTB,DELTA,X,WA1,WA2) where N is a positive integer input variable set to the order of R. R is an input array of length LR which must contain the upper triangular matrix R stored by rows. LR is a positive integer input variable not less than (N*(N+1))/2. DIAG is an input array of length N which must contain the diagonal elements of the matrix D. QTB is an input array of length N which must contain the first N elements of the vector (Q TRANSPOSE)*B. DELTA is a positive input variable which specifies an upper bound on the Euclidean norm of D*X. X is an output array of length N which contains the desired convex combination of the Gauss-Newton direction and the scaled gradient direction. WA1 and WA2 are work arrays of length N.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Nonlinear equations`
- Mathematical domain: `nonlinear-equations`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SNSQ, SNSQE`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dogleg.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dogleg.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dogleg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dogleg.f) — `verified_cached`
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
