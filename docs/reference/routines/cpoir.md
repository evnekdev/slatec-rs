# CPOIR

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a positive definite Hermitian system of linear equations. Iterative refinement is used to obtain an error estimate.

## Description

Subroutine CPOIR solves a complex positive definite Hermitian NxN system of single precision linear equations using LINPACK subroutines CPOFA and CPOSL. One pass of iterative refine- ment is used only to obtain an estimate of the accuracy. That is, if A is an NxN complex positive definite Hermitian matrix and if X and B are complex N-vectors, then CPOIR solves the equation A*X=B. Care should be taken not to use CPOIR with a non-Hermitian matrix. The matrix A is first factored into upper and lower triangular matrices R and R-TRANSPOSE. These factors are used to calculate the solution, X. Then the residual vector is found and used

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `complex_f32`
- Scalar kind: `complex`
- Primary family: `Dense linear algebra`
- Mathematical domain: `dense-linear-algebra`
- Package provenance: `unknown`
- GAMS classifications: `D2D1B`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `bound`
- Build/profile status: `selected_by_profile`
- Audit status: `family_inventory_only`
- Safe-API status: `none`
- Implementation status: `not_exposed_as_safe_api`
- Deferment status: Catalogue inclusion does not imply a Rust binding or safe API.

## Providers

- Canonical provider: `main-src/src/cpoir.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cpoir.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cpoir.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cpoir.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-structured`
- Documentation evidence: source prologue, verified source hash, and fixed-form executable analysis where an argument section is absent
- Exact Netlib source: [CPOIR](https://www.netlib.org/slatec/src/cpoir.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDA, *) | Array argument classified by fixed-form executable read/write analysis. |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 4 | `V` | `input` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | Array argument classified by fixed-form executable read/write analysis. |
| 5 | `ITASK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Scalar argument classified by fixed-form executable read/write analysis. |
| 6 | `IND` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | Status argument classified by fixed-form executable read/write analysis. |
| 7 | `WORK` | `workspace` | `workspace` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (N, *) | Workspace argument classified by fixed-form executable read/write analysis. |

Argument evidence records nullability, shape, relationships, leading dimensions, workspace rules, options, and overwrite behavior in the authoritative public-documentation inventory. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

This interface declares no callback argument.

### Error and status values

IND estimates the accuracy of the solution only when the input matrix and the right hand side are represented exactly in the computer and does not take into account If the equation A*X=B is to be solved for more than one vector B, the factoring of A does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions.  In this case, the contents of A, LDA, N, and WORK must not have been altered by the user following factorization (ITASK=1).  IND will not be changed by CPOIR in this case. Argument Description *** A       COMPLEX(LDA,N) the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix.  Only the upper triangle, including the diagonal, of the coefficient matrix need be entered.  A is not altered by the routine. LDA    INTEGER the leading dimension of the array A.  LDA must be great- N      INTEGER the order of the matrix A.  N must be greater than V      COMPLEX(N) on entry, the singly subscripted array(vector) of di- mension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X . ITASK  INTEGER if ITASK = 1, the matrix A is factored and then the linear equation is solved. if ITASK .GT. 1, the equation is solved using the existing factored matrix A (stored in WORK). printed. IND    INTEGER GT. 0  IND is a rough estimate of the number of digits of accuracy in the solution, X.  IND=75 means that the solution vector X is zero. WORK   COMPLEX(N*(N+1)) a singly subscripted array of dimension at least N*(N+1). IND=-1  terminal   N is greater than LDA. IND=-2  terminal   N is less than one. IND=-3  terminal   ITASK is less than one. IND=-4  terminal   The matrix A is computationally singular or is not positive definite. A solution has not been computed. IND=-10 warning    The solution has no apparent significance. the solution may be inaccurate or the matrix a may be poorly scaled. designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 .  LEVEL=0 will be printed followed by an abort.

### Storage and workspace requirements

`WORK`: Workspace argument classified by fixed-form executable read/write analysis.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpoir`. Native symbol: `cpoir_`. Declaration feature: `linear-algebra-complex`. Provider feature: `linear-algebra-complex`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32,mut_complex32_array_rank2)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpoir`
- Public declaration feature: `linear-algebra-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
