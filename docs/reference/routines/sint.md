# SINT

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the sine transform of a real, odd sequence.

## Description

Subroutine SINT computes the discrete Fourier sine transform of an odd sequence X(I). The transform is defined below at output parameter X. SINT is the unnormalized inverse of itself since a call of SINT followed by another call of SINT will multiply the input sequence X by 2*(N+1). The array WSAVE which is used by subroutine SINT must be initialized by calling subroutine SINTI(N,WSAVE). Input Parameters N the length of the sequence to be transformed. The method is most efficient when N+1 is the product of small primes. X an array which contains the sequence to be transformed WSAVE a work array with dimension at least INT(3.5*N+16) in the program that calls SINT. The WSAVE array must be initialized by calling subroutine SINTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. Output Parameters X For I=1,...,N X(I)= the sum from K=1 to K=N 2*X(K)*SIN(K*I*PI/(N+1)) A call of SINT followed by another call of SINT will multiply the sequence X by 2*(N+1). Hence SINT is the unnormalized inverse of itself. WSAVE contains initialization calculations which must not be destroyed between calls of SINT.

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
- Safe Rust paths: `slatec::fftpack::SineTransformPlan::transform`

## Providers

- Canonical provider: `fishfft/sint.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/sint.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/sint.f) — `verified_cached`
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
- Current legacy Rust paths: `slatec_sys::fftpack::sint`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::fftpack::SineTransformPlan::transform`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
