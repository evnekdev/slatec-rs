# CBIRY

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the Airy function Bi(z) or its derivative dBi/dz for complex argument z. A scaling option is available to help avoid overflow.

## Description

On KODE=1, CBIRY computes the complex Airy function Bi(z) or its derivative dBi/dz on ID=0 or ID=1 respectively. On KODE=2, a scaling option exp(abs(Re(zeta)))*Bi(z) or exp(abs(Re(zeta)))*dBi/dz is provided to remove the exponential behavior in both the left and right half planes where zeta=(2/3)*z**(3/2). The Airy functions Bi(z) and dBi/dz are analytic in the whole z-plane, and the scaling option does not destroy this property.

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
- GAMS classifications: `C10D`
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

- Canonical provider: `main-src/src/cbiry.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cbiry.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cbiry.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cbiry.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_c_automated_public`
- Canonical Rust path: `slatec_sys::special::complex::cbiry`
- Current legacy Rust paths: `none`
- Public declaration feature: `batch-c-special`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `representative_batch_smoke_only`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
