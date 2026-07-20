# CFFTI1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Initialize a real and an integer work array for CFFTF1 and CFFTB1.

## Description

Subroutine CFFTI1 initializes the work arrays WA and IFAC which are used in both CFFTF1 and CFFTB1. The prime factorization of N and a tabulation of the trigonometric functions are computed and stored in IFAC and WA, respectively. Input Parameter N the length of the sequence to be transformed Output Parameters WA a real work array which must be dimensioned at least 2*N. IFAC an integer work array which must be dimensioned at least 15. The same work arrays can be used for both CFFTF1 and CFFTB1 as long as N remains unchanged. Different WA and IFAC arrays are required for different values of N. The contents of WA and IFAC must not be changed between calls of CFFTF1 or CFFTB1.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `FFTPACK transforms`
- Mathematical domain: `transforms`
- Package provenance: `fftpack`
- GAMS classifications: `J1A2`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::transforms::fft::complex::ComplexFftPlan32::new`

## Providers

- Canonical provider: `fishfft/cffti1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cffti1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::fftpack::numerical::cffti1`
- Current legacy Rust paths: `slatec_sys::fftpack_complex::cffti1`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::transforms::fft::complex::ComplexFftPlan32::new`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
