# DCHKW

[Family: Runtime and machine support](../families/runtime-and-machine-support.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

SLAP WORK/IWORK Array Bounds Checker. This routine checks the work array lengths and interfaces to the SLATEC error handler if a problem is found.

## Description

*Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). Name of the calling routine. This is used in the output message, if an error is detected. LOCIW :IN Integer. Location of the first free element in the integer workspace array. LENIW :IN Integer. Length of the integer workspace array. LOCW :IN Integer. Location of the first free element in the double precision workspace array. LENRW :IN Integer. Length of the double precision workspace array. IERR :OUT Integer. Return error flag. IERR = 0 => All went well. IERR = 1 => Insufficient storage allocated for WORK or IWORK. ITER :OUT Integer. Set to zero on return. ERR :OUT Double Precision. Set to the smallest positive magnitude if all went well. Set to a very large number if an error is detected.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
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

- Canonical provider: `lin/dchkw.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/dchkw.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/dchkw.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Runtime and machine support](../families/runtime-and-machine-support.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `NAME` | output | `UNKNOWN` (`unknown`) | `UNKNOWN` | scalar | *Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LOCIW` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LENIW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LOCW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LENW` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ITER` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | *Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ERR` | output | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | *Usage: CHARACTER*(*) NAME INTEGER LOCIW, LENIW, LOCW, LENW, IERR, ITER DOUBLE PRECISION ERR CALL DCHKW( NAME, LOCIW, LENIW, LOCW, LENW, IERR, ITER, ERR ) *Arguments: NAME :IN Character*(*). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `historical-program`
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
- Exclusion or deferment reason: `source exists but no reviewed or ABI-validated public declaration is recorded`
<!-- raw-api-status:end -->
