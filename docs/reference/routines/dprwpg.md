# DPRWPG

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DSPLP

## Description

DPRWPG LIMITS THE TYPE OF STORAGE TO A SEQUENTIAL SCHEME. VIRTUAL MEMORY PAGE READ/WRITE SUBROUTINE. DEPENDING ON THE VALUE OF KEY, SUBROUTINE DPRWPG() PERFORMS A PAGE READ OR WRITE OF PAGE IPAGE. THE PAGE HAS LENGTH LPG. KEY IS A FLAG INDICATING WHETHER A PAGE READ OR WRITE IS TO BE PERFORMED. IF KEY = 1 DATA IS READ. IF KEY = 2 DATA IS WRITTEN. IPAGE IS THE PAGE NUMBER OF THE MATRIX TO BE ACCESSED. LPG IS THE LENGTH OF THE PAGE OF THE MATRIX TO BE ACCESSED. SX(*),IX(*) IS THE MATRIX TO BE ACCESSED. THIS SUBROUTINE IS A MODIFICATION OF THE SUBROUTINE LRWPGE, SANDIA LABS. REPT. SAND78-0785. MODIFICATIONS BY K.L. HIEBERT AND R.J. HANSON REVISED 811130-1000 REVISED YYMMDD-HHMM

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

- Canonical provider: `main-src/src/dprwpg.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dprwpg.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dprwpg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dprwpg.f) — `verified_cached`
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
