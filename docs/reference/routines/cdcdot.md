# CDCDOT

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the inner product of two vectors with extended precision accumulation.

## Description

B L A S Subprogram Description of Parameters

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1A4`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/cdcdot.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cdcdot.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CDCDOT](https://www.netlib.org/slatec/lin/cdcdot.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of elements in input vector(s). |
| 2 | `CB` | `input` | `scalar` | `COMPLEX` | `*mut crate::Complex32` | scalar | complex scalar to be added to inner product. |
| 3 | `CX` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | complex vector with N elements. |
| 4 | `INCX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | storage spacing between elements of CX. |
| 5 | `CY` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | complex vector with N elements. |
| 6 | `INCY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | storage spacing between elements of CY. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:complex32(mut_i32,mut_complex32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::cdcdot`. Native symbol: `cdcdot_`. Declaration feature: `blas-complex`. Provider feature: `blas-complex`. ABI fingerprint: `function:complex32(mut_i32,mut_complex32,mut_complex32_array_rank1,mut_i32,mut_complex32_array_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::blas::level1::cdcdot`
- Public declaration feature: `blas-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
