# LSAME

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Test two characters to determine if they are the same letter, except for case.

## Description

LSAME tests if CA is the same letter as CB regardless of case. CB is assumed to be an upper case letter. LSAME returns .TRUE. if CA is either the same as CB or the equivalent lower case letter. N.B. This version of the code is correct for both ASCII and EBCDIC systems. Installers must modify the routine for other character-codes. For CDC systems using 6-12 bit representations, the systemspecific code in comments must be activated. Parameters CA - CHARACTER*1 CB - CHARACTER*1 On entry, CA and CB specify characters to be compared. Unchanged on exit.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Runtime and machine support`
- Mathematical domain: `runtime-support`
- Package provenance: `unknown`
- GAMS classifications: `R`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/lsame.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/lsame.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/lsame.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

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
- Exclusion or deferment reason: `source exists but no reviewed or ABI-validated public declaration is recorded`
<!-- raw-api-status:end -->
