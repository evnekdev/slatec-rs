# DGAUS8

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Integrate a real function of one variable over a finite interval using an adaptive 8-point Legendre-Gauss algorithm. Intended primarily for high accuracy integration or integration of smooth functions.

## Description

Abstract *** a DOUBLE PRECISION routine *** DGAUS8 integrates real functions of one variable over finite intervals using an adaptive 8-point Legendre-Gauss algorithm. DGAUS8 is intended primarily for high accuracy integration or integration of smooth functions. The maximum number of significant digits obtainable in ANS is the smaller of 18 and the number of digits carried in double precision arithmetic.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A1A1`
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

- Canonical provider: `main-src/src/dgaus8.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dgaus8.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dgaus8.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dgaus8.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DGAUS8](https://www.netlib.org/slatec/src/dgaus8.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `FUN` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | name of external function to be integrated. This name must be in an EXTERNAL statement in the calling program. must be a DOUBLE PRECISION function of one DOUBLE PRECISION argument. The value of the argument to FUN is the variable of integration which ranges from A to B. |
| 2 | `A` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | lower limit of integration B. -1 A and B are too nearly equal to allow normal integration. ANS is set to zero. --Abnormal code 2 ANS probably does not meet requested error tolerance. |
| 3 | `B` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | upper limit of integration (may be less than A). |
| 4 | `ERR` | `input-output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | is a requested pseudorelative error tolerance. Normally pick a value of ABS(ERR) so that DTOL. LT. ABS(ERR). LE. 1. |
| 5 | `ANS` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | computed value of integral. |
| 6 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | a status code --Normal codes 1 ANS most likely meets requested error tolerance,. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dgaus8`. Native symbol: `dgaus8_`. Declaration feature: `quadrature`. Provider feature: `quadrature-direct`. ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::dgaus8`
- Public declaration feature: `quadrature`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
