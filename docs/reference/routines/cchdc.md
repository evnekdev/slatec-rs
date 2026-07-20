# CCHDC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the Cholesky decomposition of a positive definite matrix. A pivoting option allows the user to estimate the condition number of a positive definite matrix or determine the rank of a positive semidefinite matrix.

## Description

CCHDC computes the Cholesky decomposition of a positive definite matrix. A pivoting option allows the user to estimate the condition of a positive definite matrix or determine the rank of a positive semidefinite matrix. On Entry A COMPLEX(LDA,P). A contains the matrix whose decomposition is to be computed. Only the upper half of A need be stored. The lower part of The array A is not referenced. LDA INTEGER. LDA is the leading dimension of the array A. P INTEGER. P is the order of the matrix. WORK COMPLEX. WORK is a work array. JPVT INTEGER(P). JPVT contains integers that control the selection of the pivot elements, if pivoting has been requested. Each diagonal element A(K,K) is placed in one of three classes according to the value of JPVT(K)). If JPVT(K)) .GT. 0, then X(K) is an initial element. If JPVT(K)) .EQ. 0, then X(K) is a free element. If JPVT(K)) .LT. 0, then X(K) is a final element. Before the decomposition is computed, initial elements are moved by symmetric row and column interchanges to the beginning of the array A and final elements to the end. Both initial and final elements are frozen in place during the computation and only free elements are moved. At the K-th stage of the reduction, if A(K,K) is occupied by a free element it is interchanged with the largest free element A(L,L) with L .GE. K. JPVT is not referenced if JOB .EQ. 0. JOB INTEGER. JOB is an integer that initiates column pivoting. IF JOB .EQ. 0, no pivoting is done. IF JOB .NE. 0, pivoting is done. On Return A A contains in its upper half the Cholesky factor of the matrix A as it has been permuted by pivoting. JPVT JPVT(J) contains the index of the diagonal element of A that was moved into the J-th position, provided pivoting was requested. INFO contains the index of the last positive diagonal element of the Cholesky factor. For positive definite matrices INFO = P is the normal return. For pivoting with positive semidefinite matrices INFO will in general be less than P. However, INFO may be greater than the rank of A, since rounding error can cause an otherwise zero element to be positive. Indefinite systems will always cause INFO to be less than P.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2D1B`
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

- Canonical provider: `lin/cchdc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cchdc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cchdc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_c_automated_public`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cchdc`
- Current legacy Rust paths: `none`
- Public declaration feature: `batch-c-linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `representative_batch_smoke_only`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
