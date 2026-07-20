# SINTRP

[Family: ODE solvers](../families/ode-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Approximate the solution at XOUT by evaluating the polynomial computed in STEPS at XOUT. Must be used in conjunction with STEPS.

## Description

The methods in subroutine STEPS approximate the solution near X by a polynomial. Subroutine SINTRP approximates the solution at XOUT by evaluating the polynomial there. Information defining this polynomial is passed from STEPS so SINTRP cannot be used alone. Subroutine STEPS is completely explained and documented in the text, "Computer Solution of Ordinary Differential Equations, the Initial Value Problem" by L. F. Shampine and M. K. Gordon. Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. The remaining parameters are defined in STEPS and passed to SINTRP from that subroutine Output from SINTRP -- YOUT(*) -- solution at XOUT YPOUT(*) -- derivative of solution at XOUT The remaining parameters are returned unaltered from their input values. Integration with STEPS may be continued.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- GAMS classifications: `I1A1B`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/sintrp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sintrp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sintrp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [ODE solvers](../families/ode-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `X` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | The methods in subroutine STEPS approximate the solution near X by a polynomial. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `XOUT` | output | `REAL` (`implicit_rule`) | `*mut f32` | scalar | Subroutine SINTRP approximates the solution at XOUT by evaluating the polynomial there. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YOUT` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `YPOUT` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NEQN` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KOLD` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `PHI` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (NEQN, 16) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IVC` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IV` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (10) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KGI` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `GI` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (11) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALPHA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (12) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `OG` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (13) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `OW` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (12) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `OX` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `OY` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. | Input to SINTRP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::ode::sintrp`. Native symbol: `sintrp_`. Feature: `ode`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::ode::sintrp`
- Compatibility aliases: `slatec_sys::ode::numerical::sintrp`
- Public declaration feature: `ode`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
