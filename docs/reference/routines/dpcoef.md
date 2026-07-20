# DPCOEF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Convert the DPOLFT coefficients to Taylor series form.

## Description

Abstract DPOLFT computes the least squares polynomial fit of degree L as a sum of orthogonal polynomials. DPCOEF changes this fit to its Taylor expansion about any point C , i.e. writes the polynomial as a sum of powers of (X-C). Taking C=0. gives the polynomial in powers of X, but a suitable non-zero C often leads to polynomials which are better scaled and more accurately evaluated. The parameters for DPCOEF are INPUT -- All TYPE REAL variables are DOUBLE PRECISION L - Indicates the degree of polynomial to be changed to its Taylor expansion. To obtain the Taylor coefficients in reverse order, input L as the negative of the degree desired. The absolute value of L must be less than or equal to NDEG, the highest degree polynomial fitted by DPOLFT . C - The point about which the Taylor expansion is to be made. A - Work and output array containing values from last call to DPOLFT . OUTPUT -- All TYPE REAL variables are DOUBLE PRECISION TC - Vector containing the first LL+1 Taylor coefficients where LL=ABS(L). If L.GT.0 , the coefficients are in the usual Taylor series order, i.e. P(X) = TC(1) + TC(2)*(X-C) + ... + TC(N+1)*(X-C)**N If L .LT. 0, the coefficients are in reverse order, i.e. P(X) = TC(1)*(X-C)**N + ... + TC(N)*(X-C) + TC(N+1)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1A1A2`
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

- Canonical provider: `main-src/src/dpcoef.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dpcoef.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dpcoef.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dpcoef.f) — `verified_cached`
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
