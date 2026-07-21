# EZFFTF

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute a simplified real, periodic, fast Fourier forward transform.

## Description

Subroutine EZFFTF computes the Fourier coefficients of a real periodic sequence (Fourier analysis). The transform is defined below at Output Parameters AZERO, A and B. EZFFTF is a simplified but slower version of RFFTF.

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [EZFFTF](https://www.netlib.org/slatec/fishfft/ezfftf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | the length of the array R to be transformed. The method is most efficient when N is the product of small primes. |
| 2 | `R` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a real array of length N which contains the sequence to be transformed. R is not destroyed. |
| 3 | `AZERO` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | the sum from I=1 to I=N of R(I)/N. |
| 4 | `A` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | for N even B(N/2)=0. and A(N/2) is the sum from I=1 to I=N of (-1)**(I-1)*R(I)/N for N even define KMAX=N/2-1 for N odd define KMAX=(N-1)/2 then for K=1,. ,KMAX A(K) equals the sum from I=1 to I=N of 2. /N*R(I)*COS(K*(I-1)*2*PI/N) B(K) equals the sum from I=1 to I=N of 2. /N*R(I)*SIN(K*(I-1)*2*PI/N). |
| 5 | `B` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | for N even B(N/2)=0. and A(N/2) is the sum from I=1 to I=N of (-1)**(I-1)*R(I)/N for N even define KMAX=N/2-1 for N odd define KMAX=(N-1)/2 then for K=1,. ,KMAX A(K) equals the sum from I=1 to I=N of 2. /N*R(I)*COS(K*(I-1)*2*PI/N) B(K) equals the sum from I=1 to I=N of 2. /N*R(I)*SIN(K*(I-1)*2*PI/N). |
| 6 | `WSAVE` | `workspace-output` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTF. The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and workspace requirements

`WSAVE`: a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTF. The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N. This initialization does not have to be repeated so long as N remains unchanged. Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::fftpack::ezfftf`. Native symbol: `ezfftf_`. Declaration feature: `fftpack`. Provider feature: `fftpack-extended-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::ezfftf`
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
