# ULSIA

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve an underdetermined linear system of equations by performing an LQ factorization of the matrix using Householder transformations. Emphasis is put on detecting possible rank deficiency.

## Description

ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. User input bounds on the uncertainty in the elements of A are used to detect numerical rank deficiency. The algorithm employs a row and column pivot strategy to minimize the growth of uncertainty and round-off errors. ULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. * * * ******************************************************************

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D9`
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

- Canonical provider: `main-src/src/ulsia.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ulsia.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ulsia.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ulsia.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Dense linear algebra](../families/dense-linear-algebra.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `A` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (MDA, *) | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDA` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. | ULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (MDB, *) | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDB` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. | ULSIA requires (MDA+1)*N + (MDB+1)*NB + 6*M dimensioned space ****************************************************************** * * * WARNING - All input arrays are changed on exit. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NB` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. | ULSIA computes the minimal length solution(s) to the problem AX=B where A is an M by N matrix with M.LE.N and B is the M by NB matrix of right hand sides. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RE` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AE` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KEY` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MODE` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NP` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KRANK` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KSURE` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORM` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LW` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIW` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::ulsia`. Native symbol: `ulsia_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::ulsia`
- Compatibility aliases: `none`
- Public declaration feature: `linear-algebra`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
