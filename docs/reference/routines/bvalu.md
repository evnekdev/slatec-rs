# BVALU

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate the B-representation of a B-spline at X for the function value or any of its derivatives.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract BVALU is the BVALUE function of the reference. BVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV = 0 or any of its derivatives on IDERIV = 1,2,...,K-1. Right limiting values (right derivatives) are returned except at the right end point X=T(N+1) where left limiting values are computed. The spline is defined on T(K) .LE. X .LE. T(N+1). BVALU returns a fatal error message when X is outside of this interval. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. BVALU calls INTRV Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
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
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::interpolation::bspline::BSpline::derivative, slatec::interpolation::bspline::BSpline::evaluate, slatec::interpolation::bspline::BSpline::evaluate_into`

## Providers

- Canonical provider: `main-src/src/bvalu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bvalu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bvalu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bvalu.f) — `verified_cached`
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
| `T` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | BVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV = 0 or any of its derivatives on IDERIV = 1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | BVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV = 0 or any of its derivatives on IDERIV = 1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | BVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV = 0 or any of its derivatives on IDERIV = 1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | BVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV = 0 or any of its derivatives on IDERIV = 1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDERIV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | BVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV = 0 or any of its derivatives on IDERIV = 1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | BVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV = 0 or any of its derivatives on IDERIV = 1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INBV` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f32` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `function:f32(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`.

### ABI and safety

Canonical path: `slatec_sys::interpolation::bvalu`. Native symbol: `bvalu_`. Feature: `interpolation`. Provider status: `selected_provider_verified`. ABI fingerprint: `function:f32(mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::bvalu`
- Compatibility aliases: `slatec_sys::interpolation::numerical::bvalu`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::interpolation::bspline::BSpline::derivative`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
