# RT

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvalues and eigenvectors of a special real tridiagonal matrix.

## Description

This subroutine calls the recommended sequence of subroutines from the eigensystem subroutine package (EISPACK) to find the eigenvalues and eigenvectors (if desired) of a special REAL TRIDIAGONAL matrix. The property of the matrix required for use of this subroutine is that the products of pairs of corresponding off-diagonal elements be all non-negative. If eigenvectors are desired, no product can be zero unless both factors are zero.

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
- GAMS classifications: `D4A5`
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

- Canonical provider: `lin/rt.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/rt.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/rt.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [RT](https://www.netlib.org/slatec/lin/rt.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameter, A and Z, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix A. N is an INTEGER variable. must be less than or equal to NM. |
| 3 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, 3) | contains the special real tridiagonal matrix in its first three columns. The subdiagonal elements are stored in the last N-1 positions of the first column, the diagonal elements in the second column, and the superdiagonal elements in the first N-1 positions of the third column. Elements A(1,1) and are arbitrary. A is a two-dimensional REAL array, dimensioned A(NM,3). |
| 4 | `W` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the eigenvalues in ascending order. W is a one-dimensional REAL array, dimensioned W(N). |
| 5 | `MATZ` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER variable set equal to zero if only eigenvalues are desired. Otherwise, it is set to any non-zero integer for both eigenvalues and eigenvectors. |
| 6 | `Z` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the eigenvectors if MATZ is not zero. The eigen- vectors are not normalized. Z is a two-dimensional REAL array, dimensioned Z(NM,N). |
| 7 | `FV1` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | is a one-dimensional REAL array used for temporary storage, dimensioned FV1(N). Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |
| 8 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, 10*N if N is greater than NM, N+J if A(J,1)*A(J-1,3) is negative, 2*N+J if the product is zero with one factor non-zero, and MATZ is non-zero; J if the J-th eigenvalue has not been determined after 30 iterations. The eigenvalues and eigenvectors in the W and Z arrays should be correct for indices 1, 2,. , IERR-1. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::rt`. Native symbol: `rt_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::rt`
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
