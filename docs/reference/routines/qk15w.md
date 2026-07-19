# QK15W

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To compute I = Integral of F*W over (A,B), with error estimate J = Integral of ABS(F*W) over (A,B)

## Description

Integration rules Standard fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. W - Real Function subprogram defining the integrand WEIGHT function W(X). The actual name for W needs to be declared E X T E R N A L in the calling program. P1, P2, P3, P4 - Real Parameters in the WEIGHT function KP - Integer Key for indicating the type of WEIGHT function A - Real Lower limit of integration B - Real Upper limit of integration ON RETURN RESULT - Real Approximation to the integral I RESULT is computed by applying the 15-point Kronrod rule (RESK) obtained by optimal addition of abscissae to the 7-point Gauss rule (RESG). ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) RESABS - Real Approximation to the integral of ABS(F) RESASC - Real Approximation to the integral of ABS(F-I/(B-A))

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/qk15w.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qk15w.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qk15w.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qk15w.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
