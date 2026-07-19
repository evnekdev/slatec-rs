# BAKVEC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of a certain real non-symmetric tridiagonal matrix from a symmetric tridiagonal matrix output from FIGI.

## Description

This subroutine forms the eigenvectors of a NONSYMMETRIC TRIDIAGONAL matrix by back transforming those of the corresponding symmetric matrix determined by FIGI. On INPUT NM must be set to the row dimension of the two-dimensional array parameters, T and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix T. N is an INTEGER variable. N must be less than or equal to NM. T contains the nonsymmetric matrix. Its subdiagonal is stored in the last N-1 positions of the first column, its diagonal in the N positions of the second column, and its superdiagonal in the first N-1 positions of the third column. T(1,1) and T(N,3) are arbitrary. T is a two-dimensional REAL array, dimensioned T(NM,3). E contains the subdiagonal elements of the symmetric matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned E(N). M is the number of eigenvectors to be back transformed. M is an INTEGER variable. Z contains the eigenvectors to be back transformed in its first M columns. Z is a two-dimensional REAL array, dimensioned Z(NM,M). On OUTPUT T is unaltered. E is destroyed. Z contains the transformed eigenvectors in its first M columns. IERR is an INTEGER flag set to Zero for normal return, 2*N+I if E(I) is zero with T(I,1) or T(I-1,3) non-zero. In this case, the symmetric matrix is not similar to the original matrix, and the eigenvectors cannot be found by this program. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4C4`
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

- Canonical provider: `lin/bakvec.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/bakvec.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
