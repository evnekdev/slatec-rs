# WNNLS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve a linearly constrained least squares problem with equality constraints and nonnegativity constraints on selected variables.

## Description

Abstract This subprogram solves a linearly constrained least squares problem. Suppose there are given matrices E and A of respective dimensions ME by N and MA by N, and vectors F and B of respective lengths ME and MA. This subroutine solves the problem EX = F, (equations to be exactly satisfied) AX = B, (equations to be approximately satisfied, in the least squares sense) subject to components L+1,...,N nonnegative Any values ME.GE.0, MA.GE.0 and 0.LE. L .LE.N are permitted. The problem is reposed as problem WNNLS (WT*E)X = (WT*F) ( A) ( B), (least squares) subject to components L+1,...,N nonnegative. The subprogram chooses the heavy weight (or penalty parameter) WT. The parameters for WNNLS are

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Approximation`
- Mathematical domain: `approximation`
- Package provenance: `unknown`
- GAMS classifications: `K1A2A`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::linear_least_squares::solve_nonnegative_least_squares_f32`

## Providers

- Canonical provider: `main-src/src/wnnls.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/wnnls.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/wnnls.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/wnnls.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::approximation::numerical::wnnls`
- Current legacy Rust paths: `slatec_sys::linear_least_squares::wnnls`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::linear_least_squares::solve_nonnegative_least_squares_f32`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
