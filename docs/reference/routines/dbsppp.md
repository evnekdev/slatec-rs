# DBSPPP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Convert the B-representation of a B-spline to the piecewise polynomial (PP) form.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPPP is the BSPLPP routine of the reference. DBSPPP converts the B-representation (T,A,N,K) to the piecewise polynomial (PP) form (C,XI,LXI,K) for use with DPPVAL. Here XI(*), the break point array of length LXI, is the knot array T(*) with multiplicities removed. The columns of the matrix C(I,J) contain the right Taylor derivatives for the polynomial expansion about XI(J) for the intervals XI(J) .LE. X .LE. XI(J+1), I=1,K, J=1,LXI. Function DPPVAL makes this evaluation at a specified point X in XI(1) .LE. X .LE. XI(LXI+1) Description of Arguments Input T,A are double precision T - knot vector of length N+K A - B-spline coefficient vector of length N N - number of B-spline coefficients N = sum of knot multiplicities-K K - order of the B-spline, K .GE. 1 LDC - leading dimension of C, LDC .GE. K Output C,XI,WORK are double precision C - matrix of dimension at least (K,LXI) containing right derivatives at break points XI - XI break point vector of length LXI+1 LXI - number of break points, LXI .LE. N-K+1 WORK - work vector of length K*(N+3) Error Conditions Improper input is a fatal error

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
- GAMS classifications: `E3`
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

- Canonical provider: `main-src/src/dbsppp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbsppp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbsppp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbsppp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::interpolation::numerical::dbsppp`
- Current legacy Rust paths: `slatec_sys::piecewise_polynomial::dbsppp`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
