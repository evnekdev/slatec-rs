# EZFFTB

[Family: FFTPACK transforms](../families/fftpack-transforms.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

A simplified real, periodic, backward fast Fourier transform.

## Description

Subroutine EZFFTB computes a real periodic sequence from its Fourier coefficients (Fourier synthesis). The transform is defined below at Output Parameter R. EZFFTB is a simplified but slower version of RFFTB.

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

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [EZFFTB](https://www.netlib.org/slatec/fishfft/ezfftb.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | 1)/2 locations are required 1)/2 locations are required WSAVE   a work array which must be dimensioned at least 3*N+15 WSAVE   a work array which must be dimensioned at least 3*N+15 in the program that calls EZFFTB.  The WSAVE array must be in the program that calls EZFFTB.  The WSAVE array must be initialized by calling subroutine EZFFTI(N,WSAVE), and a initialized by calling subroutine EZFFTI(N,WSAVE), and a different WSAVE array must be used for each different different WSAVE array must be used for each different value of N.  This initialization does not have to be value of N.  This initialization does not have to be repeated so long as N remains unchanged.  Thus subsequent repeated so long as N remains unchanged.  Thus subsequent transforms can be obtained faster than the first. transforms can be obtained faster than the first. The same WSAVE array can be used by EZFFTF and EZFFTB. The same WSAVE array can be used by EZFFTF and EZFFTB. N/2 1)/2 1)/2 Then for I=1,...,N Then for I=1,...,N |
| 2 | `R` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | N/2 AZERO plus the sum from K=1 to K=KMAX of KMAX to K=KMAX of C(K)*EXP(I*K*(J-1)*2*PI/N) where 1 to K=KMAX of ALPHA(K)*COS(K*(I-1)*2*PI/N+BETA(K)) where ALPHA(K) = SQRT(A(K)*A(K)+B(K)*B(K)) COS(BETA(K))=A(K)/ALPHA(K) SIN(BETA(K))=-B(K)/ALPHA(K) |
| 3 | `AZERO` | `output` | `scalar` | `REAL` | `*mut f32` | scalar | 1 to K=KMAX of ALPHA(K)*COS(K*(I-1)*2*PI/N+BETA(K)) where ALPHA(K) = SQRT(A(K)*A(K)+B(K)*B(K)) COS(BETA(K))=A(K)/ALPHA(K) SIN(BETA(K))=-B(K)/ALPHA(K) |
| 4 | `A` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | 1)*2*PI/N)+B(K)*SIN(K*(I-1)*2*PI/N) Complex Notation ************************** For J=1,...,N B(K))   for K=1,...,KMAX C(-K) = CONJG(C(K)) C(0) = AZERO and I=SQRT(-1) Amplitude - Phase Notation *********************** For I=1,...,N |
| 5 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `WSAVE` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`WSAVE`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::fftpack::ezfftb`. Native symbol: `ezfftb_`. Declaration feature: `fftpack`. Provider feature: `fftpack-extended-real`. ABI fingerprint: `subroutine:void(mut_i32,mut_f32_ptr_rank1,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::fftpack::ezfftb`
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
