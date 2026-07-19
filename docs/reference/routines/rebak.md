# REBAK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of a generalized symmetric eigensystem from the eigenvectors of derived matrix output from REDUC or REDUC2.

## Description

This subroutine is a translation of the ALGOL procedure REBAKA, NUM. MATH. 11, 99-110(1968) by Martin and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 303-314(1971). This subroutine forms the eigenvectors of a generalized SYMMETRIC eigensystem by back transforming those of the derived symmetric matrix determined by REDUC or REDUC2. On Input NM must be set to the row dimension of the two-dimensional array parameters, B and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix system. N is an INTEGER variable. N must be less than or equal to NM. B contains information about the similarity transformation (Cholesky decomposition) used in the reduction by REDUC or REDUC2 in its strict lower triangle. B is a twodimensional REAL array, dimensioned B(NM,N). DL contains further information about the transformation. DL is a one-dimensional REAL array, dimensioned DL(N). M is the number of eigenvectors to be back transformed. M is an INTEGER variable. Z contains the eigenvectors to be back transformed in its first M columns. Z is a two-dimensional REAL array dimensioned Z(NM,M). On Output Z contains the transformed eigenvectors in its first M columns. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Canonical provider: `lin/rebak.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/rebak.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
