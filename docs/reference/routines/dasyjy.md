# DASYJY

[Family: Special functions](../families/special-functions.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBESJ and DBESY

## Description

DASYJY computes Bessel functions J and Y for arguments X.GT.0.0 and orders FNU .GE. 35.0 on FLGJY = 1 and FLGJY = -1 respectively

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBESJ, DBESY`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dasyjy.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dasyjy.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dasyjy.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dasyjy.f) — `verified_cached`
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
| `FUNJY` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DASYJY computes Bessel functions J and Y for arguments X.GT.0.0 and orders FNU .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FNU` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | DASYJY computes Bessel functions J and Y for arguments X.GT.0.0 and orders FNU .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FLGJY` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | 35.0 on FLGJY = 1 and FLGJY = -1 respectively | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IN` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Y` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | DASYJY computes Bessel functions J and Y for arguments X.GT.0.0 and orders FNU .GE. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WK` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IFLW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
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
