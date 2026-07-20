# POCH1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate a generalization of Pochhammer's symbol starting from first order.

## Description

Evaluate a generalization of Pochhammer's symbol for special situations that require especially accurate values when X is small in POCH1(A,X) = (POCH(A,X)-1)/X = (GAMMA(A+X)/GAMMA(A) - 1.0)/X . This specification is particularly suited for stably computing expressions such as (GAMMA(A+X)/GAMMA(A) - GAMMA(B+X)/GAMMA(B))/X = POCH1(A,X) - POCH1(B,X) Note that POCH1(A,0.0) = PSI(A) When ABS(X) is so small that substantial cancellation will occur if the straightforward formula is used, we use an expansion due to Fields and discussed by Y. L. Luke, The Special Functions and Their Approximations, Vol. 1, Academic Press, 1969, page 34. The ratio POCH(A,X) = GAMMA(A+X)/GAMMA(A) is written by Luke as (A+(X-1)/2)**X * polynomial in (A+(X-1)/2)**(-2) . In order to maintain significance in POCH1, we write for positive A (A+(X-1)/2)**X = EXP(X*LOG(A+(X-1)/2)) = EXP(Q) = 1.0 + Q*EXPREL(Q) . Likewise the polynomial is written POLY = 1.0 + X*POLY1(A,X) . Thus, POCH1(A,X) = (POCH(A,X) - 1) / X = EXPREL(Q)*(Q/X + Q*POLY1(A,X)) + POLY1(A,X)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `fnlib/poch1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/poch1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/poch1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
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
- Public declaration feature: `raw-ffi-scalar-functions`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
