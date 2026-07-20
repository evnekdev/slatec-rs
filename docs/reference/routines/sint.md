# SINT

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

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

<!-- release-readiness:start -->
## Interface documentation quality

- Evidence level: `complete_structured`
- Description provenance: `source_prologue`
- Assessment: the selected source supplies a meaningful description and separable evidence for every argument
- Dedicated family page: [FFTPACK transforms](../families/fftpack-transforms.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | SINT is the unnormalized inverse of itself since a call of SINT followed by another call of SINT will multiply the input sequence X by 2*(N+1). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `X` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Subroutine SINT computes the discrete Fourier sine transform of an odd sequence X(I). | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WSAVE` | input | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | The array WSAVE which is used by subroutine SINT must be initialized by calling subroutine SINTI(N,WSAVE). | The array WSAVE which is used by subroutine SINT must be initialized by calling subroutine SINTI(N,WSAVE). Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::fftpack::sint`. Native symbol: `sint_`. Feature: `fftpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::sint`
- Compatibility aliases: `slatec_sys::fftpack::numerical::sint`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::fftpack::SineTransformPlan::transform`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
