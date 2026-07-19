# DQK41

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To compute I = Integral of F over (A,B), with error estimate J = Integral of ABS(F) over (A,B)

## Description

Integration rules Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand FUNCTION F(X). The actual name for F needs to be declared E X T E R N A L in the calling program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration ON RETURN RESULT - Double precision Approximation to the integral I RESULT is computed by applying the 41-POINT GAUSS-KRONROD RULE (RESK) obtained by optimal addition of abscissae to the 20-POINT GAUSS RULE (RESG). ABSERR - Double precision Estimate of the modulus of the absolute error, which should not exceed ABS(I-RESULT) RESABS - Double precision Approximation to the integral J RESASC - Double precision Approximation to the integral of ABS(F-I/(B-A)) over (A,B)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A1A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dqk41.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqk41.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqk41.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqk41.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
