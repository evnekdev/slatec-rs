# IMTQL2

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and eigenvectors of a symmetric tridiagonal matrix using the implicit QL method.

## Description

This subroutine is a translation of the ALGOL procedure IMTQL2, NUM. MATH. 12, 377-383(1968) by Martin and Wilkinson, as modified in NUM. MATH. 15, 450(1970) by Dubrulle. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 241-248(1971). This subroutine finds the eigenvalues and eigenvectors of a SYMMETRIC TRIDIAGONAL matrix by the implicit QL method. The eigenvectors of a FULL SYMMETRIC matrix can also be found if TRED2 has been used to reduce this full matrix to tridiagonal form. On INPUT NM must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix. N is an INTEGER variable. N must be less than or equal to NM. D contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned E(N). Z contains the transformation matrix produced in the reduction by TRED2, if performed. This transformation matrix is necessary if you want to obtain the eigenvectors of the full symmetric matrix. If the eigenvectors of the symmetric tridiagonal matrix are desired, Z must contain the identity matrix. Z is a two-dimensional REAL array, dimensioned Z(NM,N). On OUTPUT D contains the eigenvalues in ascending order. If an error exit is made, the eigenvalues are correct but unordered for indices 1, 2, ..., IERR-1. E has been destroyed. Z contains orthonormal eigenvectors of the full symmetric or symmetric tridiagonal matrix, depending on what it contained on input. If an error exit is made, Z contains the eigenvectors associated with the stored eigenvalues. IERR is an INTEGER flag set to Zero for normal return, J if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors should be correct for indices 1, 2, ..., IERR-1, but the eigenvalues are not ordered. Calls PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `integer_or_index`
- Scalar kind: `integer`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4A5`
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

- Canonical provider: `lin/imtql2.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/imtql2.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/imtql2.f) — `verified_cached`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
