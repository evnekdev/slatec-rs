# PPVAL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of the IDERIV-th derivative of the B-spline from the PP-representation.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract PPVAL is the PPVALU function of the reference. PPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). The Taylor expansion about XI(J) for X in the interval XI(J) .LE. X .LT. XI(J+1) is evaluated, J=1,LXI. Right limiting values at X=XI(J) are obtained. PPVAL will extrapolate beyond XI(1) and XI(LXI+1). To obtain left limiting values (left derivatives) at XI(J), replace LXI by J-1 and set X=XI(J),J=2,LXI+1. Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Interpolation`
- Mathematical domain: `interpolation`
- Package provenance: `unknown`
- GAMS classifications: `E3`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative, slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate, slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::evaluate_into`

## Providers

- Canonical provider: `main-src/src/ppval.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/ppval.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/ppval.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/ppval.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::interpolation::numerical::ppval`
- Current legacy Rust paths: `slatec_sys::piecewise_polynomial::ppval`
- Public declaration feature: `interpolation`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::interpolation::piecewise_polynomial::PiecewisePolynomial::derivative`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
