# QMOMO

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

This routine computes modified Chebyshev moments. The K-th modified Chebyshev moment is defined as the integral over (-1,1) of W(X)*T(K,X), where T(K,X) is the Chebyshev polynomial of degree K.

## Description

MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE REAL VERSION PARAMETERS ALFA - Real Parameter in the weight function W(X), ALFA.GT.(-1) BETA - Real Parameter in the weight function W(X), BETA.GT.(-1) RI - Real Vector of dimension 25 RI(K) is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1, ..., 25. RJ - Real Vector of dimension 25 RJ(K) is the integral over (-1,1) of (1-X)**BETA*T(K-1,X), K = 1, ..., 25. RG - Real Vector of dimension 25 RG(K) is the integral over (-1,1) of (1+X)**ALFA*LOG((1+X)/2)*T(K-1,X), K = 1, ..., 25. RH - Real Vector of dimension 25 RH(K) is the integral over (-1,1) of (1-X)**BETA*LOG((1-X)/2)*T(K-1,X), K = 1, ..., 25. INTEGR - Integer Input parameter indicating the modified Moments to be computed INTEGR = 1 compute RI, RJ = 2 compute RI, RJ, RG = 3 compute RI, RJ, RH = 4 compute RI, RJ, RG, RH

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
- GAMS classifications: `H2A2A1`
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

- Canonical provider: `main-src/src/qmomo.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qmomo.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qmomo.f) — `verified_cached`
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
| `ALFA` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE REAL VERSION PARAMETERS ALFA - Real Parameter in the weight function W(X), ALFA.GT.(-1) BETA - Real Parameter in the weight function W(X), BETA.GT.(-1) RI - Real Vector of dimension 25 RI(K) is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1, ..., 25. | MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE REAL VERSION PARAMETERS ALFA - Real Parameter in the weight function W(X), ALFA.GT.(-1) BETA - Real Parameter in the weight function W(X), BETA.GT.(-1) RI - Real Vector of dimension 25 RI(K) is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1, ..., 25. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BETA` | input | `REAL` (`explicit`) | `*mut f32` | scalar | MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE REAL VERSION PARAMETERS ALFA - Real Parameter in the weight function W(X), ALFA.GT.(-1) BETA - Real Parameter in the weight function W(X), BETA.GT.(-1) RI - Real Vector of dimension 25 RI(K) is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1, ..., 25. | MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE REAL VERSION PARAMETERS ALFA - Real Parameter in the weight function W(X), ALFA.GT.(-1) BETA - Real Parameter in the weight function W(X), BETA.GT.(-1) RI - Real Vector of dimension 25 RI(K) is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1, ..., 25. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RI` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (25) | MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE REAL VERSION PARAMETERS ALFA - Real Parameter in the weight function W(X), ALFA.GT.(-1) BETA - Real Parameter in the weight function W(X), BETA.GT.(-1) RI - Real Vector of dimension 25 RI(K) is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1, ..., 25. | MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE REAL VERSION PARAMETERS ALFA - Real Parameter in the weight function W(X), ALFA.GT.(-1) BETA - Real Parameter in the weight function W(X), BETA.GT.(-1) RI - Real Vector of dimension 25 RI(K) is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1, ..., 25. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RJ` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (25) | RJ - Real Vector of dimension 25 RJ(K) is the integral over (-1,1) of (1-X)**BETA*T(K-1,X), K = 1, ..., 25. | RJ - Real Vector of dimension 25 RJ(K) is the integral over (-1,1) of (1-X)**BETA*T(K-1,X), K = 1, ..., 25. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RG` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (25) | RG - Real Vector of dimension 25 RG(K) is the integral over (-1,1) of (1+X)**ALFA*LOG((1+X)/2)*T(K-1,X), K = 1, ..., 25. | RG - Real Vector of dimension 25 RG(K) is the integral over (-1,1) of (1+X)**ALFA*LOG((1+X)/2)*T(K-1,X), K = 1, ..., 25. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RH` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (25) | RH - Real Vector of dimension 25 RH(K) is the integral over (-1,1) of (1-X)**BETA*LOG((1-X)/2)*T(K-1,X), K = 1, ..., 25. | RH - Real Vector of dimension 25 RH(K) is the integral over (-1,1) of (1-X)**BETA*LOG((1-X)/2)*T(K-1,X), K = 1, ..., 25. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INTEGR` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | INTEGR - Integer Input parameter indicating the modified Moments to be computed INTEGR = 1 compute RI, RJ = 2 compute RI, RJ, RG = 3 compute RI, RJ, RH = 4 compute RI, RJ, RG, RH | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::quadrature::qmomo`. Native symbol: `qmomo_`. Feature: `quadrature`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::qmomo`
- Compatibility aliases: `slatec_sys::quadrature::numerical::qmomo`
- Public declaration feature: `quadrature`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
