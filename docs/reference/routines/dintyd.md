# DINTYD

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DDEBDF

## Description

DINTYD approximates the solution and derivatives at T by polynomial interpolation. Must be used in conjunction with the integrator package DDEBDF. DINTYD computes interpolated values of the K-th derivative of the dependent variable vector Y, and stores it in DKY. This routine is called by DDEBDF with K = 0,1 and T = TOUT, but may also be called by the user for any K up to the current order. (see detailed instructions in LSODE usage documentation.) The computed values in DKY are gotten by interpolation using the Nordsieck history array YH. This array corresponds uniquely to a vector-valued polynomial of degree NQCUR or less, and DKY is set to the K-th derivative of this polynomial at T. The formula for DKY is.. Q DKY(I) = Sum C(J,K) * (T - TN)**(J-K) * H**(-J) * YH(I,J+1) J=K where C(J,K) = J*(J-1)*...*(J-K+1), Q = NQCUR, TN = TCUR, H = HCUR. The quantities NQ = NQCUR, L = NQ+1, N = NEQ, TN, and H are communicated by common. The above sum is done in reverse order. IFLAG is returned negative if either K or T is out of bounds.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DDEBDF`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dintyd.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dintyd.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dintyd.f) — `verified_cached`
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
