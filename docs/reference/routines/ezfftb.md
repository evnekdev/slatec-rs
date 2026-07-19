# EZFFTB

[Back to family index](../routines-by-family.md) · [Alphabetical index](../routines-alphabetical.md) · [Coverage](../routine-coverage.md)

## Purpose

A simplified real, periodic, backward fast Fourier transform.

## Description

Subroutine EZFFTB computes a real periodic sequence from its Fourier coefficients (Fourier synthesis). The transform is defined below at Output Parameter R. EZFFTB is a simplified but slower version of RFFTB. Input Parameters N the length of the output array R. The method is most efficient when N is the product of small primes. AZERO the constant Fourier coefficient A,B arrays which contain the remaining Fourier coefficients. These arrays are not destroyed. The length of these arrays depends on whether N is even or odd. If N is even, N/2 locations are required. If N is odd, (N-1)/2 locations are required WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTB. The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. Output Parameters R if N is even, define KMAX=N/2 if N is odd, define KMAX=(N-1)/2 Then for I=1,...,N R(I)=AZERO plus the sum from K=1 to K=KMAX of A(K)*COS(K*(I-1)*2*PI/N)+B(K)*SIN(K*(I-1)*2*PI/N) ********************* Complex Notation ************************** For J=1,...,N R(J) equals the sum from K=-KMAX to K=KMAX of C(K)*EXP(I*K*(J-1)*2*PI/N) where C(K) = .5*CMPLX(A(K),-B(K)) for K=1,...,KMAX C(-K) = CONJG(C(K)) C(0) = AZERO and I=SQRT(-1) *************** Amplitude - Phase Notation *********************** For I=1,...,N R(I) equals AZERO plus the sum from K=1 to K=KMAX of ALPHA(K)*COS(K*(I-1)*2*PI/N+BETA(K)) where ALPHA(K) = SQRT(A(K)*A(K)+B(K)*B(K)) COS(BETA(K))=A(K)/ALPHA(K) SIN(BETA(K))=-B(K)/ALPHA(K)

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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
- Safe Rust paths: `slatec::fftpack::EasyRealFftPlan::backward`

## Providers

- Canonical provider: `fishfft/ezfftb.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/ezfftb.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/ezfftb.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.
