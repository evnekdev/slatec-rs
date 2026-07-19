# MPMUL

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Subsidiary to DQDOTA and DQDOTI

## Description

Multiplies X and Y, returning result in Z, for 'mp' X, Y and Z. The simple o(t**2) algorithm is used, with four guard digits and R*-rounding. Advantage is taken of zero digits in X, but not in Y. Asymptotically faster algorithms are known (see Knuth, VOL. 2), but are difficult to implement in FORTRAN in an efficient and machine-independent manner. In comments to other 'mp' routines, M(t) is the time to perform t-digit 'mp' multiplication. Thus M(t) = o(t**2) with the present version of MPMUL, but M(t) = o(t.log(t).log(log(t))) is theoretically possible. The arguments X(*), Y(*), and Z(*), and the variable R in COMMON are all INTEGER arrays of size 30. See the comments in the routine MPBLAS for the reason for this choice.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- Family evidence: `parent_inheritance` (`high`)
- Parent-family evidence: `DQDOTA, DQDOTI`

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/mpmul.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/mpmul.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/mpmul.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/mpmul.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
