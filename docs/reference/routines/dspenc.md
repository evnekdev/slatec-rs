# DSPENC

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute a form of Spence's integral due to K. Mitchell.

## Description

DSPENC(X) calculates the double precision Spence's integral for double precision argument X. Spence's function defined by integral from 0 to X of -LOG(1-Y)/Y DY. For ABS(X) .LE. 1, the uniformly convergent expansion DSPENC = sum K=1,infinity X**K / K**2 is valid. This is a form of Spence's integral due to K. Mitchell which differs from the definition in the NBS Handbook of Mathematical Functions. Spence's function can be used to evaluate much more general integral forms. For example, integral from 0 to Z of LOG(A*X+B)/(C*X+D) DX = LOG(ABS(B-A*D/C))*LOG(ABS(A*(C*X+D)/(A*D-B*C)))/C - DSPENC (A*(C*Z+D)/(A*D-B*C)) / C. Ref -- K. Mitchell, Philosophical Magazine, 40, p.351 (1949). Stegun and Abromowitz, AMS 55, p.1004. Series for SPEN on the interval 0. to 5.00000E-01 with weighted error 4.74E-32 log weighted error 31.32 significant figures required 30.37 decimal places required 32.11

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
- GAMS classifications: `C5`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::scalar_expanded::spence_integral`

## Providers

- Canonical provider: `fnlib/dspenc.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dspenc.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dspenc.f) — `verified_cached`
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
- Current legacy Rust paths: `slatec_sys::special_scalar_expanded::dspenc`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::special::scalar_expanded::spence_integral`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
