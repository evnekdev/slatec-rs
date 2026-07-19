# DQCHEB

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

This routine computes the CHEBYSHEV series expansion of degrees 12 and 24 of a function using A FAST FOURIER TRANSFORM METHOD F(X) = SUM(K=1,..,13) (CHEB12(K)*T(K-1,X)), F(X) = SUM(K=1,..,25) (CHEB24(K)*T(K-1,X)), Where T(K,X) is the CHEBYSHEV POLYNOMIAL OF DEGREE K.

## Description

Chebyshev Series Expansion Standard Fortran Subroutine Double precision version PARAMETERS ON ENTRY X - Double precision Vector of dimension 11 containing the Values COS(K*PI/24), K = 1, ..., 11 FVAL - Double precision Vector of dimension 25 containing the function values at the points (B+A+(B-A)*COS(K*PI/24))/2, K = 0, ...,24, where (A,B) is the approximation interval. FVAL(1) and FVAL(25) are divided by two (these values are destroyed at output). ON RETURN CHEB12 - Double precision Vector of dimension 13 containing the CHEBYSHEV coefficients for degree 12 CHEB24 - Double precision Vector of dimension 25 containing the CHEBYSHEV Coefficients for degree 24

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `quadpack`
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

- Canonical provider: `main-src/src/dqcheb.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqcheb.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqcheb.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
