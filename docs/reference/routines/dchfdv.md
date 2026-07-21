# DCHFDV

[Family: PCHIP](../families/pchip.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate a cubic polynomial given in Hermite form and its first derivative at an array of points. While designed for use by DPCHFD, it may be useful directly as an evaluator for a piecewise cubic Hermite function in applications, such as graphing, where the interval is known in advance. If only function values are required, use DCHFEV instead.

## Description

DCHFDV: Cubic Hermite Function and Derivative Evaluator Evaluates the cubic polynomial determined by function values F1,F2 and derivatives D1,D2 on interval (X1,X2), together with its first derivative, at the points XE(J), J=1(1)NE. If only function values are required, use DCHFEV, instead. ---------------------------------------------------------------------- Calling sequence: INTEGER NE, NEXT(2), IERR DOUBLE PRECISION X1, X2, F1, F2, D1, D2, XE(NE), FE(NE), DE(NE) CALL DCHFDV (X1,X2, F1,F2, D1,D2, NE, XE, FE, DE, NEXT, IERR) Parameters: X1,X2 -- (input) endpoints of interval of definition of cubic.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
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

- Canonical provider: `pchip/dchfdv.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/dchfdv.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/pchip/dchfdv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DCHFDV](https://www.netlib.org/slatec/pchip/dchfdv.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `X1` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `X2` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `F1` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `F2` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 5 | `D1` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `D2` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 7 | `NE` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 8 | `XE` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 9 | `FE` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `DE` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 11 | `NEXT` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (2) | Array argument classified by fixed-form executable read/write analysis. |
| 12 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

F1,F2 -- (input) values of function at X1 and X2, respectively. D1,D2 -- (input) values of derivative at X1 and X2, respectively. NE.LT.1 .) XE -- (input) real*8 array of points at which the functions are to be evaluated.  If any of the XE are outside the interval FE -- (output) real*8 array of values of the cubic function defined by  X1,X2, F1,F2, D1,D2  at the points  XE. DE -- (output) real*8 array of values of the first derivative of the same function at the points  XE. NEXT -- (output) integer array indicating number of extrapolation points: NEXT(1) = number of evaluation points to left of interval. NEXT(2) = number of evaluation points to right of interval. Normal return: IERR = -1  if NE.LT.1 . IERR = -2  if X1.EQ.X2 . (Output arrays have not been changed in either case.)

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dchfdv`. Native symbol: `dchfdv_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dchfdv`
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
