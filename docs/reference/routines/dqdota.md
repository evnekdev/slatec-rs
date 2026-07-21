# DQDOTA

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the inner product of two vectors with extended precision accumulation and result.

## Description

B L A S Subprogram Description of Parameters

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1A4`
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

- Canonical provider: `main-src/src/dqdota.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqdota.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqdota.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dqdota.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DQDOTA](https://www.netlib.org/slatec/src/dqdota.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of elements in input vector(S). |
| 2 | `DB` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | double precision scalar to be added to inner product. |
| 3 | `QC` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (30) | extended precision scalar to be added to inner product extended precision result D. P. dot product with extended precision accumulation (and result) QC and DQDOTA are set = DB + QC + sum for I = 0 to N-1 of DX(LX+I*INCX) * DY(LY+I*INCY), where QC is an extended precision result previously computed by DQDOTI or DQDOTA and LX = 1 if INCX. GE. 0, else LX = (-INCX)*N, and LY is defined in a similar way using INCY. The MP package by Richard P. |
| 4 | `DX` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | double precision vector with N elements. |
| 5 | `INCX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | storage spacing between elements of DX. |
| 6 | `DY` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | double precision vector with N elements. |
| 7 | `INCY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | storage spacing between elements of DY. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `unavailable`.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::dqdota`. Native symbol: `dqdota_`. Declaration feature: `blas-level1`. Provider feature: `blas-level1`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::dqdota`
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
