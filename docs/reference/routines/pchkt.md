# PCHKT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute B-spline knot sequence for PCHBS.

## Description

Set a knot sequence for the B-spline representation of a PCH function with breakpoints X. All knots will be at least double. Endknots are set as: (1) quadruple knots at endpoints if KNOTYP=0; (2) extrapolate the length of end interval if KNOTYP=1; (3) periodic if KNOTYP=2. Input arguments: N, X, KNOTYP. Output arguments: T. Restrictions/assumptions: 1. N.GE.2 . (not checked) 2. X(i).LT.X(i+1), i=1,...,N . (not checked) 3. 0.LE.KNOTYP.LE.2 . (Acts like KNOTYP=0 for any other value.)

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `PCHIP`
- Mathematical domain: `interpolation`
- Package provenance: `pchip`
- GAMS classifications: `E3`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `pchip/pchkt.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/pchip/pchkt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/pchip/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
