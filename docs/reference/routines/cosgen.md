# COSGEN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to GENBUN

## Description

This subroutine computes required cosine values in ascending order. When IJUMP .GT. 1 the routine computes values 2*COS(J*PI/L) , J=1,2,...,L and J .NE. 0(MOD N/IJUMP+1) where L = IJUMP*(N/IJUMP+1). when IJUMP = 1 it computes 2*COS((J-FNUM)*PI/(N+FDEN)) , J=1, 2, ... ,N where FNUM = 0.5, FDEN = 0.0, for regular reduction values. FNUM = 0.0, FDEN = 1.0, for B-R and C-R when ISTAG = 1 FNUM = 0.0, FDEN = 0.5, for B-R and C-R when ISTAG = 2 FNUM = 0.5, FDEN = 0.5, for B-R and C-R when ISTAG = 2 in POISN2 only.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `FISHPACK elliptic PDE solvers`
- Mathematical domain: `pde-integral-equations`
- Package provenance: `fishpack`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `GENBUN`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cosgen.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cosgen.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cosgen.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cosgen.f) — `verified_cached`
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
