# PINITM

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

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
