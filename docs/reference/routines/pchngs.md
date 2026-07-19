# PCHNGS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to SPLP

## Description

PCHNGS LIMITS THE TYPE OF STORAGE TO A SEQUENTIAL SCHEME. SPARSE MATRIX ELEMENT ALTERATION SUBROUTINE. SUBROUTINE PCHNGS() CHANGES ELEMENT II IN VECTOR +/- IRCX TO THE VALUE XVAL. II THE ABSOLUTE VALUE OF THIS INTEGER IS THE SUBSCRIPT FOR THE ELEMENT TO BE CHANGED. XVAL NEW VALUE OF THE MATRIX ELEMENT BEING CHANGED. IPLACE POINTER INFORMATION WHICH IS MAINTAINED BY THE PACKAGE. SX(*),IX(*) THE WORK ARRAYS WHICH ARE USED TO STORE THE SPARSE MATRIX. THESE ARRAYS ARE AUTOMATICALLY MAINTAINED BY THE PACKAGE FOR THE USER. IRCX POINTS TO THE VECTOR OF THE MATRIX BEING UPDATED. A NEGATIVE VALUE OF IRCX INDICATES THAT ROW -IRCX IS BEING UPDATED. A POSITIVE VALUE OF IRCX INDICATES THAT COLUMN IRCX IS BEING UPDATED. A ZERO VALUE OF IRCX IS AN ERROR. SINCE DATA ITEMS ARE KEPT SORTED IN THE SEQUENTIAL DATA STRUCTURE, CHANGING A MATRIX ELEMENT CAN REQUIRE THE MOVEMENT OF ALL THE DATA ITEMS IN THE MATRIX. FOR THIS REASON, IT IS SUGGESTED THAT DATA ITEMS BE ADDED A COL. AT A TIME, IN ASCENDING COL. SEQUENCE. FURTHERMORE, SINCE DELETING ITEMS FROM THE DATA STRUCTURE MAY ALSO REQUIRE MOVING LARGE AMOUNTS OF DATA, ZERO ELEMENTS ARE EXPLICITLY STORED IN THE MATRIX. THIS SUBROUTINE IS A MODIFICATION OF THE SUBROUTINE LCHNGS, SANDIA LABS. REPT. SAND78-0785. MODIFICATIONS BY K.L. HIEBERT AND R.J. HANSON REVISED 811130-1000 REVISED YYMMDD-HHMM

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

- Canonical provider: `main-src/src/pchngs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/pchngs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/pchngs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/pchngs.f) — `verified_cached`
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
