# BVDER

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to BVSUP

## Description

********************************************************************** NFC = Number of base solution vectors NCOMP = Number of components per solution vector 1 -- Nonzero particular solution INHOMO = 2 or 3 -- Zero particular solution 0 -- Inhomogeneous vector term G(X) identically zero IGOFX = 1 -- Inhomogeneous vector term G(X) not identically zero G = Inhomogeneous vector term G(X) XSAV = Previous value of X C = Normalization factor for the particular solution 0 ( if NEQIVP = 0 ) IVP = Number of differential equations integrated due to the original boundary value problem ( if NEQIVP .GT. 0 ) NOFST - For problems with auxiliary initial value equations, NOFST communicates to the routine FMAT how to access the dependent variables corresponding to this initial value problem. For example, during any call to FMAT, the first dependent variable for the initial value problem is in position Y(NOFST + 1). See example in SAND77-1328. **********************************************************************

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `BVSUP`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/bvder.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bvder.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bvder.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bvder.f) — `verified_cached`
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
