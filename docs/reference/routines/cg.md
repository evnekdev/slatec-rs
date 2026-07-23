# CG

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a complex general matrix.

## Description

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) of a COMPLEX GENERAL matrix.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4A4`
- Family evidence: `netlib_gams` (`high`)

## Project coverage

- Source status: `provider_present`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `lin/cg.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/cg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/cg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CG](https://www.netlib.org/slatec/lin/cg.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameters, AR, AI, ZR and ZI, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix A=(AR,AI). N is an INTEGER variable. N must be less than or equal to NM. |
| 3 | `AR` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the complex general matrix. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). |
| 4 | `AI` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the complex general matrix. AR and AI are two-dimensional REAL arrays, dimensioned AR(NM,N) and AI(NM,N). |
| 5 | `WR` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | the real and imaginary parts, respectively, of the eigenvalues. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 6 | `WI` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | the real and imaginary parts, respectively, of the eigenvalues. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 7 | `MATZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER variable set equal to zero if only eigenvalues are desired. Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. |
| 8 | `ZR` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the eigenvectors if MATZ is not zero. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and. |
| 9 | `ZI` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | the real and imaginary parts, respectively, of the eigenvectors if MATZ is not zero. ZR and ZI are two-dimensional REAL arrays, dimensioned ZR(NM,N) and. |
| 10 | `FV1` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | one-dimensional REAL arrays used for temporary storage, dimensioned FV1(N), FV2(N), and FV3(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |
| 11 | `FV2` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | one-dimensional REAL arrays used for temporary storage, dimensioned FV1(N), FV2(N), and FV3(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |
| 12 | `FV3` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | one-dimensional REAL arrays used for temporary storage, dimensioned FV1(N), FV2(N), and FV3(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |
| 13 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, 10*N if N is greater than NM, J if the J-th eigenvalue has not been determined after a total of 30 iterations. The eigenvalues should be correct for indices IERR+1, IERR+2,. , N, but no eigenvectors are computed. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::cg`. Native symbol: `cg_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::cg`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
