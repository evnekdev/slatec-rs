# DGAMRN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DBSKIN

## Description

Abstract * A Double Precision Routine * DGAMRN computes the GAMMA function ratio GAMMA(X)/GAMMA(X+0.5) for real X.gt.0. If X.ge.XMIN, an asymptotic expansion is evaluated. If X.lt.XMIN, an integer is added to X to form a new value of X.ge.XMIN and the asymptotic expansion is evaluated for this new value of X. Successive application of the recurrence relation W(X)=W(X+1)*(1+0.5/X) reduces the argument to its original value. XMIN and computational tolerances are computed as a function of the number of digits carried in a word by calls to I1MACH and D1MACH. However, the computational accuracy is limited to the maximum of unit roundoff (=D1MACH(4)) and 1.0D-18 since critical constants are given to only 18 digits. Input X is Double Precision X - Argument, X.gt.0.0D0 Output DGAMRN is DOUBLE PRECISION DGAMRN - Ratio GAMMA(X)/GAMMA(X+0.5)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DBSKIN`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dgamrn.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dgamrn.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dgamrn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dgamrn.f) — `verified_cached`
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
