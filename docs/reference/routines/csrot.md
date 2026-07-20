# CSROT

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Apply a plane Givens rotation.

## Description

CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY. Argument Description N (integer) number of elements in each vector CX (complex array) beginning of one vector INCX (integer) memory spacing of successive elements of vector CX CY (complex array) beginning of the other vector INCY (integer) memory spacing of successive elements of vector CY C (real) cosine term of the rotation S (real) sine term of the rotation.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1B10`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/csrot.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/csrot.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Linear algebra kernels](../families/linear-algebra-kernels.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CX` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INCX` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CY` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INCY` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `S` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::blas::level1::csrot`. Native symbol: `csrot_`. Feature: `blas-level1`. Provider status: `selected_provider_verified`. ABI fingerprint: `unavailable`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::csrot`
- Compatibility aliases: `slatec_sys::families::blas_level1::csrot`
- Public declaration feature: `blas-level1`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
