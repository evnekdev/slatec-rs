# CSROT

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Apply a plane Givens rotation.

## Description

CSROT applies the complex Givens rotation (X) ( C S)(X) (Y) = (-S C)(Y) N times where for I = 0,...,N-1 X = CX(LX+I*INCX) Y = CY(LY+I*INCY), where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and LY is defined in a similar way using INCY. Argument Description

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CSROT](https://www.netlib.org/slatec/lin/csrot.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (integer) number of elements in each vector. |
| 2 | `CX` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | (complex array) beginning of one vector. |
| 3 | `INCX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (integer) memory spacing of successive elements of vector CX. |
| 4 | `CY` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | (complex array) beginning of the other vector. |
| 5 | `INCY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (integer) memory spacing of successive elements of vector CY. |
| 6 | `C` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (real) cosine term of the rotation. |
| 7 | `S` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (real) sine term of the rotation. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::csrot`. Native symbol: `csrot_`. Declaration feature: `blas-level1`. Provider feature: `blas-level1`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::csrot`
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
