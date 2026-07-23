# D1MERG

[Family: Shared numerical utilities](../families/shared-numerical-utilities.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Merge two strings of ascending double precision numbers.

## Description

This subroutine merges two ascending strings of numbers in the array TCOS. The first string is of length M1 and starts at TCOS(I1+1). The second string is of length M2 and starts at TCOS(I2+1). The merged string goes into TCOS(I3+1). This routine is currently unused, but was added to complete the set of routines S1MERG and C1MERG (both of which are used).

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Shared numerical utilities`
- Mathematical domain: `data-utilities`
- Package provenance: `shared-utility`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/d1merg.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/d1merg.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/d1merg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/d1merg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Shared numerical utilities](../families/shared-numerical-utilities.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `TCOS` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | This subroutine merges two ascending strings of numbers in the array TCOS. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `I1` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The first string is of length M1 and starts at TCOS(I1+1). | The first string is of length M1 and starts at TCOS(I1+1). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M1` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The first string is of length M1 and starts at TCOS(I1+1). | The first string is of length M1 and starts at TCOS(I1+1). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `I2` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The second string is of length M2 and starts at TCOS(I2+1). | The second string is of length M2 and starts at TCOS(I2+1). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `M2` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The second string is of length M2 and starts at TCOS(I2+1). | The second string is of length M2 and starts at TCOS(I2+1). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `I3` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | The merged string goes into TCOS(I3+1). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
