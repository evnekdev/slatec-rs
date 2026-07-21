# DPOLCF

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the coefficients of the polynomial fit (including Hermite polynomial fits) produced by a previous call to POLINT.

## Description

Subroutine DPOLCF computes the coefficients of the polynomial fit (including Hermite polynomial fits ) produced by a previous call to DPLINT. The coefficients of the polynomial, expanded about XX, are stored in the array D. The expansion is of the form P(Z) = D(1) + D(2)*(Z-XX) +D(3)*((Z-XX)**2) + ... +

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
- GAMS classifications: `E1B`
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

- Canonical provider: `main-src/src/dpolcf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpolcf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpolcf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DPOLCF](https://www.netlib.org/slatec/src/dpolcf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `XX` | `input` | `scalar` | `DOUBLE PRECISION` | `*mut f64` | scalar | The point about which the Taylor expansion is to be made. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | **** N, X, and C must remain unchanged between the. |
| 3 | `X` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | * call to DPLINT and the call to DPOLCF. |
| 4 | `C` | `input` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | **** OUTPUT PARAMETER All TYPE REAL variables are DOUBLE PRECISION ***. |
| 5 | `D` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | Between the call to DPLINT and the call to DPOLCF the variable N and the arrays X and C must not be altered. The array of coefficients for the Taylor expansion as explained in the abstract STORAGE PARAMETER. |
| 6 | `WORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | This is an array to provide internal working storage. It must be dimensioned by at least 2*N in the calling program. Note - There are two methods for evaluating the fit produced by DPLINT. You may call DPOLVL to perform the task, or you may call DPOLCF to obtain the coefficients of the Taylor expansion and then write your own evaluation scheme. Due to the inherent errors in the computations of the Taylor expansion from the Newton coefficients produced by DPLINT, much more accuracy may be expected by calling DPOLVL as opposed to writing your own scheme. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`WORK`: This is an array to provide internal working storage. It must be dimensioned by at least 2*N in the calling program. Note - There are two methods for evaluating the fit produced by DPLINT. You may call DPOLVL to perform the task, or you may call DPOLCF to obtain the coefficients of the Taylor expansion and then write your own evaluation scheme. Due to the inherent errors in the computations of the Taylor expansion from the Newton coefficients produced by DPLINT, much more accuracy may be expected by calling DPOLVL as opposed to writing your own scheme.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::interpolation::dpolcf`. Native symbol: `dpolcf_`. Declaration feature: `interpolation`. Provider feature: `interpolation-general`. ABI fingerprint: `subroutine:void(mut_f64,mut_i32,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1,mut_f64_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::interpolation::dpolcf`
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
