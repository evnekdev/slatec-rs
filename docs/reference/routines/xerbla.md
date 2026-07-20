# XERBLA

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Error handler for the Level 2 and Level 3 BLAS Routines.

## Description

Purpose It is called by Level 2 and 3 BLAS routines if an input parameter is invalid. Parameters SRNAME - CHARACTER*6. On entry, SRNAME specifies the name of the routine which called XERBLA. INFO - INTEGER. On entry, INFO specifies the position of the invalid parameter in the parameter-list of the calling routine.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Error handling`
- Mathematical domain: `runtime-support`
- Package provenance: `slatec-error`
- GAMS classifications: `R3`
- Family evidence: `package_provenance` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/xerbla.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xerbla.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xerbla.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xerbla.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `not_generated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `not_assigned`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `runtime or machine-support unit is not independently callable`
<!-- raw-api-status:end -->
