# DPOLCF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the coefficients of the polynomial fit (including Hermite polynomial fits) produced by a previous call to POLINT.

## Description

Abstract Subroutine DPOLCF computes the coefficients of the polynomial fit (including Hermite polynomial fits ) produced by a previous call to DPLINT. The coefficients of the polynomial, expanded about XX, are stored in the array D. The expansion is of the form P(Z) = D(1) + D(2)*(Z-XX) +D(3)*((Z-XX)**2) + ... + D(N)*((Z-XX)**(N-1)). Between the call to DPLINT and the call to DPOLCF the variable N and the arrays X and C must not be altered. ***** INPUT PARAMETERS *** All TYPE REAL variables are DOUBLE PRECISION *** XX - The point about which the Taylor expansion is to be made. N - **** * N, X, and C must remain unchanged between the X - * call to DPLINT and the call to DPOLCF. C - **** ***** OUTPUT PARAMETER *** All TYPE REAL variables are DOUBLE PRECISION *** D - The array of coefficients for the Taylor expansion as explained in the abstract ***** STORAGE PARAMETER WORK - This is an array to provide internal working storage. It must be dimensioned by at least 2*N in the calling program. **** Note - There are two methods for evaluating the fit produced by DPLINT. You may call DPOLVL to perform the task, or you may call DPOLCF to obtain the coefficients of the Taylor expansion and then write your own evaluation scheme. Due to the inherent errors in the computations of the Taylor expansion from the Newton coefficients produced by DPLINT, much more accuracy may be expected by calling DPOLVL as opposed to writing your own scheme.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E1B`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dpolcf.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpolcf.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpolcf.f) — `verified_cached`
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
