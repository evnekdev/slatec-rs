# QS2I1R

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Sort an integer array, moving an integer and real array. This routine sorts the integer array IA and makes the same interchanges in the integer array JA and the real array A. The array IA may be sorted in increasing order or decreasing order. A slightly modified QUICKSORT algorithm is used.

## Description

Written by Rondall E Jones Modified by John A. Wisniewski to use the Singleton QUICKSORT algorithm. date 18 November 1976. Further modified by David K. Kahaner National Bureau of Standards August, 1981 Even further modification made to bring the code up to the Fortran 77 level and make it more readable and to carry along one integer array and one real array during the sort by Mark K. Seager Lawrence Livermore National Laboratory November, 1987 This routine was adapted from the ISORT routine. This routine sorts an integer array IA and makes the same interchanges in the integer array JA and the real array A. The array IA may be sorted in increasing order or decreasing order. A slightly modified quicksort algorithm is used.

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

- Canonical provider: `lin/qs2i1r.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/qs2i1r.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/qs2i1r.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
