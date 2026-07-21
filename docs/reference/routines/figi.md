# FIGI

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Transforms certain real non-symmetric tridiagonal matrix to symmetric tridiagonal matrix.

## Description

Given a NONSYMMETRIC TRIDIAGONAL matrix such that the products of corresponding pairs of off-diagonal elements are all non-negative, this subroutine reduces it to a symmetric tridiagonal matrix with the same eigenvalues. If, further, a zero product only occurs when both factors are zero, the reduced matrix is similar to the original matrix.

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

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [FIGI](https://www.netlib.org/slatec/lin/figi.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the row dimension of the two-dimensional array parameter, T, as declared in the calling program dimension statement. NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix T. N is an INTEGER variable. must be less than or equal to NM. |
| 3 | `T` | `input-output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, 3) | contains the nonsymmetric matrix. Its subdiagonal is stored in the last N-1 positions of the first column, its diagonal in the N positions of the second column, and its superdiagonal in the first N-1 positions of the third column. T(1,1) and T(N,3) are arbitrary. is a two-dimensional REAL array, dimensioned T(NM,3). is unaltered. |
| 4 | `D` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the diagonal elements of the tridiagonal symmetric matrix. D is a one-dimensional REAL array, dimensioned D(N). |
| 5 | `E` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the subdiagonal elements of the tridiagonal symmetric matrix in its last N-1 positions. E(1) is not set. is a one-dimensional REAL array, dimensioned E(N). |
| 6 | `E2` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the squares of the corresponding elements of E. E2 may coincide with E if the squares are not needed. is a one-dimensional REAL array, dimensioned E2(N). |
| 7 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero for normal return, N+I if T(I,1)*T(I-1,3) is negative and a symmetric matrix cannot be produced with FIGI, -(3*N+I) if T(I,1)*T(I-1,3) is zero with one factor non-zero. In this case, the eigenvectors of the symmetric matrix are not simply related to those of T and should not be sought. Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Storage and array requirements

Array arguments use Fortran column-major storage and must satisfy their documented shape and leading-dimension relationships.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::figi`. Native symbol: `figi_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::figi`
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
