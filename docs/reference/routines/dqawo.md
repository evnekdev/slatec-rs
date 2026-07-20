# DQAWO

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate an approximation to a given definite integral I= Integral of F(X)*W(X) over (A,B), where W(X) = COS(OMEGA*X) or W(X) = SIN(OMEGA*X), hopefully satisfying the following claim for accuracy ABS(I-RESULT).LE.MAX(EPSABS,EPSREL*ABS(I)).

## Description

Computation of oscillatory integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Double precision Lower limit of integration B - Double precision Upper limit of integration OMEGA - Double precision Parameter in the integrand weight function INTEGR - Integer Indicates which of the weight functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) If INTEGR.NE.1.AND.INTEGR.NE.2, the routine will

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
- Safe Rust paths: `slatec::quadrature::integrate_oscillatory`

## Providers

- Canonical provider: `main-src/src/dqawo.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqawo.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqawo.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqawo.f) — `verified_cached`
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
| `F` | callback | `DOUBLE PRECISION` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Computation of oscillatory integrals Standard fortran subroutine Double precision version PARAMETERS ON ENTRY F - Double precision Function subprogram defining the function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | The actual name for F needs to be declared E X T E R N A L in the driver program. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | A - Double precision Lower limit of integration B - Double precision Upper limit of integration OMEGA - Double precision Parameter in the integrand weight function INTEGR - Integer Indicates which of the weight functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) If INTEGR.NE.1.AND.INTEGR.NE.2, the routine will | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `OMEGA` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | A - Double precision Lower limit of integration B - Double precision Upper limit of integration OMEGA - Double precision Parameter in the integrand weight function INTEGR - Integer Indicates which of the weight functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) If INTEGR.NE.1.AND.INTEGR.NE.2, the routine will | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INTEGR` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | A - Double precision Lower limit of integration B - Double precision Upper limit of integration OMEGA - Double precision Parameter in the integrand weight function INTEGR - Integer Indicates which of the weight functions is used INTEGR = 1 W(X) = COS(OMEGA*X) INTEGR = 2 W(X) = SIN(OMEGA*X) If INTEGR.NE.1.AND.INTEGR.NE.2, the routine will | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSABS` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `EPSREL` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEVAL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IER` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LENIW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MAXP1` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LENW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LAST` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::dqawo`. Native symbol: `dqawo_`. Feature: `quadrature-oscillatory`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::quadrature::dqawo`
- Compatibility aliases: `none`
- Public declaration feature: `quadrature-oscillatory`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::quadrature::integrate_oscillatory`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
