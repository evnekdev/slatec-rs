# TSTURM

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Find those eigenvalues of a symmetric tridiagonal matrix in a given interval and their associated eigenvectors by Sturm sequencing.

## Description

This subroutine finds those eigenvalues of a TRIDIAGONAL SYMMETRIC matrix which lie in a specified interval and their associated eigenvectors, using bisection and inverse iteration.

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `unknown`
- Scalar kind: `unknown`
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

- Canonical provider: `lin/tsturm.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/tsturm.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/lin/tsturm.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [TSTURM](https://www.netlib.org/slatec/lin/tsturm.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameter, Z, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix. N is an INTEGER variable. must be less than or equal to NM. |
| 3 | `EPS1` | `input-output` | `scalar` | `REAL` | `*mut f32` | scalar | is an absolute error tolerance for the computed eigen- values. It should be chosen so that the accuracy of these eigenvalues is commensurate with relative perturbations of the order of the relative machine precision in the matrix elements. If the input EPS1 is non-positive, it is reset for each submatrix to a default value, namely, minus the product of the relative machine precision and the 1-norm of the submatrix. EPS1 is a REAL variable. is unaltered unless it has been reset to its (last) default value. |
| 4 | `D` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the diagonal elements of the symmetric tridiagonal matrix. D is a one-dimensional REAL array, dimensioned D(N). unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero. |
| 5 | `E` | `input-output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the subdiagonal elements of the symmetric tridiagonal matrix in its last N-1 positions. E(1) is arbitrary. E is a one-dimensional REAL array, dimensioned unaltered. Elements of E2, corresponding to elements of E regarded as negligible, have been replaced by zero causing the matrix to split into a direct sum of submatrices. E2(1) is also set to zero. |
| 6 | `E2` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the squares of the corresponding elements of E. is arbitrary. E2 is a one-dimensional REAL array, dimensioned E2(N). |
| 7 | `LB` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. REAL variables. |
| 8 | `UB` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | the interval to be searched for eigenvalues. If LB is not less than UB, no eigenvalues will be found. REAL variables. |
| 9 | `MM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | should be set to an upper bound for the number of eigenvalues in the interval. MM is an INTEGER variable. WARNING - If more than MM eigenvalues are determined to lie in the interval, an error return is made with no values or vectors found. |
| 10 | `M` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of eigenvalues determined to lie in (LB,UB). is an INTEGER variable. |
| 11 | `W` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the M eigenvalues in ascending order if the matrix does not split. If the matrix splits, the eigenvalues are in ascending order for each submatrix. If a vector error exit is made, W contains those values already found. W is a one-dimensional REAL array, dimensioned W(MM). |
| 12 | `Z` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the associated set of orthonormal eigenvectors. If an error exit is made, Z contains those vectors already found. Z is a one-dimensional REAL array, dimensioned. |
| 13 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, 3*N+1 if M exceeds MM no eigenvalues or eigenvectors are computed, 4*N+J if the eigenvector corresponding to the J-th eigenvalue fails to converge in 5 iterations, then the eigenvalues and eigenvectors in W and Z should be correct for indices 1, 2,. , J-1. |
| 14 | `RV1` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and. |
| 15 | `RV2` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and. |
| 16 | `RV3` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and. |
| 17 | `RV4` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and. |
| 18 | `RV5` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and. |
| 19 | `RV6` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | temporary storage arrays, dimensioned RV1(N), RV2(N), RV3(N), RV4(N), RV5(N), and The ALGOL procedure STURMCNT contained in TRISTURM appears in TSTURM in-line. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::tsturm`. Native symbol: `tsturm_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32,mut_f32,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tsturm`
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
