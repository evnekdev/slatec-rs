# FIGI

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Transforms certain real non-symmetric tridiagonal matrix to symmetric tridiagonal matrix.

## Description

Given a NONSYMMETRIC TRIDIAGONAL matrix such that the products of corresponding pairs of off-diagonal elements are all non-negative, this subroutine reduces it to a symmetric tridiagonal matrix with the same eigenvalues. If, further, a zero product only occurs when both factors are zero, the reduced matrix is similar to the original matrix. On INPUT NM must be set to the row dimension of the two-dimensional array parameter, T, as declared in the calling program dimension statement. NM is an INTEGER variable. N is the order of the matrix T. N is an INTEGER variable. N must be less than or equal to NM. T contains the nonsymmetric matrix. Its subdiagonal is stored in the last N-1 positions of the first column, its diagonal in the N positions of the second column, and its superdiagonal in the first N-1 positions of the third column. T(1,1) and T(N,3) are arbitrary. T is a two-dimensional REAL array, dimensioned T(NM,3). On OUTPUT T is unaltered. D contains the diagonal elements of the tridiagonal symmetric matrix. D is a one-dimensional REAL array, dimensioned D(N). E contains the subdiagonal elements of the tridiagonal symmetric matrix in its last N-1 positions. E(1) is not set. E is a one-dimensional REAL array, dimensioned E(N). E2 contains the squares of the corresponding elements of E. E2 may coincide with E if the squares are not needed. E2 is a one-dimensional REAL array, dimensioned E2(N). IERR is an INTEGER flag set to Zero for normal return, N+I if T(I,1)*T(I-1,3) is negative and a symmetric matrix cannot be produced with FIGI, -(3*N+I) if T(I,1)*T(I-1,3) is zero with one factor non-zero. In this case, the eigenvectors of the symmetric matrix are not simply related to those of T and should not be sought. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY

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
- GAMS classifications: `D4C1C`
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

- Canonical provider: `lin/figi.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/figi.f) — `verified_cached`
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
| `NM` | input | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | On INPUT NM must be set to the row dimension of the two-dimensional array parameter, T, as declared in the calling program dimension statement. | On INPUT NM must be set to the row dimension of the two-dimensional array parameter, T, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `N` | output | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | N is the order of the matrix T. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `T` | output | `REAL` (`explicit`) | `*mut f32` | rank 2; dimensions (NM, 3) | On INPUT NM must be set to the row dimension of the two-dimensional array parameter, T, as declared in the calling program dimension statement. | On INPUT NM must be set to the row dimension of the two-dimensional array parameter, T, as declared in the calling program dimension statement. Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `D` | output | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | D contains the diagonal elements of the tridiagonal symmetric matrix. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | E contains the subdiagonal elements of the tridiagonal symmetric matrix in its last N-1 positions. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `E2` | unavailable | `REAL` (`explicit`) | `*mut f32` | rank 1; dimensions (*) | E2 contains the squares of the corresponding elements of E. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |
| `IERR` | unavailable | `INTEGER` (`explicit`) | `*mut crate::FortranInteger` | scalar | IERR is an INTEGER flag set to Zero for normal return, N+I if T(I,1)*T(I-1,3) is negative and a symmetric matrix cannot be produced with FIGI, -(3*N+I) if T(I,1)*T(I-1,3) is zero with one factor non-zero. | none stated in the separable source sentence Leading dimension: not established Workspace: not established | required; null is not permitted for an ordinary Fortran actual argument |

The table reports compiler/interface facts separately from source-prologue semantics. Unknown intent, aliasing, workspace, leading-dimension, and retention rules remain explicit; parameter names alone are never treated as semantic evidence. Native code does not retain ordinary argument pointers unless a reviewed declaration explicitly says otherwise.

### ABI and safety

Canonical path: `slatec_sys::linear_algebra::eigen::figi`. Native symbol: `figi_`. Feature: `eigen`. Provider status: `selected_provider_verified`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`. Every pointer must be aligned and valid for the full source-defined readable or writable extent; callers must uphold array dimensions, leading dimensions, workspace formulas, aliasing restrictions, callback lifetimes, and process-global runtime serialization.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::figi`
- Compatibility aliases: `slatec_sys::eigen::numerical::figi`
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
