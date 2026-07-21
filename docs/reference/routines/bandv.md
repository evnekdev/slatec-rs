# BANDV

[Family: Eigenvalue problems](../families/eigenvalue-problems.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Form the eigenvectors of a real symmetric band matrix associated with a set of ordered approximate eigenvalues by inverse iteration.

## Description

This subroutine finds those eigenvectors of a REAL SYMMETRIC BAND matrix corresponding to specified eigenvalues, using inverse iteration. The subroutine may also be used to solve systems

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

- Canonical provider: `lin/bandv.f` (`relocated-subset`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/lin/bandv.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/lin/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `source-backed contract awaiting rendered-rustdoc audit`
- Documentation evidence: verified source prologue or source-hash-guarded authored correction
- Exact Netlib source: [BANDV](https://www.netlib.org/slatec/lin/bandv.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `NM` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | dimensional array parameters, A and Z, as declared in the calling program dimension statement.  NM is an INTEGER variable. |
| 2 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the order of the matrix A.  N is an INTEGER variable. must be less than or equal to NM. MB positions of the first column, MB positions of the second column, further subdiagonals similarly, and finally its principal diagonal in the N positions of column MB. If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, A is 1 instead with lower triangle as above and with 1 positions of 2 positions of column MB+2, further superdiagonals similarly, MB positions of the last column.  Contents of storage locations 1). |
| 3 | `MBW` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of columns of the array A used to store the band matrix.  If the matrix is symmetric, MBW is its (half) band width, denoted MB and defined as the number of adjacent diagonals, including the principal diagonal, required to specify the non-zero portion of the lower triangle of the matrix.  If the subroutine is being used to solve systems of linear equations and the coefficient matrix is not symmetric, it must however have the same number of adjacent diagonals above the main diagonal as below, and in this 1.  MBW is an INTEGER variable.  MB must not be greater than N. |
| 4 | `A` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | symmetric band coefficient matrix. contains the lower triangle of the symmetric band input matrix stored as an N by MB array.  Its lowest subdiagonal dimensional dimensional REAL array, dimensioned A(NM,MBW). REAL array, dimensioned A(NM,MBW). W(J)*I)*X(J)=B(J), where I is the identity dimensional REAL array, dimensioned W(M). dimensional REAL array, dimensioned Z(NM,M). are unaltered. W(M)*I is available, upon return, as the product of the first N elements of RV. |
| 5 | `E21` | `input` | `scalar` | `REAL` | `*mut f32` | scalar | specifies the ordering of the eigenvalues and contains 0.0E0 if the eigenvalues are in ascending order, or 2.0E0 if the eigenvalues are in descending order. If the subroutine is being used to solve systems of linear equations, E21 should be set to 1.0E0 if the coefficient matrix is symmetric and to -1.0E0 if not.  E21 is a REAL variable. |
| 6 | `M` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is the number of specified eigenvalues or the number of systems of linear equations.  M is an INTEGER variable. |
| 7 | `W` | `workspace` | `workspace` | `REAL` | `*mut f32` | rank 1; dimensions (*) | contains the M eigenvalues in ascending or descending order. If the subroutine is being used to solve systems of linear 1,2,...,M. dimensional REAL array, dimensioned W(M). are unaltered. |
| 8 | `Z` | `input` | `array` | `REAL` | `*mut f32` | rank 2; dimensions (NM, *) | 1,2,...,M), if the subroutine is used to solve systems of linear equations. dimensional REAL array, dimensioned Z(NM,M). contains the associated set of orthogonal eigenvectors. Any vector which fails to converge is set to zero.  If the subroutine is used to solve systems of linear equations, 1,2,...,M). |
| 9 | `IERR` | `output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | is an INTEGER flag set to Zero       for normal return, -J         if the eigenvector corresponding to the J-th eigenvalue fails to converge, or if the J-th system of linear equations is nearly singular. |
| 10 | `NV` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | must be set to the dimension of the array parameter RV as declared in the calling program dimension statement. is an INTEGER variable. 1). 1). |
| 11 | `RV` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are temporary storage arrays.  If the subroutine is being used to solve systems of linear equations, the dimensional REAL arrays.  Note that RV 1). |
| 12 | `RV6` | `output` | `array` | `REAL` | `*mut f32` | rank 1; dimensions (*) | are temporary storage arrays.  If the subroutine is being used to solve systems of linear equations, the dimensional REAL arrays.  Note that RV is dimensioned RV6(N). Questions and comments should be directed to B. S. Garbow, Applied Mathematics Division, ARGONNE NATIONAL LABORATORY |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

The selected source does not provide a separate error-status section. Any status output argument is identified in the argument table; callers must also respect the legacy SLATEC error-runtime behavior described by the source.

### Storage and workspace requirements

`W`: contains the M eigenvalues in ascending or descending order. If the subroutine is being used to solve systems of linear 1,2,...,M. dimensional REAL array, dimensioned W(M). are unaltered.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::eigen::bandv`. Native symbol: `bandv_`. Declaration feature: `eigen`. Provider feature: `linear-algebra-eigen`. ABI fingerprint: `subroutine:void(mut_i32,mut_i32,mut_i32,mut_f32_ptr_rank2,mut_f32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank2,mut_i32,mut_i32,mut_f32_ptr_rank1,mut_f32_ptr_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::eigen::bandv`
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
