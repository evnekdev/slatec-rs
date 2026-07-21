# DPPVAL

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of the IDERIV-th derivative of the B-spline from the PP-representation.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DPPVAL is the PPVALU function of the reference.

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

- Canonical provider: `main-src/src/dppval.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dppval.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dppval.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dppval.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DPPVAL](https://www.netlib.org/slatec/src/dppval.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `LDC` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | leading dimension of C matrix, LDC .GE. K |
| 2 | `C` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 2; dimensions (LDC, *) | The Taylor expansion about XI(J) for X in are double precision matrix of dimension at least (K,LXI) containing right derivatives at break points XI(*). |
| 3 | `XI` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The Taylor expansion about XI(J) for X in 1,LXI. 1,LXI. are double precision break point vector of length LXI+1 |
| 4 | `LXI` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The Taylor expansion about XI(J) for X in 1 and set X=XI(J),J=2,LXI+1. number of polynomial pieces |
| 5 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The Taylor expansion about XI(J) for X in order of B-spline, K .GE. 1 |
| 6 | `IDERIV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | th derivative of the B-spline from the PP-representation. th derivative of the B-spline from the PP-representation order of the derivative, 0 .LE. IDERIV .LE. K-1 spline value |
| 7 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | th derivative of the B-spline from the PP-representation 1,LXI. XI(J) are obtained.  DPPVAL will extrapolate beyond XI(1) and XI(LXI+1). To obtain left limiting values (left derivatives) at XI(J) are double precision argument, XI(1) .LE. X .LE. XI(LXI+1) |
| 8 | `INPPV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | an initialization parameter which must be set to 1 the first time DPPVAL is called. Output     DPPVAL is double precision INPPV contains information for efficient process- ing after the initial call and INPPV must not be changed by the user.  Distinct splines require distinct INPPV parameters. DPPVAL  - value of the IDERIV-th derivative at X |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This Fortran function returns its scalar result through the compiler-validated ABI fingerprint `function:f64(mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`.

### Callback contract

This interface declares no callback argument.

### Error and status values

Improper input is a fatal error

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dppval`. Native symbol: `dppval_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `function:f64(mut_i32,mut_f64_ptr_rank2,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dppval`
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
