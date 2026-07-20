# CGEEV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a complex general matrix.

## Description

Abstract CGEEV computes the eigenvalues and, optionally, the eigenvectors of a general complex matrix. Call Sequence Parameters(The values of parameters marked with * (star) will be changed by CGEEV.) A* COMPLEX(LDA,N) complex nonsymmetric input matrix. LDA INTEGER set by the user to the leading dimension of the complex array A. N INTEGER set by the user to the order of the matrices A and V, and the number of elements in E. E* COMPLEX(N) on return from CGEEV E contains the eigenvalues of A. See also INFO below. V* COMPLEX(LDV,N) on return from CGEEV if the user has set JOB = 0 V is not referenced. = nonzero the N eigenvectors of A are stored in the first N columns of V. See also INFO below. (If the input matrix A is nearly degenerate, V will be badly conditioned, i.e. have nearly dependent columns.) LDV INTEGER set by the user to the leading dimension of the array V if JOB is also set nonzero. In that case N must be .LE. LDV. If JOB is set to zero LDV is not referenced. WORK* REAL(3N) temporary storage vector. Contents changed by CGEEV. JOB INTEGER set by the user to = 0 eigenvalues only to be calculated by CGEEV. neither V nor LDV are referenced. = nonzero eigenvalues and vectors to be calculated. In this case A & V must be distinct arrays. Also, if LDA > LDV, CGEEV changes all the elements of A thru column N. If LDA < LDV, CGEEV changes all the elements of V through column N. If LDA = LDV only A(I,J) and V(I, J) for I,J = 1,...,N are changed by CGEEV. INFO* INTEGER on return from CGEEV the value of INFO is = 0 normal return, calculation successful. = K if the eigenvalue iteration fails to converge, eigenvalues K+1 through N are correct, but no eigenvectors were computed even if they were requested (JOB nonzero). Error Messages No. 1 recoverable N is greater than LDA No. 2 recoverable N is less than one. No. 3 recoverable JOB is nonzero and N is greater than LDV No. 4 warning LDA > LDV, elements of A other than the N by N input elements have been changed No. 5 warning LDA < LDV, elements of V other than the N by N output elements have been changed

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4A4`
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

- Canonical provider: `main-src/src/cgeev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cgeev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cgeev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cgeev.f) — `verified_cached`
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
