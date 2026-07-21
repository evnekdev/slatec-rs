# TINVIT

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Compute the eigenvectors of symmetric tridiagonal matrix corresponding to specified eigenvalues, using inverse iteration.

## Description

This subroutine is a translation of the inverse iteration tech- nique in the ALGOL procedure TRISTURM by Peters and Wilkinson. HANDBOOK FOR AUTO. COMP., VOL.II-LINEAR ALGEBRA, 418-439(1971). This subroutine finds those eigenvectors of a TRIDIAGONAL SYMMETRIC matrix corresponding to specified eigenvalues, using inverse iteration.

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
- GAMS classifications: `D4C3`
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

- Canonical provider: `lin/tinvit.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/tinvit.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [TINVIT](https://www.netlib.org/slatec/lin/tinvit.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensional array parameter, Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix.  N is an INTEGER variable. must be less than or equal to NM. 1 positions.  E(1) is Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY |
| 3 | `D` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the diagonal elements of the symmetric tridiagonal dimensional REAL array, dimensioned D(N). 1).  E2(1) must contain 1).  E2(1) must contain 0.0e0 if the eigenvalues are in ascending order, or 2.0e0 0.0e0 if the eigenvalues are in ascending order, or 2.0e0 if the eigenvalues are in descending order.  If  BISECT, if the eigenvalues are in descending order.  If  BISECT, TRIDIB, or  IMTQLV  has been used to find the eigenvalues, TRIDIB, or  IMTQLV  has been used to find the eigenvalues, their output E2 array is exactly what is expected here. their output E2 array is exactly what is expected here. |
| 4 | `E` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the subdiagonal elements of the symmetric dimensional REAL array, dimensioned is considered negligible if it is not larger than the product of the relative machine precision and the sum |
| 5 | `E2` | `input` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the squares of the corresponding elements of E, with zeros corresponding to negligible elements of E. dimensional REAL array, dimensioned E2(N). |
| 6 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of specified eigenvalues for which eigenvectors are to be determined.  M is an INTEGER variable. |
| 7 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the M eigenvalues in ascending or descending order. dimensional REAL array, dimensioned W(M). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. If  BISECT  or  TRIDIB  has been used to determine the eigenvalues, their output IND array is suitable for input |
| 8 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | contains in its first M positions the submatrix indices dimensional INTEGER array, dimensioned IND(M). |
| 9 | `Z` | `output` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | contains the associated set of orthonormal eigenvectors. Any vector which fails to converge is set to zero. dimensional REAL array, dimensioned Z(NM,M). |
| 10 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero       for normal return, -J         if the eigenvector corresponding to the J-th eigenvalue fails to converge in 5 iterations. |
| 11 | `RV1` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and |
| 12 | `RV2` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and |
| 13 | `RV3` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays used for temporary storage.  They are used to store the main diagonal and the two adjacent diagonals of the triangular matrix produced in the inverse iteration process.  RV1, RV2 and are dimensioned RV1(N), RV2(N) and RV3(N). |
| 14 | `RV4` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays used for temporary storage.  RV4 holds the multipliers of the Gaussian elimination process.  RV6 holds the approximate eigenvectors are dimensioned RV4(N) and |
| 15 | `RV6` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | dimensional REAL arrays used for temporary storage.  RV4 holds the multipliers of the Gaussian elimination process.  RV6 holds the approximate eigenvectors are dimensioned RV4(N) and Questions and comments should be directed to B. S. Garbow, APPLIED MATHEMATICS DIVISION, ARGONNE NATIONAL LABORATORY |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: contains the M eigenvalues in ascending or descending order. dimensional REAL array, dimensioned W(M). 1 for eigenvalues belonging to the first submatrix from the top, 2 for those belonging to the second submatrix, etc. If  BISECT  or  TRIDIB  has been used to determine the eigenvalues, their output IND array is suitable for input

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::tinvit`. Native symbol: `tinvit_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_i32,mut_f32_ptr_rank1,mut_i32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::tinvit`
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
