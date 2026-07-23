# QPSRT

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to QAGE, QAGIE, QAGPE, QAGSE, QAWCE, QAWOE and QAWSE

## Description

1. QPSRT Ordering Routine Standard FORTRAN Subroutine REAL Version 2. PURPOSE This routine maintains the descending ordering in the list of the local error estimates resulting from the interval subdivision process. At each call two error estimates are inserted using the sequential search method, top-down for the largest error estimate and bottom-up for the smallest error estimate. 3. CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... , ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - INTEGER MAXERR = IORD(NRMAX)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `QAGE, QAGIE, QAGPE, QAGSE, QAWCE, QAWOE, QAWSE`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/qpsrt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/qpsrt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/qpsrt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `subsidiary_minimal`
- Description provenance: `source_prologue`
- Assessment: the non-public subsidiary has purpose, role, source, and disposition evidence
- Dedicated family page: [Numerical quadrature](../families/numerical-quadrature.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `LIMIT` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LAST` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MAXERR` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ERMAX` | output | `REAL` (`explicit`) | `*mut f32` | scalar | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELIST` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IORD` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NRMAX` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... | CALLING SEQUENCE CALL QPSRT(LIMIT,LAST,MAXERR,ERMAX,ELIST,IORD,NRMAX) PARAMETERS (MEANING AT OUTPUT) LIMIT - INTEGER Maximum number of error estimates the list can contain LAST - INTEGER Number of error estimates currently in the list MAXERR - INTEGER MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - REAL NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - REAL Vector of dimension LAST containing the error estimates IORD - INTEGER Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),... Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `internal-subsidiary`
- ABI validation: `compiler-validated`
- Canonical Rust path: `not_promoted`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
