# BSPPP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Convert the B-representation of a B-spline to the piecewise polynomial (PP) form.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract BSPPP is the BSPLPP routine of the reference. BSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with PPVAL. Here XI(*), the break point array of length LXI, is the knot array T(*) with multiplicities removed. The columns of the matrix C(I,J) contain the right Taylor derivatives for the polynomial expansion about XI(J) for the intervals XI(J) .LE. X .LE. XI(J+1), I=1,K, J=1,LXI. Function PPVAL makes this evaluation at a specified point X in XI(1) .LE. X .LE. XI(LXI(1) .LE. X .LE. XI+1) Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
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
- Safe Rust paths: `slatec::interpolation::bspline::BSpline::to_piecewise_polynomial`

## Providers

- Canonical provider: `main-src/src/bsppp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bsppp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bsppp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bsppp.f) — `verified_cached`
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
- Current legacy Rust paths: `slatec_sys::piecewise_polynomial::bsppp`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::interpolation::bspline::BSpline::to_piecewise_polynomial`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
