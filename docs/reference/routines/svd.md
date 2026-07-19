# SVD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Perform the singular value decomposition of a rectangular matrix.

## Description

This subroutine is a translation of the ALGOL procedure SVD, NUM. MATH. 14, 403-420(1970) by Golub and Reinsch. HANDBOOK FOR AUTO. COMP., VOL II-LINEAR ALGEBRA, 134-151(1971). This subroutine determines the singular value decomposition T A=USV of a REAL M by N rectangular matrix. Householder bidiagonalization and a variant of the QR algorithm are used. On Input NM must be set to the row dimension of the two-dimensional array parameters, A, U and V, as declared in the calling program dimension statement. NM is an INTEGER variable. Note that NM must be at least as large as the maximum of M and N. M is the number of rows of A and U. N is the number of columns of A and U and the order of V. A contains the rectangular input matrix to be decomposed. A is a two-dimensional REAL array, dimensioned A(NM,N). MATU should be set to .TRUE. if the U matrix in the decomposition is desired, and to .FALSE. otherwise. MATU is a LOGICAL variable. MATV should be set to .TRUE. if the V matrix in the decomposition is desired, and to .FALSE. otherwise. MATV is a LOGICAL variable. On Output A is unaltered (unless overwritten by U or V). W contains the N (non-negative) singular values of A (the diagonal elements of S). They are unordered. If an error exit is made, the singular values should be correct for indices IERR+1, IERR+2, ..., N. W is a one-dimensional REAL array, dimensioned W(N). U contains the matrix U (orthogonal column vectors) of the decomposition if MATU has been set to .TRUE. Otherwise, U is used as a temporary array. U may coincide with A. If an error exit is made, the columns of U corresponding to indices of correct singular values should be correct. U is a two-dimensional REAL array, dimensioned U(NM,N). V contains the matrix V (orthogonal) of the decomposition if MATV has been set to .TRUE. Otherwise, V is not referenced. V may also coincide with A if U does not. If an error exit is made, the columns of V corresponding to indices of correct singular values should be correct. V is a twodimensional REAL array, dimensioned V(NM,N). IERR is an INTEGER flag set to Zero for normal return, K if the K-th singular value has not been determined after 30 iterations. RV1 is a one-dimensional REAL array used for temporary storage, dimensioned RV1(N). CALLS PYTHAG(A,B) for sqrt(A**2 + B**2). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `linpack`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/svd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/svd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/svd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/svd.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
