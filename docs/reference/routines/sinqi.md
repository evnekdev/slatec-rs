# SINQI

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Initialize a work array for SINQF and SINQB.

## Description

Subroutine SINQI initializes the array WSAVE which is used in both SINQF and SINQB. The prime factorization of N together with a tabulation of the trigonometric functions are computed and stored in WSAVE. Input Parameter N the length of the sequence to be transformed. The method is most efficient when N is a product of small primes. Output Parameter WSAVE a work array which must be dimensioned at least 3*N+15. The same work array can be used for both SINQF and SINQB as long as N remains unchanged. Different WSAVE arrays are required for different values of N. The contents of WSAVE must not be changed between calls of SINQF or SINQB.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `FFTPACK transforms`
- Mathematical domain: `transforms`
- Package provenance: `fftpack`
- GAMS classifications: `J1A3`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::fftpack::QuarterWaveSinePlan::new`

## Providers

- Canonical provider: `fishfft/sinqi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/sinqi.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/sinqi.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::fftpack::numerical::sinqi`
- Current legacy Rust paths: `slatec_sys::fftpack::sinqi`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::fftpack::QuarterWaveSinePlan::new`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
