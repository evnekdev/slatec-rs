# CFFTF1

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the forward transform of a complex, periodic sequence.

## Description

Subroutine CFFTF1 computes the forward complex discrete Fourier transform (the Fourier analysis). Equivalently, CFFTF1 computes the Fourier coefficients of a complex periodic sequence. The transform is defined below at output parameter C. The transform is not normalized. To obtain a normalized transform the output must be divided by N. Otherwise a call of CFFTF1 followed by a call of CFFTB1 will multiply the sequence by N. The arrays WA and IFAC which are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). Input Parameters N the length of the complex sequence C. The method is more efficient when N is the product of small primes. C a complex array of length N which contains the sequence CH a real work array of length at least 2*N WA a real work array which must be dimensioned at least 2*N. IFAC an integer work array which must be dimensioned at least 15. The WA and IFAC arrays must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. Output Parameters C For J=1,...,N C(J)=the sum from K=1,...,N of C(K)*EXP(-I*(J-1)*(K-1)*2*PI/N) where I=SQRT(-1) NOTE: WA and IFAC contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1

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
- Safe Rust paths: `slatec::transforms::fft::complex::ComplexFftPlan32::forward`

## Providers

- Canonical provider: `fishfft/cfftf1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cfftf1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/cfftf1.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
