# RFFTB

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the backward fast Fourier transform of a real coefficient array.

## Description

******************************************************************** * NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE * ******************************************************************** * * * This routine uses non-standard Fortran 77 constructs and will * * be removed from the library at a future date. You are * * requested to use RFFTB1. * * * ******************************************************************** Subroutine RFFTB computes the real periodic sequence from its Fourier coefficients (Fourier synthesis). The transform is defined below at output parameter R. Input Arguments N the length of the array R to be transformed. The method is most efficient when N is a product of small primes. N may change so long as different work arrays are provided. R a real array of length N which contains the sequence to be transformed. WSAVE a work array which must be dimensioned at least 2*N+15 in the program that calls RFFTB. The WSAVE array must be initialized by calling subroutine RFFTI, and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as remains unchanged. Thus subsequent transforms can be obtained faster than the first. Moreover, the same WSAVE array can be used by RFFTF and RFFTB as long as N remains unchanged. Output Argument R For N even and for I = 1,...,N R(I) = R(1)+(-1)**(I-1)*R(N) plus the sum from K=2 to K=N/2 of 2.*R(2*K-2)*COS((K-1)*(I-1)*2*PI/N) -2.*R(2*K-1)*SIN((K-1)*(I-1)*2*PI/N) For N odd and for I = 1,...,N R(I) = R(1) plus the sum from K=2 to K=(N+1)/2 of 2.*R(2*K-2)*COS((K-1)*(I-1)*2*PI/N) -2.*R(2*K-1)*SIN((K-1)*(I-1)*2*PI/N) Note: This transform is unnormalized since a call of RFFTF followed by a call of RFFTB will multiply the input sequence by N. WSAVE contains results which must not be destroyed between calls of RFFTB or RFFTF.

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
- Safe Rust paths: `slatec::fftpack::RealFftPlan::backward`

## Providers

- Canonical provider: `fishfft/rfftb.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/rfftb.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/rfftb.f) — `verified_cached`
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
- Current legacy Rust paths: `slatec_sys::fftpack::rfftb`
- Public declaration feature: `raw-ffi-numeric-array-subroutines`
- `all`-feature reachability: `not_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `legacy_partial_rustdoc`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime-test status: `passed`
- Safe-wrapper status: `slatec::fftpack::RealFftPlan::backward`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
