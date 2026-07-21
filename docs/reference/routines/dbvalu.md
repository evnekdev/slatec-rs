# DBVALU

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Evaluate the B-representation of a B-spline at X for the function value or any of its derivatives.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBVALU is the BVALUE function of the reference. DBVALU evaluates the B-representation (T,A,N,K) of a B-spline

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
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

- Canonical provider: `main-src/src/dbvalu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbvalu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbvalu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbvalu.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DBVALU](https://www.netlib.org/slatec/src/dbvalu.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `T` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | 1 and set X=T(I), I=K+1,N+1. DBVALU calls DINTRV are double precision knot vector of length N+K |
| 2 | `A` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is outside of this interval. To compute left derivatives or left limiting values at a are double precision B-spline coefficient vector of length N |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 and set X=T(I), I=K+1,N+1. DBVALU calls DINTRV number of B-spline coefficients K |
| 4 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1.  Right limiting values (right derivatives) are returned except at the right end order of the B-spline, K .GE. 1 |
| 5 | `IDERIV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 0 or any of its 1.  Right limiting values (right derivatives) are returned except at the right end order of the derivative, 0 .LE. IDERIV .LE. K-1 spline value |
| 6 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | 0 or any of its T(N+1) where left limiting values are computed.  The spline is defined on T(K) .LE. X .LE. T(N+1).  DBVALU returns is outside of this interval. To compute left derivatives or left limiting values at a are double precision argument, T(K) .LE. X .LE. T(N+1) |
| 7 | `INBV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | an initialization parameter which must be set to 1 the first time DBVALU is called. Output     WORK,DBVALU are double precision INBV contains information for efficient process- ing after the initial call and INBV must not be changed by the user.  Distinct splines require distinct INBV parameters. |
| 8 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | work vector of length 3*K. DBVALU  - value of the IDERIV-th derivative at X |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f64(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

An improper input is a fatal error

### Storage and workspace requirements

`WORK`: work vector of length 3*K. DBVALU  - value of the IDERIV-th derivative at X

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dbvalu`. Native symbol: `dbvalu_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `function:f64(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dbvalu`
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
