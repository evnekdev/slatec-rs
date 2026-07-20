# DBNSLV

[Family: Interpolation](../families/interpolation.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBINT4 and DBINTK

## Description

DBNSLV is the BANSLV routine from * A Practical Guide to Splines * by C. de Boor DBNSLV is a double precision routine Companion routine to DBNFAC . It returns the solution X of the linear system A*X = B in place of B , given the LU-factorization for A in the work array W from DBNFAC. ***** I N P U T ****** W,B are DOUBLE PRECISION W, NROWW,NROW,NBANDL,NBANDU.....Describe the LU-factorization of a banded matrix A of order NROW as constructed in DBNFAC . For details, see DBNFAC . B.....Right side of the system to be solved . ***** O U T P U T ****** B is DOUBLE PRECISION B.....Contains the solution X , of order NROW .

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBINT4, DBINTK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dbnslv.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbnslv.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbnslv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Interpolation](../families/interpolation.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `W` | workspace | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 2; dimensions (NROWW, *) | It returns the solution X of the linear system A*X = B in place of B , given the LU-factorization for A in the work array W from DBNFAC. | none stated in the separable source sentence Leading dimension: not established Workspace: It returns the solution X of the linear system A*X = B in place of B , given the LU-factorization for A in the work array W from DBNFAC. | required; null is not permitted for an ordinary Fortran actual argument |
| `NROWW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ***** I N P U T ****** W,B are DOUBLE PRECISION W, NROWW,NROW,NBANDL,NBANDU.....Describe the LU-factorization of a banded matrix A of order NROW as constructed in DBNFAC . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NROW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ***** I N P U T ****** W,B are DOUBLE PRECISION W, NROWW,NROW,NBANDL,NBANDU.....Describe the LU-factorization of a banded matrix A of order NROW as constructed in DBNFAC . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBANDL` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ***** I N P U T ****** W,B are DOUBLE PRECISION W, NROWW,NROW,NBANDL,NBANDU.....Describe the LU-factorization of a banded matrix A of order NROW as constructed in DBNFAC . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NBANDU` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | ***** I N P U T ****** W,B are DOUBLE PRECISION W, NROWW,NROW,NBANDL,NBANDU.....Describe the LU-factorization of a banded matrix A of order NROW as constructed in DBNFAC . | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | workspace | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | It returns the solution X of the linear system A*X = B in place of B , given the LU-factorization for A in the work array W from DBNFAC. | none stated in the separable source sentence Leading dimension: not established Workspace: It returns the solution X of the linear system A*X = B in place of B , given the LU-factorization for A in the work array W from DBNFAC. | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
