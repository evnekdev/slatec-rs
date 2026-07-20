# DBESJ

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute an N member sequence of J Bessel functions J/SUB(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X.

## Description

Abstract **** a double precision routine **** DBESJ computes an N member sequence of J Bessel functions J/sub(ALPHA+K-1)/(X), K=1,...,N for non-negative ALPHA and X. A combination of the power series, the asymptotic expansion for X to infinity and the uniform asymptotic expansion for NU to infinity are applied over subdivisions of the (NU,X) plane. For values of (NU,X) not covered by one of these formulae, the order is incremented or decremented by integer values into a region where one of the formulae apply. Backward recursion is applied to reduce orders by integer values except where the entire sequence lies in the oscillatory region. In this case forward recursion is stable and values from the asymptotic expansion for X to infinity start the recursion when it is efficient to do so. Leading terms of the series and uniform expansion are tested for underflow. If a sequence is requested and the last member would underflow, the result is set to zero and the next lower order tried, etc., until a member comes on scale or all members are set to zero. Overflow cannot occur. The maximum number of significant digits obtainable is the smaller of 14 and the number of digits carried in double precision arithmetic. Description of Arguments Input X,ALPHA are double precision X - X .GE. 0.0D0 ALPHA - order of first member of the sequence, ALPHA .GE. 0.0D0 N - number of members in the sequence, N .GE. 1 Output Y is double precision Y - a vector whose first N components contain values for J/sub(ALPHA+K-1)/(X), K=1,...,N NZ - number of components of Y set to zero due to underflow, NZ=0 , normal return, computation completed NZ .NE. 0, last NZ components of Y set to zero, Y(K)=0.0D0, K=N-NZ+1,...,N. Error Conditions Improper input arguments - a fatal error Underflow - a non-fatal error (NZ .NE. 0)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10A3`
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

- Canonical provider: `main-src/src/dbesj.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbesj.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbesj.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbesj.f) — `verified_cached`
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
