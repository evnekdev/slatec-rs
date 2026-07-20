# QAWF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given Fourier integral I = Integral of F(X)*W(X) over (A,INFINITY) where W(X) = COS(OMEGA*X) or W(X) = SIN(OMEGA*X). Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.EPSABS.

## Description

Computation of Fourier integrals Standard fortran subroutine Real version PARAMETERS ON ENTRY F - Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Real Lower limit of integration OMEGA - Real Parameter in the integrand WEIGHT function INTEGR - Integer Indicates which of the WEIGHT functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) IF INTEGR.NE.1.AND.INTEGR.NE.2, the routine will end with IER = 6. EPSABS - Real Absolute accuracy requested, EPSABS.GT.0. If EPSABS.LE.0, the routine will end with IER = 6. ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES If OMEGA.NE.0 IER = 1 Maximum number of cycles allowed has been achieved, i.e. of subintervals (A+(K-1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), FOR K = 1, 2, ..., LST. One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Examine the array IWORK which contains the error flags on the cycles, in order to look for eventual local integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 4 The extrapolation table constructed for convergence acceleration of the series formed by the integral contributions over the cycles, does not converge to within the requested accuracy. As in the case of IER = 1, it is advised to examine the array IWORK which contains the error flags on the cycles. = 6 The input is invalid because (INTEGR.NE.1 AND INTEGR.NE.2) or EPSABS.LE.0 or LIMLST.LT.1 or LENIW.LT.(LIMLST+2) or MAXP1.LT.1 or LENW.LT.(LENIW*2+MAXP1*25). RESULT, ABSERR, NEVAL, LST are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and

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
- GAMS classifications: `H2A3A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_fourier_tail_f32`

## Providers

- Canonical provider: `main-src/src/qawf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qawf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qawf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qawf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `batch_d_public_driver`
- Canonical Rust path: `slatec_sys::quadrature::qawf`
- Current legacy Rust paths: `none`
- Public declaration feature: `quadrature-fourier`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_fourier_tail_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
