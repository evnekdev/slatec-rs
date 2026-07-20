# PINITM

[Family: Optimization and least squares](../families/optimization-and-least-squares.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SPLP

## Description

PINITM LIMITS THE TYPE OF STORAGE TO A SEQUENTIAL SCHEME. THE MATRIX IS STORED BY COLUMNS. SPARSE MATRIX INITIALIZATION SUBROUTINE. M=NUMBER OF ROWS OF THE MATRIX. N=NUMBER OF COLUMNS OF THE MATRIX. SX(*),IX(*)=THE WORK ARRAYS WHICH ARE USED TO STORE THE SPARSE MATRIX. THESE ARRAYS ARE AUTOMATICALLY MAINTAINED BY THE PACKAGE FOR THE USER. LMX=LENGTH OF THE WORK ARRAY SX(*). LMX MUST BE AT LEAST N+7 WHERE FOR GREATEST EFFICIENCY LMX SHOULD BE AT LEAST N+NZ+6 WHERE NZ IS THE MAXIMUM NUMBER OF NONZEROES TO BE STORED IN THE MATRIX. VALUES OF LMX BETWEEN N+7 AND N+NZ+6 WILL CAUSE DEMAND PAGING TO OCCUR. THIS IS IMPLEMENTED BY THE PACKAGE. IX(*) MUST BE DIMENSIONED AT LEAST LMX IPAGEF=UNIT NUMBER WHERE DEMAND PAGES WILL BE STORED. THIS SUBROUTINE IS A MODIFICATION OF THE SUBROUTINE LINITM, SANDIA LABS. REPT. SAND78-0785. MODIFICATIONS BY K.L. HIEBERT AND R.J. HANSON REVISED 811130-1000 REVISED YYMMDD-HHMM

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

- Canonical provider: `main-src/src/pinitm.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/pinitm.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/pinitm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/pinitm.f) — `verified_cached`
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
| `M` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | M=NUMBER OF ROWS OF THE MATRIX. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | N=NUMBER OF COLUMNS OF THE MATRIX. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `SX` | workspace | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (LMX) | SX(*),IX(*)=THE WORK ARRAYS WHICH ARE USED TO STORE THE SPARSE MATRIX. | none stated in the separable source sentence Leading dimension: not established Workspace: SX(*),IX(*)=THE WORK ARRAYS WHICH ARE USED TO STORE THE SPARSE MATRIX. | required; null is not permitted for an ordinary Fortran actual argument |
| `IX` | workspace | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | SX(*),IX(*)=THE WORK ARRAYS WHICH ARE USED TO STORE THE SPARSE MATRIX. | none stated in the separable source sentence Leading dimension: not established Workspace: SX(*),IX(*)=THE WORK ARRAYS WHICH ARE USED TO STORE THE SPARSE MATRIX. | required; null is not permitted for an ordinary Fortran actual argument |
| `LMX` | workspace | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | LMX=LENGTH OF THE WORK ARRAY SX(*). | LMX=LENGTH OF THE WORK ARRAY SX(*). Leading dimension: not established Workspace: LMX=LENGTH OF THE WORK ARRAY SX(*). | required; null is not permitted for an ordinary Fortran actual argument |
| `IPAGEF` | unavailable | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | IX(*) MUST BE DIMENSIONED AT LEAST LMX IPAGEF=UNIT NUMBER WHERE DEMAND PAGES WILL BE STORED. | IX(*) MUST BE DIMENSIONED AT LEAST LMX IPAGEF=UNIT NUMBER WHERE DEMAND PAGES WILL BE STORED. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
