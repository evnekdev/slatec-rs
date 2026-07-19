# SINQB

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the unnormalized inverse of SINQF.

## Description

Subroutine SINQB computes the fast Fourier transform of quarter wave data. That is, SINQB computes a sequence from its representation in terms of a sine series with odd wave numbers. the transform is defined below at output parameter X. SINQF is the unnormalized inverse of SINQB since a call of SINQB followed by a call of SINQF will multiply the input sequence X by 4*N. The array WSAVE which is used by subroutine SINQB must be initialized by calling subroutine SINQI(N,WSAVE). Input Parameters N the length of the array X to be transformed. The method is most efficient when N is a product of small primes. X an array which contains the sequence to be transformed WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls SINQB. The WSAVE array must be initialized by calling subroutine SINQI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. Output Parameters X For I=1,...,N X(I)= the sum from K=1 to K=N of 4*X(K)*SIN((2*K-1)*I*PI/(2*N)) a call of SINQB followed by a call of SINQF will multiply the sequence X by 4*N. Therefore SINQF is the unnormalized inverse of SINQB. WSAVE contains initialization calculations which must not be destroyed between calls of SINQB or SINQF.

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
- Safe Rust paths: `slatec::fftpack::QuarterWaveSinePlan::backward`

## Providers

- Canonical provider: `fishfft/sinqb.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/sinqb.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/sinqb.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
