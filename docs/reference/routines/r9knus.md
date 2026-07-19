# R9KNUS

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute Bessel functions EXP(X)*K-SUB-XNU(X) and EXP(X)* K-SUB-XNU+1(X) for 0.0 .LE. XNU .LT. 1.0.

## Description

Compute Bessel functions EXP(X) * K-sub-XNU (X) and EXP(X) * K-sub-XNU+1 (X) for 0.0 .LE. XNU .LT. 1.0 . Series for C0K on the interval 0. to 2.50000D-01 with weighted error 1.60E-17 log weighted error 16.79 significant figures required 15.99 decimal places required 17.40 Series for ZNU1 on the interval -7.00000D-01 to 0. with weighted error 1.43E-17 log weighted error 16.85 significant figures required 16.08 decimal places required 17.38

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10B3`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `fnlib/r9knus.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fnlib/r9knus.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fnlib/r9knus.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fnlib/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
