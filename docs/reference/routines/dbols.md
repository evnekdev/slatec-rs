# DBOLS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Solve the problem E*X = F (in the least squares sense) with bounds on selected X values.

## Description

**** All INPUT and OUTPUT real variables are DOUBLE PRECISION **** The user must have dimension statements of the form: DIMENSION W(MDW,NCOLS+1), BL(NCOLS), BU(NCOLS), * X(NCOLS+NX), RW(5*NCOLS) INTEGER IND(NCOLS), IOPT(1+NI), IW(2*NCOLS) (Here NX=number of extra locations required for option 4; NX=0 for no options; NX=NCOLS if this option is in use. Here NI=number of extra locations required for options 1-6; NI=0 for no options.)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
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
- Safe Rust paths: `slatec::bounded_least_squares::solve_bounded_least_squares`

## Providers

- Canonical provider: `main-src/src/dbols.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dbols.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dbols.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dbols.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::approximation::numerical::dbols`
- Current legacy Rust paths: `slatec_sys::linear_least_squares::dbols`
- Public declaration feature: `approximation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::bounded_least_squares::solve_bounded_least_squares`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
