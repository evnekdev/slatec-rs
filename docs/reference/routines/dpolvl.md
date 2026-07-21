# DPOLVL

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of a polynomial and its first NDER derivatives where the polynomial was produced by a previous call to DPLINT.

## Description

Subroutine DPOLVL calculates the value of the polynomial and its first NDER derivatives where the polynomial was produced by a previous call to DPLINT.

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

- Canonical provider: `main-src/src/dpolvl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpolvl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpolvl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [DPOLVL](https://www.netlib.org/slatec/src/dpolvl.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NDER` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is .GT. 0. Note *** 0, neither YP nor WORK need to be dimensioned variables. 1, YP does not need to be a dimensioned variable. the number of derivatives to be evaluated 0, WORK does not need to be a dimensioned variable. |
| 2 | `XX` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | the argument at which the polynomial and its derivatives are to be evaluated. 1,...,NDER. |
| 3 | `YFIT` | `output` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | the value of the polynomial at XX |
| 4 | `YP` | `output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | must be dimensioned by at least NDER the derivatives of the polynomial at XX.  The derivative of 1,...,NDER. |
| 5 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must not be altered between the call to DPLINT and the call to DPOLVL. Dimensioning Information ******* is .GT. 0. Note *** ***** must not be altered between the call |
| 6 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | must not be altered between the call to DPLINT and the call to DPOLVL. Dimensioning Information ******* must be dimensioned by at least N (see the abstract ) must not be altered between the call *       to DPLINT and the call to DPOLVL. |
| 7 | `C` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | must not be altered between the call to DPLINT and the call to DPOLVL. Dimensioning Information ******* must be dimensioned by at least N (see the abstract ) must not be altered between the call ***** |
| 8 | `WORK` | `workspace` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | is .GT. 0. Note *** this is an array to provide internal working storage for DPOLVL.  It must be dimensioned by at least 2*N if NDER is |
| 9 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Output error flag with the following possible values. = 1  indicates normal execution Storage Parameters |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WORK`: is .GT. 0. Note *** this is an array to provide internal working storage for DPOLVL.  It must be dimensioned by at least 2*N if NDER is

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dpolvl`. Native symbol: `dpolvl_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f64,mut_f64,mut_f64_ptr_rank1,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dpolvl`
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
