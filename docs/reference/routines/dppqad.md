# DPPQAD

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the integral on (X1,X2) of a K-th order B-spline using the piecewise polynomial (PP) representation.

## Description

Abstract **** a double precision routine **** DPPQAD computes the integral on (X1,X2) of a K-th order B-spline using the piecewise polynomial representation (C,XI,LXI,K). Here the Taylor expansion about the left end point XI(J) of the J-th interval is integrated and evaluated on subintervals of (X1,X2) which are formed by included break points. Integration outside (XI(1),XI(LXI+1)) is permitted.

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
- GAMS classifications: `H2A2A1`
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

- Canonical provider: `main-src/src/dppqad.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dppqad.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dppqad.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dppqad.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DPPQAD](https://www.netlib.org/slatec/src/dppqad.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `LDC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | leading dimension of matrix C, LDC. GE. K. |
| 2 | `C` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDC, *) | right Taylor derivatives at XI(J), I=1,K , J=1,LXI. |
| 3 | `XI` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | break point array of length LXI+1. LE. X. XI(LXI+1) Output PQUAD is double precision. |
| 4 | `LXI` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of polynomial pieces. |
| 5 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | order of B-spline, K. GE. 1. |
| 6 | `X1` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | end points of quadrature interval, normally in. |
| 7 | `X2` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | end points of quadrature interval, normally in. |
| 8 | `PQUAD` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | integral of the PP representation over (X1,X2). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

Improper input is a fatal error

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::quadrature::dppqad`. Native symbol: `dppqad_`. Declaration feature: `quadrature`. Provider feature: `quadrature-direct`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_f64,mut_f64,mut_f64)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::quadrature::dppqad`
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
