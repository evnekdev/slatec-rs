# QNG

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = integral of F over (A,B), hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

NON-ADAPTIVE INTEGRATION STANDARD FORTRAN SUBROUTINE REAL VERSION F - Real version Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Real version Lower limit of integration B - Real version Upper limit of integration EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. ON RETURN RESULT - Real Approximation to the integral I Result is obtained by applying the 21-POINT GAUSS-KRONROD RULE (RES21) obtained by optimal addition of abscissae to the 10-POINT GAUSS RULE (RES10), or by applying the 43-POINT RULE (RES43) obtained by optimal addition of abscissae to the 21-POINT GAUSS-KRONROD RULE, or by applying the 87-POINT RULE (RES87) obtained by optimal addition of abscissae to the 43-POINT RULE. ABSERR - Real Estimate of the modulus of the absolute error, which should EQUAL or EXCEED ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - IER = 0 normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 The maximum number of steps has been executed. The integral is probably too difficult to be calculated by DQNG. = 6 The input is invalid, because EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28). RESULT, ABSERR and NEVAL are set to zero.

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
- GAMS classifications: `H2A1A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_non_adaptive_f32`

## Providers

- Canonical provider: `main-src/src/qng.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qng.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qng.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qng.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
