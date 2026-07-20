# DPRWVR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DSPLP

## Description

DPRWVR LIMITS THE TYPE OF STORAGE TO A SEQUENTIAL SPARSE MATRIX STORAGE SCHEME. THE PAGE STORAGE IS ON RANDOM ACCESS DISK. DPRWVR IS PART OF THE SPARSE LP PACKAGE, DSPLP. KEY IS A FLAG WHICH INDICATES WHETHER A READ OR WRITE OPERATION IS TO BE PERFORMED. A VALUE OF KEY=1 INDICATES A READ. A VALUE OF KEY=2 INDICATES A WRITE. IPAGE IS THE PAGE OF MATRIX MN WE ARE ACCESSING. LPG IS THE LENGTH OF THE PAGE. SX(*),IX(*) IS THE MATRIX DATA. THIS SUBROUTINE IS A MODIFICATION OF THE SUBROUTINE LRWVIR, SANDIA LABS. REPT. SAND78-0785. MODIFICATIONS BY K.L. HIEBERT AND R.J. HANSON

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Optimization and least squares`
- Mathematical domain: `optimization`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DSPLP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dprwvr.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dprwvr.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dprwvr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dprwvr.f) — `verified_cached`
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
