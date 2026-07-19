# DXRED

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

To provide double-precision floating-point arithmetic with an extended exponent range.

## Description

DOUBLE PRECISION X INTEGER IX IF RADIX**(-2L) .LE. (ABS(X),IX) .LE. RADIX**(2L) THEN DXRED TRANSFORMS (X,IX) SO THAT IX=0. IF (X,IX) IS OUTSIDE THE ABOVE RANGE, THEN DXRED TAKES NO ACTION. THIS SUBROUTINE IS USEFUL IF THE RESULTS OF EXTENDED-RANGE CALCULATIONS ARE TO BE USED IN SUBSEQUENT ORDINARY DOUBLE-PRECISION CALCULATIONS.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Arithmetic and extended-range arithmetic`
- Mathematical domain: `numeric-support`
- Package provenance: `unknown`
- GAMS classifications: `A3D`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dxred.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dxred.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dxred.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
