# CBESH

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute a sequence of the Hankel functions H(m,a,z) for superscript m=1 or 2, real nonnegative orders a=b, b+1,... where b>0, and nonzero complex argument z. A scaling option is available to help avoid overflow.

## Description

On KODE=1, CBESH computes an N member sequence of complex Hankel (Bessel) functions CY(L)=H(M,FNU+L-1,Z) for superscript M=1 or 2, real nonnegative orders FNU+L-1, L=1,..., N, and complex nonzero Z in the cut plane -pi<arg(Z)<=pi. On KODE=2, CBESH returns the scaled functions CY(L) = H(M,FNU+L-1,Z)*exp(-(3-2*M)*Z*i), i**2=-1 which removes the exponential behavior in both the upper and lower half planes. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Special functions`
- Mathematical domain: `special-functions`
- Package provenance: `unknown`
- GAMS classifications: `C10A4`
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

- Canonical provider: `main-src/src/cbesh.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cbesh.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cbesh.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cbesh.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
