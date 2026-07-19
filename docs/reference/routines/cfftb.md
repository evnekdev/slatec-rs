# CFFTB

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the unnormalized inverse of CFFTF.

## Description

******************************************************************** * NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE * ******************************************************************** * * * This routine uses non-standard Fortran 77 constructs and will * * be removed from the library at a future date. You are * * requested to use CFFTB1. * * * ******************************************************************** Subroutine CFFTB computes the backward complex discrete Fourier transform (the Fourier synthesis). Equivalently, CFFTB computes a complex periodic sequence from its Fourier coefficients. The transform is defined below at output parameter C. A call of CFFTF followed by a call of CFFTB will multiply the sequence by N. The array WSAVE which is used by subroutine CFFTB must be initialized by calling subroutine CFFTI(N,WSAVE). Input Parameters N the length of the complex sequence C. The method is more efficient when N is the product of small primes. C a complex array of length N which contains the sequence WSAVE a real work array which must be dimensioned at least 4*N+15 in the program that calls CFFTB. The WSAVE array must be initialized by calling subroutine CFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by CFFTF and CFFTB. Output Parameters C For J=1,...,N C(J)=the sum from K=1,...,N of C(K)*EXP(I*(J-1)*(K-1)*2*PI/N) where I=SQRT(-1) WSAVE contains initialization calculations which must not be destroyed between calls of subroutine CFFTF or CFFTB

## Classification

- Historical role: `subsidiary`
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
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `fishfft/cfftb.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cfftb.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/cfftb.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
