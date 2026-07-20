# POLCOF

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the coefficients of the polynomial fit (including Hermite polynomial fits) produced by a previous call to POLINT.

## Description

Written by Robert E. Huddleston, Sandia Laboratories, Livermore Abstract Subroutine POLCOF computes the coefficients of the polynomial fit (including Hermite polynomial fits ) produced by a previous call to POLINT. The coefficients of the polynomial, expanded about XX, are stored in the array D. The expansion is of the form P(Z) = D(1) + D(2)*(Z-XX) +D(3)*((Z-XX)**2) + ... + D(N)*((Z-XX)**(N-1)). Between the call to POLINT and the call to POLCOF the variable N and the arrays X and C must not be altered. ***** INPUT PARAMETERS XX - The point about which the Taylor expansion is to be made. N - **** * N, X, and C must remain unchanged between the X - * call to POLINT or the call to POLCOF. C - **** ***** OUTPUT PARAMETER D - The array of coefficients for the Taylor expansion as explained in the abstract ***** STORAGE PARAMETER WORK - This is an array to provide internal working storage. It must be dimensioned by at least 2*N in the calling program. **** Note - There are two methods for evaluating the fit produced by POLINT. You may call POLYVL to perform the task, or you may call POLCOF to obtain the coefficients of the Taylor expansion and then write your own evaluation scheme. Due to the inherent errors in the computations of the Taylor expansion from the Newton coefficients produced by POLINT, much more accuracy may be expected by calling POLYVL as opposed to writing your own scheme.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E1B`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/polcof.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/polcof.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/polcof.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Interpolation](../families/interpolation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `XX` | input | `REAL` (`implicit_rule`) | `*mut f32` | scalar | The coefficients of the polynomial, expanded about XX, are stored in the array D. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | + D(N)*((Z-XX)**(N-1)). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Between the call to POLINT and the call to POLCOF the variable N and the arrays X and C must not be altered. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Between the call to POLINT and the call to POLCOF the variable N and the arrays X and C must not be altered. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | The coefficients of the polynomial, expanded about XX, are stored in the array D. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | C - **** ***** OUTPUT PARAMETER D - The array of coefficients for the Taylor expansion as explained in the abstract ***** STORAGE PARAMETER WORK - This is an array to provide internal working storage. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::interpolation::polcof`. Native symbol: `polcof_`. Feature: `interpolation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::polcof`
- Compatibility aliases: `slatec_sys::interpolation::numerical::polcof`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
