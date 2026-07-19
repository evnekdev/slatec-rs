# BSPVN

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Calculate the value of all (possibly) nonzero basis functions at X.

## Description

Written by Carl de Boor and modified by D. E. Amos Abstract BSPVN is the BSPLVN routine of the reference. BSPVN calculates the value of all (possibly) nonzero basis functions at X of order MAX(JHIGH,(J+1)*(INDEX-1)), where T(K) .LE. X .LE. T(N+1) and J=IWORK is set inside the routine on the first call when INDEX=1. ILEFT is such that T(ILEFT) .LE. X .LT. T(ILEFT+1). A call to INTRV(T,N+1,X,ILO,ILEFT, MFLAG) produces the proper ILEFT. BSPVN calculates using the basic algorithm needed in BSPVD. If only basis functions are desired, setting JHIGH=K and INDEX=1 can be faster than calling BSPVD, but extra coding is required for derivatives (INDEX=2) and BSPVD is set up for this purpose. Left limiting values are set up as described in BSPVD. Description of Arguments

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
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
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/bspvn.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/bspvn.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/bspvn.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/bspvn.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
