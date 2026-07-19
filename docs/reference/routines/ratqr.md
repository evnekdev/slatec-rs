# RATQR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the largest or smallest eigenvalues of a symmetric tridiagonal matrix using the rational QR method with Newton correction.

## Description

This subroutine is a translation of the ALGOL procedure RATQR, NUM. MATH. 11, 264-272(1968) by REINSCH and BAUER. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 257-265(1971). This subroutine finds the algebraically smallest or largest eigenvalues of a SYMMETRIC TRIDIAGONAL matrix by the rational QR method with Newton corrections. On Input N is the order of the matrix. N is an INTEGER variable. EPS1 is a theoretical absolute error tolerance for the computed eigenvalues. If the input EPS1 is non-positive, or indeed smaller than its default value, it is reset at each iteration to the respective default value, namely, the product of the relative machine precision and the magnitude of the current eigenvalue iterate. The theoretical absolute error in the K-th eigenvalue is usually not greater than K times EPS1. EPS1 is a REAL variable. D contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E in its last N-1 positions. E2(1) is arbitrary. E2 is a onedimensional REAL array, dimensioned E2(N). M is the number of eigenvalues to be found. M is an INTEGER variable. IDEF should be set to 1 if the input matrix is known to be positive definite, to -1 if the input matrix is known to be negative definite, and to 0 otherwise. IDEF is an INTEGER variable.

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

- Canonical provider: `lin/ratqr.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/ratqr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/ratqr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
