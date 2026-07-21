# DINTRV

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the largest integer ILEFT in 1 .LE. ILEFT .LE. LXT such that XT(ILEFT) .LE. X where XT(*) is a subdivision of the X interval.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DINTRV is the INTERV routine of the reference. DINTRV computes the largest integer ILEFT in 1 .LE. ILEFT .LE. LXT such that XT(ILEFT) .LE. X where XT(*) is a subdivision of the X interval. Precisely, X .LT. XT(1) 1 -1 if XT(I) .LE. X .LT. XT(I+1) then ILEFT=I , MFLAG=0 XT(LXT) .LE. X LXT 1, That is, when multiplicities are present in the break point to the left of X, the largest index is taken for ILEFT.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
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

- Canonical provider: `main-src/src/dintrv.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dintrv.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dintrv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DINTRV](https://www.netlib.org/slatec/src/dintrv.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `XT` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | XT is a knot or break point vector of length LXT |
| 2 | `LXT` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | length of the XT vector |
| 3 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | argument |
| 4 | `ILO` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | an initialization parameter which must be set to 1 the first time the spline array XT is processed by DINTRV. ILO contains information for efficient process- ing after the initial call and ILO must not be changed by the user.  Distinct splines require distinct ILO parameters. |
| 5 | `ILEFT` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | largest integer satisfying XT(ILEFT) .LE. X |
| 6 | `MFLAG` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | signals when X lies out of bounds |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

None

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dintrv`. Native symbol: `dintrv_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_i32,mut_f64,mut_i32,mut_i32,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dintrv`
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
