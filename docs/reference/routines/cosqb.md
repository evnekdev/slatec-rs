# COSQB

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the unnormalized inverse cosine transform.

## Description

Subroutine COSQB computes the fast Fourier transform of quarter wave data. That is, COSQB computes a sequence from its representation in terms of a cosine series with odd wave numbers. The transform is defined below at output parameter X. COSQB is the unnormalized inverse of COSQF since a call of COSQB followed by a call of COSQF will multiply the input sequence X by 4*N. The array WSAVE which is used by subroutine COSQB must be initialized by calling subroutine COSQI(N,WSAVE).

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
- GAMS classifications: `J1A3`
- Family evidence: `package_provenance` (`verified`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::fftpack::QuarterWaveCosinePlan::backward`

## Providers

- Canonical provider: `fishfft/cosqb.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/fishfft/cosqb.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/fishfft/cosqb.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/fishfft/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [COSQB](https://www.netlib.org/slatec/fishfft/cosqb.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | the length of the array X to be transformed.  The method is most efficient when N is a product of small primes. |
| 2 | `X` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | an array which contains the sequence to be transformed 1,...,N the sum from K=1 to K=N of 1)*(I-1)*PI/(2*N)) A call of COSQB followed by a call of COSQF will multiply the sequence X by 4*N. Therefore COSQF is the unnormalized inverse of COSQB. |
| 3 | `WSAVE` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a work array which must be dimensioned at least 3*N+15 in the program that calls COSQB.  The WSAVE array must be initialized by calling subroutine COSQI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of COSQB or COSQF. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WSAVE`: a work array which must be dimensioned at least 3*N+15 in the program that calls COSQB.  The WSAVE array must be initialized by calling subroutine COSQI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. contains initialization calculations which must not be destroyed between calls of COSQB or COSQF.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::fftpack::cosqb`. Native symbol: `cosqb_`. Declaration feature: `fftpack`. Provider feature: `fftpack-extended-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::cosqb`
- Public declaration feature: `fftpack`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `slatec::fftpack::QuarterWaveCosinePlan::backward`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
