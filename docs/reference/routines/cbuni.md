# CBUNI

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to CBESI and CBESK

## Description

CBUNI COMPUTES THE I BESSEL FUNCTION FOR LARGE ABS(Z).GT. FNUL AND FNU+N-1.LT.FNUL. THE ORDER IS INCREASED FROM FNU+N-1 GREATER THAN FNUL BY ADDING NUI AND COMPUTING ACCORDING TO THE UNIFORM ASYMPTOTIC EXPANSION FOR I(FNU,Z) ON IFORM=1 AND THE EXPANSION FOR J(FNU,Z) ON IFORM=2

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `CBESI, CBESK`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cbuni.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cbuni.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cbuni.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cbuni.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Special functions](../families/special-functions.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `Z` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | scalar | CBUNI COMPUTES THE I BESSEL FUNCTION FOR LARGE ABS(Z).GT. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FNU` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | FNUL AND FNU+N-1.LT.FNUL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KODE` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | FNUL AND FNU+N-1.LT.FNUL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `COMPLEX` (`explicit`) | `*mut crate::Complex32` | rank 1; dimensions (N) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NZ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NUI` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | THE ORDER IS INCREASED FROM FNU+N-1 GREATER THAN FNUL BY ADDING NUI AND COMPUTING ACCORDING TO THE UNIFORM ASYMPTOTIC EXPANSION FOR I(FNU,Z) ON IFORM=1 AND THE EXPANSION FOR J(FNU,Z) ON IFORM=2 | THE ORDER IS INCREASED FROM FNU+N-1 GREATER THAN FNUL BY ADDING NUI AND COMPUTING ACCORDING TO THE UNIFORM ASYMPTOTIC EXPANSION FOR I(FNU,Z) ON IFORM=1 AND THE EXPANSION FOR J(FNU,Z) ON IFORM=2 Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NLAST` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FNUL` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | FNUL AND FNU+N-1.LT.FNUL. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `TOL` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELIM` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ALIM` | unavailable | `REAL` (`explicit`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
