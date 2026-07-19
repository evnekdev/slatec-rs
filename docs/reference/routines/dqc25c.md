# DQC25C

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To compute I = Integral of F*W over (A,B) with error estimate, where W(X) = 1/(X-C)

## Description

Integration rules for the computation of CAUCHY PRINCIPAL VALUE integrals Standard fortran subroutine Double precision version PARAMETERS F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Left end point of the integration interval B - Double precision Right end point of the integration interval, B.GT.A C - Double precision Parameter in the WEIGHT function RESULT - Double precision Approximation to the integral result is computed by using a generalized Clenshaw-Curtis method if C lies within ten percent of the integration interval. In the other case the 15-point Kronrod rule obtained by optimal addition of abscissae to the 7-point Gauss rule, is applied. ABSERR - Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) KRUL - Integer Key which is decreased by 1 if the 15-point Gauss-Kronrod scheme has been used NEVAL - Integer Number of integrand evaluations ......................................................................

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
- GAMS classifications: `H2A2A2`
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

- Canonical provider: `main-src/src/dqc25c.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqc25c.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqc25c.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqc25c.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
