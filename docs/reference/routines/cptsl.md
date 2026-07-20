# CPTSL

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a positive definite tridiagonal linear system.

## Description

CPTSL given a positive definite tridiagonal matrix and a right hand side will find the solution. On Entry N INTEGER is the order of the tridiagonal matrix. D COMPLEX(N) is the diagonal of the tridiagonal matrix. On output D is destroyed. E COMPLEX(N) is the offdiagonal of the tridiagonal matrix. E(1) through E(N-1) should contain the offdiagonal. B COMPLEX(N) is the right hand side vector. On Return B contains the solution.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2D2A`
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

- Canonical provider: `lin/cptsl.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cptsl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On Entry N INTEGER is the order of the tridiagonal matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | input | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | D COMPLEX(N) is the diagonal of the tridiagonal matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | input | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | E COMPLEX(N) is the offdiagonal of the tridiagonal matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input/output | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | B COMPLEX(N) is the right hand side vector. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::banded::complex::cptsl`. Native symbol: `cptsl_`. Feature: `linear-algebra-complex`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_complex32_array_rank1,mut_complex32_array_rank1,mut_complex32_array_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cptsl`
- Compatibility aliases: `slatec_sys::linear_algebra::dense::complex::cptsl`
- Public declaration feature: `linear-algebra-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
