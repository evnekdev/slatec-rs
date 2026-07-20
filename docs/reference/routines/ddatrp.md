# DDATRP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Interpolation routine for DDASSL.

## Description

THE METHODS IN SUBROUTINE DDASTP USE POLYNOMIALS TO APPROXIMATE THE SOLUTION. DDATRP APPROXIMATES THE SOLUTION AND ITS DERIVATIVE AT TIME XOUT BY EVALUATING ONE OF THESE POLYNOMIALS, AND ITS DERIVATIVE,THERE. INFORMATION DEFINING THIS POLYNOMIAL IS PASSED FROM DDASTP, SO DDATRP CANNOT BE USED ALONE. THE PARAMETERS ARE: X THE CURRENT TIME IN THE INTEGRATION. XOUT THE TIME AT WHICH THE SOLUTION IS DESIRED YOUT THE INTERPOLATED APPROXIMATION TO Y AT XOUT (THIS IS OUTPUT) YPOUT THE INTERPOLATED APPROXIMATION TO YPRIME AT XOUT (THIS IS OUTPUT) NEQ NUMBER OF EQUATIONS KOLD ORDER USED ON LAST SUCCESSFUL STEP PHI ARRAY OF SCALED DIVIDED DIFFERENCES OF Y PSI ARRAY OF PAST STEPSIZE HISTORY

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `ode-dae-families`
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

- Canonical provider: `main-src/src/ddatrp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ddatrp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ddatrp.f) — `verified_cached`
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
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
