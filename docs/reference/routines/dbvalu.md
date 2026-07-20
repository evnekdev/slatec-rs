# DBVALU

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate the B-representation of a B-spline at X for the function value or any of its derivatives.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBVALU is the BVALUE function of the reference. DBVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV=0 or any of its derivatives on IDERIV=1,2,...,K-1. Right limiting values (right derivatives) are returned except at the right end point X=T(N+1) where left limiting values are computed. The spline is defined on T(K) .LE. X .LE. T(N+1). DBVALU returns a fatal error message when X is outside of this interval. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. DBVALU calls DINTRV Description of Arguments Input T,A,X are double precision T - knot vector of length N+K A - B-spline coefficient vector of length N N - number of B-spline coefficients N = sum of knot multiplicities-K K - order of the B-spline, K .GE. 1 IDERIV - order of the derivative, 0 .LE. IDERIV .LE. K-1 IDERIV = 0 returns the B-spline value X - argument, T(K) .LE. X .LE. T(N+1) INBV - an initialization parameter which must be set to 1 the first time DBVALU is called. Output WORK,DBVALU are double precision INBV - INBV contains information for efficient processing after the initial call and INBV must not be changed by the user. Distinct splines require distinct INBV parameters. WORK - work vector of length 3*K. DBVALU - value of the IDERIV-th derivative at X Error Conditions An improper input is a fatal error

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
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

- Canonical provider: `main-src/src/dbvalu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbvalu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbvalu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbvalu.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Interpolation](../families/interpolation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `T` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | DBVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV=0 or any of its derivatives on IDERIV=1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Amos Abstract **** a double precision routine **** DBVALU is the BVALUE function of the reference. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DBVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV=0 or any of its derivatives on IDERIV=1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `K` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DBVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV=0 or any of its derivatives on IDERIV=1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IDERIV` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | DBVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV=0 or any of its derivatives on IDERIV=1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | input | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DBVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV=0 or any of its derivatives on IDERIV=1,2,...,K-1. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INBV` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | T(N+1) INBV - an initialization parameter which must be set to 1 the first time DBVALU is called. | T(N+1) INBV - an initialization parameter which must be set to 1 the first time DBVALU is called. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Output WORK,DBVALU are double precision INBV - INBV contains information for efficient processing after the initial call and INBV must not be changed by the user. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### Return value

The Fortran function returns `*mut f64` through the compiler-validated ABI recorded by the authoritative declaration fingerprint `function:f64(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1)`.

### ABI and safety

Canonical path: `slatec_sys::interpolation::dbvalu`. Native symbol: `dbvalu_`. Feature: `interpolation`. Provider status: `selected_provider_verified`. ABI fingerprint: `function:f64(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dbvalu`
- Compatibility aliases: `slatec_sys::interpolation::numerical::dbvalu`
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
