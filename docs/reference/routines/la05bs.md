# LA05BS

[Family: Optimization and least squares](../families/optimization-and-least-squares.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SPLP

## Description

THIS SUBPROGRAM IS A SLIGHT MODIFICATION OF A SUBPROGRAM FROM THE C. 1979 AERE HARWELL LIBRARY. THE NAME OF THE CORRESPONDING HARWELL CODE CAN BE OBTAINED BY DELETING THE FINAL LETTER =S= IN THE NAMES USED HERE. REVISED SEP. 13, 1979. ROYALTIES HAVE BEEN PAID TO AERE-UK FOR USE OF THEIR CODES IN THE PACKAGE GIVEN HERE. ANY PRIMARY USAGE OF THE HARWELL SUBROUTINES REQUIRES A ROYALTY AGREEMENT AND PAYMENT BETWEEN THE USER AND AERE-UK. ANY USAGE OF THE SANDIA WRITTEN CODES SPLP( ) (WHICH USES THE HARWELL SUBROUTINES) IS PERMITTED. IP(I,1),IP(I,2) POINT TO START OF ROW/COLUMN I OF U. IW(I,1),IW(I,2) ARE LENGTHS OF ROW/COL I OF U. IW(.,3),IW(.,4) HOLD ROW/COL NUMBERS IN PIVOTAL ORDER.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Optimization and least squares`
- Mathematical domain: `optimization`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `SPLP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/la05bs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/la05bs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/la05bs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/la05bs.f) — `verified_cached`
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
| `A` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (IA) | THIS SUBPROGRAM IS A SLIGHT MODIFICATION OF A SUBPROGRAM FROM THE C. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IND` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 2; dimensions (IA, 2) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IA` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IP` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 2; dimensions (N, 2) | IP(I,1),IP(I,2) POINT TO START OF ROW/COLUMN I OF U. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 2; dimensions (N, 8) | IW(I,1),IW(I,2) ARE LENGTHS OF ROW/COL I OF U. | IW(I,1),IW(I,2) ARE LENGTHS OF ROW/COL I OF U. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `W` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `G` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TRANS` | unavailable | `LOGICAL` (`explicit`) | `*mut crate::FortranLogical` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-logical`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
