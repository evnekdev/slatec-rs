# DPPVAL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of the IDERIV-th derivative of the B-spline from the PP-representation.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract **** a double precision routine **** DPPVAL is the PPVALU function of the reference. DPPVAL calculates (at X) the value of the IDERIV-th derivative of the B-spline from the PP-representation (C,XI,LXI,K). The Taylor expansion about XI(J) for X in the interval XI(J) .LE. X .LT. XI(J+1) is evaluated, J=1,LXI. Right limiting values at X=XI(J) are obtained. DPPVAL will extrapolate beyond XI(1) and XI(LXI+1). To obtain left limiting values (left derivatives) at XI(J) replace LXI by J-1 and set X=XI(J),J=2,LXI+1. Description of Arguments Input C,XI,X are double precision LDC - leading dimension of C matrix, LDC .GE. K C - matrix of dimension at least (K,LXI) containing right derivatives at break points XI(*). XI - break point vector of length LXI+1 LXI - number of polynomial pieces K - order of B-spline, K .GE. 1 IDERIV - order of the derivative, 0 .LE. IDERIV .LE. K-1 IDERIV=0 gives the B-spline value X - argument, XI(1) .LE. X .LE. XI(LXI+1) INPPV - an initialization parameter which must be set to 1 the first time DPPVAL is called. Output DPPVAL is double precision INPPV - INPPV contains information for efficient processing after the initial call and INPPV must not be changed by the user. Distinct splines require distinct INPPV parameters. DPPVAL - value of the IDERIV-th derivative at X Error Conditions Improper input is a fatal error

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
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

- Canonical provider: `main-src/src/dppval.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dppval.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dppval.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dppval.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::piecewise_polynomial::dppval`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
