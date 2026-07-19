# XERSVE

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Record that an error has occurred.

## Description

*Usage: INTEGER KFLAG, NERR, LEVEL, ICOUNT CHARACTER * (len) LIBRAR, SUBROU, MESSG CALL XERSVE (LIBRAR, SUBROU, MESSG, KFLAG, NERR, LEVEL, ICOUNT) *Arguments: LIBRAR :IN is the library that the message is from. SUBROU :IN is the subroutine that the message is from. MESSG :IN is the message to be saved. KFLAG :IN indicates the action to be performed. when KFLAG > 0, the message in MESSG is saved. when KFLAG=0 the tables will be dumped and cleared. when KFLAG < 0, the tables will be dumped and not cleared. NERR :IN is the error number. LEVEL :IN is the error severity. ICOUNT :OUT the number of times this message has been seen, or zero if the table has overflowed and does not contain this message specifically. When KFLAG=0, ICOUNT will not be altered. *Description: Record that this error occurred and possibly dump and clear the tables.

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

- Canonical provider: `main-src/src/xersve.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xersve.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xersve.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xersve.f) — `verified_cached`
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
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `runtime or machine-support unit is not independently callable`
<!-- raw-api-status:end -->
