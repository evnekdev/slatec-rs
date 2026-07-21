# LSEI

[Family: Approximation](../families/approximation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linearly constrained least squares problem with equality and inequality constraints, and optionally compute a covariance matrix.

## Description

This subprogram solves a linearly constrained least squares problem with both equality and inequality constraints, and, if the user requests, obtains a covariance matrix of the solution

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
- Safe Rust paths: `slatec::constrained_least_squares::solve_constrained_least_squares_f32`

## Providers

- Canonical provider: `main-src/src/lsei.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/lsei.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/lsei.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/lsei.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [LSEI](https://www.netlib.org/slatec/src/lsei.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `W` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (MDW, *) | where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are The array W(*,*) is doubly subscripted with The array W(*,*) contains the N by N symmetric covariance matrix of the solution parameters, provided this was requested on input with the option vector PRGOPT(*) and the output flag is returned with MODE = 0 or 1. |
| 2 | `MDW` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The array W(*,*) is doubly subscripted with must satisfy MDW. GE. M. The condition. LT. M is an error. |
| 3 | `ME` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG. Then. |
| 4 | `MA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG. Then. |
| 5 | `MG` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG. Then. |
| 6 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | first dimensioning parameter equal to MDW. For this discussion let us call M = ME+MA+MG. Then. |
| 7 | `PRGOPT` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are This real-valued array is the option vector. If the user is satisfied with the nominal subprogram features set 1 (or PRGOPT(1)=1. |
| 8 | `X` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are The array X(*) contains the solution parameters. |
| 9 | `RNORME` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | The array X(*) contains the solution parameters. |
| 10 | `RNORML` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | if the integer output flag MODE = 0 or 1. The definition of MODE is given directly below. When MODE = 0 or 1, RNORME and RNORML respectively contain the residual vector Euclidean lengths of F - EX and B - AX. When. |
| 11 | `MODE` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1 the equality constraint equations EX=F are contradictory, so RNORME. NE. 0. The residual vector F-EX has minimal Euclidean length. For. GE. |
| 12 | `WS` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are These are respectively type real and type integer working arrays. Their required minimal lengths are given above. |
| 13 | `IP` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (3) | where K=MAX(MA+MG,N). This allows for a solution of a range of problems in the given working space. The dimension of WS(*) given is a necessary overestimate. Once a particular problem has been run, the output parameter IP(3) gives the actual dimension required for that problem. The parameters for LSEI( ) are The amounts of working storage actually allocated for the working arrays WS(*) and IP(*), respectively. These quantities are compared with the actual amounts of storage needed by LSEI( ). |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::approximation::lsei`. Native symbol: `lsei_`. Declaration feature: `approximation`. Provider feature: `approximation-core`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::approximation::lsei`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::constrained_least_squares::solve_constrained_least_squares_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
