# ZBESH

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute a sequence of the Hankel functions H(m,a,z) for superscript m=1 or 2, real nonnegative orders a=b, b+1,... where b>0, and nonzero complex argument z. A scaling option is available to help avoid overflow.

## Description

***A DOUBLE PRECISION ROUTINE*** On KODE=1, ZBESH computes an N member sequence of complex Hankel (Bessel) functions CY(L)=H(M,FNU+L-1,Z) for superscript M=1 or 2, real nonnegative orders FNU+L-1, L=1,..., N, and complex nonzero Z in the cut plane -pi<arg(Z)<=pi where Z=ZR+i*ZI. On KODE=2, CBESH returns the scaled functions CY(L) = H(M,FNU+L-1,Z)*exp(-(3-2*M)*Z*i), i**2=-1 which removes the exponential behavior in both the upper and lower half planes. Definitions and notation are found in the NBS Handbook of Mathematical Functions (Ref. 1).

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f64`
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

- Canonical provider: `main-src/src/zbesh.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/zbesh.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/zbesh.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/zbesh.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `not_reviewed_by_raw_api_registry`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `none`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `not_documented`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
