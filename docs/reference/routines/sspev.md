# SSPEV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix stored in packed form.

## Description

Abstract SSPEV computes the eigenvalues and, optionally, the eigenvectors of a real symmetric matrix stored in packed form. Call Sequence Parameters(The values of parameters marked with * (star) will be changed by SSPEV.) A* REAL(N*(N+1)/2) real symmetric packed input matrix. Contains upper triangle and diagonal of A, by column (elements 11, 12, 22, 13, 23, 33, ...). N INTEGER set by the user to the order of the matrix A. E* REAL(N) on return from SSPEV, E contains the eigenvalues of A. See also INFO below. V* REAL(LDV,N) on return from SSPEV, if the user has set JOB = 0 V is not referenced. = nonzero the N eigenvectors of A are stored in the first N columns of V. See also INFO below. LDV INTEGER set by the user to the leading dimension of the array V if JOB is also set nonzero. In that case, N must be .LE. LDV. If JOB is set to zero, LDV is not referenced. WORK* REAL(2N) temporary storage vector. Contents changed by SSPEV. JOB INTEGER set by the user to = 0 eigenvalues only to be calculated by SSPEV. Neither V nor LDV are referenced. = nonzero eigenvalues and vectors to be calculated. In this case, A & V must be distinct arrays. Also, if LDA .GT. LDV, SSPEV changes all the elements of A thru column N. If LDA < LDV, SSPEV changes all the elements of V through column N. If LDA=LDV, only A(I,J) and V(I, J) for I,J = 1,...,N are changed by SSPEV. INFO* INTEGER on return from SSPEV, the value of INFO is = 0 for normal return. = K if the eigenvalue iteration fails to converge. Eigenvalues and vectors 1 through K-1 are correct. Error MessagesNo. 1 recoverable N is greater than LDV and JOB is nonzero No. 2 recoverable N is less than one

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

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/sspev.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/sspev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/sspev.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
