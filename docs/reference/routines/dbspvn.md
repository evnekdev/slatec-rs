# DBSPVN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of all (possibly) nonzero basis functions at X.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DBSPVN is the BSPLVN routine of the reference. DBSPVN calculates the value of all (possibly) nonzero basis functions at X of order MAX(JHIGH,(J+1)*(INDEX-1)), where T(K) .LE. X .LE. T(N+1) and J=IWORK is set inside the routine on the first call when INDEX=1. ILEFT is such that T(ILEFT) .LE. X .LT. T(ILEFT+1). A call to DINTRV(T,N+1,X,ILO,ILEFT,MFLAG) produces the proper ILEFT. DBSPVN calculates using the basic algorithm needed in DBSPVD. If only basis functions are desired, setting JHIGH=K and INDEX=1 can be faster than calling DBSPVD, but extra coding is required for derivatives (INDEX=2) and DBSPVD is set up for this purpose. Left limiting values are set up as described in DBSPVD. Description of Arguments Input T,X are double precision T - knot vector of length N+K, where N = number of B-spline basis functions N = sum of knot multiplicities-K JHIGH - order of B-spline, 1 .LE. JHIGH .LE. K K - highest possible order INDEX - INDEX = 1 gives basis functions of order JHIGH = 2 denotes previous entry with work, IWORK values saved for subsequent calls to DBSPVN. X - argument of basis functions, T(K) .LE. X .LE. T(N+1) ILEFT - largest integer such that T(ILEFT) .LE. X .LT. T(ILEFT+1) Output VNIKX, WORK are double precision VNIKX - vector of length K for spline values. WORK - a work vector of length 2*K IWORK - a work parameter. Both WORK and IWORK contain information necessary to continue for INDEX = 2. When INDEX = 1 exclusively, these are scratch variables and can be used for other purposes. Error Conditions Improper input is a fatal error.

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

- Canonical provider: `main-src/src/dbspvn.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbspvn.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbspvn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbspvn.f) — `verified_cached`
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
