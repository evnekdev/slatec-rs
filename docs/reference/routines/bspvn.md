# BSPVN

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of all (possibly) nonzero basis functions at X.

## Description

Written by Carl de Boor and modified by D. E. Amos BSPVN is the BSPLVN routine of the reference. BSPVN calculates the value of all (possibly) nonzero basis functions at X of order MAX(JHIGH,(J+1)*(INDEX-1)), where

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

- Canonical provider: `main-src/src/bspvn.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bspvn.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bspvn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bspvn.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [BSPVN](https://www.netlib.org/slatec/src/bspvn.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `T` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | LE. X. T(N+1) and J=IWORK is set inside the routine on the first call when INDEX=1. ILEFT is such that T(ILEFT). LT. T(ILEFT+1). |
| 2 | `JHIGH` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | order of B-spline, 1. LE. JHIGH. K. |
| 3 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | highest possible order. |
| 4 | `INDEX` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INDEX = 1 gives basis functions of order JHIGH = 2 denotes previous entry with WORK, IWORK values saved for subsequent calls to BSPVN. |
| 5 | `X` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | argument of basis functions,. |
| 6 | `ILEFT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | largest integer such that. |
| 7 | `VNIKX` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | vector of length K for spline values. |
| 8 | `WORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a work vector of length 2*K. |
| 9 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | scalar | a work parameter. Both WORK and IWORK contain information necessary to continue for INDEX = 2. When INDEX = 1 exclusively, these are scratch variables and can be used for other purposes. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

Improper input is a fatal error.

### Storage and workspace requirements

`WORK`: a work vector of length 2*K

`IWORK`: a work parameter. Both WORK and IWORK contain information necessary to continue for INDEX = 2. When INDEX = 1 exclusively, these are scratch variables and can be used for other purposes.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::bspvn`. Native symbol: `bspvn_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::bspvn`
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
