# DROTM

[Family: Linear algebra kernels](../families/linear-algebra-kernels.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Apply a modified Givens transformation.

## Description

B L A S Subprogram Description of Parameters

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
- GAMS classifications: `D1A8`
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

- Canonical provider: `lin/drotm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/drotm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DROTM](https://www.netlib.org/slatec/lin/drotm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for DY using LY and INCY. |
| 2 | `DX` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for DY using LY and INCY. |
| 3 | `INCX` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1, where LX = 1 if INCX .GE. 0, else LX = 1+(1-N)*INCX, and similarly for DY using LY and INCY. |
| 4 | `DY` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `INCY` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `DPARAM` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (5) | element D.P. vector.  DPARAM(1) is DFLAG described below. Locations 2-5 of SPARAM contain elements of the transformation matrix H described below. DFLAG, H has one of the following forms: DFLAG=-1.D0     DFLAG=0.D0        DFLAG=1.D0     DFLAG=-2.D0 (DH11  DH12)    (1.D0  DH12)    (DH11  1.D0)    (1.D0  0.D0) H=(          )    (          )    (          )    (          ) (DH21  DH22),   (DH21  1.D0),   (-1.D0 DH22),   (0.D0  1.D0). See DROTMG for a description of data storage in DPARAM. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::blas::level1::drotm`. Native symbol: `drotm_`. Declaration feature: `blas-level1`. Provider feature: `blas-level1`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::blas::level1::drotm`
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
