# DQNG

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given definite integral I = integral of F over (A,B), hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

NON-ADAPTIVE INTEGRATION STANDARD FORTRAN SUBROUTINE DOUBLE PRECISION VERSION F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral I Result is obtained by applying the 21-POINT GAUSS-KRONROD RULE (RES21) obtained by optimal addition of abscissae to the 10-POINT GAUSS RULE (RES10), or by applying the 43-POINT RULE (RES43) obtained by optimal addition of abscissae to the 21-POINT GAUSS-KRONROD RULE, or by applying the 87-POINT RULE (RES87) obtained by optimal addition of abscissae to the 43-POINT RULE. ABSERR - Double precision Estimate of the modulus of the absolute error, which should EQUAL or EXCEED ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - IER = 0 normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES IER = 1 The maximum number of steps has been executed. The integral is probably too difficult to be calculated by DQNG. = 6 The input is invalid, because EPSABS.LE.0 AND EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28). RESULT, ABSERR and NEVAL are set to zero.

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
- GAMS classifications: `H2A1A1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::quadrature::integrate_non_adaptive`

## Providers

- Canonical provider: `main-src/src/dqng.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqng.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqng.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqng.f) — `verified_cached`
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
| `F` | callback | `DOUBLE PRECISION` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | NON-ADAPTIVE INTEGRATION STANDARD FORTRAN SUBROUTINE DOUBLE PRECISION VERSION F - Double precision Function subprogram defining the integrand function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The actual name for F needs to be declared E X T E R N A L in the driver program. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | A - Double precision Lower limit of integration B - Double precision Upper limit of integration EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSABS` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | A - Double precision Lower limit of integration B - Double precision Upper limit of integration EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSREL` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | A - Double precision Lower limit of integration B - Double precision Upper limit of integration EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | ON RETURN RESULT - Double precision Approximation to the integral I Result is obtained by applying the 21-POINT GAUSS-KRONROD RULE (RES21) obtained by optimal addition of abscissae to the 10-POINT GAUSS RULE (RES10), or by applying the 43-POINT RULE (RES43) obtained by optimal addition of abscissae to the 21-POINT GAUSS-KRONROD RULE, or by applying the 87-POINT RULE (RES87) obtained by optimal addition of abscissae to the 43-POINT RULE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | ABSERR - Double precision Estimate of the modulus of the absolute error, which should EQUAL or EXCEED ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - IER = 0 normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEVAL` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ABSERR - Double precision Estimate of the modulus of the absolute error, which should EQUAL or EXCEED ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - IER = 0 normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IER` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - Double precision Lower limit of integration B - Double precision Upper limit of integration EPSABS - Double precision Absolute accuracy requested EPSREL - Double precision Relative accuracy requested If EPSABS.LE.0 And EPSREL.LT.MAX(50*REL.MACH.ACC.,0.5D-28), The routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::dqng`. Native symbol: `dqng_`. Feature: `quadrature-nonadaptive`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqng`
- Compatibility aliases: `none`
- Public declaration feature: `quadrature-nonadaptive`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_non_adaptive`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
