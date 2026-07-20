# FDUMP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Symbolic dump (should be locally written).

## Description

***Note*** Machine Dependent Routine FDUMP is intended to be replaced by a locally written version which produces a symbolic dump. Failing this, it should be replaced by a version which prints the subprogram nesting list. Note that this dump must be printed on each of up to five files, as indicated by the XGETUA routine. See XSETUA and XGETUA for details. Written by Ron Jones, with SLATEC Common Math Library Subcommittee

## Classification

- Historical role: `user_callable`
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

- Canonical provider: `main-src/src/fdump.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/fdump.f` (`live-main-source`)
  - `err/fdump.f` (`legacy-error-directory`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/fdump.f) — `verified_cached`
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
- Exclusion or deferment reason: `runtime or machine-support unit is not independently callable`
<!-- raw-api-status:end -->
