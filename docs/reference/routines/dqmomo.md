# DQMOMO

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

This routine computes modified Chebyshev moments. The K-th modified Chebyshev moment is defined as the integral over (-1,1) of W(X)*T(K,X), where T(K,X) is the Chebyshev polynomial of degree K.

## Description

MODIFIED CHEBYSHEV MOMENTS STANDARD FORTRAN SUBROUTINE DOUBLE PRECISION VERSION PARAMETERS ALFA - Double precision Parameter in the weight function W(X), ALFA.GT.(-1) BETA - Double precision Parameter in the weight function W(X), BETA.GT.(-1) RI - Double precision Vector of dimension 25 RI(K) is the integral over (-1,1) of (1+X)**ALFA*T(K-1,X), K = 1, ..., 25. RJ - Double precision Vector of dimension 25 RJ(K) is the integral over (-1,1) of (1-X)**BETA*T(K-1,X), K = 1, ..., 25. RG - Double precision Vector of dimension 25 RG(K) is the integral over (-1,1) of (1+X)**ALFA*LOG((1+X)/2)*T(K-1,X), K = 1, ..., 25. RH - Double precision Vector of dimension 25 RH(K) is the integral over (-1,1) of (1-X)**BETA*LOG((1-X)/2)*T(K-1,X), K = 1, ..., 25. INTEGR - Integer Input parameter indicating the modified Moments to be computed INTEGR = 1 compute RI, RJ = 2 compute RI, RJ, RG = 3 compute RI, RJ, RH = 4 compute RI, RJ, RG, RH

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Numerical quadrature`
- Mathematical domain: `quadrature`
- Package provenance: `unknown`
- GAMS classifications: `H2A2A1`
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

- Canonical provider: `main-src/src/dqmomo.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dqmomo.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dqmomo.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::quadrature::numerical::dqmomo`
- Current legacy Rust paths: `none`
- Public declaration feature: `quadrature`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
