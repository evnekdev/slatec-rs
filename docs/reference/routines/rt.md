# RT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and eigenvectors of a special real tridiagonal matrix.

## Description

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) of a special REAL TRIDIAGONAL matrix. The property of the matrix required for use of this subroutine is that the products of pairs of corresponding off-diagonal elements be all non-negative. If eigenvectors are desired, no product can be zero unless both factors are zero. On Input NM must be set to the row dimension of the two-dimensional array parameter, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix A. N is an INTEGER variable. N must be less than or equal to NM. A contains the special real tridiagonal matrix in its first three columns. The subdiagonal elements are stored in the last N-1 positions of the first column, the diagonal elements in the second column, and the superdiagonal elements in the first N-1 positions of the third column. Elements A(1,1) and A(N,3) are arbitrary. A is a two-dimensional REAL array, dimensioned A(NM,3). MATZ is an INTEGER variable set equal to zero if only eigenvalues are desired. Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. On Output W contains the eigenvalues in ascending order. W is a one-dimensional REAL array, dimensioned W(N). Z contains the eigenvectors if MATZ is not zero. The eigenvectors are not normalized. Z is a two-dimensional REAL array, dimensioned Z(NM,N). IERR is an INTEGER flag set to Zero for normal return, 10*N if N is greater than NM, N+J if A(J,1)*A(J-1,3) is negative, 2*N+J if the product is zero with one factor non-zero, and MATZ is non-zero; J if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors in the W and Z arrays should be correct for indices 1, 2, ..., IERR-1. FV1 is a one-dimensional REAL array used for temporary storage, dimensioned FV1(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Canonical provider: `lin/rt.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/rt.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/rt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
