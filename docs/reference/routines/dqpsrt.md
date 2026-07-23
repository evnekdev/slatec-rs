# DQPSRT

[Family: Numerical quadrature](../families/numerical-quadrature.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

This routine maintains the descending ordering in the list of the local error estimated resulting from the interval subdivision process. At each call two error estimates are inserted using the sequential search method, top-down for the largest error estimate and bottom-up for the smallest error estimate.

## Description

Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `quadpack`
- Family evidence: `description_inference` (`medium`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dqpsrt.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqpsrt.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqpsrt.f) — `verified_cached`
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
| `LIMIT` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `LAST` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MAXERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ERMAX` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | scalar | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `ELIST` | unavailable | `DOUBLE PRECISION` (`explicit`) | `*mut f64` | rank 1; dimensions (*) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IORD` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `NRMAX` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) | Ordering routine Standard fortran subroutine Double precision version PARAMETERS (MEANING AT OUTPUT) LIMIT - Integer Maximum number of error estimates the list can contain LAST - Integer Number of error estimates currently in the list MAXERR - Integer MAXERR points to the NRMAX-th largest error estimate currently in the list ERMAX - Double precision NRMAX-th largest error estimate ERMAX = ELIST(MAXERR) ELIST - Double precision Vector of dimension LAST containing the error estimates IORD - Integer Vector of dimension LAST, the first K elements of which contain pointers to the error estimates, such that ELIST(IORD(1)),..., ELIST(IORD(K)) form a decreasing sequence, with K = LAST if LAST.LE.(LIMIT/2+2), and K = LIMIT+1-LAST otherwise NRMAX - Integer MAXERR = IORD(NRMAX) Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

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
