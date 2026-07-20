# DBSPDR

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Use the B-representation to construct a divided difference table preparatory to a (right) derivative calculation.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPDR is the BSPLDR routine of the reference. DBSPDR uses the B-representation (T,A,N,K) to construct a divided difference table ADIF preparatory to a (right) derivative calculation in DBSPEV. The lower triangular matrix ADIF is stored in vector AD by columns. The arrays are related by ADIF(I,J) = AD(I-J+1 + (2*N-J+2)*(J-1)/2) I = J,N , J=1,NDERIV. Description of Arguments Input T,A are double precision T - knot vector of length N+K A - B-spline coefficient vector of length N N - number of B-spline coefficients N = sum of knot multiplicities-K K - order of the spline, K .GE. 1 NDERIV - number of derivatives, 1 .LE. NDERIV .LE. K. NDERIV=1 gives the zero-th derivative = function value Output AD is double precision AD - table of differences in a vector of length (2*N-NDERIV+1)*NDERIV/2 for input to DBSPEV Error Conditions Improper input is a fatal error

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

- Canonical provider: `main-src/src/dbspdr.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbspdr.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbspdr.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbspdr.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::interpolation::numerical::dbspdr`
- Current legacy Rust paths: `none`
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
