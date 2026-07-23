# WNNLS

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linearly constrained least squares problem with equality constraints and nonnegativity constraints on selected variables.

## Description

This subprogram solves a linearly constrained least squares problem. Suppose there are given matrices E and A of respective dimensions ME by N and MA by N, and vectors F and B of respective lengths ME and MA. This subroutine solves the problem EX = F, (equations to be exactly satisfied) AX = B, (equations to be approximately satisfied, in the least squares sense) subject to components L+1,...,N nonnegative Any values ME.GE.0, MA.GE.0 and 0.LE. L .LE.N are permitted. The problem is reposed as problem WNNLS

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1A2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::linear_least_squares::solve_nonnegative_least_squares_f32`

## Providers

- Canonical provider: `main-src/src/wnnls.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/wnnls.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/wnnls.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/wnnls.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [WNNLS](https://www.netlib.org/slatec/src/wnnls.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDW, *) | The array W(*,*) is double subscripted with first. |
| 2 | `MDW` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The array W(*,*) is double subscripted with first. |
| 3 | `ME` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW. GE. M. The condition MDW. |
| 4 | `MA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW. GE. M. The condition MDW. |
| 5 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW. GE. M. The condition MDW. |
| 6 | `L` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensioning parameter equal to MDW. For this discussion let us call M = ME + MA. Then MDW must satisfy MDW. GE. M. The condition MDW. |
| 7 | `PRGOPT` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | This real-valued array is the option vector. If the user is satisfied with the nominal subprogram features set 1 (or PRGOPT(1)=1. 0) Otherwise PRGOPT(*) is a linked list consisting of groups of data of the following form LINK KEY DATA SET The parameters LINK and KEY are each one word. The DATA SET can be comprised of several words. The number of items depends on the value of KEY. The value of LINK points to the first entry of the next group of data within The exception is when there are no more options to change. |
| 8 | `X` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | (WT*F) ( A) ( B), (least squares) subject to components L+1,. ,N nonnegative. The subprogram chooses the heavy weight (or penalty parameter) WT. The parameters for WNNLS are An array dimensioned at least N, which will contain the N components of the solution vector. |
| 9 | `RNORM` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | The residual norm of the solution. The value of contains the residual vector length of the equality constraints and least squares equations. |
| 10 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The value of MODE indicates the success or failure of the subprogram. 0 Subprogram completed successfully. = 1 Max. number of iterations (equal to 3*(N-L)) exceeded. Nearly all problems should complete in fewer than this number of iterations. An approximate solution and its corresponding residual vector length are in X(*) and RNORM. |
| 11 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The amounts of working storage actually allocated for the working arrays WORK(*) and IWORK(*), respectively. These quantities are compared with the actual amounts of storage needed for WNNLS( ). Insufficient storage allocated for either WORK(*) or IWORK(*) is considered an error. This feature was included in WNNLS( ) because miscalculating the storage formulas for WORK(*) and IWORK(*) might very well lead to subtle and hard-to-find execution errors. The length of WORK(*) must be at least LW = ME+MA+5*N This test will not be made if IWORK(1). LE. |
| 12 | `WORK` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | A real-valued working array of length at least M + 5*N. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`IWORK`: The amounts of working storage actually allocated for the working arrays WORK(*) and IWORK(*), respectively. These quantities are compared with the actual amounts of storage needed for WNNLS( ). Insufficient storage allocated for either WORK(*) or IWORK(*) is considered an error. This feature was included in WNNLS( ) because miscalculating the storage formulas for WORK(*) and IWORK(*) might very well lead to subtle and hard-to-find execution errors. The length of WORK(*) must be at least LW = ME+MA+5*N This test will not be made if IWORK(1).LE.0. The length of IWORK(*) must be at least LIW = ME+MA+N This test will not be made if IWORK(2).LE.0. An integer-valued working array of length at least M+N.

`WORK`: A real-valued working array of length at least M + 5*N.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::wnnls`. Native symbol: `wnnls_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_i32,mut_i32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::wnnls`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::linear_least_squares::solve_nonnegative_least_squares_f32`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
