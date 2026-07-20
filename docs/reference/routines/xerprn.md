# XERPRN

[Family: Error handling](../families/error-handling.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

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

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Error handling](../families/error-handling.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `PREFIX` | input | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | PREFIX Input argument of type CHARACTER. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NPREF` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NPREF Input argument of type INTEGER. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MESSG` | input | `CHARACTER` (`explicit`) | `*mut core::ffi::c_char` | scalar | MESSG Input argument of type CHARACTER. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NWRAP` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | NWRAP is the number of characters per piece; that is, after each NWRAP characters, we break and start a new line. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `support-routine`
- ABI validation: `pending`
- Canonical Rust path: `not_promoted`
- Compatibility aliases: `none`
- Public declaration feature: `not_assigned`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `runtime or machine-support unit is not independently callable`
<!-- raw-api-status:end -->
