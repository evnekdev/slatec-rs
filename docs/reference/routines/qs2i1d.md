# QS2I1D

[Family: Shared numerical utilities](../families/shared-numerical-utilities.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Sort an integer array, moving an integer and DP array. This routine sorts the integer array IA and makes the same interchanges in the integer array JA and the double precision array A. The array IA may be sorted in increasing order or decreasing order. A slightly modified QUICKSORT algorithm is used.

## Description

Written by Rondall E Jones Modified by John A. Wisniewski to use the Singleton QUICKSORT algorithm. date 18 November 1976. Further modified by David K. Kahaner National Bureau of Standards August, 1981 Even further modification made to bring the code up to the Fortran 77 level and make it more readable and to carry along one integer array and one double precision array during the sort by Mark K. Seager Lawrence Livermore National Laboratory November, 1987 This routine was adapted from the ISORT routine. This routine sorts an integer array IA and makes the same interchanges in the integer array JA and the double precision array A. The array IA may be sorted in increasing order or decreasing order. A slightly modified quicksort algorithm is used.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Shared numerical utilities`
- Mathematical domain: `data-utilities`
- Package provenance: `unknown`
- GAMS classifications: `N6A2A`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/qs2i1d.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/qs2i1d.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/qs2i1d.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
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
| `IA` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (N) | This routine sorts an integer array IA and makes the same interchanges in the integer array JA and the double precision array A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `JA` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (N) | This routine sorts an integer array IA and makes the same interchanges in the integer array JA and the double precision array A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (N) | Written by Rondall E Jones Modified by John A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `KFLAG` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | No separable argument description was found in the selected source prologue. | unavailable Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
