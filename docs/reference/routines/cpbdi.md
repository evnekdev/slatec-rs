# CPBDI

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the determinant of a complex Hermitian positive definite band matrix using the factors computed by CPBCO or CPBFA.

## Description

CPBDI computes the determinant of a complex Hermitian positive definite band matrix using the factors computed by CPBCO or CPBFA. If the inverse is needed, use CPBSL N times. On Entry ABD COMPLEX(LDA, N) the output from CPBCO or CPBFA. LDA INTEGER the leading dimension of the array ABD . N INTEGER the order of the matrix A . M INTEGER the number of diagonals above the main diagonal. On Return DET REAL(2) determinant of original matrix in the form determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. DET(1) .LT. 10.0 or DET(1) .EQ. 0.0 .

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
- GAMS classifications: `D3D2`
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

- Canonical provider: `lin/cpbdi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cpbdi.f) — `verified_cached`
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
| `ABD` | input | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 2; dimensions (LDA, *) | On Entry ABD COMPLEX(LDA, N) the output from CPBCO or CPBFA. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LDA` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On Entry ABD COMPLEX(LDA, N) the output from CPBCO or CPBFA. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If the inverse is needed, use CPBSL N times. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | M INTEGER the number of diagonals above the main diagonal. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `DET` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (2) | On Return DET REAL(2) determinant of original matrix in the form determinant = DET(1) * 10.0**DET(2) with 1.0 .LE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::banded::complex::cpbdi`. Native symbol: `cpbdi_`. Feature: `linear-algebra-complex`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_i32,mut_f32_array_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::banded::complex::cpbdi`
- Compatibility aliases: `slatec_sys::linear_algebra::dense::complex::cpbdi`
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
