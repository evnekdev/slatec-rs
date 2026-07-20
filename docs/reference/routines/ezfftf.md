# EZFFTF

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute a simplified real, periodic, fast Fourier forward transform.

## Description

Subroutine EZFFTF computes the Fourier coefficients of a real periodic sequence (Fourier analysis). The transform is defined below at Output Parameters AZERO, A and B. EZFFTF is a simplified but slower version of RFFTF. Input Parameters N the length of the array R to be transformed. The method is most efficient when N is the product of small primes. R a real array of length N which contains the sequence to be transformed. R is not destroyed. WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTF. The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. Output Parameters AZERO the sum from I=1 to I=N of R(I)/N A,B for N even B(N/2)=0. and A(N/2) is the sum from I=1 to I=N of (-1)**(I-1)*R(I)/N for N even define KMAX=N/2-1 for N odd define KMAX=(N-1)/2 then for K=1,...,KMAX A(K) equals the sum from I=1 to I=N of 2./N*R(I)*COS(K*(I-1)*2*PI/N) B(K) equals the sum from I=1 to I=N of 2./N*R(I)*SIN(K*(I-1)*2*PI/N)

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
- Safe Rust paths: `slatec::fftpack::EasyRealFftPlan::forward`

## Providers

- Canonical provider: `fishfft/ezfftf.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/ezfftf.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/ezfftf.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [FFTPACK transforms](../families/fftpack-transforms.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | Input Parameters N the length of the array R to be transformed. | Input Parameters N the length of the array R to be transformed. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `R` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Input Parameters N the length of the array R to be transformed. | Input Parameters N the length of the array R to be transformed. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `AZERO` | output | `REAL` (`implicit_rule`) | `*mut f32` | scalar | The transform is defined below at Output Parameters AZERO, A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Subroutine EZFFTF computes the Fourier coefficients of a real periodic sequence (Fourier analysis). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `B` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | The transform is defined below at Output Parameters AZERO, A and B. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WSAVE` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTF. | WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTF. Leading dimension: not established Workspace: WSAVE a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTF. | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::fftpack::ezfftf`. Native symbol: `ezfftf_`. Feature: `fftpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::ezfftf`
- Compatibility aliases: `slatec_sys::fftpack::numerical::ezfftf`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::fftpack::EasyRealFftPlan::forward`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
