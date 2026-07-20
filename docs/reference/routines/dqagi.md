# DQAGI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given INTEGRAL I = Integral of F over (BOUND,+INFINITY) OR I = Integral of F over (-INFINITY,BOUND) OR I = Integral of F over (-INFINITY,+INFINITY) Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Integration over infinite intervals Standard fortran subroutine PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. BOUND - Double precision Finite bound of integration range (has no meaning if interval is doubly-infinite) INF - Integer indicating the kind of integration range involved INF = 1 corresponds to (BOUND,+INFINITY), INF = -1 to (-INFINITY,BOUND), INF = 2 to (-INFINITY,+INFINITY). EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), the routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. - IER.GT.0 abnormal termination of the routine. The estimates for result and error are less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the extrapolation table. It is assumed that the requested tolerance cannot be achieved, and that the returned RESULT is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because (EPSABS.LE.0 and EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) or LIMIT.LT.1 or LENIW.LT.LIMIT*4. RESULT, ABSERR, NEVAL, LAST are set to zero. Except when LIMIT or LENIW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to ZERO, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS LIMIT - Integer Dimensioning parameter for IWORK LIMIT determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.1. If LIMIT.LT.1, the routine will end with IER = 6. LENW - Integer Dimensioning parameter for WORK LENW must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end with IER = 6. LAST - Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. WORK ARRAYS IWORK - Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),... , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise WORK - Double precision Vector of dimension at least LENW on return WORK(1), ..., WORK(LAST) contain the left

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
- GAMS classifications: `H2A3A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_infinite`

## Providers

- Canonical provider: `main-src/src/dqagi.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqagi.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqagi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqagi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `batch_d_public_driver`
- Canonical Rust path: `slatec_sys::quadrature::dqagi`
- Current legacy Rust paths: `none`
- Public declaration feature: `quadrature-basic`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_infinite`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
