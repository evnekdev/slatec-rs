# DPNNZR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DSPLP

## Description

DPNNZR LIMITS THE TYPE OF STORAGE TO A SEQUENTIAL SCHEME. SPARSE MATRIX NON ZERO RETRIEVAL SUBROUTINE. SUBROUTINE DPNNZR() GETS THE NEXT NONZERO VALUE IN ROW OR COLUMN +/- IRCX WITH AN INDEX GREATER THAN THE VALUE OF I. I ABSOLUTE VALUE OF THIS SUBSCRIPT IS TO BE EXCEEDED IN THE SEARCH FOR THE NEXT NONZERO VALUE. A NEGATIVE OR ZERO VALUE OF I CAUSES THE SEARCH TO START AT THE BEGINNING OF THE VECTOR. A POSITIVE VALUE OF I CAUSES THE SEARCH TO CONTINUE FROM THE LAST PLACE ACCESSED. ON OUTPUT, THE ARGUMENT I CONTAINS THE VALUE OF THE SUBSCRIPT FOUND. AN OUTPUT VALUE OF I EQUAL TO ZERO INDICATES THAT ALL COMPONENTS WITH AN INDEX GREATER THAN THE INPUT VALUE OF I ARE ZERO. XVAL VALUE OF THE NONZERO ELEMENT FOUND. ON OUTPUT, XVAL=0. WHENEVER I=0. IPLACE POINTER INFORMATION WHICH IS MAINTAINED BY THE PACKAGE. SX(*),IX(*) THE WORK ARRAYS WHICH ARE USED TO STORE THE SPARSE MATRIX. THESE ARRAY CONTENTS ARE AUTOMATICALLY MAINTAINED BY THE PACKAGE FOR THE USER. IRCX POINTS TO THE VECTOR OF THE MATRIX BEING SCANNED. A NEGATIVE VALUE OF IRCX INDICATES THAT ROW -IRCX IS TO BE SCANNED. A POSITIVE VALUE OF IRCX INDICATES THAT COLUMN IRCX IS TO BE SCANNED. A ZERO VALUE OF IRCX IS AN ERROR. THIS SUBROUTINE IS A MODIFICATION OF THE SUBROUTINE LNNZRS, SANDIA LABS. REPT. SAND78-0785. MODIFICATIONS BY K.L. HIEBERT AND R.J. HANSON REVISED 811130-1000 REVISED YYMMDD-HHMM

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

- Canonical provider: `main-src/src/dpnnzr.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpnnzr.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpnnzr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dpnnzr.f) — `verified_cached`
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
