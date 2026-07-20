# DQAWF

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

The routine calculates an approximation result to a given Fourier integral I=Integral of F(X)*W(X) over (A,INFINITY) where W(X) = COS(OMEGA*X) or W(X) = SIN(OMEGA*X). Hopefully satisfying following claim for accuracy ABS(I-RESULT).LE.EPSABS.

## Description

Computation of Fourier integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration OMEGA - Double precision Parameter in the integrand WEIGHT function INTEGR - Integer Indicates which of the WEIGHT functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) IF INTEGR.NE.1.AND.INTEGR.NE.2, the routine will end with IER = 6. EPSABS - Double precision Absolute accuracy requested, EPSABS.GT.0. If EPSABS.LE.0, the routine will end with IER = 6. ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. It is assumed that the requested accuracy has been achieved. IER.GT.0 Abnormal termination of the routine. The estimates for integral and error are less reliable. It is assumed that the requested accuracy has not been achieved. ERROR MESSAGES If OMEGA.NE.0 IER = 1 Maximum number of cycles allowed has been achieved, i.e. of subintervals (A+(K-1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), FOR K = 1, 2, ..., LST. One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Examine the array IWORK which contains the error flags on the cycles, in order to look for eventual local integration difficulties. If the position of a local difficulty can be determined (e.g. singularity, discontinuity within the interval) one will probably gain from splitting up the interval at this point and calling appropriate integrators on the subranges. = 4 The extrapolation table constructed for convergence acceleration of the series formed by the integral contributions over the cycles, does not converge to within the requested accuracy. As in the case of IER = 1, it is advised to examine the array IWORK which contains the error flags on the cycles. = 6 The input is invalid because (INTEGR.NE.1 AND INTEGR.NE.2) or EPSABS.LE.0 or LIMLST.LT.1 or LENIW.LT.(LIMLST+2) or MAXP1.LT.1 or LENW.LT.(LENIW*2+MAXP1*25). RESULT, ABSERR, NEVAL, LST are set to zero. = 7 Bad integrand behaviour occurs within one or more of the cycles. Location and

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
- Safe Rust paths: `slatec::quadrature::integrate_fourier_tail`

## Providers

- Canonical provider: `main-src/src/dqawf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqawf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqawf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqawf.f) — `verified_cached`
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
| `F` | callback | `DOUBLE PRECISION` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Computation of Fourier integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the integrand function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input/output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The actual name for F needs to be declared E X T E R N A L in the driver program. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `OMEGA` | input/output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | A - Double precision Lower limit of integration OMEGA - Double precision Parameter in the integrand WEIGHT function INTEGR - Integer Indicates which of the WEIGHT functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) IF INTEGR.NE.1.AND.INTEGR.NE.2, the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INTEGR` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - Double precision Lower limit of integration OMEGA - Double precision Parameter in the integrand WEIGHT function INTEGR - Integer Indicates which of the WEIGHT functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) IF INTEGR.NE.1.AND.INTEGR.NE.2, the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSABS` | input/output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | EPSABS - Double precision Absolute accuracy requested, EPSABS.GT.0. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEVAL` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ON RETURN RESULT - Double precision Approximation to the integral ABSERR - Double precision Estimate of the modulus of the absolute error, Which should equal or exceed ABS(I-RESULT) NEVAL - Integer Number of integrand evaluations IER - Integer IER = 0 Normal and reliable termination of the routine. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IER` | input/output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - Double precision Lower limit of integration OMEGA - Double precision Parameter in the integrand WEIGHT function INTEGR - Integer Indicates which of the WEIGHT functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) IF INTEGR.NE.1.AND.INTEGR.NE.2, the routine will end with IER = 6. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIMLST` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). | One can allow more cycles by increasing the value of LIMLST (and taking the according dimension adjustments into account). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LST` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | of subintervals (A+(K-1)C,A+KC) where C = (2*INT(ABS(OMEGA))+1)*PI/ABS(OMEGA), FOR K = 1, 2, ..., LST. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LENIW` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | = 6 The input is invalid because (INTEGR.NE.1 AND INTEGR.NE.2) or EPSABS.LE.0 or LIMLST.LT.1 or LENIW.LT.(LIMLST+2) or MAXP1.LT.1 or LENW.LT.(LENIW*2+MAXP1*25). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MAXP1` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | = 6 The input is invalid because (INTEGR.NE.1 AND INTEGR.NE.2) or EPSABS.LE.0 or LIMLST.LT.1 or LENIW.LT.(LIMLST+2) or MAXP1.LT.1 or LENW.LT.(LENIW*2+MAXP1*25). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LENW` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | = 6 The input is invalid because (INTEGR.NE.1 AND INTEGR.NE.2) or EPSABS.LE.0 or LIMLST.LT.1 or LENIW.LT.(LIMLST+2) or MAXP1.LT.1 or LENW.LT.(LENIW*2+MAXP1*25). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Examine the array IWORK which contains the error flags on the cycles, in order to look for eventual local integration difficulties. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::dqawf`. Native symbol: `dqawf_`. Feature: `quadrature-fourier`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqawf`
- Compatibility aliases: `none`
- Public declaration feature: `quadrature-fourier`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_fourier_tail`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
