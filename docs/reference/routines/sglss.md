# SGLSS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a linear least squares problems by performing a QR factorization of the matrix using Householder transformations. Emphasis is put on detecting possible rank deficiency.

## Description

SGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. If M.GE.N, the least squares solution is computed by decomposing the matrix A into the product of an orthogonal matrix Q and an upper triangular matrix R (QR factorization). If M.LT.N, the minimal length solution is computed by factoring the matrix A into the product of a lower triangular matrix L and an orthogonal matrix Q (LQ factorization). If the matrix A is determined to be rank deficient, that is the rank of A is less than MIN(M,N), then the minimal length least squares solution is computed. SGLSS assumes full machine precision in the data. If more control over the uncertainty in the data is desired, the codes LLSIA and ULSIA are recommended. SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. ****************************************************************** * * * WARNING - All input arrays are changed on exit. * * * ****************************************************************** SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
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

- Canonical provider: `main-src/src/sglss.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/sglss.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/sglss.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/sglss.f) — `verified_cached`
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
| `A` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (MDA, *) | SGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDA` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. | SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | SGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | SGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 2; dimensions (MDB, *) | SGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MDB` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. | SGLSS requires MDA*N + (MDB + 1)*NB + 5*MIN(M,N) dimensioned real space and M+N dimensioned integer space. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NB` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | SGLSS solves both underdetermined and overdetermined LINEAR systems AX = B, where A is an M by N matrix and B is an M by NB matrix of right hand sides. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `RNORM` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | * * * ****************************************************************** SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WORK` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | * * * ****************************************************************** SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LW` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | * * * ****************************************************************** SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IWORK` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | * * * ****************************************************************** SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LIW` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | * * * ****************************************************************** SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `INFO` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | * * * ****************************************************************** SUBROUTINE SGLSS(A,MDA,M,N,B,MDB,NB,RNORM,WORK,LW,IWORK,LIW,INFO) | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::dense::sglss`. Native symbol: `sglss_`. Feature: `linear-algebra`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_f32_ptr_rank2,mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_i32_ptr_rank1,mut_i32,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::sglss`
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
