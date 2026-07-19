# SSIEV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix.

## Description

Abstract SSIEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix. Call Sequence Parameters(The values of parameters marked with * (star) will be changed by SSIEV.) A* REAL (LDA,N) real symmetric input matrix. Only the diagonal and upper triangle of A must be input, as SSIEV copies the upper triangle to the lower. That is, the user must define A(I,J), I=1,..N, and J=I,. ..,N. On return from SSIEV, if the user has set JOB = 0 the lower triangle of A has been altered. = nonzero the N eigenvectors of A are stored in its first N columns. See also INFO below. LDA INTEGER set by the user to the leading dimension of the array A. N INTEGER set by the user to the order of the matrix A and the number of elements in E. E* REAL (N) on return from SSIEV, E contains the N eigenvalues of A. See also INFO below. WORK* REAL (2*N) temporary storage vector. Contents changed by SSIEV. JOB INTEGER set by user on input = 0 only calculate eigenvalues of A. = nonzero calculate eigenvalues and eigenvectors of A. INFO* INTEGER on return from SSIEV, the value of INFO is = 0 for normal return. = K if the eigenvalue iteration fails to converge. eigenvalues and vectors 1 through K-1 are correct. Error MessagesNo. 1 recoverable N is greater than LDA No. 2 recoverable N is less than one

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4A1`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/ssiev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ssiev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ssiev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ssiev.f) — `verified_cached`
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
