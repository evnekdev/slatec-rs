# DBSI0E

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero.

## Description

DBSI0E(X) calculates the double precision exponentially scaled modified (hyperbolic) Bessel function of the first kind of order zero for double precision argument X. The result is the Bessel function I0(X) multiplied by EXP(-ABS(X)). Series for BI0 on the interval 0. to 9.00000E+00 with weighted error 9.51E-34 log weighted error 33.02 significant figures required 33.31 decimal places required 33.65 Series for AI0 on the interval 1.25000E-01 to 3.33333E-01 with weighted error 2.74E-32 log weighted error 31.56 significant figures required 30.15 decimal places required 32.39 Series for AI02 on the interval 0. to 1.25000E-01 with weighted error 1.97E-32 log weighted error 31.71 significant figures required 30.15 decimal places required 32.63

## Classification

- Historical role: `user_callable`
- Program-unit kind: `function`
- Identity kind: `function`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B1`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::bessel::bessel_i0_scaled`

## Providers

- Canonical provider: `fnlib/dbsi0e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dbsi0e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dbsi0e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::families::dbsi0e`
- Public declaration feature: `raw-ffi-scalar-functions`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::bessel::bessel_i0_scaled`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
