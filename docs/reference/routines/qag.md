# QAG

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = integral of F over (A,B), hopefully satisfying following claim for accuracy ABS(I-RESULT)LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of a definite integral Standard fortran subroutine Real version F - Real Function subprogram defining the integrand Function F(X). The actual name for F needs to be Declared E X T E R N A L in the driver program. A - Real Lower limit of integration B - Real Upper limit of integration EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. KEY - Integer Key for choice of local integration rule A GAUSS-KRONROD PAIR is used with 7 - 15 POINTS If KEY.LT.2, 10 - 21 POINTS If KEY = 2, 15 - 31 POINTS If KEY = 3, 20 - 41 POINTS If KEY = 4, 25 - 51 POINTS If KEY = 5, 30 - 61 POINTS If KEY.GT.5. ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, Which should EQUAL or EXCEED ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine The estimates for RESULT and ERROR are Less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 Maximum number of subdivisions allowed has been achieved. One can allow more subdivisions by increasing the value of LIMIT (and taking the according dimension adjustments into account). HOWEVER, If this yield no improvement it is advised to analyze the integrand in order to determine the integration difficulties. If the position of a local difficulty can be determined (I.E. SINGULARITY, DISCONTINUITY WITHIN THE INTERVAL) One will probably gain from splitting up the interval at this point and calling the INTEGRATOR on the SUBRANGES. If possible, AN APPROPRIATE SPECIAL-PURPOSE INTEGRATOR should be used which is designed for handling the type of difficulty involved. = 2 The occurrence of roundoff error is detected, which prevents the requested tolerance from being achieved. = 3 Extremely bad integrand behaviour occurs at some points of the integration interval. = 6 The input is invalid, because (EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) OR LIMIT.LT.1 OR LENW.LT.LIMIT*4. RESULT, ABSERR, NEVAL, LAST are set to zero. EXCEPT when LENW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. DIMENSIONING PARAMETERS LIMIT - Integer Dimensioning parameter for IWORK Limit determines the maximum number of subintervals in the partition of the given integration interval (A,B), LIMIT.GE.1. If LIMIT.LT.1, the routine will end with IER = 6. LENW - Integer Dimensioning parameter for work LENW must be at least LIMIT*4. IF LENW.LT.LIMIT*4, the routine will end with IER = 6. LAST - Integer On return, LAST equals the number of subintervals produced in the subdivision process, which determines the number of significant elements actually in the WORK ARRAYS. WORK ARRAYS IWORK - Integer Vector of dimension at least limit, the first K elements of which contain pointers to the error estimates over the subintervals, such that WORK(LIMIT*3+IWORK(1)),... , WORK(LIMIT*3+IWORK(K)) form a decreasing sequence with K = LAST If LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise WORK - Real Vector of dimension at least LENW on return WORK(1), ..., WORK(LAST) contain the left end points of the subintervals in the partition of (A,B), WORK(LIMIT+1), ..., WORK(LIMIT+LAST) contain the right end points, WORK(LIMIT*2+1), ..., WORK(LIMIT*2+LAST) contain the integral approximations over the subintervals, WORK(LIMIT*3+1), ..., WORK(LIMIT*3+LAST) contain the error estimates.

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
- Safe Rust paths: `slatec::quadrature::integrate_f32`

## Providers

- Canonical provider: `main-src/src/qag.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qag.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qag.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qag.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `F` | callback | `REAL` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Computation of a definite integral Standard fortran subroutine Real version F - Real Function subprogram defining the integrand Function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | output | `REAL` (`explicit`) | `*mut f32` | scalar | Computation of a definite integral Standard fortran subroutine Real version F - Real Function subprogram defining the integrand Function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | output | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Lower limit of integration B - Real Upper limit of integration EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSABS` | output | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Lower limit of integration B - Real Upper limit of integration EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSREL` | output | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Lower limit of integration B - Real Upper limit of integration EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KEY` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | KEY - Integer Key for choice of local integration rule A GAUSS-KRONROD PAIR is used with 7 - 15 POINTS If KEY.LT.2, 10 - 21 POINTS If KEY = 2, 15 - 31 POINTS If KEY = 3, 20 - 41 POINTS If KEY = 4, 25 - 51 POINTS If KEY = 5, 30 - 61 POINTS If KEY.GT.5. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | output | `REAL` (`explicit`) | `*mut f32` | scalar | ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, Which should EQUAL or EXCEED ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | output | `REAL` (`explicit`) | `*mut f32` | scalar | ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, Which should EQUAL or EXCEED ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEVAL` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ON RETURN RESULT - Real Approximation to the integral ABSERR - Real Estimate of the modulus of the absolute error, Which should EQUAL or EXCEED ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IER` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - Real Lower limit of integration B - Real Upper limit of integration EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIMIT` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - Real Lower limit of integration B - Real Upper limit of integration EPSABS - Real Absolute accuracy requested EPSREL - Real Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LENW` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | = 6 The input is invalid, because (EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28)) OR LIMIT.LT.1 OR LENW.LT.LIMIT*4. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LAST` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | RESULT, ABSERR, NEVAL, LAST are set to zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | EXCEPT when LENW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | EXCEPT when LENW is invalid, IWORK(1), WORK(LIMIT*2+1) and WORK(LIMIT*3+1) are set to zero, WORK(1) is set to A and WORK(LIMIT+1) to B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::qag`. Native symbol: `qag_`. Feature: `quadrature-basic`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::qag`
- Compatibility aliases: `none`
- Public declaration feature: `quadrature-basic`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
