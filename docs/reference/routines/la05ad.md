# LA05AD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DSPLP

## Description

THIS SUBPROGRAM IS A SLIGHT MODIFICATION OF A SUBPROGRAM FROM THE C. 1979 AERE HARWELL LIBRARY. THE NAME OF THE CORRESPONDING HARWELL CODE CAN BE OBTAINED BY DELETING THE FINAL LETTER =D= IN THE NAMES USED HERE. REVISIONS MADE BY R J HANSON, SNLA, AUGUST, 1979. REVISED SEP. 13, 1979. ROYALTIES HAVE BEEN PAID TO AERE-UK FOR USE OF THEIR CODES IN THE PACKAGE GIVEN HERE. ANY PRIMARY USAGE OF THE HARWELL SUBROUTINES REQUIRES A ROYALTY AGREEMENT AND PAYMENT BETWEEN THE USER AND AERE-UK. ANY USAGE OF THE SANDIA WRITTEN CODES DSPLP( ) (WHICH USES THE HARWELL SUBROUTINES) IS PERMITTED. IP(I,1),IP(I,2) POINT TO THE START OF ROW/COL I. IW(I,1),IW(I,2) HOLD THE NUMBER OF NON-ZEROS IN ROW/COL I. DURING THE MAIN BODY OF THIS SUBROUTINE THE VECTORS IW(.,3),IW(.,5), IW(.,7) ARE USED TO HOLD DOUBLY LINKED LISTS OF ROWS THAT HAVE NOT BEEN PIVOTAL AND HAVE EQUAL NUMBERS OF NON-ZEROS. IW(.,4),IW(.,6),IW(.,8) HOLD SIMILAR LISTS FOR THE COLUMNS. IW(I,3),IW(I,4) HOLD FIRST ROW/COLUMN TO HAVE I NON-ZEROS OR ZERO IF THERE ARE NONE. IW(I,5), IW(I,6) HOLD ROW/COL NUMBER OF ROW/COL PRIOR TO ROW/COL I IN ITS LIST, OR ZERO IF NONE. IW(I,7), IW(I,8) HOLD ROW/COL NUMBER OF ROW/COL AFTER ROW/COL I IN ITS LIST, OR ZERO IF NONE. FOR ROWS/COLS THAT HAVE BEEN PIVOTAL IW(I,5),IW(I,6) HOLD NEGATION OF POSITION OF ROW/COL I IN THE PIVOTAL ORDERING.

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

- Canonical provider: `main-src/src/la05ad.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/la05ad.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/la05ad.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/la05ad.f) — `verified_cached`
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
