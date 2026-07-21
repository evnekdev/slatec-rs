# RGG

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and eigenvectors for a real generalized eigenproblem.

## Description

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) for the REAL GENERAL GENERALIZED eigenproblem Ax = (LAMBDA)Bx. On Input NM must be set to the row dimension of the two-dimensional array parameters, A, B, and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrices A and B. N is an INTEGER variable. N must be less than or equal to NM. A contains a real general matrix. A is a two-dimensional REAL array, dimensioned A(NM,N). B contains a real general matrix. B is a two-dimensional REAL array, dimensioned B(NM,N). MATZ is an INTEGER variable set equal to zero if only eigenvalues are desired. Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. On Output A and B have been destroyed. ALFR and ALFI contain the real and imaginary parts, respectively, of the numerators of the eigenvalues. ALFR and ALFI are one-dimensional REAL arrays, dimensioned ALFR(N) and ALFI(N). BETA contains the denominators of the eigenvalues, which are thus given by the ratios (ALFR+I*ALFI)/BETA. Complex conjugate pairs of eigenvalues appear consecutively with the eigenvalue having the positive imaginary part first. BETA is a one-dimensional REAL array, dimensioned BETA(N). Z contains the real and imaginary parts of the eigenvectors if MATZ is not zero. If the J-th eigenvalue is real, the J-th column of Z contains its eigenvector. If the J-th eigenvalue is complex with positive imaginary part, the J-th and (J+1)-th columns of Z contain the real and imaginary parts of its eigenvector. The conjugate of this vector is the eigenvector for the conjugate eigenvalue. Z is a two-dimensional REAL array, dimensioned Z(NM,N). IERR is an INTEGER flag set to Zero for normal return, 10*N if N is greater than NM, J if the J-th eigenvalue has not been determined after a total of 30*N iterations. The eigenvalues should be correct for indices IERR+1, IERR+2, ..., N, but no eigenvectors are computed. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY ------------------------------------------------------------------

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f32`
- Scalar kind: `real`
- Primary family: `Eigenvalue problems`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D4B2`
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

- Canonical provider: `lin/rgg.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/rgg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/rgg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [RGG](https://www.netlib.org/slatec/lin/rgg.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | Array argument classified by fixed-form executable read/write analysis. |
| 4 | `B` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `ALFR` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 6 | `ALFI` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 7 | `BETA` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 8 | `MATZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 9 | `Z` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | Array argument classified by fixed-form executable read/write analysis. |
| 10 | `IERR` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

This interface declares no separately named workspace argument. Array storage, if any, is Fortran column-major and must satisfy the documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::rgg`. Native symbol: `rgg_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rgg`
- Public declaration feature: `eigen`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `representative-family-coverage`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
