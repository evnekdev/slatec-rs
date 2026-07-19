# DQAGPE

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Approximate a given definite integral I = Integral of F over (A,B), hopefully satisfying the accuracy claim: ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)). Break points of the integration interval, where local difficulties of the integrand may occur (e.g. singularities or discontinuities) are provided by the user.

## Description

Computation of a definite integral Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration NPTS2 - Integer Number equal to two more than the number of user-supplied break points within the integration range, NPTS2.GE.2. If NPTS2.LT.2, the routine will end with IER = 6. POINTS - Double precision Vector of dimension NPTS2, the first (NPTS2-2) elements of which are the user provided break POINTS. If these POINTS do not constitute an ascending sequence there will be an automatic sorting. EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. LIMIT - Integer Gives an upper bound on the number of subintervals in the partition of (A,B), LIMIT.GE.NPTS2 If LIMIT.LT.NPTS2, the routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (i.e. SINGULARITY, DISCONTINUITY within the interval), it should be supplied to the routine as an element of the vector points. If necessary an appropriate special-purpose integrator must be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs At some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER.GT.0. = 6 The input is invalid because NPTS2.LT.2 or Break points are specified outside the integration range or (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or LIMIT.LT.NPTS2. RESULT, ABSERR, NEVAL, LAST, RLIST(1), and ELIST(1) are set to zero. ALIST(1) and BLIST(1) are set to A and B respectively. ALIST - Double precision Vector of dimension at least LIMIT, the first LAST elements of which are the left end points of the subintervals in the partition of the given integration range (A,B) BLIST - Double precision Vector of dimension at least LIMIT, the first LAST elements of which are the right end points of the subintervals in the partition of the given integration range (A,B) RLIST - Double precision Vector of dimension at least LIMIT, the first LAST elements of which are the integral approximations on the subintervals ELIST - Double precision Vector of dimension at least LIMIT, the first LAST elements of which are the moduli of the absolute error estimates on the subintervals PTS - Double precision Vector of dimension at least NPTS2, containing the integration limits and the break points of the interval in ascending sequence. LEVEL - Integer Vector of dimension at least LIMIT, containing the subdivision levels of the subinterval, i.e. if (AA,BB) is a subinterval of (P1,P2) where P1 as well as P2 is a user-provided break point or integration limit, then (AA,BB) has level L if ABS(BB-AA) = ABS(P2-P1)*2**(-L). NDIN - Integer Vector of dimension at least NPTS2, after first integration over the intervals (PTS(I)),PTS(I+1), I = 0,1, ..., NPTS2-2, the error estimates over some of the intervals may have been increased artificially, in order to put their subdivision forward. If this happens for the subinterval numbered K, NDIN(K) is put to 1, otherwise NDIN(K) = 0. IORD - Integer Vector of dimension at least LIMIT, the first K elements of which are pointers to the error estimates over the subintervals, such that ELIST(IORD(1)), ..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST If LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise LAST - Integer Number of subintervals actually produced in the subdivisions process

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

- Canonical provider: `main-src/src/dqagpe.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqagpe.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqagpe.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqagpe.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
