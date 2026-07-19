# DQK15W

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To compute I = Integral of F*W over (A,B), with error estimate J = Integral of ABS(F*W) over (A,B)

## Description

Integration rules Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. W - Double precision Function subprogram defining the integrand WEIGHT function W(X). The actual name for W needs to be declared E X T E R N A L in the calling program. P1, P2, P3, P4 - Double precision Parameters in the WEIGHT function KP - Integer Key for indicating the type of WEIGHT function A - Double precision Lower limit of integration B - Double precision Upper limit of integration ON RETURN RESULT - Double precision Approximation to the integral I RESULT is computed by applying the 15-point Kronrod rule (RESK) obtained by optimal addition of abscissae to the 7-point Gauss rule (RESG). ABSERR - Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) RESABS - Double precision Approximation to the integral of ABS(F) RESASC - Double precision Approximation to the integral of ABS(F-I/(B-A))

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

- Canonical provider: `main-src/src/dqk15w.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqk15w.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqk15w.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqk15w.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
