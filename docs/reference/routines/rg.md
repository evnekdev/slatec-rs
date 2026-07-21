# RG

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a real general matrix.

## Description

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) To find the eigenvalues and eigenvectors (if desired) of a REAL GENERAL matrix.

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
- GAMS classifications: `D4A2`
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

- Canonical provider: `lin/rg.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/rg.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/rg.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [RG](https://www.netlib.org/slatec/lin/rg.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM. |
| 3 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the real general matrix. A is a two-dimensional REAL array, dimensioned A(NM,N). |
| 4 | `WR` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | the real and imaginary parts, respectively, of the eigenvalues. The eigenvalues are unordered except that complex conjugate pairs of eigenvalues appear consecu- tively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2,. , N. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 5 | `WI` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | the real and imaginary parts, respectively, of the eigenvalues. The eigenvalues are unordered except that complex conjugate pairs of eigenvalues appear consecu- tively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2,. , N. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). |
| 6 | `MATZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER variable set equal to zero if only eigenvalues are desired. Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. |
| 7 | `Z` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the real and imaginary parts of the eigenvectors if MATZ is not zero. If the J-th eigenvalue is real, the J-th column of Z contains its eigenvector. If the J-th eigenvalue is complex with positive imaginary part, the J-th and (J+1)-th columns of Z contain the real and imaginary parts of its eigenvector. The conjugate of this vector is the eigenvector for the conjugate eigenvalue. is a two-dimensional REAL array, dimensioned Z(NM,N). |
| 8 | `IV1` | `output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | one-dimensional temporary storage arrays of dimension N. IV1 is of type INTEGER and FV1 of type REAL. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |
| 9 | `FV1` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | one-dimensional temporary storage arrays of dimension N. IV1 is of type INTEGER and FV1 of type REAL. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |
| 10 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, 10*N if N is greater than NM, J if the J-th eigenvalue has not been determined after a total of 30 iterations. The eigenvalues should be correct for indices IERR+1, IERR+2,. , N, but no eigenvectors are computed. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::rg`. Native symbol: `rg_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rg`
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
