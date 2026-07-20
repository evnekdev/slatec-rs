# RG

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and, optionally, the eigenvectors of a real general matrix.

## Description

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) To find the eigenvalues and eigenvectors (if desired) of a REAL GENERAL matrix. On Input NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix A. N is an INTEGER variable. N must be less than or equal to NM. A contains the real general matrix. A is a two-dimensional REAL array, dimensioned A(NM,N). MATZ is an INTEGER variable set equal to zero if only eigenvalues are desired. Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. On Output A has been destroyed. WR and WI contain the real and imaginary parts, respectively, of the eigenvalues. The eigenvalues are unordered except that complex conjugate pairs of eigenvalues appear consecutively with the eigenvalue having the positive imaginary part first. If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N. WR and WI are one-dimensional REAL arrays, dimensioned WR(N) and WI(N). Z contains the real and imaginary parts of the eigenvectors if MATZ is not zero. If the J-th eigenvalue is real, the J-th column of Z contains its eigenvector. If the J-th eigenvalue is complex with positive imaginary part, the J-th and (J+1)-th columns of Z contain the real and imaginary parts of its eigenvector. The conjugate of this vector is the eigenvector for the conjugate eigenvalue. Z is a two-dimensional REAL array, dimensioned Z(NM,N). IERR is an INTEGER flag set to Zero for normal return, 10*N if N is greater than NM, J if the J-th eigenvalue has not been determined after a total of 30 iterations. The eigenvalues should be correct for indices IERR+1, IERR+2, ..., N, but no eigenvectors are computed. IV1 and FV1 are one-dimensional temporary storage arrays of dimension N. IV1 is of type INTEGER and FV1 of type REAL. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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

- Evidence level: `argument_contract_incomplete`
- Description provenance: `source_prologue`
- Assessment: the routine description and ABI rows are complete, but at least one argument lacks separable semantic evidence
- Dedicated family page: [Eigenvalue problems](../families/eigenvalue-problems.md)

### Arguments

| Argument | Direction | Fortran type | Rust raw type | Shape | Description | Relationships and requirements | Nullable |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `NM` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On Input NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. | On Input NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | N is the order of the matrix A. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `A` | input | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NM, *) | This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) To find the eigenvalues and eigenvectors (if desired) of a REAL GENERAL matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WR` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WR and WI contain the real and imaginary parts, respectively, of the eigenvalues. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `WI` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | WR and WI contain the real and imaginary parts, respectively, of the eigenvalues. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `MATZ` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | MATZ is an INTEGER variable set equal to zero if only eigenvalues are desired. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `Z` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NM, *) | On Input NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. | On Input NM must be set to the row dimension of the two-dimensional array parameters, A and Z, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IV1` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | rank 1; dimensions (*) | IV1 and FV1 are one-dimensional temporary storage arrays of dimension N. | IV1 and FV1 are one-dimensional temporary storage arrays of dimension N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `FV1` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | IV1 and FV1 are one-dimensional temporary storage arrays of dimension N. | IV1 and FV1 are one-dimensional temporary storage arrays of dimension N. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | If an error exit is made, the eigenvalues should be correct for indices IERR+1, IERR+2, ..., N. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::eigen::rg`. Native symbol: `rg_`. Feature: `eigen`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_i32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rg`
- Compatibility aliases: `slatec_sys::eigen::numerical::rg`
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
