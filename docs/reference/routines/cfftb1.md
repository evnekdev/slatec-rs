# CFFTB1

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the unnormalized inverse of CFFTF1.

## Description

Subroutine CFFTB1 computes the backward complex discrete Fourier transform (the Fourier synthesis). Equivalently, CFFTB1 computes a complex periodic sequence from its Fourier coefficients. The transform is defined below at output parameter C. A call of CFFTF1 followed by a call of CFFTB1 will multiply the sequence by N. The arrays WA and IFAC which are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). Input Parameters N the length of the complex sequence C. The method is more efficient when N is the product of small primes. C a complex array of length N which contains the sequence CH a real work array of length at least 2*N WA a real work array which must be dimensioned at least 2*N. IFAC an integer work array which must be dimensioned at least 15. The WA and IFAC arrays must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. Output Parameters C For J=1,...,N C(J)=the sum from K=1,...,N of C(K)*EXP(I*(J-1)*(K-1)*2*PI/N) where I=SQRT(-1) NOTE: WA and IFAC contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1

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
- Safe Rust paths: `slatec::transforms::fft::complex::ComplexFftPlan32::backward`

## Providers

- Canonical provider: `fishfft/cfftb1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cfftb1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/cfftb1.f) — `verified_cached`
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
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | A call of CFFTF1 followed by a call of CFFTB1 will multiply the sequence by N. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | The transform is defined below at output parameter C. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CH` | workspace | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | C a complex array of length N which contains the sequence CH a real work array of length at least 2*N WA a real work array which must be dimensioned at least 2*N. | C a complex array of length N which contains the sequence CH a real work array of length at least 2*N WA a real work array which must be dimensioned at least 2*N. Leading dimension: not established Workspace: C a complex array of length N which contains the sequence CH a real work array of length at least 2*N WA a real work array which must be dimensioned at least 2*N. | required; null is not permitted for an ordinary Fortran actual argument |
| `WA` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | The arrays WA and IFAC which are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). | The arrays WA and IFAC which are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IFAC` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The arrays WA and IFAC which are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). | The arrays WA and IFAC which are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::fftpack::cfftb1`. Native symbol: `cfftb1_`. Feature: `fftpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::cfftb1`
- Compatibility aliases: `slatec_sys::fftpack::numerical::cfftb1`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::transforms::fft::complex::ComplexFftPlan32::backward`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
