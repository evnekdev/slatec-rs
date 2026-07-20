# RFFTI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Initialize a work array for RFFTF and RFFTB.

## Description

******************************************************************** * NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE * ******************************************************************** * * * This routine uses non-standard Fortran 77 constructs and will * * be removed from the library at a future date. You are * * requested to use RFFTI1. * * * ******************************************************************** Subroutine RFFTI initializes the array WSAVE which is used in both RFFTF and RFFTB. The prime factorization of N together with a tabulation of the trigonometric functions are computed and stored in WSAVE. Input Argument N the length of the sequence to be transformed. Output Argument WSAVE a work array which must be dimensioned at least 2*N+15. The same work array can be used for both RFFTF and RFFTB as long as N remains unchanged. Different WSAVE arrays are required for different values of N. The contents of WSAVE must not be changed between calls of RFFTF or RFFTB.

## Classification

- Historical role: `subsidiary`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `FFTPACK transforms`
- Mathematical domain: `transforms`
- Package provenance: `fftpack`
- GAMS classifications: `J1A1`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::fftpack::RealFftPlan::new`

## Providers

- Canonical provider: `fishfft/rffti.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/rffti.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/rffti.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `preexisting_family_declaration_requires_r1_review`
- Canonical Rust path: `not_promoted`
- Current legacy Rust paths: `slatec_sys::fftpack::rffti`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::fftpack::RealFftPlan::new`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
