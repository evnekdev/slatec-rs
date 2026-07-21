# EZFFTF

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute a simplified real, periodic, fast Fourier forward transform.

## Description

Subroutine EZFFTF computes the Fourier coefficients of a real periodic sequence (Fourier analysis). The transform is defined

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

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [EZFFTF](https://www.netlib.org/slatec/fishfft/ezfftf.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | the length of the array R to be transformed.  The method is most efficient when N is the product of small primes. contains the sequence to be transformed.  R is not destroyed. 0. and A(N/2) is the sum from I=1 to 0. and A(N/2) is the sum from I=1 to 1)**(I-1)*R(I)/N 1 1 1)/2 1)/2 then for  K=1,...,KMAX then for  K=1,...,KMAX 1)*2*PI/N) 1)*2*PI/N) |
| 2 | `R` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the sequence to be transformed.  R is not destroyed. 1)*2*PI/N) 1)*2*PI/N) |
| 3 | `AZERO` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | is a simplified but slower version of RFFTF. 1 to I=N of R(I)/N |
| 4 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a simplified but slower version of RFFTF. contains the sequence to be transformed.  R is not destroyed. must be dimensioned at least 3*N+15 in the program that calls EZFFTF.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. 0. and A(N/2) is the sum from I=1 to 1 to I=N of changing dummy array size declarations (1) to (*), |
| 5 | `B` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a simplified but slower version of RFFTF. 0. and A(N/2) is the sum from I=1 to 0. and A(N/2) is the sum from I=1 to 1 to I=N of changing references to intrinsic function FLOAT to REAL. 881128  Modified by Dick Valent to meet prologue standards. 890531  Changed all specific intrinsics to generic.  (WRB) 890531  REVISION DATE from Version 3.2 891214  Prologue converted to Version 4.0 format.  (BAB) 920501  Reformatted the REFERENCES section.  (WRB) |
| 6 | `WSAVE` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | must be dimensioned at least 3*N+15 in the program that calls EZFFTF.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WSAVE`: must be dimensioned at least 3*N+15 in the program that calls EZFFTF.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB.

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
