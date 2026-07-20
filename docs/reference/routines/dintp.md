# DINTP

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Approximate the solution at XOUT by evaluating the polynomial computed in DSTEPS at XOUT. Must be used in conjunction with DSTEPS.

## Description

The methods in subroutine DSTEPS approximate the solution near X by a polynomial. Subroutine DINTP approximates the solution at XOUT by evaluating the polynomial there. Information defining this polynomial is passed from DSTEPS so DINTP cannot be used alone. Subroutine DSTEPS is completely explained and documented in the text "Computer Solution of Ordinary Differential Equations, the Initial Value Problem" by L. F. Shampine and M. K. Gordon. Input to DINTP -- The user provides storage in the calling program for the arrays in the call list DIMENSION Y(NEQN),YOUT(NEQN),YPOUT(NEQN),PHI(NEQN,16),OY(NEQN) AND ALPHA(12),OG(13),OW(12),GI(11),IV(10) and defines XOUT -- point at which solution is desired. The remaining parameters are defined in DSTEPS and passed to DINTP from that subroutine Output from DINTP -- YOUT(*) -- solution at XOUT YPOUT(*) -- derivative of solution at XOUT The remaining parameters are returned unaltered from their input values. Integration with DSTEPS may be continued.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `ODE solvers`
- Mathematical domain: `ode-dae`
- Package provenance: `unknown`
- GAMS classifications: `I1A1B`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/dintp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dintp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dintp.f) — `verified_cached`
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
