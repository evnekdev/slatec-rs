# RFFTI1

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Initialize a real and an integer work array for RFFTF1 and RFFTB1.

## Description

Subroutine RFFTI1 initializes the work arrays WA and IFAC which are used in both RFFTF1 and RFFTB1. The prime factorization of N and a tabulation of the trigonometric functions are computed and stored in IFAC and WA, respectively. Input Argument N the length of the sequence to be transformed. Output Arguments WA a real work array which must be dimensioned at least N. IFAC an integer work array which must be dimensioned at least 15. The same work arrays can be used for both RFFTF1 and RFFTB1 as long as N remains unchanged. Different WA and IFAC arrays are required for different values of N. The contents of WA and IFAC must not be changed between calls of RFFTF1 or RFFTB1.

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

- Canonical provider: `fishfft/rffti1.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/rffti1.f) — `verified_cached`
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
| `N` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | scalar | The prime factorization of N and a tabulation of the trigonometric functions are computed and stored in IFAC and WA, respectively. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WA` | output | `REAL` (`implicit_rule`) | `*mut f32` | rank 1; dimensions (*) | Subroutine RFFTI1 initializes the work arrays WA and IFAC which are used in both RFFTF1 and RFFTB1. | none stated in the separable source sentence Leading dimension: not established Workspace: Subroutine RFFTI1 initializes the work arrays WA and IFAC which are used in both RFFTF1 and RFFTB1. | required; null is not permitted for an ordinary Fortran actual argument |
| `IFAC` | output | `INTEGER` (`implicit_rule`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | Subroutine RFFTI1 initializes the work arrays WA and IFAC which are used in both RFFTF1 and RFFTB1. | none stated in the separable source sentence Leading dimension: not established Workspace: Subroutine RFFTI1 initializes the work arrays WA and IFAC which are used in both RFFTF1 and RFFTB1. | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::fftpack::rffti1`. Native symbol: `rffti1_`. Feature: `fftpack`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::rffti1`
- Compatibility aliases: `slatec_sys::fftpack::numerical::rffti1`
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
