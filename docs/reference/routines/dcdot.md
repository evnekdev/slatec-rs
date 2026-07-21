# DCDOT

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the inner product of two vectors with extended precision accumulation and result.

## Description

Compute the dot product of 2 complex vectors, CX and CY, e.g. CX DOT CY, or, CXconjugate DOT CY. The real and imaginary parts of CX and CY are converted to double precision, the dot product accumulation is done in double precision and the output is given as 2 double precision numbers, corresponding to the real and imaginary part of the result.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1A4`
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

- Canonical provider: `lin/dcdot.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dcdot.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DCDOT](https://www.netlib.org/slatec/lin/dcdot.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Number of complex components of CX and CY. |
| 2 | `FM` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | =+1. 0 compute CX DOT CY. =-1. 0 compute CXconjugate DOT CY. |
| 3 | `CX` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Complex arrays of length N. |
| 4 | `INCX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Integer) Spacing of elements of CX to use. |
| 5 | `CY` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Complex arrays of length N. |
| 6 | `INCY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (Integer) Spacing of elements of CY to use. |
| 7 | `DCR` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | (Double Precision) Real part of dot product. |
| 8 | `DCI` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | (Double Precision) Imaginary part of dot product. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::dcdot`. Native symbol: `dcdot_`. Declaration feature: `blas-level1`. Provider feature: `blas-level1`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::dcdot`
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
