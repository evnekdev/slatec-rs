# DBSK0E

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the exponentially scaled modified (hyperbolic) Bessel function of the third kind of order zero.

## Description

DBSK0E(X) computes the double precision exponentially scaled modified (hyperbolic) Bessel function of the third kind of order zero for positive double precision argument X. Series for BK0 on the interval 0. to 4.00000E+00 with weighted error 3.08E-33 log weighted error 32.51 significant figures required 32.05 decimal places required 33.11 Series for AK0 on the interval 1.25000E-01 to 5.00000E-01 with weighted error 2.85E-32 log weighted error 31.54 significant figures required 30.19 decimal places required 32.33 Series for AK02 on the interval 0. to 1.25000E-01 with weighted error 2.30E-32 log weighted error 31.64 significant figures required 29.68 decimal places required 32.40

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
- Safe Rust paths: `slatec::special::bessel::bessel_k0_scaled`

## Providers

- Canonical provider: `fnlib/dbsk0e.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/dbsk0e.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/dbsk0e.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
