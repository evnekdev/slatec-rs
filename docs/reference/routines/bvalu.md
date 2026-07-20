# BVALU

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Evaluate the B-representation of a B-spline at X for the function value or any of its derivatives.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract BVALU is the BVALUE function of the reference. BVALU evaluates the B-representation (T,A,N,K) of a B-spline at X for the function value on IDERIV = 0 or any of its derivatives on IDERIV = 1,2,...,K-1. Right limiting values (right derivatives) are returned except at the right end point X=T(N+1) where left limiting values are computed. The spline is defined on T(K) .LE. X .LE. T(N+1). BVALU returns a fatal error message when X is outside of this interval. To compute left derivatives or left limiting values at a knot T(I), replace N by I-1 and set X=T(I), I=K+1,N+1. BVALU calls INTRV Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E3`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::interpolation::bspline::BSpline::derivative, slatec::interpolation::bspline::BSpline::evaluate, slatec::interpolation::bspline::BSpline::evaluate_into`

## Providers

- Canonical provider: `main-src/src/bvalu.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bvalu.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bvalu.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bvalu.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::bspline::bvalu`
- Public declaration feature: `raw-ffi-scalar-functions`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::interpolation::bspline::BSpline::derivative`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
