# DQAGP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = Integral of F over (A,B), hopefully satisfying following claim for accuracy break points of the integration interval, where local difficulties of the integrand may occur (e.g. SINGULARITIES, DISCONTINUITIES), are provided by the user.

## Description

Computation of a definite integral Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand Function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration NPTS2 - Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS.GE.2. If NPTS2.LT.2, The routine will end with IER = 6. POINTS - Double precision Vector of dimension NPTS2, the first (NPTS2-2) elements of which are the user provided break points. If these points do not constitute an ascending sequence there will be an automatic sorting. EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. The estimates for integral and error are less reliable. it is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 Maximum number of subdivisions allowed has been achieved. one can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned RESULT is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. it must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because NPTS2.LT.2 or break points are specified outside the integration range or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) RESULT, ABSERR, NEVAL, LAST are set to zero. Except when LENIW or LENW or NPTS2 is invalid, IWORK(1), IWORK(LIMIT+1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero. WORK(1) is set to A and WORK(LIMIT+1) to B (where LIMIT = (LENIW-NPTS2)/2). DIMENSIONING PARAMETERS LENIW - Integer Dimensioning parameter for IWORK LENIW determines LIMIT = (LENIW-NPTS2)/2, which is the maximum number of subintervals in the partition of the given integration interval (A,B), LENIW.GE.(3*NPTS2-2). If LENIW.LT.(3*NPTS2-2), the routine will end with IER = 6. LENW - Integer Dimensioning parameter for WORK LENW must be at least LENIW*2-NPTS2. If LENW.LT.LENIW*2-NPTS2, the routine will end with IER = 6. LAST - Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. WORK ARRAYS IWORK - Integer Vector of dimension at least LENIW. on return, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),..., WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise IWORK(LIMIT+1), ...,IWORK(LIMIT+LAST) Contain the subdivision levels of the subintervals, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as well as P2 is a user-provided break point or integration LIMIT, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L), IWORK(LIMIT*2+1), ..., IWORK(LIMIT*2+NPTS2) have no significance for the user, note that LIMIT = (LENIW-NPTS2)/2. WORK - Double precision Vector of dimension at least LENW on return WORK(1), ..., WORK(LAST) contain the left

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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_with_breakpoints`

## Providers

- Canonical provider: `main-src/src/dqagp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqagp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqagp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqagp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
