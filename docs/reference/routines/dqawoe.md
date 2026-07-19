# DQAWOE

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate an approximation to a given definite integral I = Integral of F(X)*W(X) over (A,B), where W(X) = COS(OMEGA*X) or W(X)=SIN(OMEGA*X), hopefully satisfying the following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of Oscillatory integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration OMEGA - Double precision Parameter in the integrand weight function INTEGR - Integer Indicates which of the WEIGHT functions is to be used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) If INTEGR.NE.1 and INTEGR.NE.2, the routine will end with IER = 6. EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. LIMIT - Integer Gives an upper bound on the number of subdivisions in the partition of (A,B), LIMIT.GE.1. ICALL - Integer If DQAWOE is to be used only once, ICALL must be set to 1. Assume that during this call, the Chebyshev moments (for CLENSHAW-CURTIS integration of degree 24) have been computed for intervals of lengths (ABS(B-A))*2**(-L), L=0,1,2,...MOMCOM-1. If ICALL.GT.1 this means that DQAWOE has been called twice or more on intervals of the same length ABS(B-A). The Chebyshev moments already computed are then re-used in subsequent calls. If ICALL.LT.1, the routine will end with IER = 6. MAXP1 - Integer Gives an upper bound on the number of Chebyshev moments which can be stored, i.e. for the intervals of lengths ABS(B-A)*2**(-L), L=0,1, ..., MAXP1-2, MAXP1.GE.1. If MAXP1.LT.1, the routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. - IER.GT.0 Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand, in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved due to roundoff in the extrapolation table, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid, because (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or (INTEGR.NE.1 and INTEGR.NE.2) or ICALL.LT.1 or MAXP1.LT.1. RESULT, ABSERR, NEVAL, LAST, RLIST(1), ELIST(1), IORD(1) and NNLOG(1) are set to ZERO. ALIST(1) and BLIST(1) are set to A and B respectively. LAST - Integer On return, LAST equals the number of subintervals produces in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. ALIST - Double precision Vector of dimension at least LIMIT, the first LAST elements of which are the left

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
- GAMS classifications: `H2A2A1`
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

- Canonical provider: `main-src/src/dqawoe.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqawoe.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqawoe.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqawoe.f) — `verified_cached`
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
