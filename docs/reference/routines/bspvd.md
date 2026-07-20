# BSPVD

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value and all derivatives of order less than NDERIV of all basis functions which do not vanish at X.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract BSPVD is the BSPLVD routine of the reference. BSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. ILEFT is input such that T(ILEFT) .LE. X .LT. T(ILEFT+1). A call to INTRV(T,N+1,X, ILO,ILEFT,MFLAG) will produce the proper ILEFT. The output of BSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. These basis functions have indices ILEFT-K+I, I=1,K, K .LE. ILEFT .LE. N. The nonzero part of the I-th basis function lies in (T(I),T(I+K)), I=1,N. If X=T(ILEFT+1) then VNIKX contains left limiting values (left derivatives) at T(ILEFT+1). In particular, ILEFT = N produces left limiting values at the right end point X=T(N+1). To obtain left limiting values at T(I), I=K+1,N+1, set X= next lower distinct knot, call INTRV to get ILEFT, set X=T(I), and then call BSPVD. Description of Arguments

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
- GAMS classifications: `E3`
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

- Canonical provider: `main-src/src/bspvd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bspvd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bspvd.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bspvd.f) — `verified_cached`
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
| `T` | input | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | ILEFT is input such that T(ILEFT) .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The output of BSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. | The output of BSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NDERIV` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | BSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. | BSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | input | `REAL` (`explicit`) | `*mut f32` | scalar | BSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. | BSPVD calculates the value and all derivatives of order less than NDERIV of all basis functions which do not (possibly) vanish at X. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ILEFT` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ILEFT is input such that T(ILEFT) .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDVNIK` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `VNIKX` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (LDVNIK, *) | The output of BSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. | The output of BSPVD is a matrix VNIKX(I,J) of dimension at least (K,NDERIV) whose columns contain the K nonzero basis functions and their NDERIV-1 right derivatives at X, I=1,K, J=1,NDERIV. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::interpolation::bspvd`. Native symbol: `bspvd_`. Feature: `interpolation`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::bspvd`
- Compatibility aliases: `slatec_sys::interpolation::numerical::bspvd`
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
