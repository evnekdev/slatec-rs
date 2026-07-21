# DBSPEV

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of the spline and its derivatives from the B-representation.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPEV is the BSPLEV routine of the reference. DBSPEV calculates the value of the spline and its derivatives at X from the B-representation (T,A,N,K) and returns them in SVALUE(I),I=1,NDERIV, T(K) .LE. X .LE. T(N+1). AD(I) can be the B-spline coefficients A(I), I=1,N) if NDERIV=1. Otherwise AD must be computed before hand by a call to DBSPDR (T,A,N,K, NDERIV,AD). If X=T(I),I=K,N), right limiting values are obtained. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. DBSPEV calls DINTRV, DBSPVN

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

- Canonical provider: `main-src/src/dbspev.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbspev.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbspev.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbspev.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [DBSPEV](https://www.netlib.org/slatec/src/dbspev.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `T` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | knot vector of length N+K |
| 2 | `AD` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | vector of length (2*N-NDERIV+1)*NDERIV/2 containing the difference table from DBSPDR. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of B-spline coefficients K |
| 4 | `K` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | order of the B-spline, K .GE. 1 |
| 5 | `NDERIV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | number of derivatives, 1 .LE. NDERIV .LE. K. th derivative = function value 1 derivatives in the remaining components. |
| 6 | `X` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | argument, T(K) .LE. X .LE. T(N+1) |
| 7 | `INEV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | an initialization parameter which must be set to 1 the first time DBSPEV is called. Output     SVALUE,WORK are double precision INEV contains information for efficient process- ing after the initial call and INEV must not be changed by the user.  Distinct splines require distinct INEV parameters. |
| 8 | `SVALUE` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | vector of length NDERIV containing the spline 1 derivatives in the remaining components. |
| 9 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | work vector of length 3*K |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: work vector of length 3*K

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dbspev`. Native symbol: `dbspev_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dbspev`
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
