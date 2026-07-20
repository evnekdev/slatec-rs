# COST

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the cosine transform of a real, even sequence.

## Description

Subroutine COST computes the discrete Fourier cosine transform of an even sequence X(I). The transform is defined below at output parameter X. COST is the unnormalized inverse of itself since a call of COST followed by another call of COST will multiply the input sequence X by 2*(N-1). The transform is defined below at output parameter X. The array WSAVE which is used by subroutine COST must be initialized by calling subroutine COSTI(N,WSAVE). Input Parameters N the length of the sequence X. N must be greater than 1. The method is most efficient when N-1 is a product of small primes. X an array which contains the sequence to be transformed WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls COST. The WSAVE array must be initialized by calling subroutine COSTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. Output Parameters X For I=1,...,N X(I) = X(1)+(-1)**(I-1)*X(N) + the sum from K=2 to K=N-1 2*X(K)*COS((K-1)*(I-1)*PI/(N-1)) A call of COST followed by another call of COST will multiply the sequence X by 2*(N-1). Hence COST is the unnormalized inverse of itself. WSAVE contains initialization calculations which must not be destroyed between calls of COST.

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
- Safe Rust paths: `slatec::fftpack::CosineTransformPlan::transform`

## Providers

- Canonical provider: `fishfft/cost.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cost.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/cost.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Generated raw declaration: `generated_abi_validated`
- Reviewed family declaration: `batch_a_automated_public`
- Canonical Rust path: `slatec_sys::fftpack::numerical::cost`
- Current legacy Rust paths: `slatec_sys::fftpack::cost`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `not_required_batch_a`
- Safe-wrapper status: `slatec::fftpack::CosineTransformPlan::transform`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
