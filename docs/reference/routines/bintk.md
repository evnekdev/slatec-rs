# BINTK

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the B-representation of a spline which interpolates given data.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract BINTK is the SPLINT routine of the reference. BINTK produces the B-spline coefficients, BCOEF, of the B-spline of order K with knots T(I), I=1,...,N+K, which takes on the value Y(I) at X(I), I=1,...,N. The spline or any of its derivatives can be evaluated by calls to BVALU. The I-th equation of the linear system A*BCOEF = B for the coefficients of the interpolant enforces interpolation at X(I)), I=1,...,N. Hence, B(I) = Y(I), all I, and A is a band matrix with 2K-1 bands if A is invertible. The matrix A is generated row by row and stored, diagonal by diagonal, in the rows of Q, with the main diagonal going into row K. The banded system is then solved by a call to BNFAC (which constructs the triangular factorization for A and stores it again in Q), followed by a call to BNSLV (which then obtains the solution BCOEF by substitution). BNFAC does no pivoting, since the total positivity of the matrix A makes this unnecessary. The linear system to be solved is (theoretically) invertible if and only if T(I) .LT. X(I)) .LT. T(I+K), all I. Equality is permitted on the left for I=1 and on the right for I=N when K knots are used at X(1) or X(N). Otherwise, violation of this condition is certain to lead to an error. Description of Arguments

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
- GAMS classifications: `E1A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::interpolation::bspline::BSpline::interpolate_with_knots`

## Providers

- Canonical provider: `main-src/src/bintk.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bintk.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bintk.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bintk.f) — `verified_cached`
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
- Current legacy Rust paths: `slatec_sys::bspline::bintk`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::interpolation::bspline::BSpline::interpolate_with_knots`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
