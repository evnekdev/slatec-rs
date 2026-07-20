# DPRWPG

[Family: Optimization and least squares](../families/optimization-and-least-squares.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DSPLP

## Description

DPRWPG LIMITS THE TYPE OF STORAGE TO A SEQUENTIAL SCHEME. VIRTUAL MEMORY PAGE READ/WRITE SUBROUTINE. DEPENDING ON THE VALUE OF KEY, SUBROUTINE DPRWPG() PERFORMS A PAGE READ OR WRITE OF PAGE IPAGE. THE PAGE HAS LENGTH LPG. KEY IS A FLAG INDICATING WHETHER A PAGE READ OR WRITE IS TO BE PERFORMED. IF KEY = 1 DATA IS READ. IF KEY = 2 DATA IS WRITTEN. IPAGE IS THE PAGE NUMBER OF THE MATRIX TO BE ACCESSED. LPG IS THE LENGTH OF THE PAGE OF THE MATRIX TO BE ACCESSED. SX(*),IX(*) IS THE MATRIX TO BE ACCESSED. THIS SUBROUTINE IS A MODIFICATION OF THE SUBROUTINE LRWPGE, SANDIA LABS. REPT. SAND78-0785. MODIFICATIONS BY K.L. HIEBERT AND R.J. HANSON REVISED 811130-1000 REVISED YYMMDD-HHMM

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Optimization and least squares`
- Mathematical domain: `optimization`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DSPLP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dprwpg.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dprwpg.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dprwpg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dprwpg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Optimization and least squares](../families/optimization-and-least-squares.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `KEY` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DEPENDING ON THE VALUE OF KEY, SUBROUTINE DPRWPG() PERFORMS A PAGE READ OR WRITE OF PAGE IPAGE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAGE` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | DEPENDING ON THE VALUE OF KEY, SUBROUTINE DPRWPG() PERFORMS A PAGE READ OR WRITE OF PAGE IPAGE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LPG` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | THE PAGE HAS LENGTH LPG. | THE PAGE HAS LENGTH LPG. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SX` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | SX(*),IX(*) IS THE MATRIX TO BE ACCESSED. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IX` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | SX(*),IX(*) IS THE MATRIX TO BE ACCESSED. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
