# CQRDC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Use Householder transformations to compute the QR factorization of an N by P matrix. Column pivoting is a users option.

## Description

CQRDC uses Householder transformations to compute the QR factorization of an N by P matrix X. Column pivoting based on the 2-norms of the reduced columns may be performed at the users option. On Entry X COMPLEX(LDX,P), where LDX .GE. N. X contains the matrix whose decomposition is to be computed. LDX INTEGER. LDX is the leading dimension of the array X. N INTEGER. N is the number of rows of the matrix X. P INTEGER. P is the number of columns of the matrix X. JVPT INTEGER(P). JVPT contains integers that control the selection of the pivot columns. The K-th column X(K) of X is placed in one of three classes according to the value of JVPT(K). If JVPT(K) .GT. 0, then X(K) is an initial column. If JVPT(K) .EQ. 0, then X(K) is a free column. If JVPT(K) .LT. 0, then X(K) is a final column. Before the decomposition is computed, initial columns are moved to the beginning of the array X and final columns to the end. Both initial and final columns are frozen in place during the computation and only free columns are moved. At the K-th stage of the reduction, if X(K) is occupied by a free column it is interchanged with the free column of largest reduced norm. JVPT is not referenced if JOB .EQ. 0. WORK COMPLEX(P). WORK is a work array. WORK is not referenced if JOB .EQ. 0. JOB INTEGER. JOB is an integer that initiates column pivoting. If JOB .EQ. 0, no pivoting is done. If JOB .NE. 0, pivoting is done. On Return X X contains in its upper triangle the upper triangular matrix R of the QR factorization. Below its diagonal X contains information from which the unitary part of the decomposition can be recovered. Note that if pivoting has been requested, the decomposition is not that of the original matrix X but that of X with its columns permuted as described by JVPT. QRAUX COMPLEX(P). QRAUX contains further information required to recover the unitary part of the decomposition. JVPT JVPT(K) contains the index of the column of the original matrix that has been interchanged into the K-th column, if pivoting was requested.

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
- GAMS classifications: `D5`
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

- Canonical provider: `lin/cqrdc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cqrdc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cqrdc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
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
- Public declaration feature: `raw-ffi-complex-arguments`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
