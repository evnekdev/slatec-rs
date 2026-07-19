# SCHKW

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

SLAP WORK/IWORK Array Bounds Checker. This routine checks the work array lengths and interfaces to the SLATEC error handler if a problem is found.

## Description

*Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER REAL ERR CALL SCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). Name of the calling routine. This is used in the output message, if an error is detected. LOCIW :IN Integer. Location of the first free element in the integer workspace array. LENIW :IN Integer. Length of the integer workspace array. LOCW :IN Integer. Location of the first free element in the real workspace array. LENRW :IN Integer. Length of the real workspace array. IERR :OUT Integer. Return error flag. IERR = 0 => All went well. IERR = 1 => Insufficient storage allocated for WORK or IWORK. ITER :OUT Integer. Set to zero on return. ERR :OUT Real. Set to the smallest positive magnitude if all went well. Set to a very large number if an error is detected.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Runtime and machine support`
- Mathematical domain: `runtime-support`
- Package provenance: `unknown`
- GAMS classifications: `R2`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `not_bound`
- Build/profile status: `outside_current_immutable_snapshot`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/schkw.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/schkw.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/schkw.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
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
- Exclusion or deferment reason: `source exists but no reviewed or ABI-validated public declaration is recorded`
<!-- raw-api-status:end -->
