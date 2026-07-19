# SCOPY

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Copy a vector.

## Description

B L A S Subprogram

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Linear algebra kernels`
- Mathematical domain: `linear-algebra-kernels`
- Package provenance: `unknown`
- GAMS classifications: `D1A5`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::blas::level1::scopy, slatec::blas::level1::scopy_strided`

## Providers

- Canonical provider: `lin/scopy.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/scopy.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `reviewed_public_driver`
- Canonical Rust path: `slatec_sys::blas::level1::scopy`
- Current legacy Rust paths: `slatec_sys::families::blas_level1::scopy`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::blas::level1::scopy`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
