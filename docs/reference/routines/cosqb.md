# COSQB

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the unnormalized inverse cosine transform.

## Description

Subroutine COSQB computes the fast Fourier transform of quarter wave data. That is, COSQB computes a sequence from its representation in terms of a cosine series with odd wave numbers. The transform is defined below at output parameter X. COSQB is the unnormalized inverse of COSQF since a call of COSQB followed by a call of COSQF will multiply the input sequence X by 4*N. The array WSAVE which is used by subroutine COSQB must be initialized by calling subroutine COSQI(N,WSAVE). Input Parameters N the length of the array X to be transformed. The method is most efficient when N is a product of small primes. X an array which contains the sequence to be transformed WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls COSQB. The WSAVE array must be initialized by calling subroutine COSQI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. Output Parameters X For I=1,...,N X(I)= the sum from K=1 to K=N of 2*X(K)*COS((2*K-1)*(I-1)*PI/(2*N)) A call of COSQB followed by a call of COSQF will multiply the sequence X by 4*N. Therefore COSQF is the unnormalized inverse of COSQB. WSAVE contains initialization calculations which must not be destroyed between calls of COSQB or COSQF.

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
- GAMS classifications: `J1A3`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::fftpack::QuarterWaveCosinePlan::backward`

## Providers

- Canonical provider: `fishfft/cosqb.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cosqb.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/cosqb.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::fftpack::numerical::cosqb`
- Current legacy Rust paths: `slatec_sys::fftpack::cosqb`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::fftpack::QuarterWaveCosinePlan::backward`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
