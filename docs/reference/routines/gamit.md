# GAMIT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate Tricomi's form of the incomplete Gamma function.

## Description

Evaluate Tricomi's incomplete gamma function defined by GAMIT = X**(-A)/GAMMA(A) * integral from 0 to X of EXP(-T) * T**(A-1.) for A .GT. 0.0 and by analytic continuation for A .LE. 0.0. GAMMA(X) is the complete gamma function of X. GAMIT is evaluated for arbitrary real values of A and for nonnegative values of X (even though GAMIT is defined for X .LT. 0.0), except that for X = 0 and A .LE. 0.0, GAMIT is infinite, which is a fatal error. The function and both arguments are REAL. A slight deterioration of 2 or 3 digits accuracy will occur when GAMIT is very large or very small in absolute value, because logarithmic variables are used. Also, if the parameter A is very close to a negative integer (but not a negative integer), there is a loss of accuracy, which is reported if the result is less than half machine precision.

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
- GAMS classifications: `C7E`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::special::gamma::tricomi_incomplete_gamma_f32`

## Providers

- Canonical provider: `fnlib/gamit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/gamit.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/gamit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
