# QAGS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given Definite integral I = Integral of F over (A,B), Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of a definite integral Standard fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand Function F(X). The actual name for F needs to be Declared E X T E R N A L in the driver program. A - Real Lower limit of integration B - Real Upper limit of integration EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account. However, if this yields no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (e.g. SINGULARITY, DISCONTINUITY within the interval) one will probably gain from splitting up the interval at this point and calling the integrator on the subranges. If possible, an appropriate special-purpose integrator should be used, which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. The error may be under-estimated. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 4 The algorithm does not converge. Roundoff error is detected in the Extrapolation table. It is presumed that the requested tolerance cannot be achieved, and that the returned result is the best which can be obtained. = 5 The integral is probably divergent, or slowly convergent. It must be noted that divergence can occur with any other value of IER. = 6 The input is invalid, because (EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28) OR LIMIT.LT.1 OR LENW.LT.LIMIT*4. RESULT, ABSERR, NEVAL, LAST are set to zero. Except when LIMIT or LENW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) TO B. DIMENSIONING PARAMETERS LIMIT - Integer Dimensioning parameter for IWORK LIMIT determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.1. IF LIMIT.LT.1, the routine will end with IER = 6. LENW - Integer Dimensioning parameter for WORK LENW must be at least LIMIT*4. If LENW.LT.LIMIT*4, the routine will end with IER = 6. LAST - Integer On return, LAST equals the number of subintervals produced in the subdivision process, determines the number of significant elements actually in the WORK Arrays. WORK ARRAYS IWORK - Integer Vector of dimension at least LIMIT, the first K elements of which contain pointers to the error estimates over the subintervals such that WORK(LIMIT*3+IWORK(1)),... , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence, with K = LAST IF LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise WORK - Real Vector of dimension at least LENW on return WORK(1), ..., WORK(LAST) contain the left end-points of the subintervals in the partition of (A,B), WORK(LIMIT+1), ..., WORK(LIMIT+LAST) contain the right end-points, WORK(LIMIT*2+1), ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, WORK(LIMIT*3+1), ..., WORK(LIMIT*3+LAST) contain the error estimates.

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
- Safe Rust paths: `slatec::quadrature::integrate_singular_f32`

## Providers

- Canonical provider: `main-src/src/qags.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qags.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qags.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qags.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::quadrature::qags`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `slatec::quadrature::integrate_singular_f32`
- Exclusion or deferment reason: `pre-existing declaration remains deferred until the R1 source-hash, argument-documentation, and ABI review gate passes`
<!-- raw-api-status:end -->
