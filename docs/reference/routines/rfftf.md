# RFFTF

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

Compute the forward transform of a real, periodic sequence.

## Description

******************************************************************** * NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE NOTICE * ******************************************************************** * * * This routine uses non-standard Fortran 77 constructs and will * * be removed from the library at a future date. You are * * requested to use RFFTF1. * * * ******************************************************************** Subroutine RFFTF computes the Fourier coefficients of a real periodic sequence (Fourier analysis). The transform is defined below at output parameter R. Input Arguments N the length of the array R to be transformed. The method is most efficient when N is a product of small primes. N may change so long as different work arrays are provided. R a real array of length N which contains the sequence to be transformed. WSAVE a work array which must be dimensioned at least 2*N+15 in the program that calls RFFTF. The WSAVE array must be initialized by calling subroutine RFFTI, and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as remains unchanged. Thus subsequent transforms can be obtained faster than the first. Moreover, the same WSAVE array can be used by RFFTF and RFFTB as long as N remains unchanged. Output Argument R R(1) = the sum from I=1 to I=N of R(I) If N is even set L = N/2; if N is odd set L = (N+1)/2 then for K = 2,...,L R(2*K-2) = the sum from I = 1 to I = N of R(I)*COS((K-1)*(I-1)*2*PI/N) R(2*K-1) = the sum from I = 1 to I = N of -R(I)*SIN((K-1)*(I-1)*2*PI/N) If N is even R(N) = the sum from I = 1 to I = N of (-1)**(I-1)*R(I) Note: This transform is unnormalized since a call of RFFTF followed by a call of RFFTB will multiply the input sequence by N. WSAVE contains results which must not be destroyed between calls of RFFTF or RFFTB.

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
- Safe Rust paths: `slatec::fftpack::RealFftPlan::forward`

## Providers

- Canonical provider: `fishfft/rfftf.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/rfftf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/rfftf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

- Enriched from the 20-routine pilot; this catalogue is the canonical corpus view.
