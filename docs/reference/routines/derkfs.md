# DERKFS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DERKF

## Description

Fehlberg Fourth-Fifth order Runge-Kutta Method ********************************************************************** DERKFS integrates a system of first order ordinary differential equations as described in the comments for DERKF . The arrays YP,F1,F2,F3,F4,F5,and YS (of length at least NEQ) appear in the call list for variable dimensioning purposes. The variables H,TOLFAC,TOLD,DTSIGN,U26,RER,INIT,KSTEPS,KOP,IQUIT, STIFF,NONSTF,NTSTEP, and NSTIFS are used internally by the code and appear in the call list to eliminate local retention of variables between calls. Accordingly, these variables and the array YP should not be altered. Items of possible interest are H - An appropriate step size to be used for the next step TOLFAC - Factor of change in the tolerances YP - Derivative of solution vector at T KSTEPS - Counter on the number of steps attempted **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DERKF`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `identity_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/derkfs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/derkfs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/derkfs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/derkfs.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-callbacks`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `not_tested`
- Runtime-test status: `not_tested`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `callback ABI has compiler-shape evidence but no routine-specific callback contract`
<!-- raw-api-status:end -->
