# QC25C

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

To compute I = Integral of F*W over (A,B) with error estimate, where W(X) = 1/(X-C)

## Description

Integration rules for the computation of CAUCHY PRINCIPAL VALUE integrals Standard fortran subroutine Real version PARAMETERS F - Real Function subprogram defining the integrand function F(X). The actual name for F needs to be declared E X T E R N A L in the driver program. A - Real Left end point of the integration interval B - Real Right end point of the integration interval, B.GT.A C - Real Parameter in the WEIGHT function RESULT - Real Approximation to the integral result is computed by using a generalized Clenshaw-Curtis method if C lies within ten percent of the integration interval. In the other case the 15-point Kronrod rule obtained by optimal addition of abscissae to the 7-point Gauss rule, is applied. ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) KRUL - Integer Key which is decreased by 1 if the 15-point Gauss-Kronrod scheme has been used NEVAL - Integer Number of integrand evaluations

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
- GAMS classifications: `H2A2A2`
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

- Canonical provider: `main-src/src/qc25c.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qc25c.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qc25c.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/qc25c.f) — `verified_cached`
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
| `F` | callback | `REAL` (`explicit`) | `reviewed unsafe extern callback function pointer` | scalar | Integration rules for the computation of CAUCHY PRINCIPAL VALUE integrals Standard fortran subroutine Real version PARAMETERS F - Real Function subprogram defining the integrand function F(X). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | The actual name for F needs to be declared E X T E R N A L in the driver program. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Left end point of the integration interval B - Real Right end point of the integration interval, B.GT.A C - Real Parameter in the WEIGHT function RESULT - Real Approximation to the integral result is computed by using a generalized Clenshaw-Curtis method if C lies within ten percent of the integration interval. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Left end point of the integration interval B - Real Right end point of the integration interval, B.GT.A C - Real Parameter in the WEIGHT function RESULT - Real Approximation to the integral result is computed by using a generalized Clenshaw-Curtis method if C lies within ten percent of the integration interval. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RESULT` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | A - Real Left end point of the integration interval B - Real Right end point of the integration interval, B.GT.A C - Real Parameter in the WEIGHT function RESULT - Real Approximation to the integral result is computed by using a generalized Clenshaw-Curtis method if C lies within ten percent of the integration interval. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ABSERR` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) KRUL - Integer Key which is decreased by 1 if the 15-point Gauss-Kronrod scheme has been used NEVAL - Integer Number of integrand evaluations | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KRUL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) KRUL - Integer Key which is decreased by 1 if the 15-point Gauss-Kronrod scheme has been used NEVAL - Integer Number of integrand evaluations | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEVAL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ABSERR - Real Estimate of the modulus of the absolute error, which should equal or exceed ABS(I-RESULT) KRUL - Integer Key which is decreased by 1 if the 15-point Gauss-Kronrod scheme has been used NEVAL - Integer Number of integrand evaluations | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Callback contract

Procedure arguments use the exact reviewed `unsafe extern "C"` callback type on the canonical declaration. Callback pointers are required, must remain valid for the complete native call, must satisfy the documented mutation contract, and must never unwind into Fortran.

### ABI and safety

Canonical path: `slatec_sys::quadrature::callbacks::qc25c`. Native symbol: `qc25c_`. Feature: `quadrature-callbacks`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(fn:f32(ref_f32),mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::callbacks::qc25c`
- Compatibility aliases: `none`
- Public declaration feature: `quadrature-callbacks`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
