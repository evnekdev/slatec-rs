# CPOFS

[Family: Dense linear algebra](../families/dense-linear-algebra.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve a positive definite symmetric complex system of linear equations.

## Description

Subroutine CPOFS solves a positive definite symmetric NxN system of complex linear equations using LINPACK subroutines CPOCO and CPOSL. That is, if A is an NxN complex positive definite symmetric matrix and if X and B are complex N-vectors, then CPOFS solves the equation A*X=B. Care should be taken not to use CPOFS with a non-Hermitian matrix. The matrix A is first factored into upper and lower tri- angular matrices R and R-TRANSPOSE. These factors are used to find the solution vector X. An approximate condition number is calculated to provide a rough estimate of the number of digits of accuracy in the computed solution. If the equation A*X=B is to be solved for more than one vector B, the factoring of a does not need to be performed again and the option to only solve (ITASK .GT. 1) will be faster for the succeeding solutions. In this case, the contents of A, LDA, and N must not have been altered by the user following factorization (ITASK=1). IND will not be changed by CPOFS in this case. Argument Description ***

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

- Canonical provider: `main-src/src/cpofs.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/cpofs.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/cpofs.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/cpofs.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [CPOFS](https://www.netlib.org/slatec/src/cpofs.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `A` | `output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 2; dimensions (LDA, *) | COMPLEX(LDA,N) on entry, the doubly subscripted array with dimension (LDA,N) which contains the coefficient matrix. Only the upper triangle, including the diagonal, of the coefficient matrix need be entered and will subse- quently be referenced and changed by the routine. on return, contains in its upper triangle an upper triangular matrix R such that A = (R-TRANSPOSE) * R. |
| 2 | `LDA` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the leading dimension of the array A. LDA must be great- er than or equal to N. (terminal error message IND=-1). |
| 3 | `N` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER the order of the matrix A. N must be greater than or equal to 1. (terminal error message IND=-2). |
| 4 | `V` | `output` | `array` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(N) on entry the singly subscripted array(vector) of di- mension N which contains the right hand side B of a system of simultaneous linear equations A*X=B. on return, V contains the solution vector, X. |
| 5 | `ITASK` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER if ITASK = 1, the matrix A is factored and then the linear equation is solved. if ITASK. GT. 1, the equation is solved using the existing factored matrix A. LT. 1, then terminal error message IND=-3 is printed. |
| 6 | `IND` | `input` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | INTEGER GT. 0 IND is a rough estimate of the number of digits of accuracy in the solution, X. LT. 0 see error message corresponding to IND below. |
| 7 | `WORK` | `workspace-output` | `workspace` | `COMPLEX` | `*mut crate::Complex32` | rank 1; dimensions (*) | COMPLEX(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than 1. IND=-3 terminal ITASK is less than 1. IND=-4 terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `-1` | terminal N is greater than LDA. |
| `IND` | `-2` | terminal N is less than 1. |
| `IND` | `-3` | terminal ITASK is less than 1. |
| `IND` | `-4` | terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. |
| `IND` | `-10` | warning The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. NOTE- The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which |
| `IND` | `1` | (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort. |

### Storage and workspace requirements

`WORK`: COMPLEX(N) a singly subscripted array of dimension at least N. Error Messages Printed *** IND=-1 terminal N is greater than LDA. IND=-2 terminal N is less than 1. IND=-3 terminal ITASK is less than 1. IND=-4 terminal The matrix A is computationally singular or is not positive definite. A solution has not been computed. IND=-10 warning The solution has no apparent significance. The solution may be inaccurate or the matrix A may be poorly scaled. NOTE- The above terminal(*fatal*) error messages are designed to be handled by XERMSG in which LEVEL=1 (recoverable) and IFLAG=2 . LEVEL=0 for warning error messages from XERMSG. Unless the user provides otherwise, an error message will be printed followed by an abort.

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpofs`. Native symbol: `cpofs_`. Declaration feature: `linear-algebra-complex`. Provider feature: `linear-algebra-complex`. ABI fingerprint: `subroutine:void(mut_complex32_array_rank2,mut_i32,mut_i32,mut_complex32_array_rank1,mut_i32,mut_i32,mut_complex32_array_rank1)`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `compiler-validated`
- Canonical Rust path: `slatec_sys::linear_algebra::dense::complex::cpofs`
- Public declaration feature: `linear-algebra-complex`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_generated_abi_contract`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `not-recorded`
- Safe-wrapper status: `not_safely_wrapped`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
