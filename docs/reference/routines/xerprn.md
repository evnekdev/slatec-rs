# XERPRN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Print error messages processed by XERMSG.

## Description

This routine sends one or more lines to each of the (up to five) logical units to which error messages are to be sent. This routine is called several times by XERMSG, sometimes with a single line to print and sometimes with a (potentially very long) message that may wrap around into multiple lines. PREFIX Input argument of type CHARACTER. This argument contains characters to be put at the beginning of each line before the body of the message. No more than 16 characters of PREFIX will be used. NPREF Input argument of type INTEGER. This argument is the number of characters to use from PREFIX. If it is negative, the intrinsic function LEN is used to determine its length. If it is zero, PREFIX is not used. If it exceeds 16 or if LEN(PREFIX) exceeds 16, only the first 16 characters will be used. If NPREF is positive and the length of PREFIX is less than NPREF, a copy of PREFIX extended with blanks to length NPREF will be used. MESSG Input argument of type CHARACTER. This is the text of a message to be printed. If it is a long message, it will be broken into pieces for printing on multiple lines. Each line will start with the appropriate prefix and be followed by a piece of the message. NWRAP is the number of characters per piece; that is, after each NWRAP characters, we break and start a new line. In addition the characters '$$' embedded in MESSG are a sentinel for a new line. The counting of characters up to NWRAP starts over for each new line. The value of NWRAP typically used by XERMSG is 72 since many older error messages in the SLATEC Library are laid out to rely on wrap-around every 72 characters. NWRAP Input argument of type INTEGER. This gives the maximum size piece into which to break MESSG for printing on multiple lines. An embedded '$$' ends a line, and the count restarts at the following character. If a line break does not occur on a blank (it would split a word) that word is moved to the next line. Values of NWRAP less than 16 will be treated as 16. Values of NWRAP greater than 132 will be treated as 132. The actual line length will be NPREF + NWRAP after NPREF has been adjusted to fall between 0 and 16 and NWRAP has been adjusted to fall between 16 and 132.

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
- GAMS classifications: `R3C`
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

- Canonical provider: `main-src/src/xerprn.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/xerprn.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/xerprn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/xerprn.f) — `verified_cached`
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
