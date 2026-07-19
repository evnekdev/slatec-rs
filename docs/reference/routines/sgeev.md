# SGEEV

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a real general matrix.

## Description

Abstract SGEEV computes the eigenvalues and, optionally, the eigenvectors of a general real matrix. Call Sequence Parameters(The values of parameters marked with * (star) will be changed by SGEEV.) A* REAL(LDA,N) real nonsymmetric input matrix. LDA INTEGER set by the user to the leading dimension of the real array A. N INTEGER set by the user to the order of the matrices A and V, and the number of elements in E. E* COMPLEX(N) on return from SGEEV, E contains the eigenvalues of A. See also INFO below. V* COMPLEX(LDV,N) on return from SGEEV, if the user has set JOB = 0 V is not referenced. = nonzero the N eigenvectors of A are stored in the first N columns of V. See also INFO below. (Note that if the input matrix A is nearly degenerate, V may be badly conditioned, i.e., may have nearly dependent columns.) LDV INTEGER set by the user to the leading dimension of the array V if JOB is also set nonzero. In that case, N must be .LE. LDV. If JOB is set to zero, LDV is not referenced. WORK* REAL(2N) temporary storage vector. Contents changed by SGEEV. JOB INTEGER set by the user to = 0 eigenvalues only to be calculated by SGEEV. Neither V nor LDV is referenced. = nonzero eigenvalues and vectors to be calculated. In this case, A & V must be distinct arrays. Also, if LDA .GT. LDV, SGEEV changes all the elements of A thru column N. If LDA < LDV, SGEEV changes all the elements of V through column N. If LDA = LDV, only A(I,J) and V(I, J) for I,J = 1,...,N are changed by SGEEV. INFO* INTEGER on return from SGEEV the value of INFO is = 0 normal return, calculation successful. = K if the eigenvalue iteration fails to converge, eigenvalues K+1 through N are correct, but no eigenvectors were computed even if they were requested (JOB nonzero). Error Messages No. 1 recoverable N is greater than LDA No. 2 recoverable N is less than one. No. 3 recoverable JOB is nonzero and N is greater than LDV No. 4 warning LDA > LDV, elements of A other than the N by N input elements have been changed. No. 5 warning LDA < LDV, elements of V other than the N x N output elements have been changed.

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
- GAMS classifications: `D4A2`
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

- Canonical provider: `main-src/src/sgeev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sgeev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sgeev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sgeev.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
