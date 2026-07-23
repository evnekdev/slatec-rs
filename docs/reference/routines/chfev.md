# CHFEV

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate a cubic polynomial given in Hermite form at an array of points. While designed for use by PCHFE, it may be useful directly as an evaluator for a piecewise cubic Hermite function in applications, such as graphing, where the interval is known in advance.

## Description

CHFEV: Cubic Hermite Function EValuator Evaluates the cubic polynomial determined by function values F1,F2 and derivatives D1,D2 on interval (X1,X2) at the points XE(J), J=1(1)NE. Calling sequence: INTEGER NE, NEXT(2), IERR REAL X1, X2, F1, F2, D1, D2, XE(NE), FE(NE) CALL CHFEV (X1,X2, F1,F2, D1,D2, NE, XE, FE, NEXT, IERR)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- GAMS classifications: `E3`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `pchip/chfev.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/chfev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/chfev.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CHFEV](https://www.netlib.org/slatec/pchip/chfev.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X1` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) endpoints of interval of definition of cubic. (Error return if X1. EQ. X2. ). |
| 2 | `X2` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) endpoints of interval of definition of cubic. (Error return if X1. EQ. X2. ). |
| 3 | `F1` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) values of function at X1 and X2, respectively. |
| 4 | `F2` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) values of function at X1 and X2, respectively. |
| 5 | `D1` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) values of derivative at X1 and X2, respectively. |
| 6 | `D2` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | (input) values of derivative at X1 and X2, respectively. |
| 7 | `NE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (input) number of evaluation points. (Error return if NE. LT. 1. ). |
| 8 | `XE` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (input) real array of points at which the function is to be evaluated. If any of the XE are outside the interval [X1,X2], a warning error is returned in NEXT. |
| 9 | `FE` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (output) real array of values of the cubic function defined by X1,X2, F1,F2, D1,D2 at the points XE. |
| 10 | `NEXT` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (2) | (output) integer array indicating number of extrapolation points: number of evaluation points to left of interval. number of evaluation points to right of interval. |
| 11 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | (output) error flag. Normal return: 0 (no errors). "Recoverable" errors: -1 if NE. LT. 1. -2 if X1. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::chfev`. Native symbol: `chfev_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::chfev`
- Public declaration feature: `interpolation`
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
