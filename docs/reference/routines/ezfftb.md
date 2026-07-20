# EZFFTB

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

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

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [FFTPACK transforms](../families/fftpack-transforms.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | Input Parameters N the length of the output array R. | Input Parameters N the length of the output array R. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `R` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | The transform is defined below at Output Parameter R. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AZERO` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | scalar | AZERO the constant Fourier coefficient A,B arrays which contain the remaining Fourier coefficients. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Subroutine EZFFTB computes a real periodic sequence from its Fourier coefficients (Fourier synthesis). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | unavailable | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | AZERO the constant Fourier coefficient A,B arrays which contain the remaining Fourier coefficients. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WSAVE` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | If N is odd, (N-1)/2 locations are required WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTB. | If N is odd, (N-1)/2 locations are required WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTB. Leading dimension: not established Workspace: If N is odd, (N-1)/2 locations are required WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTB. | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::fftpack::ezfftb`. Native symbol: `ezfftb_`. Feature: `fftpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::ezfftb`
- Compatibility aliases: `slatec_sys::fftpack::numerical::ezfftb`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::fftpack::EasyRealFftPlan::backward`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
