# POLYVL

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of a polynomial and its first NDER derivatives where the polynomial was produced by a previous call to POLINT.

## Description

Written by Robert E. Huddleston, Sandia Laboratories, Livermore Subroutine POLYVL calculates the value of the polynomial and its first NDER derivatives where the polynomial was produced by a previous call to POLINT. The variable N and the arrays X and C must not be altered between the call to POLINT and the call to POLYVL. Dimensioning Information *******

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `main-src/src/polyvl.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/polyvl.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/polyvl.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [POLYVL](https://www.netlib.org/slatec/src/polyvl.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NDER` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | the number of derivatives to be evaluated. |
| 2 | `XX` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | the argument at which the polynomial and its derivatives are to be evaluated. |
| 3 | `YFIT` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | the value of the polynomial at XX. |
| 4 | `YP` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | must be dimensioned by at least NDER the derivatives of the polynomial at XX. The derivative of order J at XX is stored in YP(J) , J = 1,. ,NDER. |
| 5 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | ***** N, X, and C must not be altered between the call. |
| 6 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | must be dimensioned by at least N (see the abstract ) * to POLINT and the call to POLYVL. |
| 7 | `C` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | must be dimensioned by at least N (see the abstract ) *****. |
| 8 | `WORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | must be dimensioned by at least 2*N if NDER is. GT. 0. Note *** If NDER=0, neither YP nor WORK need to be dimensioned variables. If NDER=1, YP does not need to be a dimensioned variable. this is an array to provide internal working storage for POLYVL. |
| 9 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Output error flag with the following possible values. = 1 indicates normal execution. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`WORK`: must be dimensioned by at least 2*N if NDER is .GT. 0. Note *** If NDER=0, neither YP nor WORK need to be dimensioned variables. If NDER=1, YP does not need to be a dimensioned variable. this is an array to provide internal working storage for POLYVL. It must be dimensioned by at least 2*N if NDER is .GT. 0. If NDER=0, WORK does not need to be a dimensioned

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::polyvl`. Native symbol: `polyvl_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::polyvl`
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
