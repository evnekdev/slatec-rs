# CPROC

[Family: FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBLKTR

## Description

PROC applies a sequence of matrix operations to the vector X and stores the result in Y. AA Array containing scalar multipliers of the vector X. ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. BD,BM1,BM2 are arrays containing roots of certain B polynomials. NA is the length of the array AA. X,Y The matrix operations are applied to X and the result is Y. A,B,C are arrays which contain the tridiagonal matrix. M is the order of the matrix. D,W are work arrays. ISGN determines whether or not a change in sign is made.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `FISHPACK elliptic PDE solvers`
- Mathematical domain: `pde-integral-equations`
- Package provenance: `fishpack`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `CBLKTR`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cproc.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cproc.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cproc.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [FISHPACK elliptic PDE solvers](../families/fishpack-elliptic-pde-solvers.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ND` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BD` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NM1` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BM1` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NM2` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `BM2` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. | ND,NM1,NM2 are the lengths of the arrays BD,BM1,BM2 respectively. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NA` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | NA is the length of the array AA. | NA is the length of the array AA. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AA` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | AA Array containing scalar multipliers of the vector X. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | PROC applies a sequence of matrix operations to the vector X and stores the result in Y. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | PROC applies a sequence of matrix operations to the vector X and stores the result in Y. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | M is the order of the matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | PROC applies a sequence of matrix operations to the vector X and stores the result in Y. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | BD,BM1,BM2 are arrays containing roots of certain B polynomials. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | A,B,C are arrays which contain the tridiagonal matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | workspace | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | D,W are work arrays. | none stated in the separable source sentence Leading dimension: not established Workspace: D,W are work arrays. | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | workspace | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (*) | D,W are work arrays. | none stated in the separable source sentence Leading dimension: not established Workspace: D,W are work arrays. | required; null is not permitted for an ordinary Fortran actual argument |
| `YY` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-complex-arguments`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
