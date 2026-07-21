# CFFTF1

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the forward transform of a complex, periodic sequence.

## Description

Subroutine CFFTF1 computes the forward complex discrete Fourier transform (the Fourier analysis). Equivalently, CFFTF1 computes the Fourier coefficients of a complex periodic sequence. The transform is defined below at output parameter C. The transform is not normalized. To obtain a normalized transform the output must be divided by N. Otherwise a call of CFFTF1 followed by a call of CFFTB1 will multiply the sequence by N. The arrays WA and IFAC which are used by subroutine CFFTB1 must be initialized by calling subroutine CFFTI1 (N, WA, IFAC).

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

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CFFTF1](https://www.netlib.org/slatec/fishfft/cfftf1.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | the length of the complex sequence C. The method is more efficient when N is the product of small primes. |
| 2 | `C` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a complex array of length N which contains the sequence For J=1,. ,N the sum from K=1,. ,N of C(K)*EXP(-I*(J-1)*(K-1)*2*PI/N) where I=SQRT(-1) NOTE: WA and IFAC contain initialization calculations which must not be destroyed between calls of subroutine CFFTF1 or CFFTB1. |
| 3 | `CH` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a real work array of length at least 2*N. |
| 4 | `WA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a real work array which must be dimensioned at least 2*N. |
| 5 | `IFAC` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | an integer work array which must be dimensioned at least 15. The WA and IFAC arrays must be initialized by calling subroutine CFFTI1 (N, WA, IFAC), and different WA and IFAC arrays must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WA and IFAC arrays can be used by CFFTF1 and CFFTB1. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::fftpack::cfftf1`. Native symbol: `cfftf1_`. Declaration feature: `fftpack`. Provider feature: `fftpack-extended-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::cfftf1`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::transforms::fft::complex::ComplexFftPlan32::forward`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
