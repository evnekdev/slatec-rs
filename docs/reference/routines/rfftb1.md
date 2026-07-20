# RFFTB1

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the backward fast Fourier transform of a real coefficient array.

## Description

Subroutine RFFTB1 computes the real periodic sequence from its Fourier coefficients (Fourier synthesis). The transform is defined below at output parameter C. The arrays WA and IFAC which are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Input Arguments N the length of the array R to be transformed. The method is most efficient when N is a product of small primes. N may change so long as different work arrays are provided. C a real array of length N which contains the sequence to be transformed. CH a real work array of length at least N. WA a real work array which must be dimensioned at least N. IFAC an integer work array which must be dimensioned at least 15. The WA and IFAC arrays must be initialized by calling subroutine RFFTI1, and different WA and IFAC arrays must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WA and IFAC arrays can be used by RFFTF1 and RFFTB1. Output Argument C For N even and for I = 1,...,N C(I) = C(1)+(-1)**(I-1)*C(N) plus the sum from K=2 to K=N/2 of 2.*C(2*K-2)*COS((K-1)*(I-1)*2*PI/N) -2.*C(2*K-1)*SIN((K-1)*(I-1)*2*PI/N) For N odd and for I = 1,...,N C(I) = C(1) plus the sum from K=2 to K=(N+1)/2 of 2.*C(2*K-2)*COS((K-1)*(I-1)*2*PI/N) -2.*C(2*K-1)*SIN((K-1)*(I-1)*2*PI/N) Notes: This transform is unnormalized since a call of RFFTF1 followed by a call of RFFTB1 will multiply the input sequence by N. WA and IFAC contain initialization calculations which must not be destroyed between calls of subroutine RFFTF1 or RFFTB1.

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
- GAMS classifications: `J1A1`
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

- Canonical provider: `fishfft/rfftb1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/rfftb1.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/rfftb1.f) — `verified_cached`
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
| `N` | input | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | Input Arguments N the length of the array R to be transformed. | Input Arguments N the length of the array R to be transformed. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `C` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | The transform is defined below at output parameter C. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `CH` | workspace | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | CH a real work array of length at least N. | CH a real work array of length at least N. Leading dimension: not established Workspace: CH a real work array of length at least N. | required; null is not permitted for an ordinary Fortran actual argument |
| `WA` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | The arrays WA and IFAC which are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. | The arrays WA and IFAC which are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IFAC` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The arrays WA and IFAC which are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. | The arrays WA and IFAC which are used by subroutine RFFTB1 must be initialized by calling subroutine RFFTI1. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::fftpack::rfftb1`. Native symbol: `rfftb1_`. Feature: `fftpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::rfftb1`
- Compatibility aliases: `slatec_sys::fftpack::numerical::rfftb1`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
